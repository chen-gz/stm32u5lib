#![no_std]
#![no_main]

#[cfg(feature = "defmt")]
use defmt_rtt as _;
use u5_lib as _; // links panic handler

#[embedded_test::tests]
mod tests {
    use u5_lib::otg::{Driver, Config};
    use u5_lib::gpio::{USB_DM_PA11, USB_DP_PA12};
    use embassy_usb::Builder;

    #[init]
    fn init() {
        u5_lib::clock::init_clock(true, u5_lib::clock::ClockFreqs::KernelFreq160Mhz);
    }

    struct MyHandler;

    impl embassy_usb::Handler for MyHandler {
        fn enabled(&mut self, enabled: bool) {
            #[cfg(feature = "defmt")]
            defmt::info!("USB enabled: {}", enabled);
        }
        fn reset(&mut self) {
            #[cfg(feature = "defmt")]
            defmt::info!("USB reset");
        }
        fn addressed(&mut self, addr: u8) {
            #[cfg(feature = "defmt")]
            defmt::info!("USB addressed: {}", addr);
        }
        fn configured(&mut self, configured: bool) {
            #[cfg(feature = "defmt")]
            defmt::info!("USB configured: {}", configured);
        }
        fn suspended(&mut self, suspended: bool) {
            #[cfg(feature = "defmt")]
            if suspended {
                defmt::info!("USB suspended");
            } else {
                defmt::info!("USB resumed");
            }
        }
    }

    #[test]
    async fn test_usb_enumeration() {
        #[cfg(feature = "defmt")]
        defmt::info!("Initializing USB...");

        // Buffer for endpoints
        static mut EP_OUT_BUFFER: [u8; 1024] = [0; 1024];
        let ep_out_buffer = unsafe { &mut EP_OUT_BUFFER };

        // Config
        let config = Config::default();
        let driver = Driver::new_fs(USB_DP_PA12, USB_DM_PA11, ep_out_buffer, config);

        // Embassy USB Config
        let mut usb_config = embassy_usb::Config::new(0x1234, 0x5678);
        usb_config.manufacturer = Some("Test Manufacturer");
        usb_config.product = Some("Test Product");
        usb_config.serial_number = Some("12345678");
        usb_config.device_class = 0xEF;
        usb_config.device_sub_class = 0x02;
        usb_config.device_protocol = 0x01;
        usb_config.composite_with_iads = true;

        // Buffers for control endpoint
        static mut CONFIG_DESC: [u8; 256] = [0; 256];
        static mut BOS_DESC: [u8; 256] = [0; 256];
        static mut MSOS_DESC: [u8; 256] = [0; 256];
        static mut CONTROL_BUF: [u8; 64] = [0; 64];

        let config_desc = unsafe { &mut CONFIG_DESC };
        let bos_desc = unsafe { &mut BOS_DESC };
        let msos_desc = unsafe { &mut MSOS_DESC };
        let control_buf = unsafe { &mut CONTROL_BUF };

        let mut handler = MyHandler;

        let mut builder = Builder::new(
            driver,
            usb_config,
            config_desc,
            bos_desc,
            msos_desc,
            control_buf,
        );

        builder.handler(&mut handler);

        let mut usb = builder.build();

        #[cfg(feature = "defmt")]
        defmt::info!("Running USB device. Connect to host to enumerate.");

        // Run USB device
        // Since we want to test enumeration, we just run it.
        // We also want to verify it runs, so we can't just block forever if we want the test to 'pass' in CI.
        // But for hardware verification, loop is fine.
        // I will use `select` to run it for a duration OR until a signal?
        // Since "I will run the test in hardware", I assume interactive use.
        // But `embedded-test` has a timeout.
        // I'll make it loop. The user will see logs.

        usb.run().await;
    }
}
