use crate::clock;
use core::{future::poll_fn, sync::atomic::AtomicBool, task::Waker, time::Duration};
use cortex_m::peripheral::NVIC;
use embassy_sync::waitqueue::AtomicWaker;
use stm32_metapac::interrupt;

pub struct Lptim {
    num: u8,
    ins: stm32_metapac::lptim::LptimAdv,
}
pub use stm32_metapac::lptim::vals::Presc as LptimPrescaler;

static mut TAKEN: [bool; 8] = [false; 8]; // first bit will be ignored
const ARRAY_REPEAT_VALUE: AtomicBool = AtomicBool::new(false);
static mut TIMER_LOCKER: [AtomicBool; 4] = [ARRAY_REPEAT_VALUE; 4]; // true means the timer is running
const NEW_AW: AtomicWaker = AtomicWaker::new();
static mut WAKER: [AtomicWaker; 4] = [NEW_AW; 4];

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
    pub fn init_new(&self) {}
    pub fn init(&self) {
        clock::set_lptim_clock(self.num);
        self.ins.cr().modify(|v| v.set_enable(true));
        self.ins
            .cfgr()
            .modify(|v| v.set_presc(stm32_metapac::lptim::vals::Presc::DIV32));
        self.ins.dier().modify(|v| v.set_ueie(true));
        unsafe {
            NVIC::unmask(stm32_metapac::Interrupt::LPTIM1);
            NVIC::unmask(stm32_metapac::Interrupt::LPTIM2);
        }
    }
    pub fn get_resolution(&self) -> Duration {
        // 16MHz / 32 = 500KHz = 2us
        Duration::from_micros(2)
    }
    pub fn get_cnt(&self) -> u32 {
        self.ins.cnt().read().0
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
    pub fn on_interrupt(timer_num: u32) {
        unsafe {
            let index = timer_num as usize - 1;
            TIMER_LOCKER[index].store(false, core::sync::atomic::Ordering::Relaxed);
            WAKER[index].wake();
            // clear update event flag
            match timer_num {
                1 => stm32_metapac::LPTIM1.icr().modify(|v| v.set_uecf(true)),
                2 => stm32_metapac::LPTIM2.icr().modify(|v| v.set_uecf(true)),
                3 => stm32_metapac::LPTIM3.icr().modify(|v| v.set_uecf(true)),
                _ => todo!("not supported LPTIM"),
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

pub struct GlobalLptim {
    timer1: Lptim,
    timer2: Lptim,
    gloabl_elapsed: u128, // in tick
    wakers: [AtomicWaker; 32],
    wakers_ddl: [u128; 32],
}
use crate::hal::Timer;
impl GlobalLptim {
    fn new(tim: Lptim, tim2: Lptim) -> Self {
        tim.init();
        Self {
            timer1: tim,
            timer2: tim2,
            gloabl_elapsed: 0,
            wakers: [NEW_AW; 32],
            wakers_ddl: [0; 32],
        }
    }
}
impl Timer for GlobalLptim {
    fn elapsed(&self) -> u32 {
        return 0;
    }
    fn delay(&self, us: u32) -> impl core::future::Future<Output = ()> {
        self.timer1.after(Duration::from_micros(us as u64))
    }
    fn resolution(&self) -> u32 {
        return 0;
    }
}
