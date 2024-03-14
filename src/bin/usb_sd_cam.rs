#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]
// #![allow(unused_imports)]
// use embassy_time::{Duration, Timer};

use defmt_rtt as _;
use embassy_executor::Spawner;
use u5_lib::{*};

use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, signal::Signal};

#[path = "../camera.rs"]
mod camera;

use stm32_metapac::{RCC};
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

static DEEP_SEMAPHORE: u32 = 0; // because only 1 cpu in this board. This will works. When this value is 0, then the mcu can go deep sleep.
fn sleep_mode() {
    unsafe {
        let p = cortex_m::Peripherals::steal();
        let mut scb = p.SCB;

        if DEEP_SEMAPHORE == 0 {
            scb.set_sleepdeep();
        } else {
            scb.clear_sleepdeep();
        }
    }
}

fn setup() {
    clock::init_clock();
    CAM_PDWN.setup();
    // CAM_PDWN.set_high();
    LED_ORANGE.setup();
    // LED_ORANGE.set_high();
    LED_GREEN.setup();
    // LED_GREEN.set_high();
    LED_BLUE.setup();
    // LED_BLUE.set_high();
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

    gi2c::I2C3.write(ov5640_reg::OV5640_I2C_ADDR, &reg_val).unwrap();
    dcmi
}

// fn init_sd() -> sdmmc::SdInstance {
//     let mut sd = sdmmc::SdInstance::new(stm32_metapac::SDMMC2);
//
//     // u5_lib::clock::run_with_160mhz(|| {
//     sd.init(
//         gpio::SDMMC2_CK_PC1,
//         gpio::SDMMC2_D0_PB14,
//         gpio::SDMMC2_CMD_PA0,
//     );
//     // });
//     return sd;
// }
// #[path = "../usb_util.rs"]
// mod usb_util;
// use usb_util::usb_task;

// #[cortex_m_rt::entry]
// fn main() -> ! {
//     Executor::take().run(|spawner| {
//         // defmt::unwrap!(
//         spawner.spawn(async_main(spawner)).unwrap();
//         // );
//     });
// }

#[embassy_executor::main]
// #[embassy_executor::task]
async fn async_main(spawner: Spawner) {
    // async fn main(spawner: Spawner) {

    setup();
    LED_GREEN.set_high();
    LED_ORANGE.set_high();
    LED_BLUE.set_high();

    // let sd = init_sd();
    LED_ORANGE.set_low();
    // share sd with usb task

    // let _dcmi = setup_camera_dcmi();

    clock::delay_ms(10);

    // spawner.spawn(usb_task(sd)).unwrap();
    spawner.spawn(btn()).unwrap();
    // CAM_PDWN.set_high();

    LED_GREEN.set_low();
    // PWR.cr1().modify(|v| v.set_lpms(pwr::vals::Lpms::STOP3));
    loop {
        clock::delay_ms(10);
        const PIC_BUF_SIZE: usize = 512 * 1300; // 512byte * 1300 = 650K
        // let _pic_buf = [0u8; PIC_BUF_SIZE];
        loop {
            if unsafe { TAKE_PIC } == false {
                rtc::rtc_interrupt().await;
                continue;
            }
            LED_BLUE.set_high();
            // camera::capture(&CAM_PDWN, &dcmi, &mut pic_buf, &sd).await;
            LED_BLUE.set_low();
            rtc::rtc_interrupt().await;
        }
    }
}
static mut TAKE_PIC: bool = false;
#[embassy_executor::task]
async fn btn() {
    let mut last_time: (u8, u8, u8) = (0, 0, 0);
    loop {
        exti::EXTI2_PB2.wait_for_raising().await;
        let cur_time = u5_lib::rtc::get_time();
        //  if cur_time + 1 second == last_time, then click twice
        let mut click_twice = false;
        defmt::info!("last_time: {:?}", last_time);
        defmt::info!("cur_time: {:?}", cur_time);

        if last_time == cur_time {
            click_twice = true;
        }
        last_time.2 += 1;
        if last_time.2 >= 60 {
            last_time.2 = 0;
            last_time.1 += 1;
            if last_time.1 >= 60 {
                last_time.1 = 0;
                last_time.0 += 1;
                if last_time.0 >= 24 {
                    last_time.0 = 0;
                }
            }
        }
        if last_time == cur_time {
            click_twice = true;
        }
        last_time = cur_time;
        if click_twice == false {
            unsafe {
                if TAKE_PIC {
                    TAKE_PIC = false;
                    LED_ORANGE.set_low();
                } else {
                    TAKE_PIC = true;
                    LED_ORANGE.set_high();
                }
            }
            defmt::info!("click once");
        } else {
            // enter no deep sleep mode
            // low_power::sleep_no_deep_request();
            clock::kernel_freq_160mhz_request();
            defmt::info!("click twice");
            LED_BLUE.toggle();
        }
    }
}

