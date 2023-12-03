#![no_std]
#![no_main]
use cortex_m_rt::entry;

#[path = "../clock.rs"]
mod clock;
#[path = "../gi2c.rs"]
mod gi2c;
#[path = "../gpio.rs"]
mod gpio;

use defmt_rtt as _;
const CAM_I2C_ADDR: u16 = 0x78;
use gpio::GpioPort;
const GREEN: GpioPort = gpio::PC3;
const ORANGE: GpioPort = gpio::PC4;
const BLUE: GpioPort = gpio::PC5;
const I2C: gi2c::I2cPort = gi2c::I2C1;
fn setup() {
    GREEN.setup();
    ORANGE.setup();
    BLUE.setup();
    I2C.init(400_000, gpio::I2C1_SCL_PB6, gpio::I2C1_SDA_PB7);
}

#[entry]
fn main() -> ! {
    clock::init_clock();
    setup();

    defmt::info!("setup led finished!");
    I2C.write(CAM_I2C_ADDR, &[0x0]).unwrap();
    let mut data = [0u8; 2];
    I2C.read(CAM_I2C_ADDR, &mut data).unwrap();

    loop {
        ORANGE.toggle();
        clock::delay_ms(500);
    }
}

use core::panic::PanicInfo;
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
