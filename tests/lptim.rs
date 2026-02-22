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
    }

    #[test]
    fn test_walltimer() {
        clock::init_clock(true, clock::ClockFreqs::KernelFreq160Mhz);

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
}
