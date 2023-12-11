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

const PIC_BUF_SIZE: usize = 600_000;
// const PIC_BUF_SIZE: usize = 0xffff;
// const PIC_BUF_SIZE: usize = 140_000;
static mut PIC_BUF: [u8; PIC_BUF_SIZE] = [0; PIC_BUF_SIZE];
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


// use function from camera.rs
#[path = "./camera.rs"]
mod camera;


#[embassy_executor::main]
async fn main(spawner: Spawner) {
    setup();
    defmt::info!("init clock finished");
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
    unsafe {
        // get buffert address add print it
        defmt::info!("PIC_BUF address = 0x{:x}", PIC_BUF.as_ptr() as u32);
        dcmi.capture(dma::DCMI_DMA, &mut PIC_BUF);
        clock::delay_ms(2000);
        dcmi.stop_capture(dma::DCMI_DMA);
        // print first 10 bytes in PIC_BUF in hex
        defmt::info!("PIC_BUF[0..10] in hex =  0x{:x}", &PIC_BUF[0..10]);
        defmt::info!("dcmi status register = 0x{:x}", DCMI.sr().read().0);
        defmt::info!(
            "dcmi raw interrupt status register = 0x{:x}",
            DCMI.ris().read().0
        );
        defmt::info!("PIC_BUF[0..10] in hex =  0x{:x}", &PIC_BUF[0..10]);
        // try to find ff d9 for jpeg end
        let mut found = false;
        for i in 0..PIC_BUF_SIZE - 1 {
            if PIC_BUF[i] == 0xff && PIC_BUF[i + 1] == 0xd9 {
                found = true;
                defmt::info!("find jpeg end at index {}", i);
            }
        }
        if !found {
            defmt::info!("not find jpeg end");
        }
    }

    // GPDMA1.ch(0).tr1().modify(|w| w.set_dap(ChTr1Ap::PORT1));
    loop {
        Timer::after(Duration::from_secs(1)).await;
        LED_GREEN.toggle();
    }
}
