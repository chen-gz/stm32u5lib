/// The 'WORD' in the context of the USB peripheral is a 32-bit word.
use core::{
    cell::UnsafeCell,
    sync::atomic::{AtomicBool, AtomicU16, Ordering},
};

// use info;
use embassy_sync::waitqueue::AtomicWaker;
use embassy_usb_driver::{Direction, EndpointAddress, EndpointAllocError, EndpointInfo, EndpointType};
use stm32_metapac::{interrupt, otg};

use fifo_const::*;

use crate::{usart::USART1, *};
use crate::usb_otg::{
    bus::Bus,
    control_pipe::ControlPipe,
    endpoint::Endpoint,
    phy_type::PhyType,
};

mod bus;
mod endpoint;
mod control_pipe;
mod phy_type;

// map info, debug, trace, error to nothing

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


#[cfg(stm32u575)]
pub mod fifo_const {
    pub const MAX_EP_COUNT: usize = 6;
    // 6 endpoints include 0
    pub const FIFO_DEPTH_WORDS: u16 = 320;
    // total fifo size in words
    pub const RX_FIFO_SIZE_EACH: u16 = 64;
    // 64 bytes = 16 words
    pub const RX_FIFO_SIZE_SIZE_WORD: u16 = 96;
    pub const TX_FIFO_SIZE_WORDS: [u16; 6] = [40, 40, 40, 40, 40, 24];
}

#[cfg(stm32u5a5)]
pub mod fifo_const {
    pub const MAX_EP_COUNT: usize = 9;
    pub const FIFO_DEPTH_WORDS: u16 = 1024;
    //4Kbtes = 4096 bytes = 1024 words
    // total fifo size in words
    pub const RX_FIFO_SIZE_EACH: u16 = 128; // 128 bytes = 32 words
    pub const RX_FIFO_SIZE_SIZE_WORD: u16 = 288; // 32 * 9 = 288
    // 1024 - 288 = 736; 736 / 9 = 81.7777 =320 bytes = 80 words
    // 1024 - 288 - 80 * 8 = 96
    pub const TX_FIFO_SIZE_WORDS: [u16; 9] = [80, 80, 80, 80, 80, 80, 80, 80, 
    96];
}


const EP_OUT_BUFFER_EMPTY: u16 = u16::MAX;

/// USB driver config.
#[non_exhaustive]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Config {
    pub vbus_detection: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            vbus_detection: true,
        }
    }
}

fn quirk_setup_late_cnak(r: stm32_metapac::otg::Otg) -> bool {
    r.cid().read().0 & 0xf000 == 0x1000
}

static mut STATE: State<MAX_EP_COUNT> = State::new();

fn state() -> &'static mut State<MAX_EP_COUNT> {
    // static STATE: State<MAX_EP_COUNT> = State::new();
    unsafe { &mut STATE }
}

pub(crate) fn regs() -> otg::Otg {
    #[cfg(stm32u575)]
    return stm32_metapac::USB_OTG_FS;
    #[cfg(stm32u5a5)]
    return stm32_metapac::USB_OTG_HS;
}

unsafe impl<const EP_COUNT: usize> Send for State<EP_COUNT> {}

unsafe impl<const EP_COUNT: usize> Sync for State<EP_COUNT> {}

impl<const EP_COUNT: usize> State<EP_COUNT> {
    /// Create a new State.
    pub const fn new() -> Self {
        const NEW_AW: AtomicWaker = AtomicWaker::new();
        const NEW_BUF: UnsafeCell<*mut u8> = UnsafeCell::new(0 as _);
        const NEW_SIZE: AtomicU16 = AtomicU16::new(EP_OUT_BUFFER_EMPTY);

        Self {
            // ep0_setup_data: UnsafeCell::new([0u8; 8]),
            ep0_setup_data: [0u8; 8],
            ep0_setup_ready: AtomicBool::new(false),
            ep_in_wakers: [NEW_AW; EP_COUNT],
            ep_out_wakers: [NEW_AW; EP_COUNT],
            ep_out_buffers: [[0u8; 1024]; EP_COUNT],
            ep_out_size: [NEW_SIZE; EP_COUNT],
            bus_waker: NEW_AW,
        }
    }
}

pub struct State<const EP_COUNT: usize> {
    /// Holds received SETUP packets. Available if [State::ep0_setup_ready] is true.
    pub(crate) ep0_setup_data: [u8; 8],
    pub(crate) ep0_setup_ready: AtomicBool,
    pub(crate) ep_in_wakers: [AtomicWaker; EP_COUNT],
    pub(crate) ep_out_wakers: [AtomicWaker; EP_COUNT],
    pub(crate) ep_out_buffers: [[u8; 1024]; EP_COUNT],
    pub(crate) ep_out_size: [AtomicU16; EP_COUNT],
    pub(crate) bus_waker: AtomicWaker,
}

/// USB driver.
pub struct Driver {
    config: Config,
    ep_in: [Option<EndpointData>; MAX_EP_COUNT],
    ep_out: [Option<EndpointData>; MAX_EP_COUNT],
    ep_out_buffer: [[u8; 1024]; MAX_EP_COUNT],
    phy_type: PhyType,
}


impl Driver {
    pub fn new(
        config: Config,
        DP: gpio::GpioPort,
        DM: gpio::GpioPort,
    ) -> Self {
        DP.setup();
        DM.setup();
        Self {
            config,
            ep_in: [None; MAX_EP_COUNT],
            ep_out: [None; MAX_EP_COUNT],
            ep_out_buffer: [[0; 1024]; MAX_EP_COUNT],
            phy_type: PhyType::InternalFullSpeed,
        }
    }
    fn alloc_endpoint(
        &mut self,
        ep_type: EndpointType,
        max_packet_size: u16,
        interval_ms: u8,
        dir: Direction,
    ) -> Result<Endpoint, EndpointAllocError> {
        let eps = match dir {
            Direction::Out => &mut self.ep_out,
            Direction::In => &mut self.ep_in
        };

        let mut slot: isize = -1;
        for i in 1..eps.len() {
            if eps[i].is_none() {
                slot = i as isize;
                break;
            }
        }
        if slot == -1 {
            panic!("No free endpoints available");
        }
        let slot = slot as usize;

        match dir {
            Direction::Out => assert!(max_packet_size <= RX_FIFO_SIZE_EACH * 4),
            Direction::In => assert!(max_packet_size <= TX_FIFO_SIZE_WORDS[slot] * 4), // 0 is control
        };
        eps[slot] = Some(EndpointData {
            ep_type,
            max_packet_size,
        });
        return Ok(Endpoint {
            info: EndpointInfo {
                addr: EndpointAddress::from_parts(slot, dir),
                ep_type,
                max_packet_size,
                interval_ms,
            },
        });
    }
}


impl embassy_usb_driver::Driver<'_> for Driver {
    type EndpointOut = Endpoint;
    type EndpointIn = Endpoint;
    type ControlPipe = ControlPipe;
    type Bus = Bus;

    fn alloc_endpoint_in(&mut self, ep_type: EndpointType, max_packet_size: u16, interval_ms: u8)
                         -> Result<Self::EndpointIn, EndpointAllocError> {
        self.alloc_endpoint(ep_type, max_packet_size, interval_ms, Direction::In)
    }

    fn alloc_endpoint_out(&mut self, ep_type: EndpointType, max_packet_size: u16, interval_ms: u8)
                          -> Result<Self::EndpointOut, EndpointAllocError> {
        self.alloc_endpoint(ep_type, max_packet_size, interval_ms, Direction::Out)
    }

    fn start(mut self, control_max_packet_size: u16) -> (Self::Bus, Self::ControlPipe) {
        // assert!(control_max_packet_size in );
        // control_max_packaet_size should be 8, 16, 32, 64
        USART1.send("control_max_packet_size: \t\n".as_bytes());
        assert!(control_max_packet_size == 8 || control_max_packet_size == 16
            || control_max_packet_size == 32 || control_max_packet_size == 64);
        self.ep_in[0] = Some(EndpointData {
            ep_type: EndpointType::Control,
            max_packet_size: control_max_packet_size,
        });
        self.ep_out[0] = Some(EndpointData {
            ep_type: EndpointType::Control,
            max_packet_size: control_max_packet_size,
        });
        trace!("start");
        (
            Bus {
                config: self.config,
                ep_in: self.ep_in,
                ep_out: self.ep_out,
                phy_type: self.phy_type,
                inited: false,
            },
            ControlPipe {
                max_packet_size: control_max_packet_size,
                ep_out: Endpoint {
                    info: EndpointInfo {
                        addr: EndpointAddress::from_parts(0, Direction::Out),
                        ep_type: EndpointType::Control,
                        max_packet_size: control_max_packet_size,
                        interval_ms: 0,
                    },
                },
                ep_in: Endpoint {
                    info: EndpointInfo {
                        addr: EndpointAddress::from_parts(0, Direction::In),
                        ep_type: EndpointType::Control,
                        max_packet_size: control_max_packet_size,
                        interval_ms: 0,
                    },
                },
            },
        )
    }
}

unsafe fn on_interrupt() {
    let r = regs();
    // r.gahbcfg().modify(|w| w.set_dmaen(val));
    // r.gintmsk().write(|_w| {});
    let ints = r.gintsts().read();
    if ints.wkupint() || ints.usbsusp() || ints.enumdne() || ints.otgint() || ints.srqint() || ints.usbrst()
    {
        // wakeup interrupt, suspend interrupt, enumeration speed done interrupt, OTG interrupt, Session request (SRQ) interrupt
        // Mask interrupts and notify `Bus` to process them
        info!("bus_waker wake and mask all interrupt");
        r.gintmsk().write(|w| w.0 = 0); // 0 mask all
        state().bus_waker.wake();
        // the interrupt should be re-enabled by the bus after bus processing (in bus task functions)
    }
    // let state = &STATE;
    let mut state: &mut State<MAX_EP_COUNT> = state();

    // Handle RX
    while r.gintsts().read().rxflvl() {
        info!("rxflvl");
        // RX FIFO non-empty
        let status = r.grxstsp().read();
        // status read and popo pop register
        info!("=== status {:08x}", status.0);
        let ep_num = status.epnum() as usize;
        let len = status.bcnt() as usize;

        // assert!(ep_num < T::ENDPOINT_COUNT);
        // assert!(ep_num )

        match status.pktstsd() {
            stm32_metapac::otg::vals::Pktstsd::SETUP_DATA_RX => {
                // received SETUP packet
                trace!("SETUP_DATA_RX");
                assert!(len == 8, "invalid SETUP packet length={}", len);
                assert!(ep_num == 0, "invalid SETUP packet endpoint={}", ep_num);

                // flushing TX if something stuck in control endpoint
                if r.dieptsiz(ep_num).read().pktcnt() != 0 {
                    r.grstctl().modify(|w| {
                        w.set_txfnum(ep_num as _);
                        w.set_txfflsh(true);
                    });
                    while r.grstctl().read().txfflsh() {}
                }

                if state.ep0_setup_ready.load(Ordering::Relaxed) == false {
                    //                     // SAFETY: exclusive access ensured by atomic bool
                    // let data = unsafe { &mut *state.ep0_setup_data.get() };
                    unsafe {
                        state.ep0_setup_data[0..4].copy_from_slice(&r.fifo(0).read().0.to_ne_bytes());
                        state.ep0_setup_data[4..8].copy_from_slice(&r.fifo(0).read().0.to_ne_bytes());
                    }
                    // data[0..4].copy_from_slice(&r.fifo(0).read().0.to_ne_bytes());
                    // data[4..8].copy_from_slice(&r.fifo(0).read().0.to_ne_bytes());
                    state.ep0_setup_ready.store(true, Ordering::Release);
                    trace!("SETUP packet received, waking up");
                    state.ep_out_wakers[0].wake();
                } else {
                    error!("received SETUP before previous finished processing");
                    // discard FIFO
                    r.fifo(0).read();
                    r.fifo(0).read();
                }
            }
            stm32_metapac::otg::vals::Pktstsd::OUT_DATA_RX => {
                trace!("OUT_DATA_RX ep={} len={}", ep_num, len);
                // received OUT data
                if state.ep_out_size[ep_num].load(Ordering::Acquire) == EP_OUT_BUFFER_EMPTY {
                    // only n bytes of data are read from FIFO
                    let buf = &mut state.ep_out_buffers[ep_num][0..len];

                    for chunk in buf.chunks_mut(4) {
                        // RX FIFO is shared so always read from fifo(0)
                        let data = r.fifo(0).read().0;
                        chunk.copy_from_slice(&data.to_ne_bytes()[0..chunk.len()]);
                    }

                    state.ep_out_size[ep_num].store(len as u16, Ordering::Release);
                    state.ep_out_wakers[ep_num].wake();
                } else {
                    error!("ep_out buffer overflow index={}", ep_num);
                    // discard FIFO data
                    let len_words = (len + 3) / 4;
                    for _ in 0..len_words {
                        r.fifo(0).read().data();
                    }
                }
                // todo: this function read the data from receive fifo and copy to the buffer (ep_out_buffers[ep_num])
                // replease ep_out_wakers[ep_num] to as queue and remove locker
            }
            stm32_metapac::otg::vals::Pktstsd::OUT_DATA_DONE => {
                trace!("OUT_DATA_DONE ep={}", ep_num);
            }
            stm32_metapac::otg::vals::Pktstsd::SETUP_DATA_DONE => {
                trace!("SETUP_DATA_DONE ep={}", ep_num);

                // r.doepctl(ep_num).modify(|w| w.set_cnak(false));
                // TODO: waht happen here ?
                if quirk_setup_late_cnak(r) {
                    // Clear NAK to indicate we are ready to receive more data
                    r.doepctl(ep_num).modify(|w| w.set_cnak(false));
                }
            }
            x => {
                trace!("unknown PKTSTS: {}", x.to_bits());
            }
        }
    }

    // IN endpoint interrupt
    if ints.iepint() {
        info!("iepint");
        let mut ep_mask = r.daint().read().iepint();
        let mut ep_num = 0;

        // Iterate over endpoints while there are non-zero bits in the mask
        while ep_mask != 0 {
            if ep_mask & 1 != 0 {
                // get the interrupt mask
                info!(
                    "device in endpoint interrupt mask: {:08x}",
                    r.diepmsk().read().0
                );

                let ep_ints = r.diepint(ep_num).read();

                // clear all
                r.diepint(ep_num).write_value(ep_ints);

                // TXFE is cleared in DIEPEMPMSK
                if ep_ints.txfe() {
                    critical_section::with(|_| {
                        r.diepempmsk().modify(|w| {
                            w.set_ineptxfem(w.ineptxfem() & !(1 << ep_num));
                        });
                    });
                }

                state.ep_in_wakers[ep_num].wake();
                trace!("in ep={} irq val={:08x}", ep_num, ep_ints.0);
            }

            ep_mask >>= 1;
            ep_num += 1;
        }
    }


    // not needed? reception handled in rxflvl
    // OUT endpoint interrupt
    // if ints.oepint() {
    //     let mut ep_mask = r.daint().read().oepint();
    //     let mut ep_num = 0;

    //     while ep_mask != 0 {
    //         if ep_mask & 1 != 0 {
    //             let ep_ints = r.doepint(ep_num).read();
    //             // clear all
    //             r.doepint(ep_num).write_value(ep_ints);
    //             state.ep_out_wakers[ep_num].wake();
    //             trace!("out ep={} irq val={:08x}", ep_num, ep_ints.0);
    //         }

    //         ep_mask >>= 1;
    //         ep_num += 1;
    //     }
    // }
    // }
}


#[cfg(stm32u575)]
#[interrupt]
fn OTG_FS() {
    info!("OTG_FS interrupt");
    unsafe {
        on_interrupt();
        clock::delay_us(200);
    }
}
#[cfg(stm32u5a5)]
#[interrupt]
fn OTG_HS() {
    info!("OTG_HS interrupt");
    unsafe {
        on_interrupt();
        clock::delay_us(200);
    }
}

/// Indicates that [State::ep_out_buffers] is empty.

//
// /// USB OTG driver state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct EndpointData {
    pub(crate) ep_type: EndpointType,
    pub(crate) max_packet_size: u16, // this should not exceed fifo size
    // pub(crate) fifo_size_words: u16,
}
