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

use crate::dma::*;
use crate::gpio::*;

#[allow(clippy::too_many_arguments)]
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

    }

    pub fn capture(&self, dma: DmaChannel, buf: &mut [u8]) {
        self.port.cr().modify(|v| {
            v.set_jpeg(true);
            v.set_cm(false);
            v.set_vspol(true);
            v.set_hspol(true);
            v.set_pckpol(true);
        });
        let dst_addr = buf.as_ptr() as u32;
        let len = buf.len() as u32;
        // get draddress
        let src_addr = self.port.dr().as_ptr() as u32;
        defmt::info!("src_addr: {:x}", src_addr);
        dma.start(src_addr, dst_addr, len);
        defmt::info!("dma start with src_addr: {:x}", src_addr);
        defmt::info!("dma start with dst_addr: {:x}", dst_addr);
        defmt::info!("dma start with len: {}", len);

        // start capture
        self.port.cr().modify(|v| {
            v.set_capture(true);
            v.set_enable(true);
        });
    }
    pub fn stop_capture(&self, dma: DmaChannel) {
        self.port.cr().modify(|v| v.set_capture(false));
        self.port.cr().modify(|v| v.set_enable(false));
        // claer all interrupt flags
        self.port.icr().write(|w| w.0 = 0x1f);
        dma.stop();
    }
}
