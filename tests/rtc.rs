#![cfg(target_arch = "arm")]
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
        u5_lib::clock::reset_backup_domain();
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
        info!("Time 1: {}:{}:{}.{:03}", t1.0, t1.1, t1.2, t1.3);

        u5_lib::nucleo_u575::LED_BLUE.set_high();
        // rtc_delay has a minimum duration requirement of > 1s, so we use 2s here.
        rtc::rtc_delay(core::time::Duration::from_secs(2)).await;
        u5_lib::nucleo_u575::LED_BLUE.toggle();

        let t2 = rtc::get_time();
        info!("Time 2: {}:{}:{}.{:03}", t2.0, t2.1, t2.2, t2.3);

        let t0_ms = (t1.0 as u64 * 3600 + t1.1 as u64 * 60 + t1.2 as u64) * 1000 + t1.3 as u64;
        let t1_ms = (t2.0 as u64 * 3600 + t2.1 as u64 * 60 + t2.2 as u64) * 1000 + t2.3 as u64;

        let diff = if t1_ms >= t0_ms {
            t1_ms - t0_ms
        } else {
            // Handle rollover if needed, or panic if unexpected
            // Assuming 24h rollover
            (t1_ms + 24 * 3600 * 1000) - t0_ms
        };
        info!("Time diff: {} ms", diff);

        let target = 1500;
        // The RTC delay only guarantees a wake-up at the next second boundary + duration - 1.
        // So for a 2s delay, it waits between 1s and 2s depending on alignment.
        // We allow 100ms tolerance on top of the 1s jitter.
        let tolerance = 600;
        assert!(
            diff >= target - tolerance && diff <= target + tolerance,
            "Time difference {} ms is out of range [{}, {}]",
            diff,
            target - tolerance,
            target + tolerance
        );
    }
}
