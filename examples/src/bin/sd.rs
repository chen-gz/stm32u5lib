#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]
#![allow(dead_code)]
#![allow(unused_imports)]

use core::fmt::{write, Write};
use defmt_rtt as _;
use embassy_executor::Spawner;
use heapless::String;

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    defmt::info!("panic");
    defmt::error!(
        "Location file name: {:?}, line: {:?}, col: {:?}",
        _info.location().unwrap().file(),
        _info.location().unwrap().line(),
        _info.location().unwrap().column()
    );
    loop {}
}

use u5_lib::{
    gpio::{SDMMC2_CK_PC1, SDMMC2_CMD_PA0, SDMMC2_D0_PB14},
    *,
};

use embassy_time::{Duration, Timer};

const PIC_BUF_SIZE: usize = 600_000;
static mut PIC_BUF: [u8; PIC_BUF_SIZE] = [0; PIC_BUF_SIZE];

const LED_GREEN: gpio::GpioPort = gpio::PC3;
const LED_ORANGE: gpio::GpioPort = gpio::PC4;
const LED_BLUE: gpio::GpioPort = gpio::PC5;
const CAM_I2C: gi2c::I2cPort = gi2c::I2C3;
const CAM_PDWN: gpio::GpioPort = gpio::PB0;

fn setup() {
    // this function setup for peripheral
    clock::init_clock();
    // CAM_PDWN.setup();
    // LED_ORANGE.setup();
    // LED_ORANGE.set_high();
    LED_GREEN.setup();
    LED_GREEN.set_high();
    // LED_BLUE.setup();
    // LED_BLUE.set_high();
    // CAM_I2C.init(100_000, gpio::I2C3_SCL_PC0, gpio::I2C3_SDA_PB4);
    // CAM_PDWN.set_high();
}
use u5_lib::sdmmc::*;

struct Ftsource {}
use embedded_sdmmc::TimeSource;
use embedded_sdmmc::Timestamp;
impl TimeSource for Ftsource {
    fn get_timestamp(&self) -> Timestamp {
        Timestamp {
            year_since_1970: 0,
            zero_indexed_month: 0,
            zero_indexed_day: 0,
            hours: 0,
            minutes: 0,
            seconds: 0,
        }
    }
}

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    setup();
    let mut sd = SdInstance::new(stm32_metapac::SDMMC2);
    let ts = Ftsource {};
    sd.init(SDMMC2_CK_PC1, SDMMC2_D0_PB14, SDMMC2_CMD_PA0);
    defmt::info!("init sd card");
    let mut volume_mgr = embedded_sdmmc::VolumeManager::new(sd, ts);
    let volume0 = volume_mgr
        .open_volume(embedded_sdmmc::VolumeIdx(0))
        .unwrap();
    defmt::info!("Volume 0: {:?}", volume0);
    let root_dir = volume_mgr.open_root_dir(volume0).unwrap();

    let mut file_num = 0;
    loop {
        file_num += 1;
        let mut file_name = String::<32>::new();
        file_name
            .write_fmt(format_args!("{}.da", file_num))
            .unwrap(); 
        // if put .data as end. I got nametooLong error

        let file = match volume_mgr.open_file_in_dir(
            root_dir,
            file_name.as_str(),
            embedded_sdmmc::Mode::ReadWriteCreateOrTruncate,
        ) {
            Ok(f) => f,
            Err(err) => {
                defmt::error!("open file failed {:?}", err);
                break;
            }
        };
        defmt::info!("open file");
        let buf = b"hello world, from rust";
        let _ = volume_mgr.write(file, &buf[..]).unwrap();

        defmt::info!("write file");
        let _ = volume_mgr.close_file(file).unwrap();
        defmt::info!("close file");
        LED_BLUE.toggle();
    }

    // GPDMA1.ch(0).tr1().modify(|w| w.set_dap(ChTr1Ap::PORT1));
    loop {
        Timer::after(Duration::from_secs(1)).await;
        LED_GREEN.toggle();
    }
}
