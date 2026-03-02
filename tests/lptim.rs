#![no_std]
#![no_main]

use cortex_m_rt as _;
#[cfg(feature = "defmt")]
use defmt_rtt as _;
use u5_lib as _;
#[embedded_test::tests]
mod tests {

    use core::time::Duration;
    use u5_lib::{
        clock::{self, delay_ms},
        lptim::{timeout, Lptim, TimeoutError, WallTimer},
    };

    /// This function is run before each test case.
    #[init]
    fn init() {}

    #[test]
    async fn test_timeout() {
        clock::init_clock(true, clock::ClockFreqs::KernelFreq160Mhz);
        let lptim2 = Lptim::new(2);

        // Test timeout (future never completes)
        let result = timeout(
            &lptim2,
            Duration::from_millis(10),
            core::future::poll_fn(|_| core::task::Poll::<()>::Pending),
        )
        .await;
        assert_eq!(result, Err(TimeoutError));

        // Test success (future completes immediately)
        let result = timeout(&lptim2, Duration::from_millis(100), async { 42 }).await;
        assert_eq!(result, Ok(42));
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
