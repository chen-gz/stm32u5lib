#![allow(unused)]

use crate::clock;
use crate::com_interface::ComInterface;
use stm32_metapac::{I2C1, RCC};

static mut TAKEN: [bool; 8] = [false; 8]; // first bit will be ignored

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
}

impl I2cConfig {
    pub fn new(
        port_num: u8,
        freq: u32,
        scl_pin: crate::gpio::GpioPort,
        sda_pin: crate::gpio::GpioPort,
    ) -> Self {
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

pub struct I2cMessage<'a> {
    pub addr: u16,
    pub data: &'a mut [u8],
}

impl I2c {
    pub fn write_read(
        &mut self,
        addr: u16,
        send_data: &mut [u8],
        read_data: &mut [u8],
    ) -> Result<(), I2cError> {
        assert!(send_data.len() <= 255);
        assert!(read_data.len() <= 255);
        let message = I2cMessage {
            addr,
            data: send_data,
        };
        self.send(&message)?;
        let option = I2cMessage {
            addr,
            data: read_data,
        };
        self.receive(option)
    }
}

/////////////////////////// HAL implementation /////////////////////////////
use crate::gpio::{
    GpioPort, I2C1_SCL_PB6, I2C1_SCL_PB8, I2C1_SCL_PINS, I2C1_SDA_PB7, I2C1_SDA_PINS, I2C2_SCL_PF1,
    I2C2_SCL_PINS, I2C2_SDA_PINS, I2C3_SCL_PINS, I2C3_SDA_PB4, I2C3_SDA_PINS, PB4,
};
use crate::hal;
impl hal::I2c<GpioPort> for I2c {
    fn new(
        freq: hal::I2cFrequency,
        sda_pin: GpioPort,
        scl_pin: GpioPort,
    ) -> Result<Self, hal::I2cError> {
        let port_num = pin_to_port(&scl_pin, &sda_pin);
        let freq = match freq {
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
        let (presc, scll, sclh, sdadel, scldel) = match freq {
            10_000 => (3, 0xC7, 0xC3, 0x2, 0x4), // 10khz
            400_000 => (1, 9, 3, 2, 3),          // 400khz
            // 1_000_000 => (3, 0x13, 0xF, 0x2, 0x4),  // 1Mhz; check with reference manual
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
        Ok(I2c { port, port_num })
    }

    fn write(&self, addr: u16, data: &[u8]) -> Result<(), hal::I2cError> {
        // let mut data = data;
        // let message = I2cMessage {
        //     addr,
        //     data: &mut data,
        // };
        // self.send(&message)
        // let addr = message.addr;
        // let data = &message.data;
        // assert!(data.len() <= 255);
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
    fn read_async(
        &self,
        addr: u16,
        data: &mut [u8],
    ) -> impl core::future::Future<Output = Result<(), hal::I2cError>> + Send {
        async move {
            todo!("Implement data fetching logic here");
        }
    }

    fn read(&self, addr: u16, data: &mut [u8]) -> Result<(), hal::I2cError> {
        // fn receive(&mut self, option: ) -> Result<Self::Response, Self::Error> {
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
        self.port.icr().write(|v| v.set_stopcf(true));
        Ok(())
    }
    // async fn write_aysnc(&self, addr: u16, data: &[u8]) -> Result<(), hal::I2cError> {
    //     todo!("not implement")

    // }
    fn write_async(
        &self,
        addr: u16,
        data: &[u8],
    ) -> impl core::future::Future<Output = Result<(), hal::I2cError>> + Send {
        async move {
            todo!("Implement data sending logic here");
        }
    }

    fn write_read(
        &self,
        addr: u16,
        write_data: &[u8],
        read_data: &mut [u8],
    ) -> Result<(), hal::I2cError> {
        let message = I2cMessage {
            addr,
            data: read_data,
        };
        assert!(write_data.len() <= 255);
        assert!(read_data.len() <= 255);
        self.write(addr, write_data)?;
        self.read(addr, read_data)?;
        Ok(())
    }

    fn capacity(&self) -> hal::I2cFrequency {
        todo!()
    }

    // fn write_retry(&self, addr: u16, data: &[u8], retry: u8) -> Result<(), hal::I2cError> {
    //     todo!()
    // }
}

////// deprecated will be removed in the future

impl<'a> ComInterface<'a> for I2c {
    type Error = I2cError;
    type Message = I2cMessage<'a>;
    type Response = ();
    type Config = I2cConfig;
    type ReceiveOption = I2cMessage<'a>;

    // #[deprecated]
    fn new(config: Self::Config) -> Result<Self, Self::Error>
    where
        Self: Sized,
    {
        unsafe {
            if { TAKEN[config.port_num as usize] } {
                return Err(I2cError::TAKEN);
            }
            TAKEN[config.port_num as usize] = true;
        }
        config.scl_pin.setup();
        config.sda_pin.setup();
        // delay_ms(10);
        clock::set_i2c_clock(config.port_num);

        clock::set_i2c_clock(2);
        // delay_ms(1);
        // setup gpio ports
        let port = if config.port_num == 1 {
            stm32_metapac::I2C1
        } else if config.port_num == 2 {
            stm32_metapac::I2C2
        } else if config.port_num == 3 {
            stm32_metapac::I2C3
        } else {
            panic!("invalid port number");
        };

        port.cr1().modify(|v| v.set_pe(false));
        // dealyt for 6 tick
        clock::delay_tick(6);
        // set timing
        let kernel_freq = crate::clock::get_hclk();

        // TODO: HSI 16 is used as system clock for easy setup.
        // The values are from the reference menu
        // set as 100Khz
        //
        port.timingr().modify(|v| {
            // 400khz
            v.set_presc(1); // TODO: set the prescaler based on the kernel frequency
            v.set_scll(9);
            v.set_sclh(3);
            v.set_sdadel(2);
            v.set_scldel(3);

            // v.set_presc(3);
            // v.set_scll(0x13);
            // v.set_sclh(0xF);
            // v.set_sdadel(0x2);
            // v.set_scldel(0x4);
            // 10khz
            // v.set_presc(3);
            // v.set_scll(0xC7);
            // v.set_sclh(0xC3);
            // v.set_sdadel(0x2);
            // v.set_scldel(0x4);
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
        Ok(I2c {
            port: port,
            port_num: config.port_num,
        })
    }

    fn send(&mut self, message: &Self::Message) -> Result<(), Self::Error> {
        let addr = message.addr;
        // let data = &message.data;
        // assert!(data.len() <= 255);
        assert!(message.data.len() <= 255);
        // TODO: check hardware status. Whether it allows to send data.
        //       id not allowed, return error.
        // set slave address
        self.port.cr2().modify(|v| {
            v.set_sadd(addr);
            v.set_nbytes(message.data.len() as u8);
            v.set_dir(stm32_metapac::i2c::vals::Dir::WRITE);
            v.set_start(true);
            // v.set_reload(stm32_metapac::i2c::vals::Reload::COMPLETED);
            // v.set_autoend(stm32_metapac::i2c::vals::Autoend::SOFTWARE);
        });
        for i in &*message.data {
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

    fn receive(&mut self, option: Self::ReceiveOption) -> Result<Self::Response, Self::Error> {
        let addr = option.addr;
        let data = option.data;
        // todo!()
        // fn receive(&mut self, option: ) -> Result<Self::Response, Self::Error> {
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
        self.port.icr().write(|v| v.set_stopcf(true));
        Ok(())
    }

    async fn send_async(&mut self, message: Self::Message) -> Result<(), Self::Error> {
        todo!()
    }

    async fn receive_async(&mut self) -> Result<Self::Response, Self::Error> {
        todo!()
    }

    fn enable(&mut self) -> Result<(), Self::Error> {
        todo!()
    }

    fn disable(&mut self) -> Result<(), Self::Error> {
        todo!()
    }

    fn drop(&mut self) -> Result<(), Self::Error> {
        todo!()
    }

    fn wait_connection(&mut self) -> Result<(), Self::Error> {
        todo!()
    }
}
