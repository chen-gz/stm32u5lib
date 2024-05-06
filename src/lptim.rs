use core::future::poll_fn;
use stm32_metapac::interrupt;
use crate::{clock};
pub struct Lptim {
    num: u8,
    ins: stm32_metapac::lptim::LptimAdv,
}
static mut TAKEN: [bool; 8] = [false; 8]; // first bit will be ignored

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
        // lptim request the 32.768KHz LSE clock,
        // lptim only has 16 bits counter, so the max period is 65_535 > 32_768
        // so we set the prescaler to 1, and the update event of the counter is 1Hz. (arr = 32_767)
        // the resolution of the counter is 1/32_768 = 30.5us if we need accurate timing, don't use this library for now. Please use the regular timer.
        // enable the lptim clock
        clock::set_lptim_clock(self.num);

        // disable the counter
        self.ins.cr().modify(|v| v.set_enable(false));
        // set the prescaler to 1
        // self.ins.cfgr().modify(|v| v.set_presc(0));
        // set arr to 32_767 (0, 32767) include 0 and 32_767. total 32_768 counts
        self.ins.arr().write(|v| v.0 = 32_767);
        // the default values of the counter is
        // everything use as default
        // start the counter
        self.ins.cr().modify(|v| v.set_enable(true));
    }
    pub fn get_cnt(&self) -> u32 {
        self.ins.cnt().read().0
    }
}

pub fn read_tim1() -> u32 {
    stm32_metapac::LPTIM1.cnt().read().0
}

impl Drop for Lptim {
    fn drop(&mut self) {
        unsafe {
            TAKEN[self.num as usize] = false;
        }
    }
}

use embassy_sync::waitqueue::AtomicWaker;


async fn after(_duration: core::time::Duration) {
    // convert to ticks
    unsafe {
        let tick = _duration.as_micros() as u32 / 30; // not very accurate
        // read current tick from the counter
        let cnt = read_tim1();
        let timeout_tick = GLOBAL_SECOND * 32_768 + cnt as u128 + tick as u128;
        let unused = USED.iter().position(|&x| !x).unwrap();
        poll_fn(|cx| {
            // get current tick from the counter
            WAKER[unused].register(cx.waker());
            let cnt = read_tim1();
            // get the global time
            let cur = GLOBAL_SECOND * 32_768 + cnt as u128;
            if cur > timeout_tick {
                USED[unused] = false;
                core::task::Poll::Ready(())
            } else {
                // if the current is minimum, setup the compare register
                let rest = timeout_tick - cur;
                let ccie1 = stm32_metapac::LPTIM1.ccmr().read().cce(0);
                let ccr = stm32_metapac::LPTIM1.ccr(0).read().ccr();
                if rest < 32_768 && ccie1 == false
                    || (rest < 32_768 && ccie1 == true && rest < ccr as u128)
                {
                    {
                        stm32_metapac::LPTIM1.ccmr().modify(|v| v.set_cce(0, false)); // enable the compare interrupt
                        stm32_metapac::LPTIM1.ccmr().modify(|v| v.set_ccsel(0, stm32_metapac::lptim::vals::Ccsel::OUTPUTCOMPARE));
                        stm32_metapac::LPTIM1
                            .ccr(0)
                            .write(|v| v.set_ccr(rest as u16));
                        stm32_metapac::LPTIM1.ccmr().modify(|v| v.set_cce(0, true));
                        // enable the compare interrupt
                        stm32_metapac::LPTIM1.dier().modify(|v| v.set_ccie(0, true));
                    }
                }
                core::task::Poll::Pending
            }
        })
            .await;
    }
}

fn wake_all() {
    unsafe {
        for i in 0..8 {
            WAKER[i].wake();
        }
    }
}



// this will use two timmer to implement the global timer
// TIM1 is free running timer, and the LPTIM1 is the global timer
// TIM2 used to set alarm, then we can wake up the task
pub struct LptimGlobalTimer;

const NEW_AW: AtomicWaker = AtomicWaker::new();
static mut WAKER: [AtomicWaker; 32] = [NEW_AW; 32];
static mut GLOBAL_SECOND: u128 = 0;
static mut USED: [bool; 8] = [false; 8];
static mut NEXT_INDEX: usize = USIZE_MAX;

pub trait GlobalTimer {
    fn init();
    static async fn after(duration: core::time::Duration);
    static fn now() -> core::time::Duration;
    fn resolution() -> core::time::Duration;
}
impl GlobalTimer for LptimGlobalTimer{
    fn now() -> core::time::Duration {
        let cnt = read_tim1();
        let sec = GLOBAL_SECOND;
        let cnt = cnt * 1_000_000_000 / 32_768;  // convert to nano seconds
        core::time::Duration::from_nanos((sec * 1_000_000_000 + cnt) as u64)
    }
    fn resolution() -> core::time::Duration {
        core::time::Duration::from_nanos(1_000_000_000 / 32_768)
    }
    aysnc fn after(duration: core::time::Duration) {
        let cur = Self::now();
        let target = cur + duration;
        let unused = USED.iter().position(|&x| !x).unwrap();
        USED[unused] = true;
        poll_fn(|cx| {
            WAKER[unused].register(cx.waker());
            let cur = Self::now();
            if cur >= target {
                USED[unused] = false;
                core::task::Poll::Ready(())
            } else if {
                // rest less than 1 second
                let rest = target - cur;
                // convert to ticks (1/32_768) (round up)
                let tick = (rest.as_nanos() + 32_768 - 1) / 32_768;
                // set the arr to the tick and enable the interrupt (LPTIM2)
                stm32_metapac::LPTIM2.arr().write(|v| v.0 = tick as u16);
                stm32_metapac::LPTIM2.cntr().write(|v| v.0 = 0);
                stm32_metapac::LPTIM2.dier().modify(|v| v.set_ueie(true));
                stm32_metapac::LPTIM2.cr().modify(|v| v.set_enable(true));
            }
            else {
                NEXT_INDEX = unused;
                core::task::Poll::Pending
            }
        }).await;
    }
}


#[interrupt]
fn LPTIM1() {
    unsafe {
        if stm32_metapac::LPTIM1.isr().read().ue() {
            GLOBAL_SECOND += 1;
        }
        stm32_metapac::LPTIM1.icr().modify(|v| {
            v.set_uecf(true);
        });
        WAKER[NEXT_INDEX].wake();
    }
}
#[interrupt]
fn LPTIM2() {
    unsafe {
        stm32_metapac::LPTIM2.icr().modify(|v| {
            v.set_uecf(true);
        });
        // disable the counter
        stm32_metapac::LPTIM2.cr().modify(|v| v.set_enable(false));
        WAKER[NEXT_INDEX].wake();
    }
}