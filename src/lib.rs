#![cfg_attr(all(not(test), target_os = "none"), no_std)]
#![allow(dead_code)]
#![allow(
    clippy::wrong_self_convention,
    clippy::needless_range_loop,
    clippy::needless_late_init,
    clippy::declare_interior_mutable_const,
    clippy::too_many_arguments,
    clippy::type_complexity,
    clippy::manual_flatten,
    clippy::non_minimal_cfg,
    clippy::zero_prefixed_literal,
    clippy::doc_lazy_continuation
)]
#[macro_use]
mod fmt;

macro_rules! mcu_modules {
    ($($mod:ident),* $(,)?) => {
        $(
            #[cfg(all(target_arch = "arm", target_os = "none"))]
            pub mod $mod;
        )*
    };
}

mcu_modules!(
    adc,
    clock,
    dma,
    exti,
    flash,
    gpio,
    i2c,
    low_power,
    lptim,
    rtc,
    sd_device,
    tim,
    usart,
    nucleo_u575,
    otg,
);

#[cfg(all(target_arch = "arm", target_os = "none"))]
pub use embassy_executor::{self, Spawner};
#[cfg(all(target_arch = "arm", target_os = "none"))]
pub use embassy_executor_macros::task;

pub mod drivers;
pub mod hal;
pub mod shared_i2c;
pub mod utils;

#[cfg(all(target_arch = "arm", target_os = "none", dcmi))]
pub mod dcmi;
#[cfg(all(target_arch = "arm", target_os = "none", sdmmc))]
pub mod sdmmc;
