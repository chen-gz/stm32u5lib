use crate::hal;
use crate::gpio::GpioPort;
use crate::clock;
use stm32_metapac::spi::{Spi as Peripheral};

pub struct SpiConfig {
    pub port_num: u8,
    pub _freq: u32,
    pub mode: hal::SpiMode,
    pub sck_pin: GpioPort,
    pub miso_pin: GpioPort,
    pub mosi_pin: GpioPort,
}

pub struct Spi<'d> {
    port_num: u8,
    port: Peripheral,
    #[allow(dead_code)]
    sck: GpioPort,
    #[allow(dead_code)]
    miso: GpioPort,
    #[allow(dead_code)]
    mosi: GpioPort,
    _marker: core::marker::PhantomData<&'d ()>,
}

pub fn port_num_to_spi(port_num: u8) -> Peripheral {
    match port_num {
        1 => stm32_metapac::SPI1,
        2 => stm32_metapac::SPI2,
        3 => stm32_metapac::SPI3,
        _ => panic!("invalid port number"),
    }
}

pub fn pin_to_port(sck: &GpioPort, miso: &GpioPort, mosi: &GpioPort) -> u8 {
    if crate::gpio::SPI1_SCK_PINS.contains(sck)
        && crate::gpio::SPI1_MISO_PINS.contains(miso)
        && crate::gpio::SPI1_MOSI_PINS.contains(mosi)
    {
        1
    } else if crate::gpio::SPI2_SCK_PINS.contains(sck)
        && crate::gpio::SPI2_MISO_PINS.contains(miso)
        && crate::gpio::SPI2_MOSI_PINS.contains(mosi)
    {
        2
    } else {
        panic!("invalid spi pins");
    }
}

impl<'d> core::ops::Drop for Spi<'d> {
    fn drop(&mut self) {
        // Disable SPI
        self.port.cr1().modify(|v| v.set_spe(false));
    }
}

impl<'d> hal::Spi<GpioPort> for Spi<'d> {
    fn new(_freq: u32, mode: hal::SpiMode, sck: GpioPort, miso: GpioPort, mosi: GpioPort) -> Result<Self, hal::SpiError> {
        let port_num = pin_to_port(&sck, &miso, &mosi);
        let port = port_num_to_spi(port_num);
        
        // Ensure pins are setup
        sck.setup();
        miso.setup();
        mosi.setup();

        clock::set_spi_clock(port_num);

        // Disable SPI before configuring
        port.cr1().modify(|v| v.set_spe(false));

        // Configure mode
        let (cpol, cpha) = match mode {
            hal::SpiMode::Mode0 => (stm32_metapac::spi::vals::Cpol::IDLE_LOW, stm32_metapac::spi::vals::Cpha::FIRST_EDGE),
            hal::SpiMode::Mode1 => (stm32_metapac::spi::vals::Cpol::IDLE_LOW, stm32_metapac::spi::vals::Cpha::SECOND_EDGE),
            hal::SpiMode::Mode2 => (stm32_metapac::spi::vals::Cpol::IDLE_HIGH, stm32_metapac::spi::vals::Cpha::FIRST_EDGE),
            hal::SpiMode::Mode3 => (stm32_metapac::spi::vals::Cpol::IDLE_HIGH, stm32_metapac::spi::vals::Cpha::SECOND_EDGE),
        };

        // STM32U5 SPI CFG2 setup (clock phase, polarity, master, data size, etc...)
        port.cfg2().modify(|v| {
            v.set_cpol(cpol);
            v.set_cpha(cpha);
            v.set_master(stm32_metapac::spi::vals::Master::MASTER);
            v.set_ssm(true);
            v.set_comm(stm32_metapac::spi::vals::Comm::FULL_DUPLEX);
        });
        
        port.cfg1().modify(|v| {
            // Data size 8 bits
            v.set_dsize(7); 
            
            // Set baud rate prescaler (just default to DIV8 for basic support)
            // A more complex implementation would calculate this based on freq parameter
            v.set_mbr(stm32_metapac::spi::vals::Mbr::DIV8);
        });
        
        // Wait till we know the bit field names for prescaler
        port.cr1().modify(|v| v.set_ssi(true));
        
        // Enable SPI
        port.cr1().modify(|v| v.set_spe(true));

        Ok(Self {
            port_num,
            port,
            sck,
            miso,
            mosi,
            _marker: core::marker::PhantomData,
        })
    }

    fn write(&self, data: &[u8]) -> Result<(), hal::SpiError> {
        // Simple synchronous transmit
        self.port.cr1().modify(|v| v.set_cstart(true));
        for &byte in data {
            while !self.port.sr().read().txp() {}
            unsafe {
                core::ptr::write_volatile(self.port.txdr8().as_ptr() as *mut u8, byte);
            }
        }
        while !self.port.sr().read().txc() {}
        Ok(())
    }

    fn read(&self, data: &mut [u8]) -> Result<(), hal::SpiError> {
        // Simple synchronous receive
        self.port.cr1().modify(|v| v.set_cstart(true));
        for byte in data.iter_mut() {
            while !self.port.sr().read().txp() {}
            // Write dummy byte to clock out the receive
            unsafe {
                core::ptr::write_volatile(self.port.txdr8().as_ptr() as *mut u8, 0xFF);
            }
            while !self.port.sr().read().rxp() {}
            unsafe {
                *byte = core::ptr::read_volatile(self.port.rxdr8().as_ptr() as *const u8);
            }
        }
        while !self.port.sr().read().txc() {}
        Ok(())
    }

    fn write_read(&self, write_data: &[u8], read_data: &mut [u8]) -> Result<(), hal::SpiError> {
        let len = core::cmp::max(write_data.len(), read_data.len());
        self.port.cr1().modify(|v| v.set_cstart(true));
        for i in 0..len {
            let tx_byte = if i < write_data.len() { write_data[i] } else { 0xFF };
            while !self.port.sr().read().txp() {}
            unsafe {
                core::ptr::write_volatile(self.port.txdr8().as_ptr() as *mut u8, tx_byte);
            }
            while !self.port.sr().read().rxp() {}
            let rx_byte = unsafe {
                core::ptr::read_volatile(self.port.rxdr8().as_ptr() as *const u8)
            };
            if i < read_data.len() {
                read_data[i] = rx_byte;
            }
        }
        while !self.port.sr().read().txc() {}
        Ok(())
    }
}
