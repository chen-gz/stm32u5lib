//! This is interface for low level communication with the device.
//! It design for all protocol that can be used with this device.
//! current protocol include i2c, spi, uart, usb
//!
//!



#[deprecated]
pub trait ComInterface<'a> {
    // make Self sized
    type Error: core::fmt::Debug;
    type Message: 'a;
    // For send data
    type Response: 'a;
    type Config;

    type ReceiveOption: 'a;

    fn new(config: Self::Config) -> Result<Self, Self::Error> where Self: Sized;

    fn send(&mut self, message: &Self::Message) -> Result<(), Self::Error>;

    fn send_retry(&mut self, message: Self::Message, retry: u8) -> Result<(), Self::Error> {
        let mut count = 0;
        loop {
            match self.send(&message) {
                Ok(_) => return Ok(()),
                Err(e) => {
                    count += 1;
                    if count >= retry {
                        return Err(e);
                    }
                }
            }
        }
    }

    fn receive(&mut self, option: Self::ReceiveOption) -> Result<Self::Response, Self::Error>;

    /// Send a message to the interface
    // fn send_async(&mut self, message: Self::Message) -> impl core::future::Future<Output = Result<(), Self::Error>>;
    // Result<Self::Response, Self::Error>;
    // async fn send_async(&mut self, message: Self::Message) -> Result<(), Self::Error>;
    fn send_async(&mut self, message: Self::Message) -> impl core::future::Future<Output = Result<(), Self::Error>>;


    /// Receive a message from the interface until a message is received or an error occurs
    // async fn receive_async(&mut self) -> Result<Self::Message, Self::Error>;
    // supress warning
    // async fn receive_async(&mut self) -> Result<Self::Response, Self::Error>;
    fn receive_async(&mut self) -> impl core::future::Future<Output = Result<Self::Response, Self::Error>>;


    /// Temporary enable the interface
    fn enable(&mut self) -> Result<(), Self::Error>;

    /// Temporary disable the interface
    fn disable(&mut self) -> Result<(), Self::Error>;

    /// Drop the interface. The low level resource will be released
    fn drop(&mut self) -> Result<(), Self::Error>;

    /// wait for connection
    /// after this function return Ok. The reand and write function can be used
    /// This function will automatically enable the interface
    fn wait_connection(&mut self) -> Result<(), Self::Error>;
}
