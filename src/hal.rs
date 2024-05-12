/// define hal for embedded system


/// The address should be implemented as a 7-bit address, the 8th bit is the read/write bit
/// for example, the address of the device is 0x50, the read address is 0xA0, the write address is 0xA1
pub trait Pin: Drop {
    pub fn new(pin: u32) -> Self;
    pub fn set_high(&self);
    pub fn set_low(&self);
    pub fn toggle(&self);
    pub fn setup(); 
}


pub trait I2c: Drop {
    /// create a new instance of I2c. The instance should be initialized with the default configuration.
    /// After this function is called, the I2c should be ready to use.
    pub fn new(freq: u32, sda: Pin, scl: Pin) -> Self;

    /// start  -> write(data) -> stop
    pub fn wirte(&self, addr: u8, data: &[u8]); // the length is determined by the length of data

    /// read data from addr, the length is determined by the length of data
    pub fn read(&self, addr: u8, data: &mut [u8]); // the length is determined by the length of data

    /// start -> write(write_data) -> restart -> read(read_data) -> stop
    pub fn write_read(&self, addr: u8, write_data: &[u8], read_data: &mut [u8]); // the length is determined by the length of data

}
