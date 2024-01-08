#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]
// #![allow(dead_code)]
// #![allow(unused_imports)]
use embassy_time::{Duration, Timer};

use defmt_rtt as _;
use embassy_executor::Spawner;

use u5_lib::*;

use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, signal::Signal};

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
static SIGNAL: Signal<CriticalSectionRawMutex, u32> = Signal::new();
static SIGNAL_SD_INST: Signal<CriticalSectionRawMutex, sdmmc::SdInstance> = Signal::new();

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
}

fn setup_cam_clk() {
    let cam_clk: gpio::GpioPort = gpio::GPIO_MCO_PA8;
    cam_clk.setup();
    // set PA8 as mco output for HSI48 and divide by 2 (24Mhz)
    RCC.cfgr1().modify(|w| {
        w.set_mcosel(stm32_metapac::rcc::vals::Mcosel::HSI48);
        w.set_mcopre(stm32_metapac::rcc::vals::Mcopre::DIV2);
    });
}

fn setup_camera_dcmi() -> dcmi::DcmiPort {
    setup_cam_clk();
    CAM_PDWN.set_low();
    clock::delay_ms(10);
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
    clock::delay_ms(1000); // avoid the green picture
    // CAM_PDWN.set_high();
    dcmi
}

fn init_sd() -> sdmmc::SdInstance {
    let mut sd = sdmmc::SdInstance::new(stm32_metapac::SDMMC2);
    sd.init(gpio::SDMMC2_CK_PC1, gpio::SDMMC2_D0_PB14, gpio::SDMMC2_CMD_PA0);
    return sd;
}
#[path ="../usb_util.rs"]
mod usb_util;
use usb_util::usb_task;

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    setup();
    rtc::enable_rtc_read();
    let sd = init_sd();
    // share sd with usb task
    let dcmi = setup_camera_dcmi();
    // CAM_PDWN.set_high();
    spawner.spawn(usb_task()).unwrap();

    loop {
        Timer::after(Duration::from_millis(500)).await;
        if SIGNAL.signaled() {
            SIGNAL_SD_INST.signal(sd);
            CAM_PDWN.set_high();
            let mut val = SIGNAL.wait().await;
            while val != 0 {
                defmt::info!("usb connected, stop take picture");
                val = SIGNAL.wait().await;
            }
        }
        clock::delay_ms(10);
        const PIC_BUF_SIZE: usize = 512 * 1300; // 512byte * 1300 = 650K
        let mut pic_buf = [0u8; PIC_BUF_SIZE];
        loop {
            camera::capture(&CAM_PDWN, &dcmi, &mut pic_buf, &sd).await;
        }
    }
}
