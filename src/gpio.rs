#![allow(unused)]
use core::panic;
pub use stm32_metapac::gpio::vals::Moder;
pub use stm32_metapac::gpio::vals::Odr;
pub use stm32_metapac::gpio::vals::Ot;
pub use stm32_metapac::gpio::vals::Pupdr;
use stm32_metapac::gpio::Gpio;
use stm32_metapac::{GPIOA, GPIOB, GPIOC, GPIOD, GPIOE, GPIOF};
use core::ptr as prt;

use crate::clock;
use crate::clock::delay_ms;

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
    pub port: Gpio,
    pub pin: usize,
    /// Alternate function
    pub alt_func: u8,
    pub mode: Moder,
    pub ot: Ot,
    pub pupd: Pupdr,
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
        // enable the clock
        clock::set_gpio_clock(self.port);
        self.port.otyper().modify(|v| v.set_ot(self.pin, self.ot));
        self.port
            .pupdr()
            .modify(|v| v.set_pupdr(self.pin, self.pupd));
        self.port
            .ospeedr()
            .modify(|v| v.set_ospeedr(self.pin, stm32_metapac::gpio::vals::Ospeedr::VERYHIGHSPEED));
        if self.pin < 8 {
            self.port
                .afr(0)
                .modify(|v| v.set_afr(self.pin, self.alt_func));
        } else {
            self.port
                .afr(1)
                .modify(|v| v.set_afr(self.pin - 8, self.alt_func));
        }
        self.port
            .moder()
            .modify(|v| v.set_moder(self.pin, self.mode));
    }
}
define_gpio_port!(
    PA0: GPIOA, 0,PA1: GPIOA, 1, PA2: GPIOA, 2, PA3: GPIOA, 3,
    PA4: GPIOA, 4,PA5: GPIOA, 5, PA6: GPIOA, 6, PA7: GPIOA, 7,
    PA8: GPIOA, 8,PA9: GPIOA, 9, PA10: GPIOA, 10, PA11: GPIOA, 11,
    PA12: GPIOA, 12,PA13: GPIOA, 13, PA14: GPIOA, 14, PA15: GPIOA, 15,
    PB0: GPIOB, 0,PB1: GPIOB, 1, PB2: GPIOB, 2, PB3: GPIOB, 3,
    PB4: GPIOB, 4,PB5: GPIOB, 5, PB6: GPIOB, 6, PB7: GPIOB, 7,
    PB8: GPIOB, 8,PB9: GPIOB, 9, PB10: GPIOB, 10, PB11: GPIOB, 11,
    PB12: GPIOB, 12,PB13: GPIOB, 13, PB14: GPIOB, 14, PB15: GPIOB, 15,

    PC0: GPIOC, 0,PC1: GPIOC, 1, PC2: GPIOC, 2, PC3: GPIOC, 3,
    PC4: GPIOC, 4,PC5: GPIOC, 5, PC6: GPIOC, 6, PC7: GPIOC, 7,
    PC8: GPIOC, 8,PC9: GPIOC, 9, PC10: GPIOC, 10, PC11: GPIOC, 11,
    PC12: GPIOC, 12,PC13: GPIOC, 13, PC14: GPIOC, 14, PC15: GPIOC, 15,

    PD0: GPIOD, 0,PD1: GPIOD, 1, PD2: GPIOD, 2, PD3: GPIOD, 3,
    PD4: GPIOD, 4,PD5: GPIOD, 5, PD6: GPIOD, 6, PD7: GPIOD, 7,
    PD8: GPIOD, 8,PD9: GPIOD, 9, PD10: GPIOD, 10, PD11: GPIOD, 11,
    PD12: GPIOD, 12,PD13: GPIOD, 13, PD14: GPIOD, 14, PD15: GPIOD, 15

);
define_gpio_port_alt!(
    I2C1_SCL_PB6: GPIOB, 6, 4, Moder::ALTERNATE, Ot::OPENDRAIN, Pupdr::PULLUP,
    I2C1_SDA_PB7: GPIOB, 7, 4, Moder::ALTERNATE, Ot::OPENDRAIN, Pupdr::PULLUP,
    I2C1_SCL_PB8: GPIOB, 8, 4, Moder::ALTERNATE, Ot::OPENDRAIN, Pupdr::PULLUP,
    I2C1_SDA_PB9: GPIOB, 9, 4, Moder::ALTERNATE, Ot::OPENDRAIN, Pupdr::PULLUP,
    I2C2_SDA_PF0: GPIOF, 0, 4, Moder::ALTERNATE, Ot::OPENDRAIN, Pupdr::FLOATING,
    I2C2_SCL_PF1: GPIOF, 1, 4, Moder::ALTERNATE, Ot::OPENDRAIN, Pupdr::FLOATING,
    I2C2_SDA_PB10: GPIOB, 10, 4, Moder::ALTERNATE, Ot::OPENDRAIN, Pupdr::PULLUP,
    I2C2_SCL_PB11: GPIOB, 11, 4, Moder::ALTERNATE, Ot::OPENDRAIN, Pupdr::PULLUP,

    I2C3_SCL_PC0: GPIOC, 0, 4, Moder::ALTERNATE, Ot::OPENDRAIN, Pupdr::PULLUP,
    I2C3_SDA_PB4: GPIOB, 4, 4, Moder::ALTERNATE, Ot::OPENDRAIN, Pupdr::PULLUP,

    DCMI_D0_PA9: GPIOA, 9, 5, Moder::ALTERNATE, Ot::PUSHPULL, Pupdr::FLOATING,
    DCMI_D1_PA10: GPIOA, 10, 5, Moder::ALTERNATE, Ot::PUSHPULL, Pupdr::FLOATING,
    DCMI_D2_PE0: GPIOE, 0, 10, Moder::ALTERNATE, Ot::PUSHPULL, Pupdr::FLOATING,
    DCMI_D3_PE1: GPIOE, 1, 10, Moder::ALTERNATE, Ot::PUSHPULL, Pupdr::FLOATING,
    DCMI_D4_PE4: GPIOE, 4, 10, Moder::ALTERNATE, Ot::PUSHPULL, Pupdr::FLOATING,
    DCMI_D6_PE5: GPIOE, 5, 10, Moder::ALTERNATE, Ot::PUSHPULL, Pupdr::FLOATING,
    DCMI_D7_PE6: GPIOE, 6, 10, Moder::ALTERNATE, Ot::PUSHPULL, Pupdr::FLOATING,
    DCMI_PIXCLK_PD9: GPIOD, 9, 10, Moder::ALTERNATE, Ot::PUSHPULL, Pupdr::FLOATING,

    DCMI_D0_PC6: GPIOC, 6, 10, Moder::ALTERNATE, Ot::PUSHPULL, Pupdr::FLOATING,
    DCMI_D1_PC7: GPIOC, 7, 10, Moder::ALTERNATE, Ot::PUSHPULL, Pupdr::FLOATING,
    DCMI_D2_PC8: GPIOC, 8, 10, Moder::ALTERNATE, Ot::PUSHPULL, Pupdr::FLOATING,
    DCMI_D3_PC9: GPIOC, 9, 4, Moder::ALTERNATE, Ot::PUSHPULL, Pupdr::FLOATING,
    DCMI_D4_PC11: GPIOC, 11, 10, Moder::ALTERNATE, Ot::PUSHPULL, Pupdr::FLOATING,
    DCMI_D5_PB6: GPIOB, 6, 10, Moder::ALTERNATE, Ot::PUSHPULL, Pupdr::FLOATING,
    DCMI_D6_PB8: GPIOB, 8, 10, Moder::ALTERNATE, Ot::PUSHPULL, Pupdr::FLOATING,
    DCMI_D7_PB9: GPIOB, 9, 10, Moder::ALTERNATE, Ot::PUSHPULL, Pupdr::FLOATING,
    DCMI_VSYNC_PB7: GPIOB, 7, 10, Moder::ALTERNATE, Ot::PUSHPULL, Pupdr::FLOATING,
    DCMI_HSYNC_PA4: GPIOA, 4, 10, Moder::ALTERNATE, Ot::PUSHPULL, Pupdr::FLOATING,
    DCMI_PIXCLK_PA6: GPIOA, 6, 4, Moder::ALTERNATE, Ot::PUSHPULL, Pupdr::FLOATING,

    SDMMC2_CK_PC1: GPIOC, 1, 12, Moder::ALTERNATE, Ot::PUSHPULL, Pupdr::FLOATING,
    SDMMC2_CMD_PA0: GPIOA, 0, 12, Moder::ALTERNATE, Ot::PUSHPULL, Pupdr::FLOATING,
    SDMMC2_CK_PD6: GPIOD, 6, 11, Moder::ALTERNATE, Ot::PUSHPULL, Pupdr::FLOATING,
    SDMMC2_CMD_PD7: GPIOD, 7, 11, Moder::ALTERNATE, Ot::PUSHPULL, Pupdr::PULLUP,
    SDMMC2_D0_PB14: GPIOB, 14, 12, Moder::ALTERNATE, Ot::PUSHPULL, Pupdr::FLOATING,

    SDMMC2_D1_PB15: GPIOB, 15, 12, Moder::ALTERNATE, Ot::PUSHPULL, Pupdr::FLOATING,
    SDMMC2_D2_PB3: GPIOB, 3, 12, Moder::ALTERNATE, Ot::PUSHPULL, Pupdr::FLOATING,
    SDMMC2_D3_PB4: GPIOB, 4, 12, Moder::ALTERNATE, Ot::PUSHPULL, Pupdr::FLOATING,
    SDMMC2_D4_PB8: GPIOB, 8, 12, Moder::ALTERNATE, Ot::PUSHPULL, Pupdr::FLOATING,
    SDMMC2_D5_PB9: GPIOB, 9, 12, Moder::ALTERNATE, Ot::PUSHPULL, Pupdr::FLOATING,
    SDMMC2_D6_PC6: GPIOC, 6, 11, Moder::ALTERNATE, Ot::PUSHPULL, Pupdr::FLOATING,
    SDMMC2_D7_PC7: GPIOC, 7, 11, Moder::ALTERNATE, Ot::PUSHPULL, Pupdr::FLOATING,


    GPIO_MCO_PA8: GPIOA, 8, 0, Moder::ALTERNATE, Ot::PUSHPULL, Pupdr::FLOATING,

    GPIO_EXTI_PB2: GPIOB, 2, 0, Moder::INPUT, Ot::OPENDRAIN, Pupdr::PULLUP,
    GPIO_EXTI_PB3: GPIOB, 3, 0, Moder::INPUT, Ot::OPENDRAIN, Pupdr::FLOATING,
    GPIO_EXTI_PC13: GPIOC, 13, 0, Moder::INPUT, Ot::OPENDRAIN, Pupdr::FLOATING,

    USB_DM_PA11: GPIOA, 11, 10, Moder::ALTERNATE, Ot::PUSHPULL, Pupdr::FLOATING,
    USB_DP_PA12: GPIOA, 12, 10, Moder::ALTERNATE, Ot::PUSHPULL, Pupdr::FLOATING,

    USART_TX_PA9: GPIOA, 9, 7, Moder::ALTERNATE, Ot::PUSHPULL, Pupdr::FLOATING,
    USART_RX_PA10: GPIOA, 10, 7, Moder::ALTERNATE, Ot::PUSHPULL, Pupdr::FLOATING,
    ADC1_IN3_PC2: GPIOC, 2, 0, Moder::ANALOG, Ot::PUSHPULL, Pupdr::FLOATING,
    ADC1_IN1_PC0: GPIOC, 0, 0, Moder::ANALOG, Ot::PUSHPULL, Pupdr::FLOATING,
    ADC1_IN6_PC0: GPIOC, 0, 0, Moder::ANALOG, Ot::PUSHPULL, Pupdr::FLOATING,
    ADC1_IN2_PC1: GPIOC, 1, 0, Moder::ANALOG, Ot::PUSHPULL, Pupdr::FLOATING,
    ADC1_IN5_PA0: GPIOA, 0, 0, Moder::ANALOG, Ot::PUSHPULL, Pupdr::FLOATING

);
