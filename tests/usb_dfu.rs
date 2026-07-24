#![no_std]
#![no_main]

#[cfg(feature = "defmt")]
use defmt_rtt as _;
use u5_lib as _; // links panic handler

#[embedded_test::tests]
mod tests {
    use embassy_usb::Builder;
    use u5_lib::dfu::{dfu_attributes, DfuClass, APP_BASE_ADDRESS};
    use u5_lib::gpio::{USB_DM_PA11, USB_DP_PA12};
    use u5_lib::otg::{Config, Driver};

    #[init]
    fn init() {
        u5_lib::clock::init_clock(true, u5_lib::clock::ClockFreqs::KernelFreq160Mhz);
    }

    #[test]
    #[timeout(30)]
    async fn test_usb_dfu_initialization() {
        #[cfg(feature = "defmt")]
        defmt::info!("Initializing USB DFU...");

        // Buffer for endpoints
        static mut EP_OUT_BUFFER: [u8; 1024] = [0; 1024];
        let ep_out_buffer = &raw mut EP_OUT_BUFFER;

        // Config
        let config = Config::default();
        let driver = Driver::new_fs(
            USB_DP_PA12,
            USB_DM_PA11,
            unsafe { &mut *ep_out_buffer },
            config,
        );

        // Embassy USB Config
        let mut usb_config = embassy_usb::Config::new(0x0483, 0xdf11); // Standard STMicroelectronics DFU VID/PID
        usb_config.manufacturer = Some("STMicroelectronics");
        usb_config.product = Some("STM32U5 DFU Bootloader");
        usb_config.serial_number = Some("DFU-U5-001");
        usb_config.max_power = 100;
        usb_config.max_packet_size_0 = 64;

        // Buffers for control endpoint
        static mut CONFIG_DESC: [u8; 256] = [0; 256];
        static mut BOS_DESC: [u8; 256] = [0; 256];
        static mut MSOS_DESC: [u8; 256] = [0; 256];
        static mut CONTROL_BUF: [u8; 64] = [0; 64];
        static mut DFU_BUFFER: [u8; 2048] = [0; 2048];

        let config_desc = unsafe { &mut *core::ptr::addr_of_mut!(CONFIG_DESC) };
        let bos_desc = unsafe { &mut *core::ptr::addr_of_mut!(BOS_DESC) };
        let msos_desc = unsafe { &mut *core::ptr::addr_of_mut!(MSOS_DESC) };
        let control_buf = unsafe { &mut *core::ptr::addr_of_mut!(CONTROL_BUF) };
        let dfu_buf = unsafe { &mut *core::ptr::addr_of_mut!(DFU_BUFFER) };

        let mut dfu_class = DfuClass::new(dfu_buf, 2048, APP_BASE_ADDRESS);

        let mut builder = Builder::new(
            driver,
            usb_config,
            config_desc,
            bos_desc,
            msos_desc,
            control_buf,
        );

        // Register DFU Interface and Descriptor
        DfuClass::register(
            &mut builder,
            2048,
            1000,
            dfu_attributes::CAN_DOWNLOAD | dfu_attributes::CAN_UPLOAD | dfu_attributes::WILL_DETACH,
        );

        builder.handler(&mut dfu_class);

        let _usb = builder.build();

        #[cfg(feature = "defmt")]
        defmt::info!("USB DFU initialized successfully.");
    }
}
