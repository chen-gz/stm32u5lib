use super::vals;
use crate::usb::regs;
use crate::usb::{ep0_mpsiz, ep_fifo_size, ep_irq_mask, to_eptyp, Config, EndpointData, OtgInstance, PhyType};
use core::future::poll_fn;
use core::task::Poll;
use embassy_usb_driver::{Bus as OtherBus, Direction, EndpointAddress, Event, Unsupported};

/// USB bus.
pub struct Bus<'d, const MAX_EP_COUNT: usize> {
    pub(crate) config: Config,
    pub(crate) ep_in: [Option<EndpointData>; MAX_EP_COUNT],
    pub(crate) ep_out: [Option<EndpointData>; MAX_EP_COUNT],
    pub(crate) instance: OtgInstance<'d, MAX_EP_COUNT>,
    pub(crate) inited: bool,
}

impl<'d, const MAX_EP_COUNT: usize> Bus<'d, MAX_EP_COUNT> {
    fn restore_irqs(&mut self) {
        info!("restore_irqs");
        self.instance.regs.gintmsk().write(|w| {
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

    /// Returns the PHY type.
    pub fn phy_type(&self) -> PhyType {
        self.instance.phy_type
    }

    /// Configures the PHY as a device.
    pub fn configure_as_device(&mut self) {
        let r = self.instance.regs;
        let phy_type = self.instance.phy_type;
        r.gusbcfg().write(|w| {
            // Force device mode
            w.set_fdmod(true);
            // Enable internal full-speed PHY
            w.set_physel(phy_type.internal() && !phy_type.high_speed());
        });
    }

    /// Applies configuration specific to
    /// Core ID 0x0000_1100 and 0x0000_1200
    pub fn config_v1(&mut self) {
        let r = self.instance.regs;
        let phy_type = self.instance.phy_type;
        assert!(phy_type != PhyType::InternalHighSpeed);

        r.gccfg_v1().modify(|w| {
            // Enable internal full-speed PHY, logic is inverted
            w.set_pwrdwn(phy_type.internal());
        });

        // F429-like chips have the GCCFG.NOVBUSSENS bit
        r.gccfg_v1().modify(|w| {
            w.set_novbussens(!self.config.vbus_detection);
            w.set_vbusasen(false);
            w.set_vbusbsen(self.config.vbus_detection);
            w.set_sofouten(false);
        });
    }

    /// Applies configuration specific to
    /// Core ID 0x0000_2000, 0x0000_2100, 0x0000_2300, 0x0000_3000 and 0x0000_3100
    pub fn config_v2v3(&mut self) {
        let r = self.instance.regs;
        let phy_type = self.instance.phy_type;

        // F446-like chips have the GCCFG.VBDEN bit with the opposite meaning
        r.gccfg_v2().modify(|w| {
            // Enable internal full-speed PHY, logic is inverted
            w.set_pwrdwn(phy_type.internal() && !phy_type.high_speed());
            w.set_phyhsen(phy_type.internal() && phy_type.high_speed());
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

    /// Applies configuration specific to
    ///
    /// Core ID 0x0000_5000
    pub fn config_v5(&mut self) {
        let r = self.instance.regs;
        let phy_type = self.instance.phy_type;

        if phy_type == PhyType::InternalHighSpeed {
            r.gccfg_v3().modify(|w| {
                w.set_vbvaloven(!self.config.vbus_detection);
                w.set_vbvaloval(!self.config.vbus_detection);
                w.set_vbden(self.config.vbus_detection);
            });
        } else {
            r.gotgctl().modify(|w| {
                w.set_bvaloen(!self.config.vbus_detection);
                w.set_bvaloval(!self.config.vbus_detection);
            });
            r.gccfg_v3().modify(|w| {
                w.set_vbden(self.config.vbus_detection);
            });
        }
    }

    fn init(&mut self) {
        let r = self.instance.regs;
        let phy_type = self.instance.phy_type;

        // Soft disconnect.
        r.dctl().write(|w| w.set_sdis(true));

        // Set speed.
        r.dcfg().write(|w| {
            w.set_pfivl(vals::Pfivl::FRAME_INTERVAL_80);
            w.set_dspd(phy_type.to_dspd());
            if self.config.xcvrdly {
                w.set_xcvrdly(true);
            }
        });

        // Unmask transfer complete EP interrupt
        r.diepmsk().write(|w| {
            w.set_xfrcm(true);
        });

        // Unmask SETUP received EP interrupt
        r.doepmsk().write(|w| {
            w.set_stupm(true);
        });

        // Unmask and clear core interrupts
        self.restore_irqs();
        r.gintsts().write_value(regs::Gintsts(0xFFFF_FFFF));

        // Unmask global interrupt
        r.gahbcfg().write(|w| {
            w.set_gint(true); // unmask global interrupt
        });

        // Connect
        r.dctl().write(|w| w.set_sdis(false));
        info!("start connection");
    }

    fn init_fifo(&mut self) {
        trace!("init_fifo");

        let regs = self.instance.regs;
        // ERRATA NOTE: Don't interrupt FIFOs being written to. The interrupt
        // handler COULD interrupt us here and do FIFO operations, so ensure
        // the interrupt does not occur.
        critical_section::with(|_| {
            // Configure RX fifo size. All endpoints share the same FIFO area.
            let rx_fifo_size_words = self.instance.extra_rx_fifo_words + ep_fifo_size(&self.ep_out);
            trace!("configuring rx fifo size={}", rx_fifo_size_words);

            regs.grxfsiz().modify(|w| w.set_rxfd(rx_fifo_size_words));

            // Configure TX (USB in direction) fifo size for each endpoint
            let mut fifo_top = rx_fifo_size_words;
            for i in 0..self.instance.endpoint_count {
                if let Some(ep) = self.ep_in[i] {
                    trace!(
                        "configuring tx fifo ep={}, offset={}, size={}",
                        i,
                        fifo_top,
                        ep.fifo_size_words
                    );

                    let dieptxf = if i == 0 { regs.dieptxf0() } else { regs.dieptxf(i - 1) };

                    dieptxf.write(|w| {
                        w.set_fd(ep.fifo_size_words);
                        w.set_sa(fifo_top);
                    });

                    fifo_top += ep.fifo_size_words;
                }
            }

            assert!(
                fifo_top <= self.instance.fifo_depth_words,
                "FIFO allocations exceeded maximum capacity"
            );

            // Flush fifos
            regs.grstctl().write(|w| {
                w.set_rxfflsh(true);
                w.set_txfflsh(true);
                w.set_txfnum(0x10);
            });
        });

        loop {
            let x = regs.grstctl().read();
            if !x.rxfflsh() && !x.txfflsh() {
                break;
            }
        }
    }

    fn configure_endpoints(&mut self) {
        trace!("configure_endpoints");

        let regs = self.instance.regs;

        // Configure IN endpoints
        for (index, ep) in self.ep_in.iter().enumerate() {
            if let Some(ep) = ep {
                critical_section::with(|_| {
                    regs.diepctl(index).write(|w| {
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
        // Configure OUT endpoints
        for (index, ep) in self.ep_out.iter().enumerate() {
            if let Some(ep) = ep {
                critical_section::with(|_| {
                    regs.doepctl(index).write(|w| {
                        if index == 0 {
                            w.set_mpsiz(ep0_mpsiz(ep.max_packet_size));
                        } else {
                            w.set_mpsiz(ep.max_packet_size);
                            w.set_eptyp(to_eptyp(ep.ep_type));
                            w.set_sd0pid_sevnfrm(true);
                        }
                    });

                    regs.doeptsiz(index).modify(|w| {
                        w.set_xfrsiz(ep.max_packet_size as _);
                        if index == 0 {
                            w.set_rxdpid_stupcnt(3);
                        } else {
                            w.set_pktcnt(1);
                        }
                    });
                });
            }
        }

        // Enable IRQs for allocated endpoints
        regs.daintmsk().modify(|w| {
            w.set_iepm(ep_irq_mask(&self.ep_in));
            w.set_oepm(ep_irq_mask(&self.ep_out));
        });
    }

    fn disable_all_endpoints(&mut self) {
        for i in 0..self.instance.endpoint_count {
            self.endpoint_set_enabled(EndpointAddress::from_parts(i, Direction::In), false);
            self.endpoint_set_enabled(EndpointAddress::from_parts(i, Direction::Out), false);
        }
    }
}

impl<'d, const MAX_EP_COUNT: usize> embassy_usb_driver::Bus for Bus<'d, MAX_EP_COUNT> {
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

            let regs = self.instance.regs;
            self.instance.state.bus_waker.register(cx.waker());

            let ints = regs.gintsts().read();

            if ints.srqint() {
                trace!("vbus detected");

                regs.gintsts().write(|w| w.set_srqint(true)); // clear
                self.restore_irqs();

                if self.config.vbus_detection {
                    return Poll::Ready(Event::PowerDetected);
                }
            }

            if ints.otgint() {
                let otgints = regs.gotgint().read();
                regs.gotgint().write_value(otgints); // clear all
                self.restore_irqs();

                if otgints.sedet() {
                    trace!("vbus removed");
                    if self.config.vbus_detection {
                        self.disable_all_endpoints();
                        return Poll::Ready(Event::PowerRemoved);
                    }
                }
            }

            if ints.usbrst() {
                trace!("reset");

                self.init_fifo();
                self.configure_endpoints();

                // Reset address
                critical_section::with(|_| {
                    regs.dcfg().modify(|w| {
                        w.set_dad(0);
                    });
                });

                regs.gintsts().write(|w| w.set_usbrst(true)); // clear
                self.restore_irqs();
            }

            if ints.enumdne() {
                trace!("enumdne");

                let speed = regs.dsts().read().enumspd();
                let trdt = (self.instance.calculate_trdt_fn)(speed);
                trace!("  speed={} trdt={}", speed.to_bits(), trdt);
                regs.gusbcfg().modify(|w| w.set_trdt(trdt));

                regs.gintsts().write(|w| w.set_enumdne(true)); // clear
                self.restore_irqs();

                return Poll::Ready(Event::Reset);
            }

            if ints.usbsusp() {
                trace!("suspend");
                regs.gintsts().write(|w| w.set_usbsusp(true)); // clear
                self.restore_irqs();
                return Poll::Ready(Event::Suspend);
            }

            if ints.wkupint() {
                trace!("resume");
                regs.gintsts().write(|w| w.set_wkupint(true)); // clear
                self.restore_irqs();
                return Poll::Ready(Event::Resume);
            }

            Poll::Pending
        })
        .await
    }

    fn endpoint_set_stalled(&mut self, ep_addr: EndpointAddress, stalled: bool) {
        trace!("endpoint_set_stalled ep={:?} en={}", ep_addr, stalled);

        assert!(
            ep_addr.index() < self.instance.endpoint_count,
            "endpoint_set_stalled index {} out of range",
            ep_addr.index()
        );

        let regs = self.instance.regs;
        let state = self.instance.state;
        match ep_addr.direction() {
            Direction::Out => {
                critical_section::with(|_| {
                    regs.doepctl(ep_addr.index()).modify(|w| {
                        w.set_stall(stalled);
                    });
                });

                state.ep_states[ep_addr.index()].out_waker.wake();
            }
            Direction::In => {
                critical_section::with(|_| {
                    regs.diepctl(ep_addr.index()).modify(|w| {
                        w.set_stall(stalled);
                    });
                });

                state.ep_states[ep_addr.index()].in_waker.wake();
            }
        }
    }

    fn endpoint_is_stalled(&mut self, ep_addr: EndpointAddress) -> bool {
        assert!(
            ep_addr.index() < self.instance.endpoint_count,
            "endpoint_is_stalled index {} out of range",
            ep_addr.index()
        );

        let regs = self.instance.regs;
        match ep_addr.direction() {
            Direction::Out => regs.doepctl(ep_addr.index()).read().stall(),
            Direction::In => regs.diepctl(ep_addr.index()).read().stall(),
        }
    }

    fn endpoint_set_enabled(&mut self, ep_addr: EndpointAddress, enabled: bool) {
        trace!("endpoint_set_enabled ep={:?} en={}", ep_addr, enabled);

        assert!(
            ep_addr.index() < self.instance.endpoint_count,
            "endpoint_set_enabled index {} out of range",
            ep_addr.index()
        );

        let regs = self.instance.regs;
        let state = self.instance.state;
        match ep_addr.direction() {
            Direction::Out => {
                critical_section::with(|_| {
                    // cancel transfer if active
                    if !enabled && regs.doepctl(ep_addr.index()).read().epena() {
                        regs.doepctl(ep_addr.index()).modify(|w| {
                            w.set_snak(true);
                            w.set_epdis(true);
                        })
                    }

                    regs.doepctl(ep_addr.index()).modify(|w| {
                        w.set_usbaep(enabled);
                    });

                    // Flush tx fifo
                    regs.grstctl().write(|w| {
                        w.set_txfflsh(true);
                        w.set_txfnum(ep_addr.index() as _);
                    });
                    loop {
                        let x = regs.grstctl().read();
                        if !x.txfflsh() {
                            break;
                        }
                    }
                });

                // Wake `Endpoint::wait_enabled()`
                state.ep_states[ep_addr.index()].out_waker.wake();
            }
            Direction::In => {
                critical_section::with(|_| {
                    // cancel transfer if active
                    if !enabled && regs.diepctl(ep_addr.index()).read().epena() {
                        regs.diepctl(ep_addr.index()).modify(|w| {
                            w.set_snak(true); // set NAK
                            w.set_epdis(true);
                        })
                    }

                    regs.diepctl(ep_addr.index()).modify(|w| {
                        w.set_usbaep(enabled);
                        w.set_cnak(enabled); // clear NAK that might've been set by SNAK above.
                    })
                });

                // Wake `Endpoint::wait_enabled()`
                state.ep_states[ep_addr.index()].in_waker.wake();
            }
        }
    }

    async fn enable(&mut self) {
        trace!("enable");
        // TODO: enable the peripheral once enable/disable semantics are cleared up in embassy-usb
    }

    async fn disable(&mut self) {
        trace!("disable");

        // TODO: disable the peripheral once enable/disable semantics are cleared up in embassy-usb
        //Bus::disable(self);
    }

    async fn remote_wakeup(&mut self) -> Result<(), Unsupported> {
        Err(Unsupported)
    }
}
