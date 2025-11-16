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
        clock::{self, delay_s},
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
    fn test_normal_delay_and_rtc() {
        clock::init_clock(true, u5_lib::clock::ClockFreqs::KernelFreq160Mhz);
        // get rtc time and printout
        u5_lib::rtc::enable_rtc_read();
        // get rtc time and printout
        let (hh0, mm0, ss0) = u5_lib::rtc::get_time();
        debug!("RTC Time: {:02}:{:02}:{:02}", hh0, mm0, ss0);
        delay_s(3);
        let (hh1, mm1, ss1) = u5_lib::rtc::get_time();
        debug!("RTC Time after delay: {:02}:{:02}:{:02}", hh1, mm1, ss1);
        assert!(hh0 != 0 || hh1 != 0 || mm0 != 0 || mm1 != 0 || ss0 != 0 || ss1 != 0);
    }
}
