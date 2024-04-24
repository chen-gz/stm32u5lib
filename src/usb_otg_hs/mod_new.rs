


pub static mut SETUP_DATA: [u8; 64] = [0; 64];
pub static mut SETUP_RETURN_DATA: [u8; 256] = [0; 256];

/// Initializes FIFOs based on the fifo_const.


pub fn init_setaddress(address: u8) {
    // RM0456 Rev 5, p3423
    // 1. program the otg_dcfg register to set the device address.
    regs().dcfg().modify(|w| w.set_dad(address));
}

// let res = process_setup_packet(SETUP_DATA);

use crate::usb_otg_hs::descriptor::*;
use crate::usb_otg_hs::endpoint_new::{Endpoint, EpType, MaxPacketSize};
use crate::usb_otg_hs::global_states::{regs};


pub struct SetupResponse {
    pub(crate) setup: SetupPacket,
    pub(crate) request: Request,
    pub(crate) data_stage_direction: Direction,
    pub(crate) has_data_stage: bool,
    pub(crate) data: [u8; 256],
    pub(crate) len: usize,
}

pub fn process_setup_packet_new(buf: &[u8]) -> SetupResponse {
    defmt::info!("process_setup_packet, {:x}", buf);
    let setup = SetupPacket::from_bytes(buf);
    let mut response = SetupResponse {
        setup,
        request: Request::from_setup(&setup),
        data_stage_direction: setup.direction,
        has_data_stage: setup.length != 0,
        data: [0; 256],
        len: setup.length as usize,
    };
    if setup.direction == Direction::In {
        match response.request {
            Request::GetDeviceDescriptor(len) => {
                let desc = USB_CDC_ACM_DEVICE_DESCRIPTOR.as_bytes();
                let len = core::cmp::min(len as usize, desc.len());
                for i in 0..len {
                    response.data[i] = desc[i];
                }
                response.len = len;
                defmt::info!("GetDeviceDescriptor, len={}", len);
            }
            Request::GetConfigurationDescriptor(len) => {
                let desc = cdc_conig_desc_full();
                let len = core::cmp::min(len as usize, desc.len());
                for i in 0..len {
                    response.data[i] = desc[i];
                }
                response.len = len;
                defmt::info!("GetConfigurationDescriptor, len={}", len);
            }
            Request::GetStringLangDescriptor(len) => {
                let val = StringDescriptor::language();
                for i in 0..val[0] as _ {
                    response.data[i] = val[i];
                }
                response.len = val[0] as usize;
                defmt::info!("GetStringLangDescriptor, len={}", len);
            }
            Request::GetStringManufacturerDescriptor(_len) => {
                let val = StringDescriptor::manufacturer("GGETA");
                for i in 0..val[0] as _ {
                    response.data[i] = val[i];
                }
                response.len = val[0] as usize;
            }
            Request::GetStringProductDescriptor(_len) => {
                let val = StringDescriptor::product("USB Example Device");
                for i in 0..val[0] as _ {
                    response.data[i] = val[i];
                }
                response.len = val[0] as usize;
            }
            Request::GetSerialNumberDescriptor(_len) => {
                let val = StringDescriptor::serial_number("123456");
                for i in 0..val[0] as _ {
                    response.data[i] = val[i];
                }
                response.len = val[0] as usize;
            }
            Request::GetDeviceQualifierDescriptor(_len) => {
                let val = USB_CDC_DEVICE_QUALIFIER_DESCRIPTOR.as_bytes();
                for i in 0..10 {
                    response.data[i] = val[i];
                }
                response.len = 10;
            }
            _ => {
                defmt::panic!("Unknown request");
            }
        }
    }
    return response;
}

pub struct EndpointGG;


// #[embassy_executor::task]
pub async fn cdc_acm_ep2_read() {
    defmt::info!("cdc_acm_ep2_read start");
    // let ep2_in = Endpoint::new(crate::usb_otg_hs::endpoint_new::Direction::In, 2, EpType::Bulk, MaxPacketSize::Size64, 0).unwrap();
    let ep2_out = Endpoint::new(crate::usb_otg_hs::endpoint_new::Direction::Out, 2, EpType::Bulk, MaxPacketSize::Size64, 0).unwrap();
    // let mut buf : [u8; 16] =
    //     let buf = "Hello, World!".as_bytes();
    // generate a buf with 100_000  bytes
    let mut buf = [08u8; 64];
    // // last byte is 0
    // buf[29_998] = 0;
    // buf[29_999] = 0;
    // ep2_in.write(&buf).await;
    // defmt::info!("ep2 write done, data={:x}", buf);
    let _ = ep2_out.read(&mut buf).await;
    defmt::info!("ep2 read done, data={:x}", buf);
}



