pub static mut SETUP_DATA: [u8; 64] = [0; 64];
pub static mut SETUP_RETURN_DATA: [u8; 256] = [0; 256];

pub fn init_setaddress(address: u8) {
    // RM0456 Rev 5, p3423
    // 1. program the otg_dcfg register to set the device address.
    regs().dcfg().modify(|w| w.set_dad(address));
}

// let res = process_setup_packet(SETUP_DATA);

use aligned::Aligned;
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
        defmt::info!("IN");
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
                defmt::info!("GetStringManufacturerDescriptor, len={}", _len);
            }
            Request::GetStringProductDescriptor(_len) => {
                let val = StringDescriptor::product("USB Example Device");
                for i in 0..val[0] as _ {
                    response.data[i] = val[i];
                }
                response.len = val[0] as usize;
                defmt::info!("GetStringProductDescriptor, len={}", _len);
            }
            Request::GetSerialNumberDescriptor(_len) => {
                let val = StringDescriptor::serial_number("123456");
                for i in 0..val[0] as _ {
                    response.data[i] = val[i];
                }
                response.len = val[0] as usize;
                defmt::info!("GetSerialNumberDescriptor, len={}", _len);
            }
            Request::GetDeviceQualifierDescriptor(_len) => {
                let val = USB_CDC_DEVICE_QUALIFIER_DESCRIPTOR.as_bytes();
                for i in 0..10 {
                    response.data[i] = val[i];
                }
                response.len = 10;
                defmt::info!("GetDeviceQualifierDescriptor, len={}", _len);
            }
            Request::GetLineCoding(_len) => {
                // 115200 8N1
                let val = [0x00, 0xc2, 0x01, 0x00, 0x00, 0x00, 0x08];
                for i in 0..7 {
                    response.data[i] = val[i];
                }
                response.len = 7;
                defmt::info!("GetLineCoding, len={}", _len);
            }
            _ => {
                panic!("Unknown request");
            }
        }
    }
    return response;
}

pub struct EndpointGG;


pub async fn cdc_acm_ep2_read() -> (Aligned<aligned::A4, [u8; 64]>, usize) {
    let ep2_out = Endpoint::new(crate::usb_otg_hs::endpoint_new::Direction::Out, 2, EpType::Bulk, MaxPacketSize::Size64, 0).unwrap();
    let mut buf = [0u32; 16];
    // convert u32 to u8 using unsafe a pointer
    // let mut _buf = unsafe { core::mem::transmute::<[u32; 16], [u8; 64]>(buf) };
    // let mut _buf = unsafe { core::mem::transmute(&mut buf) };
    // let mut _buf: &mut [u8; 64] = unsafe { core::mem::transmute(&mut buf) };
    let mut buf : Aligned<aligned::A4, [u8; 64]> = Aligned([0u8; 64]);
    match ep2_out.read(&mut buf[0..64]).await {
        Ok(len) => {
            defmt::info!("cdc_acm_ep2_read, {:?}, len={}", &buf[0..len], len);
            return (buf, len);
        }
        Err(e) => {
            // defmt::info!("cdc_acm_ep2_read, {:?}", e);
            return (buf, 0);
        }
    }
}
pub async fn cdc_acm_ep2_write(buf: &[u8]) {
    let ep2_in = Endpoint::new(crate::usb_otg_hs::endpoint_new::Direction::In, 2, EpType::Bulk, MaxPacketSize::Size64, 0).unwrap();
    let _ = ep2_in.write(buf).await;
}



