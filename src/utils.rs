// #![feature(panic_info_message)]

use core::panic::PanicInfo;
use core::time::Duration;

#[cfg(feature = "utils")]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    defmt::info!("panic");
    defmt::error!(
        "Location file name: {:?}, line: {:?}, col: {:?}",
        _info.location().unwrap().file(),
        _info.location().unwrap().line(),
        _info.location().unwrap().column()
    );
    defmt::info!("panic endless loop");
    // if let Some(args) = _info.message() {
    //     defmt::error!("Panic message: {:?}", args.as_str());
    // }
    loop {}
}

const SECONDS_IN_A_DAY: u64 = 86400;

fn is_leap_year(year: u8) -> bool {
    (year % 4 == 0 && year % 100 != 0)
}

fn days_in_month(year: u8, month: u8) -> u8 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => if is_leap_year(year) { 29 } else { 28 },
        _ => 0,
    }
}

//  the year only last two digits
pub fn seconds_since_2000(year: u8, month: u8, day: u8, hour: u8, min: u8, sec: u8) -> u64 {
    let mut total_days = 0;
    for y in 0..year {
        total_days += if is_leap_year(y) { 366 } else { 365 };
    }
    for m in 1..month {
        total_days += days_in_month(year, m) as u64;
    }
    // defmt::info!("year: {}, month: {}, day: {}", year, month, day);
    // defmt::info!("total_days: {}", total_days);
    total_days += day as u64 - 1;
    let total_seconds = total_days * SECONDS_IN_A_DAY + (hour as u64 * 3600) + (min as u64 * 60) + sec as u64;
    total_seconds
}
pub fn duration_since_2000 (year: u8, month: u8, day: u8, hour: u8, min: u8, sec: u8) -> Duration {
    Duration::from_secs(seconds_since_2000(year, month, day, hour, min, sec))
}
pub fn time_date_from_duration_since_2000(duration: Duration) -> (u8, u8, u8, u8, u8, u8) {
    let mut total_seconds = duration.as_secs();
    let mut year = 0;
    let mut month = 1;
    let mut day = 1;
    let mut hour = 0;
    let mut min = 0;
    let mut sec = 0;

    // Calculate year
    while total_seconds >= SECONDS_IN_A_DAY * if is_leap_year(year) { 366 } else { 365 } {
        total_seconds -= SECONDS_IN_A_DAY * if is_leap_year(year) { 366 } else { 365 };
        year += 1;
    }

    // Calculate month
    while total_seconds >= SECONDS_IN_A_DAY * days_in_month(year, month) as u64 {
        total_seconds -= SECONDS_IN_A_DAY * days_in_month(year, month) as u64;
        month += 1;
    }

    // Calculate day
    day += (total_seconds / SECONDS_IN_A_DAY) as u8;
    total_seconds %= SECONDS_IN_A_DAY;

    // Calculate hour
    hour = (total_seconds / 3600) as u8;
    total_seconds %= 3600;

    // Calculate minute
    min = (total_seconds / 60) as u8;
    total_seconds %= 60;

    // Remaining seconds
    sec = total_seconds as u8;

    (year, month, day, hour, min, sec)
}
