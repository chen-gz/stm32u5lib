use core::cell::UnsafeCell;
use core::sync::atomic::{AtomicBool, AtomicU16};
use embassy_sync::waitqueue::AtomicWaker;
use stm32_metapac::otg;

#[cfg(stm32u5a5)]
pub mod fifo_const {
    pub const MAX_EP_COUNT: usize = 6;
    pub const FIFO_DEPTH_WORDS: u16 = 1024;
    pub const RX_FIFO_SIZE_EACH: u16 = 128;
    pub const RX_FIFO_SIZE_SIZE_WORD: u16 = 288;
    pub const TX_FIFO_SIZE_WORDS: [u16; MAX_EP_COUNT] = [64, 64, 64, 64, 64, 64];
    //  80, 80, 96];
}
use fifo_const::*;
const EP_OUT_BUFFER_EMPTY: u16 = u16::MAX;

pub struct State<const EP_COUNT: usize> {
    /// Holds received SETUP packets. Available if [State::ep0_setup_ready] is true.
    pub(crate) ep0_setup_data: [u8; 8],
    pub(crate) ep0_setup_ready: AtomicBool,
    pub(crate) ep_in_wakers: [AtomicWaker; EP_COUNT],
    pub(crate) ep_out_wakers: [AtomicWaker; EP_COUNT],
    // pub(crate) ep_out_buffers: [[u8; 1024]; EP_COUNT],
    // pub(crate) ep_out_size: [AtomicU16; EP_COUNT],
    pub(crate) bus_waker: AtomicWaker,
}


static mut STATE: State<MAX_EP_COUNT> = State::new();

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
            // ep_out_buffers: [[0u8; 1024]; EP_COUNT],
            // ep_out_size: [NEW_SIZE; EP_COUNT],
            // ep_out_buffers: [],
            bus_waker: NEW_AW,
        }
    }
}
pub fn state() -> &'static mut State<MAX_EP_COUNT> {
    // static STATE: State<MAX_EP_COUNT> = State::new();
    // disable warning for following line
    #[allow(static_mut_refs)]
    unsafe { &mut STATE }
    // unsafe {addr_of_mut!(STATE)}
}

pub(crate) fn regs() -> otg::Otg {
    #[cfg(otg_fs)]
    return stm32_metapac::USB_OTG_FS;
    #[cfg(otg_hs)]
    return stm32_metapac::USB_OTG_HS;
}
use defmt;
pub fn restore_irqs() {
    defmt::info!("restore_irqs");
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
pub static BUS_WAKER_PWR: AtomicWaker = AtomicWaker::new();
