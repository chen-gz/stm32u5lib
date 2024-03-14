use core::future::poll_fn;
// use std::sync::atomic::Ordering;
// use std::task::Poll;
use core::task::Poll;

/// USB bus.
use cortex_m::peripheral::NVIC;
// use crate::usb_otg::usb::{EndpointData, PhyType, regs};
use defmt::trace;
use embassy_usb_driver::{Direction, EndpointAddress, EndpointIn, EndpointOut, EndpointType, Event, Unsupported};
use stm32_metapac::otg;

use crate::usb_otg::{Config, ENDPOINT_COUNT, EndpointData, FIFO_DEPTH_WORDS, MAX_EP_COUNT, PhyType, regs, RX_FIFO_EXTRA_SIZE_WORDS, state};

macro_rules! trace_bus_event {
    ($($arg:tt)*) => {
        defmt::trace!($($arg)*)
    };
}

pub struct Bus {
    pub(crate) config: Config,
    pub(crate) ep_in: [Option<EndpointData>; MAX_EP_COUNT],
    pub(crate) ep_out: [Option<EndpointData>; MAX_EP_COUNT],
    pub(crate) phy_type: PhyType,
    pub(crate) inited: bool,
}

pub(crate) fn start_irq() {
    regs().gintmsk().write(|w| {
        w.set_usbrst(true);
        w.set_enumdnem(true);
        w.set_usbsuspm(true);
        w.set_wuim(true);
        w.set_iepint(true);
        w.set_oepint(true);
        w.set_rxflvlm(true);
        w.set_srqim(true);
        w.set_otgint(true);
    });
}


impl Bus {
    fn restore_irqs() {
        regs().gintmsk().write(|w| {
            w.set_usbrst(true);
            w.set_enumdnem(true);
            w.set_usbsuspm(true);
            w.set_wuim(true);
            w.set_iepint(true);
            w.set_oepint(true);
            w.set_rxflvlm(true);
            w.set_srqim(true);
            w.set_otgint(true);
        });
    }
}

impl Bus {
    fn init(&mut self) {
        // Enable USB power
        critical_section::with(|_| {
            stm32_metapac::PWR.svmcr().modify(|w| {
                w.set_usv(true);
                w.set_uvmen(true);
            });
        });
        // Wait for USB power to stabilize
        while !stm32_metapac::PWR.svmsr().read().vddusbrdy() {}
        defmt::trace!("USB power stabilized");

        // Select HSI48 as USB clock source.
        critical_section::with(|_| {
            stm32_metapac::RCC.ccipr1().modify(|w| {
                w.set_iclksel(stm32_metapac::rcc::vals::Iclksel::HSI48);
            })
        });
        stm32_metapac::RCC.ahb2enr1()
            .modify(|w| w.set_usb_otg_fsen(true));
        unsafe {
            NVIC::unpend(stm32_metapac::Interrupt::OTG_FS);
            NVIC::unmask(stm32_metapac::Interrupt::OTG_FS);
            start_irq();
            trace!("USB IRQs start");
        }

        let r = regs();
        let core_id = r.cid().read().0;
        trace!("Core id {:08x}", core_id);

        // Wait for AHB ready.
        while !r.grstctl().read().ahbidl() {}

        r.gusbcfg().write(|w| {
            w.set_fdmod(true);  // Force device mode TODO: no host mode support
            w.set_physel(self.phy_type.internal() && !self.phy_type.high_speed());  // Enable internal full-speed PHY
        });

        // Configuring Vbus sense and SOF output
        match core_id { // this is used to distinguish differnet stm32 chips
            0x0000_1200 | 0x0000_1100 => {
                assert!(self.phy_type != PhyType::InternalHighSpeed);

                r.gccfg_v1().modify(|w| {
                    // Enable internal full-speed PHY, logic is inverted
                    w.set_pwrdwn(self.phy_type.internal());
                });

                // F429-like chips have the GCCFG.NOVBUSSENS bit
                r.gccfg_v1().modify(|w| {
                    w.set_novbussens(!self.config.vbus_detection);
                    // w.set_novbussens(!false);
                    w.set_vbusasen(false);
                    w.set_vbusbsen(self.config.vbus_detection);
                    w.set_sofouten(false);
                });
            }
            0x0000_2000 | 0x0000_2100 | 0x0000_2300 | 0x0000_3000 | 0x0000_3100 => {
                // F446-like chips have the GCCFG.VBDEN bit with the opposite meaning
                r.gccfg_v2().modify(|w| {
                    // Enable internal full-speed PHY, logic is inverted
                    w.set_pwrdwn(self.phy_type.internal() && !self.phy_type.high_speed());
                    w.set_phyhsen(self.phy_type.internal() && self.phy_type.high_speed());
                });

                r.gccfg_v2().modify(|w| {
                    w.set_vbden(self.config.vbus_detection);
                });

                // Force B-peripheral session
                r.gotgctl().modify(|w| {
                    w.set_bvaloen(!self.config.vbus_detection);
                    w.set_bvaloval(true);
                });
            }
            _ => unimplemented!("Unknown USB core id {:X}", core_id),
        }

        // Soft disconnect.
        r.dctl().write(|w| w.set_sdis(true));

        // Set speed.
        r.dcfg().write(|w| {
            w.set_pfivl(otg::vals::Pfivl::FRAME_INTERVAL_80); // set period frame interval TODO: figure out what is this
            w.set_dspd(self.phy_type.to_dspd());
        });

        r.diepmsk().write(|w| {
            w.set_xfrcm(true);    // Unmask transfer complete EP interrupt
        });

        // Unmask and clear core interrupts
        // Bus::<T>::restore_irqs();
        // r.gintsts().write_value(regs::Gintsts(0xFFFF_FFFF));
        r.gintsts().write_value(stm32_metapac::otg::regs::Gintsts(0xFFFF_FFFF));

        // Unmask global interrupt
        r.gahbcfg().write(|w| {
            w.set_gint(true); // unmask global interrupt
        });

        // Connect
        r.dctl().write(|w| w.set_sdis(false));
    }

    fn init_fifo(&mut self) {
        trace_bus_event!("init_fifo");

        // let r = T::regs();
        let r = regs();

        // Configure RX fifo size. All endpoints share the same FIFO area.
        let rx_fifo_size_words = RX_FIFO_EXTRA_SIZE_WORDS + ep_fifo_size(&self.ep_out);
        trace!("configuring rx fifo size={}", rx_fifo_size_words);

        let rx_fifo_size_words = 64;

        r.grxfsiz().modify(|w| w.set_rxfd(rx_fifo_size_words));

        // Configure TX (USB in direction) fifo size for each endpoint
        let mut fifo_top = rx_fifo_size_words;
        for i in 0..ENDPOINT_COUNT {
            if let Some(ep) = self.ep_in[i] {
                trace!(
                    "configuring tx fifo ep={}, offset={}, size={}",
                    i,
                    fifo_top,
                    ep.fifo_size_words
                );

                let dieptxf = if i == 0 {
                    r.dieptxf0()
                } else {
                    r.dieptxf(i - 1)
                };

                dieptxf.write(|w| {
                    w.set_fd(ep.fifo_size_words);
                    w.set_sa(fifo_top);
                });

                fifo_top += ep.fifo_size_words;
            }
        }

        assert!(
            fifo_top <= FIFO_DEPTH_WORDS,
            "FIFO allocations exceeded maximum capacity"
        );

        // Flush fifos
        r.grstctl().write(|w| {
            w.set_rxfflsh(true);
            w.set_txfflsh(true);
            w.set_txfnum(0x10);
        });
        loop {
            let x = r.grstctl().read();
            if !x.rxfflsh() && !x.txfflsh() {
                break;
            }
        }
    }

    fn configure_endpoints(&mut self) {
        trace_bus_event!("configure_endpoints");

        // let r = T::regs();
        let r = regs();

        // Configure IN endpoints
        for (index, ep) in self.ep_in.iter().enumerate() {
            if let Some(ep) = ep {
                critical_section::with(|_| {
                    r.diepctl(index).write(|w| {
                        if index == 0 {
                            w.set_mpsiz(ep0_mpsiz(ep.max_packet_size));
                        } else {
                            w.set_mpsiz(ep.max_packet_size);
                            w.set_eptyp(to_eptyp(ep.ep_type));
                            w.set_sd0pid_sevnfrm(true);
                            w.set_txfnum(index as _);
                            w.set_snak(true);
                        }
                    });
                });
            }
        }

        // Configure OUT endpoints
        for (index, ep) in self.ep_out.iter().enumerate() {
            if let Some(ep) = ep {
                critical_section::with(|_| {
                    r.doepctl(index).write(|w| {
                        if index == 0 {
                            w.set_mpsiz(ep0_mpsiz(ep.max_packet_size));
                        } else {
                            w.set_mpsiz(ep.max_packet_size);
                            w.set_eptyp(to_eptyp(ep.ep_type));
                            w.set_sd0pid_sevnfrm(true);
                        }
                    });

                    r.doeptsiz(index).modify(|w| {
                        w.set_xfrsiz(ep.max_packet_size as _);
                        if index == 0 {
                            w.set_rxdpid_stupcnt(1);
                        } else {
                            w.set_pktcnt(1);
                        }
                    });
                });
            }
        }

        // Enable IRQs for allocated endpoints
        r.daintmsk().modify(|w| {
            w.set_iepm(ep_irq_mask(&self.ep_in));
            // OUT interrupts not used, handled in RXFLVL
            // w.set_oepm(ep_irq_mask(&self.ep_out));
        });
    }

    fn disable_all_endpoints(&mut self) {
        trace_bus_event!("disable_all_endpoints in bus self");
        todo!("check why this call! Low level should not call this")
        // for i in 0..ENDPOINT_COUNT {
            // self.endpoint_set_enabled(EndpointAddress::from_parts(i, Direction::In), false);
            // self.endpoint_set_enabled(EndpointAddress::from_parts(i, Direction::Out), false);
        // }
    }

    fn disable(&mut self) {
        trace_bus_event!("disable in bus self");

        // Interrupt::disable();
        unsafe {
            NVIC::mask(stm32_metapac::Interrupt::OTG_FS);
        }


        // TODO: disable USB peripheral
        // <T as RccPeripheral>::disable();

        #[cfg(stm32l4)]
        crate::pac::PWR.cr2().modify(|w| w.set_usv(false));
        // Cannot disable PWR, because other peripherals might be using it
    }
}

impl embassy_usb_driver::Bus for Bus {
    async fn enable(&mut self) {
        trace_bus_event!("bus call enable");

        // TODO: enable the peripheral once enable/disable semantics are cleared up in embassy-usb
        // clock::kernel_freq_160mhz_request();
    }

    async fn disable(&mut self) {
        trace_bus_event!("bus call disable");
        // clock::kernel_freq_160mhz_release();

        // TODO: disable the peripheral once enable/disable semantics are cleared up in embassy-usb
        //Bus::disable(self);
    }

    async fn poll(&mut self) -> Event {
        poll_fn(move |cx| {
            if !self.inited {
                self.init();
                self.inited = true;

                // If no vbus detection, just return a single PowerDetected event at startup.
                if !self.config.vbus_detection {
                    return Poll::Ready(Event::PowerDetected);
                }
            }

            // let r = T::regs();
            let r = regs();

            unsafe {

                state().bus_waker.register(cx.waker());
            }

            let ints = r.gintsts().read();

            if ints.srqint() {
                trace!("vbus detected");

                r.gintsts().write(|w| w.set_srqint(true)); // clear
                Self::restore_irqs();

                if self.config.vbus_detection {
                    return Poll::Ready(Event::PowerDetected);
                }
            }

            if ints.otgint() {
                let otgints = r.gotgint().read();
                r.gotgint().write_value(otgints); // clear all
                Self::restore_irqs();

                if otgints.sedet() {
                    trace!("vbus removed");
                    if self.config.vbus_detection {
                        self.disable_all_endpoints();
                        return Poll::Ready(Event::PowerRemoved);
                    }
                }
            }

            if ints.usbrst() {
                trace_bus_event!("reset");

                self.init_fifo();
                self.configure_endpoints();

                // Reset address
                critical_section::with(|_| {
                    r.dcfg().modify(|w| {
                        w.set_dad(0);
                    });
                });

                r.gintsts().write(|w| w.set_usbrst(true)); // clear
                Self::restore_irqs();
            }

            if ints.enumdne() {
                trace!("enumdne");

                let speed = r.dsts().read().enumspd();
                let trdt = calculate_trdt(speed, 160_000_000);
                //TODO: get HCLK frequency
                trace!("  speed={} trdt={}", speed.to_bits(), trdt);
                r.gusbcfg().modify(|w| w.set_trdt(trdt));

                r.gintsts().write(|w| w.set_enumdne(true)); // clear
                Self::restore_irqs();

                return Poll::Ready(Event::Reset);
            }

            if ints.usbsusp() {
                trace!("suspend");
                r.gintsts().write(|w| w.set_usbsusp(true)); // clear
                Self::restore_irqs();
                return Poll::Ready(Event::Suspend);
            }

            if ints.wkupint() {
                trace!("resume");
                r.gintsts().write(|w| w.set_wkupint(true)); // clear
                Self::restore_irqs();
                return Poll::Ready(Event::Resume);
            }

            Poll::Pending
        })
            .await
    }

    fn endpoint_set_enabled(&mut self, ep_addr: EndpointAddress, enabled: bool) {
        trace!("endpoint_set_enabled ep={:?} en={}", ep_addr, enabled);

        assert!(
            ep_addr.index() < ENDPOINT_COUNT,
            "endpoint_set_enabled index {} out of range",
            ep_addr.index()
        );

        let r = regs();
        match ep_addr.direction() {
            Direction::Out => {
                critical_section::with(|_| {
                    // cancel transfer if active
                    if !enabled && r.doepctl(ep_addr.index()).read().epena() {
                        r.doepctl(ep_addr.index()).modify(|w| {
                            w.set_snak(true);
                            w.set_epdis(true);
                        })
                    }

                    r.doepctl(ep_addr.index()).modify(|w| {
                        w.set_usbaep(enabled);
                    });

                    // Flush tx fifo
                    r.grstctl().write(|w| {
                        w.set_txfflsh(true);
                        w.set_txfnum(ep_addr.index() as _);
                    });
                    loop {
                        let x = r.grstctl().read();
                        if !x.txfflsh() {
                            break;
                        }
                    }
                });

                // Wake `Endpoint::wait_enabled()`
                // T::state().ep_out_wakers[ep_addr.index()].wake();
                unsafe {
                    state().ep_out_wakers[ep_addr.index()].wake();
                }
            }
            Direction::In => {
                critical_section::with(|_| {
                    // cancel transfer if active
                    if !enabled && r.diepctl(ep_addr.index()).read().epena() {
                        r.diepctl(ep_addr.index()).modify(|w| {
                            w.set_snak(true); // set NAK
                            w.set_epdis(true);
                        })
                    }

                    r.diepctl(ep_addr.index()).modify(|w| {
                        w.set_usbaep(enabled);
                        w.set_cnak(enabled); // clear NAK that might've been set by SNAK above.
                    })
                });

                // Wake `Endpoint::wait_enabled()`
                // T::state().ep_in_wakers[ep_addr.index()].wake();
                unsafe {
                    state().ep_in_wakers[ep_addr.index()].wake();
                }
            }
        }
    }

    fn endpoint_set_stalled(&mut self, ep_addr: EndpointAddress, stalled: bool) {
        trace_bus_event!("endpoint_set_stalled ep={:?} en={}", ep_addr, stalled);

        assert!(
            ep_addr.index() < ENDPOINT_COUNT,
            "endpoint_set_stalled index {} out of range",
            ep_addr.index()
        );

        // let regs = T::regs();
        let regs = regs();

        match ep_addr.direction() {
            Direction::Out => {
                critical_section::with(|_| {
                    regs.doepctl(ep_addr.index()).modify(|w| {
                        w.set_stall(stalled);
                    });
                });

                // T::state().ep_out_wakers[ep_addr.index()].wake();
                unsafe {
                    state().ep_out_wakers[ep_addr.index()].wake();
                }
            }
            Direction::In => {
                critical_section::with(|_| {
                    regs.diepctl(ep_addr.index()).modify(|w| {
                        w.set_stall(stalled);
                    });
                });

                // T::state().ep_in_wakers[ep_addr.index()].wake();
                unsafe {
                    state().ep_in_wakers[ep_addr.index()].wake();
                }
            }
        }
    }

    fn endpoint_is_stalled(&mut self, ep_addr: EndpointAddress) -> bool {
        trace!("endpoint_is_stalled ep={:?}", ep_addr);
        assert!(
            ep_addr.index() < ENDPOINT_COUNT,
            "endpoint_is_stalled index {} out of range",
            ep_addr.index()
        );

        let regs = regs();

        match ep_addr.direction() {
            Direction::Out => regs.doepctl(ep_addr.index()).read().stall(),
            Direction::In => regs.diepctl(ep_addr.index()).read().stall(),
        }
    }

    async fn remote_wakeup(&mut self) -> Result<(), Unsupported> {
        trace_bus_event!("bus call remote_wakeup");
        Err(Unsupported)
    }
}

// impl< T: Instance> Drop for Bus<'d, T> {
//     fn drop(&mut self) {
//         Bus::disable(self);
//     }
// }
impl Drop for Bus {
    fn drop(&mut self) {
        Bus::disable(self);
    }
}

trait Dir {
    fn dir() -> Direction;
}

//
// /// Marker type for the "IN" direction.
pub enum In {}

impl Dir for In {
    fn dir() -> Direction {
        Direction::In
    }
}

/// Marker type for the "OUT" direction.
pub enum Out {}

impl Dir for Out {
    fn dir() -> Direction {
        Direction::Out
    }
}



// /// Translates HAL [EndpointType] into PAC [vals::Eptyp]
fn to_eptyp(ep_type: EndpointType) -> otg::vals::Eptyp {
    match ep_type {
        EndpointType::Control => otg::vals::Eptyp::CONTROL,
        EndpointType::Isochronous => otg::vals::Eptyp::ISOCHRONOUS,
        EndpointType::Bulk => otg::vals::Eptyp::BULK,
        EndpointType::Interrupt => otg::vals::Eptyp::INTERRUPT,
    }
}

//
// /// Calculates total allocated FIFO size in words
fn ep_fifo_size(eps: &[Option<EndpointData>]) -> u16 {
    eps.iter()
        .map(|ep| ep.map(|ep| ep.fifo_size_words).unwrap_or(0))
        .sum()
}


/// Generates IRQ mask for enabled endpoints
fn ep_irq_mask(eps: &[Option<EndpointData>]) -> u16 {
    eps.iter().enumerate().fold(0, |mask, (index, ep)| {
        if ep.is_some() {
            mask | (1 << index)
        } else {
            mask
        }
    })
}

/// Calculates MPSIZ value for EP0, which uses special values.
fn ep0_mpsiz(max_packet_size: u16) -> u16 {
    match max_packet_size {
        8 => 0b11,
        16 => 0b10,
        32 => 0b01,
        64 => 0b00,
        other => panic!("Unsupported EP0 size: {}", other),
    }
}

//
fn calculate_trdt(speed: otg::vals::Dspd, ahb_freq: u32) -> u8 {
    match speed {
        otg::vals::Dspd::HIGH_SPEED => {
            // From RM0431 (F72xx), RM0090 (F429), RM0390 (F446)
            // if ahb_freq.0 >= 30_000_000 {
            //     0x9
            // } else {
            //     panic!("AHB frequency is too low")
            // }
            0x9
        }
        otg::vals::Dspd::FULL_SPEED_EXTERNAL | otg::vals::Dspd::FULL_SPEED_INTERNAL => {
            // From RM0431 (F72xx), RM0090 (F429)
            match ahb_freq {
                0..=14_199_999 => panic!("AHB frequency is too low"),
                14_200_000..=14_999_999 => 0xF,
                15_000_000..=15_999_999 => 0xE,
                16_000_000..=17_199_999 => 0xD,
                17_200_000..=18_499_999 => 0xC,
                18_500_000..=19_999_999 => 0xB,
                20_000_000..=21_799_999 => 0xA,
                21_800_000..=23_999_999 => 0x9,
                24_000_000..=27_499_999 => 0x8,
                27_500_000..=31_999_999 => 0x7, // 27.7..32 in code from CubeIDE
                32_000_000..=u32::MAX => 0x6,
            };
            0x6
        }
        _ => unimplemented!(),
    }
}



// //! USB On The Go (OTG)
//
// use crate::rcc::RccPeripheral;
// use crate::{interrupt, peripherals};
//
// mod usb;
// pub use usb::*;
//
// // Using Instance::ENDPOINT_COUNT requires feature(const_generic_expr) so just define maximum eps
//

// pub(crate) mod sealed {
//     pub trait Instance {
//         const HIGH_SPEED: bool;
//         const FIFO_DEPTH_WORDS: u16;
//         const ENDPOINT_COUNT: usize;
//
//         fn regs() -> crate::pac::otg::Otg;
//         fn state() -> &'static super::State<{ super::MAX_EP_COUNT }>;
//     }
// }
//
// /// USB OTG instance.
// pub trait Instance: sealed::Instance + RccPeripheral {
//     /// Interrupt for this USB OTG instance.
//     type Interrupt: interrupt::typelevel::Interrupt;
// }
//
// // Internal PHY pins
// pin_trait!(DpPin, Instance);
// pin_trait!(DmPin, Instance);
//
// // External PHY pins
// pin_trait!(UlpiClkPin, Instance);
// pin_trait!(UlpiDirPin, Instance);
// pin_trait!(UlpiNxtPin, Instance);
// pin_trait!(UlpiStpPin, Instance);
// pin_trait!(UlpiD0Pin, Instance);
// pin_trait!(UlpiD1Pin, Instance);
// pin_trait!(UlpiD2Pin, Instance);
// pin_trait!(UlpiD3Pin, Instance);
// pin_trait!(UlpiD4Pin, Instance);
// pin_trait!(UlpiD5Pin, Instance);
// pin_trait!(UlpiD6Pin, Instance);
// pin_trait!(UlpiD7Pin, Instance);
//
// foreach_interrupt!(
//     (USB_OTG_FS, otg, $block:ident, GLOBAL, $irq:ident) => {
//         impl sealed::Instance for peripherals::USB_OTG_FS {
//             const HIGH_SPEED: bool = false;
//
//             cfg_if::cfg_if! {
//                 if #[cfg(stm32f1)] {
//                     const FIFO_DEPTH_WORDS: u16 = 128;
//                     const ENDPOINT_COUNT: usize = 8;
//                 } else if #[cfg(any(
//                     stm32f2,
//                     stm32f401,
//                     stm32f405,
//                     stm32f407,
//                     stm32f411,
//                     stm32f415,
//                     stm32f417,
//                     stm32f427,
//                     stm32f429,
//                     stm32f437,
//                     stm32f439,
//                 ))] {
//                     const FIFO_DEPTH_WORDS: u16 = 320;
//                     const ENDPOINT_COUNT: usize = 4;
//                 } else if #[cfg(any(
//                     stm32f412,
//                     stm32f413,
//                     stm32f423,
//                     stm32f446,
//                     stm32f469,
//                     stm32f479,
//                     stm32f7,
//                     stm32l4,
//                     stm32u5,
//                 ))] {
//                     const FIFO_DEPTH_WORDS: u16 = 320;
//                     const ENDPOINT_COUNT: usize = 6;
//                 } else if #[cfg(stm32g0x1)] {
//                     const FIFO_DEPTH_WORDS: u16 = 512;
//                     const ENDPOINT_COUNT: usize = 8;
//                 } else if #[cfg(stm32h7)] {
//                     const FIFO_DEPTH_WORDS: u16 = 1024;
//                     const ENDPOINT_COUNT: usize = 9;
//                 } else if #[cfg(stm32u5)] {
//                     const FIFO_DEPTH_WORDS: u16 = 320;
//                     const ENDPOINT_COUNT: usize = 6;
//                 } else {
//                     compile_error!("USB_OTG_FS peripheral is not supported by this chip.");
//                 }
//             }
//
//             fn regs() -> crate::pac::otg::Otg {
//                 crate::pac::USB_OTG_FS
//             }
//
//                         fn state() -> &'static State<MAX_EP_COUNT> {
//                 static STATE: State<MAX_EP_COUNT> = State::new();
//                 &STATE
//             }
//         }
//
//         impl Instance for peripherals::USB_OTG_FS {
//             type Interrupt = crate::interrupt::typelevel::$irq;
//         }
//     };
//
//     (USB_OTG_HS, otg, $block:ident, GLOBAL, $irq:ident) => {
//         impl sealed::Instance for peripherals::USB_OTG_HS {
//             const HIGH_SPEED: bool = true;
//
//             cfg_if::cfg_if! {
//                 if #[cfg(any(
//                     stm32f2,
//                     stm32f405,
//                     stm32f407,
//                     stm32f415,
//                     stm32f417,
//                     stm32f427,
//                     stm32f429,
//                     stm32f437,
//                     stm32f439,
//                 ))] {
//                     const FIFO_DEPTH_WORDS: u16 = 1024;
//                     const ENDPOINT_COUNT: usize = 6;
//                 } else if #[cfg(any(
//                     stm32f446,
//                     stm32f469,
//                     stm32f479,
//                     stm32f7,
//                     stm32h7,
//                 ))] {
//                     const FIFO_DEPTH_WORDS: u16 = 1024;
//                     const ENDPOINT_COUNT: usize = 9;
//                 } else if #[cfg(stm32u5)] {
//                     const FIFO_DEPTH_WORDS: u16 = 1024;
//                     const ENDPOINT_COUNT: usize = 9;
//                 } else {
//                     compile_error!("USB_OTG_HS peripheral is not supported by this chip.");
//                 }
//             }
//
//             fn regs() -> crate::pac::otg::Otg {
//                 // OTG HS registers are a superset of FS registers
//                 unsafe { crate::pac::otg::Otg::from_ptr(crate::pac::USB_OTG_HS.as_ptr()) }
//             }
//
//                         fn state() -> &'static State<MAX_EP_COUNT> {
//                 static STATE: State<MAX_EP_COUNT> = State::new();
//                 &STATE
//             }
//         }
//
//         impl Instance for peripherals::USB_OTG_HS {
//             type Interrupt = crate::interrupt::typelevel::$irq;
//         }
//     };
// );
// use stm32_metapac::Interrupt::OTG_FS;