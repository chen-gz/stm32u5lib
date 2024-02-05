use ebcmd::Response;
use embassy_usb::{
    class::cdc_acm::{CdcAcmClass, State},
    driver::EndpointError,
    Builder,
};
use u5_lib::{clock, gpio, rtc, sdmmc::SdInstance, *};

const SIZE_BLOCK: u32 = 1; // first block store the number of image files
const IMG_START_BLOCK: u32 = 10;
const IMG_SIZE: u32 = 2000; // 2000 block = 2000 * 512 = 1M

use embassy_stm32::{
    bind_interrupts,
    peripherals::{self},
    usb_otg::{self, Driver, Instance},
};

bind_interrupts!(struct Irqs {
    OTG_FS => usb_otg::InterruptHandler<peripherals::USB_OTG_FS>;
});
use futures::future::join;
#[embassy_executor::task]
pub async fn usb_task(sd_instace: SdInstance) {
    let p = unsafe { embassy_stm32::Peripherals::steal() };
    // init USB CDC
    let mut ep_out_buffer = [0u8; 256];
    let mut config = embassy_stm32::usb_otg::Config::default();
    // config.vbus_detection = true;
    config.vbus_detection = false;
    let driver = Driver::new_fs(
        p.USB_OTG_FS,
        Irqs,
        p.PA12,
        p.PA11,
        &mut ep_out_buffer,
        config,
    );
    //
    // // Create embassy-usb Config
    let mut config = embassy_usb::Config::new(0xc0de, 0xcafe);
    config.manufacturer = Some("Embassy");
    config.product = Some("USB-serial example");
    config.serial_number = Some("12345678");

    config.device_class = 0xEF;
    config.device_sub_class = 0x02;
    config.device_protocol = 0x01;
    config.composite_with_iads = true;

    let mut device_descriptor = [0; 256];
    let mut config_descriptor = [0; 256];
    let mut bos_descriptor = [0; 256];
    let mut control_buf = [0; 64];
    let mut msos_descriptor = [0; 256];

    let mut state = State::new();

    let mut builder = Builder::new(
        driver,
        config,
        &mut device_descriptor,
        &mut config_descriptor,
        &mut bos_descriptor,
        &mut msos_descriptor,
        &mut control_buf,
    );

    let mut class = CdcAcmClass::new(&mut builder, &mut state, 64);
    // Build the builder.
    let mut usb = builder.build();
    let usb_fut = usb.run(); // Run the USB device.
    let handler_fut = async {
        loop {
            class.wait_connection().await;

            u5_lib::clock::run_with_160mhz_async(|| async {
                u5_lib::low_power::run_no_deep_sleep_async(|| async {
                    // clock::set_clock_to_pll(); // fast clock for camera
                    // SIGNAL.signal(1);
                    // clock::set_clock_to_pll();
                    // clock::set_cpu_freq(160_000_000);
                    defmt::info!("connected");
                    let _ = usb_handler(&mut class, &sd_instace).await;
                    defmt::info!("disconnected");
                    // clock::set_clock_to_hsi();
                    // SIGNAL.signal(0);
                })
                .await;
            })
            .await;
            // LED_BLUE.set_high();
        }
    };
    join(usb_fut, handler_fut).await; // Run everything concurrently.
}
struct Disconnected {}

impl From<EndpointError> for Disconnected {
    fn from(val: EndpointError) -> Self {
        match val {
            EndpointError::BufferOverflow => panic!("Buffer overflow"),
            EndpointError::Disabled => Disconnected {},
        }
    }
}
// #[path = "../ebcmd.rs"]
// mod ebcmd;
use eb_cmds as ebcmd;

use crate::{LED_BLUE, LED_GREEN};

async fn usb_handler<'d, T: Instance + 'd>(
    class: &mut CdcAcmClass<'d, Driver<'d, T>>,
    sd_instace: &SdInstance,
) -> Result<(), Disconnected> {
    let mut buf: [u8; 128] = [0; 128]; // the maximum size of the command is 64 bytes
                                       // let sd = SdInstance::new(stm32_metapac::SDMMC2);
                                       // let sd = init_sd();
                                       // get sd instance from main task
                                       // let sd = SIGNAL_SD_INST.wait().await;

    // loop {
    //     let n = class.read_packet(&mut buf).await.unwrap();
    // }
    defmt::info!("start usb handler");
    loop {
        let n = class.read_packet(&mut buf).await.unwrap();
        let command = ebcmd::Command::from_array(&buf[..n]);
        match command {
            ebcmd::Command::SetTim(year, month, day, hour, minute, second, period) => {
            //     rtc::setup(
            //         year,
            //         month,
            //         day,
            //         hour,
            //         minute,
            //         second,
            //         period,
            //         stm32_metapac::rcc::vals::Rtcsel::LSI,
            //     );
                let res = ebcmd::Response::SetTim(0);
                let (buf, len) = res.to_array();
                class.write_packet(&buf[0..len]).await.unwrap();
            }
            ebcmd::Command::GetTim => {
                let date = rtc::get_date();
                let time = rtc::get_time();
                let res = ebcmd::Response::GetTim(date.0, date.1, date.2, time.0, time.1, time.2);
                let (buf, len) = res.to_array();
                // let len = 48;
                // class.write_packet(&buf[0..len]).await.unwrap();
                // class.write_packet(&buf[0..len]).await.unwrap();
                // class.write_packet(&buf[0..len]).await.unwrap();
                // class.write_packet(&buf[0..len]).await.unwrap();
                // class.write_packet(&buf[0..len]).await.unwrap();
                class.write_packet(&buf[0..len]).await; // .unwrap();
            }
            ebcmd::Command::GetPic(id) => {
                let mut buf = [0; 64];
                buf[0] = 0x02;
                let mut pic_buf = [0; 512];
                let start_block = (id + IMG_START_BLOCK) * IMG_SIZE;
                sd_instace
                    .read_single_block_async(&mut pic_buf, start_block)
                    .await
                    .unwrap();
                // get the end of picture
                let pic_end = ((pic_buf[0] as u32) << 24)
                    | ((pic_buf[1] as u32) << 16)
                    | ((pic_buf[2] as u32) << 8)
                    | (pic_buf[3] as u32);
                let block_count: u32 = ((pic_end + 512 - 1) / 512) as u32;
                let mut ordinal = 0;
                let mut send_len: usize;
                let mut res: Response;
                let mut start = 16;
                loop {
                    if start >= pic_buf.len() {
                        break;
                    }
                    (ordinal, send_len, res) =
                        ebcmd::Response::pic_res_from_data(id, ordinal, &pic_buf[start..]);
                    if send_len == 0 {
                        break;
                    }
                    start += send_len;

                    let (buf_tmp, len) = res.to_array();
                    class.write_packet(&buf_tmp[0..len]).await.unwrap();
                    // Timer::after(Duration::from_millis(100)).await;
                    // LED_BLUE.toggle();
                }
                // LED_GREEN.toggle();
                for block in 1..block_count {
                    // sd.read_single_block(&mut buf, start_block + block).unwrap();
                    // let mut pic_buf = [0; 512]; // why without this line, the program not work?
                    sd_instace
                        .read_single_block_async(&mut pic_buf, start_block + block)
                        .await
                        .unwrap();
                    start = 0;
                    loop {
                        if start >= pic_buf.len() {
                            break;
                        }
                        (ordinal, send_len, res) =
                            ebcmd::Response::pic_res_from_data(id, ordinal, &pic_buf[start..]);
                        if send_len == 0 {
                            break;
                        }
                        start += send_len;
                        let (buf_tmp, len) = res.to_array();
                        class.write_packet(&buf_tmp[0..len]).await.unwrap();
                        // Timer::after(Duration::from_millis(100)).await;
                    }
                    // LED_GREEN.toggle();
                }

                // let date = rtc::get_date();
                // let time = rtc::get_time();
                // let res = ebcmd::Response::GetTim(date.0, date.1, date.2, time.0, time.1, time.2);
                // let (buf, len) = res.to_array();
                // class.write_packet(&buf[0..len]).await.unwrap();
            }
            ebcmd::Command::GetPicNum => {
                let mut buf = [0u8; 512];
                sd_instace
                    .read_single_block_async(&mut buf, SIZE_BLOCK)
                    .await
                    .unwrap();
                let num = ((buf[0] as u32) << 24)
                    | ((buf[1] as u32) << 16)
                    | ((buf[2] as u32) << 8)
                    | (buf[3] as u32);
                // ebcmd::Response::GetPicNum(num)
                let res = ebcmd::Response::GetPicNum(num);
                let (buf, len) = res.to_array();
                class.write_packet(&buf[0..len]).await.unwrap();
            }
            ebcmd::Command::ClearPic => {
                let buf = [0u8; 512];
                sd_instace
                    .write_single_block_async(&buf, SIZE_BLOCK)
                    .await
                    .unwrap();
                let res = ebcmd::Response::ClearPic(0);
                let (buf, len) = res.to_array();
                class.write_packet(&buf[0..len]).await.unwrap();
            }
        };
    }
}
