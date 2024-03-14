use core::cell::UnsafeCell;
use core::sync::atomic::{AtomicBool, AtomicU16, Ordering};

use defmt::info;
use embassy_sync::waitqueue::AtomicWaker;
use embassy_usb_driver::{Direction,
                         EndpointAddress,
                         EndpointAllocError, EndpointInfo, EndpointType};
use stm32_metapac::{otg, USB_OTG_FS};
use stm32_metapac::interrupt;

use crate::gpio::GpioPort;
use crate::usb_otg::bus::{Bus, start_irq};
use crate::usb_otg::control_pipe::ControlPipe;
use crate::usb_otg::endpoint::Endpoint;

mod bus;
mod endpoint;
mod control_pipe;

macro_rules! trace_bus_event {
    ($($arg:tt)*) => {
        defmt::trace!($($arg)*)
    };
}

const MAX_EP_COUNT: usize = 9;
const FIFO_DEPTH_WORDS: u16 = 1024;
const ENDPOINT_COUNT: usize = 9;

const RX_FIFO_EXTRA_SIZE_WORDS: u16 = 30;
const EP_OUT_BUFFER_EMPTY: u16 = u16::MAX;

/// USB driver config.
#[non_exhaustive]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Config {
    /// Enable VBUS detection.
    ///
    /// The USB spec requires USB devices monitor for USB cable plug/unplug and react accordingly.
    /// This is done by checkihg whether there is 5V on the VBUS pin or not.
    ///
    /// If your device is bus-powered (powers itself from the USB host via VBUS), then this is optional.
    /// (if there's no power in VBUS your device would be off anyway, so it's fine to always assume
    /// there's power in VBUS, i.e. the USB cable is always plugged in.)
    ///
    /// If your device is self-powered (i.e. it gets power from a source other than the USB cable, and
    /// therefore can stay powered through USB cable plug/unplug) then you MUST set this to true.
    ///
    /// If you set this to true, you must connect VBUS to PA9 for FS, PB13 for HS, possibly with a
    /// voltage divider. See ST application note AN4879 and the reference manual for more details.
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
    return stm32_metapac::USB_OTG_FS;
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

/// USB driver.
pub struct Driver {
    config: Config,
    // phantom: PhantomData<&'d mut T>,
    // ep_in: [Option<crate::usb::EndpointData>; MAX_EP_COUNT],
    ep_in: [Option<EndpointData>; MAX_EP_COUNT],
    ep_out: [Option<EndpointData>; MAX_EP_COUNT],
    // ep_out: [Option<crate::usb::EndpointData>; MAX_EP_COUNT],
    ep_out_buffer: [[u8; 1024]; MAX_EP_COUNT],
    ep_out_buffer_offset: usize,
    phy_type: PhyType,
}

pub struct State<const EP_COUNT: usize> {
    /// Holds received SETUP packets. Available if [State::ep0_setup_ready] is true.
    pub(crate) ep0_setup_data: [u8; 8],
    pub(crate) ep0_setup_ready: AtomicBool,
    pub(crate) ep_in_wakers: [AtomicWaker; EP_COUNT],
    // ep_in_wakers: [Signal<CriticalSectionRawMutex, u32>; EP_COUNT],
    pub(crate) ep_out_wakers: [AtomicWaker; EP_COUNT],
    // ep_out_wakers: [Signal<CriticalSectionRawMutex, u32>; EP_COUNT],
    /// RX FIFO is shared so extra buffers are needed to dequeue all data without waiting on each endpoint.
    /// Buffers are ready when associated [State::ep_out_size] != [EP_OUT_BUFFER_EMPTY].
    // ep_out_buffers: [UnsafeCell<*mut u8>; EP_COUNT],
    pub(crate) ep_out_buffers: [[u8; 1024]; EP_COUNT],
    pub(crate) ep_out_size: [AtomicU16; EP_COUNT],
    pub(crate) bus_waker: AtomicWaker,
}

impl Driver {
    pub fn new(
        config: Config,
        DP: GpioPort,
        DM: GpioPort,
    ) -> Self {
        DP.setup();
        DM.setup();
        Self {
            config,
            ep_in: [None; MAX_EP_COUNT],
            ep_out: [None; MAX_EP_COUNT],
            ep_out_buffer: [[0; 1024]; MAX_EP_COUNT],
            ep_out_buffer_offset: 0,
            phy_type: PhyType::InternalFullSpeed,
        }
    }

    // Returns total amount of words (u32) allocated in dedicated FIFO
    // fn allocated_fifo_words(&self) -> u16 {
    //     RX_FIFO_EXTRA_SIZE_WORDS + ep_fifo_size(&self.ep_out) + ep_fifo_size(&self.ep_in)
    // }

    fn alloc_endpoint(
        &mut self,
        ep_type: EndpointType,
        max_packet_size: u16,
        interval_ms: u8,
        dir: Direction,
    ) -> Result<Endpoint, EndpointAllocError> {
        defmt::trace!(
            "allocating type={:?} mps={:?} interval_ms={}, dir={:?}",
            ep_type,
            max_packet_size,
            interval_ms,
            dir
            // D::dir()
        );
        if ep_type == EndpointType::Control {
            return Ok(Endpoint {
                info: EndpointInfo {
                    addr: EndpointAddress::from_parts(0, dir),
                    ep_type,
                    max_packet_size,
                    interval_ms,
                },
            });
        }

        let eps = match dir {
            Direction::Out => &mut self.ep_out,
            Direction::In => &mut self.ep_in,
        };

        for i in 1..MAX_EP_COUNT {
            if eps[i].is_none() {
                eps[i] = Some(crate::usb_otg::EndpointData {
                    ep_type,
                    max_packet_size,
                    fifo_size_words: 64,
                });
                return Ok(Endpoint {
                    info: EndpointInfo {
                        addr: EndpointAddress::from_parts(i, dir),
                        ep_type,
                        max_packet_size,
                        interval_ms,
                    },
                });
            }
        }
        panic!("No free endpoints available");
    }
}


impl embassy_usb_driver::Driver<'_> for Driver {
    type EndpointOut = Endpoint;
    type EndpointIn = Endpoint;
    type ControlPipe = ControlPipe;
    type Bus = Bus;

    fn alloc_endpoint_in(
        &mut self,
        ep_type: EndpointType,
        max_packet_size: u16,
        interval_ms: u8,
    ) -> Result<Self::EndpointIn, EndpointAllocError> {
        self.alloc_endpoint(ep_type, max_packet_size, interval_ms, Direction::In)
    }

    fn alloc_endpoint_out(
        &mut self,
        ep_type: EndpointType,
        max_packet_size: u16,
        interval_ms: u8,
    ) -> Result<Self::EndpointOut, EndpointAllocError> {
        self.alloc_endpoint(ep_type, max_packet_size, interval_ms, Direction::Out)
    }

    fn start(mut self, control_max_packet_size: u16) -> (Self::Bus, Self::ControlPipe) {
        let ep_out = self.alloc_endpoint(EndpointType::Control, control_max_packet_size, 0, Direction::Out).unwrap();
        let ep_in = self.alloc_endpoint(EndpointType::Control, control_max_packet_size, 0, Direction::In).unwrap();
        // assert_eq!(ep_out.info.addr.index(), 0);
        // assert_eq!(ep_in.info.addr.index(), 0);

        defmt::trace!("start");

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
                ep_out,
                ep_in,
            },
        )
    }
}

unsafe fn on_interrupt() {
    let r = USB_OTG_FS;
    r.gintmsk().write(|_w| {});
    let ints = r.gintsts().read();
    // core interrupt status
    // info!("irq with state {:?}, and mask {:?}", ints.0, r.gintmsk().read().0);
    //show in hex
    defmt::info!(
        "irq with state {:08x}, and mask {:08x}",
        ints.0,
        r.gintmsk().read().0
    );
    if ints.wkupint() || ints.usbsusp() || ints.enumdne() || ints.otgint() || ints.srqint() || ints.usbrst()
    {
        // wakeup interrupt, suspend interrupt, enumeration speed done interrupt, OTG interrupt, Session request (SRQ) interrupt
        // Mask interrupts and notify `Bus` to process them
        state().bus_waker.wake();
        // STATE.bus_waker.wake();
        // BUS_WAKER_SIGNAL.signal(1);
        // T::state().bus_waker.wake();
    }
    // let state = &STATE;
    let mut state: &mut State<9> = state();

    // Handle RX
    while r.gintsts().read().rxflvl() {
        defmt::info!("rxflvl");
        // RX FIFO non-empty
        let status = r.grxstsp().read();
        // status read and popo pop register
        defmt::info!("=== status {:08x}", status.0);
        let ep_num = status.epnum() as usize;
        let len = status.bcnt() as usize;

        // assert!(ep_num < T::ENDPOINT_COUNT);
        // assert!(ep_num )

        match status.pktstsd() {
            stm32_metapac::otg::vals::Pktstsd::SETUP_DATA_RX => {
                // received SETUP packet
                defmt::trace!("SETUP_DATA_RX");
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
                    defmt::trace!("SETUP packet received, waking up");
                    state.ep_out_wakers[0].wake();
                } else {
                    defmt::error!("received SETUP before previous finished processing");
                    // discard FIFO
                    r.fifo(0).read();
                    r.fifo(0).read();
                }
            }
            stm32_metapac::otg::vals::Pktstsd::OUT_DATA_RX => {
                defmt::trace!("OUT_DATA_RX ep={} len={}", ep_num, len);

                if state.ep_out_size[ep_num].load(Ordering::Acquire) == EP_OUT_BUFFER_EMPTY {
                    // SAFETY: Buffer size is allocated to be equal to endpoint's maximum packet size
                    // We trust the peripheral to not exceed its configured MPSIZ
                    // let buf = unsafe {
                    //     core::slice::from_raw_parts_mut(
                    //         // *state.ep_out_buffers[ep_num].get(),
                    //         &mut STATE.ep_out_buffers[ep_num],
                    //         len,
                    //
                    //     )
                    // };
                    // let buf = &mut STATE.ep_out_buffers[ep_num];
                    // only n bytes of data are read from FIFO
                    let buf = &mut state.ep_out_buffers[ep_num][0..len];

                    // TODO: what if the len does not divide by 4?
                    for chunk in buf.chunks_mut(4) {
                        // RX FIFO is shared so always read from fifo(0)
                        let data = r.fifo(0).read().0;
                        chunk.copy_from_slice(&data.to_ne_bytes()[0..chunk.len()]);
                    }

                    state.ep_out_size[ep_num].store(len as u16, Ordering::Release);
                    state.ep_out_wakers[ep_num].wake();
                } else {
                    defmt::error!("ep_out buffer overflow index={}", ep_num);

                    // discard FIFO data
                    let len_words = (len + 3) / 4;
                    for _ in 0..len_words {
                        r.fifo(0).read().data();
                    }
                }
            }
            stm32_metapac::otg::vals::Pktstsd::OUT_DATA_DONE => {
                defmt::trace!("OUT_DATA_DONE ep={}", ep_num);
            }
            stm32_metapac::otg::vals::Pktstsd::SETUP_DATA_DONE => {
                defmt::trace!("SETUP_DATA_DONE ep={}", ep_num);

                // r.doepctl(ep_num).modify(|w| w.set_cnak(false));
                // TODO: waht happen here ?
                if quirk_setup_late_cnak(r) {
                    // Clear NAK to indicate we are ready to receive more data
                    r.doepctl(ep_num).modify(|w| w.set_cnak(false));
                }
            }
            x => {
                defmt::trace!("unknown PKTSTS: {}", x.to_bits());
            }
        }
    }

    // IN endpoint interrupt
    if ints.iepint() {
        defmt::info!("iepint");
        let mut ep_mask = r.daint().read().iepint();
        let mut ep_num = 0;

        // Iterate over endpoints while there are non-zero bits in the mask
        while ep_mask != 0 {
            if ep_mask & 1 != 0 {
                // get the interrupt mask
                defmt::info!(
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
                defmt::trace!("in ep={} irq val={:08x}", ep_num, ep_ints.0);
            }

            ep_mask >>= 1;
            ep_num += 1;
        }
    }

    if ints.wkupint() || ints.usbsusp() || ints.enumdne() || ints.otgint() || ints.srqint() || ints.usbrst()
    {} else {
        start_irq();
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

#[interrupt]
fn OTG_FS() {
    info!("OTG_FS interrupt");
    unsafe {
        on_interrupt();
    }
}

/// USB PHY type
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum PhyType {
    /// Internal Full-Speed PHY
    ///
    /// Available on most High-Speed peripherals.
    InternalFullSpeed,
    /// Internal High-Speed PHY
    ///
    /// Available on a few STM32 chips.
    InternalHighSpeed,
    /// External ULPI High-Speed PHY
    ExternalHighSpeed,
}

impl PhyType {
    /// Get whether this PHY is any of the internal types.
    pub fn internal(&self) -> bool {
        match self {
            PhyType::InternalFullSpeed | PhyType::InternalHighSpeed => true,
            PhyType::ExternalHighSpeed => false,
        }
    }

    /// Get whether this PHY is any of the high-speed types.
    pub fn high_speed(&self) -> bool {
        match self {
            PhyType::InternalFullSpeed => false,
            PhyType::ExternalHighSpeed | PhyType::InternalHighSpeed => true,
        }
    }

    pub(crate) fn to_dspd(&self) -> stm32_metapac::otg::vals::Dspd {
        match self {
            PhyType::InternalFullSpeed => stm32_metapac::otg::vals::Dspd::FULL_SPEED_INTERNAL,
            PhyType::InternalHighSpeed => stm32_metapac::otg::vals::Dspd::HIGH_SPEED,
            PhyType::ExternalHighSpeed => stm32_metapac::otg::vals::Dspd::HIGH_SPEED,
        }
    }
}

/// Indicates that [State::ep_out_buffers] is empty.

//
// /// USB OTG driver state.
#[derive(Debug, Clone, Copy)]
pub(crate) struct EndpointData {
    pub(crate) ep_type: EndpointType,
    pub(crate) max_packet_size: u16,
    pub(crate) fifo_size_words: u16,
}
