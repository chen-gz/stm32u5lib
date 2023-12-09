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
// use embassy_usb::{
//     class::cdc_acm::{CdcAcmClass, State},
//     driver::EndpointError,
//     Builder, UsbDevice,
// };
use futures::future::join;
use heapless::String;

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    defmt::info!("panic");
    loop {}
}

bind_interrupts!(struct Irqs {
    DCMI_PSSI => embassy_stm32::dcmi::InterruptHandler<peripherals::DCMI>;
    OTG_FS => usb_otg::InterruptHandler<peripherals::USB_OTG_FS>;
    // GPDMA1_CH0 => embassy_stm32::dma::<peripherals::GPDMA1_CH0>;
});
// #[path = "../ov5640_reg.rs"]
// mod ov5640_reg;
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

// const PIC_BUF_SIZE: usize = 0xffff;
// const PIC_BUF_SIZE: usize = 140_000;
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

fn setup_dcmi() {
    let p = unsafe { embassy_stm32::Peripherals::steal() };
    let mut dcmi_config = embassy_stm32::dcmi::Config::default();
    dcmi_config.vsync_level = embassy_stm32::dcmi::VSyncDataInvalidLevel::High;
    dcmi_config.hsync_level = embassy_stm32::dcmi::HSyncDataInvalidLevel::High;
    dcmi_config.pixclk_polarity = embassy_stm32::dcmi::PixelClockPolarity::RisingEdge;

    let mut dcmi = embassy_stm32::dcmi::Dcmi::new_8bit(
        p.DCMI,
        p.GPDMA1_CH0,
        Irqs,
        p.PC6,
        p.PC7,
        p.PC8,
        p.PC9,
        p.PC11,
        p.PB6,
        p.PB8,
        p.PB9,
        p.PB7,
        p.PA4,
        p.PA6,
        dcmi_config,
    );
    DCMI.cr().modify(|w| {
        w.set_jpeg(true);
    });
}

fn setup_camera() {
    let mut read_val: [u8; 2] = [0u8; 2];
    CAM_I2C
        .write_read(
            OV5640_I2C_ADDR,
            &[
                (OV5640_CHIP_ID_HIGH_BYTE >> 8) as u8,
                OV5640_CHIP_ID_HIGH_BYTE as u8,
            ],
            &mut read_val[0..1],
        )
        .unwrap();
    CAM_I2C
        .write_read(
            OV5640_I2C_ADDR,
            &[
                (OV5640_CHIP_ID_LOW_BYTE >> 8) as u8,
                OV5640_CHIP_ID_LOW_BYTE as u8,
            ],
            &mut read_val[1..2],
        )
        .unwrap();
    assert!(read_val[0] == 0x56 && read_val[1] == 0x40);

    // read PIDH
    let mut read_val = [0u8; 1];

    defmt::info!("writing ov5640 common regs");
    for &(reg, val) in ov5640_reg::OV5640_Common.iter() {
        let mut reg_val = [0u8; 3];
        reg_val[0] = (reg >> 8) as u8;
        reg_val[1] = reg as u8;
        reg_val[2] = val as u8;
        match CAM_I2C.write(ov5640_reg::OV5640_I2C_ADDR, &reg_val) {
            Ok(_) => {}
            Err(_) => {
                defmt::panic!("failed when writing ov5640 common regs");
            }
        }
    }

    defmt::info!("writing ov5640 jpeg regs");
    for &(reg, val) in OV5640_PF_JPEG.iter() {
        let mut reg_val = [0u8; 3];
        reg_val[0] = (reg >> 8) as u8;
        reg_val[1] = reg as u8;
        reg_val[2] = val as u8;
        match CAM_I2C.write(ov5640_reg::OV5640_I2C_ADDR, &reg_val) {
            Ok(_) => {}
            Err(_) => {
                defmt::panic!("failed when writing ov5640 jpeg regs");
            }
        }
    }
    for &(reg, val) in OV5640_JPEG_MODE.iter() {
        let mut reg_val = [0u8; 3];
        reg_val[0] = (reg >> 8) as u8;
        reg_val[1] = reg as u8;
        reg_val[2] = val as u8;
        match CAM_I2C.write(ov5640_reg::OV5640_I2C_ADDR, &reg_val) {
            Ok(_) => {}
            Err(_) => {
                defmt::panic!("failed when writing ov5640 jpeg mode");
            }
        }
    }
    defmt::info!("writing ov5640 jpeg regs finished");

    let mut read_val = [0u8; 1];
    let mut reg_addr = [0u8; 2];
    // OV5640_TIMING_TC_REG21
    reg_addr[0] = (ov5640_reg::OV5640_TIMING_TC_REG21 >> 8) as u8;
    reg_addr[1] = ov5640_reg::OV5640_TIMING_TC_REG21 as u8;
    CAM_I2C
        .write_read(ov5640_reg::OV5640_I2C_ADDR, &reg_addr, &mut read_val)
        .unwrap();
    let mut write_val = [0u8; 3];
    write_val[0] = reg_addr[0];
    write_val[1] = reg_addr[1];
    write_val[2] = read_val[0] | (1 << 5);
    CAM_I2C
        .write(ov5640_reg::OV5640_I2C_ADDR, &write_val)
        .unwrap();

    // SYSREM_RESET02
    reg_addr[0] = (ov5640_reg::OV5640_SYSREM_RESET02 >> 8) as u8;
    reg_addr[1] = ov5640_reg::OV5640_SYSREM_RESET02 as u8;
    CAM_I2C
        .write_read(ov5640_reg::OV5640_I2C_ADDR, &reg_addr, &mut read_val)
        .unwrap();
    let mut write_val = [0u8; 3];
    write_val[0] = reg_addr[0];
    write_val[1] = reg_addr[1];
    write_val[2] = read_val[0] & !(1 << 2 | 1 << 3 | 1 << 4);
    CAM_I2C
        .write(ov5640_reg::OV5640_I2C_ADDR, &write_val)
        .unwrap();

    // OV5640_CLOCK_ENABLE02
    reg_addr[0] = (ov5640_reg::OV5640_CLOCK_ENABLE02 >> 8) as u8;
    reg_addr[1] = ov5640_reg::OV5640_CLOCK_ENABLE02 as u8;
    CAM_I2C
        .write_read(ov5640_reg::OV5640_I2C_ADDR, &reg_addr, &mut read_val)
        .unwrap();
    let mut write_val = [0u8; 3];
    write_val[0] = reg_addr[0];
    write_val[1] = reg_addr[1];
    write_val[2] = read_val[0] | (1 << 3 | 1 << 5);
    CAM_I2C
        .write(ov5640_reg::OV5640_I2C_ADDR, &write_val)
        .unwrap();
    defmt::info!("setup camera registers finished");
}

struct ftsource {}

use embedded_sdmmc::{Error, File, TimeSource, BlockDevice};
use embedded_sdmmc::{Timestamp, Volume};
use u5_lib::clock::delay_ms;
use u5_lib::gpio::{SDMMC2_CK_PC1, SDMMC2_CMD_PA0, SDMMC2_D0_PB14};

impl TimeSource for ftsource {
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
fn setup_camera_dcmi() -> dcmi::DcmiPort {
    setup_cam_clk();
    CAM_PDWN.set_low();
    clock::delay_ms(10);
    setup_camera();
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

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    setup();
    let dcmi = setup_camera_dcmi();

    // update
    let mut sd = SdInstance::new(stm32_metapac::SDMMC2);
    let ts = ftsource {};
    sd.init(SDMMC2_CK_PC1, SDMMC2_D0_PB14, SDMMC2_CMD_PA0);
    let mut volume_mgr = embedded_sdmmc::VolumeManager::new(sd, ts);
    let volume0 = volume_mgr
        .open_volume(embedded_sdmmc::VolumeIdx(0))
        .unwrap();
    defmt::info!("Volume 0: {:?}", volume0);
    let mut root_dir = volume_mgr.open_root_dir(volume0).unwrap();
    defmt::info!("open root dir");



    delay_ms(10);
    const PIC_BUF_SIZE: usize = 600_000;
    let mut PIC_BUF: [u8; PIC_BUF_SIZE] = [0; PIC_BUF_SIZE];
    let mut pic_num = 0;
    // for pic_num in 0..1000 {
    loop {
        pic_num += 1;

        clock::set_clock_to_pll(); // fast clock for camera
        CAM_PDWN.set_low();
        delay_ms(2);
        dcmi.capture(dma::DCMI_DMA, &mut PIC_BUF);
        while !dcmi.get_picture() {}
        dcmi.stop_capture(dma::DCMI_DMA);
        CAM_PDWN.set_high();
        LED_BLUE.toggle();

        let mut found = false;
        let mut pic_end = 0;
        for i in 0..PIC_BUF_SIZE - 1 {
            if PIC_BUF[i] == 0xff && PIC_BUF[i + 1] == 0xd9 {
                found = true;
                pic_end = i;
                defmt::info!("find jpeg end at index {}", i);
                break;
            }
        }
        if !found {
            defmt::error!("not find jpeg end");
            continue; // not found the end of jpeg, continue to capture the next picture
        }
        clock::set_clock_to_hsi(); // slow clock for sd card

        // let mut file = volume_mgr.open_file_in_dir(root_dir, "4.jpg", embedded_sdmmc::Mode::ReadWriteCreate).unwrap();

        let mut file_name = String::<32>::new();
        file_name
            .write_fmt(format_args!("{}.jpg", pic_num))
            .unwrap();
        let file = match volume_mgr.open_file_in_dir(
            root_dir,
            file_name.as_str(),
            embedded_sdmmc::Mode::ReadWriteCreateOrTruncate,
        ) {
            Ok(f) => {
                defmt::info!("open file success");
                f
            }
            Err(err) => {
                defmt::panic!("open file failed {:?}", err)
            }
        };

        defmt::info!("open file success");
        // write buf to file
        match volume_mgr.write(file, &PIC_BUF[0..pic_end]) {
            Ok(_) => {
                defmt::info!("write file success");
            }
            Err(err) => {
                defmt::panic!("write file failed {:?}", err)
            }
        }
        defmt::info!("write file");
        // close file
        let _ = volume_mgr.close_file(file).unwrap();
        defmt::info!("close file");
        LED_GREEN.toggle();
        // Timer::after(Duration::from_secs(5)).await;
    }
    // GPDMA1.ch(0).tr1().modify(|w| w.set_dap(ChTr1Ap::PORT1));
    loop {
        Timer::after(Duration::from_secs(1)).await;
    }
}
