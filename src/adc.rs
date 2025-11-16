#![allow(unused)]

use embassy_sync::channel::Channel;
use stm32_metapac::adc;
// Module: adc
/// Continuous conversion mode and discontinuous mode
// the continuous conversion mode and discontinuous mode are not refer the same thing.
// continuous conversion mode means the adc will keep converting the channel until the adc is disabled.
// The inverse of continuous conversion mode is single conversion mode.
// Discontinuous mode is for a sequence of channels.
// In a trig of conversion, discontinuous mode will not convert all the channels in the sequence.
use stm32_metapac::adc::Adc;

use crate::clock::set_adc_clock;
use crate::gpio::GpioPort;

pub struct AdcPort {
    port: Adc,
    num_chs: u8, // number of convert channels
}

pub const ADC1: AdcPort = AdcPort {
    port: stm32_metapac::ADC1,
    num_chs: 0,
};

static mut ADC1_CALIBRATION: u16 = 0; // this value will be change only once when init the adc

pub fn set_adc1_calibration(calibration: u16) {
    let mut called = false;
    if called {
        return;
    }
    called = true;
    unsafe {
        ADC1_CALIBRATION = calibration;
    }
}

pub fn get_adc1_calibration() -> u16 {
    unsafe {
        return ADC1_CALIBRATION;
    }
}

// 000: 5 ADC clock cycles
// 001: 6 ADC clock cycles
// 010: 12 ADC clock cycles
// 011: 20 ADC clock cycles
// 100: 36 ADC clock cycles
// 101: 68 ADC clock cycles
// 110: 391 ADC clock cycles
// 111: 814 ADC clock cycles
pub enum SampleTime {
    Cycles5 = 0,
    Cycles6 = 1,
    Cycles12 = 2,
    Cycles20 = 3,
    Cycles36 = 4,
    Cycles68 = 5,
    Cycles391 = 6,
    Cycles814 = 7,
}

impl SampleTime {
    pub fn to_duration(&self) -> core::time::Duration {
        // the adc clock is 16MHz
        match self {
            SampleTime::Cycles5 => core::time::Duration::from_nanos(5 * 1_000 / 16),
            SampleTime::Cycles6 => core::time::Duration::from_nanos(6 * 1_000 / 16),
            SampleTime::Cycles12 => core::time::Duration::from_nanos(12 * 1_000 / 16),
            SampleTime::Cycles20 => core::time::Duration::from_nanos(20 * 1_000 / 16),
            SampleTime::Cycles36 => core::time::Duration::from_nanos(36 * 1_000 / 16),
            SampleTime::Cycles68 => core::time::Duration::from_nanos(68 * 1_000 / 16),
            SampleTime::Cycles391 => core::time::Duration::from_nanos(391 * 1_000 / 16),
            SampleTime::Cycles814 => core::time::Duration::from_nanos(814 * 1_000 / 16),
        }
    }
}

impl AdcPort {
    pub fn init(&self) {
        set_adc_clock(); // hsi16 set as adc clock (async clock)
        unsafe {
            // adc12 common register
            let addr = 0x4202_8308 as *mut u32;
            // write value (1<<22) to the address 0x4202_8308
            *addr = (1 << 22) | (1011 << 18) | (1 << 23) | (1 << 24);
        }
        self.port.cr().modify(|v| {
            v.set_deeppwd(false);
            v.set_advregen(true); // enable adc voltage regulator
        });
        // delay_us(30);
        // wait for adc voltage regulator to be ready
        while !self.port.isr().read().ldordy() {}
        // while !self.port.cr().read().advregen() {}

        self.port.cfgr().modify(|v| {
            v.set_res(adc::vals::Res::BITS14); // 14 bit resolution
            v.set_autdly(false); // disable automatic delay
        });
        // self.port.difsel().write(|v| {
        //     v.set_difsel(0);   // single ended mode
        // });
        self.port.calfact().modify(|v| {
            v.set_capture_coef(false);
            v.set_latch_coef(false);
        });
        // start calibration
        self.port.cr().modify(|v| {
            v.set_adcal(true);
        });
        while self.port.cr().read().adcal() {} // wait for calibration finish

        // enable adc
        self.port.cr().modify(|v| {
            v.set_aden(true);
        });
    }

    pub fn add_channel(&mut self, channel: u8, sample_time: adc::vals::SampleTime) {
        // todo!(add single or diff
        self.port.pcsel().modify(|v| {
            v.set_pcsel(channel as usize, adc::vals::Pcsel::PRESELECTED); // select the channel "ch" as the input
        });
        self.port.cfgr().modify(|v| {
            v.set_cont(false); // disable continuous conversion mode
        });
        self.num_chs += 1;
        self.port.sqr1().modify(|v| {
            v.set_l(self.num_chs - 1);
        });
        // sqrt1 from 1 to 4
        if self.num_chs <= 4 {
            self.port.sqr1().modify(|v| {
                v.set_sq((self.num_chs - 1) as _, channel);
            });
        } else if self.num_chs <= 9 {
            self.port.sqr2().modify(|v| {
                v.set_sq((self.num_chs - 5) as _, channel);
            });
        } else {
            todo!();
        }
        self.port.smpr((channel / 10) as _).modify(|v| {
            v.set_smp(channel as usize, sample_time); // sete sample time to 640.5 cycles
        });
    }
    pub fn start_conversion_sw(&self, channel: u8) -> u32 {
        self.port.pcsel().modify(|v| {
            v.set_pcsel(channel as usize, adc::vals::Pcsel::PRESELECTED); // select the channel "ch" as the input
        });
        self.port.cfgr().modify(|v| {
            v.set_cont(false); // disable continuous conversion mode
        });
        // start conversion with software trigger
        self.port.sqr1().modify(|v| {
            v.set_l(0b0000); // 1 conversion
        });
        self.port.sqr1().modify(|v| {
            // v.set_sq1(channel); // channel "ch"
            v.set_sq(0, channel);
        });
        if (channel < 10) {
            self.port.smpr(0).modify(|v| {
                v.set_smp(channel as usize, adc::vals::SampleTime::CYCLES160_5);
                // sete sample time to 640.5 cycles
            });
        } else {
            self.port.smpr(1).modify(|v| {
                v.set_smp((channel - 10) as usize, adc::vals::SampleTime::CYCLES160_5);
            });
        }
        // delay_ms(5);
        self.port.cr().modify(|v| {
            v.set_adstart(true); // start conversion
        });
        // delay_ms(3);
        // wait for conversion finish
        while !self.port.isr().read().eoc() {}
        // read the conversion result
        let _result = self.port.dr().read().rdata();
        // stop conversion
        self.port.cr().modify(|v| {
            v.set_adstp(adc::vals::Adstp::STOP);
        });
        // disable adc
        // self.port.cr().modify(|v| {
        //     v.set_addis(true);
        // });

        return _result as u32;
    }
    pub fn get_vref_int_raw(&self) -> u32 {
        let addr = 0x0BFA_07A5 as *const u8;
        let val = unsafe { addr.read_volatile() as u8 } as u32;
        let addr = 0x0BFA_07A6 as *const u8;
        // let val = val as u16 | (unsafe { addr.read_volatile() as u16 } << 8);
        let val2 = unsafe { addr.read_volatile() as u8 } as u32;
        return val2 << 8 | val;
        // return val << 8 | val2;
    }
    // pub fn get_calibration_factor(&self) -> f64 {

    // }
    // if use tim1 trgo as the trigger source, the trigger source is 9.
    // pub fn start_conversion_ext(&self, pin: GpioPort, channel: u8, extsel: u8,
    //                             &mut results: &mut [u16]) { self.port.cfgr().modify(|v| {
    //         v.set_cont(false); // disable continuous conversion mode
    //     });
    //     // start conversion with external trigger
    //     self.port.sqr1().modify(|v| {
    //         v.set_l(0b0000); // 1 conversion
    //     });
    //     self.port.sqr1().modify(|v| {
    //         v.set_sq1(channel); // channel "ch"
    //     });
    //     self.port.smpr1().modify(|v| {
    //         v.set_smp(channel, 0b111); // sete sample time to 640.5 cycles
    //     });
    //     // external trigger
    //     self.port.cfgr().modify(|v| {
    //         v.set_exten(0b01); // enable external trigger
    //         // v.set_extsel(0b0000); // enable external trigger
    //         v.set_extsel(extsel); // enable external trigger
    //     });
    //     // enable adc
    //     self.port.cr().modify(|v| {
    //         v.set_aden(true);
    //     });
    //     self.port.cr().modify(|v| {
    //         v.set_adstart(true); // start conversion
    //     });
    //     for i in 0..results.len() {
    //         // wait for conversion finish
    //         while !self.port.isr().read().eoc() {}
    //         // read the conversion result
    //         results[i] = self.port.dr().read().data();
    //     }
    //     // stop conversion
    //     self.port.cr().modify(|v| {
    //         v.set_adstp(true);
    //     });
    //     while self.port.cr().read().adstp() {}
    //     self.port.cr().modify(|v| {
    //         v.set_aden(false);
    //     });
    // }
}
