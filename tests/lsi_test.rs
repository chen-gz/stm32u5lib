#![no_std]
#![no_main]

use cortex_m_rt as _;
#[cfg(feature = "defmt")]
use defmt_rtt as _;

use u5_lib as _;

#[cfg(test)]
#[embedded_test::tests]
mod tests {

    use u5_lib::{
        clock::{self, delay_s},
        debug,
    };
    use stm32_metapac::rcc::vals::Rtcsel;

    #[init]
    fn init() {
    }

    #[test]
    #[timeout(5)]
    async fn test_lsi_init() {
        // Explicitly initialize with LSI if possible, or assume lse is disabled via cargo features.
        // We can check if 'lse' feature is enabled using cfg!.

        #[cfg(not(feature = "lse"))]
        {
            clock::init_clock(true, u5_lib::clock::ClockFreqs::KernelFreq160Mhz);
            let lsirdy = u5_lib::clock::RCC.bdcr().read().lsirdy();
            let rtc_status = u5_lib::clock::RCC.bdcr().read().rtcen();

            // If init_clock succeeds (doesn't hang), and we are here, we verify LSI is ready.
            assert!(lsirdy);
            assert!(rtc_status);
        }

        #[cfg(feature = "lse")]
        {
            debug!("Skipping LSI test because LSE is enabled");
        }
    }
}
