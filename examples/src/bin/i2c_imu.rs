#![no_std]
#![no_main]
use cortex_m::delay;
use cortex_m_rt::entry;

use u5_lib::clock;
use u5_lib::gi2c;
use u5_lib::gpio;

use defmt_rtt as _;
const CAM_I2C_ADDR: u16 = 0b110_1000;
use gpio::GpioPort;
const GREEN: GpioPort = gpio::PC3;
const NCS: GpioPort = gpio::PC13;
const ORANGE: GpioPort = gpio::PC4;
const BLUE: GpioPort = gpio::PC5;
const I2C: gi2c::I2cPort = gi2c::I2C3;
fn setup() {
    GREEN.setup();
    ORANGE.setup();
    BLUE.setup();
    NCS.setup();
    NCS.set_high();
    I2C.init(100_000, gpio::I2C3_SCL_PC0, gpio::I2C3_SDA_PB4);

    // CAM_I2C.init(100_000, gpio::I2C3_SCL_PC0, gpio::I2C3_SDA_PB4);
}

#[entry]
fn main() -> ! {
    clock::init_clock();
    setup();

    clock::delay_ms(1000);
    defmt::info!("setup led finished!");
    I2C.write(CAM_I2C_ADDR, &[0x00]).unwrap();
    let mut data = [0u8; 1];
    I2C.read(CAM_I2C_ADDR, &mut data).unwrap();
    defmt::info!("data: {}", data[0]);

    loop {
        ORANGE.toggle();
        clock::delay_ms(500);
    }
}

use core::panic::PanicInfo;
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    defmt::info!("panic");
    defmt::error!(
        "Location file name: {:?}, line: {:?}, col: {:?}",
        _info.location().unwrap().file(),
        _info.location().unwrap().line(),
        _info.location().unwrap().column()
    );
    loop {}
}
