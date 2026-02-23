use crate::usb::endpoint::Endpoint;
use crate::usb::reg::Otg;
use crate::usb::{ControlPipeSetupState, In, Out};
use core::future::poll_fn;
use core::sync::atomic::Ordering;
use core::task::Poll;
use embassy_usb_driver::{EndpointError, EndpointIn, EndpointOut};

/// USB control pipe.
pub struct ControlPipe<'d> {
    pub(crate) max_packet_size: u16,
    pub(crate) regs: Otg,
    pub(crate) setup_state: &'d ControlPipeSetupState,
    pub(crate) ep_in: Endpoint<'d, In>,
    pub(crate) ep_out: Endpoint<'d, Out>,
}

impl<'d> embassy_usb_driver::ControlPipe for ControlPipe<'d> {
    fn max_packet_size(&self) -> usize {
        usize::from(self.max_packet_size)
    }

    async fn setup(&mut self) -> [u8; 8] {
        poll_fn(|cx| {
            self.ep_out.state.out_waker.register(cx.waker());

            if self.setup_state.setup_ready.load(Ordering::Relaxed) {
                let mut data = [0; 8];
                data[0..4].copy_from_slice(&self.setup_state.setup_data[0].load(Ordering::Relaxed).to_ne_bytes());
                data[4..8].copy_from_slice(&self.setup_state.setup_data[1].load(Ordering::Relaxed).to_ne_bytes());
                self.setup_state.setup_ready.store(false, Ordering::Release);

                // EP0 should not be controlled by `Bus` so this RMW does not need a critical section
                self.regs.doeptsiz(self.ep_out.info.addr.index()).modify(|w| {
                    w.set_rxdpid_stupcnt(3);
                });

                // Clear NAK to indicate we are ready to receive more data
                self.regs.doepctl(self.ep_out.info.addr.index()).modify(|w| w.set_cnak(true));

                trace!("SETUP received: {:?}", data);
                Poll::Ready(data)
            } else {
                trace!("SETUP waiting");
                Poll::Pending
            }
        })
        .await
    }

    async fn data_out(&mut self, buf: &mut [u8], _first: bool, _last: bool) -> Result<usize, EndpointError> {
        trace!("control: data_out");
        let len = self.ep_out.read(buf).await?;
        trace!("control: data_out read: {:?}", &buf[..len]);
        Ok(len)
    }

    async fn data_in(&mut self, data: &[u8], _first: bool, last: bool) -> Result<(), EndpointError> {
        trace!("control: data_in write: {:?}", data);
        self.ep_in.write(data).await?;

        // wait for status response from host after sending the last packet
        if last {
            trace!("control: data_in waiting for status");
            self.ep_out.read(&mut []).await?;
            trace!("control: complete");
        }

        Ok(())
    }

    async fn accept(&mut self) {
        trace!("control: accept");

        self.ep_in.write(&[]).await.ok();

        trace!("control: accept OK");
    }

    async fn reject(&mut self) {
        trace!("control: reject");

        // EP0 should not be controlled by `Bus` so this RMW does not need a critical section
        self.regs.diepctl(self.ep_in.info.addr.index()).modify(|w| {
            w.set_stall(true);
        });
        self.regs.doepctl(self.ep_out.info.addr.index()).modify(|w| {
            w.set_stall(true);
        });
    }

    async fn accept_set_address(&mut self, addr: u8) {
        trace!("setting addr: {}", addr);
        critical_section::with(|_| {
            self.regs.dcfg().modify(|w| {
                w.set_dad(addr);
            });
        });

        // synopsys driver requires accept to be sent after changing address
        self.accept().await
    }
}
