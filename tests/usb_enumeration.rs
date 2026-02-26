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
    use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
    use embassy_sync::signal::Signal;

    #[init]
    fn init() {
        u5_lib::clock::init_clock(true, u5_lib::clock::ClockFreqs::KernelFreq160Mhz);
    }

    struct MyHandler<'a> {
        configured: &'a Signal<CriticalSectionRawMutex, ()>,
    }

    impl<'a> embassy_usb::Handler for MyHandler<'a> {
        fn enabled(&mut self, _enabled: bool) {
            #[cfg(feature = "defmt")]
            defmt::info!("USB enabled: {}", _enabled);
        }
        fn reset(&mut self) {
            #[cfg(feature = "defmt")]
            defmt::info!("USB reset");
        }
        fn addressed(&mut self, _addr: u8) {
            #[cfg(feature = "defmt")]
            defmt::info!("USB addressed: {}", _addr);
        }
        fn configured(&mut self, _configured: bool) {
            #[cfg(feature = "defmt")]
            defmt::info!("USB configured: {}", _configured);
            if _configured {
                self.configured.signal(());
            }
        }
        fn suspended(&mut self, _suspended: bool) {
            #[cfg(feature = "defmt")]
            if _suspended {
                defmt::info!("USB suspended");
            } else {
                defmt::info!("USB resumed");
            }
        }
    }

    #[test]
    #[timeout(30)]
    async fn test_usb_enumeration() {
        #[cfg(feature = "defmt")]
        defmt::info!("Initializing USB...");

        // Buffer for endpoints
        static mut EP_OUT_BUFFER: [u8; 1024] = [0; 1024];
        let ep_out_buffer = &raw mut EP_OUT_BUFFER;

        // Config
        let config = Config::default();
        let driver = Driver::new_fs(USB_DP_PA12, USB_DM_PA11, unsafe { &mut *ep_out_buffer }, config);

        // Embassy USB Config
        let mut usb_config = embassy_usb::Config::new(0x1234, 0x5678);
        usb_config.device_class = 0x00;
        usb_config.device_sub_class = 0x00;
        usb_config.device_protocol = 0x00;
        usb_config.composite_with_iads = false;

        // Buffers for control endpoint
        static mut CONFIG_DESC: [u8; 256] = [0; 256];
        static mut BOS_DESC: [u8; 256] = [0; 256];
        static mut MSOS_DESC: [u8; 256] = [0; 256];
        static mut CONTROL_BUF: [u8; 64] = [0; 64];

        let config_desc = unsafe { &mut *(&raw mut CONFIG_DESC) };
        let bos_desc = unsafe { &mut *(&raw mut BOS_DESC) };
        let msos_desc = unsafe { &mut *(&raw mut MSOS_DESC) };
        let control_buf = unsafe { &mut *(&raw mut CONTROL_BUF) };

        static CONFIGURED: Signal<CriticalSectionRawMutex, ()> = Signal::new();
        let mut handler = MyHandler { configured: &CONFIGURED };

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
        let usb_fut = usb.run();
        let configured_fut = CONFIGURED.wait();

        embassy_futures::select::select(usb_fut, configured_fut).await;

        #[cfg(feature = "defmt")]
        defmt::info!("USB device configured! Test passed.");
    }
}
