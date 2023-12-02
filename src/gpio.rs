#![allow(unused)]
pub mod gg {

    macro_rules! define_gpio_port {
        ($($name:ident: $port:ident, $pin:expr, $alt:expr),*) => {
            $(
                pub const $name: GpioPort = GpioPort {
                    port: $port,
                    pin: $pin,
                    alt_func: $alt,
                };
            )*
        };
    }

    use core::panic;

    pub use stm32_metapac::gpio::vals::Moder;
    pub use stm32_metapac::gpio::vals::Odr;
    pub use stm32_metapac::gpio::vals::Ot;
    pub use stm32_metapac::gpio::vals::Pupdr;
    use stm32_metapac::gpio::Gpio;
    use stm32_metapac::{GPIOA, GPIOB, GPIOC, GPIOD, GPIOE};

    pub struct GpioPort {
        port: Gpio,
        pin: usize,
        /// Alternate function
        alt_func: u8,
    }

    impl GpioPort {
        pub fn set_high(&self) {
            self.port.bsrr().write(|v| v.set_bs(self.pin, true))
        }
        pub fn set_low(&self) {
            self.port.bsrr().write(|v| v.set_br(self.pin, true))
        }
        pub fn toggle(&self) {
            self.port.odr().modify(|v| {
                v.set_odr(
                    self.pin,
                    if v.odr(self.pin) == Odr::HIGH {
                        Odr::LOW
                    } else {
                        Odr::HIGH
                    },
                );
            })
        }
        pub fn setup(&self, moder: Moder, otypte: Ot, pupd: Pupdr) {
            self.port.moder().modify(|v| v.set_moder(self.pin, moder));
            self.port.otyper().modify(|v| v.set_ot(self.pin, otypte));
            self.port.pupdr().modify(|v| v.set_pupdr(self.pin, pupd));
            if self.pin < 8 {
                self.port
                    .afr(0)
                    .modify(|v| v.set_afr(self.pin, self.alt_func));
            } else {
                self.port
                    .afr(1)
                    .modify(|v| v.set_afr(self.pin - 8, self.alt_func));
            }
        }
    }
    define_gpio_port!(
    PB0: GPIOB, 0, 0, PB1: GPIOB, 1,0, PB2: GPIOB, 2, 0, PB3: GPIOB, 3,0,
    PB4: GPIOB, 4, 0, PB5: GPIOB, 5,0, PB6: GPIOB, 6, 0, PB7: GPIOB, 7,0,
    PB8: GPIOB, 8, 0, PB9: GPIOB, 9,0, PB10: GPIOB, 10, 0, PB11: GPIOB, 11,0,
    PB12: GPIOB, 12, 0, PB13: GPIOB, 13,0, PB14: GPIOB, 14, 0, PB15: GPIOB, 15,0,

    PC0: GPIOC, 0, 0, PC1: GPIOC, 1,0, PC2: GPIOC, 2, 0, PC3: GPIOC, 3,0,
    PC4: GPIOC, 4, 0, PC5: GPIOC, 5,0, PC6: GPIOC, 6, 0, PC7: GPIOC, 7,0,
    PC8: GPIOC, 8, 0, PC9: GPIOC, 9,0, PC10: GPIOC, 10, 0, PC11: GPIOC, 11,0,
    PC12: GPIOC, 12, 0, PC13: GPIOC, 13,0, PC14: GPIOC, 14, 0, PC15: GPIOC, 15,0,

    // alternative functions
    I2C1_SCL_PB6: GPIOB, 6, 4,
    I2C1_SDA_PB7: GPIOB, 7, 4,
    I2C1_SCL_PB8: GPIOB, 8, 4,
    I2C1_SDA_PB9: GPIOB, 9, 4

    );
}
