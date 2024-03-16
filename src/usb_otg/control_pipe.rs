use core::future::poll_fn;
use core::sync::atomic::Ordering;
use core::task::Poll;

use embassy_usb_driver::{EndpointError, EndpointIn, EndpointOut};

use crate::usb_otg::{quirk_setup_late_cnak, regs, state};
use crate::usb_otg::endpoint::Endpoint;
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
// /// USB control pipe.
pub struct ControlPipe {
    // _phantom: PhantomData<&'d mut T>,
    pub(crate) max_packet_size: u16,
    pub(crate) ep_in: Endpoint,
    pub(crate) ep_out: Endpoint,
}

impl embassy_usb_driver::ControlPipe for ControlPipe {
    /// Maximum packet size for the control pipe
    fn max_packet_size(&self) -> usize {
        usize::from(self.max_packet_size)
    }

    async fn setup(&mut self) -> [u8; 8] {
        trace!("control: setup start");
        poll_fn(|cx| {
            // let state = T::state();
            let state = state();

            state.ep_out_wakers[0].register(cx.waker());

            // let r = T::regs();
            let r = regs();

            if state.ep0_setup_ready.load(Ordering::Relaxed) {
                // let data = unsafe { *state.ep0_setup_data.get() };
                // let data = unsafe {STATE.ep0_setup_data.co
                // let data = unsafe { state.ep0_setup_data.as_ptr().read() };
                // copy data from the buffer (state.ep0_setup_data) to a new array
                let mut data = [0; 8];
                data.copy_from_slice(&state.ep0_setup_data);
                state.ep0_setup_ready.store(false, Ordering::Release);

                // EP0 should not be controlled by `Bus` so this RMW does not need a critical section
                // Receive 1 SETUP packet
                r.doeptsiz(self.ep_out.info.addr.index()).modify(|w| {
                    w.set_rxdpid_stupcnt(1);
                });

                // Clear NAK to indicate we are ready to receive more data
                if !quirk_setup_late_cnak(r) {
                    r.doepctl(self.ep_out.info.addr.index())
                        .modify(|w| w.set_cnak(true));
                }

                trace!("SETUP received: {:?}", data);
                Poll::Ready(data)
            } else {
                trace!("SETUP waiting");
                Poll::Pending
            }
        }).await
    }

    async fn data_out(
        &mut self,
        buf: &mut [u8],
        _first: bool,
        _last: bool,
    ) -> Result<usize, EndpointError> {
        trace!("control: data_out");
        let len = self.ep_out.read(buf).await?;
        trace!("control: data_out read: {:?}", &buf[..len]);
        Ok(len)
    }

    async fn data_in(
        &mut self,
        mut data: &[u8],
        _first: bool,
        last: bool,
    ) -> Result<(), EndpointError> {
        // // trace!("control: data_in write: {:?}", data);
        // let tmp_dat = [0x09,0x02,0x30,0x00,0x02,0x01,0x00,0x80,0x32,0x09,0x04,0x00,0x00,0x01,0x02,0x02,0x00,0x00,0x07,0x05,0x80,0x03,0x08,0x00,0xFF,0x09,0x04,0x01,0x00,0x02,0x0A,0x00,0x00,0x00,0x07,0x05,0x01,0x02,0x40,0x00,0x00,0x07,0x05,0x82,0x02,0x40,0x00,0x00];

        // if data[0] == 0x09 && data[1] ==0x02 {
        //     trace!("control: data_in write: {:?}", data);
        //     data = &tmp_dat;
        // }

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
        // let regs = T::regs();
        let regs = regs();
        regs.diepctl(self.ep_in.info.addr.index()).modify(|w| {
            w.set_stall(true);
        });
        regs.doepctl(self.ep_out.info.addr.index()).modify(|w| {
            w.set_stall(true);
        });
    }

    async fn accept_set_address(&mut self, addr: u8) {
        trace!("setting addr: {}", addr);
        critical_section::with(|_| {
            regs().dcfg().modify(|w| {
                w.set_dad(addr);
            });
        });

        // synopsys driver requires accept to be sent after changing address
        self.accept().await
    }
}
