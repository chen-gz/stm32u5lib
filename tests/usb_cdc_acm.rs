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
    use embassy_usb::class::cdc_acm::{CdcAcmClass, State};
    use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
    use embassy_sync::signal::Signal;

    #[init]
    fn init() {
        u5_lib::clock::init_clock(true, u5_lib::clock::ClockFreqs::KernelFreq160Mhz);
    }

    #[test]
    #[timeout(60)]
    async fn test_usb_cdc_acm() {
        #[cfg(feature = "defmt")]
        defmt::info!("Initializing USB CDC ACM...");

        // Buffer for endpoints
        static mut EP_OUT_BUFFER: [u8; 1024] = [0; 1024];
        let ep_out_buffer = &raw mut EP_OUT_BUFFER;

        // Config
        let config = Config::default();
        let driver = Driver::new_fs(USB_DP_PA12, USB_DM_PA11, unsafe { &mut *ep_out_buffer }, config);

        // Embassy USB Config
        let mut usb_config = embassy_usb::Config::new(0x1234, 0x5678);
        usb_config.manufacturer = Some("Embassy");
        usb_config.product = Some("USB-serial example");
        usb_config.serial_number = Some("12345678");
        usb_config.max_power = 100;
        usb_config.max_packet_size_0 = 64;

        // Buffers for control endpoint
        static mut CONFIG_DESC: [u8; 256] = [0; 256];
        static mut BOS_DESC: [u8; 256] = [0; 256];
        static mut MSOS_DESC: [u8; 256] = [0; 256];
        static mut CONTROL_BUF: [u8; 64] = [0; 64];

        let config_desc = unsafe { &mut *(&raw mut CONFIG_DESC) };
        let bos_desc = unsafe { &mut *(&raw mut BOS_DESC) };
        let msos_desc = unsafe { &mut *(&raw mut MSOS_DESC) };
        let control_buf = unsafe { &mut *(&raw mut CONTROL_BUF) };

        let mut state = State::new();
        let mut builder = Builder::new(
            driver,
            usb_config,
            config_desc,
            bos_desc,
            msos_desc,
            control_buf,
        );

        // Create classes on the builder.
        let mut class = CdcAcmClass::new(&mut builder, &mut state, 64);

        let mut usb = builder.build();

        #[cfg(feature = "defmt")]
        defmt::info!("Running USB device. Connect to host and open serial port.");

        static QUIT: Signal<CriticalSectionRawMutex, ()> = Signal::new();

        let usb_fut = usb.run();
        let echo_fut = async {
            loop {
                #[cfg(feature = "defmt")]
                defmt::info!("Waiting for connection...");
                class.wait_connection().await;
                #[cfg(feature = "defmt")]
                defmt::info!("Connected! Starting echo loop.");
                let _ = echo(&mut class, &QUIT).await;
                #[cfg(feature = "defmt")]
                defmt::info!("Disconnected");
            }
        };

        embassy_futures::select::select3(usb_fut, echo_fut, QUIT.wait()).await;

        #[cfg(feature = "defmt")]
        defmt::info!("Test finished via QUIT command");
    }

    async fn echo<'d, D: embassy_usb_driver::Driver<'d>>(
        class: &mut CdcAcmClass<'d, D>,
        quit: &Signal<CriticalSectionRawMutex, ()>,
    ) -> Result<(), embassy_usb::driver::EndpointError> {
        let mut buf = [0; 64];
        loop {
            #[cfg(feature = "defmt")]
            defmt::trace!("Waiting for packet...");
            let n = class.read_packet(&mut buf).await?;
            let data = &buf[..n];
            #[cfg(feature = "defmt")]
            defmt::info!("Received {} bytes: {:x}", n, data);

            if data == b"QUIT" {
                #[cfg(feature = "defmt")]
                defmt::info!("QUIT command received");
                quit.signal(());
                return Ok(());
            }

            class.write_packet(data).await?;
            #[cfg(feature = "defmt")]
            defmt::trace!("Echoed packet back");
        }
    }
}
