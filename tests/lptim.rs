#![cfg(target_arch = "arm")]
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
        lptim::{Lptim, WallTimer},
    };

    /// This function is run before each test case.
    #[init]
    fn init() {
        clock::init_clock(true, clock::ClockFreqs::KernelFreq160Mhz);
    }

    #[test]
    fn test_walltimer() {
        let lptim1 = Lptim::new(1);
        let timer = WallTimer::new(lptim1);

        let start = timer.now();
        delay_ms(100);
        let end = timer.now();

        assert!(end > start);
        let diff = end - start;
        // 100ms = 100,000us
        // Allow some tolerance
        assert!(diff >= 99_000 && diff <= 110_000);
    }

    #[test]
    fn test_monotonic() {
        let lptim1 = Lptim::new(1);
        let timer = WallTimer::new(lptim1);

        let mut last = timer.now();
        for _ in 0..10 {
            delay_ms(10);
            let now = timer.now();
            assert!(now >= last);
            last = now;
        }
    }
}
