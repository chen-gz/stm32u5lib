/// The 'WORD' in the context of the USB peripheral is a 32-bit word.
use core::{
    cell::UnsafeCell,
    sync::atomic::{AtomicBool, AtomicU16, Ordering},
};
use crate::usb_otg_hs::{
    // bus::Bus,
    // control_pipe::ControlPipe,
    // endpoint::Endpoint,
    phy_type::PhyType,
};
use embassy_sync::waitqueue::AtomicWaker;
use embassy_usb_driver::{Direction, EndpointAddress, EndpointAllocError, EndpointInfo, EndpointType};
use stm32_metapac::{interrupt, otg};



pub mod mod_new;
// mod bus;
// mod endpoint;
// mod control_pipe;
mod phy_type;
mod descriptor;
mod endpoint_new;
pub mod global_states;

use defmt::{info, trace, error};

