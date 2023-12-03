#![no_std]
#![no_main]
use cortex_m_rt::entry;

// #[path = "../clock.rs"]
// mod clock;
// #[path = "../gpio.rs"]
// mod gpio;
use u5_lib::clock;
use u5_lib::gpio;

use defmt_rtt as _;
use gpio::GpioPort;
const GREEN: GpioPort = gpio::PC3;
const ORANGE: GpioPort = gpio::PC4;
const BLUE: GpioPort = gpio::PC5;
fn setup() {
    GREEN.setup();
    ORANGE.setup();
    BLUE.setup();
}

#[entry]
fn main() -> ! {
    clock::init_clock();
    setup();

    defmt::info!("setup led finished!");
    loop {
        GREEN.toggle();
        ORANGE.toggle();
        clock::delay_ms(500);
    }
}

use core::panic::PanicInfo;
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
