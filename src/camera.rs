use crate::hal::{Delay, Pin, Rtc};

// use 4 byte in first block to store the number of image files
const IMG_START_BLOCK: u32 = 10;
const IMG_SIZE: u32 = 2000;
// 2000 block = 2000 * 512 = 1M
const SIZE_BLOCK: u32 = 1; // first block store the number of image files

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

/// Abstraction over digital camera interface (DCMI).
pub trait CameraDcmi {
    fn capture(&self, pic_buf: &mut [u8]) -> impl core::future::Future<Output = ()> + Send;
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
    info!("first 100 bytes: {:x}", crate::fmt::Bytes(&pic_buf[0..100]));
    if !found {
        panic!("not find jpeg end");
    }
    info!("find jpeg end at {}, = {}kb", pic_end, pic_end / 1024);
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
    info!(
        "start write picture to sd card, block_count: {}",
        block_count
    );
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
        .write_multiple_blocks_async(
            &pic_buf[0..end],
            (num + IMG_START_BLOCK) * IMG_SIZE,
            block_count,
        )
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

/// Capture a single frame from DCMI.
pub async fn capture<P: Pin, D: Delay, DCMI: CameraDcmi>(
    pdwn: &P,
    delay: &D,
    dcmi: &DCMI,
    pic_buf: &mut [u8],
) {
    pdwn.set_low(); // set power down to low. Enable camera
    delay.delay_ms(2);
    delay.delay_ms(200);
    dcmi.capture(&mut pic_buf[16..]).await;
    info!("finish take picture");
    pdwn.set_high();
}

#[cfg(all(target_arch = "arm", target_os = "none", sdmmc, dcmi))]
impl CameraSdCard for crate::sdmmc::SdInstance {
    type Error = crate::sdmmc::SdError;

    async fn read_single_block_async(
        &self,
        buf: &mut [u8],
        block_idx: u32,
    ) -> Result<(), Self::Error> {
        self.read_single_block_async(buf, block_idx).await
    }

    async fn write_single_block_async(
        &self,
        buf: &[u8],
        block_idx: u32,
    ) -> Result<(), Self::Error> {
        self.write_single_block_async(buf, block_idx).await
    }

    async fn write_multiple_blocks_async(
        &self,
        buf: &[u8],
        block_idx: u32,
        block_count: u32,
    ) -> Result<(), Self::Error> {
        self.write_multiple_blocks_async(buf, block_idx, block_count)
            .await
    }
}

#[cfg(all(target_arch = "arm", target_os = "none", sdmmc, dcmi))]
impl CameraDcmi for crate::dcmi::DcmiPort {
    async fn capture(&self, pic_buf: &mut [u8]) {
        self.capture(crate::dma::DMA_DCMI, pic_buf).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::executor::block_on;
    use std::collections::HashMap;
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::Mutex as StdMutex;

    struct MockPin {
        low_called: AtomicBool,
        high_called: AtomicBool,
    }
    impl Pin for MockPin {
        fn setup(&self) {}
        fn set_high(&self) {
            self.high_called.store(true, Ordering::SeqCst);
        }
        fn set_low(&self) {
            self.low_called.store(true, Ordering::SeqCst);
        }
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
        fail_read: AtomicBool,
        fail_write_single: AtomicBool,
        fail_write_multiple: AtomicBool,
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
            if self.fail_read.load(Ordering::SeqCst) {
                return Err(MockSdError);
            }
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
            if self.fail_write_single.load(Ordering::SeqCst) {
                return Err(MockSdError);
            }
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
            if self.fail_write_multiple.load(Ordering::SeqCst) {
                return Err(MockSdError);
            }
            let mut blocks = self.blocks.lock().unwrap();
            for i in 0..block_count {
                let start = (i * 512) as usize;
                let end = ((i + 1) * 512) as usize;
                blocks.insert(block_idx + i, buf[start..end].to_vec());
            }
            Ok(())
        }
    }

    struct MockDcmi {
        captured: AtomicBool,
    }
    impl CameraDcmi for MockDcmi {
        async fn capture(&self, pic_buf: &mut [u8]) {
            self.captured.store(true, Ordering::SeqCst);
            // Write some fake JPEG data (with FFD9 end marker)
            pic_buf[0] = 0xFF;
            pic_buf[1] = 0xD8;
            pic_buf[2] = 0x00;
            pic_buf[100] = 0xFF;
            pic_buf[101] = 0xD9; // FFD9
        }
    }

    #[test]
    fn test_camera_capture() {
        let pin = MockPin {
            low_called: AtomicBool::new(false),
            high_called: AtomicBool::new(false),
        };
        pin.setup();
        pin.toggle();
        let delay = MockDelay;
        let dcmi = MockDcmi {
            captured: AtomicBool::new(false),
        };
        let mut buf = [0u8; 200];

        block_on(capture(&pin, &delay, &dcmi, &mut buf));

        assert!(pin.low_called.load(Ordering::SeqCst));
        assert!(pin.high_called.load(Ordering::SeqCst));
        assert!(dcmi.captured.load(Ordering::SeqCst));
    }

    #[test]
    fn test_camera_save_picture() {
        let sd = MockSdCard::default();
        let rtc = MockRtc;

        // Set initial picture count to 5
        let mut first_block = [0u8; 512];
        first_block[3] = 5;
        sd.blocks
            .lock()
            .unwrap()
            .insert(SIZE_BLOCK, first_block.to_vec());

        // Create a fake JPEG buffer
        let mut pic_buf = vec![0u8; 2000];
        pic_buf[16] = 0xFF;
        pic_buf[17] = 0xD8;
        pic_buf[18] = 0xAA;
        pic_buf[100] = 0xFF;
        pic_buf[101] = 0xD9; // End of image (marker)

        // The JPEG data starts at index 16 (DCMI writes offset 16)
        block_on(save_picture(&mut pic_buf[16..], &sd, &rtc));

        // Read first block back and verify the picture count is incremented to 6
        let mut check_buf = [0u8; 512];
        block_on(sd.read_single_block_async(&mut check_buf, SIZE_BLOCK)).unwrap();
        let num = ((check_buf[0] as u32) << 24)
            | ((check_buf[1] as u32) << 16)
            | ((check_buf[2] as u32) << 8)
            | (check_buf[3] as u32);
        assert_eq!(num, 6);

        // Verify that the date/time is written into the image buffer offset
        assert_eq!(pic_buf[16 + 4], 26); // year
        assert_eq!(pic_buf[16 + 5], 6); // month
        assert_eq!(pic_buf[16 + 6], 21); // day
    }

    #[test]
    #[should_panic(expected = "not find jpeg end")]
    fn test_camera_save_picture_no_jpeg_end() {
        let sd = MockSdCard::default();
        let rtc = MockRtc;
        let mut pic_buf = vec![0u8; 100];
        block_on(save_picture(&mut pic_buf, &sd, &rtc));
    }

    #[test]
    #[should_panic(expected = "read picture number from sd card fail")]
    fn test_camera_save_picture_read_fail() {
        let sd = MockSdCard::default();
        sd.fail_read.store(true, Ordering::SeqCst);
        let rtc = MockRtc;
        let mut pic_buf = vec![0u8; 2000];
        pic_buf[20] = 0xFF;
        pic_buf[21] = 0xD9;
        block_on(save_picture(&mut pic_buf, &sd, &rtc));
    }

    #[test]
    #[should_panic(expected = "write picture number to sd card fail")]
    fn test_camera_save_picture_write_single_fail() {
        let sd = MockSdCard::default();
        sd.fail_write_single.store(true, Ordering::SeqCst);
        let rtc = MockRtc;
        let mut pic_buf = vec![0u8; 2000];
        pic_buf[20] = 0xFF;
        pic_buf[21] = 0xD9;
        block_on(save_picture(&mut pic_buf, &sd, &rtc));
    }

    #[test]
    #[should_panic(expected = "write picture to sd card fail")]
    fn test_camera_save_picture_write_multiple_fail() {
        let sd = MockSdCard::default();
        sd.fail_write_multiple.store(true, Ordering::SeqCst);
        let rtc = MockRtc;
        let mut pic_buf = vec![0u8; 2000];
        pic_buf[20] = 0xFF;
        pic_buf[21] = 0xD9;
        block_on(save_picture(&mut pic_buf, &sd, &rtc));
    }
}
