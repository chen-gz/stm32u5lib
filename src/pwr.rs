///////////////////////////////////////////////////////////
/// USB power monitor
use cortex_m::peripheral::NVIC;
use defmt::info;
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::signal::Signal;

use crate::clock::{set_clock, ClockFreqs, CLOCK_REQUESTS};
use crate::low_power;

#[embassy_executor::task]
pub async fn vddusb_monitor_up() {
    // exti line 19 for uvm
    unsafe {
        NVIC::unmask(stm32_metapac::Interrupt::PVD_PVM);
    }
    loop {
        static mut USB_POWER_UP: bool = false;
        stm32_metapac::PWR.svmcr().modify(|w| {
            w.set_uvmen(true);
        });
        stm32_metapac::EXTI.rtsr(0).modify(|v| v.set_line(19, true));
        stm32_metapac::EXTI.ftsr(0).modify(|v| v.set_line(19, true));
        stm32_metapac::EXTI.imr(0).modify(|v| v.set_line(19, true));
        stm32_metapac::EXTI.emr(0).modify(|v| v.set_line(19, true));
        // get vddusb status
        let vddusb = stm32_metapac::PWR.svmsr().read().vddusbrdy();
        if vddusb != unsafe { USB_POWER_UP } {
            if vddusb {
                info!("USB power up, call pwoer_up_init");
                unsafe {
                    USB_POWER_UP = true;
                    CLOCK_REQUESTS[ClockFreqs::KernelFreq160Mhz.to_idx()] += 1; // request 160Mhz
                    set_clock();
                    low_power::no_deep_sleep_request();
                }
                // crate::usb_otg_hs::mod_new::power_up_init();
                // crate::usb_otg_hs::po
                power_up_init();
                PD15.set_high();
                unsafe {
                    BUS_WAKER_PWR.wake();
                }
            } else {
                unsafe {
                    USB_POWER_UP = false;
                    CLOCK_REQUESTS[ClockFreqs::KernelFreq160Mhz.to_idx()] -= 1;
                    low_power::no_deep_sleep_release();
                    PD15.set_low();
                }
            }
        }
        PVM_SIGNAL.wait().await;
    }
}

static PVM_SIGNAL: Signal<CriticalSectionRawMutex, u32> = Signal::new();


use stm32_metapac::interrupt;
use crate::gpio::PD15;
use crate::usb_otg_hs::global_states::BUS_WAKER_PWR;
use crate::usb_otg_hs::power::power_up_init;

#[interrupt]
fn PVD_PVM() {
    PVM_SIGNAL.signal(1);
    stm32_metapac::EXTI.fpr(0).write(|v| v.set_line(19, true));
    stm32_metapac::EXTI.rpr(0).write(|v| v.set_line(19, true));
}

pub fn vddusb_monitor_up_tmp() {
    // exti line 19 for uvm
    static mut USB_POWER_UP: bool = false;
    stm32_metapac::PWR.svmcr().modify(|w| {
        w.set_uvmen(true);
    });
    stm32_metapac::EXTI.rtsr(0).modify(|v| v.set_line(19, true));
    stm32_metapac::EXTI.ftsr(0).modify(|v| v.set_line(19, true));
    stm32_metapac::EXTI.imr(0).modify(|v| v.set_line(19, true));
    stm32_metapac::EXTI.emr(0).modify(|v| v.set_line(19, true));
    // get vddusb status
    while stm32_metapac::PWR.svmsr().read().vddusbrdy() == false {}
    unsafe {
        USB_POWER_UP = true;
        CLOCK_REQUESTS[ClockFreqs::KernelFreq160Mhz.to_idx()] += 1; // request 160Mhz
        set_clock();
        low_power::no_deep_sleep_request();
    }
}
