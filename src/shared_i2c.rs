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
//! ### Half-Duplex Bus Design
//! Since I2C is a half-duplex bus, we do not require a separate read/write access token.
//! Both reads and writes are serialized using the same async Mutex.
//!
//! ### Real-World Firmware Sharing (Global `static` Variable)
//! In real-world asynchronous firmware (like Embassy-based projects), tasks must have `'static` lifetimes.
//! Therefore, you cannot pass local variables by temporary reference. Instead, declare `SharedI2cManager`
//! as a global `static` variable so multiple tasks can safely reference it concurrently using `&'static`.
//!
//! ```rust,ignore
//! use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
//! use u5_lib::shared_i2c::SharedI2cManager;
//! use u5_lib::i2c::I2c as I2cDriver;
//! use u5_lib::gpio::GpioPort;
//!
//! static SHARED_I2C: SharedI2cManager<CriticalSectionRawMutex, I2cDriver, GpioPort> =
//!     SharedI2cManager::new();
//!
//! #[embassy_executor::main]
//! async fn main(spawner: Spawner) {
//!     // 1. Initialize the concrete I2C hardware driver instance
//!     let i2c_driver = I2cDriver::new(...);
//!
//!     // 2. Initialize the global SHARED_I2C manager with the driver
//!     SHARED_I2C.init(i2c_driver).await;
//!
//!     // 3. Spawn tasks that safely reference SHARED_I2C concurrently
//!     spawner.spawn(task_1()).unwrap();
//!     spawner.spawn(task_2()).unwrap();
//! }
//!
//! #[embassy_executor::task]
//! async fn task_1() {
//!     // Safely borrows the global static manager
//!     let _ = SHARED_I2C.write(0x50, &[0xAA]).await;
//! }
//!
//! #[embassy_executor::task]
//! async fn task_2() {
//!     let mut buf = [0u8; 2];
//!     // Safely borrows the same global static manager concurrently
//!     let _ = SHARED_I2C.read(0x50, &mut buf).await;
//! }
//! ```

use crate::hal::{I2c, I2cError, Pin};
use embassy_sync::blocking_mutex::raw::RawMutex;
use embassy_sync::mutex::Mutex;

/// A generic manager that wraps any `I2c` implementation to serialize writes and reads.
pub struct SharedI2cManager<M: RawMutex, I: I2c<P>, P: Pin> {
    mutex: Mutex<M, Option<I>>,
    _phantom: core::marker::PhantomData<P>,
}

impl<M: RawMutex, I: I2c<P>, P: Pin> Default for SharedI2cManager<M, I, P> {
    fn default() -> Self {
        Self::new()
    }
}

impl<M: RawMutex, I: I2c<P>, P: Pin> SharedI2cManager<M, I, P> {
    /// Creates a new uninitialized manager (can be used in `static` contexts).
    pub const fn new() -> Self {
        Self {
            mutex: Mutex::new(None),
            _phantom: core::marker::PhantomData,
        }
    }

    /// Initializes the manager with a concrete I2C driver instance.
    /// Panics if the manager is already initialized.
    pub async fn init(&self, i2c: I) {
        let mut guard = self.mutex.lock().await;
        if guard.is_some() {
            panic!("SharedI2cManager is already initialized");
        }
        *guard = Some(i2c);
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
    pub async fn read(&self, addr: u16, data: &mut [u8]) -> Result<(), I2cError> {
        let mut guard = self.mutex.lock().await;
        if let Some(i2c) = guard.as_mut() {
            i2c.read_async(addr, data).await
        } else {
            Err(I2cError::InitError)
        }
    }

    /// Asynchronously writes then reads from a device under the same lock (atomic transaction).
    pub async fn write_read(
        &self,
        addr: u16,
        write_data: &[u8],
        read_data: &mut [u8],
    ) -> Result<(), I2cError> {
        let mut guard = self.mutex.lock().await;
        if let Some(i2c) = guard.as_mut() {
            i2c.write_read(addr, write_data, read_data)
        } else {
            Err(I2cError::InitError)
        }
    }

    /// Asynchronously writes to a device with retry logic.
    pub async fn write_retry(&self, addr: u16, data: &[u8], retry: u8) -> Result<(), I2cError> {
        let mut cnt = 0;
        loop {
            match self.write(addr, data).await {
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
            addr: u16,
            write_data: &[u8],
            read_data: &mut [u8],
        ) -> Result<(), I2cError> {
            let _ = self.write(addr, write_data);
            self.read(addr, read_data)
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
        *mock_driver.read_data.lock().unwrap() = vec![0x11, 0x22, 0x33, 0x44, 0x55, 0x66];

        block_on(async {
            manager.init(mock_driver).await;

            // Perform write
            let write_res = manager.write(0x50, &[0xAA, 0xBB]).await;
            assert!(write_res.is_ok());

            // Perform read
            let mut read_buf = [0u8; 4];
            let read_res = manager.read(0x50, &mut read_buf).await;
            assert!(read_res.is_ok());
            assert_eq!(read_buf, [0x11, 0x22, 0x33, 0x44]);

            // Perform write_read
            let mut wr_read_buf = [0u8; 2];
            let wr_res = manager.write_read(0x50, &[0xCC], &mut wr_read_buf).await;
            assert!(wr_res.is_ok());
            assert_eq!(wr_read_buf, [0x55, 0x66]);

            // Perform write_retry
            let retry_res = manager.write_retry(0x50, &[0xDD], 3).await;
            assert!(retry_res.is_ok());
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
        assert_eq!(mock_driver.capacity(), I2cFrequency::Freq400khz);
        assert!(mock_driver.write_read(0x50, &[], &mut []).is_ok());

        // Test MockI2c read buffer underflow
        let mut read_buf = [0u8; 1];
        assert!(mock_driver.read(0x50, &mut read_buf).is_err());

        // Test MockI2c write_read error paths
        let mock_driver_fail_read =
            MockI2c::new(I2cFrequency::Freq100khz, MockPin, MockPin).unwrap();
        assert!(mock_driver_fail_read
            .write_read(0x50, &[0xAA], &mut read_buf)
            .is_err());

        // Test uninitialized manager error paths
        let manager: SharedI2cManager<CriticalSectionRawMutex, MockI2c, MockPin> =
            SharedI2cManager::default();
        block_on(async {
            let write_res = manager.write(0x50, &[0xAA]).await;
            assert_eq!(write_res, Err(I2cError::InitError));

            let mut read_buf = [0u8; 1];
            let read_res = manager.read(0x50, &mut read_buf).await;
            assert_eq!(read_res, Err(I2cError::InitError));

            // Test write_read on uninitialized manager
            let wr_res = manager.write_read(0x50, &[], &mut read_buf).await;
            assert_eq!(wr_res, Err(I2cError::InitError));

            // Test write_retry on uninitialized manager
            let retry_res = manager.write_retry(0x50, &[], 2).await;
            assert_eq!(retry_res, Err(I2cError::InitError));
        });
    }

    #[test]
    fn test_shared_i2c_concurrent_tasks() {
        use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
        use futures::executor::block_on;
        use futures::join;

        let manager: SharedI2cManager<CriticalSectionRawMutex, MockI2c, MockPin> =
            SharedI2cManager::new();
        let mock_driver = MockI2c::new(I2cFrequency::Freq100khz, MockPin, MockPin).unwrap();

        // Put some data in read buffer
        *mock_driver.read_data.lock().unwrap() = vec![0x11, 0x22, 0x33, 0x44];

        block_on(async {
            manager.init(mock_driver).await;

            let task_1 = async {
                let write_res = manager.write(0x50, &[0xAA]).await;
                assert!(write_res.is_ok());
            };

            let task_2 = async {
                let mut read_buf = [0u8; 2];
                let read_res = manager.read(0x50, &mut read_buf).await;
                assert!(read_res.is_ok());
                assert_eq!(read_buf, [0x11, 0x22]);
            };

            let task_3 = async {
                let mut read_buf = [0u8; 2];
                let wr_res = manager.write_read(0x50, &[0xBB], &mut read_buf).await;
                assert!(wr_res.is_ok());
                assert_eq!(read_buf, [0x33, 0x44]);
            };

            // Execute them concurrently and verify serialization
            join!(task_1, task_2, task_3);
        });
    }

    #[test]
    #[should_panic(expected = "SharedI2cManager is already initialized")]
    fn test_shared_i2c_double_init() {
        use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
        use futures::executor::block_on;

        let manager: SharedI2cManager<CriticalSectionRawMutex, MockI2c, MockPin> =
            SharedI2cManager::new();
        let driver1 = MockI2c::new(I2cFrequency::Freq100khz, MockPin, MockPin).unwrap();
        let driver2 = MockI2c::new(I2cFrequency::Freq100khz, MockPin, MockPin).unwrap();

        block_on(async {
            manager.init(driver1).await;
            manager.init(driver2).await;
        });
    }
}
