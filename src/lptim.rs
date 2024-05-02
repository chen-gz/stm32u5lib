use core::future::poll_fn;

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

const NEW_AW: AtomicWaker = AtomicWaker::new();
static mut WAKER: [AtomicWaker; 8] = [NEW_AW; 8];
static mut GLOBAL_SECOND: u128 = 0;
static mut USED: [bool; 8] = [false; 8];

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

use stm32_metapac::interrupt;

#[interrupt]
fn LPTIM1() {
    // clear the interrupt flag
    // if stm32_metapac::LPTIM1.isr().read().
    unsafe {
        if stm32_metapac::LPTIM1.isr().read().ue() {
            GLOBAL_SECOND += 1;
        }
        stm32_metapac::LPTIM1.icr().modify(|v| {
            v.set_cccf(0, true);
            v.set_uecf(true);
        });
        // disable the compare interrupt
        stm32_metapac::LPTIM1.dier().modify(|v| v.set_ccie(0, false));
    }
    wake_all();
}
