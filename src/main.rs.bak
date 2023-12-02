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

#[entry]
fn main() -> ! {
    let p = embassy_stm32::init(Default::default());
    let mut led1 = Output::new(
        p.PC3,
        embassy_stm32::gpio::Level::High,
        embassy_stm32::gpio::Speed::VeryHigh,
    );
    let mut led2 = Output::new(
        p.PC4,
        embassy_stm32::gpio::Level::High,
        embassy_stm32::gpio::Speed::VeryHigh,
    );
    let mut led3 = Output::new(
        p.PC5,
        embassy_stm32::gpio::Level::High,
        embassy_stm32::gpio::Speed::VeryHigh,
    );
    clock::gg::delay_enable(4_000_000);
    loop {
        led1.toggle();
        led2.toggle();
        led3.toggle();
        // led.set_high();
        clock::gg::delay_ms(500);
        // led.set_low();
        //clock::gg::delay_ms(500);
    }
    // clock::gg::delay_enable(4_000_000);
    // let mut i2c_config = i2c::Config::default();
    // i2c_config.sda_pullup = true;
    // i2c_config.scl_pullup = true;
    // // setup i2c time base on 400khz
    // // 100khz is 10us per bit
    // // SCL high time is 5us and low time is 5us
    // // the i2c clock is 4Mhz
    // // t_{SCLH} =
    // let mut i2c_plus = embassy_stm32::i2c::I2c::new(
    //     p.I2C1,
    //     p.PB6,
    //     p.PB3,
    //     Irqs,
    //     NoDma,
    //     NoDma,
    //     khz(100),
    //     i2c_config, // Default::default(),
    // );

    // let mut i2c_minus = embassy_stm32::i2c::I2c::new(
    //     p.I2C2,
    //     p.PB13,
    //     p.PB14,
    //     Irqs,
    //     NoDma,
    //     NoDma,
    //     khz(100),
    //     i2c_config, // Default::default(),
    // );

    // let mut s0 = Output::new(
    //     p.PA8,
    //     embassy_stm32::gpio::Level::High,
    //     embassy_stm32::gpio::Speed::VeryHigh,
    // );
    // let mut s1 = Output::new(
    //     p.PA9,
    //     embassy_stm32::gpio::Level::High,
    //     embassy_stm32::gpio::Speed::VeryHigh,
    // );
    // let mut s2 = Output::new(
    //     p.PA10,
    //     embassy_stm32::gpio::Level::High,
    //     embassy_stm32::gpio::Speed::VeryHigh,
    // );
    // let mut s3 = Output::new(
    //     p.PA11,
    //     embassy_stm32::gpio::Level::High,
    //     embassy_stm32::gpio::Speed::VeryHigh,
    // );

    // let mut add0 = Output::new(
    //     p.PB4,
    //     embassy_stm32::gpio::Level::High,
    //     embassy_stm32::gpio::Speed::VeryHigh,
    // );
    // let mut add1 = Output::new(
    //     p.PB5,
    //     embassy_stm32::gpio::Level::High,
    //     embassy_stm32::gpio::Speed::VeryHigh,
    // );
    // s0.set_low();
    // s1.set_low();
    // s2.set_low();
    // s3.set_low();
    // add0.set_low();
    // add1.set_low();
    // let mut data: [u8; 2] = [MEM1_ADDR, 0xA0];
    // i2c_plus.blocking_write(SLAVE_ADDR, data[0..2].as_ref());
    // data[0] = MEM2_ADDR;
    // i2c_plus.blocking_write(SLAVE_ADDR, data[0..2].as_ref());
    // data[0] = MEM3_ADDR;
    // i2c_plus.blocking_write(SLAVE_ADDR, data[0..2].as_ref());
    // data[0] = MEM4_ADDR;
    // i2c_plus.blocking_write(SLAVE_ADDR, data[0..2].as_ref());
    // defmt::info!("send data={=[?]}", data[0..2].as_ref());
    // // defmt::info!(data[0..1]);

    // // set defmt logger level
    // loop {
    //     s0.set_high();
    //     s1.set_high();
    //     s2.set_high();
    //     s3.set_high();
    //     clock::gg::delay_ms(4);
    //     s0.set_low();
    //     s1.set_low();
    //     s2.set_low();
    //     s3.set_low();
    //     clock::gg::delay_ms(4);
    // }
}

use core::panic::PanicInfo;
use core::time::Duration;
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
