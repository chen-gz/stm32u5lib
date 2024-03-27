use stm32_metapac::RCC;
/// This file is used to control the timer.
/// This file is implement for STM32U5 TIM1 and TIM8
/// the clock source of the timer is the same as the PCLK2 in clock_tree
/// In my clock implementation, the PCLK2 is same as the core clock.
/// The TIM1 and TIM8 are not working in stop modes (deep sleep mode) and standby mode.
/// Should be careful the system clock may be changed depend on the system lode and the power mode.
/// This clock will be affected by the system clock.
use stm32_metapac::timer::TimAdv;
use stm32_metapac::timer::vals::*;

pub struct TimIns {
    ins: TimAdv,
    init: bool,
}

pub static mut TIM1: TimIns = TimIns {
    ins: stm32_metapac::TIM1,
    init: false,  // Some Timer setting are not allowed to change after the timer is enabled.
    // only capture and compare mode can be changed after the timer is enabled.
};

pub enum TimError {
    ReInitError,
}

pub struct Config {
    pub prescaler: u32,
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
            arpe: false,
            rcr: 0,
            arr: 0,
            mms: Mms::UPDATE, // default is update event (UEV). Generate when the counter reach the value of arr.
        }
    }
}


impl TimIns {
    pub fn init(&mut self, config: Config) -> Result<(), TimError> {
        // counter value is 0 -> arr (include 0 and arr).
        // So the tick is arr + 1;
        if self.init {
            return Err(TimError::ReInitError);
        }
        self.init = true;
        RCC.apb2enr().modify(|v| v.set_tim1en(true)); // refer ot file header
        self.ins.cr1().modify(|v| v.set_cen(false));  // disable counter for configuration
        self.ins.cr1().modify(|v| {
            v.set_dir(config.dir);
            v.set_cms(config.cms);
            v.set_arpe(false); // auto reload preload disable
        });
        self.ins.psc().write_value(0); // no prescaler
        self.ins.rcr().modify(|v| v.set_rep(config.rcr));
        self.ins.cr1().modify(|v| v.set_cen(true)); // enable counter
        self.ins.cr2().modify(|v| v.set_mms(config.mms));
        Ok(())
    }


    // pub fn set_pwm(&self, freq: u32) {
    //     // We use pwm mode 1 and the counter is upcounting.
    //     // The output is high hwne `cnt < timccr1` and low when `cnt >= timccr1`
    //     // Example: arr = 7, and timccr1 = 4; then we have high = 4 and low = 4;
    //     // 160MHz --> 20MHz = 8
    //     self.ins.arr().write(|v| v.0 = 7);
    //     self.ins.ccr(0).write(|v| v.0 = 4);
    // }
    // pub fn enable_output(&self) {
    //     self.ins
    //         .ccmr_output(0)
    //         .modify(|v| v.set_ocm(0, Ocm::PWMMODE1));
    //     self.ins.ccer().modify(|v| v.set_cce(0, true));
    // }
}