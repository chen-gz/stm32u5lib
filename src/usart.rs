#![allow(unused)]
use stm32_metapac::usart::vals::{M0, M1, Over8, Stop};
use crate::{clock, gpio};
pub struct UsartPort {
    port: stm32_metapac::usart::Usart,
}

pub const USART1: UsartPort = UsartPort {
    port: stm32_metapac::USART1
};
const USART_CLOCK: u32 = 16_000_000; // default use HSI16

impl UsartPort {
    /// current use default configuration, 115200 baudrate, 8 bit data, 1 stop bit, no parity
    pub fn setup(&self, gpio_tx: gpio::GpioPort, gpio_rx: gpio::GpioPort) {
        // todo!("enable usart1 clock and setup gpio for usart1");
        gpio_tx.setup();
        gpio_rx.setup();
        clock::set_usart_clock();
        // todo!("enable interrupt ");
        // self.port
        self.port.cr1().modify(|v| {
            v.set_m0(M0::BIT8);
            v.set_m1(M1::M0);
            v.set_pce(false);
            v.set_over8(Over8::OVERSAMPLING16); // oversampling by 16

            v.set_ue(true);
            v.set_te(true);
            v.set_re(true);
        });
        self.port.cr2().modify(|v| {
            v.set_stop(Stop::STOP1);   // 1 stop bit
        });
        self.port.cr3().modify(|v| {
            // v.set_owr_ddr(true);
        });
        self.port.brr().write(|v| {
            v.set_brr((USART_CLOCK / 115200) as u16);
        });
    }
    pub fn send(&self, data: &[u8]) {
        for &c in data {
            while self.port.isr().read().txe() == false {}
            self.port.tdr().write(|v| {
                v.set_dr(c as u16);
            });
        }
    }
    pub fn recv(&self) -> u8 {
        todo!();
    }
}