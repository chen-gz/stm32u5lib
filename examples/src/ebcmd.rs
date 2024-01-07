use core::{fmt, panic};
#[derive(Debug)]
pub enum Command {
    /// Set the time, year, month, day, hour, minute, second
    SetTim(u8, u8, u8, u8, u8, u8), // year, month, day, hour, minute, second
    GetTim,
    GetPic(u32), // get picture with id
    GetPicNum,   // get the number of pictures
    ClearPic,
}
impl Command {
    pub fn from_array(buf: &[u8]) -> Command {
        match buf[0] {
            0x01 => Command::SetTim(buf[1], buf[2], buf[3], buf[4], buf[5], buf[6]),
            0x02 => Command::GetTim,
            0x03 => {
                let id = ((buf[1] as u32) << 24)
                    | ((buf[2] as u32) << 16)
                    | ((buf[3] as u32) << 8)
                    | (buf[4] as u32);
                Command::GetPic(id)
            }
            0x04 => Command::GetPicNum,
            0x05 => Command::ClearPic,
            _ => panic!("unknown command"),
        }
    }
    pub fn to_array(&self) -> ([u8; 64], usize) {
        match self {
            Command::SetTim(year, month, day, hour, minute, second) => {
                let mut buf = [0; 64];
                buf[0] = 0x01;
                buf[1] = *year;
                buf[2] = *month;
                buf[3] = *day;
                buf[4] = *hour;
                buf[5] = *minute;
                buf[6] = *second;
                (buf, 7)
            }
            Command::GetTim => {
                let mut buf = [0; 64];
                buf[0] = 0x02;
                (buf, 1)
            }
            Command::GetPic(id) => {
                let mut buf = [0; 64];
                buf[0] = 0x03;
                buf[1] = (*id >> 24) as u8;
                buf[2] = (*id >> 16) as u8;
                buf[3] = (*id >> 8) as u8;
                buf[4] = *id as u8;
                (buf, 5)
            }
            Command::GetPicNum => {
                let mut buf = [0; 64];
                buf[0] = 0x04;
                (buf, 1)
            }
            Command::ClearPic => {
                let mut buf = [0; 64];
                buf[0] = 0x05;
                (buf, 1)
            }
        }
    }
}


// #[derive(Debug)]
pub enum Response {
    SetTim(u8), // response for set time. 0 for success, 1 for fail
    /// Get the time, year, month, day, hour, minute, second
    GetTim(u8, u8, u8, u8, u8, u8), // year, month, day, hour, minute, second
    GetPic(u32, u16, u8, [u8; 48]), // id(4), ordinal(2), data length(1), data(48) = 55
    GetPicNum(u32), // get the number of pictures
    ClearPic(u8), // clear all pictures. Set the picture number to 10
}
impl fmt::Debug for Response {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Response::SetTim(status) => {
                if status == &0 {
                    write!(f, "SetTim: success")
                } else {
                    write!(f, "SetTim: fail")
                }
            }
            Response::GetTim(year, month, day, hour, minute, second) => write!(
                f,
                // all use 2 digits
                "GetTim: {:02}-{:02}-{:02} {:02}:{:02}:{:02}",
                year, month, day, hour, minute, second
            ),
            Response::GetPic(id, ordinal, data_length, _data) => write!(
                f,
                "GetPic: id: {}, ordinal: {}, data_length: {}, data hex: {:x?}",
                id, ordinal, data_length, _data
            ),
            Response::GetPicNum(num) => write!(f, "GetPicNum: {}", num),
            Response::ClearPic(status) => {
                if status == &0 {
                    write!(f, "ClearPic: success")
                } else {
                    write!(f, "ClearPic: fail")
                }
            }
        }
    }
}

impl Response {
    pub fn from_array(buf: &[u8]) -> Response {
        match buf[0] {
            0x01 => Response::SetTim(buf[1]),
            0x02 => Response::GetTim(buf[1], buf[2], buf[3], buf[4], buf[5], buf[6]),
            0x03 => {
                let id = ((buf[1] as u32) << 24)
                    | ((buf[2] as u32) << 16)
                    | ((buf[3] as u32) << 8)
                    | (buf[4] as u32);
                let ordinal = ((buf[5] as u16) << 8) | (buf[6] as u16);
                Response::GetPic(id, ordinal, buf[7], buf[8..56].try_into().unwrap())
            }
            0x04 => {
                // get picture number
                let num = ((buf[1] as u32) << 24)
                    | ((buf[2] as u32) << 16)
                    | ((buf[3] as u32) << 8)
                    | (buf[4] as u32);

                Response::GetPicNum(num)
            }
            0x05 => Response::ClearPic(buf[1]),
            _ => panic!("unknown response"),
        }
    }
    pub fn to_array(&self) -> ([u8; 64], usize) {
        match self {
            Response::SetTim(status) => {
                let mut buf = [0; 64];
                buf[0] = 0x01;
                buf[1] = *status;
                (buf, 2)
            }
            Response::GetTim(year, month, day, hour, minute, second) => {
                let mut buf = [0; 64];
                buf[0] = 0x02;
                buf[1] = *year;
                buf[2] = *month;
                buf[3] = *day;
                buf[4] = *hour;
                buf[5] = *minute;
                buf[6] = *second;
                (buf, 7)
            }
            Response::GetPic(id, ordinal, data_length, data) => {
                let mut buf = [0; 64];
                buf[0] = 0x03;
                buf[1] = (*id >> 24) as u8;
                buf[2] = (*id >> 16) as u8;
                buf[3] = (*id >> 8) as u8;
                buf[4] = *id as u8;
                buf[5] = (*ordinal >> 8) as u8;
                buf[6] = *ordinal as u8;
                buf[7] = *data_length;
                buf[8..56].copy_from_slice(data);
                (buf, 56)
            }
            Response::GetPicNum(num) => {
                let mut buf = [0; 64];
                buf[0] = 0x04;
                buf[1] = (*num >> 24) as u8;
                buf[2] = (*num >> 16) as u8;
                buf[3] = (*num >> 8) as u8;
                buf[4] = *num as u8;
                (buf, 5)
            }
            Response::ClearPic(status) => {
                let mut buf = [0; 64];
                buf[0] = 0x05;
                buf[1] = *status;
                (buf, 2)
            }
        }
    }
    pub fn pic_res_from_data(id: u32, ordinal: u16, buf: &[u8]) -> (u16, usize, Response) {
        let len = buf.len();
        let mut data = [0; 48];
        let res_len:usize;
        if len > 48 {
            res_len = 48;
        } else {
            res_len = len;
        }
            // data.copy_from_slice(&buf[0..48]);
        if res_len == 0 {
            return (ordinal, res_len, Response::GetPic(id, ordinal, res_len as u8, data));
        }
        // copy to data 
        for i in 0..res_len {
            data[i] = buf[i];
        }
        // data.copy_from_slice(&buf[0..res_len]);
        (ordinal+1, res_len, Response::GetPic(id, ordinal, res_len as u8, data))
    }
}
