use crate::gpio::GpioPort;
use crate::i2c::{I2c, I2cMessage};
pub mod ov5640_reg;
use ov5640_reg::*;
use crate::com_interface::ComInterface;

pub fn setup_camera(i2c:&mut I2c, power_down: &GpioPort, reset_b: &GpioPort) {
    //todo: following the ov5640 datasheet to power up the camera
    power_down.set_low();  // enable camera
    reset_b.set_high();
    let mut read_val: [u8; 2] = [0u8; 2];
    i2c.write_read(
        OV5640_I2C_ADDR,
        &mut [
            (OV5640_CHIP_ID_HIGH_BYTE >> 8) as u8,
            OV5640_CHIP_ID_HIGH_BYTE as u8,
        ],
        &mut read_val[0..1],
    )
        .unwrap();
    i2c.write_read(
        OV5640_I2C_ADDR,
        &mut [
            (OV5640_CHIP_ID_LOW_BYTE >> 8) as u8,
            OV5640_CHIP_ID_LOW_BYTE as u8,
        ],
        &mut read_val[1..2],
    )
        .unwrap();
    assert!(read_val[0] == 0x56 && read_val[1] == 0x40);

    defmt::info!("writing ov5640 common regs");
    for &(reg, val) in ov5640_reg::OV5640_COMMON.iter() {
        let mut reg_val = [(reg >> 8) as u8, reg as u8, val as u8];
        i2c.send_retry(I2cMessage { addr: ov5640_reg::OV5640_I2C_ADDR, data: &mut reg_val }, 5).unwrap();
    }

    for &(reg, val) in OV5640_PF_JPEG.iter() {
        let mut reg_val = [(reg >> 8) as u8, reg as u8, val as u8];
        i2c.send_retry(I2cMessage { addr: ov5640_reg::OV5640_I2C_ADDR, data: &mut reg_val }, 5).unwrap();
    }

    for &(reg, val) in OV5640_JPEG_MODE.iter() {
        let mut reg_val = [(reg >> 8) as u8, reg as u8, val as u8];
        i2c.send_retry(I2cMessage { addr: ov5640_reg::OV5640_I2C_ADDR, data: &mut reg_val }, 5).unwrap();
    }
    defmt::info!("writing ov5640 jpeg regs finished");

    let mut read_val = [0u8; 1];
    let mut reg_addr = [(ov5640_reg::OV5640_TIMING_TC_REG21 >> 8) as u8, ov5640_reg::OV5640_TIMING_TC_REG21 as u8];
    i2c.write_read(ov5640_reg::OV5640_I2C_ADDR, &mut reg_addr, &mut read_val).unwrap();

    let mut write_val = [(ov5640_reg::OV5640_TIMING_TC_REG21 >> 8) as u8, ov5640_reg::OV5640_TIMING_TC_REG21 as u8, read_val[0] | (1 << 5)];
    let write_val = I2cMessage { addr: ov5640_reg::OV5640_I2C_ADDR, data: &mut write_val };
    i2c.send_retry(write_val, 5).unwrap();

    // SYSREM_RESET02
    let mut reg_addr = [(ov5640_reg::OV5640_SYSREM_RESET02 >> 8) as u8, ov5640_reg::OV5640_SYSREM_RESET02 as u8];
    // let reg_addr = I2cMessage { addr: ov5640_reg::OV5640_I2C_ADDR, data: &mut reg_addr };
    i2c.write_read(ov5640_reg::OV5640_I2C_ADDR, &mut reg_addr, &mut read_val)
        .unwrap();
    let mut write_val = [(ov5640_reg::OV5640_SYSREM_RESET02 >> 8) as u8, ov5640_reg::OV5640_SYSREM_RESET02 as u8, read_val[0] & !(1 << 2 | 1 << 3 | 1 << 4)];
    let write_val = I2cMessage { addr: ov5640_reg::OV5640_I2C_ADDR, data: &mut write_val };
    i2c.send_retry(write_val, 5).unwrap();
    // i2c.write(ov5640_reg::OV5640_I2C_ADDR, &write_val).unwrap();

    // OV5640_CLOCK_ENABLE02
    let mut reg_addr = [(ov5640_reg::OV5640_CLOCK_ENABLE02 >> 8) as u8, ov5640_reg::OV5640_CLOCK_ENABLE02 as u8];

    i2c.write_read(ov5640_reg::OV5640_I2C_ADDR, &mut reg_addr, &mut read_val).unwrap();
    let mut write_val = [(ov5640_reg::OV5640_CLOCK_ENABLE02 >> 8) as u8, ov5640_reg::OV5640_CLOCK_ENABLE02 as u8, read_val[0] | (1 << 3 | 1 << 5)];
    let write_val = I2cMessage { addr: ov5640_reg::OV5640_I2C_ADDR, data: &mut write_val };
    // i2c.write(ov5640_reg::OV5640_I2C_ADDR, &write_val).unwrap();
    i2c.send_retry(write_val, 5).unwrap();
    defmt::info!("setup camera registers finished");

    // soft sleep
    // let mut reg_val = [(OV5640_SYSTEM_CTROL0 >> 8) as u8, OV5640_SYSTEM_CTROL0 as u8, (1 << 6) | 0x02];
    // let reg_val = I2cMessage { addr: OV5640_I2C_ADDR, data: &mut reg_val };
    // i2c.send_retry(reg_val, 5).unwrap();
}

