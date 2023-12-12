#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]
#![allow(dead_code)]
#![allow(unused_imports)]

use core::future::Future;
use core::hash::Hasher;
use cortex_m::prelude::_embedded_hal_blocking_spi_Write;
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_stm32::{
    bind_interrupts,
    gpio::{Output, OutputType},
    peripherals,
    time::{khz, Hertz},
    usb_otg,
    usb_otg::{Driver, Instance},
    Config, Peripheral,
};
use embassy_usb::{
    class::cdc_acm::{CdcAcmClass, State},
    driver::EndpointError,
    Builder, UsbDevice,
};
use futures::future::join;

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

bind_interrupts!(struct Irqs {
    OTG_FS => usb_otg::InterruptHandler<peripherals::USB_OTG_FS>;
});
// #[path = "../ov5640_reg.rs"]
// mod ov5640_reg;
use u5_lib::ov5640_reg;
use u5_lib::*;

use ov5640_reg::*;

use embassy_stm32::timer::simple_pwm;
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, signal::Signal};
use embassy_time::{Duration, Timer};
use stm32_metapac::{
    gpdma::vals::ChTr1Ap, gpio::vals::Ospeedr, timer::vals::Arpe, GPDMA1, GPIOB, GPIOC, I2C3, PWR,
    RCC, TIM1,
};

// const PIC_BUF_SIZE: usize = 130_000;
const PIC_BUF_SIZE: usize = 0xffff;
// const PIC_BUF_SIZE: usize = 140_000;
static mut PIC_BUF: [u32; PIC_BUF_SIZE] = [0; PIC_BUF_SIZE];
static SIGNAL: Signal<CriticalSectionRawMutex, u32> = Signal::new();
static SIGNAL2: Signal<CriticalSectionRawMutex, u32> = Signal::new();

const LED_GREEN: gpio::GpioPort = gpio::PC3;
const LED_ORANGE: gpio::GpioPort = gpio::PC4;
const LED_BLUE: gpio::GpioPort = gpio::PC5;
const CAM_I2C: gi2c::I2cPort = gi2c::I2C3;
const CAM_PDWN: gpio::GpioPort = gpio::PB0;

fn setup() {
    // this function setup for peripheral
    clock::init_clock();
    CAM_PDWN.setup();
    LED_ORANGE.setup();
    LED_ORANGE.set_high();
    LED_GREEN.setup();
    LED_GREEN.set_high();
    LED_BLUE.setup();
    LED_BLUE.set_high();
    CAM_I2C.init(100_000, gpio::I2C3_SCL_PC0, gpio::I2C3_SDA_PB4);
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    setup();
    defmt::info!("init clock finished");
    setup_cam_clk();
    // CAM_PDWN.set_high();
    clock::delay_ms(10);
    setup_camera();

    // start usb task
    // spawner.spawn(usb_task()).unwrap();

    spawner.spawn(usb_task()).unwrap();

    GPDMA1.ch(0).tr1().modify(|w| w.set_dap(ChTr1Ap::PORT1));
    loop {
        Timer::after(Duration::from_secs(1)).await;
        LED_GREEN.toggle();
    }
    // DCMI.capture(&mut PIC_BUF).await;

    // loop {
    //     unsafe {
    //         dcmi.capture(&mut PIC_BUF).await;
    //     }
    //     let buf = unsafe { &PIC_BUF };
    //     if unsafe { (PIC_BUF[0] & 0xffff) == 0xd8ff } {
    //     } else {
    //         continue;
    //     }
    //     // try to find oxff 0xd9
    //     let mut pix_size = 0;
    //     for i in 0..PIC_BUF_SIZE {
    //         if unsafe { PIC_BUF[i] & 0xffff == 0xd9ff || (PIC_BUF[i] >> 16) & 0xffff == 0xd9ff } {
    //             // led_red.toggle();
    //             LED_GREEN.toggle();
    //             pix_size = i;
    //             break;
    //         }
    //     }
    //     SIGNAL.signal(pix_size as u32);
    //     SIGNAL2.wait().await;
    //     Timer::after(Duration::from_secs(3)).await;
    // }
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

    // Create embassy-usb DeviceBuilder using the driver and config.
    // It needs some buffers for building the descriptors.
    let mut device_descriptor = [0; 256];
    let mut config_descriptor = [0; 256];
    let mut bos_descriptor = [0; 256];
    let mut control_buf = [0; 64];
    let mut msos_descriptor = [0; 256];

    let mut state = State::new();

    // Create classes on the builder.
    //
    // pub fn new(
    //     driver: D,
    //     config: Config<'d>,
    //     device_descriptor_buf: &'d mut [u8],
    //     config_descriptor_buf: &'d mut [u8],
    //     bos_descriptor_buf: &'d mut [u8],
    //     msos_descriptor_buf: &'d mut [u8],
    //     control_buf: &'d mut [u8],
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
                             // Do stuff with the class!
    let echo_fut = async {
        loop {
            class.wait_connection().await;
            let num = SIGNAL.wait().await as usize;
            let times = num / 16 + 1;
            let mut addr_shift = 0;

            for i in 0..times {
                let mut buf_u8 = [0u8; 64];
                for j in 0..16 {
                    if (i * 16 + j > num + 1) {
                        break;
                    }
                    unsafe {
                        buf_u8[4 * j] = (PIC_BUF[i * 16 + j]) as u8;
                        buf_u8[4 * j + 1] = (PIC_BUF[i * 16 + j] >> 8) as u8;
                        buf_u8[4 * j + 2] = (PIC_BUF[i * 16 + j] >> 16) as u8;
                        buf_u8[4 * j + 3] = (PIC_BUF[i * 16 + j] >> 24) as u8;
                    }
                }
                class.write_packet(&buf_u8).await;
            }
            SIGNAL2.signal(1);
        }
    };

    join(usb_fut, echo_fut).await; // Run everything concurrently.
    loop {
        Timer::after(Duration::from_secs(1)).await;
    }
}
