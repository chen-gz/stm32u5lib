#![feature(noop_waker)]
#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use u5_lib::low_power::mcu_no_deep_sleep;
use u5_lib::{clock, exti};
use defmt_rtt as _;
use gpio::GpioPort;
use u5_lib::gpio;

const GREEN: GpioPort = gpio::PB7;

fn setup() {
    GREEN.setup();
}
use u5_lib::rtc;

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    clock::init_clock(true, false, clock::ClockFreqs::KernelFreq4Mhz);
    mcu_no_deep_sleep();
    setup();
    defmt::info!("setup led finished!");
    spawner.spawn(btn()).unwrap();
    // clock::delay_ms(300);
    loop {
        rtc::rtc_interrupt().await;
    }
}
use core::panic::PanicInfo;
use embassy_executor::Spawner;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[embassy_executor::task]
async fn btn() {
    let _last_time: (u8, u8, u8) = (0, 0, 0);
    defmt::info!("waiting for btn");
    loop {
        defmt::info!("button clicked");
        exti::EXTI13_PC13.wait_for_raising().await;
        GREEN.toggle();
    }
}
