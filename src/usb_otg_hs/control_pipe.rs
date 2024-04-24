use core::future::poll_fn;
use core::sync::atomic::Ordering;
use core::task::Poll;
use defmt::{trace, info};
use futures::{future};
use future::select;
use futures::pin_mut;
use crate::usb_otg_hs::descriptor::{Direction, Request};
use crate::usb_otg_hs::endpoint_new::PhyState;
use crate::usb_otg_hs::global_states::{regs, state};
use crate::usb_otg_hs::interrupt::{init_reset, RESET};
use crate::usb_otg_hs::mod_new::{init_setaddress, process_setup_packet_new, SETUP_DATA};

async fn read0(buf: &mut [u8]) -> Result<PhyState, PhyState> {
    trace!("read start len={}", buf.len());
    let r = regs();
    r.doepdma(0).write(|w| { w.set_dmaaddr(buf.as_ptr() as u32) });
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
    match poll_fn(|cx| {
        state().ep_out_wakers[0].register(cx.waker());
        if unsafe { RESET } {
            return Poll::Ready(PhyState::Reset);
        }
        if r.dsts().read().suspsts() {
            return Poll::Ready(PhyState::Suspend);
        }
        if r.doepint(0).read().xfrc() {
            r.doepint(0).write(|w| {
                w.set_xfrc(true);
            });
            // clear xfrc
            trace!("read done len={}", buf.len() as u32 - regs().doeptsiz(0).read().xfrsiz());
            // Poll::Ready(())
            Poll::Ready(PhyState::Active)
        } else {
            Poll::Pending
        }
    }).await {
        PhyState::Active => {
            trace!("read len={} done", buf.len());
            Ok(PhyState::Active)
        }
        PhyState::Suspend => {
            defmt::error!("read len={} suspend", buf.len());
            Err(PhyState::Suspend)
        }
        _ => {
            defmt::error!("read len={} error", buf.len());
            Err(PhyState::Error)
        }
    }
}

async fn write0(buf: &[u8]) -> Result<PhyState, PhyState> {
    trace!("write start len={}, data={:x}", buf.len(), buf);
    let r = regs();
    r.diepdma(0)
        .write(|w| { w.set_dmaaddr(buf.as_ptr() as u32) });

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
    match poll_fn(|cx| {
        state().ep_in_wakers[0].register(cx.waker());
        if r.dsts().read().suspsts() {
            defmt::error!("write len={} suspend", buf.len());
            return Poll::Ready(PhyState::Suspend);
        }
        if unsafe { RESET } {
            defmt::error!("write len={} reset", buf.len());
            return Poll::Ready(PhyState::Reset);
        }
        if r.diepint(0).read().xfrc() {
            r.diepint(0).write(|w| w.set_xfrc(true)); // clear xfrc
            r.diepmsk().modify(|w| w.set_xfrcm(true));
            // unmask
            trace!("write done len={}", buf.len());
            // Poll::Ready(())
            Poll::Ready(PhyState::Active)
        } else {
            Poll::Pending
        }
    })
        .await {
        PhyState::Active => {
            trace!("write len={} done", buf.len());
            Ok(PhyState::Active)
        }
        PhyState::Suspend => {
            defmt::error!("write len={} suspend", buf.len());
            Err(PhyState::Suspend)
        }
        _ => {
            defmt::error!("write len={} error", buf.len());
            Err(PhyState::Error)
        }
    }
    // trace!("write len={} done", buf.len());
}

pub enum BusEvent {
    Reset,
    Suspend,
    Resume,
    Disconnect,
}

pub fn wakeup_all() {
    let state = state();
    for waker in state.ep_in_wakers.iter() {
        waker.wake();
    }
    for waker in state.ep_out_wakers.iter() {
        waker.wake();
    }
    state.bus_waker.wake();
}

#[embassy_executor::task]
pub async fn setup_process() {
    // wait for suspend clear
    loop {
        poll_fn(|cx| {
            state().bus_waker.register(cx.waker());
            if !regs().dsts().read().suspsts() {
                Poll::Ready(())
            } else {
                Poll::Pending
            }
        }).await;

        unsafe { RESET = false };
        defmt::info!("restart setup_process");
        loop {
            if setup_process_inner().await.is_err() {
                defmt::error!("setup_process_inner error");
                break;
            }
        }
    }
}

pub async fn setup_process_inner() -> Result<PhyState, PhyState> {
    unsafe {
        read0(&mut SETUP_DATA[0..8]).await?;
        defmt::info!("wait for setup packet ready");
        defmt::info!("doepctl0: {:x}", regs().doepctl(0).read().0);
        defmt::info!("doeptsiz0: {:x}", regs().doeptsiz(0).read().0);
        poll_fn(|cx| {
            state().ep_out_wakers[0].register(cx.waker());
            if unsafe { RESET } {
                return Poll::Ready(Err(PhyState::Reset));
            }
            if state().ep0_setup_ready.load(Ordering::Relaxed) {
                state().ep0_setup_ready.store(false, Ordering::Release);
                // regs().doepint(0).write(|w| w.0 = 0xFFFF_FFFF);
                return Poll::Ready(Ok(PhyState::Active));
            } else {
                Poll::Pending
            }
        })
            .await?;
        // regs().doepctl(0).modify(|v| v.set_snak(true));
        defmt::info!( "setup packet ready, processing package **********{:x}", SETUP_DATA
            );
        // let (res, size) = process_setup_packet(&SETUP_DATA);
        let mut tmp = process_setup_packet_new(&SETUP_DATA[0..8]);
        if tmp.has_data_stage {
            match tmp.data_stage_direction {
                Direction::In => {
                    write0(&tmp.data[0..tmp.len]).await?;
                    read0(&mut tmp.data[0..0]).await?; // status stage no data
                    return Ok(PhyState::Active);
                }
                Direction::Out => {
                    read0(&mut tmp.data[0..tmp.len]).await?;
                    write0(&[0u8; 0]).await? // status stage no data
                }
            };
        } else {
            // status stage no data
            match tmp.setup.direction {
                Direction::In => {
                    // read0(&mut buf[0..0]).await; // status stage no data
                    read0(&mut tmp.data[0..0]).await? // status stage no data
                }
                Direction::Out => {
                    write0(&[0u8; 0]).await? // status stage no data
                }
            };
        }
        // end of data stage

        match tmp.request {
            Request::SetAddress(addr) => {
                init_setaddress(addr);
            }
            Request::SetConfiguration(_) => {
                // init all endpoints
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
        //         write0(&SETUP_RETURN_DATA[0..size]).await;
        //     }
        //     read0(&mut buf[0..0]).await; // status stage no data
        // } else {
        //     write0(&[0u8; 0]).await; // status stage no data
        // }
    }
    Ok(PhyState::Active)
}
