#![allow(unused)]
use core::default;

use crate::{clock, gpio::{self, USART1_RX_PINS, USART1_TX_PINS}, hal};
use defmt::todo;
use stm32_metapac::{
    common::R,
    usart::vals::{Over8, Stop, M0, M1},
};
pub struct Usart {
    port: stm32_metapac::usart::Usart,
    port_num: u8,
}

// pub const USART1: UsartPort = UsartPort {
//     port: stm32_metapac::USART1,
//     port_num: 1,
// };

const USART_CLOCK: u32 = 16_000_000; // default use HSI16
pub struct Config {
    port_num: u8,
    gpio_tx: gpio::GpioPort,
    gpio_rx: gpio::GpioPort,
    pub baudrate: u32,
    pub data_bits: u8,
    pub stop_bits: u8,
    pub parity: u8,
    pub flow_control: u8,
}

impl default::Default for Config{
    fn default() -> Self {
        Config {
            port_num: 1,
            gpio_tx: gpio::USART_TX_PA9,
            gpio_rx: gpio::USART_RX_PA10,
            baudrate: 115200,
            data_bits: 8,
            stop_bits: 1,
            parity: 0,
            flow_control: 0,
        }
    }
}

fn port_num_to_usart(port_num: u8) -> stm32_metapac::usart::Usart {
    match port_num {
        1 => stm32_metapac::USART1,
        2 => stm32_metapac::USART2,
        3 => stm32_metapac::USART3,
        _ => panic!("invalid port number"),
    }
}
#[derive(core::fmt::Debug)]
pub enum UsartError {
    TAKEN,
    BufferOverflow,
    Disabled,
}

use embassy_sync::waitqueue::AtomicWaker;
const NEW_AWAKER: AtomicWaker = AtomicWaker::new();

static mut WAKERS: [AtomicWaker; 8] = [NEW_AWAKER; 8];
static mut TAKEN: [bool; 8] = [false; 8];

fn pin_to_port(tx: &gpio::GpioPort, rx: &gpio::GpioPort) -> u8 {
    if USART1_TX_PINS.contains(tx) && USART1_RX_PINS.contains(rx) {
        1
    } else  {
        todo!()
    }
}

use gpio::GpioPort;
impl hal::Usart<GpioPort> for Usart {
    // fn new() -> Result<Self, Self::Error> {
    fn new(baudrate: u32, tx: GpioPort, rx: GpioPort) -> Result<Self, hal::UsartError> 
        where Self: Sized {
        let port_num = pin_to_port(&tx, &rx);
        // let port_num = port;
        if unsafe { TAKEN[port_num as usize] } {
            return Err(hal::UsartError::InitError);
        }
        unsafe {
            TAKEN[port_num as usize] = true;
        }
        tx.setup();
        tx.setup();
        clock::set_usart_clock();
        // get port from port number
        let port = port_num_to_usart(port_num);
        // self.port
        port.cr1().modify(|v| {
            v.set_m0(M0::BIT8);
            v.set_m1(M1::M0);
            v.set_pce(false);
            v.set_over8(Over8::OVERSAMPLING16); // oversampling by 16

            v.set_ue(true);
            v.set_te(true);
            v.set_re(true);
        });
        port.cr2().modify(|v| {
            v.set_stop(Stop::STOP1); // 1 stop bit
        });
        port.cr3().modify(|v| {
            // v.set_owr_ddr(true);
        });
        port.brr().write(|v| {
            v.set_brr((USART_CLOCK / 115200) as u16);
        });
        Ok(Usart{ port, port_num: port_num })
    }
    fn read(&self, data: &mut [u8]) -> Result<(), hal::UsartError> {
        // todo!()
        for i in 0..data.len() {
            while self.port.isr().read().rxne() == false {}
            data[i] = self.port.rdr().read().dr() as u8;
        }
        Ok(())
    }
    // fn read_async(&self, data: &mut [u8]) -> impl core::future::Future<Output = Result<(), hal::UsartError>> + Send {
    //     async move {
    //         todo!()
    //     }
    // }
    async fn read_async(&self, data: &mut [u8]) -> Result<(), hal::UsartError> {
        todo!()
    }
    fn write(&self, data: &[u8]) -> Result<(), hal::UsartError> {
        for &c in data{
            while self.port.isr().read().txe() == false {}
            self.port.tdr().write(|v| {
                v.set_dr(c as u16);
            });
        }
        while self.port.isr().read().tc() == false {}
        Ok(())
    }
    // fn write_async(&self, data: &[u8]) -> impl core::future::Future<Output = Result<(), hal::UsartError>> + Send {
    //     async move {
    //         todo!()
    //     }
    // }
    async fn write_async(&self, data: &[u8]) -> Result<(), hal::UsartError> {
        todo!()
    }
}
// impl UsartPort {
//     /// current use default configuration, 115200 baudrate, 8 bit data, 1 stop bit, no parity
//     pub fn setup(&self, gpio_tx: gpio::GpioPort, gpio_rx: gpio::GpioPort) {
//         // todo!("enable usart1 clock and setup gpio for usart1");
//         gpio_tx.setup();
//         gpio_rx.setup();
//         clock::set_usart_clock();
//         // todo!("enable interrupt ");
//         // self.port
//         self.port.cr1().modify(|v| {
//             v.set_m0(M0::BIT8);
//             v.set_m1(M1::M0);
//             v.set_pce(false);
//             v.set_over8(Over8::OVERSAMPLING16); // oversampling by 16
//             v.set_ue(true);
//             v.set_te(true);
//             v.set_re(true);
//         });
//         self.port.cr2().modify(|v| {
//             v.set_stop(Stop::STOP1); // 1 stop bit
//         });
//         self.port.cr3().modify(|v| {
//             // v.set_owr_ddr(true);
//         });
//         self.port.brr().write(|v| {
//             v.set_brr((USART_CLOCK / 115200) as u16);
//         });
//     }
//     pub fn send(&self, data: &[u8]) {
//         for &c in data {
//             while self.port.isr().read().txe() == false {}
//             self.port.tdr().write(|v| {
//                 v.set_dr(c as u16);
//             });
//         }
//     }
//     pub fn recv(&self) -> u8 {
//         todo!();
//     }
// }



// impl<'a> ComInterface<'a> for Usart {
//     type Error = UsartError;
//     type Message = &'a [u8];
//     type Response = &'a [u8];
//     type Config = Config;
//     type ReceiveOption = ();
//     fn new(config: Self::Config) -> Result<Self, Self::Error> {
//         if unsafe { TAKEN[config.port_num as usize] } {
//             return Err(UsartError::TAKEN);
//         }
//         unsafe {
//             TAKEN[config.port_num as usize] = true;
//         }
//         config.gpio_rx.setup();
//         config.gpio_tx.setup();
//         clock::set_usart_clock();
//         // get port from port number
//         let port = port_num_to_usart(config.port_num);
//         // self.port
//         port.cr1().modify(|v| {
//             v.set_m0(M0::BIT8);
//             v.set_m1(M1::M0);
//             v.set_pce(false);
//             v.set_over8(Over8::OVERSAMPLING16); // oversampling by 16
//
//             v.set_ue(true);
//             v.set_te(true);
//             v.set_re(true);
//         });
//         port.cr2().modify(|v| {
//             v.set_stop(Stop::STOP1); // 1 stop bit
//         });
//         port.cr3().modify(|v| {
//             // v.set_owr_ddr(true);
//         });
//         port.brr().write(|v| {
//             v.set_brr((USART_CLOCK / 115200) as u16);
//         });
//         Ok(Usart{ port, port_num: config.port_num })
//     }
//
//     fn send(&mut self, message: &Self::Message) -> Result<(), Self::Error> {
//         for &c in *message {
//             while self.port.isr().read().txe() == false {}
//             self.port.tdr().write(|v| {
//                 v.set_dr(c as u16);
//             });
//         }
//         Ok(())
//     }
//
//     fn receive(&mut self, option: Self::ReceiveOption) -> Result<Self::Response, Self::Error> {
//         todo!()
//     }
//
//     // fn receive(&mut self) -> Result<Self::Message, Self::Error> {
//     //     todo!()
//     // }
//
//
//     async fn send_async(&mut self, message: Self::Message) -> Result<(), Self::Error> {
//         todo!()
//     }
//
//     async fn receive_async(&mut self) ->  Result<Self::Message, Self::Error> {
//         todo!()
//     }
//
//     fn enable(&mut self) -> Result<(), Self::Error> {
//         todo!()
//     }
//
//     fn disable(&mut self) -> Result<(), Self::Error> {
//         todo!()
//     }
//
//     fn drop(&mut self) -> Result<(), Self::Error> {
//         todo!()
//     }
//
//     fn wait_connection(&mut self) -> Result<(), Self::Error> {
//         todo!()
//     }
//
// }
