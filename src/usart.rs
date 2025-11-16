//! USART Driver with Async Support and Separated TX/RX Wakers
#![allow(unused)]

use crate::dma::DmaChannel;
use crate::low_power::run_no_deep_sleep_async;
use crate::{
    clock,
    gpio::{self, USART1_RX_PINS, USART1_TX_PINS},
    hal,
};
use core::task::Poll;
use cortex_m::peripheral::NVIC;
use embassy_sync::waitqueue::AtomicWaker;
use gpio::GpioPort;
use stm32_metapac::interrupt;
use stm32_metapac::{
    common::R,
    usart::vals::{Over8, Stop, M0, M1},
};

pub struct Usart {
    port: stm32_metapac::usart::Usart,
    port_num: u8,
    use_dma: bool,
    dma: Option<DmaChannel>,
}

const USART_CLOCK: u32 = 16_000_000; // default use HSI16

#[derive(core::fmt::Debug)]
pub enum UsartError {
    TAKEN,
    BufferOverflow,
    Disabled,
    BusError,
}

static RX_WAKERS: [AtomicWaker; 8] = [const { AtomicWaker::new() }; 8];
static TX_WAKERS: [AtomicWaker; 8] = [const { AtomicWaker::new() }; 8];
static TAKEN: [core::sync::atomic::AtomicBool; 8] = [const { core::sync::atomic::AtomicBool::new(false) }; 8];

fn port_num_to_usart(port_num: u8) -> stm32_metapac::usart::Usart {
    match port_num {
        1 => stm32_metapac::USART1,
        2 => stm32_metapac::USART2,
        3 => stm32_metapac::USART3,
        _ => panic!("invalid port number"),
    }
}

fn pin_to_port(tx: &gpio::GpioPort, rx: &gpio::GpioPort) -> u8 {
    if USART1_TX_PINS.contains(tx) && USART1_RX_PINS.contains(rx) {
        1
    } else {
        panic!("not defined");
    }
}

impl Drop for Usart {
    fn drop(&mut self) {
        TAKEN[self.port_num as usize].store(false, core::sync::atomic::Ordering::Relaxed);
    }
}

impl hal::Usart<GpioPort> for Usart {
    fn new(baudrate: u32, tx: GpioPort, rx: GpioPort) -> Result<Self, hal::UsartError> {
        let port_num = pin_to_port(&tx, &rx);

        if TAKEN[port_num as usize].swap(true, core::sync::atomic::Ordering::AcqRel) {
            return Err(hal::UsartError::InitError);
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

        Ok(Usart {
            port,
            port_num,
            use_dma: false,
            dma: None,
        })
    }

    fn read(&self, data: &mut [u8]) -> Result<(), hal::UsartError> {
        for i in 0..data.len() {
            while self.port.isr().read().rxne() == false {}
            data[i] = self.port.rdr().read().dr() as u8;
        }
        Ok(())
    }

    async fn read_async(&self, data: &mut [u8]) -> Result<(), hal::UsartError> {
        return run_no_deep_sleep_async(|| {
            if self.use_dma {
                #[cfg(feature = "usart_dma")]
                {
                    self.read_async_dma(data).await
                }
                #[cfg(not(feature = "usart_dma"))]
                {
                    // Err(hal::UsartError::BusError)
                    panic!("not supported dma");
                }
            } else {
                return self.read_async_interrupt(data);
            }
        })
        .await;
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
        run_no_deep_sleep_async(|| {
            if self.use_dma {
                #[cfg(feature = "usart_dma")]
                {
                    self.write_async_dma(data).await
                }
                #[cfg(not(feature = "usart_dma"))]
                {
                    panic!("not supported dma");
                }
            } else {
                self.write_async_interrupt(data)
            }
        })
        .await
    }
}

impl Usart {
    // todo: test this function
    pub async fn read_async_interrupt(&self, data: &mut [u8]) -> Result<(), hal::UsartError> {
        for i in 0..data.len() {
            core::future::poll_fn(|cx| {
                RX_WAKERS[self.port_num as usize].register(cx.waker());
                self.port.cr1().modify(|v| v.set_rxneie(true));
                if self.port.isr().read().rxne() {
                    Poll::Ready(())
                } else {
                    Poll::Pending
                }
            })
            .await;

            data[i] = self.port.rdr().read().dr() as u8;
        }
        Ok(())
    }

    // todo: test this function
    pub async fn write_async_interrupt(&self, data: &[u8]) -> Result<(), hal::UsartError> {
        for &c in data {
            core::future::poll_fn(|cx| {
                TX_WAKERS[self.port_num as usize].register(cx.waker());
                self.port.cr1().modify(|v| v.set_txeie(true));
                if self.port.isr().read().txe() {
                    Poll::Ready(())
                } else {
                    Poll::Pending
                }
            })
            .await;
            self.port.tdr().write(|v| v.set_dr(c as u16));
        }

        core::future::poll_fn(|cx| {
            TX_WAKERS[self.port_num as usize].register(cx.waker());
            self.port.cr1().modify(|v| v.set_tcie(true));
            if self.port.isr().read().tc() {
                Poll::Ready(())
            } else {
                Poll::Pending
            }
        })
        .await;

        Ok(())
    }

    #[cfg(feature = "usart_dma")]
    pub async fn write_async_dma(&self, data: &[u8]) -> Result<(), hal::UsartError> {
        let src_addr = data.as_ptr() as u32;
        let dst_addr = &self.port.tdr().as_ptr().cast::<u8>() as *const _ as u32;

        self.dma.start(src_addr, true, dst_addr, false, data.len() as u32).await;

        Ok(())
    }

    #[cfg(feature = "usart_dma")]
    pub async fn read_async_dma(&self, buffer: &mut [u8]) -> Result<(), hal::UsartError> {
        let src_addr = &self.port.rdr().as_ptr().cast::<u8>() as *const _ as u32;
        let dst_addr = buffer.as_mut_ptr() as u32;

        self.dma.start(src_addr, false, dst_addr, true, buffer.len() as u32).await;

        Ok(())
    }
}

#[interrupt]
fn USART1() {
    handle_usart_interrupt(stm32_metapac::USART1, 1);
}

#[interrupt]
fn USART2() {
    handle_usart_interrupt(stm32_metapac::USART2, 2);
}

#[interrupt]
fn USART3() {
    handle_usart_interrupt(stm32_metapac::USART3, 3);
}
fn handle_usart_interrupt(usart: stm32_metapac::usart::Usart, index: usize) {
    let isr = usart.isr().read();

    if isr.rxne() {
        RX_WAKERS[index].wake();
        usart.cr1().modify(|v| v.set_rxneie(false));
    }

    if isr.txe() || isr.tc() {
        TX_WAKERS[index].wake();
        usart.cr1().modify(|v| {
            v.set_txeie(false);
            v.set_tcie(false);
        });
    }
}
