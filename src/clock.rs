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
use core::panic;

use cortex_m::{self};
// current system clock frequenciess
/// to avoid the clock frequency changing that make the system unstable. All clock frequency are not allow to chagne after first time set.
struct CLOCKS {
    msis: u32,
    msik: u32,
    hse: u32,
    pll1_p: u32,
    pll1_q: u32,
    pll1_r: u32,
    hclk: u32, // aka system clock
    iclk: u32,
}
const MSIS_FREQ: u32 = 4_000_000;
const MSIK_FREQ: u32 = 4_000_000;
const HSE_FREQ: u32 = 16_000_000;
const PLL1_R_FREQ: u32 = 160_000_000;
const PLL1_Q_FREQ: u32 = 160_000_000;
const PLL1_P_FREQ: u32 = 0;
static mut HSE_AVAILABLE: bool = false;

struct ClockRef {
    hsi16: u32,
    hsi48: u32,
    kernel_freq_160mhz: u32,
    kernel_freq_80mhz: u32,
    kernel_freq_64mhz: u32,
    kernel_freq_48mhz: u32,
    kernel_freq_32mhz: u32,
    kernel_freq_16mhz: u32,
    kernel_freq_4mhz: u32,
}

pub fn hsi16_request() {
    unsafe {
        CLOCK_REF.hsi16 += 1;
    }
    set_clock();
}
pub fn hsi16_release() {
    unsafe {
        CLOCK_REF.hsi16 -= 1;
    }
    set_clock();
}

pub fn hsi48_request() {
    unsafe {
        CLOCK_REF.hsi48 += 1;
    }
    set_clock();
}
pub fn hsi48_release() {
    unsafe {
        CLOCK_REF.hsi48 -= 1;
    }
    set_clock();
}
pub fn kernel_freq_160mhz_request() {
    unsafe {
        CLOCK_REF.kernel_freq_160mhz += 1;
    }
    set_clock();
}
fn kernel_freq_160mhz_release() {
    unsafe {
        CLOCK_REF.kernel_freq_160mhz -= 1;
    }
    set_clock();
}

pub fn run_with_160mhz<F>(code: F)
where
    F: FnOnce(),
{
    unsafe {
        CLOCK_REF.kernel_freq_160mhz += 1;
        set_clock();
        code();
        CLOCK_REF.kernel_freq_160mhz -= 1;
    }
}
pub async fn run_with_160mhz_async<F, R>(code: F)
where
    F: FnOnce() -> R,
    R: core::future::Future<Output = ()>,
{
    unsafe {
        CLOCK_REF.kernel_freq_160mhz += 1;
        set_clock();
        let result = code();
        result.await;
        kernel_freq_160mhz_release();
        CLOCK_REF.kernel_freq_160mhz -= 1;
        set_clock();
    }
}
static mut CLOCK_REQUESTS: [u16; 32] = [0; 32];
enum ClockFreqs {
    KernelFreq160Mhz, // 160Mhz
    KernelFreq80Mhz,  // 80Mhz
    KernelFreq64Mhz,  // 64Mhz
    KernelFreq48Mhz,  // 48Mhz
    KernelFreq40Mhz,  // 40Mhz
    KernelFreq32Mhz,  // 32Mhz
    KernelFreq24Mhz,  // 24Mhz
    KernelFreq20Mhz,  // 20Mhz
    KernelFreq16Mhz,  // 16Mhz
    KernelFreq8Mhz,   // 8Mhz
    KernelFreq4Mhz,   // 4Mhz
    KernelFreq2Mhz,   // 2Mhz
    KernelFreq1Mhz,   // 1Mhz
}

fn set_pll() {
    if unsafe { HSE_AVAILABLE } {
        RCC.pll1cfgr().modify(|w| {
            w.set_pllsrc(stm32_metapac::rcc::vals::Pllsrc::HSE);
            w.set_pllm(stm32_metapac::rcc::vals::Pllm::DIV4);
        });
    } else {
        RCC.pll1cfgr().modify(|w| {
            w.set_pllsrc(stm32_metapac::rcc::vals::Pllsrc::MSIS);
            w.set_pllm(stm32_metapac::rcc::vals::Pllm::DIV1);
        });
    }
    RCC.pll1divr().modify(|v| {
        v.set_plln(stm32_metapac::rcc::vals::Plln::MUL80);
        v.set_pllr(stm32_metapac::rcc::vals::Plldiv::DIV2);
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

static mut CLOCK_REF: ClockRef = ClockRef {
    hsi16: 1,
    hsi48: 1,
    kernel_freq_160mhz: 0,
    kernel_freq_80mhz: 0,
    kernel_freq_64mhz: 0,
    kernel_freq_48mhz: 0,
    kernel_freq_32mhz: 0,
    kernel_freq_16mhz: 0,
    kernel_freq_4mhz: 1,
};
fn clock_ref() -> &'static ClockRef {
    unsafe { &CLOCK_REF }
}

use defmt::info;
use stm32_metapac::{gpio, pwr, rcc, DBGMCU, FLASH, PWR, RCC};
pub static mut SYSTEM_CLOCK: u32 = 4_000_000; // this is default value when the system start
pub fn get_kernel_freq() -> u32 {
    unsafe { SYSTEM_CLOCK }
}
fn set_kernel_freq(freq: u32) {
    unsafe {
        SYSTEM_CLOCK = freq;
    }
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

pub fn set_gpio_clock(gpio: stm32_metapac::gpio::Gpio) {
    // todo check VDDIO2
    if gpio == stm32_metapac::GPIOA {
        RCC.ahb2enr1().modify(|v| v.set_gpioaen(true));
    } else if gpio == stm32_metapac::GPIOB {
        RCC.ahb2enr1().modify(|v| v.set_gpioben(true));
    } else if gpio == stm32_metapac::GPIOC {
        RCC.ahb2enr1().modify(|v| v.set_gpiocen(true));
    } else if gpio == stm32_metapac::GPIOD {
        RCC.ahb2enr1().modify(|v| v.set_gpioden(true));
    } else if gpio == stm32_metapac::GPIOE {
        RCC.ahb2enr1().modify(|v| v.set_gpioeen(true));
    } else if gpio == stm32_metapac::GPIOG {
        RCC.ahb2enr1().modify(|v| v.set_gpiogen(true));
    } else {
        panic!("Not supported gpio");
    }
}

pub fn set_sdmmc_clock(sdmmc: stm32_metapac::sdmmc::Sdmmc) {
    // if clock_ref().has_hse {

    // }
    // use hsi48 for sdmmc clock
    // RCC.ccipr2()
    //     .modify(|v| v.set_sdmmcsel(rcc::vals::Sdmmcsel::ICLK)); // use iclk as clock source

    // enable sdmmc clock
    RCC.ahb2enr1().modify(|v| v.set_sdmmc1en(true));

    // enable sdmmc2 clock
    RCC.ahb2enr1().modify(|v| v.set_sdmmc2en(true));
}
pub fn set_usart_clock() {
    // set usart1 clock source to hsi
    RCC.ccipr1()
        .modify(|v| v.set_usart1sel(stm32_metapac::rcc::vals::Usart1sel::HSI));
    // enable usart1 clock
    RCC.apb2enr().modify(|v| v.set_usart1en(true));
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

pub fn init_clock() {
    DBGMCU.cr().modify(|cr| {
        cr.set_dbg_stop(true);
        cr.set_dbg_standby(true);
    });
    delay_enable();
    // set_gpio_clock(); // todo: remove this and add to gpio_setup
    set_clock();
}

pub fn init_clock_new(has_hse: bool, enable_dbg: bool) {
    // unsafe {CLOCK_REF.has_hse = has_hse;}
    unsafe {
        HSE_AVAILABLE = has_hse;
    }
    static mut CALLED: bool = false;
    if unsafe { CALLED } {
        panic!("init_clock_new should only be called once");
    }
    unsafe {
        CALLED = true;
    }

    // if has_hse {
    // hse_request(); // default use hse; the frequency should be 16Mhz
    // }
    if enable_dbg {
        DBGMCU.cr().modify(|cr| {
            cr.set_dbg_stop(true);
            cr.set_dbg_standby(true);
        });
    }
    delay_enable();
    set_clock();
}
/// set the cpu frequency to 160Mhz
/// this is the maximum frequency
/// The pll take msis 4Mhz as input source
/// m = 1, vco = 4Mhz * n (80) = 320Mhz, pll_p = 320 / p = 160Mhz, pll_q = 320 / q = 160Mhz, pll_r = 320 / r = 160Mhz
/// pll1_r set to system clock
fn set_cpu_freq_pll_msis_160mhz() {
    // RCC.cr().modify(|w| {
    //     w.set_pllon(0, false);
    // });
    // wait for disable pll1
    // while RCC.cr().read().pllon() {}

    // 1. set pllm and pllsrc
    RCC.pll1cfgr().modify(|w| {
        w.set_pllsrc(stm32_metapac::rcc::vals::Pllsrc::MSIS); // set pll source to msis
        w.set_pllm(stm32_metapac::rcc::vals::Pllm::DIV1); // set pllm to 1
        w.set_pllren(true); // enable pll1_r
    });
    RCC.pll1divr().modify(|v| {
        v.set_plln(stm32_metapac::rcc::vals::Plln::MUL80); // the default value is 129 (not valid )
        v.set_pllr(stm32_metapac::rcc::vals::Plldiv::DIV2); // this is default value
        v.set_pllq(stm32_metapac::rcc::vals::Plldiv::DIV2); // this is default value
        v.set_pllp(stm32_metapac::rcc::vals::Plldiv::DIV2); // this is default value
    });

    // RCC.pll1cfgr().modify(|w| {
    //     w.set_pllren(true); // enable pll1_r
    // });

    PWR.vosr().modify(|w| {
        w.set_boosten(true);
    });
    while !PWR.vosr().read().boostrdy() {}

    // turn on pll1
    RCC.cr().modify(|w| {
        w.set_pllon(0, true);
    });

    // wait for pll1 ready
    while !RCC.cr().read().pllrdy(0) {}

    // 2. set pll1 as system clock
    RCC.cfgr1().modify(|w| {
        w.set_sw(stm32_metapac::rcc::vals::Sw::PLL1_R);
    });

    while RCC.cfgr1().read().sws() != stm32_metapac::rcc::vals::Sw::PLL1_R {}
}
fn set_cpu_freq_pll_hse16m_160mhz() {}

/// The default clock for the system
/// msis is on when chip reset
pub fn set_clock_to_msis_4mhz() {
    RCC.cfgr1().modify(|w| {
        w.set_sw(stm32_metapac::rcc::vals::Sw::MSIS);
    });
    while RCC.cfgr1().read().sws() != stm32_metapac::rcc::vals::Sw::MSIS {}
}
// pub fn set_master_clock()
pub fn set_cpu_freq(freq: u32) {
    // all base on highest voltage range (not dynamic voltage scaling supported)
    /*
     *  Increase the CPU frequency
     * 1.  Program the new number of wait states to LATENCY bits in FLASH_ACR.
     * 2.  Check that the new number of wait states is taken into account to access the flash memory by reading back FLASH_ACR.
     * 3.  Modify the CPU clock source by writing SW bits in RCC_CFGR1.
     * 4.  Modify the CPU clock prescaler, if needed, by writing HPRE bits in RCC_CFGR2.
     * 5.  Check that the new CPU clock source or/and the new CPU clock prescaler value is/are taken into account by reading the clock source status (SWS bits) or/and the AHB prescaler value (HPRE bits), respectively, in RCC_CFGR1 and RCC_CFGR2
     */

    //enable flash prefetch
    FLASH.acr().modify(|w| w.set_prften(true));
    //enable pwr clock
    RCC.ahb3enr().modify(|w| w.set_pwren(true));
    // vos set to range 1
    PWR.vosr().modify(|w| {
        w.set_vos(pwr::vals::Vos::RANGE1);
    });
    while !PWR.vosr().read().vosrdy() {}

    PWR.vosr().modify(|w| {
        w.set_boosten(true);
    });
    // wait for boost ready
    // while !PWR.vosr().read().boostrdy() {}

    // EPOD (embedded power distribution) booster
    // § 10.5.4: if we're targeting >= 55 MHz, we must configure PLL1MBOOST to a prescaler
    // value that results in an output between 4 and 16 MHz for the PWR EPOD boost
    // The EPOD booster input frequency is PLL1 input clock frequency/PLL1MBOOST (PLL1 input clock frquency is 16Mhz in our case)
    RCC.pll1cfgr().modify(|w| {
        w.set_pllmboost(rcc::vals::Pllmboost::DIV1);
    });

    // wait for vos ready
    if freq > unsafe { SYSTEM_CLOCK } {
        // 1. get new wait states and set it to flash
        let wait_states = get_wait_states(freq, VoltageScale::RANGE1);
        FLASH.acr().modify(|w| w.set_latency(wait_states));
        // 2. check wait states
        while FLASH.acr().read().latency() != wait_states {}
        // 3. modify cpu clock source
        if freq == 160_000_000 {
            set_cpu_freq_pll_msis_160mhz();
        } else {
            panic!("Not supported frequency");
        }
        // 4. modify cpu clock prescaler (not change)
        // 5. check cpu clock source // done in set_cpu_freq_pll_msis_160mhz
    }
    /*
     * Decrease the CPU frequency
     * 1. Modify the CPU clock source by writing SW bits in RCC_CFGR1.
     * 2. Modify the CPU clock prescaler, if needed, by writing HPRE bits in RCC_CFGR2.
     * 3. Check that the new CPU clock source or/and the new CPU clock prescaler value is/are taken into account by reading the clock source status (SWS bits) or/and the AHB prescaler value (HPRE bits), respectively, in RCC_CFGR1 and RCC_CFGR2.
     * 4. Program the new number of wait states to LATENCY bits in FLASH_ACR.
     * 5. Check that the new number of wait states is used to access the flash memory by reading back FLASH_ACR.
     */
    else if freq < unsafe { SYSTEM_CLOCK } {
        // 1. modify cpu clock source
        if freq == 160_000_000 {
            set_cpu_freq_pll_msis_160mhz();
        } else if freq == 4_000_000 {
            set_clock_to_msis_4mhz();
        }
        // 2. modify cpu clock prescaler (not change)
        // 3. check cpu clock source (done in set_cpu_freq_pll_msis_4mhz)
        // 4. get new wait states and set it to flash
        let wait_states = get_wait_states(freq, VoltageScale::RANGE1);
        FLASH.acr().modify(|w| w.set_latency(wait_states));
        // 5. check wait states
        while FLASH.acr().read().latency() != wait_states {}
    }

    unsafe {
        SYSTEM_CLOCK = freq;
    }
}

use stm32_metapac::pwr::vals::Vos as VoltageScale;
pub fn get_wait_states(sys_clk: u32, voltage_range: VoltageScale) -> u8 {
    FLASH.acr().modify(|w| w.set_prften(true));
    // prefetch always enable for now
    // from rm0456 rev4 table 54 (LPM = 0) Flash Low power mode is disabled
    match voltage_range {
        // VOS 1 range VCORE 1.26V - 1.40V
        VoltageScale::RANGE1 => {
            if sys_clk <= 32_000_000 {
                0
            } else if sys_clk <= 64_000_000 {
                1
            } else if sys_clk <= 96_000_000 {
                2
            } else if sys_clk <= 128_000_000 {
                3
            } else if sys_clk <= 160_000_000 {
                4
            } else {
                panic!("sys_clk is too high")
            }
        }
        // VOS 2 range VCORE 1.15V - 1.26V
        VoltageScale::RANGE2 => {
            if sys_clk <= 30_000_000 {
                0
            } else if sys_clk <= 60_000_000 {
                1
            } else if sys_clk <= 90_000_000 {
                2
            } else if sys_clk <= 110_000_000 {
                3
            } else {
                panic!("sys_clk is too high")
            }
        }
        // VOS 3 range VCORE 1.05V - 1.15V
        VoltageScale::RANGE3 => {
            if sys_clk <= 24_000_000 {
                0
            } else if sys_clk <= 48_000_000 {
                1
            } else if sys_clk <= 55_000_000 {
                2
            } else {
                panic!("sys_clk is too high")
            }
        }
        VoltageScale::RANGE4 => {
            if sys_clk <= 12_000_000 {
                0
            } else if sys_clk <= 25_000_000 {
                1
            } else {
                panic!("sys_clk is too high")
            }
        }
    }
}

pub fn set_clock() {
    // check the clock requirement to determine the kernel clock
    // default kernel clock is 4Mhz
    let kernel_freq = unsafe {
        if CLOCK_REF.kernel_freq_160mhz > 0 {
            160_000_000
        } else if CLOCK_REF.kernel_freq_80mhz > 0 {
            80_000_000
        } else if CLOCK_REF.kernel_freq_64mhz > 0 {
            64_000_000
        } else if CLOCK_REF.kernel_freq_48mhz > 0 {
            48_000_000
        } else if CLOCK_REF.kernel_freq_32mhz > 0 {
            32_000_000
        } else if CLOCK_REF.kernel_freq_16mhz > 0 {
            16_000_000
        } else if CLOCK_REF.kernel_freq_4mhz > 0 {
            4_000_000
        } else {
            panic!("No kernel clock is set")
        }
    };
    // current only support for 160Mhz and 4Mhz
    set_cpu_freq(kernel_freq);
    info!("kernel freq: {}", kernel_freq);
    unsafe {
        if CLOCK_REF.hsi16 > 0 {
            // enable hsi16
            RCC.cr().modify(|w| w.set_hsion(true));
            while !RCC.cr().read().hsirdy() {}
        }
        // if CLOCK_REF.hse > 0 {
        if HSE_AVAILABLE {
            // enable hse
            RCC.cr().modify(|w| w.set_hseon(true));
            while !RCC.cr().read().hserdy() {}
        }
        if CLOCK_REF.hsi48 > 0 {
            // enable hsi48
            RCC.cr().modify(|w| w.set_hsi48on(true));
            while !RCC.cr().read().hsi48rdy() {}
        }
    }
    set_kernel_freq(kernel_freq);
}
