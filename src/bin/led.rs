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
        0,
    );
    ORANGE.setup(
        gpio::gg::Moder::OUTPUT,
        gpio::gg::Ot::PUSHPULL,
        gpio::gg::Pupdr::FLOATING,
        0,
    );
    BLUE.setup(
        gpio::gg::Moder::OUTPUT,
        gpio::gg::Ot::PUSHPULL,
        gpio::gg::Pupdr::FLOATING,
        0,
    );
}

use embassy_stm32::rcc::*;
#[entry]
fn main() -> ! {
    let mut config = embassy_stm32::Config::default();
    config.rcc.mux = ClockSrc::PLL1_R(PllConfig::hsi_160mhz());
    config.rcc.voltage_range = VoltageScale::RANGE1; // this is for high frquency. This should be
                                                     // should better set in the rcc module. instead of here.

    let p = embassy_stm32::init(config);
    clock::gg::delay_enable(160_000_000);
    setup();

    defmt::info!("setup led finished!");
    loop {
        GREEN.toggle();
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
