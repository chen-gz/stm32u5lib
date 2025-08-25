use crate::hal::I2c;
use crate::{gpio, i2c};

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

pub fn icm20948_read_imu(i2c: &mut i2c::I2c) -> [u8; 6] {
    let mut buf = [0u8; 6];
    i2c.write_read(ICM20948_ADDR, &mut [ICM20948_ACC_XOUT_H], &mut buf[0..1])
        .unwrap();
    i2c.write_read(ICM20948_ADDR, &mut [ICM20948_ACC_XOUT_L], &mut buf[1..2])
        .unwrap();
    i2c.write_read(ICM20948_ADDR, &mut [ICM20948_ACC_YOUT_H], &mut buf[2..3])
        .unwrap();
    i2c.write_read(ICM20948_ADDR, &mut [ICM20948_ACC_YOUT_L], &mut buf[3..4])
        .unwrap();
    i2c.write_read(ICM20948_ADDR, &mut [ICM20948_ACC_ZOUT_H], &mut buf[4..5])
        .unwrap();
    i2c.write_read(ICM20948_ADDR, &mut [ICM20948_ACC_ZOUT_L], &mut buf[5..6])
        .unwrap();
    return buf;
}
pub fn icm20948_setup(i2c: &mut i2c::I2c, fsync: &mut gpio::GpioPort) {
    // pa3 to ground
    // let fsync = gpio::PA3;
    fsync.setup();
    fsync.set_low();
    defmt::info!("start setup imu");
    // icm-20948
    let mut buf = [0u8; 1];
    i2c.write_read(ICM20948_ADDR, &mut [ICM20948_WHO_AM_I], &mut buf)
        .unwrap();
    // read who am i
    defmt::info!("imu who am i: {:?}", buf[0]);
    i2c.write(ICM20948_ADDR, &[ICM20948_PWR_MGMT_1, 0x00])
        .unwrap();

    // reset the device
    i2c.write(ICM20948_ADDR, &[ICM20948_PWR_MGMT_1, 0x80])
        .unwrap();

    // config the accelerometer
    i2c.write(ICM20948_ADDR, &[ICM20948_ACCEL_CONFIG, 0x18])
        .unwrap(); // config accelerometer full scale range to 16g
    i2c.write(ICM20948_ADDR, &[ICM20948_ACCEL_CONFIG2, 0x00])
        .unwrap(); // disable accelerometer low pass filter

    // config the gyroscope
    i2c.write(ICM20948_ADDR, &[ICM20948_GYRO_CONFIG_1, 0x18])
        .unwrap(); // config gyroscope full scale range to 2000dps
    i2c.write(ICM20948_ADDR, &[ICM20948_GYRO_CONFIG_2, 0x00])
        .unwrap(); // disable gyroscope low pass filter

    // config magnetometer
    // todo: add magnetometer config
    //
    // todo: configure samle rate (all sensors at 100Hz)
    //

    // Enable sensor
    i2c.write(ICM20948_ADDR, &[ICM20948_PWR_MGMT_2, 0x00])
        .unwrap(); // enable the device
    i2c.write(ICM20948_ADDR, &[ICM20948_PWR_MGMT_1, 0x09])
        .unwrap(); // enable the i2c master
}
