#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]
#![allow(dead_code)]
#![allow(unused_imports)]
use core::fmt::Write;
use cortex_m::delay;
use embedded_sdmmc::BlockDevice;
use heapless::String;

use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_stm32::{
    bind_interrupts,
    peripherals::{self, RCC},
    usb_otg::Driver,
    usb_otg::{self, Instance},
};
use embassy_usb::{
    class::cdc_acm::{CdcAcmClass, State},
    driver::EndpointError,
    Builder, msos::DescriptorSetInformation,
};
use futures::future::join;

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

bind_interrupts!(struct Irqs {
    OTG_FS => usb_otg::InterruptHandler<peripherals::USB_OTG_FS>;
});
use u5_lib::gpio::{SDMMC2_CK_PC1, SDMMC2_CMD_PA0, SDMMC2_D0_PB14};
use u5_lib::{sdmmc::SdInstance, *};

use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, signal::Signal};
use embassy_time::{Duration, Timer};

const LED_GREEN: gpio::GpioPort = gpio::PC3;
const LED_ORANGE: gpio::GpioPort = gpio::PC4;
const LED_BLUE: gpio::GpioPort = gpio::PC5;
const CAM_I2C: gi2c::I2cPort = gi2c::I2C3;
const CAM_PDWN: gpio::GpioPort = gpio::PB0;
const CAM_CLOCK: gpio::GpioPort = gpio::MCO_PA8;
static SIGNAL: Signal<CriticalSectionRawMutex, u32> = Signal::new();
static SIGNAL_SD_INST: Signal<CriticalSectionRawMutex, SdInstance> = Signal::new();

use u5_lib::rtc;

use u5_lib::{
    gpio::{
        DCMI_D0_PC6, DCMI_D1_PC7, DCMI_D2_PC8, DCMI_D3_PC9, DCMI_D4_PC11, DCMI_D5_PB6, DCMI_D6_PB8,
        DCMI_D7_PB9, DCMI_HSYNC_PA4, DCMI_PIXCLK_PA6, DCMI_VSYNC_PB7,
    },
    ov5640_reg,
};
fn setup() {
    clock::init_clock();
    CAM_PDWN.setup();
    // CAM_PDWN.set_high();
    LED_ORANGE.setup();
    LED_ORANGE.set_high();
    LED_GREEN.setup();
    LED_GREEN.set_high();
    LED_BLUE.setup();
    LED_BLUE.set_high();
    // CAM_CLOCK.setup();
    // CAM_I2C.init(100_000, gpio::I2C3_SCL_PC0, gpio::I2C3_SDA_PB4);
    CAM_I2C.init(100_000, gpio::I2C3_SCL_PC0, gpio::I2C3_SDA_PB4);
}
use stm32_metapac::RCC;

use embassy_stm32::{
    timer::simple_pwm,

    gpio::{Output, OutputType},
};
use embassy_stm32::time::khz;
use stm32_metapac::TIM1;
use stm32_metapac::timer::vals::Arpe;
fn setup_cam_clk() {
    // set PA8 as mco output for HSI48 and divide by 2 (24Mhz)
    // RCC.cfgr1().modify(|w| {
    //     w.set_mcosel(stm32_metapac::rcc::vals::Mcosel::HSI48);
    //     w.set_mcopre(stm32_metapac::rcc::vals::Mcopre::DIV2);
    // });
    // p
    // set tim1 as PWN output for 24Mhz as camera clock
    let p = unsafe { embassy_stm32::Peripherals::steal() };
    let ch1 = simple_pwm::PwmPin::new_ch1(p.PA8, OutputType::PushPull);
    let mut cam_xclk = simple_pwm::SimplePwm::new(
        p.TIM1,
        Some(ch1),
        None,
        None,
        None,
        khz(24000),
        // simple_pwm::CountingMode::EdgeAlignedUp,
        embassy_stm32::timer::CountingMode::EdgeAlignedUp,
    );
    let max = cam_xclk.get_max_duty();
    cam_xclk.set_duty(embassy_stm32::timer::Channel::Ch1, max / 2);
    TIM1.cr1().modify(|w| {
        w.set_arpe(Arpe::ENABLED);
    });
    cam_xclk.enable(embassy_stm32::timer::Channel::Ch1);
}

#[path = "../camera.rs"]
mod camera;

fn setup_camera_dcmi() -> dcmi::DcmiPort {
    setup_cam_clk();
    CAM_PDWN.set_low();
    clock::delay_ms(10);
    camera::setup_camera(CAM_I2C);
    let dcmi = dcmi::DCMI;
    dcmi.init(
        DCMI_D0_PC6,
        DCMI_D1_PC7,
        DCMI_D2_PC8,
        DCMI_D3_PC9,
        DCMI_D4_PC11,
        DCMI_D5_PB6,
        DCMI_D6_PB8,
        DCMI_D7_PB9,
        DCMI_HSYNC_PA4,
        DCMI_VSYNC_PB7,
        DCMI_PIXCLK_PA6,
    );
    // CAM_PDWN.set_high();
    dcmi
}
fn init_sd() -> SdInstance {
    let mut sd = SdInstance::new(stm32_metapac::SDMMC2);
    sd.init(SDMMC2_CK_PC1, SDMMC2_D0_PB14, SDMMC2_CMD_PA0);
    return sd;
}
// use 4 byte in first block to store the number of image files
const IMG_START_BLOCK: u32 = 10;
const IMG_SIZE: u32 = 2000; // 2000 block = 2000 * 512 = 1M
const SIZE_BLOCK: u32 = 1; // first block store the number of image files

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    setup();
    rtc::enable_rtc_read();
    spawner.spawn(usb_task()).unwrap();
    let mut pic_num = 10;
    loop {
        // setup();
        let dcmi = setup_camera_dcmi();
        let sd = init_sd();
        clock::delay_ms(10);
        const PIC_BUF_SIZE: usize = 512 * 1000;
        let mut pic_buf = [0u8; PIC_BUF_SIZE];

        loop {
            if SIGNAL.signaled() {
                SIGNAL_SD_INST.signal(sd);
                let mut val = SIGNAL.wait().await;
                while val != 0 {
                    defmt::info!("usb connected, stop take picture");
                    val = SIGNAL.wait().await;
                }
            }
            defmt::info!("start take picture");
            pic_num += 1;
            clock::set_clock_to_pll(); // fast clock for camera
            CAM_PDWN.set_low();
            clock::delay_ms(2);
            // dcmi.capture(dma::DCMI_DMA, &pic_buf);
            // while !dcmi.get_picture() {}
            // dcmi.stop_capture(dma::DCMI_DMA);
            defmt::info!("finish take picture");
            CAM_PDWN.set_high();
            LED_BLUE.toggle();

            let mut found = false;
            let mut pic_end = 0;
            for i in 0..PIC_BUF_SIZE - 1 {
                if pic_buf[i] == 0xff && pic_buf[i + 1] == 0xd9 {
                    found = true;
                    pic_end = i;
                    break;
                }
            }
            found = true;
            pic_end = 6000;
            if !found {
                defmt::error!("not find jpeg end");
                continue; // not found the end of jpeg, continue to capture the next picture
            }

            // first 10 bytes are used to store the size of the picture(4 bytes)
            // and the timestamp(6 bytes) yymmddhhmmss
            let date = rtc::get_date();
            let time = rtc::get_time();
            pic_buf[0] = (pic_end >> 24) as u8;
            pic_buf[1] = ((pic_end >> 16) & 0xff) as u8;
            pic_buf[2] = ((pic_end >> 8) & 0xff) as u8;
            pic_buf[3] = (pic_end & 0xff) as u8;

            pic_buf[4] = date.0;
            pic_buf[5] = date.1;
            pic_buf[6] = date.2;
            pic_buf[7] = time.0;
            pic_buf[8] = time.1;
            pic_buf[9] = time.2;
            // clock::set_clock_to_hsi(); // slow clock for sd card
            let block_count: u32 = ((pic_end + 512 - 1) / 512) as u32;
            let end: usize = block_count as usize * 512;
            defmt::info!("start write picture to sd card, block_count: {}", block_count);
             sd.write_multiple_blocks_async(&pic_buf[0..end], pic_num * IMG_SIZE, block_count).await.unwrap();
             defmt::info!("finish write picture to sd card");
            // update first block
            let mut buf = [0u8; 512];
            // sd.read_single_block(&mut buf, SIZE_BLOCK).unwrap();
            sd.read_single_block_async(&mut buf, SIZE_BLOCK).await.unwrap();

            let mut num = ((buf[0] as u32) << 24)
                | ((buf[1] as u32) << 16)
                | ((buf[2] as u32) << 8)
                | (buf[3] as u32);
            num += 1;
            buf[0] = (num >> 24) as u8;
            buf[1] = ((num >> 16) & 0xff) as u8;
            buf[2] = ((num >> 8) & 0xff) as u8;
            buf[3] = (num & 0xff) as u8;

            // sd.write_single_block(&buf, SIZE_BLOCK).unwrap();
            sd.write_single_block_async(&buf, SIZE_BLOCK).await.unwrap();

            let mut file_name = String::<32>::new();
            file_name
                .write_fmt(format_args!("{}.jpg", pic_num))
                .unwrap(); // This shout not have error
            defmt::info!("finish take picture file name {}", file_name.as_str());
            LED_GREEN.toggle();
            // Timer::after(Duration::from_millis(1000)).await;
        }
    }
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
            SIGNAL.signal(1);
            defmt::info!("connected");
            let _ = usb_handler(&mut class).await;
            defmt::info!("disconnected");
            SIGNAL.signal(0);
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
                                       // let sd = SdInstance::new(stm32_metapac::SDMMC2);
    // let sd = init_sd();
    // get sd instance from main task
    let sd = SIGNAL_SD_INST.wait().await;

    // let sd = SdInstance::new(stm32_metapac::SDMMC2);
    // let sd 

    loop {
        let n = class.read_packet(&mut buf).await.unwrap();
        let command = ebcmd::Command::from_array(&buf[..n]);
        match command {
            ebcmd::Command::SetTim(year, month, day, hour, minute, second) => {
                rtc::setup(year, month, day, hour, minute, second);
                let res = ebcmd::Response::SetTim(0);
                let (buf, len) = res.to_array();
                class.write_packet(&buf[0..len]).await.unwrap();
            }
            ebcmd::Command::GetTim => {
                let date = rtc::get_date();
                let time = rtc::get_time();
                let res = ebcmd::Response::GetTim(date.0, date.1, date.2, time.0, time.1, time.2);
                let (buf, len) = res.to_array();
                class.write_packet(&buf[0..len]).await.unwrap();
            }
            ebcmd::Command::GetPic(id) => {
                let mut buf = [0; 64];
                buf[0] = 0x02;
                let _pic_buf = [0; 256];
                // get data from sd storage and put it into buf
                let start_block = id * IMG_SIZE;
                // get the size of the picture
                sd.read_single_block(&mut buf, start_block).unwrap();
                // get the end of picture
                let pic_end = ((buf[0] as u32) << 24)
                    | ((buf[1] as u32) << 16)
                    | ((buf[2] as u32) << 8)
                    | (buf[3] as u32);
                let block_count: u32 = ((pic_end + 512 - 1) / 512) as u32;
                let mut ordinal = 0;
                let mut send_len = 0;
                let mut res = ebcmd::Response::GetTim(0, 0, 0, 0, 0, 0);
                let mut start = 10;
                loop {
                    (ordinal, send_len, res) =
                        ebcmd::Response::pic_res_from_data(id, ordinal, &buf[start..]);
                    if send_len == 0 {
                        break;
                    }
                    start += send_len;
                    let (buf, len) = res.to_array();
                    class.write_packet(&buf[0..len]).await.unwrap();
                }
                // let tmp = ebcmd::Response::pic_res_from_data(id, 0, &buf[10..]);
                // let end: usize = block_count as usize * 512;
                // send current block
                for block in 1..block_count {
                    sd.read_single_block(&mut buf, start_block + block).unwrap();
                    start = 0;
                    loop {
                        (ordinal, send_len, res) =
                            ebcmd::Response::pic_res_from_data(id, ordinal, &buf[start..]);
                        if send_len == 0 {
                            break;
                        }
                        start += send_len;
                        let (buf, len) = res.to_array();
                        class.write_packet(&buf[0..len]).await.unwrap();
                    }
                }
            }
            ebcmd::Command::GetPicNum => {
                let mut buf = [0; 64];
                buf[0] = 0x04;
                let mut buf = [0u8; 512];
                sd.read_single_block(&mut buf, SIZE_BLOCK).unwrap();
                let num = ((buf[0] as u32) << 24)
                    | ((buf[1] as u32) << 16)
                    | ((buf[2] as u32) << 8)
                    | (buf[3] as u32);
                // ebcmd::Response::GetPicNum(num)
                let res = ebcmd::Response::GetPicNum(0);
                let (buf, len) = res.to_array();
                class.write_packet(&buf[0..len]).await.unwrap();
            }
        };
    }
}
