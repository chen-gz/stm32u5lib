use core::time::Duration;

/// define hal for embedded system

/// The address should be implemented as a 7-bit address, the 8th bit is the read/write bit
/// for example, the address of the device is 0x50, the read address is 0xA0, the write address is 0xA1
pub trait Pin {
    fn setup(&self); // initialize the pin, this function can only been called once before `Drop` is called
    fn set_high(&self);
    fn set_low(&self);
    fn toggle(&self);
}

#[derive(Debug)]
pub enum I2cFrequency {
    Freq100khz,
    Freq400khz,
    Freq1Mhz,
}
#[derive(Debug)]
pub enum I2cError {
    InitError,
    BusError,
    Nack,
    Timeout,
}

pub trait I2c<T: Pin> {
    /// create a new instance of I2c. The instance should be initialized with the default configuration.
    /// After this function is called, the I2c should be ready to use.
    fn new(freq: I2cFrequency, sda: T, scl: T) -> Result<Self, I2cError> 
    where Self: Sized;

    /// start  -> write(data) -> stop
    fn write(&self, addr: u16, data: &[u8]) -> Result<(), I2cError>;
    // async fn write_aysnc(&self, addr: u16, data: &[u8]) -> Result<(), I2cError>;
    fn write_async(&self, addr: u16, data: &[u8]) -> impl core::future::Future<Output = Result<(), I2cError>> + Send;

    /// read data from addr, the length is determined by the length of data
    fn read(&self, addr: u16, data: &mut [u8]) -> Result<(), I2cError>;
    // async fn read_async(&self, addr: u16, data: &mut [u8]) -> Result<(), I2cError>;
    fn read_async(&self, addr: u16, data: &mut [u8]) -> impl core::future::Future<Output = Result<(), I2cError>> + Send;

    /// start -> write(write_data) -> restart -> read(read_data) -> stop
    fn write_read(
        &self,
        addr: u16,
        write_data: &[u8],
        read_data: &mut [u8],
    ) -> Result<(), I2cError>;

    /// return the maximum frequency that the I2c can support
    fn capacity(&self) -> I2cFrequency;

    fn write_retry(&self, addr: u16, data: &[u8], retry: u8) -> Result<(), I2cError>;
}


#[derive(Debug)]
pub enum UsartError {
    InitError,
    BusError,
    Nack,
    Timeout,
}

pub trait Usart<T: Pin> {
    fn new(baudrate: u32, tx: T, rx: T) -> Result<Self, UsartError> 
    where Self: Sized;
    
    fn write(&self, data: &[u8]) -> Result<(), UsartError>;
    fn read(&self, data: &mut [u8]) -> Result<(), UsartError>;
    fn write_async(&self, data: &[u8]) -> impl core::future::Future<Output = Result<(), UsartError>> + Send;
    fn read_async(&self, data: &mut [u8]) -> impl core::future::Future<Output = Result<(), UsartError>> + Send;

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

/// define general purpose timer (working as global timer)
pub trait Timer {
    // fn start(&self);
    // fn stop(&self);
    // fn reset(&self);
    // return the elapsed time in ns
    fn elapsed(&self) -> Duration;
    fn delay(&self, duration: Duration) -> impl core::future::Future<Output = ()>;
    /// return the resolution of the timer in ns
    fn resolution(&self) -> u32;
}
