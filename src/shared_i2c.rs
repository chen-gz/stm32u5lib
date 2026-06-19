use crate::hal::{I2c, I2cError, Pin};
use embassy_sync::blocking_mutex::raw::RawMutex;
use embassy_sync::mutex::Mutex;

/// A non-cloneable token that grants exclusive access to the I2C read interface.
pub struct I2cRxToken {
    _private: (),
}

impl I2cRxToken {
    pub(crate) fn new() -> Self {
        Self { _private: () }
    }
}

/// A generic manager that wraps any `I2c` implementation to serialize writes and restrict reads.
pub struct SharedI2cManager<M: RawMutex, I: I2c<P>, P: Pin> {
    mutex: Mutex<M, Option<I>>,
    _phantom: core::marker::PhantomData<P>,
}

impl<M: RawMutex, I: I2c<P>, P: Pin> SharedI2cManager<M, I, P> {
    /// Creates a new uninitialized manager (can be used in `static` contexts).
    pub const fn new() -> Self {
        Self {
            mutex: Mutex::new(None),
            _phantom: core::marker::PhantomData,
        }
    }

    /// Initializes the manager with a concrete I2C driver instance and returns the unique RX token.
    pub async fn init(&self, i2c: I) -> I2cRxToken {
        let mut guard = self.mutex.lock().await;
        *guard = Some(i2c);
        I2cRxToken::new()
    }

    /// Asynchronously writes to a device on the shared I2C bus.
    /// If another write/read is in progress, this will wait (yielding to the executor).
    pub async fn write(&self, addr: u16, data: &[u8]) -> Result<(), I2cError> {
        let mut guard = self.mutex.lock().await;
        if let Some(i2c) = guard.as_mut() {
            i2c.write_async(addr, data).await
        } else {
            Err(I2cError::InitError)
        }
    }

    /// Asynchronously reads from a device.
    /// Requires exclusive mutable access to the unique `I2cRxToken` to compile-time restrict who can call it.
    pub async fn read(&self, _token: &mut I2cRxToken, addr: u16, data: &mut [u8]) -> Result<(), I2cError> {
        let mut guard = self.mutex.lock().await;
        if let Some(i2c) = guard.as_mut() {
            i2c.read_async(addr, data).await
        } else {
            Err(I2cError::InitError)
        }
    }
}
