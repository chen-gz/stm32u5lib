#![allow(unused)]
pub mod gg {

    macro_rules! define_gpio_port {
    ($($name:ident: $port:ident, $pin:expr),*) => {
        $(
            pub const $name: GpioPort = GpioPort {
                port: $port,
                pin: $pin,
            };
        )*
    };
}

    pub use stm32_metapac::gpio::vals::Moder;
    pub use stm32_metapac::gpio::vals::Odr;
    pub use stm32_metapac::gpio::vals::Ot;
    pub use stm32_metapac::gpio::vals::Pupdr;
    use stm32_metapac::gpio::Gpio;
    use stm32_metapac::{GPIOA, GPIOB, GPIOC, GPIOD, GPIOE};

    pub struct GpioPort {
        port: Gpio,
        pin: usize,
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
        pub fn setup(&self, moder: Moder, otypte: Ot, pupd: Pupdr, alt: u8) {
            self.port.moder().modify(|v| v.set_moder(self.pin, moder));
            self.port.otyper().modify(|v| v.set_ot(self.pin, otypte));
            self.port.pupdr().modify(|v| v.set_pupdr(self.pin, pupd));
            if self.pin < 8 {
                self.port.afr(0).modify(|v| v.set_afr(self.pin, alt));
            } else {
                self.port.afr(1).modify(|v| v.set_afr(self.pin - 8, alt));
            }
        }
    }
    define_gpio_port!(
    PC0: GPIOC, 0, PC1: GPIOC, 1, PC2: GPIOC, 2, PC3: GPIOC, 3,
    PC4: GPIOC, 4, PC5: GPIOC, 5, PC6: GPIOC, 6, PC7: GPIOC, 7,
    PC8: GPIOC, 8, PC9: GPIOC, 9, PC10: GPIOC, 10, PC11: GPIOC, 11,
    PC12: GPIOC, 12, PC13: GPIOC, 13, PC14: GPIOC, 14, PC15: GPIOC, 15

    );

    //  define_gpio_port!(
    //      PA0: 0, PA1: 1, PA2: 2, PA3: 3,
    //      PA4: 4, PA5: 5, PA6: 6, PA7: 7,
    //      PA8: 8, PA9: 9, PA10: 10, PA11: 11,
    //      PA12: 12, PA13: 13, PA14: 14, PA15: 15,

    //      PB0: 0, PB1: 1, PB2: 2, PB3: 3,
    //      PB4: 4, PB5: 5, PB6: 6, PB7: 7,
    //      PB8: 8, PB9: 9, PB10: 10, PB11: 11,
    //      PB12: 12, PB13: 13, PB14: 14, PB15: 15,

    //      PC0: 0, PC1: 1, PC2: 2, PC3: 3,
    //      PC4: 4, PC5: 5, PC6: 6, PC7: 7,
    //      PC8: 8, PC9: 9, PC10: 10, PC11: 11,
    //      PC12: 12, PC13: 13, PC14: 14, PC15: 15,

    //      PD0: 0, PD1: 1, PD2: 2, PD3: 3,
    //      PD4: 4, PD5: 5, PD6: 6, PD7: 7,
    //      PD8: 8, PD9: 9, PD10: 10, PD11: 11,
    //      PD12: 12, PD13: 13, PD14: 14, PD15: 15,

    //      PE0: 0, PE1: 1, PE2: 2, PE3: 3,
    //      PE4: 4, PE5: 5, PE6: 6, PE7: 7,
    //      PE8: 8, PE9: 9, PE10: 10, PE11: 11,
    //      PE12: 12, PE13: 13, PE14: 14, PE15: 15
    //  );
}
