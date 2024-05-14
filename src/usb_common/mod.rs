pub mod descriptor;
use descriptor::*;
use aligned::Aligned;

pub trait BasicDescriptor {
    fn device_descriptor(&self) -> descriptor::USBDeviceDescriptor;
    /// Returns the configuration descriptor for the device.
    fn configuration_descriptor(&self) -> descriptor::ConfigurationDescriptor;
    fn string_lang_descriptor(&self) -> descriptor::StringDescriptor;
    fn string_manufacturer_descriptor(&self) -> descriptor::StringDescriptor;
    fn string_product_descriptor(&self) -> descriptor::StringDescriptor;
    fn string_serial_number_descriptor(&self) -> descriptor::StringDescriptor;
    fn device_qualifier_descriptor(&self) -> descriptor::DeviceQualifierDescriptor;
}

pub trait CdcSetup {
    fn get_line_coding(&self) -> [u8; 7];
    fn set_line_coding(&self, data: &[u8; 7]);
    fn set_control_line_state(&self, data: u16);
    fn send_break(&self, data: u16);
}


pub struct SetupResponse {
    pub setup: SetupPacket,
    pub request: Request,
    pub data_stage_direction: Direction,
    pub has_data_stage: bool,
    pub data: Aligned<aligned::A4, [u8; 256]>,
    pub len: usize,
}

pub fn process_setup_packet_new(buf: &[u8]) -> SetupResponse {
    assert!(buf.len() >= 8);
    defmt::info!("process_setup_packet, {:x}", buf);
    let setup = SetupPacket::from_bytes(buf);
    let mut response = SetupResponse {
        setup,
        request: Request::from_setup(&setup),
        data_stage_direction: setup.direction,
        has_data_stage: setup.length != 0,
        data: Aligned([0u8; 256]),
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
