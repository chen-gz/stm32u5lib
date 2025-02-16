use stm32_metapac::timer::vals::*;
/// This file is used to control the timer.
/// This file is implement for STM32U5 TIM1 and TIM8
/// the clock source of the timer is the same as the PCLK2 in clock_tree
/// In my clock implementation, the PCLK2 is same as the core clock.
/// The TIM1 and TIM8 are not working in stop modes (deep sleep mode) and standby mode.
/// Should be careful the system clock may be changed depend on the system lode and the power mode.
/// This clock will be affected by the system clock.
use stm32_metapac::timer::TimAdv;
use stm32_metapac::timer::TimGp32;
use stm32_metapac::RCC;
// todo!("The deepsleep mode does not working when this timer is using.");

pub struct TimAdvIns {
    ins: TimAdv,
    // init: bool,
}
pub struct TimBasicIns {
    ins: TimGp32,
}

pub const TIM1: TimAdvIns = TimAdvIns {
    ins: stm32_metapac::TIM1,
    // init: false, // Some Timer setting are not allowed to change after the timer is enabled.
    //              // only capture and compare mode can be changed after the timer is enabled.
};

pub const TIM3: TimBasicIns = TimBasicIns {
    ins: stm32_metapac::TIM3,
    // init: false,
};

pub enum TimError {
    ReInitError,
}

pub struct Config {
    pub prescaler: u16, // the prescaler value. timer clock = core clock / (prescaler + 1)
    pub dir: Dir,
    pub cms: Cms,
    /// auto reload preload. Whether the auto reload register is buffered or not.
    /// If the auto reload preload is enabled, when writing new value to auto reload register (ARR),
    /// it does not take effect until the next update event.
    /// If the auto reload preload is disabled, when writing new value to auto reload register (ARR),
    /// it takes effect immediately.
    pub arpe: bool,
    /// repetition counter
    /// The repetition counter is defined how many counter overflow events are needed to generate an update event (UEV).
    pub rcr: u16,
    /// update event will generate when the counter reach the value of arr.
    /// The counter value is 0 -> arr (include 0 and arr).
    pub arr: u16,

    /// Set the source of tim_trgo signal
    pub mms: Mms,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            prescaler: 0,
            dir: Dir::UP,
            cms: Cms::EDGEALIGNED,
            arpe: false,      // auto reload preload disable
            rcr: 0,           // no repetition
            arr: 0, // counter value is 0 -> arr (include 0 and arr). So the tick is arr + 1;
            mms: Mms::UPDATE, // default is update event (UEV). Generate when the counter reach the value of arr.
        }
    }
}

impl TimAdvIns {
    pub fn get_frequency() -> u32 {
        todo!("get the frequency of the timer.");
        // return 160_000_000 / (prescaler + 1) as u32;
    }
    pub fn set_clock(&self) {
        // STM32 U5 TIM 1,8, 15, 16 and 17 use apb2 clock.
        // It can be apb2 clock \times 1 or \times (not sure where to set it. But found this in the "clock tree" in the reference manual)
        //
        RCC.apb2enr().modify(|v| v.set_tim1en(true)); // refer ot file header
        self.ins.cr1().modify(|v| v.set_cen(false)); // disable counter for configuration
                                                     // current suppose the main clock is 160MHz
    }

    pub fn init(&self, config: Config) -> Result<(), TimError> {
        // set_clock source
        // counter value is 0 -> arr (include 0 and arr).
        // So the tick is arr + 1;
        self.set_clock();
        self.ins.cr1().modify(|v| {
            v.set_dir(config.dir);
            v.set_cms(config.cms);
            v.set_arpe(config.arpe);
        });
        self.ins.psc().write_value(config.prescaler);
        self.ins.rcr().modify(|v| v.set_rep(config.rcr));
        self.ins.cr1().modify(|v| v.set_cen(true)); // enable counter
        self.ins.cr2().modify(|v| v.set_mms(config.mms));
        Ok(())
    }

    pub fn set_pwm(&self, ch: u8, sum: u16, low: u16) {
        // We use pwm mode 1 and the counter is upcounting.
        // The output is high hwne `cnt < timccr1` and low when `cnt >= timccr1`
        // Example: arr = 7, and timccr1 = 4; then we have high = 4 and low = 4;
        // 160MHz --> 20MHz = 8
        // arr = 160 and timccr = 80 then the output clock is 1MHz
        let arr = sum;
        self.ins.arr().write(|v| v.0 = arr as u32);
        // self.ins.ccr(0).write(|v| v.0 = (duty_cycle * arr as f32) as u32);
        // self.ins.ccr(0).write(|v| v.0 = low);
        self.ins.ccr((ch - 1) as _).write(|v| v.0 = low as u32);
    }

    pub fn enable_output(
        &self,
        // channel from 1 to 4
        channel: u8,
    ) {
        let ch = channel - 1;
        if ch <= 1 {
            self.ins.ccmr_output(0).modify(|v| {
                v.set_ccs(ch as _, CcmrOutputCcs::OUTPUT);
                v.set_ocm(ch as _, Ocm::PWMMODE1);
            });
        } else {
            // channel 3 and 4
            self.ins.ccmr_output(1).modify(|v| {
                v.set_ccs(ch as usize - 2, CcmrOutputCcs::OUTPUT);
                v.set_ocm(ch as usize - 2, Ocm::PWMMODE1);
            });
        }
        self.ins.ccer().modify(|v| v.set_cce(ch as _, true));
        self.ins.bdtr().modify(|v| {
            v.set_moe(true);
            v.set_aoe(true);
        });
    }
    pub fn disable_output(
        &self,
        channel: u8,
    ) {
        let ch = channel - 1;
        self.ins.ccer().modify(|v| v.set_cce(ch as _, false));
    }
}

impl TimBasicIns {
    pub fn get_frequency() -> u32 {
        todo!("get the frequency of the timer.");
        // return 160_000_000 / (prescaler + 1) as u32;
    }
    pub fn set_clock(&self) {
        // STM32 U5 TIM 1,8, 15, 16 and 17 use apb2 clock.
        // It can be apb2 clock \times 1 or \times (not sure where to set it. But found this in the "clock tree" in the reference manual)
        //
        RCC.apb1enr1().modify(|v| v.set_tim3en(true)); // refer ot file header
        self.ins.cr1().modify(|v| v.set_cen(false)); // disable counter for configuration
                                                     // current suppose the main clock is 160MHz
    }

    pub fn init(&self, config: Config) -> Result<(), TimError> {
        // set_clock source
        // counter value is 0 -> arr (include 0 and arr).
        // So the tick is arr + 1;
        self.set_clock();
        self.ins.cr1().modify(|v| {
            v.set_dir(config.dir);
            v.set_cms(config.cms);
            v.set_arpe(config.arpe);
        });
        self.ins.psc().write_value( config.prescaler);
        // self.ins.rcr().modify(|v| v.set_rep(config.rcr));
        self.ins.arr().write(|_v| config.arr);
        self.ins.cr1().modify(|v| v.set_cen(true)); // enable counter
        self.ins.cr2().modify(|v| v.set_mms(config.mms));
        Ok(())
    }

    pub fn set_pwm(&self, ch: u8, sum: u16, low: u16) {
        // We use pwm mode 1 and the counter is upcounting.
        // The output is high hwne `cnt < timccr1` and low when `cnt >= timccr1`
        // Example: arr = 7, and timccr1 = 4; then we have high = 4 and low = 4;
        // 160MHz --> 20MHz = 8
        // arr = 160 and timccr = 80 then the output clock is 1MHz
        let arr = sum;
        // self.ins.arr().write(|v| v.0 = arr as u32);
        // self.ins.arr().modify(|v| v = arr as &mut u32);
        defmt::info!("arr: {}", arr);
        self.ins.arr().write(|v| *v = arr as u32);
        // self.ins.ccr(0).write(|v| v.0 = (duty_cycle * arr as f32) as u32);
        // self.ins.ccr(0).write(|v| v.0 = low);
        // self.ins.ccr((ch - 1) as _).write(|_| low as u32);
        self.ins.ccr((ch - 1) as _).write(|v| *v = low as u32);
        defmt::info!("arr: {}", self.ins.arr().read());
        // ccr
        defmt::info!("ccr: {}", self.ins.ccr(0).read());
    }

    pub fn enable_output(
        &self,
        // channel from 1 to 4
        channel: u8,
    ) {
        let ch = channel - 1;
        if ch <= 1 {
            self.ins.ccmr_output(0).modify(|v| {
                v.set_ccs(ch as _, CcmrOutputCcs::OUTPUT);
                v.set_ocm(ch as _, Ocm::PWMMODE1);
            });
        } else {
            // channel 3 and 4
            self.ins.ccmr_output(1).modify(|v| {
                v.set_ccs(ch as usize - 2, CcmrOutputCcs::OUTPUT);
                v.set_ocm(ch as usize - 2, Ocm::PWMMODE1);
            });
        }
        self.ins.ccer().modify(|v| v.set_cce(ch as _, true));
        // self.ins.bdtr().modify(|v| {
        //     v.set_moe(true);
        //     v.set_aoe(true);
        // });
        self.ins.cr1().modify(|v| v.set_cen(true)); // enable counter
    }
    pub fn disable_output(
        &self,
        channel: u8,
    ) {
        let ch = channel - 1;
        self.ins.ccer().modify(|v| v.set_cce(ch as _, false));
    }
}
