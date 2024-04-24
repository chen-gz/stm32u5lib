use core::{future::poll_fn, task::Poll};
use defmt::trace;
use stm32_metapac::otg::vals::Eptyp;
use crate::usb_otg_hs::global_states::{regs, state};
// use core::task::{Poll, poll_fn};


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
    Sleep,
    Error,
}

impl Endpoint {
    pub async fn read(&self, buf: &mut [u8]) -> Result<PhyState, PhyState> {
        trace!("read ep={:?}, len={:?}", self.addr, buf.len());
        // for control endpoint, pktcnt always 1 (only 1 bit in register) check the buf
        if self.addr == 0 && buf.len() > self.max_packet_size as usize {
            defmt::panic!("Endpoint 0 read error, buf.len() > max_packet_size");
        }
        let index = self.addr as usize;
        let r = regs();

        trace!("index: {:?}, doepdma: {:x}", index, r.doepdma(index).read().0);
        r.doepdma(index).write(|w| { w.set_dmaaddr(buf.as_mut_ptr() as u32) });

        let pktcnt: u16;
        if buf.len() == 0 {
            pktcnt = 1;
        } else {
            pktcnt = (buf.len() + 63) as u16 / 64u16
        }
        r.doeptsiz(index).modify(|w| {
            w.set_xfrsiz(buf.len() as _);
            w.set_pktcnt(pktcnt);
            if index == 0 {
                w.set_stupcnt(3);
            }
        });
        r.daintmsk().modify(|v| {
            v.set_oepm(v.oepm() | (1 << index));
        });

        // for dma this is required
        r.doepctl(index).modify(|w| {
            w.set_epena(true);
            w.set_cnak(true);
        });

        // enable interrupt
        // r.daintmsk().modify(|w| w.set_oepint(index as _)); // r.daintmsk().write(|w| { w.set_oepm((w.oepm() | index as u16) as _); });
        // wait for transfer complete interrupt
        match poll_fn(|cx| {
            state().ep_out_wakers[index].register(cx.waker());
            if r.dsts().read().suspsts() {
                return Poll::Ready(PhyState::Suspend);
            }
            if r.doepint(index).read().xfrc() {
                r.doepint(index).write(|w| w.set_xfrc(true));  // clear xfrc
                // In the interrupt handler, the `xfrc`  was masked to avoid re-entering the interrupt.
                r.doepmsk().modify(|w| w.set_xfrcm(true));
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
                trace!("read len={} suspend", buf.len());
                Err(PhyState::Suspend)
            }
            _ => {
                trace!("read len={} error", buf.len());
                Err(PhyState::Error)
            }
        }
    }

    pub async fn write(&self, buf: &[u8]) -> Result<PhyState, PhyState> {
        trace!("write ep={:?}, data={:?}", self.addr, buf);
        let r = regs();
        let index = self.addr as usize;
        r.diepdma(index).write(|w| { w.set_dmaaddr(buf.as_ptr() as u32) });
        let pktcnt;
        if buf.len() == 0 {
            pktcnt = 1;
        } else {
            pktcnt = (buf.len() + 63) / 64;
        }
        r.dieptsiz(index).modify(|w| {
            w.set_xfrsiz(buf.len() as u32);
            w.set_pktcnt(pktcnt as _);
        });
        r.daintmsk().modify(|v| {
            v.set_iepm(v.iepm() | (1 << index));
        });
        r.diepctl(index).modify(|w| {
            w.set_cnak(true);
            w.set_epena(true);
        });

        // wait for transfer complete interrupt
        match poll_fn(|cx| {
            state().ep_in_wakers[index].register(cx.waker());
            if r.dsts().read().suspsts() {
                return Poll::Ready(PhyState::Suspend);
            }
            if r.diepint(index).read().xfrc() {
                r.diepint(index).write(|w| w.set_xfrc(true)); // clear xfrc
                // In the interrupt handler, the `xfrc` was masked to avoid re-entering the interrupt.
                r.diepmsk().modify(|w| w.set_xfrcm(true));
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
                trace!("write len={} suspend", buf.len());
                Err(PhyState::Suspend)
            }
            _ => {
                trace!("write len={} error", buf.len());
                Err(PhyState::Error)
            }
        }
    }
    fn init(&self) {
        // this function can not be used for endpoint 0 for now.
        let index = self.addr as usize;
        let r = regs();
        match self.direction {
            Direction::In => {
                r.diepctl(index).modify(|w| {
                    w.set_usbaep(true);
                    w.set_mpsiz(self.max_packet_size.to_u16());
                    w.set_eptyp(self.ep_type.to_stm32());
                    w.set_stall(false);
                    w.set_txfnum(index as _);
                    w.set_snak(true);
                    // w.set_epena(true);
                });
            }
            Direction::Out => {
                r.doepctl(index).modify(|v| {
                    v.set_usbaep(true);
                    v.set_mpsiz(self.max_packet_size.to_u16());
                    v.set_eptyp(self.ep_type.to_stm32());
                    v.set_stall(false);
                    v.set_snak(true);
                    // v.set_epena(true);
                });
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