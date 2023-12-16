#![no_std]
#![no_main]

#![feature(type_alias_impl_trait)]
#![allow(dead_code)]
#![allow(unused_imports)]
use heapless::String;
use core::fmt::Write;

use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_stm32::{bind_interrupts, peripherals, usb_otg::{self, Instance}, usb_otg::Driver};
use embassy_usb::{
    class::cdc_acm::{CdcAcmClass, State},
    Builder, driver::EndpointError,
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

static SIGNAL: Signal<CriticalSectionRawMutex, u32> = Signal::new();
static SIGNAL2: Signal<CriticalSectionRawMutex, u32> = Signal::new();

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
    rtc::setup(23, 12, 14, 21, 33, 0);
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    setup();
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
    let echo_fut = async {
        loop {
            class.wait_connection().await;
            defmt::info!("connected");
            let _ = echo(&mut class).await;
            defmt::info!("disconnected");
        }
    };

    join(usb_fut, echo_fut).await; // Run everything concurrently.
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


async fn echo<'d, T: Instance + 'd>(class: &mut CdcAcmClass<'d, Driver<'d, T>>) -> Result<(), Disconnected> {
    let mut buf = [0; 64];
    loop {
            let n = class.read_packet(&mut buf).await.unwrap();
            let data = &buf[..n];
            // info!("data: {:x}", data);
            let time = rtc::get_time();
            let mut s = String::<64>::new();
            s.write_fmt(format_args!("{}: ", time)).unwrap();
            // convert s to u8 array
            let mut buf_u8 = [0; 64];
            for (i, c) in s.as_bytes().iter().enumerate() {
                buf_u8[i] = *c;
            }
            class.write_packet(data).await.unwrap();
            class.write_packet(&buf_u8).await.unwrap();
            LED_BLUE.toggle();
    }
}
