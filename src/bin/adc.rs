#![feature(noop_waker)]
#![no_std]
#![no_main]
use cortex_m_rt::entry;
use u5_lib::clock;
use u5_lib::gpio;
use defmt_rtt as _;
use gpio::GpioPort;
use u5_lib::gpio::{
    USART_TX_PA9,
    USART_RX_PA10
};
use u5_lib::low_power::mcu_no_deep_sleep;
use u5_lib::usart::USART1;
const GREEN: GpioPort = gpio::PB7;
const ORANGE: GpioPort = gpio::PC4;
const BLUE: GpioPort = gpio::PC5;
fn setup() {
    GREEN.setup();
    ORANGE.setup();
    BLUE.setup();
}
use heapless::String;
use core::fmt::Write;

#[entry]
fn main() -> ! {
    clock::init_clock();
    clock::kernel_freq_160mhz_request();
    mcu_no_deep_sleep();
    setup();

    USART1.setup(USART_TX_PA9, USART_RX_PA10);
    // USART1.setup(USART_TX_PA9, u5_lib::gpio::USART_RX_PA10);
    // unsafe {
    //     u5_lib::tim::TIM1.init(Config::default());
    // }
u5_lib::gpio::ADC1_IN1_PC0.setup();

    USART1.send("usart init finished\n".as_bytes());
    ADC1.init();
    USART1.send("adc init finished\n".as_bytes());
    // let rst = ADC1.start_conversion_sw(u5_lib::gpio::ADC1_IN2_PC1, 2);
    // defmt::info!("adc result: {}", rst);
    // for i in 0..10 {
        let rst = ADC1.get_vref_int_raw();
        defmt::info!("vref int raw: {}, float: {}",rst, rst as f32 /(1<<14)as f32 * 3.0);
        // let rst = ADC1.start_conversion_sw(u5_lib::gpio::ADC1_IN3_PC2, 0);
        // defmt::info!("adc result: {}", rst as f32 /(1 <<14) as f32 * 3.3);
        defmt::info!("adc result raw: {}, float: {}",rst, rst as f32 /(1<<14)as f32 * 3.3);
        clock::delay_ms(10);
        let rst = ADC1.start_conversion_sw(u5_lib::gpio::ADC1_IN1_PC0, 1);
        // let rst = ADC1.start_conversion_sw(u5_lib::gpio::ADC1_IN6_PC0, 5);
        // defmt::info!("adc result of PC2: {}", rst as f32/(1 <<14) as f32 * 3.3);
        defmt::info!("adc result raw PA0: {}, float: {}",rst, rst as f32 /(1<<14)as f32 * 3.3);
        clock::delay_ms(500);
        let rst = ADC1.start_conversion_sw(u5_lib::gpio::ADC1_IN1_PC0, 1);
        clock::delay_ms(500);
        defmt::info!("adc result raw PA0: {}, float: {}",rst, rst as f32 /(1<<14)as f32 * 3.3);

    // }
    // let rst = ADC1.start_conversion_sw(u5_lib::gpio::ADC1_IN2_PC1, 2);
    // defmt::info!("adc result: {}", rst);

    // USART1.send(write!(s, "adc result: {}\n", rst).unwrap().as_bytes());

    loop {
        GREEN.toggle();
        // ORANGE.toggle();
        clock::delay_ms(500);
    }
}

use core::panic::PanicInfo;
use u5_lib::adc::ADC1;
use u5_lib::tim::Config;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
