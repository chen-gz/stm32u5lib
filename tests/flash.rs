#![no_std]
#![no_main]

use cortex_m_rt as _;
#[cfg(feature = "defmt")]
use defmt_rtt as _;
use u5_lib as _;

#[embedded_test::tests]
mod tests {
    use u5_lib::clock;
    use u5_lib::flash;

    #[init]
    fn init() {
        clock::reset_backup_domain();
        clock::init_clock(true, clock::ClockFreqs::KernelFreq160Mhz);
    }

    #[test]
    fn test_flash_layout() {
        let safe_start = flash::get_flash_safe_start();
        let flash_size = flash::get_flash_size();
        let flash_end = flash::get_flash_end();

        assert!(safe_start > flash::FLASH_START);
        assert!(flash_size >= 1024 * 1024); // at least 1MB
        assert_eq!(flash_end, flash::FLASH_START + flash_size);
        assert!(safe_start < flash_end);
    }

    #[test]
    fn test_flash_write_read_erase() {
        let flash_end = flash::get_flash_end();
        // Use the very last page of flash for testing to avoid code region
        let test_page_addr = flash_end - flash::PAGE_SIZE;
        assert!(test_page_addr >= flash::get_flash_safe_start());

        // 1. Erase the page
        assert!(flash::erase_page(test_page_addr).is_ok());

        // 2. Read to verify it is erased (all 0xFF)
        let mut read_buf = [0u8; 16];
        assert!(flash::read(test_page_addr, &mut read_buf).is_ok());
        for &val in &read_buf {
            assert_eq!(val, 0xFF);
        }

        // 3. Write a test pattern (16 bytes)
        let pattern: [u8; 16] = [
            0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD,
            0xEE, 0xFF,
        ];
        assert!(flash::write(test_page_addr, &pattern).is_ok());

        // 4. Read back and verify the written pattern
        let mut read_back = [0u8; 16];
        assert!(flash::read(test_page_addr, &mut read_back).is_ok());
        assert_eq!(read_back, pattern);

        // 5. Erase again
        assert!(flash::erase_page(test_page_addr).is_ok());

        // 6. Verify it is erased (all 0xFF)
        assert!(flash::read(test_page_addr, &mut read_back).is_ok());
        for &val in &read_back {
            assert_eq!(val, 0xFF);
        }
    }
}
