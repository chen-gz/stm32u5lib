#![no_std]
#![no_main]
#![allow(dead_code, unused)]
#[cfg(feature = "defmt")]
use defmt_rtt as _;
use u5_lib as _; // links panic handler

#[embedded_test::tests]
mod tests {
    use embassy_futures::join::join;
    use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
    use u5_lib::gpio::{GpioPort, I2C1_SCL_PB8, I2C1_SDA_PB9, I2C2_SCL_PB13, I2C2_SDA_PB14};
    use u5_lib::hal::{I2c, I2cFrequency, I2cSlave, I2cSlaveEvent};
    use u5_lib::i2c::I2c as I2cDriver;
    use u5_lib::shared_i2c::SharedI2cManager;

    // 1. Declare the static global shared I2C manager.
    static SHARED_I2C: SharedI2cManager<CriticalSectionRawMutex, I2cDriver, GpioPort> =
        SharedI2cManager::new();

    #[init]
    fn init() {
        u5_lib::clock::init_clock(true, u5_lib::clock::ClockFreqs::KernelFreq160Mhz);
    }

    // #[test]
    async fn test_shared_i2c_communication() {
        // Host: I2C1 with PB8 (SCL) and PB9 (SDA)
        let host_driver =
            <I2cDriver as I2c<GpioPort>>::new(I2cFrequency::Freq100khz, I2C1_SDA_PB9, I2C1_SCL_PB8)
                .unwrap();

        // 2. Initialize the shared I2C manager.
        SHARED_I2C.init(host_driver).await;

        // Slave: I2C2 with PB13 (SCL) and PB14 (SDA), Address 0x50
        let slave =
            <I2cDriver as I2cSlave<GpioPort>>::new_slave(I2C2_SDA_PB14, I2C2_SCL_PB13, 0x50)
                .unwrap();

        // We run two concurrent host write tasks, and one host read task, to verify serialization.
        let host_task = async {
            // Give slave time to start
            u5_lib::clock::delay_ms(100);

            // Write Task 1
            let write_task_1 = async {
                let write_data = [0xAA, 0xBB];
                #[cfg(feature = "defmt")]
                defmt::info!("Host Task 1: Writing...");
                let res = SHARED_I2C.write(0x50, &write_data).await;
                res.expect("Host Task 1 write failed");
            };

            // Write Task 2
            let write_task_2 = async {
                let write_data = [0xCC, 0xDD];
                #[cfg(feature = "defmt")]
                defmt::info!("Host Task 2: Writing...");
                let res = SHARED_I2C.write(0x50, &write_data).await;
                res.expect("Host Task 2 write failed");
            };

            // Join the two write tasks. They will execute concurrently and be serialized by the SharedI2cManager.
            join(write_task_1, write_task_2).await;

            // Host: Read from slave
            u5_lib::clock::delay_ms(100);
            let mut read_buf = [0u8; 4];
            #[cfg(feature = "defmt")]
            defmt::info!("Host: Reading from slave...");
            let res = SHARED_I2C.read(0x50, &mut read_buf).await;
            #[cfg(feature = "defmt")]
            defmt::info!("Host: Read result: {:?} data: {:x}", res, read_buf);
            res.expect("Host read failed");
            assert_eq!(read_buf, [0x11, 0x22, 0x33, 0x44]);
        };

        let slave_task = async {
            // Slave: Wait for first write (2 bytes)
            #[cfg(feature = "defmt")]
            defmt::info!("Slave: Waiting for write 1...");
            let event = slave
                .slave_wait_address_async()
                .await
                .expect("Slave wait_address failed");
            match event {
                I2cSlaveEvent::Write => {
                    let mut rx_buf = [0u8; 2];
                    slave
                        .slave_read_async(&mut rx_buf)
                        .await
                        .expect("Slave read failed");
                    #[cfg(feature = "defmt")]
                    defmt::info!("Slave: Received data 1: {:x}", rx_buf);
                }
                I2cSlaveEvent::Read => panic!("Slave expected Write, got Read"),
            }

            // Slave: Wait for second write (2 bytes)
            #[cfg(feature = "defmt")]
            defmt::info!("Slave: Waiting for write 2...");
            let event = slave
                .slave_wait_address_async()
                .await
                .expect("Slave wait_address failed");
            match event {
                I2cSlaveEvent::Write => {
                    let mut rx_buf = [0u8; 2];
                    slave
                        .slave_read_async(&mut rx_buf)
                        .await
                        .expect("Slave read failed");
                    #[cfg(feature = "defmt")]
                    defmt::info!("Slave: Received data 2: {:x}", rx_buf);
                }
                I2cSlaveEvent::Read => panic!("Slave expected Write, got Read"),
            }

            // Slave: Wait for read request and reply
            #[cfg(feature = "defmt")]
            defmt::info!("Slave: Waiting for read...");
            let event = slave
                .slave_wait_address_async()
                .await
                .expect("Slave wait_address failed");
            match event {
                I2cSlaveEvent::Read => {
                    let tx_data = [0x11, 0x22, 0x33, 0x44];
                    slave
                        .slave_write_async(&tx_data)
                        .await
                        .expect("Slave write failed");
                    #[cfg(feature = "defmt")]
                    defmt::info!("Slave: Sent data: {:x}", tx_data);
                }
                I2cSlaveEvent::Write => panic!("Slave expected Read, got Write"),
            }
        };

        join(host_task, slave_task).await;

        #[cfg(feature = "defmt")]
        defmt::info!("I2C Shared Host-Slave test passed!");
    }
}
