#![no_std]
#![no_main]

use cortex_m_rt as _;
use defmt_rtt as _;
use u5_lib as _;

#[embedded_test::tests]
mod tests {

    use u5_lib::{
        clock::{self, delay_ms},
        gpio::{TIM1_CH2_PA9, TIM3_CH1_PA6},
        tim::Config,
    };

    /// This function is run before each test case.
    #[init]
    fn init() {
        u5_lib::clock::init_clock(true, clock::ClockFreqs::KernelFreq160Mhz);
    }

    #[test]
    fn test_led() {
        let gpio = TIM3_CH1_PA6;
        gpio.setup();
        let tim = u5_lib::tim::TIM3;
        tim.init(Config::default()).unwrap();
        for _ in 0..50 {
            tim.set_pwm(1, 100, 50);
            tim.enable_output(1);
            delay_ms(1);
        }
    }

    #[test]
    #[should_panic]
    fn test_this_panics() {
        panic!("This is expected");
    }
}
