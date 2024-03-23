#![feature(noop_waker)]
#![no_std]
#![no_main]
use cortex_m_rt::entry;
use u5_lib::clock;
use u5_lib::gpio;
use defmt_rtt as _;
use gpio::GpioPort;
use u5_lib::low_power::mcu_no_deep_sleep;
const GREEN: GpioPort = gpio::PB7;
const ORANGE: GpioPort = gpio::PC4;
const BLUE: GpioPort = gpio::PC5;
fn setup() {
    GREEN.setup();
    ORANGE.setup();
    BLUE.setup();
}

#[entry]
fn main() -> ! {
    clock::init_clock(true, false, clock::ClockFreqs::KernelFreq160Mhz);
    // clock::kernel_freq_160mhz_request();
    mcu_no_deep_sleep();
    setup();

    defmt::info!("setup led finished!");
    loop {
        GREEN.toggle();
        // ORANGE.toggle();
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
