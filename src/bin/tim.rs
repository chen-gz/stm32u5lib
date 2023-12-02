#![no_std]
#![no_main]
#![allow(unused)]
use cortex_m_rt::entry;
use embassy_stm32::time::Hertz;
// use embassy_stm32::pac::metadata::sdmmc;
// use embassy_stm32::sdmmc;
use embassy_stm32::{bind_interrupts, timer};
use embassy_stm32::{dma::NoDma, gpio::Output, i2c, interrupt, peripherals, rcc, sdmmc, time::khz};

#[path = "../clock.rs"]
mod clock;
#[path = "../gpio.rs"]
mod gpio;

#[path = "../gi2c.rs"]
mod gi2c;

use defmt_rtt as _;
const CAM_I2C_ADDR: u8 = 0x78;
use gpio::gg::GpioPort;
const GREEN: GpioPort = gpio::gg::PC3;
const ORANGE: GpioPort = gpio::gg::PC4;
const BLUE: GpioPort = gpio::gg::PC5;
fn setup() {
    GREEN.setup(
        gpio::gg::Moder::OUTPUT,
        gpio::gg::Ot::PUSHPULL,
        gpio::gg::Pupdr::FLOATING,
    );
    ORANGE.setup(
        gpio::gg::Moder::OUTPUT,
        gpio::gg::Ot::PUSHPULL,
        gpio::gg::Pupdr::FLOATING,
    );
    BLUE.setup(
        gpio::gg::Moder::OUTPUT,
        gpio::gg::Ot::PUSHPULL,
        gpio::gg::Pupdr::FLOATING,
    );
}

use embassy_stm32::rcc::*;
#[entry]
fn main() -> ! {
    clock::gg::init_clock();
    setup();

    defmt::info!("setup led finished!");
    let i2c1 = gi2c::gg::I2C1;
    i2c1.init(400_000, gpio::gg::I2C1_SCL_PB6, gpio::gg::I2C1_SDA_PB7);

    loop {
        // GREEN.toggle();
        ORANGE.toggle();
        // BLUE.toggle();
        clock::gg::delay_ms(500);
    }
}

use core::panic::PanicInfo;
use core::time::Duration;
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
