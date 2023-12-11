#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]
#![allow(dead_code)]
#![allow(unused_imports)]

use core::fmt::{write, Write};
use core::{future::Future, hash::Hasher};
use cortex_m::{asm, prelude::_embedded_hal_blocking_spi_Write};
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
use futures::future::join;
use heapless::String;

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
struct FtSource {}

use embedded_sdmmc::{BlockDevice, Directory, Error, File, TimeSource, VolumeManager};
use embedded_sdmmc::{Timestamp, Volume};
use u5_lib::clock::delay_ms;
use u5_lib::gpio::{SDMMC2_CK_PC1, SDMMC2_CMD_PA0, SDMMC2_D0_PB14};

impl TimeSource for FtSource {
    fn get_timestamp(&self) -> Timestamp {
        Timestamp {
            year_since_1970: 0,
            zero_indexed_month: 0,
            zero_indexed_day: 0,
            hours: 0,
            minutes: 0,
            seconds: 0,
        }
    }
}
use u5_lib::*;
use u5_lib::{
    gpio::{
        DCMI_D0_PC6, DCMI_D1_PC7, DCMI_D2_PC8, DCMI_D3_PC9, DCMI_D4_PC11, DCMI_D5_PB6, DCMI_D6_PB8,
        DCMI_D7_PB9, DCMI_HSYNC_PA4, DCMI_PIXCLK_PA6, DCMI_VSYNC_PB7,
    },
    ov5640_reg,
};

use ov5640_reg::*;

use embassy_stm32::timer::simple_pwm;
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::signal::Signal;
use embassy_time::{Duration, Timer};
use stm32_metapac::gpdma::vals::ChTr1Ap;
use stm32_metapac::gpio::vals::Ospeedr;
use stm32_metapac::timer::vals::Arpe;
use stm32_metapac::{DCMI, GPDMA1, GPIOB, GPIOC, I2C3, PWR, RCC, TIM1};
use u5_lib::sdmmc::{SdError, SdInstance};

static SIGNAL: Signal<CriticalSectionRawMutex, u32> = Signal::new();
static SIGNAL2: Signal<CriticalSectionRawMutex, u32> = Signal::new();

const LED_GREEN: gpio::GpioPort = gpio::PC3;
const LED_ORANGE: gpio::GpioPort = gpio::PC4;
const LED_BLUE: gpio::GpioPort = gpio::PC5;
const CAM_I2C: gi2c::I2cPort = gi2c::I2C3;
const CAM_PDWN: gpio::GpioPort = gpio::PB0;

bind_interrupts!(struct Irqs {
    OTG_FS => usb_otg::InterruptHandler<peripherals::USB_OTG_FS>;
    // GPDMA1_CH0 => embassy_stm32::dma::<peripherals::GPDMA1_CH0>;
});

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

fn setup_cam_clk() {
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
    dcmi
}
fn init_sd() -> SdInstance {
    // -> (VolumeManager<SdInstance, FtSource>, Directory) {
    let mut sd = SdInstance::new(stm32_metapac::SDMMC2);
    let ts = FtSource {};
    sd.init(SDMMC2_CK_PC1, SDMMC2_D0_PB14, SDMMC2_CMD_PA0);
    // let mut volume_mgr = embedded_sdmmc::VolumeManager::new(sd, ts);
    // let volume0 = volume_mgr
    //     .open_volume(embedded_sdmmc::VolumeIdx(0))
    //     .unwrap();
    // let root_dir = volume_mgr.open_root_dir(volume0).unwrap();
    // let dir = volume_mgr.open_dir(root_dir, "hello").unwrap();
    // return (volume_mgr, root_dir);
    return sd;
}

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let mut pic_num = 10;
    const multipiler: u32 = 1000;
    loop {
        setup();
        let dcmi = setup_camera_dcmi();
        // let (mut volume_mgr, root_dir) = init_sd();
        let sd = init_sd();

        // update
        delay_ms(10);
        const PIC_BUF_SIZE: usize = 600_000;
        let pic_buf: [u8; PIC_BUF_SIZE] = [0; PIC_BUF_SIZE];
        loop {
            pic_num += 1;
            clock::set_clock_to_pll(); // fast clock for camera
            CAM_PDWN.set_low();
            delay_ms(2);
            dcmi.capture(dma::DCMI_DMA, &pic_buf);
            while !dcmi.get_picture() {}
            dcmi.stop_capture(dma::DCMI_DMA);
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
            if !found {
                defmt::error!("not find jpeg end");
                continue; // not found the end of jpeg, continue to capture the next picture
            }
            clock::set_clock_to_hsi(); // slow clock for sd card
            let block_count: u32 = ((pic_end + 512 - 1) / 512) as u32;
            let end: usize = block_count as usize * 512;
            sd.write_multiple_blocks(&pic_buf[0..end], pic_num * multipiler, block_count)
                .unwrap();

            // let mut file = volume_mgr.open_file_in_dir(root_dir, "4.jpg", embedded_sdmmc::Mode::ReadWriteCreate).unwrap();

            let mut file_name = String::<32>::new();
            file_name
                .write_fmt(format_args!("{}.jpg", pic_num))
                .unwrap(); // This shout not have error
                           // let file = match volume_mgr.open_file_in_dir(
                           //     root_dir,
                           //     file_name.as_str(),
                           //     embedded_sdmmc::Mode::ReadWriteCreateOrTruncate,
                           // ) {
                           //     Ok(f) => f,
                           //     Err(err) => {
                           //         defmt::error!("open file failed {:?}", err);
                           //         break;
                           //     }
                           // };
                           // // write buf to file
                           // match volume_mgr.write(file, &pic_buf[0..pic_end]) {
                           //     Ok(_) => {}
                           //     Err(err) => {
                           //         defmt::error!("write file failed {:?}", err);
                           //         break;
                           //     }
                           // }
                           // // close file
                           // match volume_mgr.close_file(file) {
                           //     Ok(_) => {}
                           //     Err(err) => {
                           //         defmt::error!("close file failed {:?}", err);
                           //         break;
                           //     }
                           // }
            defmt::info!("finish take picture file name {}", file_name.as_str());
            LED_GREEN.toggle();
        }
    }
}
