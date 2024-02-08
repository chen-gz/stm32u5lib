// use crate::clock::gg::delay_tick;
// use stm32_metapac::LPTIM1;
// pub struct LptimIns {
//     // ins: ,
// }
// pub const TIM1: TimIns = TimIns {
//     ins: stm32_metapac::TIM1,
// };

// use crate::gpio::gg::GpioPort;
// use crate::gpio::*;
// use stm32_metapac::timer::vals::*;
// impl TimIns {
//     // counter value is 0 -> arr (include 0 and arr).
//     // So the tick is arr + 1;
//     pub fn init(&self, freq: u32) {
//         self.ins.cr1().modify(|v| v.set_cen(false)); // disable counter
//         self.ins.cr1().modify(|v| {
//             v.set_dir(Dir::UP);
//             v.set_cms(Cms::EDGEALIGNED);
//             v.set_arpe(Arpe::DISABLED); // auto reload preload disable
//         });
//         self.ins.psc().write(|v| v.0 = 0); // no prescaler
//         self.ins.rcr().modify(|v| v.set_rep(0)); // no repetition
//         self.ins.dier().modify(|v| v.set_uie(false)); // disable interrupt
//         self.ins.egr().write(|v| v.set_ug(true)); // generate update event to reload the registers immediately
//         self.ins.cr1().modify(|v| v.set_cen(true)); // enable counter

//         // the lock of the timer is same as the PCLK
//     }
//     pub fn set_pwm(&self, freq: u32) {
//         // We use pwm mode 1 and the counter is upcounting.
//         // The output is high hwne `cnt < timccr1` and low when `cnt >= timccr1`
//         // Example: arr = 7, and timccr1 = 4; then we have high = 4 and low = 4;
//         // 160MHz --> 20MHz = 8
//         self.ins.arr().write(|v| v.0 = 7);
//         self.ins.ccr(0).write(|v| v.0 = 4);
//     }
//     pub fn enable_output(&self) {
//         self.ins
//             .ccmr_output(0)
//             .modify(|v| v.set_ocm(0, Ocm::PWMMODE1));
//         self.ins.ccer().modify(|v| v.set_cce(0, true));
//     }
// }
