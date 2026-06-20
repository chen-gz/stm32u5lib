//! # Shared I2C Manager
//!
//! This module provides a thread-safe, asynchronous wrapper (`SharedI2cManager`) for sharing a single
//! physical I2C peripheral among multiple tasks.
//!
//! ## Design & Architecture
//!
//! In embedded systems using Embassy (or other async executors), multiple drivers (e.g., sensors, displays)
//! often need to share the same I2C bus. `SharedI2cManager` solves this by wrapping the underlying `I2c`
//! driver in an asynchronous Mutex (`embassy_sync::mutex::Mutex`).
//!
//! ### Async-Only Mutex vs. Blocking Mutex
//! - **Async Mutex (Chosen)**: Locks are acquired asynchronously (`lock().await`). If the bus is busy,
//!   the calling task yields control back to the executor, allowing other tasks to run. This maximizes
//!   CPU utilization and prevents blocking the entire system.
//! - **Blocking Mutex**: Acquiring a lock synchronously blocks the execution thread. In a single-threaded
//!   async executor, blocking the thread halts the entire executor, preventing other async tasks from running
//!   and potentially causing system-wide latency or deadlocks.
//!
//! ### Why Synchronous `read` and `write` are Inappropriate
//! Adding synchronous (blocking) `read` and `write` methods to this manager is **strongly discouraged** and
//! inappropriate for the following reasons:
//! 1. **Deadlock Risk**: If an async task holds the lock (across `.await` points) and a synchronous context
//!    attempts to block-wait for the same lock, the async task will never get polled to release the lock,
//!    causing a permanent deadlock.
//! 2. **Executor Starvation**: Synchronous I2C operations actively spin/wait for hardware flags (like `TXIS` or `RXNE`).
//!    This starves the async executor and negates the benefits of asynchronous driver design.
//! 3. **API Incompatibility**: The underlying lock (`embassy_sync::mutex::Mutex`) does not expose a blocking
//!    synchronous acquire method. Using `try_lock` in a spin-loop is an anti-pattern that consumes 100% CPU.
//!
//! ### Compile-Time Reader Safety (`I2cRxToken`)
//! To prevent multiple tasks from concurrently reading from the same bus (which could mix up received data),
//! `SharedI2cManager` enforces a single-reader model at compile-time:
//! - Writing (`write`) can be called by any task holding a reference to the manager.
//! - Reading (`read`) requires exclusive mutable access to a unique `I2cRxToken` returned during initialization.

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
    pub async fn read(
        &self,
        _token: &mut I2cRxToken,
        addr: u16,
        data: &mut [u8],
    ) -> Result<(), I2cError> {
        let mut guard = self.mutex.lock().await;
        if let Some(i2c) = guard.as_mut() {
            i2c.read_async(addr, data).await
        } else {
            Err(I2cError::InitError)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hal::I2cFrequency;
    use std::sync::Mutex as StdMutex;

    struct MockPin;
    impl Pin for MockPin {
        fn setup(&self) {}
        fn set_high(&self) {}
        fn set_low(&self) {}
        fn toggle(&self) {}
    }

    struct MockI2c {
        write_log: StdMutex<Vec<(u16, Vec<u8>)>>,
        read_data: StdMutex<Vec<u8>>,
    }

    impl I2c<MockPin> for MockI2c {
        fn new(_freq: I2cFrequency, _sda: MockPin, _scl: MockPin) -> Result<Self, I2cError> {
            Ok(Self {
                write_log: StdMutex::new(Vec::new()),
                read_data: StdMutex::new(Vec::new()),
            })
        }

        fn write(&self, addr: u16, data: &[u8]) -> Result<(), I2cError> {
            self.write_log.lock().unwrap().push((addr, data.to_vec()));
            Ok(())
        }

        fn write_async(
            &self,
            addr: u16,
            data: &[u8],
        ) -> impl core::future::Future<Output = Result<(), I2cError>> + Send {
            let res = self.write(addr, data);
            async move { res }
        }

        fn read(&self, _addr: u16, data: &mut [u8]) -> Result<(), I2cError> {
            let mut read_buf = self.read_data.lock().unwrap();
            if read_buf.len() < data.len() {
                return Err(I2cError::BusError);
            }
            let drain = read_buf.drain(0..data.len()).collect::<Vec<_>>();
            data.copy_from_slice(&drain);
            Ok(())
        }

        fn read_async(
            &self,
            addr: u16,
            data: &mut [u8],
        ) -> impl core::future::Future<Output = Result<(), I2cError>> + Send {
            let res = self.read(addr, data);
            async move { res }
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
            I2cFrequency::Freq400khz
        }
    }

    #[test]
    fn test_shared_i2c_manager_write_read() {
        use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
        use futures::executor::block_on;

        let manager: SharedI2cManager<CriticalSectionRawMutex, MockI2c, MockPin> =
            SharedI2cManager::new();
        let mock_driver = MockI2c::new(I2cFrequency::Freq100khz, MockPin, MockPin).unwrap();

        // Put some data in read buffer
        *mock_driver.read_data.lock().unwrap() = vec![0x11, 0x22, 0x33, 0x44];

        block_on(async {
            let mut token = manager.init(mock_driver).await;

            // Perform write
            let write_res = manager.write(0x50, &[0xAA, 0xBB]).await;
            assert!(write_res.is_ok());

            // Perform read
            let mut read_buf = [0u8; 4];
            let read_res = manager.read(&mut token, 0x50, &mut read_buf).await;
            assert!(read_res.is_ok());
            assert_eq!(read_buf, [0x11, 0x22, 0x33, 0x44]);
        });
    }

    #[test]
    fn test_shared_i2c_manager_uninitialized_and_edge_cases() {
        use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
        use futures::executor::block_on;

        // Test Pin mock functions
        MockPin.setup();
        MockPin.set_high();
        MockPin.set_low();
        MockPin.toggle();

        // Test MockI2c other functions
        let mock_driver = MockI2c::new(I2cFrequency::Freq100khz, MockPin, MockPin).unwrap();
        assert!(matches!(mock_driver.capacity(), I2cFrequency::Freq400khz));
        assert!(mock_driver.write_read(0x50, &[], &mut []).is_ok());

        // Test MockI2c read buffer underflow
        let mut read_buf = [0u8; 1];
        assert!(mock_driver.read(0x50, &mut read_buf).is_err());

        // Test uninitialized manager error paths
        let manager: SharedI2cManager<CriticalSectionRawMutex, MockI2c, MockPin> =
            SharedI2cManager::new();
        block_on(async {
            let write_res = manager.write(0x50, &[0xAA]).await;
            assert!(matches!(write_res, Err(I2cError::InitError)));

            let mut token = I2cRxToken::new();
            let mut read_buf = [0u8; 1];
            let read_res = manager.read(&mut token, 0x50, &mut read_buf).await;
            assert!(matches!(read_res, Err(I2cError::InitError)));
        });
    }
}
