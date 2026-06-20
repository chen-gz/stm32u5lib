// #![feature(panic_info_message)]

#[cfg(all(target_arch = "arm", target_os = "none", feature = "utils"))]
use core::panic::PanicInfo;

use core::time::Duration;

/// Profile the execution time of a synchronous closure.
///
/// ### Technical Details & Limitations:
/// - **Hardware:** Uses the ARM Cortex-M DWT (Data Watchpoint and Trace) cycle counter.
/// - **Rollover:** The 32-bit cycle counter will roll over. At 160MHz, this occurs approximately every **26.8 seconds**.
///   Measurements exceeding this duration will be inaccurate (modulo 26.8s).
/// - **Interrupts:** Time spent in interrupt handlers triggered during the execution is **included** in the profile (Wall Clock Time).
/// - **Overhead:** The function call and cycle counter reads add a small overhead (typically a few dozen cycles).
/// - **Sleep:** The DWT counter may stop during low-power sleep modes (WFI/WFE) depending on the MCU debug configuration.
/// - **Precision:** Precision is 1 CPU cycle (~6.25ns at 160MHz). Time values in `us` and `ms` are truncated.
#[cfg(all(target_arch = "arm", target_os = "none"))]
pub fn profile<F, R>(name: &str, f: F) -> R
where
    F: FnOnce() -> R,
{
    use crate::clock;
    let start = unsafe { cortex_m::Peripherals::steal().DWT.cyccnt.read() };
    let res = f();
    let end = unsafe { cortex_m::Peripherals::steal().DWT.cyccnt.read() };
    let cycles = end.wrapping_sub(start);
    let hclk = clock::get_hclk();
    let us = (cycles as u64 * 1_000_000) / hclk as u64;
    let ms = us / 1000;
    info!("profile: {} took {} cycles ({} us, {} ms)", name, cycles, us as u32, ms as u32);
    res
}

#[cfg(not(all(target_arch = "arm", target_os = "none")))]
pub fn profile<F, R>(name: &str, f: F) -> R
where
    F: FnOnce() -> R,
{
    let start = std::time::Instant::now();
    let res = f();
    let duration = start.elapsed();
    info!("profile: {} took {:?}", name, duration);
    res
}

/// Profile the execution time of an asynchronous future.
///
/// ### Technical Details & Limitations:
/// - **Hardware:** Uses the ARM Cortex-M DWT cycle counter.
/// - **Rollover:** The 32-bit cycle counter will roll over. At 160MHz, this occurs approximately every **26.8 seconds**.
/// - **Interrupts:** Time spent in interrupt handlers during the execution is **included**.
/// - **Executor Overhead:** Includes the time spent by the executor switching tasks if other tasks are polled during the await.
/// - **Sleep:** The DWT counter may stop during low-power sleep modes.
/// - **Precision:** Precision is 1 CPU cycle. Time values in `us` and `ms` are truncated.
#[cfg(all(target_arch = "arm", target_os = "none"))]
pub async fn profile_async<F, R>(name: &str, f: F) -> R
where
    F: core::future::Future<Output = R>,
{
    use crate::clock;
    let start = unsafe { cortex_m::Peripherals::steal().DWT.cyccnt.read() };
    let res = f.await;
    let end = unsafe { cortex_m::Peripherals::steal().DWT.cyccnt.read() };
    let cycles = end.wrapping_sub(start);
    let hclk = clock::get_hclk();
    let us = (cycles as u64 * 1_000_000) / hclk as u64;
    let ms = us / 1000;
    info!("profile_async: {} took {} cycles ({} us, {} ms)", name, cycles, us as u32, ms as u32);
    res
}

#[cfg(not(all(target_arch = "arm", target_os = "none")))]
pub async fn profile_async<F, R>(name: &str, f: F) -> R
where
    F: core::future::Future<Output = R>,
{
    let start = std::time::Instant::now();
    let res = f.await;
    let duration = start.elapsed();
    info!("profile_async: {} took {:?}", name, duration);
    res
}

#[macro_export]
macro_rules! profile {
    ($name:expr, $code:expr) => {
        $crate::utils::profile($name, || $code)
    };
    ($code:expr) => {
        $crate::utils::profile(core::stringify!($code), || $code)
    };
}

#[macro_export]
macro_rules! profile_async {
    ($name:expr, $code:expr) => {
        $crate::utils::profile_async($name, $code)
    };
    ($code:expr) => {
        $crate::utils::profile_async(core::stringify!($code), $code)
    };
}

#[cfg(all(target_arch = "arm", target_os = "none", feature = "utils"))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    info!("panic");
    error!(
        "Location file name: {:?}, line: {:?}, col: {:?}",
        _info.location().unwrap().file(),
        _info.location().unwrap().line(),
        _info.location().unwrap().column()
    );
    info!("panic endless loop");
    // if let Some(args) = _info.message() {
    //     error!("Panic message: {:?}", args.as_str());
    // }
    loop {}
}

const SECONDS_IN_A_DAY: u64 = 86400;

fn is_leap_year(year: u8) -> bool {
    let abs_year = 2000 + year as u16;
    abs_year % 4 == 0 && (abs_year % 100 != 0 || abs_year % 400 == 0)
}

fn days_in_month(year: u8, month: u8) -> u8 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => {
            if is_leap_year(year) {
                29
            } else {
                28
            }
        }
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
pub fn duration_since_2000(year: u8, month: u8, day: u8, hour: u8, min: u8, sec: u8) -> Duration {
    Duration::from_secs(seconds_since_2000(year, month, day, hour, min, sec))
}
pub fn time_date_from_duration_since_2000(duration: Duration) -> (u8, u8, u8, u8, u8, u8) {
    let mut total_seconds = duration.as_secs();
    let mut year = 0;
    let mut month = 1;
    let mut day = 1;
    let hour;
    let min;
    let sec;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_leap_year() {
        // year 0 is 2000, which is a leap year
        assert!(is_leap_year(0));
        // year 4 is 2004, which is a leap year
        assert!(is_leap_year(4));
        // year 1 is 2001, not a leap year
        assert!(!is_leap_year(1));
        // year 100 is 2100, which is not a leap year
        assert!(!is_leap_year(100));
    }

    #[test]
    fn test_days_in_month() {
        // Leap year Feb
        assert_eq!(days_in_month(0, 2), 29);
        // Non-leap year Feb
        assert_eq!(days_in_month(1, 2), 28);
        // Wildcard / invalid month
        assert_eq!(days_in_month(1, 0), 0);
        assert_eq!(days_in_month(1, 13), 0);
    }

    #[test]
    fn test_date_conversions() {
        let (y, m, d, h, min, s) = (24, 6, 19, 14, 8, 44);
        let duration = duration_since_2000(y, m, d, h, min, s);
        let roundtrip = time_date_from_duration_since_2000(duration);
        assert_eq!(roundtrip, (y, m, d, h, min, s));

        // Test non-leap year Feb date conversion roundtrip
        let (y2, m2, d2, h2, min2, s2) = (1, 2, 28, 23, 59, 59);
        let duration2 = duration_since_2000(y2, m2, d2, h2, min2, s2);
        let roundtrip2 = time_date_from_duration_since_2000(duration2);
        assert_eq!(roundtrip2, (y2, m2, d2, h2, min2, s2));

        // Test leap year Feb date conversion roundtrip
        let (y3, m3, d3, h3, min3, s3) = (0, 2, 29, 12, 0, 0);
        let duration3 = duration_since_2000(y3, m3, d3, h3, min3, s3);
        let roundtrip3 = time_date_from_duration_since_2000(duration3);
        assert_eq!(roundtrip3, (y3, m3, d3, h3, min3, s3));
    }

    #[test]
    fn test_host_profiling() {
        let res = profile!("host_test_delay", {
            std::thread::sleep(Duration::from_millis(10));
            42
        });
        assert_eq!(res, 42);

        // Also test the single-parameter macro version
        let res2 = profile!({
            std::thread::sleep(Duration::from_millis(10));
            43
        });
        assert_eq!(res2, 43);
    }

    #[test]
    fn test_host_profiling_async() {
        use futures::executor::block_on;
        let res = block_on(profile_async!("host_test_delay_async", async {
            std::thread::sleep(Duration::from_millis(10));
            42
        }));
        assert_eq!(res, 42);

        // Also test the single-parameter macro version
        let res2 = block_on(profile_async!(async {
            std::thread::sleep(Duration::from_millis(10));
            43
        }));
        assert_eq!(res2, 43);
    }
}


