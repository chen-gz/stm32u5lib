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
    I2C3_ER => embassy_stm32::i2c::ErrorInterruptHandler<peripherals::I2C3>;
    I2C3_EV => embassy_stm32::i2c::EventInterruptHandler<peripherals::I2C3>;
    // GPDMA1_CH0 => embassy_stm32::dma::<peripherals::GPDMA1_CH0>;
});
#[path = "../ov5640_reg.rs"]
mod ov5640_reg;

use ov5640_reg::*;

#[path = "../gi2c.rs"]
mod gi2c;
#[path = "../gpio.rs"]
mod gpio;

use crate::ov5640_reg::{OV5640_Common, OV5640_JFIFO_OVERFLOW, OV5640_JPEG_MODE, OV5640_PF_JPEG};
use embassy_stm32::timer::simple_pwm;
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::signal::Signal;
use embassy_time::{Duration, Timer};
use stm32_metapac::gpdma::vals::ChTr1Ap;
use stm32_metapac::gpio::vals::Ospeedr;
use stm32_metapac::timer::vals::Arpe;
use stm32_metapac::{DCMI, GPDMA1, GPIOB, GPIOC, I2C3, PWR, RCC, TIM1};

// const PIC_BUF_SIZE: usize = 130_000;
const PIC_BUF_SIZE: usize = 0xffff;
// const PIC_BUF_SIZE: usize = 140_000;
static mut PIC_BUF: [u32; PIC_BUF_SIZE] = [0; PIC_BUF_SIZE];
static SIGNAL: Signal<CriticalSectionRawMutex, u32> = Signal::new();
static SIGNAL2: Signal<CriticalSectionRawMutex, u32> = Signal::new();
#[path = "../clock.rs"]
mod clock;
#[embassy_executor::main]
async fn main(spawner: Spawner) {
    // let mut config = Config::default();
    // let p = embassy_stm32::init(config);
    clock::gg::init_clock();
    let p = unsafe { embassy_stm32::Peripherals::steal() };

    let led_orange = gpio::gg::PC4;
    led_orange.setup();

    defmt::info!("start init");

    // start usb task
    // set tim1 as PWN output for 24Mhz as camera clock
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
    // spawner.spawn(usb_task()).unwrap();

    let res = spawner.spawn(usb_task());
    match res {
        Ok(_) => {}
        Err(err) => {
            defmt::info!("spawn usb task failed");
        }
    }

    cam_xclk.set_duty(embassy_stm32::timer::Channel::Ch1, max / 2);
    TIM1.cr1().modify(|w| {
        w.set_arpe(Arpe::ENABLED);
    });
    cam_xclk.enable(embassy_stm32::timer::Channel::Ch1);

    let cam_i2c = gi2c::gg::I2C3;
    cam_i2c.init(100_000, gpio::gg::I2C3_SCL_PC0, gpio::gg::I2C3_SDA_PB4);

    let mut read_val: [u8; 2] = [0u8; 2];
    cam_i2c
        .write_read(
            OV5640_I2C_ADDR as u16,
            &[
                (OV5640_CHIP_ID_HIGH_BYTE >> 8) as u8,
                OV5640_CHIP_ID_HIGH_BYTE as u8,
            ],
            &mut read_val[0..1],
        )
        .unwrap();
    cam_i2c
        .write_read(
            OV5640_I2C_ADDR as u16,
            &[
                (OV5640_CHIP_ID_LOW_BYTE >> 8) as u8,
                OV5640_CHIP_ID_LOW_BYTE as u8,
            ],
            &mut read_val[1..2],
        )
        .unwrap();
    defmt::info!("read chipid success (hex): 0x{:x}", read_val);

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
    let led_green = gpio::gg::PC3;
    led_green.setup();

    // read PIDH
    let mut read_val = [0u8; 1];
    cam_i2c
        .write_read(OV5640_I2C_ADDR, &[0x30, 0x0A], &mut read_val)
        .unwrap();
    defmt::info!("PIDH: {}", read_val[0]);

    let mut read_val = [0u8; 1];

    cam_i2c
        .write_read(ov5640_reg::OV5640_I2C_ADDR, &[0x30, 0x0B], &mut read_val)
        .unwrap();

    for &(reg, val) in ov5640_reg::OV5640_Common.iter() {
        let mut reg_val = [0u8; 3];
        reg_val[0] = (reg >> 8) as u8;
        reg_val[1] = reg as u8;
        reg_val[2] = val as u8;
        match cam_i2c.write(ov5640_reg::OV5640_I2C_ADDR, &reg_val) {
            Ok(_) => {}
            Err(_) => {
                defmt::info!("write reg failed")
            }
        }
    }
    for &(reg, val) in OV5640_PF_JPEG.iter() {
        let mut reg_val = [0u8; 3];
        reg_val[0] = (reg >> 8) as u8;
        reg_val[1] = reg as u8;
        reg_val[2] = val as u8;
        match cam_i2c.write(ov5640_reg::OV5640_I2C_ADDR, &reg_val) {
            Ok(_) => {}
            Err(_) => {
                defmt::info!("write reg failed")
            }
        }
    }
    for &(reg, val) in OV5640_JPEG_MODE.iter() {
        let mut reg_val = [0u8; 3];
        reg_val[0] = (reg >> 8) as u8;
        reg_val[1] = reg as u8;
        reg_val[2] = val as u8;
        match cam_i2c.write(ov5640_reg::OV5640_I2C_ADDR, &reg_val) {
            Ok(_) => {}
            Err(_) => {
                defmt::info!("write reg failed")
            }
        }
    }

    let mut read_val = [0u8; 1];
    let mut reg_addr = [0u8; 2];
    // OV5640_TIMING_TC_REG21
    reg_addr[0] = (ov5640_reg::OV5640_TIMING_TC_REG21 >> 8) as u8;
    reg_addr[1] = ov5640_reg::OV5640_TIMING_TC_REG21 as u8;
    cam_i2c.write_read(ov5640_reg::OV5640_I2C_ADDR, &reg_addr, &mut read_val);
    let mut write_val = [0u8; 3];
    write_val[0] = reg_addr[0];
    write_val[1] = reg_addr[1];
    write_val[2] = read_val[0] | (1 << 5);
    cam_i2c.write(ov5640_reg::OV5640_I2C_ADDR, &write_val);

    // SYSREM_RESET02
    reg_addr[0] = (ov5640_reg::OV5640_SYSREM_RESET02 >> 8) as u8;
    reg_addr[1] = ov5640_reg::OV5640_SYSREM_RESET02 as u8;
    cam_i2c.write_read(ov5640_reg::OV5640_I2C_ADDR, &reg_addr, &mut read_val);
    let mut write_val = [0u8; 3];
    write_val[0] = reg_addr[0];
    write_val[1] = reg_addr[1];
    write_val[2] = read_val[0] & !(1 << 2 | 1 << 3 | 1 << 4);
    cam_i2c.write(ov5640_reg::OV5640_I2C_ADDR, &write_val);

    // OV5640_CLOCK_ENABLE02
    reg_addr[0] = (ov5640_reg::OV5640_CLOCK_ENABLE02 >> 8) as u8;
    reg_addr[1] = ov5640_reg::OV5640_CLOCK_ENABLE02 as u8;
    cam_i2c.write_read(ov5640_reg::OV5640_I2C_ADDR, &reg_addr, &mut read_val);
    let mut write_val = [0u8; 3];
    write_val[0] = reg_addr[0];
    write_val[1] = reg_addr[1];
    write_val[2] = read_val[0] | (1 << 3 | 1 << 5);
    cam_i2c.write(ov5640_reg::OV5640_I2C_ADDR, &write_val);

    PWR.svmcr().modify(|w| w.set_io2sv(true));
    // let mut led_red = Output::new(p.PG2, embassy_stm32::gpio::Level::High, embassy_stm32::gpio::Speed::VeryHigh);
    GPDMA1.ch(0).tr1().modify(|w| w.set_dap(ChTr1Ap::PORT1));
    loop {
        Timer::after(Duration::from_secs(1)).await;
        led_green.toggle();
    }

    loop {
        unsafe {
            dcmi.capture(&mut PIC_BUF).await;
        }
        let buf = unsafe { &PIC_BUF };
        if unsafe { (PIC_BUF[0] & 0xffff) == 0xd8ff } {
        } else {
            continue;
        }
        // try to find oxff 0xd9
        let mut pix_size = 0;
        for i in 0..PIC_BUF_SIZE {
            if unsafe { PIC_BUF[i] & 0xffff == 0xd9ff || (PIC_BUF[i] >> 16) & 0xffff == 0xd9ff } {
                // led_red.toggle();
                led_green.toggle();
                pix_size = i;
                break;
            }
        }
        SIGNAL.signal(pix_size as u32);
        SIGNAL2.wait().await;
        Timer::after(Duration::from_secs(3)).await;
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
