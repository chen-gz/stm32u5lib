#![feature(noop_waker)]
#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]


use u5_lib::{clock, exti};

use u5_lib::gpio;

use defmt_rtt as _;
use gpio::GpioPort;

const GREEN: GpioPort = gpio::PB7;

fn setup() {
    GREEN.setup();
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    clock::init_clock();
    setup();
    defmt::info!("setup led finished!");
    spawner.spawn(btn()).unwrap();

    // loop {
    // GREEN.toggle();
    // clock::delay_ms(1000);
    // }
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
        exti::EXTI13_PC13.wait_for_raising().await;
        GREEN.toggle();
    }
}
