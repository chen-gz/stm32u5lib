#![cfg_attr(not(test), no_std)]
#![allow(dead_code)]
pub mod clock;
#[cfg(dcmi)]
pub mod dcmi;
pub mod dma;
pub mod gpio;
pub mod i2c;
pub mod sd_device;

#[cfg(sdmmc)]
pub mod sdmmc;

pub mod exti;
pub mod low_power;
pub mod lptim;
pub mod rtc;

// only when sdmmc and dcmi and ov5640 are enabled
#[cfg(all(sdmmc, dcmi))]
pub mod camera;

pub mod adc;
pub mod tim;
pub mod usart;

pub use embassy_executor::Spawner;

pub use embassy_executor;
pub use embassy_executor_macros::task;
pub mod com_interface;

pub mod drivers;
pub mod utils;

pub mod hal;
pub mod usb;
pub mod otg;
