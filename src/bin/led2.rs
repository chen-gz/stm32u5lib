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
use defmt_rtt as _; // global logger
bind_interrupts!( struct Irqs {
    I2C1_EV => embassy_stm32::i2c::InterruptHandler<peripherals::I2C1>;
    I2C2_EV => embassy_stm32::i2c::InterruptHandler<peripherals::I2C2>;
    I2C3_EV => embassy_stm32::i2c::InterruptHandler<peripherals::I2C3>;
    SDMMC2 => embassy_stm32::sdmmc::InterruptHandler<peripherals::SDMMC2>;
});
// PB4, PC0 --> I2C3
const SLAVE_ADDR: u8 = 0x90;
const MEM1_ADDR: u8 = 0xF8;
const MEM2_ADDR: u8 = 0xF9;
const MEM3_ADDR: u8 = 0xFA;
const MEM4_ADDR: u8 = 0xFB;
const CAM_I2C_ADDR: u8 = 0x78;

#[entry]
fn main() -> ! {
    let p = embassy_stm32::init(Default::default());
    clock::gg::delay_enable(4_000_000);
    let mut green = Output::new(
        p.PC3,
        embassy_stm32::gpio::Level::Low,
        embassy_stm32::gpio::Speed::VeryHigh,
    );
    let mut orange = Output::new(
        p.PC4,
        embassy_stm32::gpio::Level::Low,
        embassy_stm32::gpio::Speed::VeryHigh,
    );
    let mut blue = Output::new(
        p.PC5,
        embassy_stm32::gpio::Level::Low,
        embassy_stm32::gpio::Speed::VeryHigh,
    );
    defmt::info!("setup led finished!");
    let mut i2c_config = i2c::Config::default();
    i2c_config.sda_pullup = true;
    i2c_config.scl_pullup = true;
    let mut cam_led = embassy_stm32::i2c::I2c::new(
        p.I2C3,
        p.PC0,
        p.PB4,
        Irqs,
        NoDma,
        NoDma,
        khz(100),
        i2c_config,
    );
    // try to read the camera registers
    let mut data: [u8; 2] = [0; 2]; // ov5640 address have 2 bytes
    data = [0x30, 0x0A]; // chip ID High Byte
                         // cam_led.read(CAM_I2C_ADDR, &mut data).unwrap();

    defmt::info!("read data: {:x}", data);
    // send register address first
    cam_led.blocking_write(CAM_I2C_ADDR, &data);

    let mut ret_data: [u8; 2] = [0; 2];
    cam_led.blocking_read(CAM_I2C_ADDR, &mut ret_data);
    defmt::info!("read data: {:x}", ret_data);
    // try to read the sdmmc registers
    //
    // get kernel frequency
    // defmt::info!("kernel frequency: {:?});", rcc::);
    let sd = sdmmc::Sdmmc::new_1bit(p.SDMMC2, Irqs, p.PC1, p.PA0, p.PB14, Default::default());
    // sd.init_card(Hertz::kHz(400)).unwrap();
    // sd.init_card(Hertz::khz(400)).await.unwrap();

    // pub fn new_1bit(
    //     sdmmc: impl Peripheral<P = T> + 'd,
    //     _irq: impl interrupt::typelevel::Binding<T::Interrupt, InterruptHandler<T>> + 'd,
    //     clk: impl Peripheral<P = impl CkPin<T>> + 'd,
    //     cmd: impl Peripheral<P = impl CmdPin<T>> + 'd,
    //     d0: impl Peripheral<P = impl D0Pin<T>> + 'd,
    //     config: Config,
    // ) -> Self {

    loop {
        // led1.toggle();
        // led2.toggle();
        blue.toggle();
        clock::gg::delay_ms(500);
    }
}

use core::panic::PanicInfo;
use core::time::Duration;
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
