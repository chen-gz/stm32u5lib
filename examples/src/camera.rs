
use u5_lib::ov5640_reg;
use u5_lib::ov5640_reg::*;
use u5_lib::gi2c;
use core::assert;
use defmt;
use core::result::Result::{Ok, Err};

use defmt_rtt as _;

pub(crate) fn setup_camera(i2c: gi2c::I2cPort) {
    let mut read_val: [u8; 2] = [0u8; 2];
    i2c
        .write_read(
            OV5640_I2C_ADDR,
            &[
                (OV5640_CHIP_ID_HIGH_BYTE >> 8) as u8,
                OV5640_CHIP_ID_HIGH_BYTE as u8,
            ],
            &mut read_val[0..1],
        )
        .unwrap();
    i2c
        .write_read(
            OV5640_I2C_ADDR,
            &[
                (OV5640_CHIP_ID_LOW_BYTE >> 8) as u8,
                OV5640_CHIP_ID_LOW_BYTE as u8,
            ],
            &mut read_val[1..2],
        )
        .unwrap();
    assert!(read_val[0] == 0x56 && read_val[1] == 0x40);


    defmt::info!("writing ov5640 common regs");
    for &(reg, val) in ov5640_reg::OV5640_COMMON.iter() {
        let mut reg_val = [0u8; 3];
        reg_val[0] = (reg >> 8) as u8;
        reg_val[1] = reg as u8;
        reg_val[2] = val as u8;
        match i2c.write(ov5640_reg::OV5640_I2C_ADDR, &reg_val) {
            Ok(_) => {}
            Err(_) => {
                defmt::panic!("failed when writing ov5640 common regs");
            }
        }
    }

    defmt::info!("writing ov5640 jpeg regs");
    for &(reg, val) in OV5640_PF_JPEG.iter() {
        let mut reg_val = [0u8; 3];
        reg_val[0] = (reg >> 8) as u8;
        reg_val[1] = reg as u8;
        reg_val[2] = val as u8;
        match i2c.write(ov5640_reg::OV5640_I2C_ADDR, &reg_val) {
            Ok(_) => {}
            Err(_) => {
                defmt::panic!("failed when writing ov5640 jpeg regs");
            }
        }
    }
    for &(reg, val) in OV5640_JPEG_MODE.iter() {
        let mut reg_val = [0u8; 3];
        reg_val[0] = (reg >> 8) as u8;
        reg_val[1] = reg as u8;
        reg_val[2] = val as u8;
        match i2c.write(ov5640_reg::OV5640_I2C_ADDR, &reg_val) {
            Ok(_) => {}
            Err(_) => {
                defmt::panic!("failed when writing ov5640 jpeg mode");
            }
        }
    }
    defmt::info!("writing ov5640 jpeg regs finished");

    let mut read_val = [0u8; 1];
    let mut reg_addr = [0u8; 2];
    // OV5640_TIMING_TC_REG21
    reg_addr[0] = (ov5640_reg::OV5640_TIMING_TC_REG21 >> 8) as u8;
    reg_addr[1] = ov5640_reg::OV5640_TIMING_TC_REG21 as u8;
    i2c
        .write_read(ov5640_reg::OV5640_I2C_ADDR, &reg_addr, &mut read_val)
        .unwrap();
    let mut write_val = [0u8; 3];
    write_val[0] = reg_addr[0];
    write_val[1] = reg_addr[1];
    write_val[2] = read_val[0] | (1 << 5);
    i2c
        .write(ov5640_reg::OV5640_I2C_ADDR, &write_val)
        .unwrap();

    // SYSREM_RESET02
    reg_addr[0] = (ov5640_reg::OV5640_SYSREM_RESET02 >> 8) as u8;
    reg_addr[1] = ov5640_reg::OV5640_SYSREM_RESET02 as u8;
    i2c
        .write_read(ov5640_reg::OV5640_I2C_ADDR, &reg_addr, &mut read_val)
        .unwrap();
    let mut write_val = [0u8; 3];
    write_val[0] = reg_addr[0];
    write_val[1] = reg_addr[1];
    write_val[2] = read_val[0] & !(1 << 2 | 1 << 3 | 1 << 4);
    i2c
        .write(ov5640_reg::OV5640_I2C_ADDR, &write_val)
        .unwrap();

    // OV5640_CLOCK_ENABLE02
    reg_addr[0] = (ov5640_reg::OV5640_CLOCK_ENABLE02 >> 8) as u8;
    reg_addr[1] = ov5640_reg::OV5640_CLOCK_ENABLE02 as u8;
    i2c
        .write_read(ov5640_reg::OV5640_I2C_ADDR, &reg_addr, &mut read_val)
        .unwrap();
    let mut write_val = [0u8; 3];
    write_val[0] = reg_addr[0];
    write_val[1] = reg_addr[1];
    write_val[2] = read_val[0] | (1 << 3 | 1 << 5);
    i2c
        .write(ov5640_reg::OV5640_I2C_ADDR, &write_val)
        .unwrap();
    defmt::info!("setup camera registers finished");
}