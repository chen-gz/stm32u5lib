#![allow(unused)]
use crate::clock::delay_tick;
use stm32_metapac::common::R;
use stm32_metapac::common::W;
use stm32_metapac::{pwr, rtc, rtc::Rtc, PWR, RCC, RTC};
pub struct RtcPort;

impl RtcPort {
    fn setup(year: u8, month: u8, day: u8, hour: u8, minute: u8, second: u8) {
        // enable rtc clock and backup domain
        RCC.bdcr().modify(|v| {
            v.set_lseon(true);
            v.set_rtcsel(rcc::vals::Rtcsel::LSE);
            v.set_rtcen(true);
            v.set_bdrst(true);
        });
        RCC.ahb3enr().modify(|v| {
            v.set_pwren(true);
        });
        RCC.apb3enr().modify(|v| {
            v.set_rtcapben(true);
        });
        // enable backup domain write
        PWR.dbpcr().modify(|v| v.set_dbp(true));
        RCC.bdcr().modify(|v| {
            v.set_rtcen(true);
            v.set_lseon(true);
        });
        // wait for lse ready
        while !RCC.bdcr().read().lserdy() {}

        // set rtc clock to lse
        RCC.bdcr()
            .modify(|v| v.set_rtcsel(stm32_metapac::rcc::vals::Rtcsel::LSE));
        // set to BCD format
        RTC.icsr().modify(|v| v.set_bin(rtc::vals::Bin::BCD));
        // write protection disable
        RTC.wpr().write(|w| unsafe { w.0 = 0xCA });
        RTC.wpr().write(|w| unsafe { w.0 = 0x53 });
        // enter init mode
        RTC.icsr().modify(|v| v.set_init(rtc::vals::Init::INITMODE));

        // wait for init mode ready
        while !RTC.icsr().read().initf() {}
        // set prescale to 1Hz

        // set async prescale to 127
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
    fn get_year() -> u8 {
        let dr = RTC.dr().read();
        dr.yu() + dr.yt() * 10
    }
    fn get_month() -> u8 {
        let dr = RTC.dr().read();
        dr.mu() + dr.mt() * 10
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
    fn get_date() -> u32 {
        // yymmdd
        let dr = RTC.dr().read();
        let year: u32 = dr.yu() as u32 + dr.yt() as u32 * 10;
        let month: u32 = dr.mu() as u32 + dr.mt() as u32 * 10;
        let day: u32 = dr.du() as u32 + dr.dt() as u32 * 10;
        year * 10000 + month * 100 + day
    }
    fn get_time() -> u32 {
        // hhmmss
        let tr = RTC.tr().read();
        let hour: u32 = tr.hu() as u32 + tr.ht() as u32 * 10;
        let minute: u32 = tr.mnu() as u32 + tr.mnt() as u32 * 10;
        let second: u32 = tr.su() as u32 + tr.st() as u32 * 10;
        hour * 10000 + minute * 100 + second
    }
}
