#![cfg_attr(all(not(test), target_os = "none"), no_std)]
#![allow(dead_code)]
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
    gpio,
    i2c,
    low_power,
    lptim,
    rtc,
    sd_device,
    tim,
    usart,
    drivers,
    nucleo_u575,
    otg,
);

#[cfg(all(target_arch = "arm", target_os = "none"))]
pub use embassy_executor::{self, Spawner};
#[cfg(all(target_arch = "arm", target_os = "none"))]
pub use embassy_executor_macros::task;

pub mod hal;
pub mod shared_i2c;
pub mod utils;

#[cfg(all(target_arch = "arm", target_os = "none", sdmmc, dcmi))]
pub mod camera;
#[cfg(all(target_arch = "arm", target_os = "none", dcmi))]
pub mod dcmi;
#[cfg(all(target_arch = "arm", target_os = "none", sdmmc))]
pub mod sdmmc;
