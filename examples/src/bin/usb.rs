#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]
#![allow(dead_code)]
#![allow(unused_imports)]
use core::fmt::Write;
use heapless::String;

use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_stm32::{
    bind_interrupts, peripherals,
    usb_otg::Driver,
    usb_otg::{self, Instance},
};
use embassy_usb::{
    class::cdc_acm::{CdcAcmClass, State},
    driver::EndpointError,
    Builder,
};
use futures::future::join;

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

bind_interrupts!(struct Irqs {
    OTG_FS => usb_otg::InterruptHandler<peripherals::USB_OTG_FS>;
});
use u5_lib::*;

use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, signal::Signal};
use embassy_time::{Duration, Timer};

const LED_GREEN: gpio::GpioPort = gpio::PC3;
const LED_ORANGE: gpio::GpioPort = gpio::PC4;
const LED_BLUE: gpio::GpioPort = gpio::PC5;
const CAM_I2C: gi2c::I2cPort = gi2c::I2C3;
const CAM_PDWN: gpio::GpioPort = gpio::PB0;

use u5_lib::rtc;
fn setup() {
    clock::init_clock();
    CAM_PDWN.setup();
    LED_ORANGE.setup();
    LED_ORANGE.set_high();
    LED_GREEN.setup();
    LED_GREEN.set_high();
    LED_BLUE.setup();
    LED_BLUE.set_high();
    // setup rtc
    // rtc::setup(23, 12, 14, 21, 33, 0);
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    setup();
    rtc::enable_rtc_read();
    defmt::info!("init clock finished");
    clock::delay_ms(10);
    spawner.spawn(usb_task()).unwrap();
    LED_BLUE.set_low();
}

#[embassy_executor::task]
async fn usb_task() {
    let p = unsafe { embassy_stm32::Peripherals::steal() };
    // init USB CDC
    let mut ep_out_buffer = [0u8; 256];
    let mut config = embassy_stm32::usb_otg::Config::default();
    // config.vbus_detection = true;
    config.vbus_detection = false;
    let driver = Driver::new_fs(
        p.USB_OTG_FS,
        Irqs,
        p.PA12,
        p.PA11,
        &mut ep_out_buffer,
        config,
    );
    //
    // // Create embassy-usb Config
    let mut config = embassy_usb::Config::new(0xc0de, 0xcafe);
    config.manufacturer = Some("Embassy");
    config.product = Some("USB-serial example");
    config.serial_number = Some("12345678");

    config.device_class = 0xEF;
    config.device_sub_class = 0x02;
    config.device_protocol = 0x01;
    config.composite_with_iads = true;

    let mut device_descriptor = [0; 256];
    let mut config_descriptor = [0; 256];
    let mut bos_descriptor = [0; 256];
    let mut control_buf = [0; 64];
    let mut msos_descriptor = [0; 256];

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

#[path = "../ebcmd.rs"]
mod ebcmd;
async fn usb_handler<'d, T: Instance + 'd>(
    class: &mut CdcAcmClass<'d, Driver<'d, T>>,
) -> Result<(), Disconnected> {
    let mut buf: [u8; 128] = [0; 128]; // the maximum size of the command is 64 bytes
    loop {
        let n = class.read_packet(&mut buf).await.unwrap();
        // let command = ebcmd::bytes_to_command(&buf[..n]);
        let command = ebcmd::Command::from_array(&buf[..n]);
        let res = match command {
            ebcmd::Command::SetTim(year, month, day, hour, minute, second) => {
                rtc::setup(year, month, day, hour, minute, second);
                ebcmd::Response::SetTim(0)
            }
            ebcmd::Command::GetTim => {
                let date = rtc::get_date();
                let time = rtc::get_time();
                ebcmd::Response::GetTim(date.0, date.1, date.2, time.0, time.1, time.2)
            }
            ebcmd::Command::GetPic(_id) => {
                let mut buf = [0; 64];
                buf[0] = 0x02;
                // get data from sd storage and put it into buf
                let _pic_buf = [0; 256];
                ebcmd::Response::GetTim(0, 0, 0, 0, 0, 0)
            }
        };
        let (buf, len) = ebcmd::response_to_bytes(res);
        class.write_packet(&buf[0..len]).await.unwrap();
        LED_BLUE.toggle();
    }
}
