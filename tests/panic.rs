#![no_std]
#![no_main]

use cortex_m_rt as _;
#[cfg(feature = "defmt")]
use defmt_rtt as _;
use u5_lib as _;
#[embedded_test::tests]
mod tests {

    /// This function is run before each test case.
    #[init]
    fn init() {
        // u5_lib::clock::init_clock(true, clock::ClockFreqs::KernelFreq160Mhz);
    }

    #[test]
    #[should_panic]
    fn test_this_panics() {
        panic!("This is expected");
    }
}
