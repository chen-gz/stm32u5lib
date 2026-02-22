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

    use u5_lib::{clock, info, rtc};
    /// This function is run before each test case.
    #[init]
    fn init() {
        // u5_lib::clock::init_clock(true, clock::ClockFreqs::KernelFreq160Mhz);
    }

    #[test]
    #[timeout(5)]
    async fn test_rtc() {
        info!("Clock initialized");
        clock::init_clock(true, u5_lib::clock::ClockFreqs::KernelFreq160Mhz);
        info!("RTC initialized");
        u5_lib::nucleo_u575::LED_BLUE.setup();
        u5_lib::rtc::enable_rtc_read();

        let t1 = rtc::get_time();
        info!("Time 1: {}:{}:{}.{}", t1.0, t1.1, t1.2, t1.3);

        u5_lib::nucleo_u575::LED_BLUE.set_high();
        rtc::rtc_delay(core::time::Duration::from_secs(2)).await;
        u5_lib::nucleo_u575::LED_BLUE.toggle();

        let t2 = rtc::get_time();
        info!("Time 2: {}:{}:{}.{}", t2.0, t2.1, t2.2, t2.3);
    }
}
