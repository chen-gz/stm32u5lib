#[derive(Copy, Clone, PartialEq)]
pub enum Direction {
    /// Host to device
    Out,
    /// Device to host
    In,
}

#[derive(Copy, Clone)]
pub enum RequestType {
    Standard,
    Class,
    Vendor,
    Reserved,
}

#[derive(Copy, Clone)]
pub enum Recipient {
    Device,
    Interface,
    Endpoint,
    Other,
    Reserved,
}

#[derive(Copy, Clone)]
pub struct SetupPacket {
    pub direction: Direction,
    pub request_type: RequestType,
    pub recipient: Recipient,
    pub request: u8,
    pub value: u16,
    pub index: u16,
    pub length: u16,
}

pub enum Request {
    GetDeviceDescriptor(u8),
    GetConfigurationDescriptor(u8),
    GetStringLangDescriptor(u8),
    GetStringManufacturerDescriptor(u8),
    GetStringProductDescriptor(u8),
    GetSerialNumberDescriptor(u8),
    GetDeviceQualifierDescriptor(u8),


    SetAddress(u8),
    SetConfiguration(u8),


    // CDC
    // SetLineCoding(u8, u32, u8, u8, u8),// (length, baudrate, stopbits, parity, databits)
    SetLineCoding(u8),  // , u32, u8, u8, u8),// (length, baudrate, stopbits, parity, databits)
    SetControlLineState(u8),
    ClearFeature(u16),
}

const REQUEST_GET_DESCRIPTOR: u8 = 0x06;

impl Request {
    pub fn from_setup(setup: &SetupPacket) -> Request {
        match setup.direction {
            Direction::Out => {
                match setup.request_type {
                    RequestType::Standard => {
                        match setup.recipient {
                            Recipient::Device => {
                                match setup.request {
                                    0x05 => Request::SetAddress(setup.value as u8),
                                    0x09 => Request::SetConfiguration(setup.value as u8),
                                    _ => defmt::panic!("Unknown request"),
                                }
                            }
                            Recipient::Endpoint => {
                                match setup.request {
                                    0x01 => Request::ClearFeature(setup.value),
                                    _ => defmt::panic!("Unknown request"),
                                }
                            }
                            _ => defmt::panic!("Unknown recipient"),
                        }
                    }
                    RequestType::Class => {
                        match setup.recipient {
                            Recipient::Interface => {
                                match setup.request {
                                    0x20 => Request::SetLineCoding(setup.length as u8),
                                    0x22 => Request::SetControlLineState(setup.length as u8),
                                    _ => defmt::panic!("Unknown request"),
                                }
                            }
                            _ => defmt::panic!("Unknown recipient"),
                        }
                    }
                    _ => defmt::panic!("Unknown request type"),
                }
            }
            Direction::In => {
                match setup.request_type {
                    RequestType::Standard => {
                        match setup.recipient {
                            Recipient::Device => {
                                match setup.request {
                                    REQUEST_GET_DESCRIPTOR => {
                                        match setup.value {
                                            0x0100 => Request::GetDeviceDescriptor(setup.length as u8),
                                            0x0200 => Request::GetConfigurationDescriptor(setup.length as u8),
                                            0x0300 => Request::GetStringLangDescriptor(setup.length as u8),
                                            0x0301 => Request::GetStringManufacturerDescriptor(setup.length as u8),
                                            0x0302 => Request::GetStringProductDescriptor(setup.length as u8),
                                            0x0303 => Request::GetSerialNumberDescriptor(setup.length as u8),
                                            0x0600 => Request::GetDeviceQualifierDescriptor(setup.length as u8),
                                            _ => defmt::panic!("Unknown descriptor"),
                                        }
                                    }
                                    _ => defmt::panic!("Unknown request"),
                                }
                            }
                            // Recipient::Interface => {}
                            // Recipient::Endpoint => {}
                            // Recipient::Other => {}
                            // Recipient::Reserved => {}
                            _ => defmt::panic!("No support for other recipient"),
                        }
                    }

                    // RequestType::Class => {}
                    // RequestType::Vendor => {}
                    // RequestType::Reserved => {}
                    _ => defmt::panic!("no support for other request type"),
                }
            }
        }
    }
}

impl SetupPacket {
    pub fn from_bytes(data: &[u8]) -> SetupPacket {
        let direction = if data[0] & 0x80 == 0x80 {
            Direction::In
        } else {
            Direction::Out
        };
        let request_type = match data[0] & 0x60 {
            0x00 => RequestType::Standard,
            0x20 => RequestType::Class,
            0x40 => RequestType::Vendor,
            _ => RequestType::Reserved,
        };
        let recipient = match data[0] & 0x1F {
            0x00 => Recipient::Device,
            0x01 => Recipient::Interface,
            0x02 => Recipient::Endpoint,
            0x03 => Recipient::Other,
            _ => Recipient::Reserved,
        };
        let request = data[1];
        let value = u16::from_le_bytes([data[2], data[3]]);
        let index = u16::from_le_bytes([data[4], data[5]]);
        let length = u16::from_le_bytes([data[6], data[7]]);
        SetupPacket {
            direction,
            request_type,
            recipient,
            request,
            value,
            index,
            length,
        }
    }
}


pub struct USBDeviceDescriptor {
    pub b_length: u8,
    pub b_descriptor_type: u8,
    pub bcd_usb: u16,
    pub b_device_class: u8,
    pub b_device_sub_class: u8,
    pub b_device_protocol: u8,
    pub b_max_packet_size0: u8,
    pub id_vendor: u16,
    pub id_product: u16,
    pub bcd_device: u16,
    pub i_manufacturer: u8,
    pub i_product: u8,
    pub i_serial_number: u8,
    pub b_num_configurations: u8,
}

impl USBDeviceDescriptor {
    pub fn as_bytes(&self) -> [u8; 18] {
        // length is 18
        let mut buf = [0u8; 18];
        buf[0] = self.b_length;
        buf[1] = self.b_descriptor_type;
        buf[2] = self.bcd_usb as u8;
        buf[3] = (self.bcd_usb >> 8) as u8;
        buf[4] = self.b_device_class;
        buf[5] = self.b_device_sub_class;
        buf[6] = self.b_device_protocol;
        buf[7] = self.b_max_packet_size0;
        buf[8] = self.id_vendor as u8;
        buf[9] = (self.id_vendor >> 8) as u8;
        buf[10] = self.id_product as u8;
        buf[11] = (self.id_product >> 8) as u8;
        buf[12] = self.bcd_device as u8;
        buf[13] = (self.bcd_device >> 8) as u8;
        buf[14] = self.i_manufacturer;
        buf[15] = self.i_product;
        buf[16] = self.i_serial_number;
        buf[17] = self.b_num_configurations;
        buf
    }
}

pub struct DeviceQualifierDescriptor {
    pub b_length: u8,
    pub b_descriptor_type: u8,
    pub bcd_usb: u16,
    pub b_device_class: u8,
    pub b_device_sub_class: u8,
    pub b_device_protocol: u8,
    pub b_max_packet_size0: u8,
    pub b_num_configurations: u8,
    pub b_reserved: u8,
}

impl DeviceQualifierDescriptor {
    pub fn as_bytes(&self) -> [u8; 10] {
        // length is 10
        let mut buf = [0u8; 10];
        buf[0] = self.b_length;
        buf[1] = self.b_descriptor_type;
        buf[2] = self.bcd_usb as u8;
        buf[3] = (self.bcd_usb >> 8) as u8;
        buf[4] = self.b_device_class;
        buf[5] = self.b_device_sub_class;
        buf[6] = self.b_device_protocol;
        buf[7] = self.b_max_packet_size0;
        buf[8] = self.b_num_configurations;
        buf[9] = self.b_reserved;
        buf
    }
}

pub struct ConfigurationDescriptor {
    pub b_length: u8,
    // Size of this descriptor in bytes
    pub b_descriptor_type: u8,
    // Configuration descriptor type (2)
    pub w_total_length: u16,
    // Total length of data returned for this configuration
    pub b_num_interfaces: u8,
    // Number of interfaces in this configuration
    pub b_configuration_value: u8,
    // Value to use as an argument to select this configuration
    pub i_configuration: u8,
    // Index of string descriptor describing this configuration
    pub bm_attributes: u8,
    // Configuration characteristics
    pub b_max_power: u8,   // Maximum power consumption  (2mA units)
    // the configuration descriptor is followed by the interface descriptor and endpoint descriptor
}

impl ConfigurationDescriptor {
    pub fn as_bytes(&self) -> [u8; 9] {
        // length is 9
        let mut buf = [0u8; 9];
        buf[0] = self.b_length;
        buf[1] = self.b_descriptor_type;
        buf[2] = self.w_total_length as u8;
        buf[3] = (self.w_total_length >> 8) as u8;
        buf[4] = self.b_num_interfaces;
        buf[5] = self.b_configuration_value;
        buf[6] = self.i_configuration;
        buf[7] = self.bm_attributes;
        buf[8] = self.b_max_power;
        buf
    }
}

pub struct InterfaceDescriptor {
    pub b_length: u8,
    // Size of this descriptor in bytes
    pub b_descriptor_type: u8,
    // Interface descriptor type (4)
    pub b_interface_number: u8,
    // Number of this interface
    pub b_alternate_setting: u8,
    // Value used to select this alternate setting for the interface
    pub b_num_endpoints: u8,
    // Number of endpoints used by this interface (excluding endpoint zero)
    pub b_interface_class: u8,
    // Class code (assigned by the USB-IF). 0xFF is vendor-specific
    pub b_interface_sub_class: u8,
    // Sub class code (assigned by the USB-IF)
    pub b_interface_protocol: u8,
    // Protocol code (assigned by the USB). 0xFF is vendor-specific
    pub i_interface: u8, // Index of string descriptor describing this interface
}

impl InterfaceDescriptor {
    pub fn as_bytes(&self) -> [u8; 9] {
        // length is 9
        let mut buf = [0u8; 9];
        buf[0] = self.b_length;
        buf[1] = self.b_descriptor_type;
        buf[2] = self.b_interface_number;
        buf[3] = self.b_alternate_setting;
        buf[4] = self.b_num_endpoints;
        buf[5] = self.b_interface_class;
        buf[6] = self.b_interface_sub_class;
        buf[7] = self.b_interface_protocol;
        buf[8] = self.i_interface;
        buf
    }
}

pub struct EndpointDescriptor {
    pub b_length: u8,
    // Size of this descriptor in bytes
    pub b_descriptor_type: u8,
    // Endpoint descriptor type (5)
    pub b_endpoint_address: u8,
    // The address of the endpoint on the USB device described by this descriptor
    pub bm_attributes: u8,
    // The endpoint's attributes when it is configured using the bEndpointAddress
    pub w_max_packet_size: u16,
    // Maximum packet size this endpoint is capable of sending or receiving
    pub b_interval: u8, // Interval for polling endpoint for data transfers
}

impl EndpointDescriptor {
    pub fn as_bytes(&self) -> [u8; 7] {
        // length is 7
        let mut buf = [0u8; 7];
        buf[0] = self.b_length;
        buf[1] = self.b_descriptor_type;
        buf[2] = self.b_endpoint_address;
        buf[3] = self.bm_attributes;
        buf[4] = self.w_max_packet_size as u8;
        buf[5] = (self.w_max_packet_size >> 8) as u8;
        buf[6] = self.b_interval;
        buf
    }
}

pub struct InterfaceAssociationDescriptor {
    pub b_length: u8,
    // Size of this descriptor in bytes
    pub b_descriptor_type: u8,
    // Interface association descriptor type (11)
    pub b_first_interface: u8,
    // The first interface number associated with this function
    pub b_interface_count: u8,
    // The number of contiguous interfaces associated with this function
    pub b_function_class: u8,
    // Class code (assigned by the USB-IF). 0xFF is vendor-specific
    pub b_function_sub_class: u8,
    // Sub class code (assigned by the USB-IF)
    pub b_function_protocol: u8,
    // Protocol code (assigned by the USB). 0xFF is vendor-specific
    pub i_function: u8, // Index of string descriptor describing this function
}

impl InterfaceAssociationDescriptor {
    pub fn as_bytes(&self) -> [u8; 8] {
        // length is 8
        let mut buf = [0u8; 8];
        buf[0] = self.b_length;
        buf[1] = self.b_descriptor_type;
        buf[2] = self.b_first_interface;
        buf[3] = self.b_interface_count;
        buf[4] = self.b_function_class;
        buf[5] = self.b_function_sub_class;
        buf[6] = self.b_function_protocol;
        buf[7] = self.i_function;
        buf
    }
}

pub struct StringDescriptor {
    pub b_length: u8,
    // Size of this descriptor in bytes
    pub b_descriptor_type: u8,
    // String descriptor type (3)
    pub w_data: [u16; 32], // String data
}

impl StringDescriptor {
    pub fn as_bytes(&self) -> [u8; 64] {
        // length is 64
        let mut buf = [0u8; 64];
        buf[0] = self.b_length;
        buf[1] = self.b_descriptor_type;
        for i in 0..20 {
            buf[2 + 2 * i] = self.w_data[i] as u8;
            buf[3 + 2 * i] = (self.w_data[i] >> 8) as u8;
        }
        buf
    }
    pub fn manufacturer(str: &str) -> [u8; 64] {
        let mut descriptor = StringDescriptor {
            b_length: str.len() as u8 * 2 + 2,
            b_descriptor_type: 3,
            w_data: [0; 32],
        };
        let mut _index = 0;
        for (i, c) in str.chars().enumerate() {
            descriptor.w_data[i] = c as u16;
            _index += 1;
        }
        descriptor.as_bytes()
    }

    pub fn product(str: &str) -> [u8; 64] {
        let mut descriptor = StringDescriptor {
            b_length: str.len() as u8 * 2 + 2,
            b_descriptor_type: 3,
            w_data: [0; 32],
        };
        let mut _index = 0;
        for (i, c) in str.chars().enumerate() {
            descriptor.w_data[i] = c as u16;
            _index += 1;
        }
        descriptor.as_bytes()
    }
    pub fn serial_number(str: &str) -> [u8; 64] {
        let mut descriptor = StringDescriptor {
            b_length: str.len() as u8 * 2 + 2,
            b_descriptor_type: 3,
            w_data: [0; 32],
        };
        let mut _index = 0;
        for (i, c) in str.chars().enumerate() {
            descriptor.w_data[i] = c as u16;
            _index += 1;
        }
        descriptor.as_bytes()
    }
    pub fn language() -> [u8; 4] {
        [4, 3, 0x09, 0x04] // english
    }
}


/////////////////////////// descriptors for cdc ///////////////////////////
pub const USB_CDC_ACM_DEVICE_DESCRIPTOR: USBDeviceDescriptor = USBDeviceDescriptor {
    b_length: 18,
    b_descriptor_type: 1,
    bcd_usb: 0x0200,             // modify to match the USB version
    b_device_class: 0x02,        // CDC class
    b_device_sub_class: 0x02,
    b_device_protocol: 0x01,
    b_max_packet_size0: 64,
    // id_vendor: 0x0483,           // STMicroelectronics
    // id_product: 0x5740,
    id_vendor: 0x05e1,
    id_product: 0x16c0,
    // Virtual COM Port
    bcd_device: 0x0100,          // device release number
    i_manufacturer: 0x01,
    i_product: 0x02,
    i_serial_number: 0x03,
    b_num_configurations: 0x01,
};
pub const USB_CDC_DEVICE_QUALIFIER_DESCRIPTOR: DeviceQualifierDescriptor = DeviceQualifierDescriptor {
    b_length: 10,
    b_descriptor_type: 6,
    bcd_usb: 0x0200,             // modify to match the USB version
    b_device_class: 0x02,        // CDC class
    b_device_sub_class: 0x02,
    b_device_protocol: 0x01,
    b_max_packet_size0: 64,
    b_num_configurations: 0x01,
    b_reserved: 0x00,
};
const CDC_ACM_CONFIGURATION_DESCRIPTOR: ConfigurationDescriptor = ConfigurationDescriptor {
    b_length: 9,
    b_descriptor_type: 2,
    w_total_length: CDC_CONFIG_DESC_LEN as _,    // this value is calculated based on the size of the descriptors
    b_num_interfaces: 2,
    b_configuration_value: 1,
    i_configuration: 0,
    bm_attributes: 0x80,  // bus powered
    b_max_power: 50,     // 100mA
};
const CDC_ACM_INTERFACE_ASSOCIATION_DESCRIPTOR: InterfaceAssociationDescriptor = InterfaceAssociationDescriptor {
    b_length: 8,
    b_descriptor_type: 11,
    b_first_interface: 0,
    b_interface_count: 2,
    b_function_class: 0x02,    // CDC
    b_function_sub_class: 0x02,    // ACM
    b_function_protocol: 0x01,    // AT commands
    i_function: 0,
};
const CDC_ACM_INTERFACE_DESCRIPTOR: InterfaceDescriptor = InterfaceDescriptor {
    b_length: 9,
    b_descriptor_type: 4,
    b_interface_number: 0,
    b_alternate_setting: 0,
    b_num_endpoints: 1,
    b_interface_class: 0x02,    // CDC
    b_interface_sub_class: 0x02,    // ACM
    b_interface_protocol: 0x01,    // AT commands
    i_interface: 0,
};
const CDC_ACM_HEADER_FUNCTIONAL_DESCRIPTOR: [u8; 5] = [0x05, 0x24, 0x00, 0x10, 0x01];
const CDC_ACM_CALL_MANAGEMENT_FUNCTIONAL_DESCRIPTOR: [u8; 5] = [0x05, 0x24, 0x01, 0x03, 0x01];
const CDC_ACM_FUNCTIONAL_DESCRIPTOR: [u8; 4] = [0x04, 0x24, 0x02, 0x02];
// chcek the last byte
const CDC_ACM_UNION_FUNCTIONAL_DESCRIPTOR: [u8; 5] = [0x05, 0x24, 0x06, 0x00, 0x01];

const CDC_ACM_ENDPOINT_DESCRIPTOR: EndpointDescriptor = EndpointDescriptor {
    b_length: 7,
    b_descriptor_type: 5,
    b_endpoint_address: 0x81,
    bm_attributes: 0x03,
    w_max_packet_size: 64,
    b_interval: 10,
};
const CDC_ACM_INTERFACE_DESCRIPTOR_2: InterfaceDescriptor = InterfaceDescriptor {
    b_length: 9,
    b_descriptor_type: 4,
    b_interface_number: 1,
    b_alternate_setting: 0,
    b_num_endpoints: 2,
    b_interface_class: 0x0A,    // CDC data
    b_interface_sub_class: 0x00,
    b_interface_protocol: 0x00,
    i_interface: 0,
};
const CDC_ACM_DATA_ENDPOINT_DESCRIPTOR_IN: EndpointDescriptor = EndpointDescriptor {
    b_length: 7,
    b_descriptor_type: 5,
    b_endpoint_address: 0x82,
    bm_attributes: 0x02,
    w_max_packet_size: 64,
    b_interval: 0x00,
};
const CDC_ACM_DATA_ENDPOINT_DESCRIPTOR_OUT: EndpointDescriptor = EndpointDescriptor {
    b_length: 7,
    b_descriptor_type: 5,
    b_endpoint_address: 0x02,
    bm_attributes: 0x02,
    w_max_packet_size: 64,
    b_interval: 0x00,
};
const CDC_CONFIG_DESC_LEN: usize = 9 + 8 + 9 + 5 + 5 + 4 + 5 + 7 + 9 + 7 + 7;

// 75 this can be finish in one packet
pub fn cdc_conig_desc_full() -> [u8; CDC_CONFIG_DESC_LEN] {
    // return the full configuration descriptor
    let mut buf = [0u8; CDC_CONFIG_DESC_LEN];
    let mut index = 0;
    let temp = CDC_ACM_CONFIGURATION_DESCRIPTOR.as_bytes();
    for i in 0..temp.len() {
        buf[index] = temp[i];
        index += 1;
    }
    let temp = CDC_ACM_INTERFACE_ASSOCIATION_DESCRIPTOR.as_bytes();
    for i in 0..temp.len() {
        buf[index] = temp[i];
        index += 1;
    }
    let temp = CDC_ACM_INTERFACE_DESCRIPTOR.as_bytes();
    for i in 0..temp.len() {
        buf[index] = temp[i];
        index += 1;
    }
    for i in 0..CDC_ACM_HEADER_FUNCTIONAL_DESCRIPTOR.len() {
        buf[index] = CDC_ACM_HEADER_FUNCTIONAL_DESCRIPTOR[i];
        index += 1;
    }
    for i in 0..CDC_ACM_CALL_MANAGEMENT_FUNCTIONAL_DESCRIPTOR.len() {
        buf[index] = CDC_ACM_CALL_MANAGEMENT_FUNCTIONAL_DESCRIPTOR[i];
        index += 1;
    }
    for i in 0..CDC_ACM_FUNCTIONAL_DESCRIPTOR.len() {
        buf[index] = CDC_ACM_FUNCTIONAL_DESCRIPTOR[i];
        index += 1;
    }
    for i in 0..CDC_ACM_UNION_FUNCTIONAL_DESCRIPTOR.len() {
        buf[index] = CDC_ACM_UNION_FUNCTIONAL_DESCRIPTOR[i];
        index += 1;
    }
    let temp = CDC_ACM_ENDPOINT_DESCRIPTOR.as_bytes();
    for i in 0..temp.len() {
        buf[index] = temp[i];
        index += 1;
    }
    let temp = CDC_ACM_INTERFACE_DESCRIPTOR_2.as_bytes();
    for i in 0..temp.len() {
        buf[index] = temp[i];
        index += 1;
    }
    let temp = CDC_ACM_DATA_ENDPOINT_DESCRIPTOR_IN.as_bytes();
    for i in 0..temp.len() {
        buf[index] = temp[i];
        index += 1;
    }
    let temp = CDC_ACM_DATA_ENDPOINT_DESCRIPTOR_OUT.as_bytes();
    for i in 0..temp.len() {
        buf[index] = temp[i];
        index += 1;
    }
    buf
}
