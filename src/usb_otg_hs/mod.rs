/// The 'WORD' in the context of the USB peripheral is a 32-bit word.
use core::{
    cell::UnsafeCell,
    sync::atomic::{AtomicBool, AtomicU16, Ordering},
};
use crate::usb_otg_hs::{
    bus::Bus,
    control_pipe::ControlPipe,
    endpoint::Endpoint,
    phy_type::PhyType,
};

use embassy_sync::waitqueue::AtomicWaker;
use embassy_usb_driver::{Direction, EndpointAddress, EndpointAllocError, EndpointInfo, EndpointType};
use stm32_metapac::{interrupt, otg};

use fifo_const::*;


pub mod mod_new;
mod bus;
mod endpoint;
mod control_pipe;
mod phy_type;
mod descriptor;
mod endpoint_new;

use defmt::{info, trace, error};


#[cfg(stm32u5a5)]
pub mod fifo_const {
    pub const MAX_EP_COUNT: usize = 6;
    pub const FIFO_DEPTH_WORDS: u16 = 1024;
    //4Kbtes = 4096 bytes = 1024 words
    // total fifo size in words
    pub const RX_FIFO_SIZE_EACH: u16 = 128;
    // 128 bytes = 32 words
    pub const RX_FIFO_SIZE_SIZE_WORD: u16 = 288;
    // 32 * 9 = 288
    // 1024 - 288 = 736; 736 / 9 = 81.7777 =320 bytes = 80 words
    // 1024 - 288 - 80 * 8 = 96
    pub const TX_FIFO_SIZE_WORDS: [u16; MAX_EP_COUNT] = [64, 64, 64, 64, 64, 64];
    //  80, 80, 96];
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

pub fn state() -> &'static mut State<MAX_EP_COUNT> {
    // static STATE: State<MAX_EP_COUNT> = State::new();
    unsafe { &mut STATE }
}

pub(crate) fn regs() -> otg::Otg {
    #[cfg(stm32u575)]
    return stm32_metapac::USB_OTG_FS;
    #[cfg(stm32u5a5)]
    return stm32_metapac::USB_OTG_HS;
}

pub fn restore_irqs() {
    info!("restore_irqs");
    regs().gintmsk().write(|w| {
        w.set_usbrst(true);
        w.set_enumdnem(true);
        w.set_usbsuspm(true);
        w.set_wuim(true);
        w.set_iepint(true);
        w.set_oepint(true);
        // w.set_rxflvlm(true);
        w.set_srqim(true);
        w.set_otgint(true);
    });
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

pub static mut BUS_WAKER_PWR: AtomicWaker = AtomicWaker::new();

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

use crate::gpio;
use crate::rtc::setup;
use crate::usb_otg_hs::mod_new::{init_enumeration_done, init_reset,setup_data, setup_return_data};

impl Driver {
    pub fn new(
        config: Config,
        dp: gpio::GpioPort,
        dm: gpio::GpioPort,
    ) -> Self {
        dp.setup();
        dm.setup();
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
        // USART1.send("control_max_packet_size: \t\n".as_bytes());
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


/// Indicates that [State::ep_out_buffers] is empty.

//
// /// USB OTG driver state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct EndpointData {
    pub(crate) ep_type: EndpointType,
    pub(crate) max_packet_size: u16, // this should not exceed fifo size
    // pub(crate) fifo_size_words: u16,
}

