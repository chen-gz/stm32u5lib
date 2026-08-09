#![no_std]
#![no_main]
#![allow(dead_code, unused)]
#[cfg(feature = "defmt")]
use defmt_rtt as _;
use u5_lib as _; // links panic handler

#[embedded_test::tests]
mod tests {
    use u5_lib::gpio::{SPI1_MISO_PA6, SPI1_MOSI_PA7, SPI1_SCK_PA5};
    use u5_lib::hal::{Spi, SpiMode};
    use u5_lib::spi::Spi as SpiDriver;

    #[init]
    fn init() {
        // Initialize clock tree to default
        u5_lib::clock::init_clock(true, u5_lib::clock::ClockFreqs::KernelFreq160Mhz);
    }

    // #[test]
    async fn test_spi_communication() {
        // Initialize SPI1 as Master with PA5 (SCK), PA6 (MISO), and PA7 (MOSI)
        // SPI loopback test requires external wire connecting PA6 (MISO) and PA7 (MOSI)
        let spi = SpiDriver::new(
            1_000_000, // 1 MHz freq (placeholder, using default DIV8 prescaler)
            SpiMode::Mode0,
            SPI1_SCK_PA5,
            SPI1_MISO_PA6,
            SPI1_MOSI_PA7,
        )
        .unwrap();

        // 1. Test Write
        let tx_data = [0xDE, 0xAD, 0xBE, 0xEF];
        #[cfg(feature = "defmt")]
        defmt::info!("SPI: Writing data...");
        spi.write(&tx_data).expect("SPI write failed");

        // 2. Test Write/Read (Full Duplex)
        let mut rx_data = [0u8; 4];
        #[cfg(feature = "defmt")]
        defmt::info!("SPI: Write and Read...");
        // This will successfully send tx_data and read back whatever is on the MISO line concurrently
        spi.write_read(&tx_data, &mut rx_data)
            .expect("SPI write_read failed");

        #[cfg(feature = "defmt")]
        defmt::info!("SPI: Read result: {:x}", rx_data);

        // If loopback is wired (MISO=MOSI), rx_data should equal tx_data
        // assert_eq!(tx_data, rx_data);

        // 3. Test Read (dummy write)
        let mut rx_data_only = [0u8; 4];
        spi.read(&mut rx_data_only).expect("SPI read failed");

        #[cfg(feature = "defmt")]
        defmt::info!("SPI initialization and basic transactions test completed!");
    }
}
