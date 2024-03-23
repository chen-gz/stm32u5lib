#![feature(noop_waker)]
#![no_std]
#![no_main]
use cortex_m_rt::entry;
use u5_lib::clock;
use u5_lib::com_interface::ComInterface;
use u5_lib::gpio;
use defmt_rtt as _;
use gpio::GpioPort;
const GREEN: GpioPort = gpio::PB7;
const ORANGE: GpioPort = gpio::PC4;
const BLUE: GpioPort = gpio::PC5;
fn setup() {
}

#[entry]
fn main() -> ! {
    clock::init_clock(true, false, clock::ClockFreqs::KernelFreq160Mhz);
    setup();
    let mut uart = u5_lib::usart::UsartPort::new(default::Default::default()).unwrap();


    defmt::info!("setup led finished!");
    loop {
        uart.send("Hello, world!".as_bytes());
        // USART1.send("Hello, world!".as_bytes());
        // GREEN.toggle();
        // ORANGE.toggle();
        clock::delay_ms(500);
    }
}

use core::default;
use core::panic::PanicInfo;
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
