#![allow(unused)]
pub mod gg {
    use core::panic;
    pub use stm32_metapac::gpio::vals::Moder;
    pub use stm32_metapac::gpio::vals::Odr;
    pub use stm32_metapac::gpio::vals::Ot;
    pub use stm32_metapac::gpio::vals::Pupdr;
    use stm32_metapac::gpio::Gpio;
    use stm32_metapac::{GPIOA, GPIOB, GPIOC, GPIOD, GPIOE};

    macro_rules! define_gpio_port {
        ($($name:ident: $port:ident, $pin:expr),*) => {
            $(
                pub const $name: GpioPort = GpioPort {
                    port: $port,
                    pin: $pin,
                    alt_func: 0,
                    mode: Moder::OUTPUT,
                    ot: Ot::PUSHPULL,
                    pupd: Pupdr::FLOATING,
                };
            )*
        };
    }
    macro_rules! define_gpio_port_alt {
        ($($name:ident: $port:ident, $pin:expr, $alt_func: expr, $mode:expr, $ot : expr, $pupd:expr),*) => {
            $(pub const $name: GpioPort = GpioPort {
                port: $port,
                pin: $pin,
                alt_func: $alt_func,
                mode: $mode,
                ot: $ot,
                pupd: $pupd,
            };)*
        };

    }

    pub struct GpioPort {
        port: Gpio,
        pin: usize,
        /// Alternate function
        alt_func: u8,
        mode: Moder,
        ot: Ot,
        pupd: Pupdr,
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
        pub fn setup(&self) {
            //  moder: Moder, otypte: Ot, pupd: Pupdr) {
            self.port
                .moder()
                .modify(|v| v.set_moder(self.pin, self.mode));
            self.port.otyper().modify(|v| v.set_ot(self.pin, self.ot));
            self.port
                .pupdr()
                .modify(|v| v.set_pupdr(self.pin, self.pupd));
            self.port.ospeedr().modify(|v| {
                v.set_ospeedr(self.pin, stm32_metapac::gpio::vals::Ospeedr::VERYHIGHSPEED)
            });
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
        PB0: GPIOB, 0,PB1: GPIOB, 1, PB2: GPIOB, 2, PB3: GPIOB, 3,
        PB4: GPIOB, 4,PB5: GPIOB, 5, PB6: GPIOB, 6, PB7: GPIOB, 7,
        PB8: GPIOB, 8,PB9: GPIOB, 9, PB10: GPIOB, 10, PB11: GPIOB, 11,
        PB12: GPIOB, 12,PB13: GPIOB, 13, PB14: GPIOB, 14, PB15: GPIOB, 15,

        PC0: GPIOC, 0,PC1: GPIOC, 1, PC2: GPIOC, 2, PC3: GPIOC, 3,
        PC4: GPIOC, 4,PC5: GPIOC, 5, PC6: GPIOC, 6, PC7: GPIOC, 7,
        PC8: GPIOC, 8,PC9: GPIOC, 9, PC10: GPIOC, 10, PC11: GPIOC, 11,
        PC12: GPIOC, 12,PC13: GPIOC, 13, PC14: GPIOC, 14, PC15: GPIOC, 15

    );
    define_gpio_port_alt!(
        I2C1_SCL_PB6: GPIOB, 6, 4, Moder::ALTERNATE, Ot::OPENDRAIN, Pupdr::PULLUP,
        I2C1_SDA_PB7: GPIOB, 7, 4, Moder::ALTERNATE, Ot::OPENDRAIN, Pupdr::PULLUP,
        I2C1_SCL_PB8: GPIOB, 8, 4, Moder::ALTERNATE, Ot::OPENDRAIN, Pupdr::PULLUP,
        I2C1_SDA_PB9: GPIOB, 9, 4, Moder::ALTERNATE, Ot::OPENDRAIN, Pupdr::PULLUP,

        I2C3_SCL_PC0: GPIOC, 0, 4, Moder::ALTERNATE, Ot::OPENDRAIN, Pupdr::PULLUP,
        I2C3_SDA_PB4: GPIOB, 4, 4, Moder::ALTERNATE, Ot::OPENDRAIN, Pupdr::PULLUP
    );
}
