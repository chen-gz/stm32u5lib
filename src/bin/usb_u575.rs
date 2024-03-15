#![feature(noop_waker)]
#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use core::default::Default;
use core::panic::PanicInfo;
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_usb::{
    class::cdc_acm::{CdcAcmClass, State},
    driver::EndpointError,
    Builder,
};
use futures::future::{join, select};
use u5_lib::{clock, gpio, *};
use u5_lib::{exti, low_power::mcu_no_deep_sleep};

// define defmt format
#[derive(defmt::Format)]
pub enum UsbError {
    BufferOverflow,
    Disabled,
}

const GREEN: gpio::GpioPort = gpio::PB7;

fn setup() {
    clock::init_clock();
    GREEN.setup();
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    setup();
    mcu_no_deep_sleep();
    clock::kernel_freq_160mhz_request();
    defmt::info!("setup led finished!");
    spawner.spawn(btn()).unwrap();
    spawner.spawn(usb_task()).unwrap();
    loop {
        rtc::rtc_interrupt().await;
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    defmt::info!("panic");
    defmt::error!(
        "Location file name: {:?}, line: {:?}, col: {:?}",
        _info.location().unwrap().file(),
        _info.location().unwrap().line(),
        _info.location().unwrap().column()
    );

    loop {}
}

#[embassy_executor::task]
async fn btn() {
    let _last_time: (u8, u8, u8) = (0, 0, 0);
    defmt::info!("waiting for btn");
    loop {
        exti::EXTI13_PC13.wait_for_raising().await;
        GREEN.toggle();
        defmt::info!("button clicked");
    }
}

#[embassy_executor::task]
pub async fn usb_task() {
    let _ep_out_buffer = [0u8; 256];
    let mut config = usb_otg::Config::default();
    config.vbus_detection = false;
    let driver = usb_otg::Driver::new(config, gpio::USB_DM_PA11, gpio::USB_DP_PA12);

    // // Create embassy-usb Config
    let mut config = embassy_usb::Config::new(0xabcd, 0xefba);
    config.manufacturer = Some("ggeta");
    config.product = Some("USB-serial example");
    config.serial_number = Some("12345678");

    config.device_class = 0xEF;
    config.device_sub_class = 0x02;
    config.device_protocol = 0x01;
    config.composite_with_iads = true;

    let mut device_descriptor = [0; 512];
    let mut config_descriptor = [0; 512];
    let mut bos_descriptor = [0; 512];
    let mut control_buf = [0; 64];
    let mut msos_descriptor = [0; 512];

    let mut state = State::new();

    let mut builder = Builder::new(
        driver,
        config,
        &mut device_descriptor,
        &mut config_descriptor,
        &mut bos_descriptor,
        &mut msos_descriptor,
        &mut control_buf,
    );

    let mut class = CdcAcmClass::new(&mut builder, &mut state, 64);
    // Build the builder.
    let mut usb = builder.build();
    let usb_fut = usb.run(); // Run the USB device.
    let handler_fut = async {
        loop {
            class.wait_connection().await;
            defmt::info!("connected");
            let _ = usb_handler(&mut class).await;
            defmt::info!("disconnected");
        }
    };
    join(usb_fut, handler_fut).await; // Run everything concurrently.
}

struct Disconnected {}

impl From<EndpointError> for Disconnected {
    fn from(val: EndpointError) -> Self {
        match val {
            EndpointError::BufferOverflow => panic!("Buffer overflow"),
            EndpointError::Disabled => Disconnected {},
        }
    }
}

async fn usb_handler<'d>(class: &mut CdcAcmClass<'d, usb_otg::Driver>) -> Result<(), Disconnected> {
    let mut buf: [u8; 128] = [0; 128];
    // the maximum size of the command is 64 bytes
    defmt::info!("start usb handler");
    loop {
        // select(future1, future2)
        let n = class.read_packet(&mut buf).await.unwrap();
        class.write_packet(&buf[0..n]).await.unwrap();
    }
}
