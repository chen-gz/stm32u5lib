#![no_std]
#![no_main]
#![allow(unused)]
use cortex_m_rt::entry;
use embassy_stm32::{bind_interrupts, timer};
use embassy_stm32::{dma::NoDma, gpio::Output, i2c, interrupt, peripherals, time::khz};

mod clock;
use defmt_rtt as _; // global logger
bind_interrupts!( struct Irqs {
    I2C1_EV => embassy_stm32::i2c::InterruptHandler<peripherals::I2C1>;
    I2C2_EV => embassy_stm32::i2c::InterruptHandler<peripherals::I2C2>;
    I2C3_EV => embassy_stm32::i2c::InterruptHandler<peripherals::I2C3>;
});
const SLAVE_ADDR: u8 = 0x90;
const MEM1_ADDR: u8 = 0xF8;
const MEM2_ADDR: u8 = 0xF9;
const MEM3_ADDR: u8 = 0xFA;
const MEM4_ADDR: u8 = 0xFB;
use stm32_metapac::Interrupt;

#[interrupt]
fn WWDG() {
    defmt::info!("WWDG");
}

#[entry]
fn main() -> ! {
    let p = embassy_stm32::init(Default::default());
    let mut led1 = Output::new(
        p.PC3,
        embassy_stm32::gpio::Level::Low,
        embassy_stm32::gpio::Speed::VeryHigh,
    );
    let mut led2 = Output::new(
        p.PC4,
        embassy_stm32::gpio::Level::High,
        embassy_stm32::gpio::Speed::VeryHigh,
    );
    let mut led3 = Output::new(
        p.PC5,
        embassy_stm32::gpio::Level::Low,
        embassy_stm32::gpio::Speed::VeryHigh,
    );
    clock::gg::delay_enable(4_000_000);
    loop {
        // led1.toggle();
        led2.toggle();
        clock::gg::delay_ms(500);
    }
}

use core::panic::PanicInfo;
use core::time::Duration;
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
