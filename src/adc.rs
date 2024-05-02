#![allow(unused)]

// Module: adc
/// Continuous conversion mode and discontinuous mode
// the continuous conversion mode and discontinuous mode are not refer the same thing.
// continuous conversion mode means the adc will keep converting the channel until the adc is disabled.
// The inverse of continuous conversion mode is single conversion mode.
// Discontinuous mode is for a sequence of channels.
// In a trig of conversion, discontinuous mode will not convert all the channels in the sequence.

use stm32_metapac::adc::Adc;

use crate::clock::{delay_ms, delay_us, set_adc_clock};
use crate::gpio::GpioPort;

pub struct AdcPort {
    port: Adc,
}

pub const ADC1: AdcPort = AdcPort {
    port: stm32_metapac::ADC1,
};

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
        delay_us(30);
        // wait for adc voltage regulator to be ready
        while !self.port.isr().read().ldordy() {}
        // while !self.port.cr().read().advregen() {}

        self.port.cfgr().modify(|v| {
            v.set_res(0b00); // 14 bit resolution
            v.set_autdly(false); // disable automatic delay
        });
        self.port.difsel().write(|v| {
            v.set_difsel(0);   // single ended mode
        });
        self.port.calfact().modify(|v| {
            v.set_capture_coef(false);
            v.set_latch_coef(false);
        });
        // start calibration
        self.port.cr().modify(|v| {
            v.set_adcal(true);
        });
        while self.port.cr().read().adcal() {} // wait for calibration finish
    }
    pub fn start_conversion_sw(&self, channel: u8) -> u32 {
        self.port.pcsel().modify(|v| {
            v.set_pcsel(channel as usize, true); // select the channel "ch" as the input
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
                v.set_smp(channel as usize, 0b111); // sete sample time to 640.5 cycles
            });
        } else {
            self.port.smpr(1).modify(|v| {
                v.set_smp((channel - 10) as usize, 0b111); // sete sample time to 640.5 cycles
            });
        }
        // enable adc
        self.port.cr().modify(|v| {
            v.set_aden(true);
        });
        self.port.cr().modify(|v| {
            v.set_adstart(true); // start conversion
        });
        delay_ms(3);
        // wait for conversion finish
        while !self.port.isr().read().eoc() {}
        // read the conversion result
        let _result = self.port.dr().read().rdata();
        // stop conversion
        self.port.cr().modify(|v| {
            v.set_adstp(true);
        });
        // disable adc
        self.port.cr().modify(|v| {
            v.set_addis(true);
        });

        return _result as u32;
    }
    pub fn get_vref_int_raw(&self) -> u32 {
        let addr = 0x0BFA_07A5 as *const u16;
        let val = unsafe { addr.read_volatile() as u32 };
        return val;
    }
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


