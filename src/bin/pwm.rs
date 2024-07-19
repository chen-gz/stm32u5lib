// #![feature(noop_waker)]
#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use defmt_rtt as _;
use embassy_executor::Spawner;

use u5_lib::{
    gpio::{self, GpioPort, TIM1_CH2_PA9, TIM1_CH3_PA10, TIM3_CH1_PA6},
    low_power::{no_deep_sleep_request, Executor},
    tim::{Config, TIM1, TIM3},
    *,
};
const BLUE: GpioPort = gpio::PB7;

#[task]
async fn async_main(_spawner: Spawner) {
    clock::init_clock(
        false,
        true,
        160_000_000,
        true,
        clock::ClockFreqs::KernelFreq160Mhz,
    );
    TIM1_CH2_PA9.setup();
    TIM1_CH3_PA10.setup();
    TIM3_CH1_PA6.setup();
    let _ = TIM1.init(Config::default());
    let _ = TIM3.init(Config::default());
    TIM1.set_pwm(1, 160, 80);
    TIM1.set_pwm(2, 160, 80);
    TIM1.set_pwm(3, 160, 80);
    TIM3.set_pwm(1, 160, 80);
    // TIM3.set_pwm(2, 160, 80);
    TIM1.enable_output(1);
    TIM1.enable_output(2);
    TIM1.enable_output(3);
    TIM3.enable_output(1);
    TIM3.enable_output(2);
    unsafe {
        no_deep_sleep_request();
    }
    BLUE.setup();

    defmt::info!("setup led finished!");
    loop {
        BLUE.toggle();
        // ORANGE.toggle();
        clock::delay_ms(500);
    }
}

#[cortex_m_rt::entry]
fn main() -> ! {
    Executor::take().run(|spawner| {
        spawner.spawn(async_main(spawner)).unwrap();
    });
}
