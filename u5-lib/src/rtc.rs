#![allow(unused)]
use crate::clock::delay_tick;
use stm32_metapac::common::R;
use stm32_metapac::common::W;
use stm32_metapac::{pwr, rcc, rtc, rtc::Rtc, PWR, RCC, RTC};
pub struct RtcPort;

// impl RtcPort {
pub fn setup(year: u8, month: u8, day: u8, hour: u8, minute: u8, second: u8) {
    if !RCC.ahb3enr().read().pwren() {
        // enable power clock
        RCC.ahb3enr().modify(|v| {
            v.set_pwren(true); // RM0456 Rev4 Page 406
        });
    }
    // if LSE is not enabled, enable it
    if RCC.bdcr().read().lserdy() {
        RCC.bdcr().modify(|v| {
            v.set_lseon(true);
        });
        // wait for lse ready
        while !RCC.bdcr().read().lserdy() {}
    }

    RCC.apb3enr().modify(|v| { // this wil 
        v.set_rtcapben(true);
    });

    RCC.bdcr().modify(|v| {
        v.set_rtcen(true);
        v.set_rtcsel(rcc::vals::Rtcsel::LSE);
    });

    PWR.dbpcr().modify(|v| v.set_dbp(true)); // enable backup domain write

    RTC.icsr().modify(|v| v.set_bin(rtc::vals::Bin::BCD)); // set to BCD format: 4bit for each digit
    RTC.wpr().write(|w| unsafe { w.0 = 0xCA });  // write protection disable
    delay_tick(10);
    RTC.wpr().write(|w| unsafe { w.0 = 0x53 });  // write protection disable
    RTC.icsr().modify(|v| v.set_init(rtc::vals::Init::INITMODE));  // enter init mode

    // wait for init mode ready
    while !RTC.icsr().read().initf() {}

    // set prescale to 1Hz
    RTC.prer().modify(|v| {
        v.set_prediv_a(127);
        v.set_prediv_s(255);
    }); // input clock is 32768Hz. 32768/128/256 = 1Hz

    // set to 24h format
    RTC.cr()
        .modify(|v| v.set_fmt(rtc::vals::Fmt::TWENTYFOURHOUR));
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
    // exit init mode
    RTC.icsr()
        .modify(|v| v.set_init(rtc::vals::Init::FREERUNNINGMODE));

    // disable backup domain write
    PWR.dbpcr().modify(|v| v.set_dbp(false));

    // write protection enable
    RTC.wpr().write(|w| unsafe { w.0 = 0xFF });
}
pub fn enable_rtc_read() {
    // enable rtc clock
    if !RCC.ahb3enr().read().pwren() {
        // enable power clock
        RCC.ahb3enr().modify(|v| {
            v.set_pwren(true); // RM0456 Rev4 Page 406
        });
    }

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
    // let year: u32 = dr.yu() as u32 + dr.yt() as u32 * 10;
    // let month: u32 = dr.mu() as u32 + dr.mt() as u32 * 10;
    // let day: u32 = dr.du() as u32 + dr.dt() as u32 * 10;
    // year * 10000 + month * 100 + day
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
