/// define hal for embedded system


/// The address should be implemented as a 7-bit address, the 8th bit is the read/write bit
/// for example, the address of the device is 0x50, the read address is 0xA0, the write address is 0xA1
pub trait Pin: Drop {
    pub fn setup();  // initialize the pin, this function can only been called once before `Drop` is called
    pub fn set_high(&self);
    pub fn set_low(&self);
    pub fn toggle(&self);
}


pub trait I2c: Drop {
    pub enum Frequency {
        FREQ_100KHz,
        FREQ_400KHz,
        FREQ_1MHz,
    }
    pub enum Error {
        BusError,
        Nack,
        Timeout,
    }
    /// create a new instance of I2c. The instance should be initialized with the default configuration.
    /// After this function is called, the I2c should be ready to use.
    pub fn new(freq: u32, scl: Pin, sda: Pin) -> Self;

    /// start  -> write(data) -> stop
    pub fn wirte(&self, addr: u8, data: &[u8]) -> Result<(), Error>;

    /// read data from addr, the length is determined by the length of data
    pub fn read(&self, addr: u8, data: &mut [u8]) -> Result<(), Error>;

    /// start -> write(write_data) -> restart -> read(read_data) -> stop
    pub fn write_read(&self, addr: u8, write_data: &[u8], read_data: &mut [u8]) -> Result<(), Error>;

    /// return the maximum frequency that the I2c can support
    pub fn capacity(&self) -> Frequency;
}

pub trait Spi: Drop {
    pub enum Error {
        BusError,
        Timeout,
    }
    pub enum Mode {
        Mode0,
        Mode1,
        Mode2,
        Mode3,
    }
    pub enum Frequency {
        FREQ_1MHz,
        FREQ_2MHz,
        FREQ_4MHz,
        FREQ_8MHz,
        FREQ_16MHz,
        FREQ_32MHz,
        FREQ_64MHz,
        FREQ_128MHz,
    }
    /// create a new instance of Spi. The instance should be initialized with the default configuration.
    /// After this function is called, the Spi should be ready to use.
    pub fn new(freq: u32, mode: Mode, sck: Pin, miso: Pin, mosi: Pin) -> Self;

    pub fn write(&self, data: &[u8]) -> Result<(), Error>;

    pub fn read(&self, data: &mut [u8]) -> Result<(), Error>;

    pub fn write_read(&self, write_data: &[u8], read_data: &mut [u8]) -> Result<(), Error>;
}


