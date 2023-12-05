#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]
// #![allow(dead_code)]
// #![allow(unused_imports)]

use defmt_rtt as _;
use embassy_executor::Spawner;

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    defmt::info!("panic");
    loop {}
}

use u5_lib::{
    gpio::{SDMMC2_CK_PC1, SDMMC2_CMD_PA0, SDMMC2_D0_PB14},
    *,
};

use embassy_time::{Duration, Timer};

const PIC_BUF_SIZE: usize = 600_000;
static mut PIC_BUF: [u8; PIC_BUF_SIZE] = [0; PIC_BUF_SIZE];

const LED_GREEN: gpio::GpioPort = gpio::PC3;
const LED_ORANGE: gpio::GpioPort = gpio::PC4;
const LED_BLUE: gpio::GpioPort = gpio::PC5;
const CAM_I2C: gi2c::I2cPort = gi2c::I2C3;
const CAM_PDWN: gpio::GpioPort = gpio::PB0;

fn setup() {
    // this function setup for peripheral
    clock::init_clock();
    // CAM_PDWN.setup();
    // LED_ORANGE.setup();
    // LED_ORANGE.set_high();
    // LED_GREEN.setup();
    // LED_GREEN.set_high();
    // LED_BLUE.setup();
    // LED_BLUE.set_high();
    // CAM_I2C.init(100_000, gpio::I2C3_SCL_PC0, gpio::I2C3_SDA_PB4);
    // CAM_PDWN.set_high();
}
use u5_lib::sdmmc::*;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    setup();
    let mut sd = SdInstance::new(stm32_metapac::SDMMC2);
    sd.init(SDMMC2_CK_PC1, SDMMC2_D0_PB14, SDMMC2_CMD_PA0);
    defmt::info!("init sd card");

    // GPDMA1.ch(0).tr1().modify(|w| w.set_dap(ChTr1Ap::PORT1));
    loop {
        Timer::after(Duration::from_secs(1)).await;
        LED_GREEN.toggle();
    }
}
