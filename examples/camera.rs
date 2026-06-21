use std::collections::HashMap;
use std::sync::Mutex as StdMutex;
use u5_lib::hal::{Dcmi, Delay, Pin, Rtc};

// Use 4 byte in first block to store the number of image files
const IMG_START_BLOCK: u32 = 10;
const IMG_SIZE: u32 = 2000;
const SIZE_BLOCK: u32 = 1; // First block stores the number of image files

/// Abstraction over SD card block device.
pub trait CameraSdCard {
    type Error: core::fmt::Debug;

    fn read_single_block_async(
        &self,
        buf: &mut [u8],
        block_idx: u32,
    ) -> impl core::future::Future<Output = Result<(), Self::Error>> + Send;

    fn write_single_block_async(
        &self,
        buf: &[u8],
        block_idx: u32,
    ) -> impl core::future::Future<Output = Result<(), Self::Error>> + Send;

    fn write_multiple_blocks_async(
        &self,
        buf: &[u8],
        block_idx: u32,
        block_count: u32,
    ) -> impl core::future::Future<Output = Result<(), Self::Error>> + Send;
}

/// Save a JPEG picture to the SD card.
pub async fn save_picture<SD: CameraSdCard, R: Rtc>(pic_buf: &mut [u8], sd: &SD, rtc: &R) {
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
    if !found {
        panic!("not find jpeg end");
    }
    let (date, time) = rtc.get_date_time();
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

    let block_count: u32 = (pic_end as u32).div_ceil(512);
    let end: usize = block_count as usize * 512;
    let mut buf = [0u8; 512];
    match sd.read_single_block_async(&mut buf, SIZE_BLOCK).await {
        Ok(_) => {}
        Err(err) => {
            panic!("read picture number from sd card fail: {:?}", err);
        }
    }
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
        Ok(_) => {}
        Err(err) => {
            panic!("write picture number to sd card fail: {:?}", err);
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
        Ok(_) => {}
        Err(err) => {
            panic!("write picture to sd card fail: {:?}", err);
        }
    }
}

// Simple implementations for a mock scenario
struct MockPin;
impl Pin for MockPin {
    fn setup(&self) {}
    fn set_high(&self) {}
    fn set_low(&self) {}
    fn toggle(&self) {}
}

struct MockDelay;
impl Delay for MockDelay {
    fn delay_ms(&self, _ms: u32) {}
}

struct MockRtc;
impl Rtc for MockRtc {
    fn get_date_time(&self) -> ((u8, u8, u8), (u8, u8, u8)) {
        ((26, 6, 21), (12, 0, 0))
    }
}

#[derive(Default)]
struct MockSdCard {
    blocks: StdMutex<HashMap<u32, Vec<u8>>>,
}

#[derive(Debug)]
struct MockSdError;

impl CameraSdCard for MockSdCard {
    type Error = MockSdError;

    async fn read_single_block_async(
        &self,
        buf: &mut [u8],
        block_idx: u32,
    ) -> Result<(), Self::Error> {
        let blocks = self.blocks.lock().unwrap();
        if let Some(data) = blocks.get(&block_idx) {
            buf.copy_from_slice(data);
        } else {
            buf.fill(0);
        }
        Ok(())
    }

    async fn write_single_block_async(
        &self,
        buf: &[u8],
        block_idx: u32,
    ) -> Result<(), Self::Error> {
        let mut blocks = self.blocks.lock().unwrap();
        blocks.insert(block_idx, buf.to_vec());
        Ok(())
    }

    async fn write_multiple_blocks_async(
        &self,
        buf: &[u8],
        block_idx: u32,
        block_count: u32,
    ) -> Result<(), Self::Error> {
        let mut blocks = self.blocks.lock().unwrap();
        for i in 0..block_count {
            let start = (i * 512) as usize;
            let end = ((i + 1) * 512) as usize;
            blocks.insert(block_idx + i, buf[start..end].to_vec());
        }
        Ok(())
    }
}

struct MockDcmi;
impl Dcmi for MockDcmi {
    async fn capture(&self, pic_buf: &mut [u8]) {
        // Write some fake JPEG data (with FFD9 end marker)
        pic_buf[0] = 0xFF;
        pic_buf[1] = 0xD8;
        pic_buf[2] = 0x00;
        pic_buf[100] = 0xFF;
        pic_buf[101] = 0xD9; // FFD9
    }
}

fn main() {
    println!("Running camera integration example...");

    let pin = MockPin;
    let delay = MockDelay;
    let dcmi = MockDcmi;
    let sd = MockSdCard::default();
    let rtc = MockRtc;

    let mut buf = [0u8; 2000];

    futures::executor::block_on(async {
        // 1. Capture a frame using the camera driver
        u5_lib::drivers::ov5640::capture_frame(&pin, &delay, &dcmi, &mut buf[16..]).await;

        // 2. Save the captured frame to SD card
        save_picture(&mut buf[16..], &sd, &rtc).await;
    });

    println!("Camera capture and save sequence completed successfully!");
}
