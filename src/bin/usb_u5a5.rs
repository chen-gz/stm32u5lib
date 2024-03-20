#![feature(noop_waker)]
#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use core::default::Default;
use core::panic::PanicInfo;

use cortex_m::peripheral::NVIC;
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_usb::{
    Builder,
    class::cdc_acm::{CdcAcmClass, State},
    driver::EndpointError,
};
use futures::future::{join, select};
use stm32_metapac;
use stm32_metapac::interrupt;
use stm32_metapac::USB_OTG_HS;

use u5_lib::{*, clock, gpio};
use u5_lib::{
    clock::delay_ms,
    gpio::{USART_RX_PA10, USART_TX_PA9},
};
use u5_lib::{exti, low_power::mcu_no_deep_sleep};
use u5_lib::usart::USART1;

// define defmt format
#[derive(defmt::Format)]
pub enum UsbError {
    BufferOverflow,
    Disabled,
}

const GREEN: gpio::GpioPort = gpio::PB7;

fn setup() {
    clock::init_clock();
    clock::set_gpio_clock();
    USART1.setup(USART_TX_PA9, USART_RX_PA10);
    GREEN.setup();
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    setup();
    mcu_no_deep_sleep();
    clock::kernel_freq_160mhz_request();
    defmt::info!("setup led finished!");
    spawner.spawn(btn()).unwrap();
    // USART1.send("start task success!\n".as_bytes());
    // core initialisation
    // enable usb clock
    let rcc = stm32_metapac::RCC;
    let syscfg = stm32_metapac::SYSCFG;
    let pwr = stm32_metapac::PWR;

    unsafe {
        NVIC::unmask(stm32_metapac::Interrupt::OTG_HS);
    }

    critical_section::with(|_| {
        rcc.ahb3enr().modify(|v| {
            v.set_pwren(true);
        });
        pwr.svmcr().modify(|w| {
            w.set_usv(true);
        });
        // rcc.pll1cfgr().modify(|v| {
        //     v.set_pllmboost(stm32_metapac::rcc::vals::Pllmboost::DIV2);
        // });
        pwr.vosr().modify(|v| {
            v.0 |= (1 << 19) | (1 << 20);
            // SBPWREN and USBBOOSTEN in PWR_VOSR.
            // v.boosten();
        });
        delay_ms(100);
        // wait fo USBBOOSTRDY
        // while !pwr.vosr().read().usbboostrdy() {}

        rcc.ccipr2().modify(|w| {
            w.set_otghssel(stm32_metapac::rcc::vals::Otghssel::HSE);
        });

        rcc.apb3enr().modify(|w| {
            w.set_syscfgen(true);
        });
        rcc.ahb2enr1().modify(|w| {
            w.set_usb_otg_hs_phyen(true);
            w.set_usb_otg_hsen(true);
        });
        unsafe {
            syscfg.otghsphycr().modify(|v| {
                v.set_clksel(0b11);
                v.set_en(true);
            });

            // let addr1 = syscfg.as_ptr();
            // // move to 0x74 next
            // let addr = addr1.wrapping_offset(0x74) as *mut u32;
            // addr.write_volatile(0b1100 as u32);
            // let addr2 = addr1.wrapping_offset(0x7C) as *mut u32;
            // addr2.write_volatile(0b101 as u32);
            // addr.write_volatile(0b1101 as u32);
        }
    });
    spawner.spawn(usb_task()).unwrap();

    // let r = stm32_metapac::USB_OTG_HS;
    // r.gahbcfg().modify(|w| {
    //     w.set_gint(true);
    //     w.set_txfelvl(true);
    // });
    // r.gintmsk().modify(|w| {
    //     w.set_rxflvlm(true);
    // });
    // r.gusbcfg().modify(|v| {
    //     v.set_tocal(0x7); //figure out what it is. timeout calibration
    //     v.set_trdt(9); // turn around time
    // });
    // r.gintmsk().modify(|w| {
    //     w.set_otgint(true);
    //     w.set_mmism(true);
    // });
    // // force device mode
    // r.gusbcfg().modify(|v| {
    //     // v.set_fhnslow(true);
    //     v.set_fdmod(true);
    // });
    // r.dctl().modify(|w| {
    //     w.set_sdis(true);
    // });
    defmt::info!("usb init finished!");
    loop {
        exti::EXTI13_PC13.wait_for_raising().await;
        GREEN.toggle();
        USART1.send("button clicked\n".as_bytes());
    }
}

// #[interrupt]
// fn OTG_HS() {
//     GREEN.toggle();
//     defmt::info!("OTG_HS interrupt");
//     unsafe {
//         // on_interrupt();
//         clock::delay_us(200);
//     }
// }

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    defmt::info!("panic");
    defmt::error!(
        "Location file name: {:?}, line: {:?}, col: {:?}",
        _info.location().unwrap().file(),
        _info.location().unwrap().line(),
        _info.location().unwrap().column()
    );

    loop {}
}

#[embassy_executor::task]
async fn btn() {
    let _last_time: (u8, u8, u8) = (0, 0, 0);
    defmt::info!("waiting for btn");
    loop {
        exti::EXTI13_PC13.wait_for_raising().await;
        GREEN.toggle();
        // defmt::info!("button clicked");
        //USB initsts
        defmt::info!(
            "HS INITSTS: 0x{:x}",
            stm32_metapac::USB_OTG_HS.gintsts().read().0
        );
        // :?}", stm32_metapac::USB_OTG_HS.  gintsts().read().0);
        defmt::info!(
            "HS INITSTS mask: 0x{:x}",
            stm32_metapac::USB_OTG_HS.gintmsk().read().0
        );
    }
}

#[embassy_executor::task]
pub async fn usb_task() {
    let _ep_out_buffer = [0u8; 256];
    let mut config = usb_otg::Config::default();
    config.vbus_detection = false;
    let driver = usb_otg::Driver::new(config, gpio::USB_DM_PA11, gpio::USB_DP_PA12);

    // // Create embassy-usb Config
    let mut config = embassy_usb::Config::new(0xaaaa, 0xefba);
    config.manufacturer = Some("ggeta");
    config.product = Some("USB-serial example");
    config.serial_number = Some("12345678");

    config.device_class = 0xEF;
    config.device_sub_class = 0x02;
    config.device_protocol = 0x01;
    config.composite_with_iads = true;

    let mut device_descriptor = [0; 512];
    let mut config_descriptor = [0; 512];
    let mut bos_descriptor = [0; 512];
    let mut control_buf = [0; 64];
    let mut msos_descriptor = [0; 512];

    let mut state = State::new();
    USART1.send("starting usb task new!\n\n".as_bytes());

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
    USART1.send("declare class success!\n".as_bytes());
    // Build the builder.
    let mut usb = builder.build();
    // USART1.send("success!\n".as_bytes());
    let usb_fut = usb.run(); // Run the USB device.
    let handler_fut = async {
        loop {
            class.wait_connection().await;
            defmt::info!("connected");
            let _ = usb_handler(&mut class).await;
            defmt::info!("disconnected");
        }
    };
    USART1.send("start usb task success!\n".as_bytes());
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

async fn usb_handler<'d>(class: &mut CdcAcmClass<'d, usb_otg::Driver>) -> Result<(), Disconnected> {
    let mut buf: [u8; 128] = [0; 128];
    // the maximum size of the command is 64 bytes
    defmt::info!("start usb handler");
    loop {
        // select(future1, future2)
        let ret = class.read_packet(&mut buf).await;
        match ret {
            Ok(n) => {
                defmt::info!("read {} bytes", n);
                class.write_packet(&buf[0..n]).await.unwrap();
            }
            Err(e) => {
                defmt::info!("error: {:?}", e);
                return Err(e.into());
            }
        }
        // class.write_packet(&buf[0..n]).await.unwrap();
    }
}
