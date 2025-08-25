#![allow(unused)]

use crate::clock;
use crate::clock::delay_tick;
use crate::gpio::GpioPort;
use cortex_m::peripheral::NVIC;
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::signal::Signal;
use stm32_metapac::dcmi::Dcmi;
use stm32_metapac::RCC;

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
        RCC.ahb2enr1().modify(|v| v.set_dcmien(true));
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

    pub async fn capture(&self, dma: DmaChannel, buf: &[u8]) {
        // this function requires 160mhz clock and the deep sleep mode not allowed
        // crate::clock::run_with_160mhz_async(|| async {
            clock::hclk_request_async(clock::ClockFreqs::KernelFreq160Mhz, || async {
            crate::low_power::run_no_deep_sleep_async(|| async {
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
                // dma.start(src_addr, dst_addr, len);
                dma.start(src_addr, true, dst_addr, true, len).await;

                // start capture
                self.port.cr().modify(|v| {
                    v.set_capture(true);
                    v.set_enable(true);
                });

                self.get_picture_async().await;
                self.stop_capture(dma);
            }).await;
        }).await;
    }
    pub fn stop_capture(&self, dma: DmaChannel) {
        self.port.cr().modify(|v| v.set_capture(false));
        self.port.cr().modify(|v| v.set_enable(false));
        // claer all interrupt flags
        self.port.icr().write(|w| w.0 = 0x1f);
        dma.stop();
    }
    pub fn get_picture(&self) -> bool {
        self.port.ris().read().frame_ris()
    }
    pub async fn get_picture_async(&self) {
        // enable interrupt
        unsafe { NVIC::unmask(stm32_metapac::Interrupt::DCMI_PSSI) };
        self.port.ier().write(|w| w.set_frame_ie(true));

        // self.port.ris().read().frame_ris()
        let mut val = stm32_metapac::dcmi::regs::Ris(SIGNAL.wait().await);
        while !val.frame_ris() {
            val = stm32_metapac::dcmi::regs::Ris(SIGNAL.wait().await);
        }
    }
}

static SIGNAL: Signal<CriticalSectionRawMutex, u32> = Signal::new();

use stm32_metapac::interrupt;

#[interrupt]
fn DCMI_PSSI() {
    // clear interrupt flag
    // DCMI.icr().write(|w| w.0 = 0x1f);
    //
    let ris = stm32_metapac::DCMI.ris().read().0;

    SIGNAL.signal(ris);
    // clear itnerupt
    stm32_metapac::DCMI.icr().write(|w| w.0 = 0x1f);
    // clear pending bit
    // NVIC::unpend(Interrupt::DCMI_PSSI);
}
