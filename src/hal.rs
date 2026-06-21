//! define hal for embedded system

/// The address should be implemented as a 7-bit address, the 8th bit is the read/write bit
/// for example, the address of the device is 0x50, the read address is 0xA0, the write address is 0xA1
pub trait Pin {
    fn setup(&self); // initialize the pin, this function can only been called once before `Drop` is called
    fn set_high(&self);
    fn set_low(&self);
    fn toggle(&self);
}
pub trait DMA {
    fn start(
        &self,
        src_addr: u32,
        src_inc: bool,
        dar_addr: u32,
        dst_inc: bool,
        len: u32,
    ) -> impl core::future::Future<Output = ()> + Send + Sync;
    fn stop(&self);
}

/// Abstraction over hardware delay.
pub trait Delay {
    fn delay_ms(&self, ms: u32);
}

/// Abstraction over hardware RTC.
pub trait Rtc {
    /// Returns (year, month, day) and (hour, minute, second)
    fn get_date_time(&self) -> ((u8, u8, u8), (u8, u8, u8));
}

#[derive(Debug, Copy, Clone, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum I2cFrequency {
    Freq100khz,
    Freq400khz,
    Freq1Mhz,
}
#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum I2cError {
    InitError,
    BusError,
    Nack,
    Timeout,
    Overrun,
}

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum I2cSlaveEvent {
    Read,  // Host wants to read from us
    Write, // Host wants to write to us
}

pub trait I2cSlave<T: Pin> {
    fn new_slave(sda: T, scl: T, addr: u16) -> Result<Self, I2cError>
    where
        Self: Sized;

    /// Wait for an address match from a host. Returns whether the host wants to Read or Write.
    fn slave_wait_address(&self) -> Result<I2cSlaveEvent, I2cError>;
    fn slave_wait_address_async(
        &self,
    ) -> impl core::future::Future<Output = Result<I2cSlaveEvent, I2cError>> + Send;

    /// Read data from the host (host is writing)
    fn slave_read(&self, data: &mut [u8]) -> Result<(), I2cError>;
    fn slave_read_async(
        &self,
        data: &mut [u8],
    ) -> impl core::future::Future<Output = Result<(), I2cError>> + Send;

    /// Write data to the host (host is reading)
    fn slave_write(&self, data: &[u8]) -> Result<(), I2cError>;
    fn slave_write_async(
        &self,
        data: &[u8],
    ) -> impl core::future::Future<Output = Result<(), I2cError>> + Send;
}

pub trait I2c<T: Pin> {
    /// create a new instance of I2c. The instance should be initialized with the default configuration.
    /// After this function is called, the I2c should be ready to use.
    fn new(freq: I2cFrequency, sda: T, scl: T) -> Result<Self, I2cError>
    where
        Self: Sized;

    /// start  -> write(data) -> stop
    fn write(&self, addr: u16, data: &[u8]) -> Result<(), I2cError>;
    // async fn write_aysnc(&self, addr: u16, data: &[u8]) -> Result<(), I2cError>;
    fn write_async(
        &self,
        addr: u16,
        data: &[u8],
    ) -> impl core::future::Future<Output = Result<(), I2cError>> + Send;

    /// read data from addr, the length is determined by the length of data
    fn read(&self, addr: u16, data: &mut [u8]) -> Result<(), I2cError>;
    // async fn read_async(&self, addr: u16, data: &mut [u8]) -> Result<(), I2cError>;
    fn read_async(
        &self,
        addr: u16,
        data: &mut [u8],
    ) -> impl core::future::Future<Output = Result<(), I2cError>> + Send;

    /// start -> write(write_data) -> restart -> read(read_data) -> stop
    /// Note: This function always keeps using the blocking way.
    fn write_read(
        &self,
        addr: u16,
        write_data: &[u8],
        read_data: &mut [u8],
    ) -> Result<(), I2cError>;

    /// return the maximum frequency that the I2c can support
    fn capacity(&self) -> I2cFrequency;

    fn write_retry(&self, addr: u16, data: &[u8], retry: u8) -> Result<(), I2cError> {
        let mut cnt = 0;
        loop {
            match self.write(addr, data) {
                Ok(_) => return Ok(()),
                Err(e) => {
                    cnt += 1;
                    if cnt >= retry {
                        return Err(e);
                    }
                }
            }
        }
    }
}

#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum UsartError {
    InitError,
    BusError,
    Nack,
    Timeout,
}

pub trait Usart<T: Pin> {
    fn new(baudrate: u32, tx: T, rx: T) -> Result<Self, UsartError>
    where
        Self: Sized;

    fn write(&self, data: &[u8]) -> Result<(), UsartError>;
    fn write_retry(&self, data: &[u8], retry: u8) -> Result<(), UsartError> {
        let mut cnt = 0;
        loop {
            match self.write(data) {
                Ok(_) => return Ok(()),
                // Err(UsartError::BusError) => {
                //     cnt += 1;
                //     if cnt >= retry {
                //         return Err(UsartError::BusError);
                //     }
                // }
                Err(e) => {
                    cnt += 1;
                    if cnt >= retry {
                        return Err(e);
                    }
                }
            }
        }
    }
    fn read(&self, data: &mut [u8]) -> Result<(), UsartError>;
    fn write_async(
        &self,
        data: &[u8],
    ) -> impl core::future::Future<Output = Result<(), UsartError>> + Send;
    fn read_async(
        &self,
        data: &mut [u8],
    ) -> impl core::future::Future<Output = Result<(), UsartError>> + Send;
}

// pub trait Spi: Drop {
//     pub enum Error {
//         BusError,
//         Timeout,
//     }
//     pub enum Mode {
//         Mode0,
//         Mode1,
//         Mode2,
//         Mode3,
//     }
//     pub enum Frequency {
//         FREQ_1MHz,
//         FREQ_2MHz,
//         FREQ_4MHz,
//         FREQ_8MHz,
//         FREQ_16MHz,
//         FREQ_32MHz,
//         FREQ_64MHz,
//         FREQ_128MHz,
//     }
//     /// create a new instance of Spi. The instance should be initialized with the default configuration.
//     /// After this function is called, the Spi should be ready to use.
//     pub fn new(freq: u32, mode: Mode, sck: Pin, miso: Pin, mosi: Pin) -> Self;
//
//     pub fn write(&self, data: &[u8]) -> Result<(), Error>;
//
//     pub fn read(&self, data: &mut [u8]) -> Result<(), Error>;
//
//     pub fn write_read(&self, write_data: &[u8], read_data: &mut [u8]) -> Result<(), Error>;
// }
//
//

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;

    struct DummyPin;
    impl Pin for DummyPin {
        fn setup(&self) {}
        fn set_high(&self) {}
        fn set_low(&self) {}
        fn toggle(&self) {}
    }

    struct MockI2c {
        fail_count: Cell<u8>,
    }

    impl I2c<DummyPin> for MockI2c {
        fn new(_freq: I2cFrequency, _sda: DummyPin, _scl: DummyPin) -> Result<Self, I2cError> {
            Ok(Self {
                fail_count: Cell::new(0),
            })
        }

        fn write(&self, _addr: u16, _data: &[u8]) -> Result<(), I2cError> {
            let current = self.fail_count.get();
            if current > 0 {
                self.fail_count.set(current - 1);
                Err(I2cError::BusError)
            } else {
                Ok(())
            }
        }

        fn write_async(
            &self,
            _addr: u16,
            _data: &[u8],
        ) -> impl core::future::Future<Output = Result<(), I2cError>> + Send {
            let res = self.write(_addr, _data);
            async move { res }
        }

        fn read(&self, _addr: u16, _data: &mut [u8]) -> Result<(), I2cError> {
            Ok(())
        }

        fn read_async(
            &self,
            _addr: u16,
            _data: &mut [u8],
        ) -> impl core::future::Future<Output = Result<(), I2cError>> + Send {
            async move { Ok(()) }
        }

        fn write_read(
            &self,
            _addr: u16,
            _write_data: &[u8],
            _read_data: &mut [u8],
        ) -> Result<(), I2cError> {
            Ok(())
        }

        fn capacity(&self) -> I2cFrequency {
            I2cFrequency::Freq100khz
        }
    }

    struct MockUsart {
        fail_count: Cell<u8>,
    }

    impl Usart<DummyPin> for MockUsart {
        fn new(_baudrate: u32, _tx: DummyPin, _rx: DummyPin) -> Result<Self, UsartError> {
            Ok(Self {
                fail_count: Cell::new(0),
            })
        }

        fn write(&self, _data: &[u8]) -> Result<(), UsartError> {
            let current = self.fail_count.get();
            if current > 0 {
                self.fail_count.set(current - 1);
                Err(UsartError::BusError)
            } else {
                Ok(())
            }
        }

        fn read(&self, _data: &mut [u8]) -> Result<(), UsartError> {
            Ok(())
        }

        fn write_async(
            &self,
            _data: &[u8],
        ) -> impl core::future::Future<Output = Result<(), UsartError>> + Send {
            async move { Ok(()) }
        }

        fn read_async(
            &self,
            _data: &mut [u8],
        ) -> impl core::future::Future<Output = Result<(), UsartError>> + Send {
            async move { Ok(()) }
        }
    }

    #[test]
    fn test_i2c_write_retry() {
        use futures::executor::block_on;

        // Test Pin mock functions
        DummyPin.setup();
        DummyPin.set_high();
        DummyPin.set_low();
        DummyPin.toggle();

        let i2c = MockI2c::new(I2cFrequency::Freq100khz, DummyPin, DummyPin).unwrap();

        // 1. Success on first attempt
        i2c.fail_count.set(0);
        assert!(i2c.write_retry(0x50, &[1, 2], 3).is_ok());

        // 2. Success after 2 failures (on the 3rd attempt)
        i2c.fail_count.set(2);
        assert!(i2c.write_retry(0x50, &[1, 2], 3).is_ok());

        // 3. Failure after exceeding retries
        i2c.fail_count.set(3);
        assert!(i2c.write_retry(0x50, &[1, 2], 2).is_err());

        // Invoke other mock methods to cover their implementation lines
        i2c.fail_count.set(0);
        let mut buf = [0u8; 1];
        assert!(block_on(i2c.write_async(0x50, &[])).is_ok());
        assert!(i2c.read(0x50, &mut buf).is_ok());
        assert!(block_on(i2c.read_async(0x50, &mut buf)).is_ok());
        assert!(i2c.write_read(0x50, &[], &mut buf).is_ok());
        assert_eq!(i2c.capacity(), I2cFrequency::Freq100khz);
    }

    #[test]
    fn test_usart_write_retry() {
        use futures::executor::block_on;
        let usart = MockUsart::new(115200, DummyPin, DummyPin).unwrap();

        // 1. Success on first attempt
        usart.fail_count.set(0);
        assert!(usart.write_retry(&[1, 2], 3).is_ok());

        // 2. Success after 2 failures
        usart.fail_count.set(2);
        assert!(usart.write_retry(&[1, 2], 3).is_ok());

        // 3. Failure after exceeding retries
        usart.fail_count.set(3);
        assert!(usart.write_retry(&[1, 2], 2).is_err());

        // Invoke other mock methods to cover their implementation lines
        let mut buf = [0u8; 1];
        assert!(usart.read(&mut buf).is_ok());
        assert!(block_on(usart.write_async(&[])).is_ok());
        assert!(block_on(usart.read_async(&mut buf)).is_ok());
    }
}
