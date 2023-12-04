#![allow(unused)]
use crate::clock::delay_tick;
use crate::gpio::GpioPort;
use stm32_metapac::dcmi::Dcmi;
pub struct DcmiPort {
    port: Dcmi,
}
pub const DCMI: DcmiPort = DcmiPort {
    port: stm32_metapac::DCMI,
};

use crate::gpio::*;

#[derive(Copy, Clone, Debug)]
pub enum I2cError {
    NoError,
    ArbitrationLost,
    BusError,
    Nack,
    OverrunUnderrun,
    PecError,
    Timeout,
    Alert,
}
impl DcmiPort {
    pub fn init(
        &self,
        d0: GpioPort,
        d1: GpioPort,
        d2: GpioPort,
        d3: GpioPort,
        d4: GpioPort,
        d5: GpioPort,
        d6: GpioPort,
        d7: GpioPort,
        hs: GpioPort,
        vs: GpioPort,
        pclk: GpioPort,
    ) {
        // setup gpio ports
        d0.setup();
        d1.setup();
        d2.setup();
        d3.setup();
        d4.setup();
        d5.setup();
        d6.setup();
        d7.setup();
        hs.setup();
        vs.setup();
        pclk.setup();
        self.port.cr().modify(|v| {
            v.set_jpeg(true);
            v.set_cm(false);
            v.set_vspol(true);
            v.set_hspol(true);
            v.set_pckpol(true);
        });
    }

    // pub fn capture(dma: Option<_>, buf: &[u8]) {}
}
