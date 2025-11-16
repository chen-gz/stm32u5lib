use crate::{clock, dcmi, dma, gpio, i2c, rtc, sdmmc::SdInstance};
use crate::{error, info};
use core::result::Result::{Err, Ok};

// use 4 byte in first block to store the number of image files
const IMG_START_BLOCK: u32 = 10;
const IMG_SIZE: u32 = 2000;
// 2000 block = 2000 * 512 = 1M
const SIZE_BLOCK: u32 = 1; // first block store the number of image files

pub async fn save_picture(pic_buf: &mut [u8], sd: &SdInstance) {
    let mut found = false;
    let mut pic_end = 0;
    let len = pic_buf.len();
    for i in 0..len - 1 {
        if pic_buf[i] == 0xff && pic_buf[i + 1] == 0xd9 {
            found = true;
            pic_end = i + 2;
            break;
        }
    }
    info!("first 100 bytes: {:x}", &pic_buf[0..100]);
    if !found {
        // TODO: return error code
        panic!("not find jpeg end");
    }
    info!("find jpeg end at {}, = {}kb", pic_end, pic_end / 1024);
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
    info!("start write picture to sd card, block_count: {}", block_count);
    let mut buf = [0u8; 512];
    match sd.read_single_block_async(&mut buf, SIZE_BLOCK).await {
        Ok(_) => {
            // info!("read picture number from sd card success");
        }
        Err(err) => {
            panic!("read picture number from sd card fail: {:?}", err);
        }
    }
    let mut num = ((buf[0] as u32) << 24) | ((buf[1] as u32) << 16) | ((buf[2] as u32) << 8) | (buf[3] as u32);
    num += 1;
    buf[0] = (num >> 24) as u8;
    buf[1] = ((num >> 16) & 0xff) as u8;
    buf[2] = ((num >> 8) & 0xff) as u8;
    buf[3] = (num & 0xff) as u8;
    // current picture number
    info!("current picture number: {}", num);
    match sd.write_single_block_async(&buf, SIZE_BLOCK).await {
        Ok(_) => {
            info!("write picture number to sd card success");
        }
        Err(err) => {
            panic!("write picture number to sd card fail: {:?}", err);
        }
    }

    match sd
        .write_multiple_blocks_async(&pic_buf[0..end], (num + IMG_START_BLOCK) * IMG_SIZE, block_count)
        .await
    {
        Ok(_) => {
            info!("write picture to sd card success");
        }
        Err(err) => {
            panic!("write picture to sd card fail: {:?}", err);
        }
    }
    info!("finish write picture to sd card");
}

pub async fn capture(
    _pdwn: &gpio::GpioPort,
    _cam_i2c: &mut i2c::I2c,
    dcmi: &dcmi::DcmiPort,
    pic_buf: &mut [u8],
    // sd: &SdInstance,
) {
    // let cam_i2c: gi2c::I2cPort = gi2c::I2C3;
    _pdwn.set_low(); // set power down to low. Enable camera
    clock::delay_ms(2);
    // let mut reg_val = [0u8; 3];
    // let mut reg_val = [(OV5640_SYSTEM_CTROL0 >> 8) as u8, OV5640_SYSTEM_CTROL0 as u8, 0x02];
    // let reg_val = I2cMessage { addr: OV5640_I2C_ADDR, data: &mut reg_val };
    // cam_i2c.send_retry(reg_val, 5).unwrap();
    // cam_i2c.write(OV5640_I2C_ADDR, &reg_val).unwrap();
    clock::delay_ms(200);
    dcmi.capture(dma::DMA_DCMI, &pic_buf[16..]).await;
    info!("finish take picture");
    _pdwn.set_high();
    // 0x3008

    // let mut reg_val = [(OV5640_SYSTEM_CTROL0 >> 8) as u8, OV5640_SYSTEM_CTROL0 as u8, (1 << 6) | 0x02];
    // let reg_val = I2cMessage { addr: OV5640_I2C_ADDR, data: &mut reg_val };
    // cam_i2c.send_retry(reg_val, 5).unwrap();
}
