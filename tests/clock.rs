#![no_std]
#![no_main]

use cortex_m_rt as _;
#[cfg(feature = "defmt")]
use defmt_rtt as _;

use u5_lib as _;
/// Sets up the logging before entering the test-body, so that embedded-test internal logs (e.g. Running Test <...>)  can also be printed.

#[cfg(test)]
#[embedded_test::tests]
mod tests {

    use u5_lib::{
        clock::{self, delay_ms, delay_s, delay_us},
        debug,
    };
    /// This function is run before each test case.
    #[init]
    fn init() {
        // u5_lib::clock::init_clock(true, clock::ClockFreqs::KernelFreq160Mhz);
    }

    #[test]
    #[timeout(1)]
    async fn test_clock() {
        clock::init_clock(true, u5_lib::clock::ClockFreqs::KernelFreq160Mhz);
        let lserdy = u5_lib::clock::RCC.bdcr().read().lserdy(); // status of lse
        let hsirdy = u5_lib::clock::RCC.cr().read().hsirdy();
        let rtc_status = u5_lib::clock::RCC.bdcr().read().rtcen();
        assert!(lserdy && hsirdy && rtc_status);
    }

    #[test]
    #[timeout(4)]
    fn test_delay_and_rtc_160mhz() {
        test_normal_delay_and_rtc(u5_lib::clock::ClockFreqs::KernelFreq160Mhz);
    }
    #[test]
    #[timeout(4)]
    fn test_delay_and_rtc_80mhz() {
        test_normal_delay_and_rtc(u5_lib::clock::ClockFreqs::KernelFreq80Mhz);
    }
    #[test]
    #[timeout(4)]
    fn test_delay_and_rtc_40mhz() {
        test_normal_delay_and_rtc(u5_lib::clock::ClockFreqs::KernelFreq40Mhz);
    }
    #[test]
    #[timeout(4)]
    fn test_delay_and_rtc_20mhz() {
        test_normal_delay_and_rtc(u5_lib::clock::ClockFreqs::KernelFreq20Mhz);
    }
    #[test]
    #[timeout(4)]
    fn test_delay_and_rtc_16mhz() {
        test_normal_delay_and_rtc(u5_lib::clock::ClockFreqs::KernelFreq16Mhz);
    }
    #[test]
    #[timeout(4)]
    fn test_delay_and_rtc_8mhz() {
        test_normal_delay_and_rtc(u5_lib::clock::ClockFreqs::KernelFreq8Mhz);
    }

    #[test]
    #[timeout(4)]
    fn test_delay_and_rtc_4mhz() {
        test_normal_delay_and_rtc(u5_lib::clock::ClockFreqs::KernelFreq4Mhz);
    }
    #[test]
    #[timeout(4)]
    fn test_delay_and_rtc_2mhz() {
        test_normal_delay_and_rtc(u5_lib::clock::ClockFreqs::KernelFreq2Mhz);
    }
    #[test]
    #[timeout(4)]
    fn test_delay_and_rtc_1mhz() {
        test_normal_delay_and_rtc(u5_lib::clock::ClockFreqs::KernelFreq1Mhz);
    }

    fn test_normal_delay_and_rtc(freq: u5_lib::clock::ClockFreqs) {
        clock::init_clock(true, freq);
        // get rtc time and printout
        u5_lib::rtc::enable_rtc_read();
        // get rtc time and printout
        let (hh0, mm0, ss0, sss0) = u5_lib::rtc::get_time();
        debug!("RTC Time: {:02}:{:02}:{:02}:{:03}", hh0, mm0, ss0, sss0);
        delay_s(3);
        let (hh1, mm1, ss1, sss1) = u5_lib::rtc::get_time();
        debug!("RTC Time after delay: {:02}:{:02}:{:02}:{:03}", hh1, mm1, ss1, sss1);
        assert!(hh0 != 0 || hh1 != 0 || mm0 != 0 || mm1 != 0 || ss0 != 0 || ss1 != 0);
        // it should increment by 3 seconds
        let inc = ((hh1 - hh0) * 60 + mm1 - mm0) * 60 + ss1 - ss0;
        assert_eq!(inc, 3);
    }
}
