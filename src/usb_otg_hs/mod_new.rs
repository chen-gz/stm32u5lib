use core::future::poll_fn;
use core::sync::atomic::Ordering;
use core::task::Poll;
use cortex_m::peripheral::NVIC;
use defmt::export::{panic, write};
use defmt::{info, trace};
use stm32_metapac::{interrupt, otg::{self, regs::Doepctl}, PWR, RCC, SYSCFG};

use crate::usb_otg_hs::{regs, restore_irqs, state, RX_FIFO_SIZE_SIZE_WORD, TX_FIFO_SIZE_WORDS, EP_OUT_BUFFER_EMPTY, State};

pub fn power_up_init() {
    trace!("init");
    PWR.svmcr().modify(|w| {
        w.set_usv(true); // RM0456 (rev 4) p 404. Romove Vddusb isolation
    });
    #[cfg(stm32u5a5)]
    {
        critical_section::with(|_| {
            PWR.vosr().modify(|v| {
                v.0 |= (1 << 19) | (1 << 20);
                // SBPWREN and USBBOOSTEN in PWR_VOSR.
                // v.boosten();
            });
            crate::clock::delay_us(100);
            // delay_ms(100);
            // wait fo USBBOOSTRDY
            // while !pwr.vosr().read().usbboostrdy() {}
            // enable hse
            RCC.cr().modify(|w| {
                w.set_hseon(true);
            });
            // wait for hse ready
            while !RCC.cr().read().hserdy() {}

            RCC.ccipr2().modify(|w| {
                w.set_otghssel(stm32_metapac::rcc::vals::Otghssel::HSE);
            });

            RCC.apb3enr().modify(|w| {
                w.set_syscfgen(true);
            });
            RCC.ahb2enr1().modify(|w| {
                w.set_usb_otg_hs_phyen(true);
                w.set_usb_otg_hsen(true);
            });
            // TODO: update this clock settings
            SYSCFG.otghsphycr().modify(|v| {
                // v.set_clksel(0b1110);
                v.set_clksel(0b0011); // 16Mhz HSE
                v.set_en(true);
            });
        });
    }
    // Wait for USB power to stabilize
    while !stm32_metapac::PWR.svmsr().read().vddusbrdy() {}
    trace!("USB power stabilized");

    // Select HSI48 as USB clock source.
    #[cfg(stm32u575)]
    critical_section::with(|_| {
        stm32_metapac::RCC.ccipr1().modify(|w| {
            w.set_iclksel(stm32_metapac::rcc::vals::Iclksel::HSI48);
        })
    });
    #[cfg(stm32u5a5)]
    critical_section::with(|_| {
        stm32_metapac::RCC.ccipr2().modify(|w| {
            w.set_otghssel(stm32_metapac::rcc::vals::Otghssel::HSE);
        })
    });
    #[cfg(stm32u575)]
    stm32_metapac::RCC
        .ahb2enr1()
        .modify(|w| w.set_usb_otg_fsen(true));

    #[cfg(stm32u575)]
    unsafe {
        NVIC::unpend(stm32_metapac::Interrupt::OTG_FS);
        NVIC::unmask(stm32_metapac::Interrupt::OTG_FS);
        // start_irq();
        Self::restore_irqs();
        trace!("USB IRQs start");
    }

    #[cfg(stm32u5a5)]
    unsafe {
        NVIC::unpend(stm32_metapac::Interrupt::OTG_HS);
        NVIC::unmask(stm32_metapac::Interrupt::OTG_HS);
        // start_irq();
        restore_irqs();
        trace!("USB IRQs start");
    }

    let r = regs();
    let core_id = r.cid().read().0;
    trace!("Core id {:08x}", core_id);

    // Wait for AHB ready.
    while !r.grstctl().read().ahbidl() {}

    r.gusbcfg().write(|w| {
        w.set_fdmod(true); // Force device mode TODO: no host mode support
        // w.set_physel(self.phy_type.internal() && !self.phy_type.high_speed());
        // Enable internal full-speed PHY
    });

    match core_id {
        // this is used to distinguish differnet stm32 chips
        0x0000_2000 | 0x0000_2100 | 0x0000_2300 | 0x0000_3000 | 0x0000_3100 => {
            // F446-like chips have the GCCFG.VBDEN bit with the opposite meaning
            r.gccfg_v2().modify(|w| {
                w.set_pwrdwn(true); // Enable internal full-speed PHY,
                // w.set_vbden(val: true); // vbus detect. these can used to save power.
            });
            // todo: vbus detection

            // Force B-peripheral session
            // r.gotgctl().modify(|w| {
            //     w.set_bvaloen(!self.config.vbus_detection); // B-peripheral session valid. Only  used as device
            //     w.set_bvaloval(true);
            // });
        }
        0x0000_5000 => {
            // U5A5
            r.gccfg_v2().modify(|w| {
                // w.set_pwrdwn(true);
                // w.set_vbden(self.config.vbus_detection);   // vbus detect. these can used to save power.
                // w.set_vbden(true);
                // w.set_phyhsen(true);
                w.0 = (1 << 24) | (1 << 23);
            });

            // Force B-peripheral session
            r.gusbcfg().modify(|w| w.set_trdt(0x09));
        }
        _ => unimplemented!("Unknown USB core id {:X}", core_id),
    }
    r.gotgctl().modify(|w| {
        w.set_bvaloen(true);
        w.set_bvaloval(true);
    });

    // Soft disconnect.
    r.dctl().write(|w| w.set_sdis(true));

    // Set speed.
    r.dcfg().write(|w| {
        w.set_pfivl(otg::vals::Pfivl::FRAME_INTERVAL_80); // set period frame interval TODO: figure out what is this
        #[cfg(stm32u575)]
        w.set_dspd(self.phy_type.to_dspd()); // todo: for u5a5, this is different. 11 is reserved
        #[cfg(stm32u5a5)]
        w.set_dspd(otg::vals::Dspd::FULL_SPEED_EXTERNAL);
        // w.set_dspd(otg::vals::Dspd::HIGH_SPEED); // todo: for u5a5, this is different. 11 is reserved
    });

    // r.diepmsk().write(|w| {
    //     w.set_xfrcm(true); // Unmask transfer complete EP interrupt
    // });
    r.gintsts()
        .write_value(stm32_metapac::otg::regs::Gintsts(0xFFFF_FFFF));

    // Unmask global interrupt
    r.gahbcfg().write(|w| {
        w.set_dmaen(true); // Enable DMA
        w.set_gintmsk(true); // unmask global interrupt
        //
    });

    // Connect
    r.dctl().write(|w| w.set_sdis(false));
}


pub fn init_reset() {
    // Rm0456 Rev 5, p3423
    let r = regs();
    // 1. set the NAK -- SNAK = 1 in OTG_DOEPCTLx register
    for i in 0..8 {
        r.doepctl(i).modify(|w| w.set_snak(true));
        r.diepctl(i).modify(|w| w.set_snak(true));
    }
    // 2. unmask interrupt bits
    r.daintmsk().write(|w| {
        w.set_iepm(1);
        w.set_oepm(1);
    });
    r.doepmsk().write(|w| {
        w.set_stupm(true);
        w.set_xfrcm(true);
    });
    r.diepmsk().write(|w| {
        w.set_xfrcm(true);
        w.set_tom(true);
    });
    // 3. set up data fifo ram for each of the fifo
    init_fifo();
    // r.doepctl(0).modify(|w| w.set_cnak(true));

    // 4 and 5 in RM0456 Rev 5, p3423
    // goto setup processing stage
    // r.doepdma(0)
    //     .write(|w| w.set_dmaaddr(unsafe { &setup_data as *const u8 as u32 }));
    // r.doeptsiz(0).write(|w| {
    //     w.set_stupcnt(3);
    //     w.set_pktcnt(1);
    //     // w.set_xfrsiz(8);
    // });
    crate::usb_otg_hs::endpoint_new::init_endpoint();
    r.daintmsk().write(|w| {
        w.set_iepm(1);
        w.set_oepm(1);
        // w.set_oepm(2);
    });
}

pub static mut setup_data: [u8; 64] = [0; 64];
pub static mut setup_return_data: [u8; 256] = [0; 256];

/// Initializes FIFOs based on the fifo_const.
pub fn init_fifo() {
    trace!("init_fifo");

    // let r = T::regs();
    let r = regs();

    r.grxfsiz().modify(|w| w.set_rxfd(RX_FIFO_SIZE_SIZE_WORD));

    for i in 0..TX_FIFO_SIZE_WORDS.len() {
        r.dieptxf(i).write(|w| {
            w.set_fd(TX_FIFO_SIZE_WORDS[i]);
            w.set_sa(RX_FIFO_SIZE_SIZE_WORD + TX_FIFO_SIZE_WORDS[0..i].iter().sum::<u16>());
        });
    }

    // Flush fifos
    r.grstctl().write(|w| {
        w.set_rxfflsh(true);
        w.set_txfflsh(true);
        w.set_txfnum(0x10);
    });
    // loop {
    //     let x = r.grstctl().read();
    //     if !x.rxfflsh() && !x.txfflsh() {
    //         break;
    //     }
    // }
}

pub fn init_enumeration_done() {
    // RM0456 Rev 5, p3423
    // 1. On the enumeration done interrupt, (ENUMDNE bit in OTG_GINTSTS register), read the otg_dsts register to get the speed of the enumeration.
    let spd = regs().dsts().read().enumspd();
    // 2. program the mpsiz field in the otg_diepctl0 to set the maximum packet size. This step configures control endpoint 0.. The maximum packet size for a control depends on the enumeration speed.
    // todo: for now, we only support full speed and high speed
    // let mpsiz = match spd {
    //     otg::vals::Enumspd::HIGH_SPEED => 64,
    //     otg::vals::Enumspd::FULL_SPEED => 64,
    //     otg::vals::Enumspd::LOW_SPEED => 8,
    // };
    regs().diepctl(0).modify(|w| w.set_mpsiz(00));
    //
    trace!("doepctl0: {:x}", regs().doepctl(0).read().0);
    trace!("doepdma0: {:x}", regs().doepdma(0).read().0);
    trace!("doeptsiz0: {:x}", regs().doeptsiz(0).read().0);
    trace!("irq mask 0: {:x}", regs().gintmsk().read().0);
    // restore_irqs();
    // 3. for usb otg_hs in dma mode,  program the otg_doepctl0 register to enable control OUT endpoint 0, to receive a setup packet.
    // regs().doepctl(0).modify(|w| {
    //     w.set_cnak(true);
    //     w.set_epena(true);
    // });
}

fn init_setaddress(address: u8) {
    // RM0456 Rev 5, p3423
    // 1. program the otg_dcfg register to set the device address.
    regs().dcfg().modify(|w| w.set_dad(address));
}

// let res = process_setup_packet(setup_data);

use usb_device::{self, device};
use usb_device::control::{Recipient, RequestType};
use usb_device::UsbDirection;
use crate::usb_otg_hs::descriptor::*;
use crate::usb_otg_hs::endpoint_new::{Endpoint, EpType, MaxPacketSize};


pub struct SetupResponse {
    setup: SetupPacket,
    request: Request,
    data_stage_direction: Direction,
    has_data_stage: bool,
    data: [u8; 256],
    len: usize,
}

pub fn process_setup_packet_new(buf: &[u8]) -> SetupResponse {
    defmt::info!("process_setup_packet, {:x}", buf);
    let setup = SetupPacket::from_bytes(buf);
    let mut response = SetupResponse {
        setup,
        request: Request::from_setup(&setup),
        data_stage_direction: setup.direction,
        has_data_stage: setup.length != 0,
        data: [0; 256],
        len: setup.length as usize,
    };
    if setup.direction == Direction::In {
        match response.request {
            Request::GetDeviceDescriptor(len) => {
                let desc = USB_CDC_ACM_DEVICE_DESCRIPTOR.as_bytes();
                let len = core::cmp::min(len as usize, desc.len());
                for i in 0..len {
                    response.data[i] = desc[i];
                }
                response.len = len;
                defmt::info!("GetDeviceDescriptor, len={}", len);
            }
            Request::GetConfigurationDescriptor(len) => {
                let desc = CDC_CONIG_DESC_FULL();
                let len = core::cmp::min(len as usize, desc.len());
                for i in 0..len {
                    response.data[i] = desc[i];
                }
                response.len = len;
                defmt::info!("GetConfigurationDescriptor, len={}", len);
            }
            Request::GetStringLangDescriptor(len) => {
                let val = StringDescriptor::language();
                for i in 0..val[0] as _ {
                    response.data[i] = val[i];
                }
                response.len = val[0] as usize;
                defmt::info!("GetStringLangDescriptor, len={}", len);
            }
            Request::GetStringManufacturerDescriptor(len) => {
                let val = StringDescriptor::manufacturer("GGETA");
                for i in 0..val[0] as _ {
                    response.data[i] = val[i];
                }
                response.len = val[0] as usize;
            }
            Request::GetStringProductDescriptor(len) => {
                let val = StringDescriptor::product("USB Example Device");
                for i in 0..val[0] as _ {
                    response.data[i] = val[i];
                }
                response.len = val[0] as usize;
            }
            Request::GetSerialNumberDescriptor(len) => {
                let val = StringDescriptor::serial_number("123456");
                for i in 0..val[0] as _ {
                    response.data[i] = val[i];
                }
                response.len = val[0] as usize;
            }
            Request::GetDeviceQualifierDescriptor(len) => {
                let val = USB_CDC_DEVICE_QUALIFIER_DESCRIPTOR.as_bytes();
                for i in 0..10 {
                    response.data[i] = val[i];
                }
                response.len = 10;
            }
            _ => {
                defmt::panic!("Unknown request");
            }
        }
    }
    return response;
}

pub struct EndpointGG;

async fn read0(buf: &mut [u8]) {
    trace!("read start len={}", buf.len());
    let r = regs();
    r.doepdma(0) .write(|w| unsafe { w.set_dmaaddr(buf.as_ptr() as u32) });
    info!("doepdma0: {:x}", r.doepdma(0).read().0);

    defmt::info!("*************************************");
    defmt::info!("doepctl0: {:x}", regs().doepctl(0).read().0);
    defmt::info!("doeptsiz0: {:x}", regs().doeptsiz(0).read().0);
    if regs().doepctl(0).read().epena() {
        defmt::error!("epena is set -- this should not happen");
        // clear epena
        r.doepctl(0).modify(|w| {
            w.set_epena(false);
        });
        // return;
    }
    defmt::info!("doepctl0: {:x}", regs().doepctl(0).read().0);
    r.doeptsiz(0).modify(|w| {
        w.set_xfrsiz(buf.len() as _);
        if buf.len() == 8 {
            w.set_stupcnt(1);
            w.set_pktcnt(1);
        } else {
            w.set_pktcnt(1);
            w.set_stupcnt(1);
        }
    });
    r.doepmsk().modify(|w| w.set_stsphsrxm(true)); // unmask

    // for dma this is required
    r.doepctl(0).modify(|w| {
        w.set_epena(true);
        w.set_cnak(true);
    });
    defmt::info!("doepctl0: {:x}", regs().doepctl(0).read().0);
    defmt::info!("doeptsiz0: {:x}", regs().doeptsiz(0).read().0);
    r.doepmsk().modify(|w| w.set_xfrcm(true)); // unmask
    // wait for transfer complete interrupt
    poll_fn(|cx| {
        state().ep_out_wakers[0].register(cx.waker());
        if r.doepint(0).read().xfrc() {
            r.doepint(0).write(|w| {
                w.set_xfrc(true);
            });
            // clear xfrc
            trace!("read done len={}", buf.len());
            Poll::Ready(())
        } else {
            Poll::Pending
        }
    }) .await;
}

async fn write0(buf: &[u8]) {
    trace!("write start len={}, data={:x}", buf.len(), buf);
    let r = regs();
    r.diepdma(0)
        .write(|w| unsafe { w.set_dmaaddr(buf.as_ptr() as u32) });

    let pktcnt;
    if buf.len() == 0 {
        pktcnt = 1;
    } else {
        pktcnt = (buf.len() + 63) / 64;
    }
    r.dieptsiz(0).modify(|w| {
        w.set_xfrsiz(buf.len() as u32);
        // w.set_pktcnt(buf.len() + 63 / 64);
        w.set_pktcnt(pktcnt as _);
    });

    r.diepctl(0).modify(|w| {
        w.set_epena(true);
        w.set_cnak(true);
    });
    // wait for transfer complete interrupt
    poll_fn(|cx| {
        state().ep_in_wakers[0].register(cx.waker());
        // defmt::info!("write0 poll_fn with
        if r.diepint(0).read().xfrc() {
            r.diepint(0).write(|w| w.set_xfrc(true)); // clear xfrc
            r.diepmsk().modify(|w| w.set_xfrcm(true));
            // unmask
            trace!("write done len={}", buf.len());
            Poll::Ready(())
        } else {
            Poll::Pending
        }
    })
        .await;
    trace!("write len={} done", buf.len());
}
// #[embassy_executor::task]
pub async fn cdc_acm_ep2_read() {
    defmt::info!("cdc_acm_ep2_read start");
    let ep2_in = Endpoint::new(crate::usb_otg_hs::endpoint_new::Direction::In, 2, EpType::Bulk, MaxPacketSize::Size64, 0).unwrap();
    // let mut buf : [u8; 16] =
    //     let buf = "Hello, World!".as_bytes();
    // generate a buf with 100_000  bytes
    let mut buf = [08u8; 30_000];
    // last byte is 0
    buf[29_998] = 0;
    buf[29_999] = 0;

        ep2_in.write(&buf).await;
        defmt::info!("ep2 write done, data={:x}", buf);
}

#[embassy_executor::task]
pub async fn setup_process() {
    // this only enabled after reset and power up
    let mut buf = [0u8; 8];
    loop {
        unsafe {
            read0(&mut setup_data[0..8]).await; // status stage no data
            defmt::info!("wait for setup packet ready");
            defmt::info!("doepctl0: {:x}", regs().doepctl(0).read().0);
            defmt::info!("doeptsiz0: {:x}", regs().doeptsiz(0).read().0);
            poll_fn(|cx| {
                state().ep_out_wakers[0].register(cx.waker());
                if state().ep0_setup_ready.load(Ordering::Relaxed) {
                    state().ep0_setup_ready.store(false, Ordering::Release);
                    // regs().doepint(0).write(|w| w.0 = 0xFFFF_FFFF);
                    Poll::Ready(())
                } else {
                    Poll::Pending
                }
            })
                .await;
            // regs().doepctl(0).modify(|v| v.set_snak(true));
            defmt::info!( "setup packet ready, processing package **********{:x}", setup_data
            );
            // let (res, size) = process_setup_packet(&setup_data);
            let mut tmp = process_setup_packet_new(&setup_data[0..8]);
            if tmp.has_data_stage {
                match tmp.data_stage_direction {
                    Direction::In => {
                        write0(&tmp.data[0..tmp.len]).await;
                        read0(&mut buf[0..0]).await; // status stage no data
                        continue;
                    }
                    Direction::Out => {
                        read0(&mut tmp.data[0..tmp.len]).await;
                        write0(&[0u8; 0]).await; // status stage no data
                    }
                }
            } else {
                // status stage no data
                match tmp.setup.direction {
                    Direction::In => {
                        read0(&mut buf[0..0]).await; // status stage no data
                    }
                    Direction::Out => {
                        write0(&[0u8; 0]).await; // status stage no data
                    }
                }
            }
            // end of data stage

            match tmp.request {
                Request::SetAddress(addr) => {
                    init_setaddress(addr);
                }
                Request::SetConfiguration(_) => {
                    // not sure what it is
                    // do nothing here
                    defmt::info!("SetConfiguration");
                }
                Request::SetLineCoding(_) => {
                    // not sure what it is
                    // do nothing here
                    defmt::info!("SetLineCoding");
                }
                Request::SetControlLineState(_) => {
                    // not sure what it is
                    // do nothing here
                    defmt::info!("SetControlLineState");
                }
                Request::ClearFeature(_) => {
                    // not sure what it is
                    // do nothing here
                    defmt::info!("ClearFeature");
                }
                _ => {
                    defmt::error!("Unknown request");
                }
            }

            // defmt::info!("process_setup packet  res: {}", res);
            // if res {
            //     // send data
            //     unsafe {
            //         write0(&setup_return_data[0..size]).await;
            //     }
            //     read0(&mut buf[0..0]).await; // status stage no data
            // } else {
            //     write0(&[0u8; 0]).await; // status stage no data
            // }
        }
    }
}

pub unsafe fn on_interrupt() {
    let r = regs();
    // r.gahbcfg().modify(|w| w.set_dmaen(val));
    // r.gintmsk().write(|_w| {});
    defmt::info!("OTG_HS interrupt with ints {:08x}  and mask {:08x}, and {:08x}", r.gintsts().read().0, r.gintmsk().read().0, r.gintsts().read().0 & r.gintmsk().read().0);
    let ints = r.gintsts().read();
    if ints.wkupint() || ints.usbsusp() || ints.enumdne() || ints.otgint() || ints.srqint() || ints.usbrst()
    {
        if ints.wkupint() {
            info!("wkupint");
            r.gintsts().write(|w| w.set_wkupint(true)); // clear
        }
        else if  ints.usbsusp() {
            info!("usbsusp");
            r.gintsts().write(|w| w.set_usbsusp(true)); // clear
        }
        else if ints.enumdne() {
            info!("enumdne");
            init_enumeration_done();

            r.gintsts().write(|w| w.set_enumdne(true)); // clear
        }
        else if ints.otgint() {
            info!("otgint");
            let otgints = r.gotgint().read();
            r.gotgint().write_value(otgints); // clear all
        }
        else if ints.srqint() {
            info!("srqint");
            r.gintsts().write(|w| w.set_srqint(true)); // clear
        }
        else if ints.usbrst() {
            info!("usbrst");
            init_reset();
            r.gintsts().write(|w| w.set_usbrst(true)); // clear
        }
    }
    // let state = &STATE;
    let state: &mut State<6> = state();

    // Handle RX
    while r.gintsts().read().rxflvl() {
        // RX FIFO non-empty
        let status = r.grxstsp().read();
        // status read and popo pop register
        let ep_num = status.epnum() as usize;
        let len = status.bcnt() as usize;
        info!("rxflvl with ep_num: {}, len: {}", ep_num, len);

        match status.pktstsd() {
            stm32_metapac::otg::vals::Pktstsd::SETUP_DATA_RX => {
                // get setup_data
                let data: u32= r.fifo(0).read().0;
                let data2: u32 = r.fifo(0).read().0;
                for i in 0..4 {
                    setup_data[i] = (data >> (i * 8)) as u8;
                    setup_data[i + 4] = (data2 >> (i * 8)) as u8;
                }
                trace!("SETUP_DATA_RX, with data {:x}, {:x}, {:x}", data, data2, setup_data[0..8]);
                state.ep_out_wakers[ep_num].wake();
                state.ep0_setup_ready.store(true, Ordering::Release);
            }
            stm32_metapac::otg::vals::Pktstsd::OUT_DATA_RX => {
                // received OUT data
                state.ep_out_size[ep_num].store(len as u16, Ordering::Release);
                state.ep_out_wakers[ep_num].wake();
                let len_words = (len + 3) / 4;
                let mut data = [0u8; 64];
                let mut index = 0;
                for _ in 0..len_words {
                    let tmp = r.fifo(0).read().data();
                    for i in 0..4 {
                        data[index] = (tmp >> (i * 8)) as u8;
                        index += 1;
                    }
                }
                trace!("OUT_DATA_RX ep={} len={}, data={:x}", ep_num, len, data[0..len]);
            }
            stm32_metapac::otg::vals::Pktstsd::OUT_DATA_DONE => {
                trace!("OUT_DATA_DONE ep={}", ep_num);
                r.doepctl(0).modify(|w| w.set_cnak(true));
            }
            stm32_metapac::otg::vals::Pktstsd::SETUP_DATA_DONE => {
                trace!("SETUP_DATA_DONE ep={}", ep_num);
                r.doepctl(0).modify(|w| w.set_cnak(true));
            }
            x => {
                trace!("unknown PKTSTS: {}", x.to_bits());
            }
        }
    }

    // IN endpoint interrupt
    if ints.iepint() {
        info!("iepint");
        let mut ep_mask = r.daint().read().iepint();
        let mut ep_num = 0;

        // Iterate over endpoints while there are non-zero bits in the mask
        while ep_mask != 0 {
            if ep_mask & 1 != 0 {
                // get the interrupt mask
                info!(
                    "device in endpoint interrupt mask: {:08x}",
                    r.diepmsk().read().0
                );

                // mask
                r.diepmsk().modify(|w| w.set_xfrcm(false));
                // r.diepmsk().modify(|w| w.set_tom(false));
                let ep_ints = r.diepint(ep_num).read();
                r.diepint(ep_num).write(|w| w.set_toc(true));

                // clear all
                // r.diepint(ep_num).write_value(ep_ints);

                // TXFE is cleared in DIEPEMPMSK
                // if ep_ints.txfe() {
                //     critical_section::with(|_| {
                //         r.diepempmsk().modify(|w| {
                //             w.set_ineptxfem(w.ineptxfem() & !(1 << ep_num));
                //         });
                //     });
                // }
                state.ep_in_wakers[ep_num].wake();
                trace!("in ep={} irq val={:08x}", ep_num, ep_ints.0);
            }

            ep_mask >>= 1;
            ep_num += 1;
        }
    }


    // OUT endpoint interrupt
    if ints.oepint() {
        let mut ep_mask = r.daint().read().oepint();
        let mut ep_num = 0;
        while ep_mask != 0 {
            if ep_mask & 1 != 0 {
                // show setup package

                defmt::info!("------------------------------------------");
                defmt::info!("oepint, ep_num: {},  intsts: {:08x}", ep_num, r.doepint(ep_num).read().0);
                defmt::info!("setup data: {:x}", setup_data);
                defmt::info!("doepctl0: {:x}", regs().doepctl(0).read().0);
                defmt::info!("doeptsiz0: {:x}", regs().doeptsiz(0).read().0);
                let ep_ints = r.doepint(ep_num).read();
                if ep_ints.stup() {
                    state.ep_out_wakers[ep_num].wake();
                    state.ep0_setup_ready.store(true, Ordering::Release);
                    r.doepint(ep_num).write(|w| w.set_stup(true));
                    defmt::info!("setup package");
                }
                else if ep_ints.stsphsrx() {
                    // let status = r.grxstsp().read();
                    r.doepint(ep_num).write(|w| w.set_stsphsrx(true));
                    defmt::info!("clear stsphsrx+++++++++++++++++++++++++++++++++++++++");
                }
                // donothing if is setup and xfrc
                // clear all
                // r.doepint(ep_num).write_value(ep_ints);

                if ep_ints.xfrc() {
                    r.doepmsk().modify(|w| w.set_xfrcm(false)); // mask the interrupt and wake up the waker
                    state.ep_out_wakers[ep_num].wake();
                    // pop ?

                }
            }
            ep_mask >>= 1;
            ep_num += 1;
        }
    }
}


#[cfg(stm32u5a5)]
#[interrupt]
fn OTG_HS() {
    // GREEN.toggle();
    // defmt::info!("OTG_HS interrupt");
    unsafe {
        on_interrupt();
        // clock::delay_us(200);
    }
}
