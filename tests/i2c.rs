#![no_std]
#![no_main]

#[cfg(feature = "defmt")]
use defmt_rtt as _;
use u5_lib as _; // links panic handler

#[embedded_test::tests]
mod tests {
    use u5_lib::hal::{I2c, I2cSlave, I2cSlaveEvent};
    use u5_lib::i2c::I2c as I2cDriver;
    use u5_lib::hal::I2cFrequency;
    use u5_lib::gpio::{I2C1_SCL_PB8, I2C1_SDA_PB9, I2C2_SCL_PB13, I2C2_SDA_PB14, GpioPort};
    use embassy_futures::join::join;

    #[init]
    fn init() {
        u5_lib::clock::init_clock(true, u5_lib::clock::ClockFreqs::KernelFreq160Mhz);
    }

    #[test]
    async fn test_i2c_host_slave_communication() {
        // Host: I2C1 with PB8 (SCL) and PB9 (SDA)
        let host = <I2cDriver as I2c<GpioPort>>::new(I2cFrequency::Freq100khz, I2C1_SDA_PB9, I2C1_SCL_PB8).unwrap();

        // Slave: I2C2 with PB13 (SCL) and PB14 (SDA), Address 0x50
        let slave = <I2cDriver as I2cSlave<GpioPort>>::new_slave(I2C2_SDA_PB14, I2C2_SCL_PB13, 0x50).unwrap();

        let host_task = async {
            // Give slave time to start
            u5_lib::clock::delay_ms(100);

            // Write to slave
            let write_data = [0xAA, 0xBB, 0xCC, 0xDD];
            #[cfg(feature = "defmt")]
            defmt::info!("Host: Writing to slave...");
            let res = host.write_async(0x50, &write_data).await;
            #[cfg(feature = "defmt")]
            defmt::info!("Host: Write result: {:?}", res);
            res.expect("Host write failed");

            // Read from slave
            u5_lib::clock::delay_ms(100);
            let mut read_buf = [0u8; 4];
            #[cfg(feature = "defmt")]
            defmt::info!("Host: Reading from slave...");
            let res = host.read_async(0x50, &mut read_buf).await;
            #[cfg(feature = "defmt")]
            defmt::info!("Host: Read result: {:?} data: {:x}", res, read_buf);
            res.expect("Host read failed");
            assert_eq!(read_buf, [0x11, 0x22, 0x33, 0x44]);
        };

        let slave_task = async {
            // 1. Wait for Host Write
            #[cfg(feature = "defmt")]
            defmt::info!("Slave: Waiting for address (expect Write)...");
            let event = slave.slave_wait_address_async().await.expect("Slave wait_address failed");
            match event {
                I2cSlaveEvent::Write => {
                    let mut rx_buf = [0u8; 4];
                    slave.slave_read_async(&mut rx_buf).await.expect("Slave read failed");
                    #[cfg(feature = "defmt")]
                    defmt::info!("Slave: Received data: {:x}", rx_buf);
                    assert_eq!(rx_buf, [0xAA, 0xBB, 0xCC, 0xDD]);
                }
                I2cSlaveEvent::Read => panic!("Slave expected Write, got Read"),
            }

            // 2. Wait for Host Read
            #[cfg(feature = "defmt")]
            defmt::info!("Slave: Waiting for address (expect Read)...");
            let event = slave.slave_wait_address_async().await.expect("Slave wait_address failed");
            match event {
                I2cSlaveEvent::Read => {
                    let tx_data = [0x11, 0x22, 0x33, 0x44];
                    slave.slave_write_async(&tx_data).await.expect("Slave write failed");
                    #[cfg(feature = "defmt")]
                    defmt::info!("Slave: Sent data: {:x}", tx_data);
                }
                I2cSlaveEvent::Write => panic!("Slave expected Read, got Write"),
            }
        };

        join(host_task, slave_task).await;
        
        #[cfg(feature = "defmt")]
        defmt::info!("I2C Host-Slave test passed!");
    }
}
