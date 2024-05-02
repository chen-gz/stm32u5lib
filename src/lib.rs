#![feature(type_alias_impl_trait, const_trait_impl)]
// #![feature(const_trait_impl)]
#![feature(noop_waker)]
#![no_std]
#![allow(dead_code)]
pub mod clock;
pub mod i2c;

pub mod dcmi;
pub mod dma;
pub mod gpio;
// #[path = "drivers/ov5640/ov5640_reg.rs"]
// pub mod ov5640_reg;
pub mod sd_device;

#[cfg(feature = "stm32u5a5")]
pub mod sdmmc;

pub mod exti;
pub mod low_power;
pub mod lptim;
pub mod rtc;
pub mod camera;

// pub mod usb_otg{
    // pub mod usb;
    // mod usb_otg;
// }
pub mod queue;
// pub mod usb_otg;
pub mod usart;
pub mod adc;
pub mod tim;
// mod usb_otg;

// pub use embassy_executor::main as main;
// pub use embassy_executor::task as task;
pub use embassy_executor::Spawner;
// pub use embassy_executor;
// pub use embassy_executor_macros::task;

pub use embassy_executor_macros::task;
pub use embassy_executor;
pub mod com_interface;
#[cfg(feature = "stm32u5a5")]
pub mod pwr;
#[cfg(feature = "stm32u5a5")]
pub mod usb_otg_hs;
pub mod drivers;
pub mod utils;