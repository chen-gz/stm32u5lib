#![feature(noop_waker)]
#![no_std]
#![no_main]
use cortex_m_rt::entry;
use u5_lib::clock;
use u5_lib::gpio;
use defmt_rtt as _;
use gpio::GpioPort;
use u5_lib::gpio::USART_RX_PA10;
use u5_lib::gpio::USART_TX_PA9;
use u5_lib::low_power::mcu_no_deep_sleep;
use u5_lib::usart::USART1;
const GREEN: GpioPort = gpio::PB7;
const ORANGE: GpioPort = gpio::PC4;
const BLUE: GpioPort = gpio::PC5;
fn setup() {
}

#[entry]
fn main() -> ! {
    clock::init_clock(true, false, clock::ClockFreqs::KernelFreq160Mhz);
    mcu_no_deep_sleep();
    setup();
    USART1.setup(USART_TX_PA9, USART_RX_PA10);


    defmt::info!("setup led finished!");
    loop {
        USART1.send("Hello, world!".as_bytes());
        // GREEN.toggle();
        // ORANGE.toggle();
        clock::delay_ms(500);
    }
}

use core::panic::PanicInfo;
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
