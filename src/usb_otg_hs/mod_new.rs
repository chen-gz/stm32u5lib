use core::future::poll_fn;
use core::sync::atomic::Ordering;
use core::task::Poll;
use cortex_m::peripheral::NVIC;
use defmt::export::{panic, write};
use defmt::{info, trace};
use stm32_metapac::{
    otg::{self, regs::Doepctl},
    PWR, RCC, SYSCFG,
};

use crate::usb_otg_hs::{regs, restore_irqs, state, RX_FIFO_SIZE_SIZE_WORD, TX_FIFO_SIZE_WORDS};

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

pub struct Endpoint {}

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
    // 4 and 5 in RM0456 Rev 5, p3423
    // goto setup processing stage
    // r.doepdma(0)
    //     .write(|w| w.set_dmaaddr(unsafe { &setup_data as *const u8 as u32 }));
    // r.doeptsiz(0).write(|w| {
    //     w.set_stupcnt(3);
    //     w.set_pktcnt(1);
    //     w.set_xfrsiz(8);
    // });
}

pub static mut setup_data: [u8; 8] = [0; 8];
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
    restore_irqs();
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


pub fn process_setup_packet(buf: &[u8; 8]) -> (bool, usize) {
    unsafe {

        // get the setup packet
        // let setup = usb_device::control::Request::parse(&buf).unwrap();
        // let buf = [0u8; 64];

        let rt = buf[0];
        let recipient = rt & 0b11111;

        let setup = usb_device::control::Request {
            direction: rt.into(),
            // request_type: unsafe { mem::transmute((rt >> 5) & 0b11) },
            request_type: if (rt >> 5) & 0b11 == 0 {
                RequestType::Standard
            } else if (rt >> 5) & 0b11 == 1 {
                RequestType::Class
            } else if (rt >> 5) & 0b11 == 2 {
                RequestType::Vendor
            } else {
                RequestType::Reserved
            },
            recipient: if recipient <= 3 {
                // unsafe { mem::transmute(recipient) }
                if recipient == 0 {
                    Recipient::Device
                } else if recipient == 1 {
                    Recipient::Interface
                } else if recipient == 2 {
                    Recipient::Endpoint
                } else {
                    Recipient::Other
                }
            } else {
                Recipient::Reserved
            },
            request: buf[1],
            value: (buf[2] as u16) | ((buf[3] as u16) << 8),
            index: (buf[4] as u16) | ((buf[5] as u16) << 8),
            length: (buf[6] as u16) | ((buf[7] as u16) << 8),
        };
        match setup.direction {
            UsbDirection::Out => {
                match setup.request_type {
                    RequestType::Standard => {
                        match setup.recipient {
                            Recipient::Device => {
                                // set address
                                init_setaddress(setup.value as u8);
                            }
                            // Recipient::Interface => {}
                            // Recipient::Endpoint => {}
                            // Recipient::Other => {}
                            // Recipient::Reserved => {}
                            _ => {
                                defmt::panic!("Recipient not supported");
                            }
                        }
                    }
                    // RequestType::Class => {}
                    // RequestType::Vendor => {}
                    // RequestType::Reserved => {}
                    _ => {
                        defmt::panic!("Request type not supported");
                    }
                }
            }
            UsbDirection::In => {
                match setup.request_type {
                    RequestType::Standard => {
                        match setup.recipient {
                            Recipient::Device => {
                                match setup.request {
                                    usb_device::control::Request::GET_DESCRIPTOR => {
                                        match setup.value {
                                            0x0100 => {
                                                let tmp = USB_CDC_ACM_DEVICE_DESCRIPTOR.as_bytes();
                                                unsafe {
                                                    for i in 0..18 {
                                                        setup_return_data[i] = tmp[i];
                                                    }
                                                }

                                                return (true, core::cmp::min(18, setup.value) as usize);
                                            }
                                            0x0200 => {
                                                //get configuration descriptor
                                                let desc = CDC_CONIG_DESC_FULL();
                                                let len = core::cmp::min(setup.length as usize, desc.len());
                                                unsafe {
                                                    for i in 0..len {
                                                        setup_return_data[i] = desc[i];
                                                    }
                                                }
                                                return (true, len);

                                                // return (true, setup.length as usize);
                                            }
                                            0x0300 => {
                                                // get string descriptor
                                                match setup.index {
                                                    // language descriptor
                                                    0x0000 => {
                                                        let val = StringDescriptor::language();
                                                        for i in 0..val[0] {
                                                            setup_return_data[i] = val[i];
                                                        }
                                                        return (true, val[0] as usize);
                                                        let tmp = [0x04, 0x03, 0x09, 0x04];
                                                        unsafe {
                                                            for i in 0..4 {
                                                                setup_return_data[i] = tmp[i];
                                                            }
                                                        }
                                                        return (true, 4);
                                                    }

                                                    _ => {
                                                        // defmt::error!("Unknown string descriptor index, {}", setup.index);
                                                        defmt::panic!("Unknown string descriptor index, {:x}", setup.index);
                                                    }
                                                }
                                            }
                                            0x0301 => {
                                                // string descriptor for manufacturer string descriptor
                                                // USB String Descriptor for "Rust" in Rust
                                                let usb_string_descriptor: [u8; 10] = [
                                                    14, // bLength: Length of descriptor in bytes
                                                    0x03, // bDescriptorType: String descriptor type
                                                    // UTF-16LE encoding of "Rust"
                                                    0x52, 0x00, // 'R'
                                                    0x75, 0x00, // 'u'
                                                    0x73, 0x00, // 's'
                                                    0x74, 0x00, // 't'
                                                ];
                                                unsafe {
                                                    for i in 0..10 {
                                                        setup_return_data[i] = usb_string_descriptor[i];
                                                    }
                                                }
                                                return (true, 10);
                                            }
                                            0x0302 => {
                                                let val = StringDescriptor::product("USB Example Device");
                                                for i in 0..val[0] {
                                                    setup_return_data[i] = val[i];
                                                }
                                                return (true, val[0] as usize);
                                                // string descriptor for product string descriptor
                                                // USB String Descriptor for "USB Example Device" in Rust
                                                let usb_string_descriptor: [u8; 38] = [
                                                    38, // bLength: Length of descriptor in bytes
                                                    0x03, // bDescriptorType: String descriptor type
                                                    // UTF-16LE encoding of "USB Example Device"
                                                    0x55, 0x00, // 'U'
                                                    0x53, 0x00, // 'S'
                                                    0x42, 0x00, // 'B'
                                                    0x20, 0x00, // ' '
                                                    0x45, 0x00, // 'E'
                                                    0x78, 0x00, // 'x'
                                                    0x61, 0x00, // 'a'
                                                    0x6D, 0x00, // 'm'
                                                    0x70, 0x00, // 'p'
                                                    0x6C, 0x00, // 'l'
                                                    0x65, 0x00, // 'e'
                                                    0x20, 0x00, // ' '
                                                    0x44, 0x00, // 'D'
                                                    0x65, 0x00, // 'e'
                                                    0x76, 0x00, // 'v'
                                                    0x69, 0x00, // 'i'
                                                    0x63, 0x00, // 'c'
                                                    0x65, 0x00, // 'e'
                                                ];
                                                unsafe {
                                                    for i in 0..38 {
                                                        setup_return_data[i] = usb_string_descriptor[i];
                                                    }
                                                }
                                                return (true, 38);
                                            }
                                            0x0303 => {
                                                let val = StringDescriptor::serial_number("123456");
                                                // copy the serial number string descriptor to the setup_return_data
                                                for i in 0..val[0] {
                                                    setup_return_data[i] = val[i];
                                                }
                                                return (true, val[0] as usize);
                                                // serial number string descriptor
                                                // USB String Descriptor for "123456" in Rust
                                                let usb_string_descriptor: [u8; 14] = [
                                                    14, // bLength: Length of descriptor in bytes
                                                    0x03, // bDescriptorType: String descriptor type
                                                    // UTF-16LE encoding of "123456"
                                                    0x31, 0x00, // '1'
                                                    0x32, 0x00, // '2'
                                                    0x33, 0x00, // '3'
                                                    0x34, 0x00, // '4'
                                                    0x35, 0x00, // '5'
                                                    0x36, 0x00, // '6'
                                                ];
                                                unsafe {
                                                    for i in 0..14 {
                                                        setup_return_data[i] = usb_string_descriptor[i];
                                                    }
                                                }
                                                return (true, 14);
                                            }
                                            0x0600 => {
                                                // get device qualifier descriptor
                                                let device_qualifier_descriptor = DeviceQualifierDescriptor {
                                                    b_length: 10,             // Descriptor size is 10 bytes
                                                    b_descriptor_type: 6,     // Device qualifier descriptor type is 6
                                                    bcd_usb: 0x0110,          // USB 2.0
                                                    b_device_class: 0x00,     // Defined at interface level
                                                    b_device_sub_class: 0x00, // No subclass
                                                    b_device_protocol: 0x00,  // No protocol
                                                    b_max_packet_size0: 64, // 64 bytes for endpoint zero (typical for USB 2.0)
                                                    b_num_configurations: 1, // One configuration
                                                    b_reserved: 0, // Reserved
                                                };
                                                let tmp = device_qualifier_descriptor.as_bytes();
                                                unsafe {
                                                    for i in 0..10 {
                                                        setup_return_data[i] = tmp[i];
                                                    }
                                                }
                                                return (true, 10);
                                            }
                                            _ => {
                                                // defmt::error!("Unknown request value, {}", setup.value);
                                                defmt::panic!("Unknown request value, {:x}", setup.value);
                                            }
                                        }
                                    }
                                    _ => {
                                        panic!("Unknown request");
                                    }
                                }
                            }
                            // Recipient::Interface => {}
                            // Recipient::Endpoint => {}
                            // Recipient::Other => {}
                            // Recipient::Reserved => {}
                            _ => {
                                defmt::panic!("Recipient not supported");
                            }
                        }
                    }
                    // RequestType::Class => {}
                    // RequestType::Vendor => {}
                    // RequestType::Reserved => {}
                    _ => {
                        defmt::panic!("Request type not supported");
                    }
                }
            }
        }
        return (false, 0);
    }
}

pub struct EndpointGG;

async fn read0(buf: &mut [u8]) {
    trace!("read start len={}", buf.len());
    let r = regs();

    r.doepdma(0)
        .write(|w| unsafe { w.set_dmaaddr(setup_data.as_ptr() as u32) });
    info!("doepdma0: {:x}", r.doepdma(0).read().0);

    r.doeptsiz(0).modify(|w| {
        w.set_xfrsiz(buf.len() as _);
        w.set_pktcnt(1);
        w.set_stupcnt(3);
    });

    // for dma this is required
    r.doepctl(0).modify(|w| {
        w.set_epena(true);
        w.set_cnak(true);
    });
    // wait for transfer complete interrupt
    poll_fn(|cx| {
        state().ep_out_wakers[0].register(cx.waker());
        if r.doepint(0).read().xfrc() {
            r.doepint(0).write(|w| w.set_xfrc(true)); // clear xfrc
            r.doepmsk().modify(|w| w.set_xfrcm(true)); // unmask
            Poll::Ready(())
        } else {
            Poll::Pending
        }
    })
        .await;
}

async fn write0(buf: &[u8]) {
    trace!("write start len={}", buf.len());
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
            r.diepmsk().modify(|w| w.set_xfrcm(true)); // unmask
            Poll::Ready(())
        } else {
            Poll::Pending
        }
    })
        .await;
    trace!("write len={} done", buf.len());
}

#[embassy_executor::task]
pub async fn setup_process() {
    // wait for STUP interrupt
    // process interrupt
    let mut buf = [0u8; 8];
    loop {
        unsafe {
            defmt::info!("wait for setup packet transfer");
            read0(&mut setup_data).await;
            // wait for xfrc interrupt
            // wait for a setup packet
            defmt::info!("wait for setup packet ready");
            poll_fn(|cx| {
                state().ep_out_wakers[0].register(cx.waker());
                if state().ep0_setup_ready.load(Ordering::Relaxed) {
                    state().ep0_setup_ready.store(false, Ordering::Release);
                    Poll::Ready(())
                } else {
                    Poll::Pending
                }
            })
                .await;

            defmt::info!(
                "setup packet ready, processing package **********{:x}",
                setup_data
            );
            let (res, size) = process_setup_packet(&setup_data);
            defmt::info!("process_setup packet  res: {}", res);
            if res {
                // send data
                unsafe {
                    write0(&setup_return_data[0..size]).await;
                }
                read0(&mut buf[0..0]).await; // status stage no data
            } else {
                write0(&[0u8; 0]).await; // status stage no data
            }
        }
    }
}
