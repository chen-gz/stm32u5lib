use crate::usb::regs;
use core::future::poll_fn;
use core::marker::PhantomData;
use core::sync::atomic::Ordering;
use core::task::Poll;
use embassy_usb_driver::{EndpointError, EndpointInfo, EndpointType};
use crate::usb::{EpState, In, Out, EP_OUT_BUFFER_EMPTY};
use defmt::trace;
use crate::usb::reg::Otg;

/// USB endpoint.
pub struct Endpoint<'d, D> {
    pub(crate) _phantom: PhantomData<D>,
    pub(crate) regs: Otg,
    pub(crate) info: EndpointInfo,
    pub(crate) state: &'d EpState,
}

impl<'d> embassy_usb_driver::Endpoint for Endpoint<'d, In> {
    fn info(&self) -> &EndpointInfo {
        &self.info
    }

    async fn wait_enabled(&mut self) {
        poll_fn(|cx| {
            let ep_index = self.info.addr.index();

            self.state.in_waker.register(cx.waker());

            if self.regs.diepctl(ep_index).read().usbaep() {
                Poll::Ready(())
            } else {
                Poll::Pending
            }
        })
            .await
    }
}

impl<'d> embassy_usb_driver::Endpoint for Endpoint<'d, Out> {
    fn info(&self) -> &EndpointInfo {
        &self.info
    }

    async fn wait_enabled(&mut self) {
        poll_fn(|cx| {
            let ep_index = self.info.addr.index();

            self.state.out_waker.register(cx.waker());

            if self.regs.doepctl(ep_index).read().usbaep() {
                Poll::Ready(())
            } else {
                Poll::Pending
            }
        })
            .await
    }
}

impl<'d> embassy_usb_driver::EndpointOut for Endpoint<'d, Out> {
    async fn read(&mut self, buf: &mut [u8]) -> Result<usize, EndpointError> {
        trace!("read start len={}", buf.len());

        poll_fn(|cx| {
            let index = self.info.addr.index();
            self.state.out_waker.register(cx.waker());

            let doepctl = self.regs.doepctl(index).read();
            trace!("read ep={:?}: doepctl {:08x}", self.info.addr, doepctl.0,);
            if !doepctl.usbaep() {
                trace!("read ep={:?} error disabled", self.info.addr);
                return Poll::Ready(Err(EndpointError::Disabled));
            }

            let len = self.state.out_size.load(Ordering::Relaxed);
            if len != EP_OUT_BUFFER_EMPTY {
                trace!("read ep={:?} done len={}", self.info.addr, len);

                if len as usize > buf.len() {
                    return Poll::Ready(Err(EndpointError::BufferOverflow));
                }

                // SAFETY: exclusive access ensured by `out_size` atomic variable
                let data = unsafe { core::slice::from_raw_parts(*self.state.out_buffer.get(), len as usize) };
                buf[..len as usize].copy_from_slice(data);

                // Release buffer
                self.state.out_size.store(EP_OUT_BUFFER_EMPTY, Ordering::Release);

                critical_section::with(|_| {
                    // Receive 1 packet
                    self.regs.doeptsiz(index).modify(|w| {
                        w.set_xfrsiz(self.info.max_packet_size as _);
                        w.set_pktcnt(1);
                    });

                    if self.info.ep_type == EndpointType::Isochronous {
                        // Isochronous endpoints must set the correct even/odd frame bit to
                        // correspond with the next frame's number.
                        let frame_number = self.regs.dsts().read().fnsof();
                        let frame_is_odd = frame_number & 0x01 == 1;

                        self.regs.doepctl(index).modify(|r| {
                            if frame_is_odd {
                                r.set_sd0pid_sevnfrm(true);
                            } else {
                                // r.set_sd1pid_soddfrm(true);
                                r.set_sd0pid_sevnfrm(false);
                            }
                        });
                    }

                    // Clear NAK to indicate we are ready to receive more data
                    self.regs.doepctl(index).modify(|w| {
                        w.set_cnak(true);
                    });
                });

                Poll::Ready(Ok(len as usize))
            } else {
                Poll::Pending
            }
        })
            .await
    }
}

impl<'d> embassy_usb_driver::EndpointIn for Endpoint<'d, In> {
    async fn write(&mut self, buf: &[u8]) -> Result<(), EndpointError> {
        trace!("write ep={:?} data={:?}", self.info.addr, buf);

        if buf.len() > self.info.max_packet_size as usize {
            return Err(EndpointError::BufferOverflow);
        }

        let index = self.info.addr.index();
        // Wait for previous transfer to complete and check if endpoint is disabled
        poll_fn(|cx| {
            self.state.in_waker.register(cx.waker());

            let diepctl = self.regs.diepctl(index).read();
            let dtxfsts = self.regs.dtxfsts(index).read();
            trace!(
                "write ep={:?}: diepctl {:08x} ftxfsts {:08x}",
                self.info.addr,
                diepctl.0,
                dtxfsts.0
            );
            if !diepctl.usbaep() {
                trace!("write ep={:?} wait for prev: error disabled", self.info.addr);
                Poll::Ready(Err(EndpointError::Disabled))
            } else if !diepctl.epena() {
                trace!("write ep={:?} wait for prev: ready", self.info.addr);
                Poll::Ready(Ok(()))
            } else {
                trace!("write ep={:?} wait for prev: pending", self.info.addr);
                Poll::Pending
            }
        })
            .await?;

        if buf.len() > 0 {
            poll_fn(|cx| {
                self.state.in_waker.register(cx.waker());

                let size_words = (buf.len() + 3) / 4;

                let fifo_space = self.regs.dtxfsts(index).read().ineptfsav() as usize;
                if size_words > fifo_space {
                    // Not enough space in fifo, enable tx fifo empty interrupt
                    critical_section::with(|_| {
                        self.regs.diepempmsk().modify(|w| {
                            w.set_ineptxfem(w.ineptxfem() | (1 << index));
                        });
                    });

                    trace!("tx fifo for ep={} full, waiting for txfe", index);

                    Poll::Pending
                } else {
                    trace!("write ep={:?} wait for fifo: ready", self.info.addr);
                    Poll::Ready(())
                }
            })
                .await
        }

        // ERRATA: Transmit data FIFO is corrupted when a write sequence to the FIFO is interrupted with
        // accesses to certain OTG_FS registers.
        //
        // Prevent the interrupt (which might poke FIFOs) from executing while copying data to FIFOs.
        critical_section::with(|_| {
            // Setup transfer size
            self.regs.dieptsiz(index).write(|w| {
                w.set_mcnt(1);
                w.set_pktcnt(1);
                w.set_xfrsiz(buf.len() as _);
            });

            if self.info.ep_type == EndpointType::Isochronous {
                // Isochronous endpoints must set the correct even/odd frame bit to
                // correspond with the next frame's number.
                let frame_number = self.regs.dsts().read().fnsof();
                let frame_is_odd = frame_number & 0x01 == 1;

                self.regs.diepctl(index).modify(|r| {
                    if frame_is_odd {
                        r.set_sd0pid_sevnfrm(true);
                    } else {
                        // r.set_sd1pid_soddfrm(true);
                        r.set_sd0pid_sevnfrm(false);
                    }
                });
            }

            // Enable endpoint
            self.regs.diepctl(index).modify(|w| {
                w.set_cnak(true);
                w.set_epena(true);
            });

            // Write data to FIFO
            for chunk in buf.chunks(4) {
                let mut tmp = [0u8; 4];
                tmp[0..chunk.len()].copy_from_slice(chunk);
                self.regs.fifo(index).write_value(regs::Fifo(u32::from_ne_bytes(tmp)));
            }
        });

        trace!("write done ep={:?}", self.info.addr);

        Ok(())
    }
}
