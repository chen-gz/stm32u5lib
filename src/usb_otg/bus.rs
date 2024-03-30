use core::future::poll_fn;
use core::task::Poll;

/// USB bus.
use cortex_m::peripheral::NVIC;
// use crate::usb_otg::usb::{EndpointData, PhyType, regs};
use defmt::{trace};
use embassy_usb_driver::{
    Direction, EndpointAddress, EndpointType, Event, Unsupported,
};
use stm32_metapac::otg;

use crate::usb_otg::fifo_const::*;
use crate::usb_otg::{regs, state, Config, EndpointData, PhyType, BUS_WAKER_PWR};

macro_rules! trace_bus_event {
    ($($arg:tt)*) => {
        defmt::info!($($arg)*)
    };
}

pub struct Bus {
    pub(crate) config: Config,
    // the endpoint here does not contain the control endpoints
    pub(crate) ep_in: [Option<EndpointData>; MAX_EP_COUNT],
    // the endpoint here does not contain the control endpoints
    pub(crate) ep_out: [Option<EndpointData>; MAX_EP_COUNT],
    pub(crate) phy_type: PhyType,
    pub(crate) inited: bool,
}

impl Bus {

    /// Initializes FIFOs based on the fifo_const.
    fn init_fifo(&mut self) {
        trace_bus_event!("init_fifo");

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
        loop {
            let x = r.grstctl().read();
            if !x.rxfflsh() && !x.txfflsh() {
                break;
            }
        }
    }

    /// Configures endpoints based on the endpoint data.
    /// This function will not activate the endpoints.
    fn configure_endpoints(&mut self) {
        trace_bus_event!("configure_endpoints");
        let r = regs();
        // configure control endpoint
        // the max_packet_size should be 8, 16, 32, 64
        let ep0_mpsiz: u16 = match self.ep_in[0] {
            Some(ep) => match ep.max_packet_size {
                8 => 0b11,
                16 => 0b10,
                32 => 0b01,
                64 => 0b00,
                _ => panic!("Unsupported EP0 size: {}", ep.max_packet_size),
            },
            None => panic!("Control endpoint not configured"),
        };
        r.diepctl(0).write(|w| {
            w.set_mpsiz(ep0_mpsiz);
            // w.set_usbaep(true); not needed for control endpoint
        });
        // todo: check if this is needed
        // r.dieptsiz(0).write(|w| {
        //     w.set_xfrsiz(self.ep_in[0].unwrap().max_packet_size as _);
        // });
        r.doepctl(0).write(|w| {
            w.set_mpsiz(ep0_mpsiz);
            // w.set_usbaep(true); not needed for control endpoint
        });
        r.doeptsiz(0).write(|w| {
            w.set_rxdpid_stupcnt(1);
            w.set_xfrsiz(self.ep_out[0].unwrap().max_packet_size as _);
        });
        r.daintmsk().modify(|w| {
            w.set_iepm(1);
        });

        // Configure IN endpoints

        for i in 1..self.ep_in.len() {
            if self.ep_in[i] == None {
                break;
            }
            let ep = self.ep_in[i].unwrap();
            critical_section::with(|_| {
                let ep_type = match ep.ep_type {
                    EndpointType::Control => otg::vals::Eptyp::CONTROL,
                    EndpointType::Isochronous => otg::vals::Eptyp::ISOCHRONOUS,
                    EndpointType::Bulk => otg::vals::Eptyp::BULK,
                    EndpointType::Interrupt => otg::vals::Eptyp::INTERRUPT,
                };
                r.diepctl(i).write(|w| {
                    w.set_mpsiz(ep.max_packet_size);
                    w.set_eptyp(ep_type);
                    w.set_sd0pid_sevnfrm(true);
                    w.set_txfnum(i as _);
                    w.set_snak(true);
                    // w.set_usbaep(true);
                });
                r.daintmsk().modify(|w| {
                    w.set_iepm(1 << i);
                }); // enable (in) interrupt for this endpoint
            });
        }

        // Configure OUT endpoints
        for i in 1..self.ep_out.len() {
            if self.ep_out[i] == None {
                break;
            }
            let ep = self.ep_out[i].unwrap();
            critical_section::with(|_| {
                let ep_type = match ep.ep_type {
                    EndpointType::Control => otg::vals::Eptyp::CONTROL,
                    EndpointType::Isochronous => otg::vals::Eptyp::ISOCHRONOUS,
                    EndpointType::Bulk => otg::vals::Eptyp::BULK,
                    EndpointType::Interrupt => otg::vals::Eptyp::INTERRUPT,
                };
                r.doepctl(i).write(|w| {
                    w.set_mpsiz(ep.max_packet_size);
                    w.set_eptyp(ep_type);
                    w.set_sd0pid_sevnfrm(true);
                });
                r.doeptsiz(i).write(|w| {
                    w.set_xfrsiz(ep.max_packet_size as _);
                    w.set_pktcnt(1);
                });
            });
        }

        // // Enable IRQs for allocated endpoints
        // r.daintmsk().modify(|w| {
        //     w.set_iepm(ep_irq_mask(&self.ep_in));
        //     // OUT interrupts not used, handled in RXFLVL
        //     // w.set_oepm(ep_irq_mask(&self.ep_out));
        // });
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
        // TODO: not review yet
        trace_bus_event!("disable in bus self");

        // Interrupt::disable();
        #[cfg(stm32u575)]
        unsafe {
            NVIC::mask(stm32_metapac::Interrupt::OTG_FS);
        }
        #[cfg(stm32u5a5)]
        NVIC::mask(stm32_metapac::Interrupt::OTG_HS);
        self.inited = false;
        // disable the power

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
            unsafe {
                state().bus_waker.register(cx.waker());
                BUS_WAKER_PWR.register(cx.waker());
            }
            defmt::info!("poll usb bus");
            // let vdd = stm32_metapac::PWR.svmsr().read().vddusbrdy();
            // if !vdd {
            //     return Poll::Pending;
            // }
            // defmt::info!("poll usb bus after vdd ready");
            if !self.inited {
                // self.init();
                // power_up_init();
                self.inited = true;

                // If no vbus detection, just return a single PowerDetected event at startup.
                if !self.config.vbus_detection {
                    return Poll::Ready(Event::PowerDetected);
                }
            }

            let r = regs();


            let ints = r.gintsts().read();

            if ints.srqint() {
                trace!("vbus detected");

                r.gintsts().write(|w| w.set_srqint(true)); // clear
                crate::usb_otg::restore_irqs();

                if self.config.vbus_detection {
                    return Poll::Ready(Event::PowerDetected);
                }
            }

            if ints.otgint() {
                let otgints = r.gotgint().read();
                r.gotgint().write_value(otgints); // clear all
                // Self::restore_irqs();

                crate::usb_otg::restore_irqs();

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
                // Self::restore_irqs();
                crate::usb_otg::restore_irqs();
            }


            if ints.enumdne() {
                trace!("enumdne");

                let speed = r.dsts().read().enumspd();
                let trdt = calculate_trdt(speed, 160_000_000);
                //TODO: get HCLK frequency
                trace!("  speed={} trdt={}", speed.to_bits(), trdt);
                #[cfg(stm32u575)]
                r.gusbcfg().modify(|w| w.set_trdt(trdt));
                #[cfg(stm32u5a5)]
                r.gusbcfg().modify(|w| w.set_trdt(0x09));

                r.gintsts().write(|w| w.set_enumdne(true)); // clear
                // Self::restore_irqs();

                crate::usb_otg::restore_irqs();
                return Poll::Ready(Event::Reset);
            }

            if ints.usbsusp() {
                trace!("suspend");
                r.gintsts().write(|w| w.set_usbsusp(true)); // clear
                // Self::restore_irqs();

                crate::usb_otg::restore_irqs();
                return Poll::Ready(Event::Suspend);
            }

            if ints.wkupint() {
                trace!("resume");
                r.gintsts().write(|w| w.set_wkupint(true)); // clear
                // Self::restore_irqs();
                crate::usb_otg::restore_irqs();
                return Poll::Ready(Event::Resume);
            }

            Poll::Pending
        })
            .await
    }

    fn endpoint_set_enabled(&mut self, ep_addr: EndpointAddress, enabled: bool) {
        trace_bus_event!("endpoint_set_enabled ep={:?} en={}", ep_addr, enabled);

        assert!(
            ep_addr.index() < MAX_EP_COUNT,
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
                state().ep_out_wakers[ep_addr.index()].wake();
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
                state().ep_in_wakers[ep_addr.index()].wake();
            }
        }
    }

    fn endpoint_set_stalled(&mut self, ep_addr: EndpointAddress, stalled: bool) {
        trace_bus_event!("endpoint_set_stalled ep={:?} en={}", ep_addr, stalled);

        assert!(
            ep_addr.index() < MAX_EP_COUNT,
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
                state().ep_out_wakers[ep_addr.index()].wake();
            }
            Direction::In => {
                critical_section::with(|_| {
                    regs.diepctl(ep_addr.index()).modify(|w| {
                        w.set_stall(stalled);
                    });
                });

                // T::state().ep_in_wakers[ep_addr.index()].wake();
                state().ep_in_wakers[ep_addr.index()].wake();
            }
        }
    }

    fn endpoint_is_stalled(&mut self, ep_addr: EndpointAddress) -> bool {
        trace!("endpoint_is_stalled ep={:?}", ep_addr);
        assert!(
            ep_addr.index() < MAX_EP_COUNT,
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
