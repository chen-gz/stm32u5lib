use core::{
    assert,
    result::Result::{Err, Ok},
};
use defmt;
use u5_lib::{clock, dcmi, dma, gi2c, gpio, ov5640_reg, ov5640_reg::*, rtc, sdmmc::SdInstance};

use defmt_rtt as _;

pub(crate) fn setup_camera(i2c: gi2c::I2cPort) {
    let mut read_val: [u8; 2] = [0u8; 2];
    i2c.write_read(
        OV5640_I2C_ADDR,
        &[
            (OV5640_CHIP_ID_HIGH_BYTE >> 8) as u8,
            OV5640_CHIP_ID_HIGH_BYTE as u8,
        ],
        &mut read_val[0..1],
    )
    .unwrap();
    i2c.write_read(
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
    i2c.write_read(ov5640_reg::OV5640_I2C_ADDR, &reg_addr, &mut read_val)
        .unwrap();
    let mut write_val = [0u8; 3];
    write_val[0] = reg_addr[0];
    write_val[1] = reg_addr[1];
    write_val[2] = read_val[0] | (1 << 5);
    i2c.write(ov5640_reg::OV5640_I2C_ADDR, &write_val).unwrap();

    // SYSREM_RESET02
    reg_addr[0] = (ov5640_reg::OV5640_SYSREM_RESET02 >> 8) as u8;
    reg_addr[1] = ov5640_reg::OV5640_SYSREM_RESET02 as u8;
    i2c.write_read(ov5640_reg::OV5640_I2C_ADDR, &reg_addr, &mut read_val)
        .unwrap();
    let mut write_val = [0u8; 3];
    write_val[0] = reg_addr[0];
    write_val[1] = reg_addr[1];
    write_val[2] = read_val[0] & !(1 << 2 | 1 << 3 | 1 << 4);
    i2c.write(ov5640_reg::OV5640_I2C_ADDR, &write_val).unwrap();

    // OV5640_CLOCK_ENABLE02
    reg_addr[0] = (ov5640_reg::OV5640_CLOCK_ENABLE02 >> 8) as u8;
    reg_addr[1] = ov5640_reg::OV5640_CLOCK_ENABLE02 as u8;
    i2c.write_read(ov5640_reg::OV5640_I2C_ADDR, &reg_addr, &mut read_val)
        .unwrap();
    let mut write_val = [0u8; 3];
    write_val[0] = reg_addr[0];
    write_val[1] = reg_addr[1];
    write_val[2] = read_val[0] | (1 << 3 | 1 << 5);
    i2c.write(ov5640_reg::OV5640_I2C_ADDR, &write_val).unwrap();
    defmt::info!("setup camera registers finished");
}

// use 4 byte in first block to store the number of image files
const IMG_START_BLOCK: u32 = 10;
const IMG_SIZE: u32 = 2000; // 2000 block = 2000 * 512 = 1M
const SIZE_BLOCK: u32 = 1; // first block store the number of image files

pub async fn save_picture(pic_buf: &mut [u8], sd: &SdInstance) {
    let mut found = false;
    let mut pic_end = 0;
    let len = pic_buf.len();
    for i in 0..len - 1 {
        if pic_buf[i] == 0xff && pic_buf[i + 1] == 0xd9 {
            found = true;
            pic_end = i;
            break;
        }
    }
    if !found {
        // TODO: return error code
        defmt::error!("not find jpeg end");
    }
    let date = rtc::get_date();
    let time = rtc::get_time();
    pic_buf[0] = (pic_end >> 24) as u8;
    pic_buf[1] = ((pic_end >> 16) & 0xff) as u8;
    pic_buf[2] = ((pic_end >> 8) & 0xff) as u8;
    pic_buf[3] = (pic_end & 0xff) as u8;

    pic_buf[4] = date.0;
    pic_buf[5] = date.1;
    pic_buf[6] = date.2;
    pic_buf[7] = time.0;
    pic_buf[8] = time.1;
    pic_buf[9] = time.2;
    // clock::set_clock_to_hsi(); // slow clock for sd card
    let block_count: u32 = ((pic_end + 512 - 1) / 512) as u32;
    let end: usize = block_count as usize * 512;
    defmt::info!(
        "start write picture to sd card, block_count: {}",
        block_count
    );
    let mut buf = [0u8; 512];
    match sd.read_single_block_async(&mut buf, SIZE_BLOCK).await {
        Ok(_) => {
            // defmt::info!("read picture number from sd card success");
        }
        Err(err) => {
            defmt::panic!("read picture number from sd card fail: {:?}", err);
        }
    }
    // .unwrap();
    let mut num = ((buf[0] as u32) << 24)
        | ((buf[1] as u32) << 16)
        | ((buf[2] as u32) << 8)
        | (buf[3] as u32);
    num += 1;
    buf[0] = (num >> 24) as u8;
    buf[1] = ((num >> 16) & 0xff) as u8;
    buf[2] = ((num >> 8) & 0xff) as u8;
    buf[3] = (num & 0xff) as u8;
    match sd.write_single_block_async(&buf, SIZE_BLOCK).await {
        Ok(_) => {
            defmt::info!("write picture number to sd card success");
        }
        Err(err) => {
            defmt::panic!("write picture number to sd card fail: {:?}", err);
        }
    }

    match sd
        .write_multiple_blocks_async(
            &pic_buf[0..end],
            (num + IMG_START_BLOCK) * IMG_SIZE,
            block_count,
        )
        .await
    {
        Ok(_) => {
            defmt::info!("write picture to sd card success");
        }
        Err(err) => {
            defmt::panic!("write picture to sd card fail: {:?}", err);
        }
    }
    defmt::info!("finish write picture to sd card");
}
pub async fn capture(
    _pdwn: &gpio::GpioPort,
    dcmi: &dcmi::DcmiPort,
    pic_buf: &mut [u8],
    sd: &SdInstance,
) {
    let cam_i2c: gi2c::I2cPort = gi2c::I2C3;
    // pdwn.set_low(); // set power down to low. Enable camera
    clock::delay_ms(1);
    let mut reg_val = [0u8; 3];
    reg_val[0] = (OV5640_SYSTEM_CTROL0 >> 8) as u8;
    reg_val[1] = OV5640_SYSTEM_CTROL0 as u8;
    reg_val[2] = 0x02;
    cam_i2c.write(OV5640_I2C_ADDR, &reg_val).unwrap();
    clock::delay_ms(200);
    dcmi.capture(dma::DCMI_DMA, &pic_buf[16..]).await;
    defmt::info!("finish take picture");
    // pdwn.set_high();
    // 0x3008

    let mut reg_val = [0u8; 3];
    reg_val[0] = (OV5640_SYSTEM_CTROL0 >> 8) as u8;
    reg_val[1] = OV5640_SYSTEM_CTROL0 as u8;
    reg_val[2] = (1 << 6) | 0x02;

    cam_i2c.write(OV5640_I2C_ADDR, &reg_val).unwrap();

    u5_lib::low_power::run_no_deep_sleep_async(|| async {
        save_picture(pic_buf, &sd).await;
    })
    .await;
}
