//! USART Driver with Async Support and Separated TX/RX Wakers
#![allow(unused)]

use core::default;
use core::task::Poll;
use crate::{clock, gpio::{self, USART1_RX_PINS, USART1_TX_PINS}, hal};
use defmt::todo;
use stm32_metapac::{
    common::R,
    usart::vals::{Over8, Stop, M0, M1},
};
use cortex_m::peripheral::NVIC;
use embassy_sync::waitqueue::AtomicWaker;
use gpio::GpioPort;
use stm32_metapac::interrupt;

pub struct Usart {
    port: stm32_metapac::usart::Usart,
    port_num: u8,
}

const USART_CLOCK: u32 = 16_000_000; // default use HSI16

pub struct Config {
    port_num: u8,
    gpio_tx: gpio::GpioPort,
    gpio_rx: gpio::GpioPort,
    pub baudrate: u32,
    pub data_bits: u8,
    pub stop_bits: u8,
    pub parity: u8,
    pub flow_control: u8,
}

impl default::Default for Config {
    fn default() -> Self {
        Config {
            port_num: 1,
            gpio_tx: gpio::USART_TX_PA9,
            gpio_rx: gpio::USART_RX_PA10,
            baudrate: 115200,
            data_bits: 8,
            stop_bits: 1,
            parity: 0,
            flow_control: 0,
        }
    }
}

fn port_num_to_usart(port_num: u8) -> stm32_metapac::usart::Usart {
    match port_num {
        1 => stm32_metapac::USART1,
        2 => stm32_metapac::USART2,
        3 => stm32_metapac::USART3,
        _ => panic!("invalid port number"),
    }
}

#[derive(core::fmt::Debug)]
pub enum UsartError {
    TAKEN,
    BufferOverflow,
    Disabled,
}

const NEW_WAKER: AtomicWaker = AtomicWaker::new();

static mut RX_WAKERS: [AtomicWaker; 8] = [NEW_WAKER; 8];
static mut TX_WAKERS: [AtomicWaker; 8] = [NEW_WAKER; 8];
static mut TAKEN: [bool; 8] = [false; 8];

fn pin_to_port(tx: &gpio::GpioPort, rx: &gpio::GpioPort) -> u8 {
    if USART1_TX_PINS.contains(tx) && USART1_RX_PINS.contains(rx) {
        1
    } else {
        todo!()
    }
}

impl hal::Usart<GpioPort> for Usart {
    fn new(baudrate: u32, tx: GpioPort, rx: GpioPort) -> Result<Self, hal::UsartError> {
        let port_num = pin_to_port(&tx, &rx);

        if unsafe { TAKEN[port_num as usize] } {
            return Err(hal::UsartError::InitError);
        }
        unsafe {
            TAKEN[port_num as usize] = true;
        }

        tx.setup();
        rx.setup();
        clock::set_usart_clock();

        let port = port_num_to_usart(port_num);

        port.cr1().modify(|v| {
            v.set_m0(M0::BIT8);
            v.set_m1(M1::M0);
            v.set_pce(false);
            v.set_over8(Over8::OVERSAMPLING16);
            v.set_ue(true);
            v.set_te(true);
            v.set_re(true);
            // v.set_rxneie(true);
            // v.set_txeie(true);
            // v.set_tcie(true);
        });

        port.cr2().modify(|v| {
            v.set_stop(Stop::STOP1);
        });

        port.brr().write(|v| {
            v.set_brr((USART_CLOCK / baudrate) as u16);
        });

        unsafe {
            match port_num {
                1 => NVIC::unmask(interrupt::USART1),
                2 => NVIC::unmask(interrupt::USART2),
                3 => NVIC::unmask(interrupt::USART3),
                _ => {}
            }
        }

        Ok(Usart { port, port_num })
    }

    fn read(&self, data: &mut [u8]) -> Result<(), hal::UsartError> {
        for i in 0..data.len() {
            while self.port.isr().read().rxne() == false {}
            data[i] = self.port.rdr().read().dr() as u8;
        }
        Ok(())
    }

    async fn read_async(&self, data: &mut [u8]) -> Result<(), hal::UsartError> {
        for i in 0..data.len() {
            while self.port.isr().read().rxne() == false {
                core::future::poll_fn(|cx| {
                    unsafe { RX_WAKERS[self.port_num as usize].register(cx.waker()) };
                    self.port.cr1().modify(|v| v.set_rxneie(true));
                    Poll::Pending
                }).await;
            }
            data[i] = self.port.rdr().read().dr() as u8;
        }
        Ok(())
    }

    fn write(&self, data: &[u8]) -> Result<(), hal::UsartError> {
        for &c in data {
            while self.port.isr().read().txe() == false {}
            self.port.tdr().write(|v| v.set_dr(c as u16));
        }
        while self.port.isr().read().tc() == false {}
        Ok(())
    }

    async fn write_async(&self, data: &[u8]) -> Result<(), hal::UsartError> {
        for &c in data {
            while self.port.isr().read().txe() == false {
                core::future::poll_fn(|cx| {
                    unsafe { TX_WAKERS[self.port_num as usize].register(cx.waker()) };
                    self.port.cr1().modify(|v| v.set_txeie(true));
                    Poll::Pending
                }).await;
            }
            self.port.tdr().write(|v| v.set_dr(c as u16));
        }

        while self.port.isr().read().tc() == false {
            core::future::poll_fn(|cx| {
                unsafe { TX_WAKERS[self.port_num as usize].register(cx.waker()) };
                self.port.cr1().modify(|v| v.set_tcie(true));
                Poll::Pending
            }).await;
        }
        Ok(())
    }
}

#[interrupt]
fn USART1() {
    unsafe {
        let isr = stm32_metapac::USART1.isr().read();

        if isr.rxne() {
            RX_WAKERS[1].wake();
            stm32_metapac::USART1.cr1().modify(|v| v.set_rxneie(false));
        }

        if isr.txe() || isr.tc() {
            TX_WAKERS[1].wake();
            stm32_metapac::USART1.cr1().modify(|v| {
                v.set_txeie(false);
                v.set_tcie(false);
            });
        }
    }
}
