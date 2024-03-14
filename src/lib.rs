#![feature(noop_waker)]
#![no_std]
#![allow(dead_code)]
pub mod clock;
pub mod gi2c;

pub mod dcmi;
pub mod dma;
pub mod gpio;
pub mod ov5640_reg;
pub mod sd_device;
pub mod sdmmc;

pub mod exti;
pub mod low_power;
pub mod lptim;
pub mod rtc;

pub mod usb_otg{
    pub mod usb;
}
pub mod queue;
// mod usb_otg;