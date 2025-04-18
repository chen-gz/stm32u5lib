use core::{future::poll_fn, task::Poll};
use core::sync::atomic::Ordering;
use defmt::trace;
use stm32_metapac::otg::vals::Eptyp;
use crate::otg_fs::global_states::{regs, state};
// use core::task::{Poll, poll_fn};
use crate::otg_fs::interrupt::RESET;

pub enum Direction {
    In,
    Out,
}

pub enum EpType {
    Control,
    Isochronous,
    Bulk,
    Interrupt,
}

impl EpType {
    pub fn to_stm32(&self) -> Eptyp {
        match self {
            EpType::Control => Eptyp::CONTROL,
            EpType::Isochronous => Eptyp::ISOCHRONOUS,
            EpType::Bulk => Eptyp::BULK,
            EpType::Interrupt => Eptyp::INTERRUPT,
        }
    }
}

#[derive(Copy, Clone)]
pub enum MaxPacketSize {
    Size8 = 8,
    Size16 = 16,
    Size32 = 32,
    Size64 = 64,
    Size128 = 128,
    Size256 = 256,
    Size512 = 512,
    Size1023 = 1023,
}

impl MaxPacketSize {
    pub fn to_u16(&self) -> u16 {
        *self as u16
    }
}

pub struct Endpoint {
    pub direction: Direction,
    pub addr: u8,
    pub ep_type: EpType,
    pub max_packet_size: MaxPacketSize,
    pub interval: u8,
}

impl Endpoint {
    pub fn new(direction: Direction, addr: u8, ep_type: EpType, max_packet_size: MaxPacketSize, interval: u8) -> Result<Self, &'static str> {
        // Check the condition: if addr is 0, max_packet_size should be <= 64
        if addr == 0 && max_packet_size.to_u16() > 64 {
            return Err("Max packet size must be <= 64 when addr is 0.");
        }

        Ok(Endpoint {
            direction,
            addr,
            ep_type,
            max_packet_size,
            interval,
        })
    }
}

pub enum PhyState {
    Active,
    Reset,
    Init,
    Suspend,
    Error,
}


impl Endpoint {
    // pub async fn read(&self, buf: &mut [u8]) -> Result<usize, PhyState> {
    pub async fn read(&self, buf: &mut [u8]) -> Result<usize, PhyState> {
        // limit to a single packet for now
        assert!(buf.len() <= 64);
        let buf_addr = buf.as_mut_ptr() as u32;
        let len = buf.len();
        let index = self.addr as usize;
        let pktcnt = if len == 0 { 1 } else { (len + 63) / 64 };
        self.transfer_parameter_check(buf_addr, len, pktcnt as u32);

        let r = regs();
        poll_fn(|cv| {
            state().ep_out_wakers[index].register(cv.waker());
            let r = regs();
            if !r.doepctl(index).read().usbaep() || r.dsts().read().suspsts() {
                return Poll::Pending;
            } else {
                return Poll::Ready(PhyState::Active);
            }
        }).await;

        // r.doepdma(index).write(|w| { w.set_dmaaddr(buf.as_mut_ptr() as u32) });
        r.doeptsiz(index).modify(|w| {
            w.set_xfrsiz(buf.len() as _);
            if index == 0 {
                w.set_rxdpid_stupcnt(3); // set the setup packet count to 3 (control transfer)
            }
            else {
                w.set_pktcnt(pktcnt as _);
            }
        });
        r.daintmsk().modify(|v| {
            v.set_oepm(v.oepm() | (1 << index)); // unmask the endpoint interrupt
        });

        r.doepmsk().modify(|v| {
            // v.set_stsphsrxm(true);
            v.set_xfrcm(true);
        });  // unmaks stsphsrxm and xfrcm

        // for dma this is required
        r.doepctl(index).modify(|w| {
            w.set_epena(true);
            w.set_cnak(true);
        });
        poll_fn(|cx| {
            state().ep_out_wakers[index].register(cx.waker());
            if unsafe {RESET} {
                defmt::error!("read len={} reset", buf.len());
                return Poll::Ready(Err(PhyState::Reset));
            }
            defmt::info!("read ep={:?}, doepctl: {:x}", self.addr, r.doepctl(index).read().0);
            if r.dsts().read().suspsts() {
                return Poll::Ready(Err(PhyState::Suspend));
            }
            if index == 0 {
                return if r.doepint(0).read().xfrc() {
                    // transfer complete, clear xfrc (xfrc is masked in the interrupt handler)
                    r.doepint(0).write(|w| {
                        w.set_xfrc(true);
                    });
                    // copy from state
                    buf.copy_from_slice(&state().ep_out_buffers[index][0..len]);
                    // Poll::Ready(Ok(PhyState::Active))
                    Poll::Ready(Ok(index))
                } else if state().ep0_setup_ready.load(Ordering::Relaxed) {
                    // setup packet received
                    // state().ep0_setup_ready.store(false, Ordering::Release);
                    buf.copy_from_slice(&state().ep_out_buffers[index][0..len]);
                    // Poll::Ready(Ok(PhyState::Active))
                    Poll::Ready(Ok(index))
                } else {
                    Poll::Pending
                }
            }
            if r.doepint(index).read().xfrc() {
                r.doepint(index).write(|w| w.set_xfrc(true));  // clear xfrc
                // In the interrupt handler, the `xfrc`  was masked to avoid re-entering the interrupt.
                r.doepmsk().modify(|w| w.set_xfrcm(true));
                // get the length of the data
                // copy from state
                buf.copy_from_slice(&state().ep_out_buffers[index][0..len]);
                let len_rest = r.doeptsiz(index).read().xfrsiz() as usize;
                return Poll::Ready(Ok(buf.len() - len_rest));
                // return Poll::Ready(Ok(buf.len()));
            } else {
                Poll::Pending
            }
        }).await
    }

    fn transfer_parameter_check(&self, addr: u32, len: usize, pktcnt: u32) {
        // the buffer should be aligned to 32 bits (4 bytes)
        if addr % 4 != 0 {
            defmt::panic!("Buffer is not aligned to 32 bits");
        }
        if len > 0x7FFFF || pktcnt > 0x3FF {
            defmt::panic!("Buffer size is too large");
        }
    }
    // pub async fn write(&self, addr: u32, len: usize) -> Result<PhyState, PhyState> {
    pub async fn write(&self, buf: &[u8]) -> Result<PhyState, PhyState> {
        // todo: if transfer size equals to max_packet_size, send a zero length packet to indicate the end of the transfer
        let len = buf.len();
        let addr = buf.as_ptr() as u32;
        let index = self.addr as usize;
        let pktcnt = if len == 0 { 1 } else { (len + 63) / 64 };
        self.transfer_parameter_check(addr, len, pktcnt as u32);
        trace!("write ep={:?}, len = {:?}, data={:?}, pktcnt={:?}", self.addr, len, buf, pktcnt);
        let r = regs();
        r.dieptsiz(index).modify(|w| {
            if index != 0 {
                w.set_mcnt(1); // this is for periodic transfer only. Leave it as 1 for now.
            }
            w.set_xfrsiz(len as u32);
            w.set_pktcnt(pktcnt as _);
        });
        // if last transmit is not complete, return unfinished
        // if r.diepctl(index).read().epena() {
        //     todo!("this will cause bug. After a while, it will trigger( but don't known why)");
        //     defmt::info!("ep last transmit is not complete, ep={:?}", self.addr);
        //     return Err(PhyState::Active);
        // }
        // r.diepdma(index).write(|w| { w.set_dmaaddr(addr) });
        r.daintmsk().modify(|v| { v.set_iepm(v.iepm() | (1 << index)); });
        // write data to the fifo

        r.diepctl(index).modify(|w| {
            w.set_cnak(true);
            w.set_epena(true);
        });

        // for val in buf {
        //     r.fifo(index).write_value(regs::Fifo(*val as u32));
        // }
        for chunk in buf.chunks(4) {
            let mut word = 0u32;
            for (j, &byte) in chunk.iter().enumerate() {
                word |= (byte as u32) << (j * 8);
            }
            defmt::info!("write ep={:?}, word={:x}", self.addr, word);
            // r.fifo(2).write_value(regs::Fifo(word));
            r.fifo(self.addr as usize).write(|w|  w.0 = word );
        }
        // Ok(PhyState::Active)
        // wait for transfer complete interrupt
        poll_fn(|cx| {
            state().ep_in_wakers[index].register(cx.waker());
            if r.dsts().read().suspsts() {
                return Poll::Ready(Err(PhyState::Suspend));
            }
            if unsafe {RESET} {
                return Poll::Ready(Err(PhyState::Reset));
            }
            if !r.diepctl(index).read().usbaep() {
                return Poll::Ready(Err(PhyState::Error));
            }
            // if the endpoint is not enabled, and nak been set, return error
            if r.diepint(index).read().xfrc() {
                r.diepint(index).write(|w| w.set_xfrc(true)); // clear xfrc
                // In the interrupt handler, the `xfrc` was masked to avoid re-entering the interrupt.
                r.diepmsk().modify(|w| w.set_xfrcm(true)); // unmask
                // Poll::Ready(())
                return Poll::Ready(Ok(PhyState::Active));
            } else {
                Poll::Pending
            }
        })
            .await
        // {
        //     PhyState::Active => {
        //         // trace!("write len={} done", buf.len());
        //         Ok(PhyState::Active)
        //     }
        //     PhyState::Suspend => {
        //         // trace!("write len={} suspend", buf.len());
        //         Err(PhyState::Suspend)
        //     }
        //     _ => {
        //         // trace!("write len={} error", buf.len());
        //         Err(PhyState::Error)
        //     }
        // }
    }
    fn init(&self) {
        // this function can not be used for endpoint 0 for now.
        let index = self.addr as usize;
        let r = regs();
        match self.direction {
            Direction::In => {
                defmt::info!("init endpoint {:?} in, doepctl: {:x}", self.addr, r.diepctl(index).read().0);
                r.diepctl(index).modify(|w| {
                    w.set_usbaep(true);
                    w.set_mpsiz(self.max_packet_size.to_u16());
                    w.set_eptyp(self.ep_type.to_stm32());
                    w.set_stall(false);
                    w.set_txfnum(index as _);
                    // w.set_snak(true);
                    // w.set_epena(true);
                });
                if r.diepctl(index).read().epena() {
                    r.diepctl(index).modify(|w| {
                        w.set_cnak(true);
                    });
                }
                defmt::info!("init endpoint {:?} in, doepctl: {:x}", self.addr, r.diepctl(index).read().0);
                // flush the fifo
                r.grstctl().modify(|w| {
                    w.set_txfnum(index as _);
                    w.set_txfflsh(true);
                });
            }

            Direction::Out => {
                defmt::info!("init endpoint {:?} out, doepctl: {:x}", self.addr, r.doepctl(index).read().0);
                r.doepctl(index).modify(|v| {
                    v.set_usbaep(true);
                    v.set_mpsiz(self.max_packet_size.to_u16());
                    v.set_eptyp(self.ep_type.to_stm32());
                    v.set_stall(false);
                    // v.set_snak(true);
                    // v.set_epena(true);
                });
                if r.doepctl(index).read().epena() {
                    r.doepctl(index).modify(|w| {
                        w.set_cnak(true);
                    });
                }
                defmt::info!("init endpoint {:?} out, doepctl: {:x}", self.addr, r.doepctl(index).read().0);
            }
        }
    }
}

// RM0456 Rev 5 P3423
pub fn init_endpoint() {
    defmt::info!("init_endpoints");
    // this is for cdc for now
    let ep1 = Endpoint::new(Direction::In, 1, EpType::Interrupt, MaxPacketSize::Size64, 10).unwrap();
    let ep2_in = Endpoint::new(Direction::In, 2, EpType::Bulk, MaxPacketSize::Size64, 0).unwrap();
    let ep2_out = Endpoint::new(Direction::Out, 2, EpType::Bulk, MaxPacketSize::Size64, 0).unwrap();

    // configure the endpoint
    ep1.init();
    ep2_in.init();
    ep2_out.init();
}
