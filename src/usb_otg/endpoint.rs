use core::future::poll_fn;
use core::sync::atomic::Ordering;
use core::task::Poll;

use defmt::{debug, trace};
use embassy_usb_driver::{Direction, EndpointAddress, EndpointError, EndpointInfo, EndpointType};
// use stm32_metapac::USB_OTG_FS;

use crate::usb_otg::{EP_OUT_BUFFER_EMPTY, state};
use crate::usb_otg::regs;

macro_rules! info {
    ($($arg:tt)*) => {};
}
macro_rules! debug {
    ($($arg:tt)*) => {};
}
macro_rules! trace {
    ($($arg:tt)*) => {};
}
macro_rules! error {
    ($($arg:tt)*) => {};
}

//
// /// USB endpoint.
#[derive(Clone, Copy, defmt::Format, PartialEq, Eq)]
pub struct Endpoint {
    // _phantom: PhantomData<(&'d mut T, D)>,
    pub(crate) info: EndpointInfo,
}

impl embassy_usb_driver::Endpoint for Endpoint {
    fn info(&self) -> &EndpointInfo {
        // &self.info
        &self.info
    }

    async fn wait_enabled(&mut self) {
        poll_fn(|cx| {
            let ep_index = self.info.addr.index();

            state().ep_in_wakers[ep_index].register(cx.waker());
            // T::state().ep_in_wakers[ep_index].register(cx.waker());

            if regs().diepctl(ep_index).read().usbaep() {
                Poll::Ready(())
            } else {
                Poll::Pending
            }
        })
            .await;
        trace!("wait_enabled end");
    }
}


// // impl<'d, T: Instance> embassy_usb_driver::Endpoint for Endpoint<'d, T, Out> {
// impl embassy_usb_driver::Endpoint for Endpoint{
//     fn info(&self) -> &EndpointInfo {
//         &self.info
//     }
//
//     async fn wait_enabled(&mut self) {
//         poll_fn(|cx| {
//             let ep_index = self.info.addr.index();
//
//             // T::state().ep_out_wakers[ep_index].register(cx.waker());
//             unsafe { STATE.ep_out_wakers[ep_index].register(cx.waker()); }
//
//             // if T::regs().doepctl(ep_index).read().usbaep() {
//             if regs().doepctl(ep_index).read().usbaep() {
//                 Poll::Ready(())
//             } else {
//                 Poll::Pending
//             }
//         })
//         .await
//     }
// }

// impl<'d, T: Instance> embassy_usb_driver::EndpointOut for Endpoint<'d, T, Out> {
impl embassy_usb_driver::EndpointOut for Endpoint {
    async fn read(&mut self, buf: &mut [u8]) -> Result<usize, EndpointError> {
        trace!("read start len={}", buf.len());

        poll_fn(|cx| {
            // let r = T::regs();
            let r = regs();
            let index = self.info.addr.index();
            // let state = T::state();
            // let state = unsafe { &STATE };
            let state = state();

            state.ep_out_wakers[index].register(cx.waker());

            let doepctl = r.doepctl(index).read();
            trace!("read ep={:?}: doepctl {:08x}", self.info.addr, doepctl.0,);
            debug!("gintmsk: {:08x}", r.gintmsk().read().0);
            trace!("gintsts: {:08x}", r.gintsts().read().0);
            if !doepctl.usbaep() {
                trace!("read ep={:?} error disabled", self.info.addr);
                return Poll::Ready(Err(EndpointError::Disabled));
            }

            let len = state.ep_out_size[index].load(Ordering::Relaxed);
            if len != EP_OUT_BUFFER_EMPTY {
                trace!("read ep={:?} done len={}", self.info.addr, len);

                if len as usize > buf.len() {
                    return Poll::Ready(Err(EndpointError::BufferOverflow));
                }

                // SAFETY: exclusive access ensured by `ep_out_size` atomic variable
                let data = unsafe {
                    // core::slice::from_raw_parts(*state.ep_out_buffers[index].get(), len as usize)
                    core::slice::from_raw_parts(state.ep_out_buffers[index].as_ptr(), len as usize)
                };
                buf[..len as usize].copy_from_slice(data);

                // Release buffer
                state.ep_out_size[index].store(EP_OUT_BUFFER_EMPTY, Ordering::Release);

                critical_section::with(|_| {
                    // Receive 1 packet
                    regs().doeptsiz(index).modify(|w| {
                        w.set_xfrsiz(self.info.max_packet_size as _);
                        w.set_pktcnt(1);
                    });

                    // Clear NAK to indicate we are ready to receive more data
                    regs().doepctl(index).modify(|w| {
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

// impl<'d, T: Instance> embassy_usb_driver::EndpointIn for Endpoint<'d, T, In> {
impl embassy_usb_driver::EndpointIn for Endpoint {
    async fn write(&mut self, buf: &[u8]) -> Result<(), EndpointError> {
        trace!("write ep={:?} data={:?}", self.info.addr.index(), buf);

        if buf.len() > self.info.max_packet_size as usize {
            return Err(EndpointError::BufferOverflow);
        }

        // let r = T::regs();
        let r = regs();
        let index = self.info.addr.index();
        let state = state();

        // Wait for previous transfer to complete and check if endpoint is disabled
        poll_fn(|cx| {
            state.ep_in_wakers[index].register(cx.waker());

            let diepctl = r.diepctl(index).read();
            let dtxfsts = r.dtxfsts(index).read();
            trace!(
                "write ep={:?}: diepctl {:08x} ftxfsts {:08x}",
                index,// self.info.addr,
                diepctl.0,
                dtxfsts.0
            );
            if !diepctl.usbaep() {
                trace!(
                    "write ep={:?} wait for prev: error disabled",
                    self.info.addr.index()
                );
                Poll::Ready(Err(EndpointError::Disabled))
            } else if !diepctl.epena() {
                trace!("write ep={:?} wait for prev: ready", self.info.addr.index());
                Poll::Ready(Ok(()))
            } else {
                trace!("write ep={:?} wait for prev: pending", self.info.addr.index());
                Poll::Pending
            }
        })
            .await?;

        if buf.len() > 0 {
            poll_fn(|cx| {
                state.ep_in_wakers[index].register(cx.waker());

                let size_words = (buf.len() + 3) / 4;

                let fifo_space = r.dtxfsts(index).read().ineptfsav() as usize;
                if size_words > fifo_space {
                    // Not enough space in fifo, enable tx fifo empty interrupt
                    critical_section::with(|_| {
                        r.diepempmsk().modify(|w| {
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

        // Setup transfer size
        r.dieptsiz(index).write(|w| {
            w.set_mcnt(1);
            w.set_pktcnt(1);
            w.set_xfrsiz(buf.len() as _);
        });

        r.daintmsk().modify(|w| {
            // w.set_iepm(ep_irq_mask(&self.ep_in));
            w.set_iepm(7);
            // OUT interrupts not used, handled in RXFLVL
            // w.set_oepm(ep_irq_mask(&self.ep_out));
        });

        r.diepmsk().write(|w| {
            w.set_xfrcm(true);    // Unmask transfer complete EP interrupt
        });

        critical_section::with(|_| {
            // Enable endpoint
            r.diepctl(index).modify(|w| {
                w.set_cnak(true);
                w.set_epena(true);
            });
        });

        // Write data to FIFO
        for chunk in buf.chunks(4) {
            let mut tmp = [0u8; 4];
            tmp[0..chunk.len()].copy_from_slice(chunk);
            r.fifo(index)
                .write_value(stm32_metapac::otg::regs::Fifo(u32::from_ne_bytes(tmp)));
            // .write_value(regs::Fifo(u32::from_ne_bytes(tmp)));
        }

        trace!("write done ep={:?}", self.info.addr);

        Ok(())
    }
}
