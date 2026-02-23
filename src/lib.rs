#![cfg_attr(not(test), no_std)]
#![allow(dead_code)]

// Pure modules (available everywhere)
pub mod time_math;
#[macro_use]
mod fmt;

// Hardware modules (available only on ARM)
#[cfg(target_arch = "arm")]
pub mod adc;
#[cfg(target_arch = "arm")]
pub mod clock;
#[cfg(target_arch = "arm")]
pub mod dma;
#[cfg(target_arch = "arm")]
pub mod exti;
#[cfg(target_arch = "arm")]
pub mod gpio;
#[cfg(target_arch = "arm")]
pub mod i2c;
#[cfg(target_arch = "arm")]
pub mod low_power;
#[cfg(target_arch = "arm")]
pub mod lptim;
#[cfg(target_arch = "arm")]
pub mod rtc;
#[cfg(target_arch = "arm")]
pub mod sd_device;
#[cfg(target_arch = "arm")]
pub mod tim;
#[cfg(target_arch = "arm")]
pub mod usart;

#[cfg(target_arch = "arm")]
pub use embassy_executor;
#[cfg(target_arch = "arm")]
pub use embassy_executor::Spawner;
#[cfg(target_arch = "arm")]
pub use embassy_executor_macros::task;

#[cfg(target_arch = "arm")]
pub mod drivers;
#[cfg(target_arch = "arm")]
pub mod hal;
#[cfg(target_arch = "arm")]
pub mod nucleo_u575;
#[cfg(target_arch = "arm")]
pub mod otg;
#[cfg(target_arch = "arm")]
pub mod usb;
#[cfg(target_arch = "arm")]
pub mod utils;

#[cfg(all(sdmmc, dcmi, target_arch = "arm"))]
pub mod camera;
#[cfg(all(dcmi, target_arch = "arm"))]
pub mod dcmi;
#[cfg(all(sdmmc, target_arch = "arm"))]
pub mod sdmmc;
