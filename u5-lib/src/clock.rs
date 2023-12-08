#![allow(dead_code)]
use cortex_m::{self};

use embassy_stm32::{interrupt::typelevel::RCC, rcc::*};
use stm32_metapac::{rcc, RCC};
pub static mut SYSTEM_CLOCK: u32 = 4_000_000;
pub fn get_kernel_freq() -> u32 {
    unsafe { SYSTEM_CLOCK }
}

pub fn delay_enable(system_clock: u32) {
    unsafe {
        SYSTEM_CLOCK = system_clock;
        let mut p = cortex_m::Peripherals::steal();
        p.DCB.enable_trace(); // enable trace
        let dwt = &mut p.DWT;
        dwt.enable_cycle_counter();
        dwt.cyccnt.modify(|_w| 0);
    }
}

pub fn delay_s(n: u32) {
    unsafe {
        let p = cortex_m::Peripherals::steal();
        let dwt = &p.DWT;
        let interval = SYSTEM_CLOCK;
        for _i in 0..n {
            let start = dwt.cyccnt.read();
            let end = start.wrapping_add(interval);
            let mut now = dwt.cyccnt.read();
            while (now >= start && (now <= end || start > end)) || (now <= end && end < start) {
                now = dwt.cyccnt.read();
            }
        }
    }
}

pub fn delay_ms(n: u32) {
    unsafe {
        let p = cortex_m::Peripherals::steal();
        let dwt = &p.DWT;
        let interval = SYSTEM_CLOCK / 1_000 * n;
        // 170 * (1e3 as u32) * n;
        let start = dwt.cyccnt.read();
        let end = start.wrapping_add(interval);
        let mut now = dwt.cyccnt.read();
        while (now >= start && (now <= end || start > end)) || (now <= end && end < start) {
            now = dwt.cyccnt.read();
        }
    }
}

pub fn delay_us(n: u32) {
    unsafe {
        let p = cortex_m::Peripherals::steal();
        let dwt = &p.DWT;
        let interval = SYSTEM_CLOCK / 1_000_000 * n;
        let start = dwt.cyccnt.read();
        let end = start.wrapping_add(interval);
        let mut now = dwt.cyccnt.read();
        while (now >= start && (now <= end || start > end)) || (now <= end && end < start) {
            now = dwt.cyccnt.read();
        }
    }
}
pub fn delay_tick(n: u32) {
    unsafe {
        let p = cortex_m::Peripherals::steal();
        let dwt = &p.DWT;
        let start = dwt.cyccnt.read();
        let end = start.wrapping_add(n);
        let mut now = dwt.cyccnt.read();
        while (now >= start && (now <= end || start > end)) || (now <= end && end < start) {
            now = dwt.cyccnt.read();
        }
    }
}

pub fn init_clock() {
    let mut config = embassy_stm32::Config::default();
    config.rcc.mux = ClockSrc::PLL1_R(PllConfig::hsi_160mhz());
    config.rcc.voltage_range = VoltageScale::RANGE1; // this is for high frquency. This should be
    //                                                  // should better set in the rcc module. instead of here.
    let _p = embassy_stm32::init(config);
    delay_enable(160_000_000);
    RCC.ccipr1()
        .modify(|v| v.set_i2c1sel(stm32_metapac::rcc::vals::Icsel::HSI));
    RCC.apb1enr1().modify(|v| v.set_i2c1en(true));

    RCC.ccipr3()
        .modify(|v| v.set_i2c3sel(stm32_metapac::rcc::vals::Icsel::HSI));
    RCC.apb3enr().modify(|v| v.set_i2c3en(true));

    // dcmi clock
    RCC.ahb2enr1().modify(|v| v.set_dcmien(true));
    // enable dma clock
    RCC.ahb1enr().modify(|v| v.set_gpdma1en(true));
    // enable gpioa clock
    RCC.ahb2enr1().modify(|v| {
        v.set_gpioaen(true);
        v.set_gpioben(true);
        v.set_gpiocen(true);
    });

    // enable gpioG clock
    RCC.ahb2enr1().modify(|v| v.set_gpiogen(true));

    // enable hsi48 clock
    RCC.cr().modify(|v| v.set_hsi48on(true));
    // warit for hsi48 ready
    while !RCC.cr().read().hsi48rdy() {}

    // hsi48 as iclk
    RCC.ccipr1()
        .modify(|v| v.set_iclksel(rcc::vals::Iclksel::HSI48));

    // use hsi48 for sdmmc clock
    RCC.ccipr2()
        .modify(|v| v.set_sdmmcsel(rcc::vals::Sdmmcsel::ICLK));

    // enable sdmmc clock
    RCC.ahb2enr1().modify(|v| v.set_sdmmc1en(true));

    // enable sdmmc2 clock
    RCC.ahb2enr1().modify(|v| v.set_sdmmc2en(true));
}
pub fn set_clock_to_pll() {
    // let mut config = embassy_stm32::Config::default();
    // config.rcc.mux = ClockSrc::PLL1_R(PllConfig::hsi_160mhz());
    // config.rcc.voltage_range = VoltageScale::RANGE1; // this is for high frquency. This should be
    // //                                                  // should better set in the rcc module. instead of here.
    // let _p = embassy_stm32::init(config);
    delay_enable(160_000_000);

    RCC.cfgr1().modify(|w| {
        w.set_sw(stm32_metapac::rcc::vals::Sw::PLL1_R);
    });
}

pub fn set_clock_to_hsi() {
    // let mut config = embassy_stm32::Config::default();
    // config.rcc.mux = ClockSrc::PLL1_R(PllConfig::hsi_160mhz());
    // config.rcc.voltage_range = VoltageScale::RANGE1; // this is for high frquency. This should be
    // //                                                  // should better set in the rcc module. instead of here.
    // let _p = embassy_stm32::init(config);
    delay_enable(4_000_000);

    RCC.cfgr1().modify(|w| {
        w.set_sw(stm32_metapac::rcc::vals::Sw::HSI);
    });
}