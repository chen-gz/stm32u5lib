//! This is interface for low level communication with the device.
//! It design for all protocol that can be used with this device.
//! current protocol include i2c, spi, uart, usb
//! 
//! 



pub trait ComInterface<'a> {
    // make Self sized
    type Error: core::fmt::Debug;
    type Message;
    type Response;
    type Config: Default;
    fn new(config: Self::Config) -> Result<Self, Self::Error> where Self: Sized;

    fn send(&mut self, message: Self::Message) -> Result<(), Self::Error>;
    fn receive(&mut self) -> Result<Self::Message, Self::Error>;


    /// Send a message to the interface
    // fn send_async(&mut self, message: Self::Message) -> impl core::future::Future<Output = Result<(), Self::Error>>;
    // Result<Self::Response, Self::Error>;
    async fn send_async(&mut self, message: Self::Message) -> Result<(), Self::Error>;

    /// Receive a message from the interface until a message is received or an error occurs
    // async fn receive_async(&mut self) -> Result<Self::Message, Self::Error>;
    // supress warning
    async fn receive_async(&mut self) ->  Result<Self::Message, Self::Error>;

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
