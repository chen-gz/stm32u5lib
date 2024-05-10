use core::future::poll_fn;
use core::sync::atomic::AtomicBool;
use core::time::Duration;
use cortex_m::asm::delay;
use cortex_m::peripheral::NVIC;
use stm32_metapac::interrupt;
use crate::{clock};
use embassy_sync::waitqueue::AtomicWaker;

pub struct Lptim {
    num: u8,
    ins: stm32_metapac::lptim::LptimAdv,
}

static mut TAKEN: [bool; 8] = [false; 8]; // first bit will be ignored
const RELOAD_VALUE: u32 = 62_500;  // half second, the clock is 16_000_000 / 128 = 125_000
const ARRAY_REPEAT_VALUE: AtomicBool = AtomicBool::new(false);
static mut TIMER_LOCKER: [AtomicBool; 4] = [ARRAY_REPEAT_VALUE; 4]; // true means the timer is running
const NEW_AW: AtomicWaker = AtomicWaker::new();
static mut WAKER: [AtomicWaker; 4] = [NEW_AW; 4];

impl Lptim {
    pub fn new(num: u8) -> Self {
        let ret = match num {
            1 => Self {
                num,
                ins: stm32_metapac::LPTIM1,
            },
            2 => Self {
                num,
                ins: stm32_metapac::LPTIM2,
            },
            3 => Self {
                num,
                ins: stm32_metapac::LPTIM3,
            },
            // 4 => { Self { num, ins: stm32_metapac::LPTIM4 } }
            _ => panic!("not supported LPTIM"),
        };
        unsafe {
            if TAKEN[num as usize] {
                panic!("LPTIM{} is already taken", num);
            }
            TAKEN[num as usize] = true;
        }
        ret
    }
    pub fn init(&self) {
        clock::set_lptim_clock(self.num);
        self.ins.cr().modify(|v| { v.set_enable(true); });
        self.ins.arr().write(|v| v.0 = 62_500);
        self.ins.cfgr().modify(|v| { v.set_presc(stm32_metapac::lptim::vals::Presc::DIV32); });
        self.ins.dier().modify(|v| { v.set_ueie(true); });
        self.ins.cr().modify(|v| { v.set_cntstrt(true); });
        unsafe {
            NVIC::unmask(stm32_metapac::Interrupt::LPTIM1);
            NVIC::unmask(stm32_metapac::Interrupt::LPTIM2);
        }
    }
    pub fn get_cnt(&self) -> u32 {
        self.ins.cnt().read().0
    }
    pub async fn after(&self, duration: Duration) {
        unsafe {

            // stm32_metapac::LPTIM2.cr().modify(|v| v.set_sngstrt(true));
            let tick = duration.as_micros() as u32 / 2;
            if tick == 0 {
                return;
            }
            // set arr and start the counter
            // stm32_metapac::LPTIM2.arr().write(|v| v.0 = tick);
            self.ins.arr().write(|v| v.0 = tick);
            let index = self.num as usize - 1;
            TIMER_LOCKER[index].store(true, core::sync::atomic::Ordering::Relaxed);
            // enable update event interrupt
            self.ins.dier().modify(|v| v.set_ueie(true));
            self.ins.cr().modify(|v| v.set_sngstrt(true));
            poll_fn(|cx| {
                WAKER[index].register(cx.waker());
                if TIMER_LOCKER[index].load(core::sync::atomic::Ordering::Relaxed) {
                    return core::task::Poll::Pending;
                } else {
                    return core::task::Poll::Ready(());
                }
            }).await;
        }
    }
    pub fn on_interrupt(timer_num: u32) {
        unsafe {
            let index = timer_num as usize - 1;
            TIMER_LOCKER[index].store(false, core::sync::atomic::Ordering::Relaxed);
            WAKER[index].wake();
            // clear update event flag
            match timer_num {
                1 => {
                    stm32_metapac::LPTIM1.icr().modify(|v| {
                        v.set_uecf(true);
                    });
                }
                2 => {
                    stm32_metapac::LPTIM2.icr().modify(|v| {
                        v.set_uecf(true);
                    });
                }
                3 => {
                    stm32_metapac::LPTIM3.icr().modify(|v| {
                        v.set_uecf(true);
                    });
                }
                _ => {}
            }
        }
    }
}


impl Drop for Lptim {
    fn drop(&mut self) {
        unsafe {
            TAKEN[self.num as usize] = false;
        }
    }
}


#[interrupt]
fn LPTIM1() {
    Lptim::on_interrupt(1);
}

#[interrupt]
fn LPTIM2() {
    Lptim::on_interrupt(2);
}