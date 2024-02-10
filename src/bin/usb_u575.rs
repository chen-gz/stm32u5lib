#![feature(noop_waker)]
#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use core::default::Default;
// use ebcmd::Response;
use embassy_usb::{
    class::cdc_acm::{CdcAcmClass, State},
    driver::EndpointError,
    Builder,
};
use u5_lib::{clock, gpio, rtc, sdmmc::SdInstance, *};
use u5_lib::{exti};

use defmt_rtt as _;
use gpio::GpioPort;

const GREEN: GpioPort = gpio::PB7;

fn setup() {
    GREEN.setup();
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    clock::init_clock();
    setup();
    defmt::info!("setup led finished!");
    spawner.spawn(btn()).unwrap();
    spawner.spawn(usb_task()).unwrap();
}

use core::panic::PanicInfo;
use embassy_executor::Spawner;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[embassy_executor::task]
async fn btn() {
    let _last_time: (u8, u8, u8) = (0, 0, 0);
    defmt::info!("waiting for btn");
    loop {
        exti::EXTI13_PC13.wait_for_raising().await;
        GREEN.toggle();
    }
}


const SIZE_BLOCK: u32 = 1;
// first block store the number of image files
const IMG_START_BLOCK: u32 = 10;
const IMG_SIZE: u32 = 2000; // 2000 block = 2000 * 512 = 1M

// use embassy_stm32::{
//     bind_interrupts,
//     peripherals::{self},
//     usb_otg::{self, Driver, Instance},
// };

// bind_interrupts!(struct Irqs {
//     OTG_FS => usb_otg::InterruptHandler<peripherals::USB_OTG_FS>;
// });
use futures::future::join;
use sdio_host::sd_cmd::speed_class_control;

#[embassy_executor::task]
pub async fn usb_task() {
    // let p = unsafe { embassy_stm32::Peripherals::steal() };
    // init USB CDC
    let mut ep_out_buffer = [0u8; 256];
    // let mut config = embassy_stm32::usb_otg::Config::default();
    // config.vbus_detection = true;
    // config.vbus_detection = false;
    // let driver = Driver::new_fs(
    //     p.USB_OTG_FS,
    //     Irqs,
    //     p.PA12,
    //     p.PA11,
    //     &mut ep_out_buffer,
    //     config,
    // );
    let config = usb::Config::default();
    let driver = u5_lib::usb::Driver::new(config);
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

            u5_lib::clock::run_with_160mhz_async(|| async {
                u5_lib::low_power::run_no_deep_sleep_async(|| async {
                    // clock::set_clock_to_pll(); // fast clock for camera
                    // SIGNAL.signal(1);
                    // clock::set_clock_to_pll();
                    // clock::set_cpu_freq(160_000_000);
                    defmt::info!("connected");
                    let _ = usb_handler(&mut class).await;
                    defmt::info!("disconnected");
                    // clock::set_clock_to_hsi();
                    // SIGNAL.signal(0);
                })
                    .await;
            })
                .await;
            // LED_BLUE.set_high();
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

async fn usb_handler<'d>(
    class: &mut CdcAcmClass<'d, usb::Driver>,
) -> Result<(), Disconnected> {
    let mut buf: [u8; 128] = [0; 128];
    // the maximum size of the command is 64 bytes
    defmt::info!("start usb handler");
    loop {
        let n = class.read_packet(&mut buf).await.unwrap();
        class.write_packet(&buf[0..n]).await.unwrap();
    }
}
