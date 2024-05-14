use core::future::poll_fn;
use core::sync::atomic::Ordering;
use core::task::Poll;
use defmt::{trace};
// use crate::otg_hs::descriptor::{Direction, Request};
use crate::usb_common::{
    descriptor::{Direction, Request},
    process_setup_packet_new,
};
use crate::otg_hs::endpoint::PhyState;
use crate::otg_hs::global_states::{regs, state};
use crate::otg_hs::interrupt::{RESET};

pub fn init_setaddress(address: u8) {
    // RM0456 Rev 5, p3423
    // 1. program the otg_dcfg register to set the device address.
    regs().dcfg().modify(|w| w.set_dad(address));
}

/// endpoint 0 read function
async fn read0(buf: &mut [u8]) -> Result<PhyState, PhyState> {
    trace!("read start len={}", buf.len());
    let r = regs();
    r.doepdma(0).write(|w| { w.set_dmaaddr(buf.as_ptr() as u32) });
    if regs().doepctl(0).read().epena() {
        defmt::error!("epena is set -- this should not happen");
    //     // clear epena
    //     r.doepctl(0).modify(|v| {
    //         v.set_epena(false);
    //     });
    }
    r.doeptsiz(0).modify(|v| {
        v.set_xfrsiz(buf.len() as _);
        v.set_stupcnt(3);
        v.set_pktcnt(1); // only one packet (due to register)
    });
    r.doepmsk().modify(|v| {
        v.set_stsphsrxm(true);
        v.set_xfrcm(true);
    });  // unmaks stsphsrxm and xfrcm

    // for dma mode, start transfer (for non-dma mode, this is done in the interrupt handler)
    r.doepctl(0).modify(|v| {
        v.set_epena(true);
        v.set_cnak(true);
    });
    // wait for transfer complete interrupt
    return poll_fn(|cx| {
        state().ep_out_wakers[0].register(cx.waker());
        if unsafe { RESET } { 
            // stop transfer if reset, the setup process will restart
            defmt::error!("read len={} reset", buf.len());
            return Poll::Ready(Err(PhyState::Reset));
        }
        if r.dsts().read().suspsts() {
            return Poll::Ready(Err(PhyState::Suspend));
        }
        if r.doepint(0).read().xfrc() {
            // transfer complete, clear xfrc (xfrc is masked in the interrupt handler)
            r.doepint(0).write(|w| {
                w.set_xfrc(true);
            });
            return Poll::Ready(Ok(PhyState::Active));
        } else if state().ep0_setup_ready.load(Ordering::Relaxed) {
            // setup packet received
            // state().ep0_setup_ready.store(false, Ordering::Release);
            return Poll::Ready(Ok(PhyState::Active));
        }
        else {
            Poll::Pending
        }
    }).await;
        // todo: handle errors for other interrupt states
}

async fn write0(buf: &[u8]) -> Result<PhyState, PhyState> {
    trace!("write start len={}, data={:x}", buf.len(), buf);
    let r = regs();
    r.diepdma(0)
        .write(|v| { v.set_dmaaddr(buf.as_ptr() as u32) });

    let pktcnt;
    if buf.len() == 0 {
        pktcnt = 1;
    } else {
        pktcnt = (buf.len() + 63) / 64;
    }
    r.dieptsiz(0).modify(|v| {
        v.set_xfrsiz(buf.len() as u32);
        v.set_pktcnt(pktcnt as _);
    });

    r.diepctl(0).modify(|w| {
        w.set_epena(true);
        w.set_cnak(true);
    });
    // wait for transfer complete interrupt
    return poll_fn(|cx| {
        state().ep_in_wakers[0].register(cx.waker());
        if r.dsts().read().suspsts() {
            return Poll::Ready(Err(PhyState::Suspend));
        }
        if unsafe { RESET } {
            return Poll::Ready(Err(PhyState::Reset));
        }
        if r.diepint(0).read().xfrc() {
            r.diepint(0).write(|w| w.set_xfrc(true)); // clear xfrc
            r.diepmsk().modify(|w| w.set_xfrcm(true)); // unmask
            return Poll::Ready(Ok(PhyState::Active));
        } else {
            Poll::Pending
        }
    })
        .await;
}

// pub enum BusEvent {
//     Reset,
//     Suspend,
//     Resume,
//     Disconnect,
// }

// pub fn wakeup_all() {
//     let state = state();
//     for waker in state.ep_in_wakers.iter() {
//         waker.wake();
//     }
//     for waker in state.ep_out_wakers.iter() {
//         waker.wake();
//     }
//     state.bus_waker.wake();
// }

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

use aligned::Aligned;
pub async fn setup_process_inner() -> Result<PhyState, PhyState> {
    let mut setup_data: Aligned<aligned::A4, [u8; 64]> = Aligned([0u8; 64]);
    unsafe {
        if state().ep0_setup_ready.load(Ordering::Relaxed) {
            state().ep0_setup_ready.store(false, Ordering::Release);
        }else {
            read0(&mut setup_data[0..64]).await?;
            poll_fn(|cx| {
                state().ep_out_wakers[0].register(cx.waker());
                if RESET  {
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
        }
        defmt::info!( "setup packet ready, processing package {:x}", setup_data[0..8]);
        let mut tmp = process_setup_packet_new(&setup_data[0..8]);
        if tmp.has_data_stage {
            match tmp.data_stage_direction {
                Direction::In => {
                    write0(&tmp.data[0..tmp.len]).await?;
                    // read0(&mut tmp.data[0..64]).await?; // status stage no data
                    read0(&mut setup_data[0..64]).await?;
                    return Ok(PhyState::Active);
                }
                Direction::Out => {
                    read0(&mut tmp.data[0..64]).await?;
                    write0(&[0u8; 0]).await? // status stage no data
                }
            };
        } else {
            // status stage no data
            match tmp.setup.direction {
                Direction::In => {
                    // read0(&mut buf[0..0]).await; // status stage no data
                    read0(&mut tmp.data[0..64]).await?; // status stage no data
                }
                Direction::Out => {
                    write0(&[0u8; 0]).await?; // status stage no data
                }
            };
        }

        match tmp.request {
            Request::SetAddress(addr) => {
                init_setaddress(addr);
            }
            Request::SetConfiguration(_) => {
                defmt::info!("SetConfiguration");
            }
            Request::SetLineCoding(_) => {
                defmt::info!("SetLineCoding");
            }
            Request::SetControlLineState(_) => {
                defmt::info!("SetControlLineState");
            }
            Request::ClearFeature(_) => {
                defmt::info!("ClearFeature");
            }
            _ => {
                defmt::panic!("Unknown request");
            }
        }
    }
    Ok(PhyState::Active)
}

