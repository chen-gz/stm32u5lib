pub mod ov5640_reg;
use crate::drivers::ov5640::ov5640_reg::*;
use crate::hal::{I2c, I2cError, Pin};
use crate::shared_i2c::SharedI2cManager;
use embassy_sync::blocking_mutex::raw::RawMutex;

pub fn setup_ov5640_camera<I2C: I2c<P>, P: Pin>(
    i2c: &mut I2C,
    power_down: &P,
    reset_b: &P,
) -> Result<(), I2cError> {
    power_down.set_low(); // enable camera
    reset_b.set_high();
    let mut read_val: [u8; 2] = [0u8; 2];
    i2c.write_read(
        OV5640_I2C_ADDR,
        &OV5640_CHIP_ID_HIGH_BYTE.to_be_bytes(),
        &mut read_val[0..1],
    )?;
    i2c.write_read(
        OV5640_I2C_ADDR,
        &OV5640_CHIP_ID_LOW_BYTE.to_be_bytes(),
        &mut read_val[1..2],
    )?;
    info!("check ov5640 chip id: {:?}", read_val);
    if read_val[0] != 0x56 || read_val[1] != 0x40 {
        return Err(I2cError::InitError);
    }

    info!("writing ov5640 common regs");
    for &(reg, val) in ov5640_reg::OV5640_COMMON.iter() {
        let reg_val = [(reg >> 8) as u8, reg as u8, val as u8];
        i2c.write_retry(ov5640_reg::OV5640_I2C_ADDR, &reg_val, 5)?;
    }

    for &(reg, val) in OV5640_PF_JPEG.iter() {
        let reg_val = [(reg >> 8) as u8, reg as u8, val as u8];
        i2c.write_retry(ov5640_reg::OV5640_I2C_ADDR, &reg_val, 5)?;
    }

    for &(reg, val) in OV5640_JPEG_MODE.iter() {
        let reg_val = [(reg >> 8) as u8, reg as u8, val as u8];
        i2c.write_retry(ov5640_reg::OV5640_I2C_ADDR, &reg_val, 5)?;
    }
    info!("writing ov5640 jpeg regs finished");

    let mut read_val = [0u8; 1];
    i2c.write_read(
        ov5640_reg::OV5640_I2C_ADDR,
        &OV5640_TIMING_TC_REG21.to_be_bytes(),
        &mut read_val,
    )?;

    let write_val = [
        (ov5640_reg::OV5640_TIMING_TC_REG21 >> 8) as u8,
        ov5640_reg::OV5640_TIMING_TC_REG21 as u8,
        read_val[0] | (1 << 5),
    ];
    i2c.write_retry(ov5640_reg::OV5640_I2C_ADDR, &write_val, 5)?;

    // SYSREM_RESET02
    i2c.write_read(
        ov5640_reg::OV5640_I2C_ADDR,
        &OV5640_SYSREM_RESET02.to_be_bytes(),
        &mut read_val,
    )?;
    let write_val = [
        (ov5640_reg::OV5640_SYSREM_RESET02 >> 8) as u8,
        ov5640_reg::OV5640_SYSREM_RESET02 as u8,
        read_val[0] & !(1 << 2 | 1 << 3 | 1 << 4),
    ];
    i2c.write_retry(ov5640_reg::OV5640_I2C_ADDR, &write_val, 5)?;

    // OV5640_CLOCK_ENABLE02
    i2c.write_read(
        ov5640_reg::OV5640_I2C_ADDR,
        &OV5640_CLOCK_ENABLE02.to_be_bytes(),
        &mut read_val,
    )?;
    let write_val = [
        (ov5640_reg::OV5640_CLOCK_ENABLE02 >> 8) as u8,
        ov5640_reg::OV5640_CLOCK_ENABLE02 as u8,
        read_val[0] | (1 << 3 | 1 << 5),
    ];
    i2c.write_retry(ov5640_reg::OV5640_I2C_ADDR, &write_val, 5)?;
    info!("setup camera registers finished");
    Ok(())
}

pub async fn setup_ov5640_camera_async<M: RawMutex, I2C: I2c<P>, P: Pin>(
    i2c: &SharedI2cManager<M, I2C, P>,
    power_down: &P,
    reset_b: &P,
) -> Result<(), I2cError> {
    power_down.set_low(); // enable camera
    reset_b.set_high();
    let mut read_val: [u8; 2] = [0u8; 2];
    i2c.write_read(
        OV5640_I2C_ADDR,
        &OV5640_CHIP_ID_HIGH_BYTE.to_be_bytes(),
        &mut read_val[0..1],
    )
    .await?;
    i2c.write_read(
        OV5640_I2C_ADDR,
        &OV5640_CHIP_ID_LOW_BYTE.to_be_bytes(),
        &mut read_val[1..2],
    )
    .await?;
    info!("check ov5640 chip id: {:?}", read_val);
    if read_val[0] != 0x56 || read_val[1] != 0x40 {
        return Err(I2cError::InitError);
    }

    info!("writing ov5640 common regs");
    for &(reg, val) in ov5640_reg::OV5640_COMMON.iter() {
        let reg_val = [(reg >> 8) as u8, reg as u8, val as u8];
        i2c.write_retry(ov5640_reg::OV5640_I2C_ADDR, &reg_val, 5)
            .await?;
    }

    for &(reg, val) in OV5640_PF_JPEG.iter() {
        let reg_val = [(reg >> 8) as u8, reg as u8, val as u8];
        i2c.write_retry(ov5640_reg::OV5640_I2C_ADDR, &reg_val, 5)
            .await?;
    }

    for &(reg, val) in OV5640_JPEG_MODE.iter() {
        let reg_val = [(reg >> 8) as u8, reg as u8, val as u8];
        i2c.write_retry(ov5640_reg::OV5640_I2C_ADDR, &reg_val, 5)
            .await?;
    }
    info!("writing ov5640 jpeg regs finished");

    let mut read_val = [0u8; 1];
    i2c.write_read(
        ov5640_reg::OV5640_I2C_ADDR,
        &OV5640_TIMING_TC_REG21.to_be_bytes(),
        &mut read_val,
    )
    .await?;

    let write_val = [
        (ov5640_reg::OV5640_TIMING_TC_REG21 >> 8) as u8,
        ov5640_reg::OV5640_TIMING_TC_REG21 as u8,
        read_val[0] | (1 << 5),
    ];
    i2c.write_retry(ov5640_reg::OV5640_I2C_ADDR, &write_val, 5)
        .await?;

    // SYSREM_RESET02
    i2c.write_read(
        ov5640_reg::OV5640_I2C_ADDR,
        &OV5640_SYSREM_RESET02.to_be_bytes(),
        &mut read_val,
    )
    .await?;
    let write_val = [
        (ov5640_reg::OV5640_SYSREM_RESET02 >> 8) as u8,
        ov5640_reg::OV5640_SYSREM_RESET02 as u8,
        read_val[0] & !(1 << 2 | 1 << 3 | 1 << 4),
    ];
    i2c.write_retry(ov5640_reg::OV5640_I2C_ADDR, &write_val, 5)
        .await?;

    // OV5640_CLOCK_ENABLE02
    i2c.write_read(
        ov5640_reg::OV5640_I2C_ADDR,
        &ov5640_reg::OV5640_CLOCK_ENABLE02.to_be_bytes(),
        &mut read_val,
    )
    .await?;
    let write_val = [
        (ov5640_reg::OV5640_CLOCK_ENABLE02 >> 8) as u8,
        ov5640_reg::OV5640_CLOCK_ENABLE02 as u8,
        read_val[0] | (1 << 3 | 1 << 5),
    ];
    i2c.write_retry(ov5640_reg::OV5640_I2C_ADDR, &write_val, 5)
        .await?;
    info!("setup camera registers finished");
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
    fn test_ov5640_setup() {
        let pd = MockPin {
            setup_called: Cell::new(false),
            high_called: Cell::new(false),
            low_called: Cell::new(false),
            toggle_called: Cell::new(false),
        };
        let rst = MockPin {
            setup_called: Cell::new(false),
            high_called: Cell::new(false),
            low_called: Cell::new(false),
            toggle_called: Cell::new(false),
        };

        let mut i2c = MockI2c {
            write_log: Rc::new(RefCell::new(Vec::new())),
            read_responses: Rc::new(RefCell::new(vec![
                vec![0x56], // CHIP_ID_HIGH_BYTE
                vec![0x40], // CHIP_ID_LOW_BYTE
                vec![0x00], // TIMING_TC_REG21 read
                vec![0x00], // SYSREM_RESET02 read
                vec![0x00], // CLOCK_ENABLE02 read
            ])),
            fail_at_tx: Rc::new(Cell::new(None)),
            tx_count: Rc::new(Cell::new(0)),
        };

        assert!(setup_ov5640_camera(&mut i2c, &pd, &rst).is_ok());
        assert!(pd.low_called.get());
        assert!(rst.high_called.get());

        let log = i2c.write_log.borrow();
        assert!(log.len() > 10);
    }

    #[test]
    fn test_ov5640_setup_async() {
        let pd = MockPin {
            setup_called: Cell::new(false),
            high_called: Cell::new(false),
            low_called: Cell::new(false),
            toggle_called: Cell::new(false),
        };
        let rst = MockPin {
            setup_called: Cell::new(false),
            high_called: Cell::new(false),
            low_called: Cell::new(false),
            toggle_called: Cell::new(false),
        };

        let i2c = MockI2c {
            write_log: Rc::new(RefCell::new(Vec::new())),
            read_responses: Rc::new(RefCell::new(vec![
                vec![0x56], // CHIP_ID_HIGH_BYTE
                vec![0x40], // CHIP_ID_LOW_BYTE
                vec![0x00], // TIMING_TC_REG21 read
                vec![0x00], // SYSREM_RESET02 read
                vec![0x00], // CLOCK_ENABLE02 read
            ])),
            fail_at_tx: Rc::new(Cell::new(None)),
            tx_count: Rc::new(Cell::new(0)),
        };

        let manager = SharedI2cManager::<CriticalSectionRawMutex, _, _>::new();
        block_on(async {
            manager.init(i2c.clone()).await;
            assert!(setup_ov5640_camera_async(&manager, &pd, &rst).await.is_ok());
            assert!(pd.low_called.get());
            assert!(rst.high_called.get());
        });
    }

    #[test]
    fn test_ov5640_setup_chip_id_mismatch() {
        let pd = MockPin {
            setup_called: Cell::new(false),
            high_called: Cell::new(false),
            low_called: Cell::new(false),
            toggle_called: Cell::new(false),
        };
        let rst = MockPin {
            setup_called: Cell::new(false),
            high_called: Cell::new(false),
            low_called: Cell::new(false),
            toggle_called: Cell::new(false),
        };

        let mut i2c = MockI2c {
            write_log: Rc::new(RefCell::new(Vec::new())),
            read_responses: Rc::new(RefCell::new(vec![vec![0x00], vec![0x00]])),
            fail_at_tx: Rc::new(Cell::new(None)),
            tx_count: Rc::new(Cell::new(0)),
        };

        assert!(setup_ov5640_camera(&mut i2c, &pd, &rst).is_err());
    }

    #[test]
    fn test_ov5640_setup_chip_id_mismatch_async() {
        let pd = MockPin {
            setup_called: Cell::new(false),
            high_called: Cell::new(false),
            low_called: Cell::new(false),
            toggle_called: Cell::new(false),
        };
        let rst = MockPin {
            setup_called: Cell::new(false),
            high_called: Cell::new(false),
            low_called: Cell::new(false),
            toggle_called: Cell::new(false),
        };

        let i2c = MockI2c {
            write_log: Rc::new(RefCell::new(Vec::new())),
            read_responses: Rc::new(RefCell::new(vec![vec![0x00], vec![0x00]])),
            fail_at_tx: Rc::new(Cell::new(None)),
            tx_count: Rc::new(Cell::new(0)),
        };

        let manager = SharedI2cManager::<CriticalSectionRawMutex, _, _>::new();
        block_on(async {
            manager.init(i2c.clone()).await;
            assert!(setup_ov5640_camera_async(&manager, &pd, &rst)
                .await
                .is_err());
        });
    }

    #[test]
    fn test_ov5640_errors() {
        let pd = MockPin {
            setup_called: Cell::new(false),
            high_called: Cell::new(false),
            low_called: Cell::new(false),
            toggle_called: Cell::new(false),
        };
        let rst = MockPin {
            setup_called: Cell::new(false),
            high_called: Cell::new(false),
            low_called: Cell::new(false),
            toggle_called: Cell::new(false),
        };

        // First run a successful setup to count the exact number of transactions
        let mut i2c_success = MockI2c {
            write_log: Rc::new(RefCell::new(Vec::new())),
            read_responses: Rc::new(RefCell::new(vec![
                vec![0x56], // CHIP_ID_HIGH_BYTE
                vec![0x40], // CHIP_ID_LOW_BYTE
                vec![0x00], // TIMING_TC_REG21 read
                vec![0x00], // SYSREM_RESET02 read
                vec![0x00], // CLOCK_ENABLE02 read
            ])),
            fail_at_tx: Rc::new(Cell::new(None)),
            tx_count: Rc::new(Cell::new(0)),
        };
        assert!(setup_ov5640_camera(&mut i2c_success, &pd, &rst).is_ok());
        let total_tx = i2c_success.tx_count.get();

        // Inject errors at each transaction step to hit all early return paths
        for i in 0..total_tx {
            let mut i2c = MockI2c {
                write_log: Rc::new(RefCell::new(Vec::new())),
                read_responses: Rc::new(RefCell::new(vec![
                    vec![0x56], // CHIP_ID_HIGH_BYTE
                    vec![0x40], // CHIP_ID_LOW_BYTE
                    vec![0x00], // TIMING_TC_REG21 read
                    vec![0x00], // SYSREM_RESET02 read
                    vec![0x00], // CLOCK_ENABLE02 read
                ])),
                fail_at_tx: Rc::new(Cell::new(Some(i))),
                tx_count: Rc::new(Cell::new(0)),
            };
            assert!(setup_ov5640_camera(&mut i2c, &pd, &rst).is_err());
        }
    }

    #[test]
    fn test_ov5640_errors_async() {
        let pd = MockPin {
            setup_called: Cell::new(false),
            high_called: Cell::new(false),
            low_called: Cell::new(false),
            toggle_called: Cell::new(false),
        };
        let rst = MockPin {
            setup_called: Cell::new(false),
            high_called: Cell::new(false),
            low_called: Cell::new(false),
            toggle_called: Cell::new(false),
        };

        let manager = SharedI2cManager::<CriticalSectionRawMutex, _, _>::new();
        block_on(async {
            // First run a successful setup to count the exact number of transactions
            let i2c_success = MockI2c {
                write_log: Rc::new(RefCell::new(Vec::new())),
                read_responses: Rc::new(RefCell::new(vec![
                    vec![0x56], // CHIP_ID_HIGH_BYTE
                    vec![0x40], // CHIP_ID_LOW_BYTE
                    vec![0x00], // TIMING_TC_REG21 read
                    vec![0x00], // SYSREM_RESET02 read
                    vec![0x00], // CLOCK_ENABLE02 read
                ])),
                fail_at_tx: Rc::new(Cell::new(None)),
                tx_count: Rc::new(Cell::new(0)),
            };
            manager.init(i2c_success.clone()).await;
            assert!(setup_ov5640_camera_async(&manager, &pd, &rst).await.is_ok());
            let total_tx = i2c_success.tx_count.get();

            // Inject errors at each transaction step to hit all early return paths
            for i in 0..total_tx {
                let i2c = MockI2c {
                    write_log: Rc::new(RefCell::new(Vec::new())),
                    read_responses: Rc::new(RefCell::new(vec![
                        vec![0x56], // CHIP_ID_HIGH_BYTE
                        vec![0x40], // CHIP_ID_LOW_BYTE
                        vec![0x00], // TIMING_TC_REG21 read
                        vec![0x00], // SYSREM_RESET02 read
                        vec![0x00], // CLOCK_ENABLE02 read
                    ])),
                    fail_at_tx: Rc::new(Cell::new(Some(i))),
                    tx_count: Rc::new(Cell::new(0)),
                };
                manager.init(i2c).await;
                assert!(setup_ov5640_camera_async(&manager, &pd, &rst)
                    .await
                    .is_err());
            }
        });
    }
}
