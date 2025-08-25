// #![allow(async_fn_in_trait)]
// #![doc = include_str!("../README.md")]
// #![warn(missing_docs)]

// This must go FIRST so that all the other modules see its macros.

pub mod endpoint;
pub mod controlpipe;
pub mod bus;
pub mod interrupt;
pub mod driver;
pub mod reg;

use crate::usb::reg::{vals, Otg};
use core::cell::UnsafeCell;
use core::sync::atomic::{AtomicBool, AtomicU16, AtomicU32};

use embassy_sync::waitqueue::AtomicWaker;
use embassy_usb_driver::{
    Direction,
    EndpointType,
};



// use stm32_metapac::otg::Otg;
// use stm32_metapac::otg::vals;
// use stm32_metapac::otg::regs;
use crate::usb::reg::regs;



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
    /// External ULPI Full-Speed PHY (or High-Speed PHY in Full-Speed mode)
    ExternalFullSpeed,
    /// External ULPI High-Speed PHY
    ExternalHighSpeed,
}

impl PhyType {
    /// Get whether this PHY is any of the internal types.
    pub fn internal(&self) -> bool {
        match self {
            PhyType::InternalFullSpeed | PhyType::InternalHighSpeed => true,
            PhyType::ExternalHighSpeed | PhyType::ExternalFullSpeed => false,
        }
    }

    /// Get whether this PHY is any of the high-speed types.
    pub fn high_speed(&self) -> bool {
        match self {
            PhyType::InternalFullSpeed | PhyType::ExternalFullSpeed => false,
            PhyType::ExternalHighSpeed | PhyType::InternalHighSpeed => true,
        }
    }

    fn to_dspd(&self) -> vals::Dspd {
        match self {
            PhyType::InternalFullSpeed => vals::Dspd::FULL_SPEED_INTERNAL,
            PhyType::InternalHighSpeed => vals::Dspd::HIGH_SPEED,
            PhyType::ExternalFullSpeed => vals::Dspd::FULL_SPEED_EXTERNAL,
            PhyType::ExternalHighSpeed => vals::Dspd::HIGH_SPEED,
        }
    }
}

/// Indicates that [State::ep_out_buffers] is empty.
const EP_OUT_BUFFER_EMPTY: u16 = u16::MAX;

pub(crate) struct EpState {
    in_waker: AtomicWaker,
    out_waker: AtomicWaker,
    /// RX FIFO is shared so extra buffers are needed to dequeue all data without waiting on each endpoint.
    /// Buffers are ready when associated [State::ep_out_size] != [EP_OUT_BUFFER_EMPTY].
    out_buffer: UnsafeCell<*mut u8>,
    out_size: AtomicU16,
}

// SAFETY: The EndpointAllocator ensures that the buffer points to valid memory exclusive for each endpoint and is
// large enough to hold the maximum packet size. Access to the buffer is synchronized between the USB interrupt and the
// EndpointOut impl using the out_size atomic variable.
unsafe impl Send for EpState {}
unsafe impl Sync for EpState {}

pub(crate) struct ControlPipeSetupState {
    /// Holds received SETUP packets. Available if [Ep0State::setup_ready] is true.
    setup_data: [AtomicU32; 2],
    setup_ready: AtomicBool,
}

/// USB OTG driver state.
pub struct State<const EP_COUNT: usize> {
    cp_state: ControlPipeSetupState,
    ep_states: [EpState; EP_COUNT],
    bus_waker: AtomicWaker,
}

unsafe impl<const EP_COUNT: usize> Send for State<EP_COUNT> {}
unsafe impl<const EP_COUNT: usize> Sync for State<EP_COUNT> {}

impl<const EP_COUNT: usize> State<EP_COUNT> {
    /// Create a new State.
    pub const fn new() -> Self {
        Self {
            cp_state: ControlPipeSetupState {
                setup_data: [const { AtomicU32::new(0) }; 2],
                setup_ready: AtomicBool::new(false),
            },
            ep_states: [const {
                EpState {
                    in_waker: AtomicWaker::new(),
                    out_waker: AtomicWaker::new(),
                    out_buffer: UnsafeCell::new(0 as _),
                    out_size: AtomicU16::new(EP_OUT_BUFFER_EMPTY),
                }
            }; EP_COUNT],
            bus_waker: AtomicWaker::new(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct EndpointData {
    ep_type: EndpointType,
    max_packet_size: u16,
    fifo_size_words: u16,
}

/// USB driver config.
#[non_exhaustive]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Config {
    /// Enable VBUS detection.
    ///
    /// The USB spec requires USB devices monitor for USB cable plug/unplug and react accordingly.
    /// This is done by checking whether there is 5V on the VBUS pin or not.
    ///
    /// If your device is bus-powered (powers itself from the USB host via VBUS), then this is optional.
    /// (If there's no power in VBUS your device would be off anyway, so it's fine to always assume
    /// there's power in VBUS, i.e. the USB cable is always plugged in.)
    ///
    /// If your device is self-powered (i.e. it gets power from a source other than the USB cable, and
    /// therefore can stay powered through USB cable plug/unplug) then you MUST set this to true.
    ///
    /// If you set this to true, you must connect VBUS to PA9 for FS, PB13 for HS, possibly with a
    /// voltage divider. See ST application note AN4879 and the reference manual for more details.
    pub vbus_detection: bool,

    /// Enable transceiver delay.
    ///
    /// Some ULPI PHYs like the Microchip USB334x series require a delay between the ULPI register write that initiates
    /// the HS Chirp and the subsequent transmit command, otherwise the HS Chirp does not get executed and the deivce
    /// enumerates in FS mode. Some USB Link IP like those in the STM32H7 series support adding this delay to work with
    /// the affected PHYs.
    pub xcvrdly: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            vbus_detection: false,
            xcvrdly: false,
        }
    }
}

/// USB endpoint direction.
trait Dir {
    /// Returns the direction value.
    fn dir() -> Direction;
}

/// Marker type for the "IN" direction.
pub enum In {}
impl Dir for In {
    fn dir() -> Direction {
        Direction::In
    }
}

/// Marker type for the "OUT" direction.
pub enum Out {}
impl Dir for Out {
    fn dir() -> Direction {
        Direction::Out
    }
}



/// Translates HAL [EndpointType] into PAC [vals::Eptyp]
fn to_eptyp(ep_type: EndpointType) -> vals::Eptyp {
    match ep_type {
        EndpointType::Control => vals::Eptyp::CONTROL,
        EndpointType::Isochronous => vals::Eptyp::ISOCHRONOUS,
        EndpointType::Bulk => vals::Eptyp::BULK,
        EndpointType::Interrupt => vals::Eptyp::INTERRUPT,
    }
}

/// Calculates total allocated FIFO size in words
fn ep_fifo_size(eps: &[Option<EndpointData>]) -> u16 {
    eps.iter().map(|ep| ep.map(|ep| ep.fifo_size_words).unwrap_or(0)).sum()
}

/// Generates IRQ mask for enabled endpoints
fn ep_irq_mask(eps: &[Option<EndpointData>]) -> u16 {
    eps.iter().enumerate().fold(
        0,
        |mask, (index, ep)| {
            if ep.is_some() {
                mask | (1 << index)
            } else {
                mask
            }
        },
    )
}

/// Calculates MPSIZ value for EP0, which uses special values.
fn ep0_mpsiz(max_packet_size: u16) -> u16 {
    match max_packet_size {
        8 => 0b11,
        16 => 0b10,
        32 => 0b01,
        64 => 0b00,
        other => panic!("Unsupported EP0 size: {}", other),
    }
}

/// Hardware-dependent USB IP configuration.
pub struct OtgInstance<'d, const MAX_EP_COUNT: usize> {
    /// The USB peripheral.
    pub regs: Otg,
    /// The USB state.
    pub state: &'d State<MAX_EP_COUNT>,
    /// FIFO depth in words.
    pub fifo_depth_words: u16,
    /// Number of used endpoints.
    pub endpoint_count: usize,
    /// The PHY type.
    pub phy_type: PhyType,
    /// Extra RX FIFO words needed by some implementations.
    pub extra_rx_fifo_words: u16,
    /// Function to calculate TRDT value based on some internal clock speed.
    pub calculate_trdt_fn: fn(speed: vals::Dspd) -> u8,
}