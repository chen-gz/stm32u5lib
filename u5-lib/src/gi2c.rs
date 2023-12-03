#![allow(unused)]
use crate::clock::delay_tick;
use stm32_metapac::i2c::I2c;
pub struct I2cPort {
    port: I2c,
}
pub const I2C1: I2cPort = I2cPort {
    port: stm32_metapac::I2C1,
};
pub const I2C3: I2cPort = I2cPort {
    port: stm32_metapac::I2C3,
};

use crate::gpio::*;

#[derive(Copy, Clone, Debug)]
pub enum I2cError {
    NoError,
    ArbitrationLost,
    BusError,
    Nack,
    OverrunUnderrun,
    PecError,
    Timeout,
    Alert,
}
impl I2cPort {
    pub fn init(&self, freq: u32, scl_pin: GpioPort, sda_pin: GpioPort) {
        // setup gpio ports
        scl_pin.setup();
        sda_pin.setup();

        self.port.cr1().modify(|v| v.set_pe(false));
        // dealyt for 6 tick
        delay_tick(6);
        // set timing
        let kernel_freq = crate::clock::get_kernel_freq();

        // TODO: HSI 16 is used as system clock for easy setup.
        // The values are from the reference menu
        // set as 100Khz
        //
        self.port.timingr().modify(|v| {
            // v.set_presc(1); // TODO: set the prescaler based on the kernel frequency
            // v.set_scll(9);
            // v.set_sclh(3);
            // v.set_sdadel(2);
            // v.set_scldel(3);
            v.set_presc(3);
            v.set_scll(0x13);
            v.set_sclh(0xF);
            v.set_sdadel(0x2);
            v.set_scldel(0x4);
        });

        // set autoend to true
        self.port
            .cr2()
            .modify(|v| v.set_autoend(stm32_metapac::i2c::vals::Autoend::AUTOMATIC));
        // disbale own address
        self.port.oar1().modify(|v| v.set_oa1en(false));
        self.port.oar2().modify(|v| v.set_oa2en(false));
        // disable general call
        self.port.cr1().modify(|v| v.set_gcen(false));
        // no stretch and enable analog filter
        self.port.cr1().modify(|v| {
            v.set_nostretch(true);
            v.set_anfoff(false);
            v.set_pe(true);
        });
        delay_tick(10);
    }
    pub fn write(&self, addr: u16, data: &[u8]) -> Result<(), I2cError> {
        assert!(data.len() <= 255);
        // TODO: check hardware status. Whether it allows to send data.
        //       id not allowed, return error.

        // set slave address
        self.port.cr2().modify(|v| {
            v.set_sadd(addr);
            v.set_nbytes(data.len() as u8);
            v.set_dir(stm32_metapac::i2c::vals::Dir::WRITE);
            v.set_start(true);
            // v.set_reload(stm32_metapac::i2c::vals::Reload::COMPLETED);
            // v.set_autoend(stm32_metapac::i2c::vals::Autoend::SOFTWARE);
        });
        for i in data {
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
    pub fn read(&self, addr: u16, data: &mut [u8]) -> Result<(), I2cError> {
        assert!(data.len() <= 255);
        // todo: check hardware status. Whether it allows to send data.
        // todo: id not allowed, return error.

        self.port.cr2().modify(|v| {
            v.set_sadd(addr);
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
        // while !self.port.isr().read().tc() {}
        self.port.icr().write(|v| v.set_stopcf(true));
        Ok(())
    }
    pub fn write_read(
        &self,
        addr: u16,
        send_data: &[u8],
        read_data: &mut [u8],
    ) -> Result<(), I2cError> {
        assert!(send_data.len() <= 255);
        assert!(read_data.len() <= 255);
        self.write(addr, send_data)?;
        self.read(addr, read_data)
    }
}
