#![feature(noop_waker)]
#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]
#![feature(impl_trait_in_assoc_type)]

use defmt_rtt as _;
use gpio::GpioPort;
use u5_lib::exti;
use u5_lib::gpio;
use u5_lib::low_power::no_deep_sleep_request;

const GREEN: GpioPort = gpio::PB7;

fn setup() {
    GREEN.setup();
}
use u5_lib::rtc;

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    // clock::init_clock(true, false, clock::ClockFreqs::KernelFreq4Mhz);
    unsafe {
        no_deep_sleep_request();
    }
    setup();
    defmt::info!("setup led finished!");
    spawner.spawn(btn()).unwrap();
    // clock::delay_ms(300);
    loop {
        rtc::rtc_interrupt().await;
    }
}
use embassy_executor::Spawner;

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
