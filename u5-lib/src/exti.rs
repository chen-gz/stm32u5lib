use cortex_m::peripheral::NVIC;
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, signal::Signal};
use stm32_metapac::{exti, RCC};
pub use stm32_metapac::EXTI;

use crate::gpio::GpioPort;

macro_rules! define_exti_port {
    ($($name:ident: $port:ident, $line:expr),*) => {
        $(
            pub const $name: ExtiPort = ExtiPort {
                gpio: $port,
                line: $line,
                reg: stm32_metapac::EXTI,
            };
        )*
    };
}
pub struct ExtiPort {
    gpio: GpioPort,
    line: usize,
    reg: exti::Exti,
}

impl ExtiPort {
    pub fn exticr_from_port(&self) -> u8 {
        if self.gpio.port == stm32_metapac::GPIOA {
            0
        } else if self.gpio.port == stm32_metapac::GPIOB {
            1
        } else if self.gpio.port == stm32_metapac::GPIOC {
            2
        } else if self.gpio.port == stm32_metapac::GPIOD {
            3
        } else if self.gpio.port == stm32_metapac::GPIOE {
            4
        } else if self.gpio.port == stm32_metapac::GPIOF {
            5
        } else if self.gpio.port == stm32_metapac::GPIOG {
            6
        } else {
            panic!("not supported port");
        }
    }
    pub async fn wait_for_raising(&self) {
        RCC.apb3enr().modify(|v| v.set_syscfgen(true));
        self.gpio.setup();
        unsafe {
            // NVIC::unmask(stm32_metapac::Interrupt::EXTI2);
            NVIC::unmask(interrupt::EXTI2);
        }
        if self.line < 8 {
            self.reg.ftsr(0).modify(|v| v.set_line(self.line, true));
            // self.reg.rtsr(0).modify(|v| v.set_line(self.line, true));
            self.reg.imr(0).modify(|v| v.set_line(self.line, true));
            self.reg.emr(0).modify(|v| v.set_line(self.line, true));
        } else {
            self.reg.ftsr(1).modify(|v| v.set_line(self.line - 8, true));
            self.reg.imr(1).modify(|v| v.set_line(self.line - 8, true));
        }
        self.reg
            .exticr(self.line / 4)
            .modify(|v| v.set_exti(self.line % 4, self.exticr_from_port()));

        if self.line == 2 {
            EXTI2_SIGNAL.wait().await;
        }
    }
}
static EXTI2_SIGNAL: Signal<CriticalSectionRawMutex, u32> = Signal::new();
static mut EXTI2_SIGNAL_VALUE: u32 = 0;

use stm32_metapac::interrupt;
// define_exti_interrupt!(EXTI2, interrupt::EXTI2);
// macro_rules! define_exti_interrupt {
//     // ($($name:ident, $handler:ident),*) => {
//     ($($handler:ident, $line:expr),*) => {
//         $(
//             #[interrupt]
//             fn $handler() {
//                 unsafe {
//                     let stat = (((EXTI.fpr(0).read().0 >> $line) & 1) << 1)
//                         | (EXTI.rpr(0).read().0 >> $line) & 1;
//                     if EXTI2_SIGNAL.signaled() {
//                         EXTI2_SIGNAL.signal(EXTI2_SIGNAL_VALUE | stat);
//                         EXTI2_SIGNAL_VALUE |= stat;
//                     } else {
//                         EXTI2_SIGNAL.signal(stat);
//                         EXTI2_SIGNAL_VALUE = stat;
//                     }
//                     EXTI.fpr(0).write(|v| v.set_line($line, true));
//                     EXTI.rpr(0).write(|v| v.set_line($line, true));
//                 }
//             }
//         )*
//     };
// }
// define_exti_interrupt!(EXTI2, 2);
#[interrupt]
fn EXTI2(){
    unsafe {
        let stat = (((EXTI.fpr(0).read().0 >> 2) & 1) << 1)
            | (EXTI.rpr(0).read().0 >> 2) & 1;
        if EXTI2_SIGNAL.signaled() {
            EXTI2_SIGNAL.signal(EXTI2_SIGNAL_VALUE | stat);
            EXTI2_SIGNAL_VALUE |= stat;
        } else {
            EXTI2_SIGNAL.signal(stat);
            EXTI2_SIGNAL_VALUE = stat;
        }
        EXTI.fpr(0).write(|v| v.set_line(2, true));
        EXTI.rpr(0).write(|v| v.set_line(2, true));
    }
}


use crate::gpio::*;
define_exti_port!(
    EXTI2_PB2: GPIO_EXTI_PB2, 2
    // EXTI3_PB3: GPIO_EXTI_PB3, 3
);
