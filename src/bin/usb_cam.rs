#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, signal::Signal};
use u5_lib::*;

#[path = "../camera.rs"]
mod camera;

use stm32_metapac::RCC;
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    defmt::info!("panic");
    defmt::error!(
        "Location file name: {:?}, line: {:?}, col: {:?}",
        _info.location().unwrap().file(),
        _info.location().unwrap().line(),
        _info.location().unwrap().column()
    );
    loop {}
}

const LED_GREEN: gpio::GpioPort = gpio::PC3;
const LED_ORANGE: gpio::GpioPort = gpio::PC4;
const LED_BLUE: gpio::GpioPort = gpio::PC5;

const CAM_PDWN: gpio::GpioPort = gpio::PB0;

fn setup() {
    clock::init_clock();
    CAM_PDWN.setup();
    LED_ORANGE.setup();
    LED_GREEN.setup();
    LED_BLUE.setup();

    LED_GREEN.set_high();
    LED_ORANGE.set_high();
    LED_BLUE.set_high();
    CAM_PDWN.set_high();
}

fn setup_cam_clk() {
    let cam_clk: gpio::GpioPort = gpio::GPIO_MCO_PA8;
    cam_clk.setup();
    // set PA8 as mco output for HSI48 and divide by 2 (24Mhz)
    RCC.cfgr1().modify(|w| {
        w.set_mcosel(stm32_metapac::rcc::vals::Mcosel::HSI);
        w.set_mcopre(stm32_metapac::rcc::vals::Mcopre::DIV1);
    });
}

fn setup_camera_dcmi() -> dcmi::DcmiPort {
    setup_cam_clk();
    CAM_PDWN.set_low();
    clock::delay_ms(100);
    let cam_i2c: gi2c::I2cPort = gi2c::I2C3;
    cam_i2c.init(100_000, gpio::I2C3_SCL_PC0, gpio::I2C3_SDA_PB4);
    camera::setup_camera(cam_i2c);
    let dcmi = dcmi::DCMI;
    dcmi.init(
        gpio::DCMI_D0_PC6,
        gpio::DCMI_D1_PC7,
        gpio::DCMI_D2_PC8,
        gpio::DCMI_D3_PC9,
        gpio::DCMI_D4_PC11,
        gpio::DCMI_D5_PB6,
        gpio::DCMI_D6_PB8,
        gpio::DCMI_D7_PB9,
        gpio::DCMI_HSYNC_PA4,
        gpio::DCMI_VSYNC_PB7,
        gpio::DCMI_PIXCLK_PA6,
    );
    LED_BLUE.set_low();
    clock::delay_ms(5000);
    // CAM_PDWN.set_high();

    let mut reg_val = [0u8; 3];
    reg_val[0] = (ov5640_reg::OV5640_SYSTEM_CTROL0 >> 8) as u8;
    reg_val[1] = ov5640_reg::OV5640_SYSTEM_CTROL0 as u8;
    reg_val[2] = (1 << 6) | 0x02;

    gi2c::I2C3
        .write(ov5640_reg::OV5640_I2C_ADDR, &reg_val)
        .unwrap();
    dcmi
}

// #[path = "../usb_util.rs"]
// mod usb_util;


#[embassy_executor::main]
// #[embassy_executor::task]
async fn async_main(spawner: Spawner) {
    setup();
    LED_GREEN.set_high();
    LED_ORANGE.set_high();
    LED_BLUE.set_high();
    LED_ORANGE.set_low();

    let _dcmi = setup_camera_dcmi();

    clock::delay_ms(10);

    spawner.spawn(usb_task()).unwrap();
    spawner.spawn(btn()).unwrap();
    // CAM_PDWN.set_high();

    LED_GREEN.set_low();
    loop {
        // clock::delay_ms(10);
        // const PIC_BUF_SIZE: usize = 512 * 1300; // 512byte * 1300 = 650K
        //                                         // let _pic_buf = [0u8; PIC_BUF_SIZE];
        loop {
            // if unsafe { TAKE_PIC } == false {
            //     rtc::rtc_interrupt().await;
            //     continue;
            // }
            // LED_BLUE.set_high();
            // // camera::capture(&CAM_PDWN, &dcmi, &mut pic_buf, &sd).await;
            // LED_BLUE.set_low();

            // let mut pic_buf = [0u8; 512 * 1300];
            // camera::capture(&CAM_PDWN, &_dcmi, &mut pic_buf).await;
            rtc::rtc_interrupt().await;
        }
    }
}

static SIGNAL: Signal<CriticalSectionRawMutex, bool> = Signal::new();

#[embassy_executor::task]
async fn btn() {
    let mut last_time: (u8, u8, u8) = (0, 0, 0);
    loop {
        exti::EXTI2_PB2.wait_for_raising().await;
        SIGNAL.signal(true);
        
    }
}

use core::default::Default;
use embassy_usb::{
    class::cdc_acm::{CdcAcmClass, State},
    driver::EndpointError,
    Builder,
};
// use u5_lib::{clock, gpio, rtc, sdmmc::SdInstance, *};

const SIZE_BLOCK: u32 = 1; // first block store the number of image files
const IMG_START_BLOCK: u32 = 10;
const IMG_SIZE: u32 = 2000; // 2000 block = 2000 * 512 = 1M
use futures::future::join;

#[embassy_executor::task]
pub async fn usb_task() {
    // let mut ep_out_buffer = [0u8; 256];
    let mut  config = Config::default();
    config.vbus_detection = false;
    let driver = u5_lib::Driver::new(config, gpio::USB_DM_PA11, gpio::USB_DP_PA12);
    // Create embassy-usb Config
    let mut config = embassy_usb::Config::new(0xc0de, 0xcafe);
    config.manufacturer = Some("Embassy");
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

            u5_lib::clock::run_with_160mhz_async(|| async {
                u5_lib::low_power::run_no_deep_sleep_async(|| async {
                    defmt::info!("connected");
                    let _ = usb_handler(&mut class).await;
                    defmt::info!("disconnected");
                })
                .await;
            })
            .await;
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
// #[path = "../ebcmd.rs"]
// mod ebcmd;
// use eb_cmds as ebcmd;


async fn usb_handler<'d>(class: &mut CdcAcmClass<'d, usb_otg::Driver>) -> Result<(), Disconnected> {
    let mut buf: [u8; 128] = [0; 128]; 
    defmt::info!("start usb handler");
    
        // let dcmi = setup_camera_dcmi();

    let _dcmi = dcmi::DCMI;
    // _dcmi.init(
    //     gpio::DCMI_D0_PC6,
    //     gpio::DCMI_D1_PC7,
    //     gpio::DCMI_D2_PC8,
    //     gpio::DCMI_D3_PC9,
    //     gpio::DCMI_D4_PC11,
    //     gpio::DCMI_D5_PB6,
    //     gpio::DCMI_D6_PB8,
    //     gpio::DCMI_D7_PB9,
    //     gpio::DCMI_HSYNC_PA4,
    //     gpio::DCMI_VSYNC_PB7,
    //     gpio::DCMI_PIXCLK_PA6,
    // );
        let _pdwn = gpio::PB0;
    loop {
        // let n = class.read_packet(&mut buf).await.unwrap();
        // let command = ebcmd::Command::from_array(&buf[..n]);
        // if btn click take a picture and send to host

        SIGNAL.wait().await;
        LED_BLUE.toggle();
        // take a picture and send to host
        let mut pic_buf = [0u8; 512 * 1300];
        camera::capture(&_pdwn, &_dcmi, &mut pic_buf).await;
        // separate picture 32 byte per packet
        for i in 0..1300*(512/32) {
            let start = i*32;
            let end = (i+1)*32;
            // let response = ebcmd::Response::Pic(pic_buf[start..end].to_vec());
            // let response = response.to_array();
            let buf = &pic_buf[start..end];
            class.write_packet(buf).await.unwrap();
            // class.write_packet(&response).await.unwrap();

        }
    }
}
