#![no_std]
#![no_main]

use cortex_m_rt as _;
#[cfg(feature = "defmt")]
use defmt_rtt as _;

use u5_lib as _;

#[cfg(test)]
#[embedded_test::tests]
mod tests {
    use u5_lib::{clock, profile, profile_async};

    #[init]
    fn init() {
        clock::init_clock(true, clock::ClockFreqs::KernelFreq160Mhz);
    }

    #[test]
    fn test_profile() {
        let res = profile!("delay_10ms", {
            clock::delay_ms(10);
            42
        });
        assert_eq!(res, 42);
    }

    #[test]
    async fn test_profile_async() {
        let res = profile_async!("async_delay_10ms", async {
            clock::delay_ms(10);
            43
        }).await;
        assert_eq!(res, 43);
    }

    #[test]
    fn test_profile_no_name() {
        let _res = profile!(clock::delay_ms(5));
        // res is () here
    }
}
