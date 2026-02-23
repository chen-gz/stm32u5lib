use crate::clock;
use core::{future::poll_fn, sync::atomic::AtomicBool, time::Duration};
use cortex_m::peripheral::NVIC;
use embassy_sync::waitqueue::AtomicWaker;
use stm32_metapac::interrupt;

pub struct Lptim {
    num: u8,
    src_clock_freq: u32,
    ins: stm32_metapac::lptim::Lptim,
}
pub use stm32_metapac::lptim::vals::Presc as LptimPrescaler;

static mut TAKEN: [bool; 8] = [false; 8]; // first bit will be ignored
const ARRAY_REPEAT_VALUE: AtomicBool = AtomicBool::new(false);
static mut TIMER_LOCKER: [AtomicBool; 4] = [ARRAY_REPEAT_VALUE; 4]; // true means the timer is running
const NEW_AW: AtomicWaker = AtomicWaker::new();
static mut WAKER: [AtomicWaker; 4] = [NEW_AW; 4];

/// number of interrupt happened in this timer
/// If this is a free run clock, time elapsed = interrupt_points * (ARR+1) * (1/frequency)
static mut INTERRUPTED_CNT: [u128; 4] = [0; 4]; // number of interrupt happened in this timer

impl Lptim {
    // two function are implement for lower power timer (lptimer)
    // 1. A free run system clock. Free run system clock will perform in low frequency as default (1Mhz) == 1us
    // The resaon that I use 1Mhz is that the MCU clock can be very low (4Mhz).
    // When the MCU clock is 4Mhz, the free run system clock have to lower than the MCU clock/4, in order to read the correct value.
    // When reading the value from the free run system clock, it has to read the value twice, in order to get the correct value
    // The system will weak up every 50ms (ARR set to 49999).
    //
    // 2. A timer, which can be used to generate a delay. The defualt frequency is 16Mhz. The lptimer is an 16 bit timer, the maximun delay is 65535 * (1/16M) = 65535/16 = 4095.9375us = 4ms.

    pub fn new(num: u8) -> Self {
        let ret = match num {
            1 => Self {
                num,
                ins: stm32_metapac::LPTIM1,
                src_clock_freq: 0,
            },
            2 => Self {
                num,
                ins: stm32_metapac::LPTIM2,
                src_clock_freq: 0,
            },
            3 => Self {
                num,
                ins: stm32_metapac::LPTIM3,
                src_clock_freq: 0,
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
    pub fn get_interrupted_cnt(&self) -> u128 {
        unsafe { INTERRUPTED_CNT[self.num as usize - 1] }
    }
    pub fn get_period(&self) -> Duration {
        let arr = self.ins.arr().read().0;
        info!("arr: {}", arr);
        self.get_resolution() * (arr + 1)
    }
    pub fn calc_arr(&self, period: Duration) -> u32 {
        (period.as_nanos() / self.get_resolution().as_nanos()) as u32 - 1
    }
    pub fn init_new(&mut self, presc: LptimPrescaler) {
        self.src_clock_freq = clock::set_lptim_clock(self.num);
        self.ins.cr().modify(|v| v.set_enable(true));
        self.ins.cfgr().modify(|v| v.set_presc(presc));
        // self.ins.dier().modify(|v| v.set_ueie(true)); // update event
        match self.num {
            1 => unsafe {
                NVIC::unmask(stm32_metapac::Interrupt::LPTIM1);
            },
            2 => unsafe {
                NVIC::unmask(stm32_metapac::Interrupt::LPTIM2);
            },
            3 => unsafe {
                NVIC::unmask(stm32_metapac::Interrupt::LPTIM3);
            },
            _ => {
                panic!("Current not support LPTIM4");
            }
        }
    }
    pub fn init(&self) {
        clock::set_lptim_clock(self.num);
        self.ins.cr().modify(|v| v.set_enable(true));
        self.ins
            .cfgr()
            .modify(|v| v.set_presc(stm32_metapac::lptim::vals::Presc::DIV32));
        self.ins.dier().modify(|v| v.set_ueie(true));
        // self.ins.arr().modify(|v| v.0 = 49_999); // default arr set to 49_999
        self.ins.arr().modify(|v| v.0 = self.calc_arr(Duration::from_millis(50)));
        unsafe {
            NVIC::unmask(stm32_metapac::Interrupt::LPTIM1);
            NVIC::unmask(stm32_metapac::Interrupt::LPTIM2);
        }
    }
    pub fn get_frequency(&self) -> u32 {
        let presc = self.ins.cfgr().read().presc();
        let div = 1 << (presc as u32);
        self.src_clock_freq / div
    }
    pub fn get_resolution(&self) -> Duration {
        if self.get_frequency() == 0 {
            info!("src_clock_freq: {}", self.src_clock_freq);
            panic!("LPTIM{} frequency is 0", self.num);
        }
        Duration::from_nanos(1_000_000_000 / self.get_frequency() as u64)
    }

    // if need cnt and interrupted_cnt, use get_cnt_and_interrupted_cnt to get correct value
    pub fn get_cnt(&self) -> u32 {
        // read cnt twice to get correct value
        let mut tmp = self.ins.cnt().read().0;
        let mut cnt = 0;
        while tmp != self.ins.cnt().read().0 {
            cnt += 1;
            tmp = self.ins.cnt().read().0;
            if cnt >= 10 {
                panic!("LPTIM{} read cnt failed", self.num);
            }
        }
        tmp
    }

    pub fn get_cnt_and_interrupted_cnt(&self) -> (u32, u128) {
        let mut cnt = self.get_cnt();
        let mut interrupted_cnt = unsafe { INTERRUPTED_CNT[self.num as usize - 1] };
        while cnt != self.get_cnt() {
            cnt = self.get_cnt();
            interrupted_cnt = unsafe { INTERRUPTED_CNT[self.num as usize - 1] };
        }
        (cnt, interrupted_cnt)
    }
    pub async fn after(&self, duration: Duration) {
        let mut duration = duration;
        while duration > Duration::from_millis(100) {
            self.after_limit(Duration::from_millis(100)).await;
            duration -= Duration::from_millis(100);
        }
        self.after_limit(duration).await;
    }
    /// following function implement the delay function with the limitation of 100ms
    /// if the duration is larger than 100ms, use after instead
    async fn after_limit(&self, duration: Duration) {
        unsafe {
            let tick = duration.as_micros() as u32 / self.get_resolution().as_micros() as u32;
            if tick == 0 {
                return;
            }
            let index = self.num as usize - 1;
            if TIMER_LOCKER[index].load(core::sync::atomic::Ordering::Relaxed) {
                panic!("LPTIM{} is already running", self.num);
            }
            TIMER_LOCKER[index].store(true, core::sync::atomic::Ordering::Relaxed);
            self.ins.arr().write(|v| v.0 = tick);
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
            })
            .await;
        }
    }
    async fn wait_for_interrupt(&self) {
        let index = self.num as usize - 1;
        poll_fn(|cx| unsafe {
            WAKER[index].register(cx.waker());
            if TIMER_LOCKER[index].load(core::sync::atomic::Ordering::Relaxed) {
                return core::task::Poll::Pending;
            } else {
                return core::task::Poll::Ready(());
            }
        })
        .await;
    }
    pub fn on_interrupt(timer_num: u32) {
        unsafe {
            // self.interrupts += 1;
            let index = timer_num as usize - 1;
            TIMER_LOCKER[index].store(false, core::sync::atomic::Ordering::Relaxed);
            WAKER[index].wake();
            INTERRUPTED_CNT[index] += 1;
            // clear update event flag
            match timer_num {
                1 => stm32_metapac::LPTIM1.icr().modify(|v| v.set_uecf(true)),
                2 => stm32_metapac::LPTIM2.icr().modify(|v| v.set_uecf(true)),
                3 => stm32_metapac::LPTIM3.icr().modify(|v| v.set_uecf(true)),
                _ => panic!("not supported LPTIM"),
            }
        }
    }
}

/// WallTimer is a monotonic timer that counts in microseconds.
pub struct WallTimer {
    lptim: Lptim,
}

impl WallTimer {
    pub fn new(mut lptim: Lptim) -> Self {
        lptim.init_new(LptimPrescaler::DIV32);
        lptim.ins.arr().write(|v| v.0 = 0xFFFF);
        lptim.ins.dier().modify(|v| v.set_ueie(true));
        lptim.ins.cr().modify(|v| v.set_cntstrt(true));
        Self { lptim }
    }

    /// Returns the current time in microseconds.
    pub fn now(&self) -> u64 {
        loop {
            let high1 = self.lptim.get_interrupted_cnt();
            let low = self.lptim.get_cnt();
            let high2 = self.lptim.get_interrupted_cnt();

            if high1 == high2 {
                let ue = self.lptim.ins.isr().read().ue();
                let res_ns = self.lptim.get_resolution().as_nanos() as u64;
                return crate::time_math::compute_walltimer(high1, low, ue, res_ns);
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

#[interrupt]
fn LPTIM3() {
    Lptim::on_interrupt(3);
}
