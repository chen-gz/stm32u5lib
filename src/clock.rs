//! There are alot of default settings are applied in the clock setting. Before changing the clock settings. Read these comments carefully. If update the default settings, please upate the comments first.
//! Current only support working with HSE = 16Mhz and LSE = 32.768Khz other frequency is not supported. Check the hardare please.
//! - pll1_r, pll1_p always set to 160Mhz
//! Two clock scheme is supported:
//! 1. without HSE.
//!     - MSI 4Mhz as pll source, the pll output are vary depend on the system clock requirement. pll1_r is always set to system clock except when the system clock is 4Mhz.
//!
//! 2. with HSE.
//!     - system start with MSI 4Mhz as clocck source. Then the system clock is set to HSE 16Mhz as default clock if system clock is less than 16Mhz. Otherwise the pll1_r is set to system clock.
//!
#![allow(dead_code)]

use crate::{gpio, rtc};
use core::sync::atomic::{AtomicU32, Ordering};
use stm32_metapac::pwr::vals::Vos as VoltageScale;
pub use stm32_metapac::rcc::vals::Sdmmcsel as SdmmcClockSource;
pub use stm32_metapac::{rcc, DBGMCU, FLASH, PWR, RCC};

// current system clock frequenciess
/// to avoid the clock frequency changing that make the system unstable. All clock frequency are not allow to chagne after first time set.
static HCLK: AtomicU32 = AtomicU32::new(4_000_000);

// kerenl clock
pub fn get_hclk() -> u32 {
    HCLK.load(Ordering::Relaxed)
}

pub const HSI_FREQ: u32 = 16_000_000;
pub const MSIS_FREQ: u32 = 4_000_000;
pub const MSIK_FREQ: u32 = 4_000_000;
pub const PLL1_R_FREQ: u32 = 160_000_000;
pub const PLL1_Q_FREQ: u32 = 160_000_000;
pub const PLL1_P_FREQ: u32 = 0;

pub fn hclk_request<F, R>(freq: ClockFreqs, code: F) -> F::Output
where
    F: FnOnce() -> R,
{
    let idx = freq.to_idx();
    CLOCK_REQUESTS[idx].fetch_add(1, Ordering::Relaxed);
    set_clock();
    let res = code();
    CLOCK_REQUESTS[idx].fetch_sub(1, Ordering::Relaxed);
    set_clock();
    res
}

pub async fn hclk_request_async<F, R>(freq: ClockFreqs, code: F)
where
    F: FnOnce() -> R,
    R: core::future::Future<Output = ()>,
{
    let idx = freq.to_idx();
    CLOCK_REQUESTS[idx].fetch_add(1, Ordering::SeqCst);
    set_clock();
    let result = code();
    result.await;
    CLOCK_REQUESTS[idx].fetch_sub(1, Ordering::SeqCst);
    set_clock();
}

fn set_pll(freq: u32) {
    // if HSE_AVAILABLE.load(Ordering::Relaxed) && HSE_FREQ.load(Ordering::Relaxed) != 16_000_000 {
    //     info!("unsupported HSE frequency, fallback to MSIS");
    // }
    // Turn PLL off before reconfiguring
    RCC.cr().modify(|w| w.set_pllon(0, false));
    while RCC.cr().read().pllrdy(0) {}

    #[cfg(feature = "hse_16mhz")]
    RCC.pll1cfgr().modify(|w| {
        w.set_pllsrc(stm32_metapac::rcc::vals::Pllsrc::HSE);
        w.set_pllm(stm32_metapac::rcc::vals::Pllm::DIV4);
    });
    #[cfg(not(any(feature = "hse_16mhz", feature = "hse_26mhz")))]
    RCC.pll1cfgr().modify(|w| {
        w.set_pllsrc(stm32_metapac::rcc::vals::Pllsrc::MSIS);
        w.set_pllm(stm32_metapac::rcc::vals::Pllm::DIV1);
    });

    // PLL input is 4MHz. VCO output is 320MHz.
    // target_freq = (4MHz * PLLN) / PLLR
    // With PLLN = 80, VCO = 320MHz.
    // target_freq = 320MHz / PLLR
    // PLLR = 320MHz / target_freq
    let pllr = 320_000_000 / freq;
    if 320_000_000 % freq != 0 {
        panic!("Unsupported frequency");
    }

    RCC.pll1divr().modify(|v| {
        v.set_plln(stm32_metapac::rcc::vals::Plln::MUL80);
        v.set_pllr(((pllr - 1) as u8).into());
        v.set_pllq(stm32_metapac::rcc::vals::Plldiv::DIV2);
        v.set_pllp(stm32_metapac::rcc::vals::Plldiv::DIV2);
    });

    RCC.pll1cfgr().modify(|w| {
        w.set_pllren(true); // enable pll1_r
        w.set_pllpen(true); // enable pll1_p
    });
    RCC.cr().modify(|w| {
        w.set_pllon(0, true);
    });
    while !RCC.cr().read().pllrdy(0) {}
}

fn delay_enable() {
    unsafe {
        // SYSTEM_CLOCK = system_clock;
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
        let interval = HCLK.load(Ordering::Relaxed);
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
        let hclk = HCLK.load(Ordering::Relaxed) as u64;
        let interval = (hclk * n as u64 / 1_000) as u32;
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
        let hclk = HCLK.load(Ordering::Relaxed) as u64;
        let interval = (hclk * n as u64 / 1_000_000) as u32;
        let start = dwt.cyccnt.read();
        let end = start.wrapping_add(interval);
        let mut now = dwt.cyccnt.read();
        while (now >= start && (now <= end || start > end)) || (now <= end && end < start) {
            now = dwt.cyccnt.read();
        }
    }
}

pub fn delay_tick(n: u32) {
    for _i in 0..n {
        cortex_m::asm::nop();
    }
}

pub fn set_gpio_clock(gpio: stm32_metapac::gpio::Gpio) {
    PWR.svmcr().modify(|v| v.set_io2sv(true));
    // todo check VDDIO2
    if gpio == stm32_metapac::GPIOA {
        RCC.ahb2enr1().modify(|v| v.set_gpioaen(true));
    } else if gpio == stm32_metapac::GPIOB {
        RCC.ahb2enr1().modify(|v| v.set_gpioben(true));
    } else if gpio == stm32_metapac::GPIOC {
        RCC.ahb2enr1().modify(|v| v.set_gpiocen(true));
    } else if gpio == stm32_metapac::GPIOD {
        if RCC.ahb2enr1().read().gpioden() {
            return;
        }
        RCC.ahb2enr1().modify(|v| v.set_gpioden(true));
    } else if gpio == stm32_metapac::GPIOE {
        RCC.ahb2enr1().modify(|v| v.set_gpioeen(true));
    } else if gpio == stm32_metapac::GPIOF {
        RCC.ahb2enr1().modify(|v| v.set_gpiofen(true));
        RCC.ahb2smenr1().modify(|v| v.set_gpiofsmen(true));
    } else if gpio == stm32_metapac::GPIOG {
        RCC.ahb2enr1().modify(|v| v.set_gpiogen(true));
    } else {
        panic!("Not supported gpio");
    }
}

#[cfg(any(sdmmc))]
pub fn set_sdmmc_clock(sdmmc: stm32_metapac::sdmmc::Sdmmc, clk_src: SdmmcClockSource) -> Result<(), ()> {
    // the clock source can only be set once
    // use HSI48 as ICLK
    RCC.ccipr1()
        .modify(|v| v.set_iclksel(stm32_metapac::rcc::vals::Iclksel::HSI48));

    if RCC.ahb2enr1().read().sdmmc1en() || RCC.ahb2enr1().read().sdmmc2en() {
        // check the clock source
        let src = RCC.ccipr2().read().sdmmcsel();
        if src != clk_src {
            panic!("Clock source can only be set once");
        }
    } else {
        // panic!("SDMMC not enabled");
        RCC.ccipr2().modify(|v| v.set_sdmmcsel(SdmmcClockSource::ICLK));
    }
    // RCC.ccipr2().modify(|v| v.set_sdmmcsel(SdmmcClockSource::PLL1_P));

    if sdmmc == stm32_metapac::SDMMC1 {
        RCC.ahb2enr1().modify(|v| v.set_sdmmc1en(true));
        Ok(())
    } else if sdmmc == stm32_metapac::SDMMC2 {
        RCC.ahb2enr1().modify(|v| v.set_sdmmc2en(true));
        Ok(())
    } else {
        panic!("Not supported sdmmc");
    }
}

pub fn set_usart_clock() {
    // set usart1 clock source to hsi
    RCC.ccipr1()
        .modify(|v| v.set_usart1sel(stm32_metapac::rcc::vals::Usart1sel::HSI));
    // enable usart1 clock
    RCC.apb2enr().modify(|v| v.set_usart1en(true));
}

pub fn set_i2c_clock(i2c_num: u8) {
    // set i2c1 clock source to hsi
    if i2c_num == 1 {
        RCC.ccipr1().modify(|v| v.set_i2c1sel(stm32_metapac::rcc::vals::I2csel::HSI));
        // enable i2c1 clock
        RCC.apb1enr1().modify(|v| v.set_i2c1en(true));
    } else if i2c_num == 2 {
        RCC.ccipr1().modify(|v| v.set_i2c2sel(stm32_metapac::rcc::vals::I2csel::HSI));
        // enable i2c2 clock
        RCC.apb1enr1().modify(|v| v.set_i2c2en(true));
        RCC.apb1smenr1().modify(|v| v.set_i2c2smen(true));
    } else if i2c_num == 3 {
        RCC.ccipr3().modify(|v| v.set_i2c3sel(stm32_metapac::rcc::vals::I2c3sel::HSI));
        // enable i2c3 clock
        RCC.apb3enr().modify(|v| v.set_i2c3en(true));
    } else {
        panic!("Invalid i2c number");
    }
}

/// enable lptim for all mode and use LSE as clock source
pub fn set_lptim_clock(num: u8) -> u32 {
    RCC.cr().modify(|v| v.set_hsikeron(true));
    match num {
        1 => {
            RCC.ccipr3()
                .modify(|v| v.set_lptim1sel(stm32_metapac::rcc::vals::Lptimsel::HSI));
            RCC.apb3enr().modify(|v| v.set_lptim1en(true));
            RCC.apb3smenr().modify(|v| v.set_lptim1smen(true));
            RCC.srdamr().modify(|v| v.set_lptim1amen(true));
            HSI_FREQ
        }
        2 => {
            RCC.ccipr1()
                .modify(|v| v.set_lptim2sel(stm32_metapac::rcc::vals::Lptim2sel::HSI));
            RCC.apb1enr2().modify(|v| v.set_lptim2en(true));
            RCC.apb3smenr().modify(|v| v.set_lptim1smen(true));
            HSI_FREQ
        }
        3 | 4 => {
            #[cfg(feature = "lse")]
            {
                RCC.ccipr3()
                    .modify(|v| v.set_lptim34sel(stm32_metapac::rcc::vals::Lptimsel::LSE));
                if num == 3 {
                    RCC.apb3enr().modify(|v| v.set_lptim3en(true));
                } else {
                    RCC.apb3enr().modify(|v| v.set_lptim4en(true));
                }
                32768 // LSE_FREQ.load(Ordering::Relaxed)
            }
            #[cfg(not(feature = "lse"))]
            {
                RCC.ccipr3()
                    .modify(|v| v.set_lptim34sel(stm32_metapac::rcc::vals::Lptimsel::LSI));
                if num == 3 {
                    RCC.apb3enr().modify(|v| v.set_lptim3en(true));
                } else {
                    RCC.apb3enr().modify(|v| v.set_lptim4en(true));
                }
                32000 // LSI_FREQ
            }
        }
        _ => panic!("Invalid lptim number"),
    }
}

pub fn set_adc_clock() {
    RCC.ahb3enr().modify(|v| v.set_pwren(true));
    PWR.svmcr().modify(|v| v.set_asv(true));

    // rm0456 rev4 p 495
    // The ADC and DAC kernel clock source is selected thanks to ADCDACSEL[2:0] in RCC_CCIPR3.
    // use hsi16 as adc clock
    RCC.ccipr3()
        .modify(|v| v.set_adcdacsel(stm32_metapac::rcc::vals::Adcdacsel::HSI));
    // enable adc clock
    RCC.ahb2enr1().modify(|v| v.set_adc12en(true));
}

pub fn init_clock(
    // has_hse: bool,
    // has_lse: bool,
    // hse_frq: u32,
    enable_dbg: bool,
    system_min_freq: ClockFreqs,
) {
    debug!(
        "setup clock with enable_dbg: {:?}, system_min_freq: {:?}",
        enable_dbg, system_min_freq
    );

    RCC.ahb3enr().modify(|v| v.set_pwren(true));
    static mut CALLED: bool = false;
    unsafe {
        // this is safe because this function should only be called once
        if CALLED {
            panic!("init_clock_new should only be called once");
        }
        CALLED = true;
    }
    if enable_dbg {
        DBGMCU.cr().modify(|cr| {
            cr.set_dbg_stop(true);
            cr.set_dbg_standby(true);
        });
    }
    CLOCK_REQUESTS[system_min_freq.to_idx()].store(1, Ordering::Relaxed);
    // check rtc is enabled or not. if enabled, do nothing
    let rtc_en = RCC.bdcr().read().rtcen();
    if !rtc_en {
        info!("RTC not enabled, enable RTC");
        // if has_lse {
        #[cfg(feature = "lse")]
        rtc::setup(20, 01, 01, 01, 01, 0, 0, rcc::vals::Rtcsel::LSE);
        #[cfg(not(feature = "lse"))]
        rtc::setup(20, 01, 01, 01, 01, 0, 0, rcc::vals::Rtcsel::LSI);
    }
    set_clock();
}

pub static CLOCK_REQUESTS: [AtomicU32; 32] = [const { AtomicU32::new(0) }; 32];
// Clock source frequency versus voltage scaling (page 494)
#[derive(Clone, Copy, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ClockFreqs {
    KernelFreq160Mhz,
    KernelFreq80Mhz,
    KernelFreq40Mhz,
    KernelFreq20Mhz,
    KernelFreq16Mhz,
    KernelFreq8Mhz,
    KernelFreq4Mhz,
    KernelFreq2Mhz,
    KernelFreq1Mhz,
}

impl ClockFreqs {
    pub fn to_idx(&self) -> usize {
        match self {
            ClockFreqs::KernelFreq160Mhz => 0,
            ClockFreqs::KernelFreq80Mhz => 1,
            ClockFreqs::KernelFreq40Mhz => 2,
            ClockFreqs::KernelFreq20Mhz => 3,
            ClockFreqs::KernelFreq16Mhz => 4,
            ClockFreqs::KernelFreq8Mhz => 5,
            ClockFreqs::KernelFreq4Mhz => 6,
            ClockFreqs::KernelFreq2Mhz => 7,
            ClockFreqs::KernelFreq1Mhz => 8,
        }
    }
    fn to_freq(&self) -> u32 {
        match self {
            ClockFreqs::KernelFreq160Mhz => 160_000_000,
            ClockFreqs::KernelFreq80Mhz => 80_000_000,
            ClockFreqs::KernelFreq40Mhz => 40_000_000,
            ClockFreqs::KernelFreq20Mhz => 20_000_000,
            ClockFreqs::KernelFreq16Mhz => 16_000_000,
            ClockFreqs::KernelFreq8Mhz => 8_000_000,
            ClockFreqs::KernelFreq4Mhz => 4_000_000,
            ClockFreqs::KernelFreq2Mhz => 2_000_000,
            ClockFreqs::KernelFreq1Mhz => 1_000_000,
        }
    }
    fn from_idx(idx: u16) -> Self {
        match idx {
            0 => ClockFreqs::KernelFreq160Mhz,
            1 => ClockFreqs::KernelFreq80Mhz,
            2 => ClockFreqs::KernelFreq40Mhz,
            3 => ClockFreqs::KernelFreq20Mhz,
            4 => ClockFreqs::KernelFreq16Mhz,
            5 => ClockFreqs::KernelFreq8Mhz,
            6 => ClockFreqs::KernelFreq4Mhz,
            7 => ClockFreqs::KernelFreq2Mhz,
            8 => ClockFreqs::KernelFreq1Mhz,
            _ => panic!("Invalid index"),
        }
    }
}

pub fn set_clock() {
    // check the clock requirement to determine the kernel clock
    // default kernel clock is 4Mhz
    let mut clk_idx: usize = 0;
    for i in 0..CLOCK_REQUESTS.len() {
        if CLOCK_REQUESTS[i].load(Ordering::Relaxed) > 0 {
            clk_idx = i;
            break;
        }
    }

    // if HSE_AVAILABLE.load(Ordering::Relaxed) {
    #[cfg(any(feature = "hse_16mhz", feature = "hse_26mhz"))]
    {
        RCC.cr().modify(|w| w.set_hseon(true));
        while !RCC.cr().read().hserdy() {}
    }

    // se hsi16 on
    RCC.cr().modify(|w| w.set_hsion(true));
    while !RCC.cr().read().hsirdy() {}

    // set hsi48 on
    RCC.cr().modify(|w| w.set_hsi48on(true));
    while !RCC.cr().read().hsi48rdy() {}

    // set hsi48 as clock source for iclk
    RCC.ccipr1()
        .modify(|v| v.set_iclksel(stm32_metapac::rcc::vals::Iclksel::HSI48));

    delay_enable();

    set_cpu_freq_new(ClockFreqs::from_idx(clk_idx as u16).to_freq(), false);
}

pub fn get_ws_and_vcore(sys_clk: u32) -> (u8, VoltageScale) {
    // return (0, VoltageScale::RANGE4);
    // refter to rm0456 rev4 table 54 and p278
    if sys_clk <= 12_000_000 {
        return (0, VoltageScale::RANGE4);
    } else if sys_clk <= 24_000_000 {
        return (0, VoltageScale::RANGE3);
    } else if sys_clk <= 48_000_000 {
        return (1, VoltageScale::RANGE3);
    } else if sys_clk <= 60_000_000 {
        return (1, VoltageScale::RANGE2);
    } else if sys_clk <= 90_000_000 {
        return (2, VoltageScale::RANGE2);
    } else if sys_clk <= 110_000_000 {
        return (3, VoltageScale::RANGE2);
    } else if sys_clk <= 128_000_000 {
        return (3, VoltageScale::RANGE1);
    } else if sys_clk <= 160_000_000 {
        return (4, VoltageScale::RANGE1);
    } else {
        panic!("sys_clk is too high");
    }
}

pub fn set_cpu_freq_new(freq: u32, _lpm: bool) {
    if HCLK.load(Ordering::Relaxed) > freq {
        dec_kern_freq(freq);
    } else if HCLK.load(Ordering::Relaxed) <= freq {
        inc_kern_freq(freq);
    }
}

fn inc_kern_freq(freq: u32) {
    // if unsafe { HSE_AVAILABLE } {
    let (ws, vcore) = get_ws_and_vcore(freq);
    if ws > 0 {
        //enable flash prefetch
        FLASH.acr().modify(|w| w.set_prften(true)); // prefetch must be set if at least one wait state is needed to access the flash memory
    }
    // update ws
    FLASH.acr().modify(|w| w.set_latency(ws));
    if freq >= 55_000_000 {
        set_pll(freq);
        RCC.pll1cfgr().modify(|w| {
            w.set_pllmboost(rcc::vals::Pllmboost::DIV1);
        });
        RCC.ahb3enr().modify(|w| w.set_pwren(true));
        // enable boost
        PWR.vosr().modify(|w| {
            w.set_vos(vcore);
            w.set_boosten(true);
        });
        // wait for boost ready
        while !PWR.vosr().read().boostrdy() {}
        while !PWR.vosr().read().vosrdy() {}
    }
    // hclk /2
    #[cfg(any(stm32u595, stm32u5a5))]
    RCC.cfgr2().modify(|w| w.set_hpre(rcc::vals::Hpre::DIV2));

    let hclk_source;
    if freq <= 16_000_000 {
        // if HSE_AVAILABLE.load(Ordering::Relaxed) {
        #[cfg(any(feature = "hse_16mhz", feature = "hse_26mhz"))]
        {
            while !RCC.cr().read().hserdy() {}
            RCC.cfgr1().modify(|w| {
                w.set_sw(stm32_metapac::rcc::vals::Sw::HSE);
            });
        }
        #[cfg(not(any(feature = "hse_16mhz", feature = "hse_26mhz")))]
        {
            while !RCC.cr().read().hsirdy() {}
            RCC.cfgr1().modify(|w| {
                w.set_sw(stm32_metapac::rcc::vals::Sw::HSI);
            });
        }
        hclk_source = 16_000_000;
    } else {
        set_pll(freq);
        RCC.cfgr2().modify(|w| w.set_hpre(rcc::vals::Hpre::DIV2));
        // set pll as system clock
        RCC.cfgr1().modify(|w| {
            w.set_sw(stm32_metapac::rcc::vals::Sw::PLL1_R);
        });
        hclk_source = freq;
    }
    //calc hclk
    let hclk = hclk_source / freq; // should be 2, 4, 8, 16, 32, 64, 128
    if hclk_source % freq != 0 {
        panic!("Invalid hclk");
    }
    match hclk {
        1 => RCC.cfgr2().modify(|w| w.set_hpre(rcc::vals::Hpre::DIV1)),
        2 => RCC.cfgr2().modify(|w| w.set_hpre(rcc::vals::Hpre::DIV2)),
        4 => RCC.cfgr2().modify(|w| w.set_hpre(rcc::vals::Hpre::DIV4)),
        8 => RCC.cfgr2().modify(|w| w.set_hpre(rcc::vals::Hpre::DIV8)),
        16 => RCC.cfgr2().modify(|w| w.set_hpre(rcc::vals::Hpre::DIV16)),
        64 => RCC.cfgr2().modify(|w| w.set_hpre(rcc::vals::Hpre::DIV64)),
        128 => RCC.cfgr2().modify(|w| w.set_hpre(rcc::vals::Hpre::DIV128)),
        _ => panic!("Invalid hclk {}", hclk),
    }
    RCC.cfgr2().read().hpre();
    HCLK.store(freq, Ordering::Relaxed);
}

fn dec_kern_freq(freq: u32) {
    #[cfg(any(stm32u595, stm32u5a5))]
    RCC.cfgr2().modify(|w| w.set_hpre(rcc::vals::Hpre::DIV2));
    let hclk_source;

    if freq <= 16_000_000 {
        // if HSE_AVAILABLE.load(Ordering::Relaxed) {
        #[cfg(any(feature = "hse_16mhz", feature = "hse_26mhz"))]
        {
            while !RCC.cr().read().hserdy() {}
            RCC.cfgr1().modify(|w| {
                w.set_sw(stm32_metapac::rcc::vals::Sw::HSE);
            });
        }
        #[cfg(not(any(feature = "hse_16mhz", feature = "hse_26mhz")))]
        {
            while !RCC.cr().read().hsirdy() {}
            RCC.cfgr1().modify(|w| {
                w.set_sw(stm32_metapac::rcc::vals::Sw::HSI);
            });
        }
        hclk_source = 16_000_000;
    } else {
        set_pll(freq);
        // set pll as system clock
        RCC.cfgr1().modify(|w| {
            w.set_sw(stm32_metapac::rcc::vals::Sw::PLL1_R);
        });
        hclk_source = freq;
    }
    //calc hclk
    let hclk = hclk_source / freq; // should be 2, 4, 8, 16, 32, 64, 128
    match hclk {
        1 => RCC.cfgr2().modify(|w| w.set_hpre(rcc::vals::Hpre::DIV1)),
        2 => RCC.cfgr2().modify(|w| w.set_hpre(rcc::vals::Hpre::DIV2)),
        4 => RCC.cfgr2().modify(|w| w.set_hpre(rcc::vals::Hpre::DIV4)),
        8 => RCC.cfgr2().modify(|w| w.set_hpre(rcc::vals::Hpre::DIV8)),
        16 => RCC.cfgr2().modify(|w| w.set_hpre(rcc::vals::Hpre::DIV16)),
        64 => RCC.cfgr2().modify(|w| w.set_hpre(rcc::vals::Hpre::DIV64)),
        128 => RCC.cfgr2().modify(|w| w.set_hpre(rcc::vals::Hpre::DIV128)),
        _ => panic!("Invalid hclk"),
    }

    let (ws, vcore) = get_ws_and_vcore(freq);
    if ws == 0 {
        FLASH.acr().modify(|w| w.set_prften(false));
    }
    FLASH.acr().modify(|w| w.set_latency(ws)); // update ws
    if HCLK.load(Ordering::Relaxed) >= 55_000_000 && freq < 55_000_000 {
        RCC.pll1cfgr().modify(|w| {
            w.set_pllmboost(rcc::vals::Pllmboost::DIV1);
        });
        // enable boost
        PWR.vosr().modify(|w| {
            w.set_boosten(false);
            w.set_vos(vcore);
        });
        while !PWR.vosr().read().vosrdy() {}
    }

    HCLK.store(freq, Ordering::Relaxed);
}

pub use stm32_metapac::rcc::vals::Mcopre;
pub use stm32_metapac::rcc::vals::Mcosel;

pub fn set_mco(pin: gpio::GpioPort, clk: Mcosel, div: stm32_metapac::rcc::vals::Mcopre) {
    pin.setup();
    RCC.cfgr1().modify(|w| {
        w.set_mcosel(clk);
        w.set_mcopre(div);
    });
}
