#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]
// use cortex_m_rt::entry;

use cortex_m::{peripheral::SCB, asm};
use embassy_executor::Spawner;
use stm32_metapac::PWR;
use u5_lib::{clock, exti::EXTI2_PB2, gpio};

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


use stm32_metapac::pwr::vals::Lpms;
#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    clock::init_clock();
    setup();
    unsafe {
        let p = cortex_m::Peripherals::steal();
        let mut scb = p.SCB;
        scb.set_sleepdeep();
    }
    // standbby mode LPMS = 11x with BREN = 1 in PWR_BDCR
    // PWR.cr1().modify(|v|v.set_lpms(Lpms::));

    defmt::info!("setup led finished!");
    loop {
        GREEN.toggle();
        ORANGE.toggle();
        defmt::info!("wait for raising");
        // clock::delay_ms(500);
        // deep sleep
        // SCB::set_sleepdeep();
        EXTI2_PB2.wait_for_raising().await;
        // pull up for PB2


        // set wakeup pin to PB2
        // wait for falling edge
        // PWR.wucr3()
        //     .modify(|v| v.set_wusel1(stm32_metapac::pwr::vals::Wusel::B_0X1));
        // // enable wakeup pin
        // PWR.wucr1().modify(|v| v.set_wupen(0, true));
        // PWR.cr1().modify(|v|v.set_lpsdsr(true));
        // // enter shutdown mode
        // PWR.bdcr1().modify(|v| v.set_bren(true));
        // PWR.cr1().modify(|v| v.set_lpms(Lpms::_RESERVED_5));
        // asm::wfi();
    }
}

use core::panic::PanicInfo;
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
