use core::fmt;
#[derive(Debug)]
pub enum Command {
    /// Set the time, year, month, day, hour, minute, second
    SetTim(u8, u8, u8, u8, u8, u8), // year, month, day, hour, minute, second
    GetTim,
    GetPic(u32), // get picture with id 
}
// #[derive(Debug)]
pub enum Response {
    SetTim(u8), // response for set time. 0 for success, 1 for fail
    /// Get the time, year, month, day, hour, minute, second
    GetTim(u8, u8, u8, u8, u8, u8), // year, month, day, hour, minute, second
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
        }
    }
}
    
pub fn bytes_to_response(buf: &[u8]) -> Response {
    match buf[0] {
        0x01 => Response::SetTim(buf[1]),
        0x02 => Response::GetTim(buf[1], buf[2], buf[3], buf[4], buf[5], buf[6]),
        _ => panic!("unknown response"),
    }
}

pub fn response_to_bytes(response: Response) -> ([u8; 64], usize) {
    match response {
        Response::SetTim(status) => {
            let mut buf = [0; 64];
            buf[0] = 0x01;
            buf[1] = status;
            (buf, 2)
        }
        Response::GetTim(year, month, day, hour, minute, second) => {
            let mut buf = [0; 64];
            buf[0] = 0x02;
            buf[1] = year;
            buf[2] = month;
            buf[3] = day;
            buf[4] = hour;
            buf[5] = minute;
            buf[6] = second;
            (buf, 7)
        }
    }
}
pub fn bytes_to_command(buf: &[u8]) -> Command {
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
        _ => panic!("unknown command"),
    }
}
pub fn command_to_bytes(command: Command) -> ([u8; 64], usize) {
    match command {
        Command::SetTim(year, month, day, hour, minute, second) => {
            let mut buf = [0; 64];
            buf[0] = 0x01;
            buf[1] = year;
            buf[2] = month;
            buf[3] = day;
            buf[4] = hour;
            buf[5] = minute;
            buf[6] = second;
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
            buf[1] = (id >> 24) as u8;
            buf[2] = (id >> 16) as u8;
            buf[3] = (id >> 8) as u8;
            buf[4] = id as u8;
            (buf, 5)
        }
    }
}
