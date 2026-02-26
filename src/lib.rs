#![cfg_attr(not(test), no_std)]
#![allow(dead_code)]
#[macro_use]
mod fmt;
pub mod adc;
pub mod clock;
pub mod dma;
pub mod exti;
pub mod gpio;
pub mod i2c;
pub mod low_power;
pub mod lptim;
pub mod rtc;
pub mod sd_device;
pub mod tim;
pub mod usart;
pub use embassy_executor;
pub use embassy_executor::Spawner;
pub use embassy_executor_macros::task;
pub mod drivers;
pub mod hal;
pub mod nucleo_u575;
pub mod otg;
pub mod utils;

#[cfg(all(sdmmc, dcmi))]
pub mod camera;
#[cfg(dcmi)]
pub mod dcmi;
#[cfg(sdmmc)]
pub mod sdmmc;
