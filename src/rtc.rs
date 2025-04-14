#![allow(unused)]

use crate::clock;
use crate::clock::{delay_ms, delay_tick};
use crate::gpio;
use core::future::poll_fn;
use cortex_m::delay;
// use cortex_m::interrupt;
use cortex_m::peripheral::scb;
use cortex_m::peripheral::NVIC;
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::blocking_mutex::raw::RawMutex;
use embassy_sync::signal::Signal;
use stm32_metapac::{pwr, rcc, rtc, rtc::Rtc, PWR, RCC, RTC};
pub struct RtcPort;

// the rtc often support by LSE (32.768kHz) or LSI (32kHz)
pub use rcc::vals::Rtcsel as RtcSource;
// impl RtcPort {
pub fn setup(
    year: u8,
    month: u8,
    day: u8,
    hour: u8,
    minute: u8,
    second: u8,
    period_wakup_s: u16,
    rtc_source: rcc::vals::Rtcsel,
) {
    defmt::info!("rtc setup with year: {}, month: {}, day: {}, hour: {}, minute: {}, second: {}, period_wakup_s: {}", year, month, day, hour, minute, second, period_wakup_s);
    match rtc_source {
        rcc::vals::Rtcsel::LSE => {
            defmt::info!("rtc source: LSE");
        }
        rcc::vals::Rtcsel::LSI => {
            defmt::info!("rtc source: LSI");
        }
        _ => {
            defmt::panic!("rtc source no support: ???");
        }
    }

    //  rest backup domain
    RCC.bdcr().write(|v| v.set_bdrst(true));
    delay_tick(10); // check the minimum time,
                    // disable BDRST
    RCC.bdcr().modify(|v| v.set_bdrst(false));
    // record the current rtc mask and pend status
    let rtc_it_enabled = unsafe { NVIC::is_enabled(stm32_metapac::Interrupt::RTC) };

    // unsafe {
    //     NVIC::mask(stm32_metapac::Interrupt::RTC);
    //     NVIC::unpend(stm32_metapac::Interrupt::RTC);
    // };

    // enable power clock
    RCC.ahb3enr().modify(|v| {
        v.set_pwren(true); // RM0456 Rev4 Page 406
    });

    RCC.apb3enr().modify(|v| {
        v.set_rtcapben(true);
    });

    PWR.dbpcr().modify(|v| v.set_dbp(true)); // enable backup domain write
                                             // fn with backup domain write
    fn_with_back_domain_write(|| {
        {
            let rs = RCC.bdcr().read().rtcsel(); // get rtc source
            if rs != rtc_source {
                // update rtc source - update rtc source requires reset
                RCC.bdcr().write(|v| v.set_bdrst(true)); // reset backup domain
                clock::delay_tick(100); // wait for reset TODO: get the minimum time
                                        // disable BDRST
                RCC.bdcr().modify(|v| v.set_bdrst(false)); // reset finished
                match rtc_source {
                    rcc::vals::Rtcsel::LSE => {
                        RCC.bdcr().modify(|v| {
                            v.set_lseon(true);
                        });
                        // wait for lse ready
                        while !RCC.bdcr().read().lserdy() {}
                    }
                    rcc::vals::Rtcsel::LSI => {
                        RCC.bdcr().modify(|v| {
                            v.set_lsion(true);
                        });
                        while !RCC.bdcr().read().lsirdy() {}
                    }
                    _ => {}
                }
                // set rtc source
                RCC.bdcr().modify(|v| {
                    v.set_rtcsel(rtc_source);
                });
            }
        }

        RTC.icsr().modify(|v| v.set_bin(rtc::vals::Bin::BCD)); // set to BCD format: 4bit for each digit
        fn_with_write_protection(|| {
            // RTC.icsr().modify(|v| v.set_init(rtc::vals::Init::INITMODE)); // enter init mode
            RTC.icsr().modify(|v| v.set_init(true)); // enter init mode
            while !RTC.icsr().read().initf() {} // wait for init mode ready

            // set prescale to 1Hz
            match rtc_source {
                rcc::vals::Rtcsel::LSE => {
                    RTC.prer().modify(|v| {
                        v.set_prediv_a(127);
                        v.set_prediv_s(255);
                    }); // input clock is 32768Hz. 32768/128/256 = 1Hz
                }
                rcc::vals::Rtcsel::LSI => {
                    RTC.prer().modify(|v| {
                        v.set_prediv_a(99);
                        v.set_prediv_s(319);
                    }); // input clock is 32kHz. 32000/100/320 = 1Hz
                }
                _ => {}
            }

            // set to 24h format
            RTC.cr()
                .modify(|v| v.set_fmt(rtc::vals::Fmt::TWENTY_FOUR_HOUR));
            RTC.tr().modify(|v| {
                v.set_su(second % 10);
                v.set_st(second / 10);
                v.set_mnu(minute % 10);
                v.set_mnt(minute / 10);
                v.set_hu(hour % 10);
                v.set_ht(hour / 10);
            });
            RTC.dr().modify(|v| {
                v.set_yu(year % 10);
                v.set_yt(year / 10);
                v.set_mu(month % 10);
                v.set_mt((month / 10) == 1); // check
                v.set_du(day % 10);
                v.set_dt(day / 10);
            });

            // clear bypass shadow register
            RTC.cr().modify(|v| v.set_bypshad(false));
            if period_wakup_s != 0 {
                // set up periodic wakeup
                RTC.cr().modify(|v| {
                    v.set_wute(false);
                    v.set_wutie(false);
                }); // disable wakeup timer before setting period
                while !RTC.icsr().read().wutwf() {} // wait for wakeup timer write flag

                match rtc_source {
                    rcc::vals::Rtcsel::LSE => {
                        RTC.wutr()
                            .write(|w| unsafe { w.set_wut(period_wakup_s * 2048) });
                        // 32768Hz/16 = 2048Hz
                    }
                    rcc::vals::Rtcsel::LSI => {
                        RTC.wutr()
                            .write(|w| unsafe { w.set_wut(period_wakup_s * 2000) });
                        // 32000Hz/16 = 2000Hz
                    }
                    _ => {}
                }

                RTC.cr().modify(|v| {
                    v.set_wucksel(rtc::vals::Wucksel::DIV16); // clock is 32768/16 = 2048Hz
                    v.set_wute(true);
                    v.set_wutie(true);
                });
            }
            // RTC.icsr().modify(|v| v.set_init(rtc::vals::Init::FREERUNNINGMODE)); // exit init mode
            RTC.icsr().modify(|v| v.set_init(false)); // exit init mode

            RCC.bdcr().modify(|v| {
                v.set_rtcen(true);
            });

            // PWR.dbpcr().modify(|v| v.set_dbp(false)); // disable backup domain write
        });
    });
    // RTC.wpr().write(|w| unsafe { w.0 = 0xFF }); // write protection enable

    if rtc_it_enabled {
        unsafe { NVIC::unmask(stm32_metapac::Interrupt::RTC) };
    }
}
pub fn enable_rtc_read() {
    // enable rtc clock
    RCC.ahb3enr().modify(|v| {
        v.set_pwren(true); // RM0456 Rev4 Page 406
    });
    RCC.srdamr().modify(|v| {
        v.set_rtcapbamen(true);
    });
    RCC.ahb3smenr().modify(|v| {
        v.set_pwrsmen(true);
    });
    if !RCC.ahb3enr().read().pwren() {
        // enable power clock
        RCC.ahb3enr().modify(|v| {
            v.set_pwren(true); // RM0456 Rev4 Page 406
        });
    }
    RCC.apb3smenr().modify(|v| {
        v.set_rtcapbsmen(true);
    });

    // todo!("check this function; seems only rtcapben is good enough");
    RCC.apb3enr().modify(|v| {
        v.set_rtcapben(true);
    });
    // enable rtc battery charge
}

fn get_year() -> u8 {
    let dr = RTC.dr().read();
    dr.yu() + dr.yt() * 10
}
fn get_month() -> u8 {
    let dr = RTC.dr().read();
    dr.mu() + dr.mt() as u8 * 10
}
fn get_day() -> u8 {
    let dr = RTC.dr().read();
    dr.du() + dr.dt() * 10
}
fn get_hour() -> u8 {
    let tr = RTC.tr().read();
    tr.hu() + tr.ht() * 10
}
fn get_minute() -> u8 {
    let tr = RTC.tr().read();
    tr.mnu() + tr.mnt() * 10
}
fn get_second() -> u8 {
    let tr = RTC.tr().read();
    tr.su() + tr.st() * 10
}
fn get_weekday() -> u8 {
    RTC.dr().read().wdu()
}
pub fn get_date() -> (u8, u8, u8) {
    // yymmdd
    let dr = RTC.dr().read();
    (
        dr.yu() + dr.yt() * 10,
        dr.mu() + dr.mt() as u8 * 10,
        dr.du() + dr.dt() * 10,
    )
}
pub fn get_time() -> (u8, u8, u8) {
    // hhmmss
    let tr = RTC.tr().read();
    (
        tr.hu() + tr.ht() * 10,
        tr.mnu() + tr.mnt() * 10,
        tr.su() + tr.st() * 10,
    )
}

use core::time::Duration;
// todo!("remove");
pub async fn rtc_interrupt() {
    unsafe { NVIC::unmask(stm32_metapac::Interrupt::RTC) };
    RCC.apb3smenr().modify(|v| {
        v.set_rtcapbsmen(true);
    });

    RCC.srdamr().modify(|v| {
        v.set_rtcapbamen(true);
    });

    RCC.apb3enr().modify(|v| {
        v.set_rtcapben(true);
    });
    RTC_SIGNAL.wait().await;
    // LED_GREEN.toggle();
}

use stm32_metapac::interrupt;
use crate::utils;
use embassy_sync::waitqueue::AtomicWaker;
fn rtc_time_to_duration() -> Duration {
    // the default duration start from 2000 years
    let date = get_date(); // year, month, day
    let time = get_time(); // hour, minute, second
                           // calculate duration from 2000
    let duration = utils::seconds_since_2000(date.0, date.1, date.2, time.0, time.1, time.2);
    // defmt::info!("rtc_time_to_duration: {:?}s", duration);
    Duration::from_secs(duration)
}

const NEW_AW: AtomicWaker = AtomicWaker::new();

struct RtcWakers {
    waker: AtomicWaker,
    wakeup_time: Duration,
    pending: bool,
}
impl Default for RtcWakers {
    fn default() -> Self {
        RtcWakers {
            waker: AtomicWaker::new(),
            wakeup_time: Default::default(),
            pending: false,
        }
    }
}
// since stm32u5 only one core. Unsafe access will not cause issue.
const NUM_WAKER: usize = 32;
static mut RTC_WAKER: [Option<RtcWakers>; NUM_WAKER] = [const { None }; NUM_WAKER];

/// becare when using this function. If delay is less(equal) than 1 second. This should not be used.
pub async fn rtc_delay(duration: Duration) {
    if duration <= Duration::from_secs(1) {
        panic!("Current not allow delay duration less (equal) than 1 second");
    }
    let waker_time = rtc_time_to_duration() + duration;
    // find an empty slot and use it
    let empty = (0..NUM_WAKER).position(|i| unsafe { &RTC_WAKER[i] }.is_none());
    let empty = empty.unwrap();
    unsafe {
        RTC_WAKER[empty] = Some(RtcWakers {
            waker: NEW_AW,
            wakeup_time: waker_time,
            pending: true,
        });
    }
    // update alarm
    // todo!("update alarm");
    // defmt::info!("update alarm");
    update_alarm();

    poll_fn(move |ctx| {
        unsafe {
            RTC_WAKER[empty]
                .as_ref()
                .unwrap()
                .waker
                .register(ctx.waker());
            // if waker.unwrap().wakeup_time
            if RTC_WAKER[empty].as_ref().unwrap().wakeup_time <= rtc_time_to_duration() {
                // release the slot
                RTC_WAKER[empty] = None;
                return core::task::Poll::Ready(());
            } else {
                return core::task::Poll::Pending;
            }
        }
    })
    .await;
}

fn update_alarm() {
    // find the minimum waker_time from RTC_WAKER
    let mut min_waker_time = None;
    for waker in unsafe { &RTC_WAKER } {
        if let Some(waker) = waker {
            if waker.pending {
                if waker.wakeup_time <= rtc_time_to_duration() {
                    waker.waker.wake();
                } else {
                    if let Some(min_time) = min_waker_time {
                        if waker.wakeup_time < min_time {
                            min_waker_time = Some(waker.wakeup_time);
                        }
                    } else {
                        min_waker_time = Some(waker.wakeup_time);
                    }
                }
            }
        }
    }
    // defmt::info!("min_time: {:?}", min_waker_time);
    if let Some(min_time) = min_waker_time {
        let alarm_time = utils::time_date_from_duration_since_2000(min_time);
        set_alarm(alarm_time.2, alarm_time.3, alarm_time.4, alarm_time.5);
    }
}
pub fn set_alarm(day: u8, hour: u8, min: u8, sec: u8) {
    unsafe { NVIC::unmask(stm32_metapac::Interrupt::RTC) };
    fn_with_back_domain_write(|| {
        RTC.cr().modify(|v| {
            v.set_alre(0, false); // disable alarm a
        });
        RTC.alrmr(0).modify(|v| {
            v.set_du(day % 10);
            v.set_dt(day / 10);
            v.set_hu(hour % 10);
            v.set_ht(hour / 10);
            v.set_mnu(min % 10);
            v.set_mnt(min / 10);
            v.set_st(sec / 10);
            v.set_su(sec % 10);
        });
        RTC.cr().modify(|v| {
            v.set_alre(0, true);
            v.set_alrie(0, true);
        });
    });
}
pub fn fn_with_back_domain_write(f: impl FnOnce()) {
    PWR.dbpcr().modify(|v| v.set_dbp(true)); // enable backup domain write
    f();
    PWR.dbpcr().modify(|v| v.set_dbp(false)); // disable backup domain write
}

pub fn fn_with_write_protection(f: impl FnOnce()) {
    RTC.wpr().write(|w| unsafe { w.0 = 0xCA }); // write protection disable
    delay_tick(10);
    RTC.wpr().write(|w| unsafe { w.0 = 0x53 }); // write protection disable
    f();
    RTC.wpr().write(|w| unsafe { w.0 = 0xFF }); // write protection enable
}

// todo!("remove");
static RTC_SIGNAL: Signal<CriticalSectionRawMutex, u32> = Signal::new();

#[interrupt]
fn RTC() {
    PWR.dbpcr().modify(|v| v.set_dbp(true)); // enable backup domain write
    defmt::info!("rtc interrupt with flag: {}", RTC.sr().read().0);
    let stat: u32 = RTC.sr().read().0;
    RTC_SIGNAL.signal(stat);
    // clear interrupt flag
    RTC.scr().write(|w| w.0 = 0x7F);
    NVIC::unpend(stm32_metapac::Interrupt::RTC);
    // disable backup domain write
    PWR.dbpcr().modify(|v| v.set_dbp(false));
    unsafe {
        for i in 0..NUM_WAKER {
            if let Some(w) = &RTC_WAKER[i] {
                // if w.wakeup_time <= rtc_time_to_duration() {
                    w.waker.wake();
                    // defmt::info!("waker up  a waker");
                // }
            }
        }
    }
    update_alarm();
}
