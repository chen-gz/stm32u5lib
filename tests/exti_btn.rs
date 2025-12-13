#![no_std]
#![no_main]

use cortex_m_rt as _;
#[cfg(feature = "defmt")]
use defmt_rtt as _;
use u5_lib as _;
#[embedded_test::tests]
mod tests {
    use u5_lib::clock::delay_ms;

    /// This function is run before each test case.
    #[init]
    fn init() {
        // u5_lib::clock::init_clock(true, clock::ClockFreqs::KernelFreq160Mhz);
    }

    #[test]
    async fn test_exti_btn() {
        u5_lib::clock::init_clock(true, u5_lib::clock::ClockFreqs::KernelFreq160Mhz);
        #[cfg(feature = "interactive-test")]{
            u5_lib::info!("waiting for btn");
            delay_ms(200);
            u5_lib::exti::EXTI13_PC13_PD.wait_for_raising().await;
        }

        #[cfg(feature = "interactive-test")]{
            u5_lib::info!("skip btn test");
        }
    }
}
