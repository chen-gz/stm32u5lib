#![feature(noop_waker)]
#![no_std]
#![no_main]
use cortex_m_rt::entry;
use defmt_rtt as _;
use u5_lib::clock;
fn setup() {}

#[entry]
fn main() -> ! {
    // clock::init_clock(true, false, clock::ClockFreqs::KernelFreq160Mhz);
    setup();
    // let mut uart = u5_lib::usart::UsartPort::new(default::Default::default()).unwrap();

    defmt::info!("setup led finished!");
    loop {
        // uart.send("Hello, world!".as_bytes()).unwrap();
        // USART1.send("Hello, world!".as_bytes());
        // GREEN.toggle();
        // ORANGE.toggle();
        clock::delay_ms(500);
    }
}
