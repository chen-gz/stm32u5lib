#![allow(unused)]

use crate::clock;
use core::task::Poll;
use cortex_m::peripheral::NVIC;
use embassy_sync::waitqueue::AtomicWaker;
use stm32_metapac::interrupt;
use stm32_metapac::{I2C1, RCC};

static mut TAKEN: [bool; 8] = [false; 8]; // first bit will be ignored
static WAKERS: [AtomicWaker; 8] = [const { AtomicWaker::new() }; 8];

#[derive(Copy, Clone, Debug)]
pub enum I2cError {
    TAKEN,
    NoError,
    ArbitrationLost,
    BusError,
    Nack,
    OverrunUnderrun,
    PecError,
    Timeout,
    Alert,
}

pub struct I2cConfig {
    pub port_num: u8,
    pub freq: u32,
    pub scl_pin: crate::gpio::GpioPort,
    pub sda_pin: crate::gpio::GpioPort,
}

pub struct I2c {
    port_num: u8,
    port: stm32_metapac::i2c::I2c,
    freq: hal::I2cFrequency,
}

impl I2cConfig {
    pub fn new(port_num: u8, freq: u32, scl_pin: crate::gpio::GpioPort, sda_pin: crate::gpio::GpioPort) -> Self {
        Self {
            port_num,
            freq,
            scl_pin,
            sda_pin,
        }
    }
}

impl Drop for I2c {
    fn drop(&mut self) {
        unsafe {
            TAKEN[self.port_num as usize] = false;
        }
        self.port.cr1().modify(|v| v.set_pe(false));
    }
}

pub fn port_num_to_i2c(port_num: u8) -> stm32_metapac::i2c::I2c {
    match port_num {
        1 => stm32_metapac::I2C1,
        2 => stm32_metapac::I2C2,
        3 => stm32_metapac::I2C3,
        4 => stm32_metapac::I2C4,
        _ => panic!("invalid port number"),
    }
}

pub fn pin_to_port(scl_pin: &GpioPort, sda_pin: &GpioPort) -> u8 {
    if I2C1_SCL_PINS.contains(scl_pin) && I2C1_SDA_PINS.contains(sda_pin) {
        1
    } else if I2C2_SCL_PINS.contains(scl_pin) && I2C2_SDA_PINS.contains(sda_pin) {
        2
    } else if I2C3_SCL_PINS.contains(scl_pin) && I2C3_SDA_PINS.contains(sda_pin) {
        3
    }
    // else if I2C4_SCL_PINS.contains(scl_pin) && I2C4_SDA_PINS.contains(sda_pin) {
    //     4
    // }
    else {
        panic!("invalid scl and sda pins or not implemented!");
    }
}

// impl Default for I2cConfig {
//     fn default() -> Self {
//         Self {
//             port_num: 1,
//             freq: 100_000, // 100Khz
//             scl_pin: crate::gpio::I2C1_SCL_PB6,
//             sda_pin: crate::gpio::I2C1_SDA_PB7,
//         }
//     }
// }

impl I2c {
    pub async fn write_async_interrupt(&self, addr: u16, data: &[u8]) -> Result<(), hal::I2cError> {
        assert!(data.len() <= 255);
        self.port.cr2().modify(|v| {
            v.set_sadd(addr << 1);
            v.set_nbytes(data.len() as u8);
            v.set_dir(stm32_metapac::i2c::vals::Dir::WRITE);
            v.set_start(true);
        });

        for &byte in data {
            core::future::poll_fn(|cx| {
                WAKERS[self.port_num as usize].register(cx.waker());
                self.port.cr1().modify(|v| {
                    v.set_txie(true);
                    v.set_nackie(true);
                });
                let isr = self.port.isr().read();
                if isr.nackf() {
                    Poll::Ready(Err(hal::I2cError::Nack))
                } else if isr.txis() {
                    Poll::Ready(Ok(()))
                } else {
                    Poll::Pending
                }
            })
            .await?;
            self.port.txdr().write(|v| v.set_txdata(byte));
        }

        core::future::poll_fn(|cx| {
            WAKERS[self.port_num as usize].register(cx.waker());
            self.port.cr1().modify(|v| {
                v.set_tcie(true);
                v.set_stopie(true);
                v.set_nackie(true);
            });
            let isr = self.port.isr().read();
            if isr.nackf() {
                Poll::Ready(Err(hal::I2cError::Nack))
            } else if isr.stopf() || isr.tc() {
                Poll::Ready(Ok(()))
            } else {
                Poll::Pending
            }
        })
        .await?;

        self.port.icr().write(|v| {
            v.set_stopcf(true);
            v.set_nackcf(true);
        });

        Ok(())
    }

    pub async fn read_async_interrupt(&self, addr: u16, data: &mut [u8]) -> Result<(), hal::I2cError> {
        assert!(data.len() <= 255);
        self.port.cr2().modify(|v| {
            v.set_sadd(addr << 1);
            v.set_nbytes(data.len() as u8);
            v.set_dir(stm32_metapac::i2c::vals::Dir::READ);
            v.set_start(true);
        });

        for i in 0..data.len() {
            core::future::poll_fn(|cx| {
                WAKERS[self.port_num as usize].register(cx.waker());
                self.port.cr1().modify(|v| {
                    v.set_rxie(true);
                    v.set_nackie(true);
                });
                let isr = self.port.isr().read();
                if isr.nackf() {
                    Poll::Ready(Err(hal::I2cError::Nack))
                } else if isr.rxne() {
                    Poll::Ready(Ok(()))
                } else {
                    Poll::Pending
                }
            })
            .await?;
            data[i] = self.port.rxdr().read().rxdata();
        }

        core::future::poll_fn(|cx| {
            WAKERS[self.port_num as usize].register(cx.waker());
            self.port.cr1().modify(|v| {
                v.set_tcie(true);
                v.set_stopie(true);
                v.set_nackie(true);
            });
            let isr = self.port.isr().read();
            if isr.nackf() {
                Poll::Ready(Err(hal::I2cError::Nack))
            } else if isr.stopf() || isr.tc() {
                Poll::Ready(Ok(()))
            } else {
                Poll::Pending
            }
        })
        .await?;

        self.port.icr().write(|v| {
            v.set_stopcf(true);
            v.set_nackcf(true);
        });

        Ok(())
    }

    pub async fn wait_address_async_interrupt(&self) -> Result<hal::I2cSlaveEvent, hal::I2cError> {
        core::future::poll_fn(|cx| {
            WAKERS[self.port_num as usize].register(cx.waker());
            self.port.cr1().modify(|v| {
                v.set_addrie(true);
            });
            let isr = self.port.isr().read();
            if isr.addr() {
                let dir = isr.dir();
                // Clear ADDR flag by writing to ADDRCF in ICR
                self.port.icr().write(|v| v.set_addrcf(true));
                if dir == stm32_metapac::i2c::vals::Dir::READ {
                    Poll::Ready(Ok(hal::I2cSlaveEvent::Read))
                } else {
                    Poll::Ready(Ok(hal::I2cSlaveEvent::Write))
                }
            } else {
                Poll::Pending
            }
        })
        .await
    }

    pub async fn read_async_slave(&self, data: &mut [u8]) -> Result<(), hal::I2cError> {
        for i in 0..data.len() {
            core::future::poll_fn(|cx| {
                WAKERS[self.port_num as usize].register(cx.waker());
                self.port.cr1().modify(|v| {
                    v.set_rxie(true);
                    v.set_stopie(true);
                });
                let isr = self.port.isr().read();
                if isr.rxne() {
                    Poll::Ready(Ok(()))
                } else if isr.stopf() {
                    // Host sent STOP before we read all data
                    Poll::Ready(Err(hal::I2cError::BusError))
                } else {
                    Poll::Pending
                }
            })
            .await?;
            data[i] = self.port.rxdr().read().rxdata();
        }
        // Clear STOPF if it was set
        self.port.icr().write(|v| v.set_stopcf(true));
        Ok(())
    }

    pub async fn write_async_slave(&self, data: &[u8]) -> Result<(), hal::I2cError> {
        for &byte in data {
            core::future::poll_fn(|cx| {
                WAKERS[self.port_num as usize].register(cx.waker());
                self.port.cr1().modify(|v| {
                    v.set_txie(true);
                    v.set_stopie(true);
                });
                let isr = self.port.isr().read();
                if isr.txis() {
                    Poll::Ready(Ok(()))
                } else if isr.stopf() {
                    Poll::Ready(Err(hal::I2cError::BusError))
                } else {
                    Poll::Pending
                }
            })
            .await?;
            self.port.txdr().write(|v| v.set_txdata(byte));
        }
        // Wait for TC or STOP
        core::future::poll_fn(|cx| {
            WAKERS[self.port_num as usize].register(cx.waker());
            self.port.cr1().modify(|v| {
                v.set_stopie(true);
            });
            let isr = self.port.isr().read();
            if isr.stopf() {
                Poll::Ready(Ok(()))
            } else {
                Poll::Pending
            }
        })
        .await?;
        self.port.icr().write(|v| v.set_stopcf(true));
        Ok(())
    }
}

pub struct I2cMessage<'a> {
    pub addr: u16,
    pub data: &'a mut [u8],
}

/////////////////////////// HAL implementation /////////////////////////////
use crate::gpio::{
    GpioPort, I2C1_SCL_PB6, I2C1_SCL_PB8, I2C1_SCL_PINS, I2C1_SDA_PB7, I2C1_SDA_PINS, I2C2_SCL_PF1, I2C2_SCL_PINS, I2C2_SDA_PINS,
    I2C3_SCL_PINS, I2C3_SDA_PB4, I2C3_SDA_PINS, PB4, I2C2_SCL_PB13, I2C2_SDA_PB14,
};
use crate::hal;
impl hal::I2c<GpioPort> for I2c {
    fn new(freq: hal::I2cFrequency, sda_pin: GpioPort, scl_pin: GpioPort) -> Result<Self, hal::I2cError> {
        let port_num = pin_to_port(&scl_pin, &sda_pin);
        let freq_val = match freq {
            hal::I2cFrequency::Freq100khz => 100_000,
            hal::I2cFrequency::Freq400khz => 400_000,
            hal::I2cFrequency::Freq1Mhz => 1_000_000,
        };
        unsafe {
            if { TAKEN[port_num as usize] } {
                return Err(hal::I2cError::InitError);
            }
            TAKEN[port_num as usize] = true;
        }
        scl_pin.setup();
        sda_pin.setup();
        clock::set_i2c_clock(port_num);
        // delay_ms(1);
        let port = port_num_to_i2c(port_num);

        port.cr1().modify(|v| v.set_pe(false));
        // dealyt for 6 tick
        clock::delay_tick(6);
        // set timing
        let kernel_freq = crate::clock::get_hclk();

        // TODO: HSI 16 is used as system clock for easy setup.
        // The values are from the reference menu
        // set as 100Khz
        let (presc, scll, sclh, sdadel, scldel) = match freq_val {
            10_000 => (3, 0xC7, 0xC3, 0x2, 0x4), // 10khz
            100_000 => (3, 0x13, 0xF, 0x2, 0x4),  // 100khz
            400_000 => (1, 9, 3, 2, 3),          // 400khz
            1_000_000 => (0, 9, 4, 1, 2),        // 1Mhz
            _ => panic!("invalid frequency"),
        };
        port.timingr().modify(|v| {
            v.set_presc(presc);
            v.set_scll(scll);
            v.set_sclh(sclh);
            v.set_sdadel(sdadel);
            v.set_scldel(scldel);
        });

        // set autoend to true
        port.cr2()
            .modify(|v| v.set_autoend(stm32_metapac::i2c::vals::Autoend::AUTOMATIC));
        // disbale own address
        port.oar1().modify(|v| v.set_oa1en(false));
        port.oar2().modify(|v| v.set_oa2en(false));
        // disable general call
        port.cr1().modify(|v| v.set_gcen(false));
        // no stretch and enable analog filter
        port.cr1().modify(|v| {
            v.set_nostretch(true);
            v.set_anfoff(false);
            v.set_pe(true);
        });
        clock::delay_tick(10);
        unsafe {
            match port_num {
                1 => {
                    NVIC::unmask(interrupt::I2C1_EV);
                    NVIC::unmask(interrupt::I2C1_ER);
                }
                2 => {
                    NVIC::unmask(interrupt::I2C2_EV);
                    NVIC::unmask(interrupt::I2C2_ER);
                }
                3 => {
                    NVIC::unmask(interrupt::I2C3_EV);
                    NVIC::unmask(interrupt::I2C3_ER);
                }
                4 => {
                    NVIC::unmask(interrupt::I2C4_EV);
                    NVIC::unmask(interrupt::I2C4_ER);
                }
                _ => {}
            }
        }
        Ok(I2c { port, port_num, freq })
    }

    fn write(&self, addr: u16, data: &[u8]) -> Result<(), hal::I2cError> {
        assert!(data.len() <= 255);
        // TODO: check hardware status. Whether it allows to send data.
        //       id not allowed, return error.
        // set slave address
        self.port.cr2().modify(|v| {
            v.set_sadd(addr << 1);
            v.set_nbytes(data.len() as u8);
            v.set_dir(stm32_metapac::i2c::vals::Dir::WRITE);
            v.set_start(true);
            // v.set_reload(stm32_metapac::i2c::vals::Reload::COMPLETED);
            // v.set_autoend(stm32_metapac::i2c::vals::Autoend::SOFTWARE);
        });
        for i in &*data {
            // wait for the transfer complete
            while !self.port.isr().read().txis() {} // txdr register is empty
                                                    // send data
            self.port.txdr().write(|v| v.set_txdata(*i));
        }
        // wait for the start bit to be cleared
        while self.port.cr2().read().start() {}
        // wait for the transfer complete
        // while !self.port.isr().read().tc() {} //  don't care about tc flag when no reload
        // and autoend is set to automatic
        // clear the transfer complete flag
        self.port.icr().write(|v| v.set_stopcf(true));
        Ok(())
    }
    // async fn read_async(&self, addr: u16, data: &mut [u8]) -> Result<(), hal::I2cError> {
    //     todo!( "not implement")
    // }
    fn read_async(&self, addr: u16, data: &mut [u8]) -> impl core::future::Future<Output = Result<(), hal::I2cError>> + Send {
        self.read_async_interrupt(addr, data)
    }

    fn read(&self, addr: u16, data: &mut [u8]) -> Result<(), hal::I2cError> {
        // fn receive(&mut self, option: ) -> Result<Self::Response, Self::Error> {
        assert!(data.len() <= 255);
        // todo: check hardware status. Whether it allows to send data.
        // todo: id not allowed, return error.
        self.port.cr2().modify(|v| {
            v.set_sadd(addr << 1);
            v.set_nbytes(data.len() as u8);
            v.set_dir(stm32_metapac::i2c::vals::Dir::READ);
            v.set_start(true);
        });
        for i in data {
            // wait for the transfer complete
            while !self.port.isr().read().rxne() {}
            // read data
            *i = self.port.rxdr().read().rxdata();
        }
        while self.port.cr2().read().start() {}
        self.port.icr().write(|v| v.set_stopcf(true));
        Ok(())
    }
    // async fn write_aysnc(&self, addr: u16, data: &[u8]) -> Result<(), hal::I2cError> {
    //     todo!("not implement")

    // }
    fn write_async(&self, addr: u16, data: &[u8]) -> impl core::future::Future<Output = Result<(), hal::I2cError>> + Send {
        self.write_async_interrupt(addr, data)
    }

    fn write_read(&self, addr: u16, write_data: &[u8], read_data: &mut [u8]) -> Result<(), hal::I2cError> {
        assert!(write_data.len() <= 255);
        assert!(read_data.len() <= 255);

        // 1. Write part (no STOP)
        self.port.cr2().modify(|v| {
            v.set_sadd(addr << 1);
            v.set_nbytes(write_data.len() as u8);
            v.set_dir(stm32_metapac::i2c::vals::Dir::WRITE);
            v.set_autoend(stm32_metapac::i2c::vals::Autoend::SOFTWARE);
            v.set_start(true);
        });

        for &byte in write_data {
            while !self.port.isr().read().txis() {}
            self.port.txdr().write(|v| v.set_txdata(byte));
        }

        // Wait for Transfer Complete (TC)
        while !self.port.isr().read().tc() {}

        // 2. Read part (with STOP)
        self.port.cr2().modify(|v| {
            v.set_sadd(addr << 1);
            v.set_nbytes(read_data.len() as u8);
            v.set_dir(stm32_metapac::i2c::vals::Dir::READ);
            v.set_autoend(stm32_metapac::i2c::vals::Autoend::AUTOMATIC);
            v.set_start(true);
        });

        for byte in read_data {
            while !self.port.isr().read().rxne() {}
            *byte = self.port.rxdr().read().rxdata();
        }

        // Wait for STOP
        while !self.port.isr().read().stopf() {}
        self.port.icr().write(|v| v.set_stopcf(true));

        Ok(())
    }

    fn capacity(&self) -> hal::I2cFrequency {
        self.freq
    }

    // fn write_retry(&self, addr: u16, data: &[u8], retry: u8) -> Result<(), hal::I2cError> {
    //     todo!()
    // }
}

impl hal::I2cSlave<GpioPort> for I2c {
    fn new_slave(sda_pin: GpioPort, scl_pin: GpioPort, addr: u16) -> Result<Self, hal::I2cError> {
        let port_num = pin_to_port(&scl_pin, &sda_pin);
        unsafe {
            if { TAKEN[port_num as usize] } {
                return Err(hal::I2cError::InitError);
            }
            TAKEN[port_num as usize] = true;
        }
        scl_pin.setup();
        sda_pin.setup();
        clock::set_i2c_clock(port_num);
        let port = port_num_to_i2c(port_num);

        port.cr1().modify(|v| v.set_pe(false));
        clock::delay_tick(6);

        // Set Own Address 1
        port.oar1().modify(|v| {
            v.set_oa1(addr << 1);
            v.set_oa1en(true);
        });

        // set autoend to false for slave (usually)
        port.cr2()
            .modify(|v| v.set_autoend(stm32_metapac::i2c::vals::Autoend::SOFTWARE));

        port.cr1().modify(|v| {
            v.set_nostretch(false); // Enable stretching for slave
            v.set_anfoff(false);
            v.set_pe(true);
        });
        clock::delay_tick(10);

        unsafe {
            match port_num {
                1 => {
                    NVIC::unmask(interrupt::I2C1_EV);
                    NVIC::unmask(interrupt::I2C1_ER);
                }
                2 => {
                    NVIC::unmask(interrupt::I2C2_EV);
                    NVIC::unmask(interrupt::I2C2_ER);
                }
                3 => {
                    NVIC::unmask(interrupt::I2C3_EV);
                    NVIC::unmask(interrupt::I2C3_ER);
                }
                4 => {
                    NVIC::unmask(interrupt::I2C4_EV);
                    NVIC::unmask(interrupt::I2C4_ER);
                }
                _ => {}
            }
        }

        Ok(I2c { port, port_num, freq: hal::I2cFrequency::Freq100khz })
    }

    fn slave_wait_address(&self) -> Result<hal::I2cSlaveEvent, hal::I2cError> {
        while !self.port.isr().read().addr() {}
        let isr = self.port.isr().read();
        let dir = isr.dir();
        self.port.icr().write(|v| v.set_addrcf(true));
        if dir == stm32_metapac::i2c::vals::Dir::READ {
            Ok(hal::I2cSlaveEvent::Read)
        } else {
            Ok(hal::I2cSlaveEvent::Write)
        }
    }

    fn slave_wait_address_async(&self) -> impl core::future::Future<Output = Result<hal::I2cSlaveEvent, hal::I2cError>> + Send {
        self.wait_address_async_interrupt()
    }

    fn slave_read(&self, data: &mut [u8]) -> Result<(), hal::I2cError> {
        for i in data {
            while !self.port.isr().read().rxne() && !self.port.isr().read().stopf() {}
            if self.port.isr().read().stopf() {
                self.port.icr().write(|v| v.set_stopcf(true));
                return Err(hal::I2cError::BusError);
            }
            *i = self.port.rxdr().read().rxdata();
        }
        while !self.port.isr().read().stopf() {}
        self.port.icr().write(|v| v.set_stopcf(true));
        Ok(())
    }

    fn slave_read_async(&self, data: &mut [u8]) -> impl core::future::Future<Output = Result<(), hal::I2cError>> + Send {
        self.read_async_slave(data)
    }

    fn slave_write(&self, data: &[u8]) -> Result<(), hal::I2cError> {
        for &byte in data {
            while !self.port.isr().read().txis() && !self.port.isr().read().stopf() {}
            if self.port.isr().read().stopf() {
                self.port.icr().write(|v| v.set_stopcf(true));
                return Err(hal::I2cError::BusError);
            }
            self.port.txdr().write(|v| v.set_txdata(byte));
        }
        while !self.port.isr().read().stopf() {}
        self.port.icr().write(|v| v.set_stopcf(true));
        Ok(())
    }

    fn slave_write_async(&self, data: &[u8]) -> impl core::future::Future<Output = Result<(), hal::I2cError>> + Send {
        self.write_async_slave(data)
    }
}

#[interrupt]
fn I2C1_EV() {
    handle_i2c_interrupt(1);
}

#[interrupt]
fn I2C1_ER() {
    handle_i2c_interrupt(1);
}

#[interrupt]
fn I2C2_EV() {
    handle_i2c_interrupt(2);
}

#[interrupt]
fn I2C2_ER() {
    handle_i2c_interrupt(2);
}

#[interrupt]
fn I2C3_EV() {
    handle_i2c_interrupt(3);
}

#[interrupt]
fn I2C3_ER() {
    handle_i2c_interrupt(3);
}

#[interrupt]
fn I2C4_EV() {
    handle_i2c_interrupt(4);
}

#[interrupt]
fn I2C4_ER() {
    handle_i2c_interrupt(4);
}

fn handle_i2c_interrupt(port_num: u8) {
    let port = port_num_to_i2c(port_num);
    WAKERS[port_num as usize].wake();
    port.cr1().modify(|v| {
        v.set_txie(false);
        v.set_rxie(false);
        v.set_tcie(false);
        v.set_stopie(false);
        v.set_nackie(false);
        v.set_addrie(false);
    });
}
