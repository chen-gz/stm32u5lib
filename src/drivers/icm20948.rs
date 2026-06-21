use crate::hal::{I2c, I2cError, Pin};
use crate::shared_i2c::SharedI2cManager;
use embassy_sync::blocking_mutex::raw::RawMutex;

pub const ICM20948_ADDR: u16 = 0x68 << 1;
pub const ICM20948_WHO_AM_I: u8 = 0x00;
pub const ICM20948_GYRO_CONFIG_1: u8 = 0x01;
pub const ICM20948_GYRO_CONFIG_2: u8 = 0x02;
pub const ICM20948_USER_CTRL: u8 = 0x03;
pub const ICM20948_LP_CONFIG: u8 = 0x05;
pub const ICM20948_PWR_MGMT_1: u8 = 0x06;
pub const ICM20948_PWR_MGMT_2: u8 = 0x07;
pub const ICM20948_ACCEL_CONFIG: u8 = 0x14;
pub const ICM20948_ACCEL_CONFIG2: u8 = 0x15;
pub const ICM20948_ACC_XOUT_H: u8 = 0x2d;
pub const ICM20948_ACC_XOUT_L: u8 = 0x2e;
pub const ICM20948_ACC_YOUT_H: u8 = 0x2f;
pub const ICM20948_ACC_YOUT_L: u8 = 0x30;
pub const ICM20948_ACC_ZOUT_H: u8 = 0x31;
pub const ICM20948_ACC_ZOUT_L: u8 = 0x32;
pub const ICM20948_BANK_SEL: u8 = 0x7f;

/// Reads IMU data synchronously.
pub fn icm20948_read_imu<I2C: I2c<P>, P: Pin>(i2c: &mut I2C) -> Result<[u8; 6], I2cError> {
    let mut buf = [0u8; 6];
    i2c.write_read(ICM20948_ADDR, &[ICM20948_ACC_XOUT_H], &mut buf[0..1])?;
    i2c.write_read(ICM20948_ADDR, &[ICM20948_ACC_XOUT_L], &mut buf[1..2])?;
    i2c.write_read(ICM20948_ADDR, &[ICM20948_ACC_YOUT_H], &mut buf[2..3])?;
    i2c.write_read(ICM20948_ADDR, &[ICM20948_ACC_YOUT_L], &mut buf[3..4])?;
    i2c.write_read(ICM20948_ADDR, &[ICM20948_ACC_ZOUT_H], &mut buf[4..5])?;
    i2c.write_read(ICM20948_ADDR, &[ICM20948_ACC_ZOUT_L], &mut buf[5..6])?;
    Ok(buf)
}

/// Sets up the IMU synchronously.
pub fn icm20948_setup<I2C: I2c<P>, P: Pin>(i2c: &mut I2C, fsync: &P) -> Result<(), I2cError> {
    fsync.setup();
    fsync.set_low();
    info!("start setup imu");

    let mut buf = [0u8; 1];
    i2c.write_read(ICM20948_ADDR, &[ICM20948_WHO_AM_I], &mut buf)?;
    info!("imu who am i: {:?}", buf[0]);

    i2c.write(ICM20948_ADDR, &[ICM20948_PWR_MGMT_1, 0x00])?;
    i2c.write(ICM20948_ADDR, &[ICM20948_PWR_MGMT_1, 0x80])?;
    i2c.write(ICM20948_ADDR, &[ICM20948_ACCEL_CONFIG, 0x18])?;
    i2c.write(ICM20948_ADDR, &[ICM20948_ACCEL_CONFIG2, 0x00])?;
    i2c.write(ICM20948_ADDR, &[ICM20948_GYRO_CONFIG_1, 0x18])?;
    i2c.write(ICM20948_ADDR, &[ICM20948_GYRO_CONFIG_2, 0x00])?;
    i2c.write(ICM20948_ADDR, &[ICM20948_PWR_MGMT_2, 0x00])?;
    i2c.write(ICM20948_ADDR, &[ICM20948_PWR_MGMT_1, 0x09])?;
    Ok(())
}

/// Reads IMU data asynchronously using the Shared I2C Manager.
pub async fn icm20948_read_imu_async<M: RawMutex, I2C: I2c<P>, P: Pin>(
    i2c: &SharedI2cManager<M, I2C, P>,
) -> Result<[u8; 6], I2cError> {
    let mut buf = [0u8; 6];
    i2c.write_read(ICM20948_ADDR, &[ICM20948_ACC_XOUT_H], &mut buf[0..1])
        .await?;
    i2c.write_read(ICM20948_ADDR, &[ICM20948_ACC_XOUT_L], &mut buf[1..2])
        .await?;
    i2c.write_read(ICM20948_ADDR, &[ICM20948_ACC_YOUT_H], &mut buf[2..3])
        .await?;
    i2c.write_read(ICM20948_ADDR, &[ICM20948_ACC_YOUT_L], &mut buf[3..4])
        .await?;
    i2c.write_read(ICM20948_ADDR, &[ICM20948_ACC_ZOUT_H], &mut buf[4..5])
        .await?;
    i2c.write_read(ICM20948_ADDR, &[ICM20948_ACC_ZOUT_L], &mut buf[5..6])
        .await?;
    Ok(buf)
}

/// Sets up the IMU asynchronously using the Shared I2C Manager.
pub async fn icm20948_setup_async<M: RawMutex, I2C: I2c<P>, P: Pin>(
    i2c: &SharedI2cManager<M, I2C, P>,
    fsync: &P,
) -> Result<(), I2cError> {
    fsync.setup();
    fsync.set_low();
    info!("start setup imu");

    let mut buf = [0u8; 1];
    i2c.write_read(ICM20948_ADDR, &[ICM20948_WHO_AM_I], &mut buf)
        .await?;
    info!("imu who am i: {:?}", buf[0]);

    i2c.write(ICM20948_ADDR, &[ICM20948_PWR_MGMT_1, 0x00])
        .await?;
    i2c.write(ICM20948_ADDR, &[ICM20948_PWR_MGMT_1, 0x80])
        .await?;
    i2c.write(ICM20948_ADDR, &[ICM20948_ACCEL_CONFIG, 0x18])
        .await?;
    i2c.write(ICM20948_ADDR, &[ICM20948_ACCEL_CONFIG2, 0x00])
        .await?;
    i2c.write(ICM20948_ADDR, &[ICM20948_GYRO_CONFIG_1, 0x18])
        .await?;
    i2c.write(ICM20948_ADDR, &[ICM20948_GYRO_CONFIG_2, 0x00])
        .await?;
    i2c.write(ICM20948_ADDR, &[ICM20948_PWR_MGMT_2, 0x00])
        .await?;
    i2c.write(ICM20948_ADDR, &[ICM20948_PWR_MGMT_1, 0x09])
        .await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
    use futures::executor::block_on;
    use std::cell::{Cell, RefCell};
    use std::rc::Rc;

    struct MockPin {
        setup_called: Cell<bool>,
        high_called: Cell<bool>,
        low_called: Cell<bool>,
        toggle_called: Cell<bool>,
    }

    impl Pin for MockPin {
        fn setup(&self) {
            self.setup_called.set(true);
        }
        fn set_high(&self) {
            self.high_called.set(true);
        }
        fn set_low(&self) {
            self.low_called.set(true);
        }
        fn toggle(&self) {
            self.toggle_called.set(true);
        }
    }

    #[derive(Clone)]
    struct MockI2c {
        write_log: Rc<RefCell<Vec<(u16, Vec<u8>)>>>,
        read_responses: Rc<RefCell<Vec<Vec<u8>>>>,
        fail_at_tx: Rc<Cell<Option<usize>>>,
        tx_count: Rc<Cell<usize>>,
    }

    impl MockI2c {
        fn check_fail(&self) -> Result<(), I2cError> {
            let count = self.tx_count.get();
            self.tx_count.set(count + 1);
            if let Some(fail_idx) = self.fail_at_tx.get() {
                if count >= fail_idx {
                    return Err(I2cError::BusError);
                }
            }
            Ok(())
        }
    }

    impl I2c<MockPin> for MockI2c {
        fn new(
            _freq: crate::hal::I2cFrequency,
            _sda: MockPin,
            _scl: MockPin,
        ) -> Result<Self, I2cError> {
            Ok(Self {
                write_log: Rc::new(RefCell::new(Vec::new())),
                read_responses: Rc::new(RefCell::new(Vec::new())),
                fail_at_tx: Rc::new(Cell::new(None)),
                tx_count: Rc::new(Cell::new(0)),
            })
        }

        fn write(&self, addr: u16, data: &[u8]) -> Result<(), I2cError> {
            self.check_fail()?;
            self.write_log.borrow_mut().push((addr, data.to_vec()));
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
            self.check_fail()?;
            let mut responses = self.read_responses.borrow_mut();
            if !responses.is_empty() {
                let resp = responses.remove(0);
                let len = resp.len().min(data.len());
                data[..len].copy_from_slice(&resp[..len]);
            }
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
            self.write(addr, write_data)?;
            self.read(addr, read_data)
        }

        fn capacity(&self) -> crate::hal::I2cFrequency {
            crate::hal::I2cFrequency::Freq400khz
        }
    }

    #[test]
    fn test_mock_helpers() {
        let pin = MockPin {
            setup_called: Cell::new(false),
            high_called: Cell::new(false),
            low_called: Cell::new(false),
            toggle_called: Cell::new(false),
        };
        pin.setup();
        pin.set_high();
        pin.set_low();
        pin.toggle();
        assert!(pin.setup_called.get());
        assert!(pin.high_called.get());
        assert!(pin.low_called.get());
        assert!(pin.toggle_called.get());

        let pin1 = MockPin {
            setup_called: Cell::new(false),
            high_called: Cell::new(false),
            low_called: Cell::new(false),
            toggle_called: Cell::new(false),
        };
        let pin2 = MockPin {
            setup_called: Cell::new(false),
            high_called: Cell::new(false),
            low_called: Cell::new(false),
            toggle_called: Cell::new(false),
        };
        let i2c = MockI2c::new(crate::hal::I2cFrequency::Freq100khz, pin1, pin2).unwrap();
        assert_eq!(i2c.capacity(), crate::hal::I2cFrequency::Freq400khz);
        let mut buf = [0u8; 1];
        block_on(i2c.read_async(0, &mut buf)).unwrap();

        // Exercise error paths inside MockI2c methods directly
        i2c.fail_at_tx.set(Some(0));
        assert!(i2c.write(0, &[]).is_err());
        assert!(i2c.read(0, &mut buf).is_err());
        assert!(block_on(i2c.read_async(0, &mut buf)).is_err());
    }

    #[test]
    fn test_icm20948_setup_and_read() {
        let pin = MockPin {
            setup_called: Cell::new(false),
            high_called: Cell::new(false),
            low_called: Cell::new(false),
            toggle_called: Cell::new(false),
        };
        let mut i2c = MockI2c {
            write_log: Rc::new(RefCell::new(Vec::new())),
            read_responses: Rc::new(RefCell::new(vec![vec![0xEA]])), // WHO_AM_I
            fail_at_tx: Rc::new(Cell::new(None)),
            tx_count: Rc::new(Cell::new(0)),
        };

        assert!(icm20948_setup(&mut i2c, &pin).is_ok());
        assert!(pin.setup_called.get());
        assert!(pin.low_called.get());

        {
            let log = i2c.write_log.borrow();
            assert_eq!(log[0], (ICM20948_ADDR, vec![ICM20948_WHO_AM_I]));
            assert_eq!(log[1], (ICM20948_ADDR, vec![ICM20948_PWR_MGMT_1, 0x00]));
            assert_eq!(log[2], (ICM20948_ADDR, vec![ICM20948_PWR_MGMT_1, 0x80]));
        }

        // Test read
        i2c.read_responses.borrow_mut().clear();
        i2c.read_responses.borrow_mut().extend(vec![
            vec![11],
            vec![12],
            vec![21],
            vec![22],
            vec![31],
            vec![32],
        ]);
        let data = icm20948_read_imu(&mut i2c).unwrap();
        assert_eq!(data, [11, 12, 21, 22, 31, 32]);
    }

    #[test]
    fn test_icm20948_setup_and_read_async() {
        let pin = MockPin {
            setup_called: Cell::new(false),
            high_called: Cell::new(false),
            low_called: Cell::new(false),
            toggle_called: Cell::new(false),
        };
        let i2c = MockI2c {
            write_log: Rc::new(RefCell::new(Vec::new())),
            read_responses: Rc::new(RefCell::new(vec![vec![0xEA]])), // WHO_AM_I
            fail_at_tx: Rc::new(Cell::new(None)),
            tx_count: Rc::new(Cell::new(0)),
        };

        let manager = SharedI2cManager::<CriticalSectionRawMutex, _, _>::new();
        block_on(async {
            manager.init(i2c.clone()).await;

            assert!(icm20948_setup_async(&manager, &pin).await.is_ok());
            assert!(pin.setup_called.get());
            assert!(pin.low_called.get());

            // Test read async
            i2c.read_responses.borrow_mut().clear();
            i2c.read_responses.borrow_mut().extend(vec![
                vec![101],
                vec![102],
                vec![103],
                vec![104],
                vec![105],
                vec![106],
            ]);
            let data = icm20948_read_imu_async(&manager).await.unwrap();
            assert_eq!(data, [101, 102, 103, 104, 105, 106]);
        });
    }

    #[test]
    fn test_icm20948_errors() {
        let pin = MockPin {
            setup_called: Cell::new(false),
            high_called: Cell::new(false),
            low_called: Cell::new(false),
            toggle_called: Cell::new(false),
        };

        // First run a successful setup to count the exact number of transactions
        let mut i2c_setup_success = MockI2c {
            write_log: Rc::new(RefCell::new(Vec::new())),
            read_responses: Rc::new(RefCell::new(vec![vec![0xEA]])),
            fail_at_tx: Rc::new(Cell::new(None)),
            tx_count: Rc::new(Cell::new(0)),
        };
        assert!(icm20948_setup(&mut i2c_setup_success, &pin).is_ok());
        let setup_tx_count = i2c_setup_success.tx_count.get();

        // Inject errors at each step
        for i in 0..setup_tx_count {
            let mut i2c = MockI2c {
                write_log: Rc::new(RefCell::new(Vec::new())),
                read_responses: Rc::new(RefCell::new(vec![vec![0xEA]])),
                fail_at_tx: Rc::new(Cell::new(Some(i))),
                tx_count: Rc::new(Cell::new(0)),
            };
            assert!(icm20948_setup(&mut i2c, &pin).is_err());
        }

        // Do the same for read_imu
        let mut i2c_read_success = MockI2c {
            write_log: Rc::new(RefCell::new(Vec::new())),
            read_responses: Rc::new(RefCell::new(vec![
                vec![11],
                vec![12],
                vec![21],
                vec![22],
                vec![31],
                vec![32],
            ])),
            fail_at_tx: Rc::new(Cell::new(None)),
            tx_count: Rc::new(Cell::new(0)),
        };
        assert!(icm20948_read_imu(&mut i2c_read_success).is_ok());
        let read_tx_count = i2c_read_success.tx_count.get();

        for i in 0..read_tx_count {
            let mut i2c = MockI2c {
                write_log: Rc::new(RefCell::new(Vec::new())),
                read_responses: Rc::new(RefCell::new(vec![
                    vec![11],
                    vec![12],
                    vec![21],
                    vec![22],
                    vec![31],
                    vec![32],
                ])),
                fail_at_tx: Rc::new(Cell::new(Some(i))),
                tx_count: Rc::new(Cell::new(0)),
            };
            assert!(icm20948_read_imu(&mut i2c).is_err());
        }
    }

    #[test]
    fn test_icm20948_errors_async() {
        let pin = MockPin {
            setup_called: Cell::new(false),
            high_called: Cell::new(false),
            low_called: Cell::new(false),
            toggle_called: Cell::new(false),
        };

        block_on(async {
            let manager = SharedI2cManager::<CriticalSectionRawMutex, _, _>::new();
            // First run successful setups to count transactions
            let i2c_setup_success = MockI2c {
                write_log: Rc::new(RefCell::new(Vec::new())),
                read_responses: Rc::new(RefCell::new(vec![vec![0xEA]])),
                fail_at_tx: Rc::new(Cell::new(None)),
                tx_count: Rc::new(Cell::new(0)),
            };
            manager.init(i2c_setup_success.clone()).await;
            assert!(icm20948_setup_async(&manager, &pin).await.is_ok());
            let setup_tx_count = i2c_setup_success.tx_count.get();

            for i in 0..setup_tx_count {
                let manager = SharedI2cManager::<CriticalSectionRawMutex, _, _>::new();
                let i2c = MockI2c {
                    write_log: Rc::new(RefCell::new(Vec::new())),
                    read_responses: Rc::new(RefCell::new(vec![vec![0xEA]])),
                    fail_at_tx: Rc::new(Cell::new(Some(i))),
                    tx_count: Rc::new(Cell::new(0)),
                };
                manager.init(i2c).await;
                assert!(icm20948_setup_async(&manager, &pin).await.is_err());
            }

            // Do the same for read_imu_async
            let manager = SharedI2cManager::<CriticalSectionRawMutex, _, _>::new();
            let i2c_read_success = MockI2c {
                write_log: Rc::new(RefCell::new(Vec::new())),
                read_responses: Rc::new(RefCell::new(vec![
                    vec![11],
                    vec![12],
                    vec![21],
                    vec![22],
                    vec![31],
                    vec![32],
                ])),
                fail_at_tx: Rc::new(Cell::new(None)),
                tx_count: Rc::new(Cell::new(0)),
            };
            manager.init(i2c_read_success.clone()).await;
            assert!(icm20948_read_imu_async(&manager).await.is_ok());
            let read_tx_count = i2c_read_success.tx_count.get();

            for i in 0..read_tx_count {
                let manager = SharedI2cManager::<CriticalSectionRawMutex, _, _>::new();
                let i2c = MockI2c {
                    write_log: Rc::new(RefCell::new(Vec::new())),
                    read_responses: Rc::new(RefCell::new(vec![
                        vec![11],
                        vec![12],
                        vec![21],
                        vec![22],
                        vec![31],
                        vec![32],
                    ])),
                    fail_at_tx: Rc::new(Cell::new(Some(i))),
                    tx_count: Rc::new(Cell::new(0)),
                };
                manager.init(i2c).await;
                assert!(icm20948_read_imu_async(&manager).await.is_err());
            }
        });
    }
}
