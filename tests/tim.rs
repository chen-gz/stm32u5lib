#![no_std]
#![no_main]

use cortex_m_rt as _;
#[cfg(feature = "defmt")]
use defmt_rtt as _;
use u5_lib as _;
#[embedded_test::tests]
mod tests {

    use u5_lib::{
        clock::{self, delay_ms},
        gpio::TIM3_CH1_PA6,
        tim::Config,
    };

    /// This function is run before each test case.
    #[init]
    fn init() {
        // u5_lib::clock::init_clock(true, clock::ClockFreqs::KernelFreq160Mhz);
    }

    #[test]
    fn test_tim_160mhz() {
        test_tim(clock::ClockFreqs::KernelFreq160Mhz);
    }
    #[test]
    fn test_tim_80mhz() {
        test_tim(clock::ClockFreqs::KernelFreq80Mhz);
    }
    #[test]
    fn test_tim_40mhz() {
        test_tim(clock::ClockFreqs::KernelFreq40Mhz);
    }
    #[test]
    fn test_tim_20mhz() {
        test_tim(clock::ClockFreqs::KernelFreq20Mhz);
    }

    fn test_tim(clock_freq: clock::ClockFreqs) {
        clock::init_clock(true, clock_freq);
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
}
