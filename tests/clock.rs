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
        u5_lib::clock::reset_backup_domain();
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
    #[timeout(2)]
    fn test_delay_and_rtc_160mhz() {
        test_normal_delay_and_rtc(u5_lib::clock::ClockFreqs::KernelFreq160Mhz);
    }
    #[test]
    #[timeout(2)]
    fn test_delay_and_rtc_80mhz() {
        test_normal_delay_and_rtc(u5_lib::clock::ClockFreqs::KernelFreq80Mhz);
    }
    #[test]
    #[timeout(2)]
    fn test_delay_and_rtc_40mhz() {
        test_normal_delay_and_rtc(u5_lib::clock::ClockFreqs::KernelFreq40Mhz);
    }
    #[test]
    #[timeout(2)]
    fn test_delay_and_rtc_20mhz() {
        test_normal_delay_and_rtc(u5_lib::clock::ClockFreqs::KernelFreq20Mhz);
    }
    #[test]
    #[timeout(2)]
    fn test_delay_and_rtc_16mhz() {
        test_normal_delay_and_rtc(u5_lib::clock::ClockFreqs::KernelFreq16Mhz);
    }
    #[test]
    #[timeout(2)]
    fn test_delay_and_rtc_8mhz() {
        test_normal_delay_and_rtc(u5_lib::clock::ClockFreqs::KernelFreq8Mhz);
    }

    #[test]
    #[timeout(2)]
    fn test_delay_and_rtc_4mhz() {
        test_normal_delay_and_rtc(u5_lib::clock::ClockFreqs::KernelFreq4Mhz);
    }
    #[test]
    #[timeout(2)]
    fn test_delay_and_rtc_2mhz() {
        test_normal_delay_and_rtc(u5_lib::clock::ClockFreqs::KernelFreq2Mhz);
    }
    #[test]
    #[timeout(2)]
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
        delay_s(1);
        let (hh1, mm1, ss1, sss1) = u5_lib::rtc::get_time();
        debug!("RTC Time after delay: {:02}:{:02}:{:02}:{:03}", hh1, mm1, ss1, sss1);
        assert!(hh0 != 0 || hh1 != 0 || mm0 != 0 || mm1 != 0 || ss0 != 0 || ss1 != 0);
        // it should increment by 1 seconds
        let t0_ms = (hh0 as u64 * 3600 + mm0 as u64 * 60 + ss0 as u64) * 1000 + sss0 as u64;
        let t1_ms = (hh1 as u64 * 3600 + mm1 as u64 * 60 + ss1 as u64) * 1000 + sss1 as u64;

        let diff = if t1_ms >= t0_ms {
            t1_ms - t0_ms
        } else {
            // Handle rollover if needed, or panic if unexpected
            // Assuming 24h rollover
            (t1_ms + 24 * 3600 * 1000) - t0_ms
        };
        debug!("Time diff: {} ms", diff);

        let target = 1000;
        let tolerance = 100;
        assert!(
            diff >= target - tolerance && diff <= target + tolerance,
            "Time difference {} ms is out of range [{}, {}]",
            diff,
            target - tolerance,
            target + tolerance
        );
    }
}
