// this file is based on embassy_stm32::embassy_stm32::low_power

//! Low-power support.
//!
//! The STM32 line of microcontrollers support various deep-sleep modes which exploit clock-gating
//! to reduce power consumption. `embassy-stm32` provides a low-power executor, [`Executor`] which
//! can use knowledge of which peripherals are currently blocked upon to transparently and safely
//! enter such low-power modes (currently, only `STOP2`) when idle.
//!
//! The executor determines which peripherals are active by their RCC state; consequently,
//! low-power states can only be entered if all peripherals have been `drop`'d. There are a few
//! exceptions to this rule:
//!
//!  * `GPIO`
//!  * `RCC`
//!
//! Since entering and leaving low-power modes typically incurs a significant latency, the
//! low-power executor will only attempt to enter when the next timer event is at least
//! [`time_driver::MIN_STOP_PAUSE`] in the future.
//!
//! Currently there is no macro analogous to `embassy_executor::main` for this executor;
//! consequently one must define their entrypoint manually. Moveover, you must relinquish control
//! of the `RTC` peripheral to the executor. This will typically look like
//!
//! ```rust,no_run
//! use embassy_executor::Spawner;
//! use embassy_stm32::low_power::Executor;
//! use embassy_stm32::rtc::{Rtc, RtcConfig};
//! use static_cell::StaticCell;
//!
//! #[cortex_m_rt::entry]
//! fn main() -> ! {
//!     Executor::take().run(|spawner| {
//!         unwrap!(spawner.spawn(async_main(spawner)));
//!     });
//! }
//!
//! #[embassy_executor::task]
//! async fn async_main(spawner: Spawner) {
//!     // initialize the platform...
//!     let mut config = embassy_stm32::Config::default();
//!     // when enabled the power-consumption is much higher during stop, but debugging and RTT is working
//!     config.enable_debug_during_sleep = false;
//!     let p = embassy_stm32::init(config);
//!
//!     // give the RTC to the executor...
//!     let mut rtc = Rtc::new(p.RTC, RtcConfig::default());
//!     static RTC: StaticCell<Rtc> = StaticCell::new();
//!     let rtc = RTC.init(rtc);
//!     embassy_stm32::low_power::stop_with_rtc(rtc);
//!
//!     // your application here...
//! }
//! ```
//!
//!
//!

use core::arch::asm;
use core::marker::PhantomData;

use cortex_m::peripheral::SCB;
use embassy_executor::*;

// use crate::interrupt;
// use crate::time_driver::{get_driver, RtcDriver};

const THREAD_PENDER: usize = usize::MAX;
static mut EXECUTOR: Option<Executor> = None;

/// Thread mode executor, using WFE/SEV.
///
/// This is the simplest and most common kind of executor. It runs on
/// thread mode (at the lowest priority level), and uses the `WFE` ARM instruction
/// to sleep when it has no more work to do. When a task is woken, a `SEV` instruction
/// is executed, to make the `WFE` exit from sleep and poll the task.
///
/// This executor allows for ultra low power consumption for chips where `WFE`
/// triggers low-power sleep without extra steps. If your chip requires extra steps,
/// you may use [`raw::Executor`] directly to program custom behavior.
pub struct Executor {
    inner: raw::Executor,
    not_send: PhantomData<*mut ()>,
    scb: SCB,
    // time_driver: &'static RtcDriver,
}
pub static mut REF_COUNT_DEEP: u32 = 0;
static mut _REF_COUNT_STOP1: u32 = 0;
static mut _REF_COUNT_STOP2: u32 = 0;
static mut _REF_COUNT_STOP3: u32 = 0;
static mut _REF_COUNT_STANDBY: u32 = 0;

/// NO DEEP SLEEP if this function is called, the mcu will not go deep sleep
pub fn no_deep_sleep_request() {
    unsafe {
        REF_COUNT_DEEP += 1;
    }
}

pub fn no_deep_sleep_release() {
    unsafe {
        REF_COUNT_DEEP -= 1;
    }
}

pub fn run_no_deep_sleep<F>(code: F)
where
    F: FnOnce(),
{
    unsafe {
        REF_COUNT_DEEP += 1;
        code();
        REF_COUNT_DEEP -= 1;
    }
}

pub async fn run_no_deep_sleep_async<F, R, T>(code: F) -> T
where
    F: FnOnce() -> R,
    R: core::future::Future<Output = T>,
{
    unsafe {
        REF_COUNT_DEEP += 1;
        let result = code().await;
        REF_COUNT_DEEP -= 1;
        result
    }
}

impl Executor {
    /// Create a new Executor.
    pub fn take() -> &'static mut Self {
        cortex_m::interrupt::free(|_| unsafe {
            // critical_section::with(|_| unsafe {
            // assert!(EXECUTOR.is_none());

            EXECUTOR = Some(Self {
                inner: raw::Executor::new(THREAD_PENDER as *mut ()),
                not_send: PhantomData,
                scb: cortex_m::Peripherals::steal().SCB,
            });
            #[allow(static_mut_refs)]
            EXECUTOR.as_mut().unwrap()
        })
    }

    // #[allow(unused_variables)]
    // fn configure_stop(&mut self) {
    //     PWR.cr1().modify(|v| v.set_lpms(pwr::vals::Lpms::STOP3));
    // }

    fn configure_pwr(&mut self) {
        unsafe {
            if REF_COUNT_DEEP == 0 {
                self.scb.set_sleepdeep();
            } else {
                self.scb.clear_sleepdeep();
            }
        }
    }

    /// Run the executor.
    ///
    /// The `init` closure is called with a [`Spawner`] that spawns tasks on
    /// this executor. Use it to spawn the initial task(s). After `init` returns,
    /// the executor starts running the tasks.
    ///
    /// To spawn more tasks later, you may keep copies of the [`Spawner`] (it is `Copy`),
    /// for example by passing it as an argument to the initial tasks.
    ///
    /// This function requires `&'static mut self`. This means you have to store the
    /// Executor instance in a place where it'll live forever and grants you mutable
    /// access. There's a few ways to do this:
    ///
    /// - a [StaticCell](https://docs.rs/static_cell/latest/static_cell/) (safe)
    /// - a `static mut` (unsafe)
    /// - a local variable in a function you know never returns (like `fn main() -> !`), upgrading its lifetime with `transmute`. (unsafe)
    ///
    /// This function never returns.
    pub fn run(&'static mut self, init: impl FnOnce(Spawner)) -> ! {
        // init(unsafe { EXECUTOR.as_mut().unwrap() }.inner.spawner());
        init(self.inner.spawner());

        loop {
            unsafe {
                // EXECUTOR.as_mut().unwrap().inner.poll();
                // self.inner.poll();
                self.inner.poll();
                // self.configure_pwr();

                if REF_COUNT_DEEP == 0 {
                    self.scb.set_sleepdeep();
                } else {
                    self.scb.clear_sleepdeep();
                }
                crate::clock::set_clock();
                asm!("wfe");
                // get wake up event
                // let mut w = self.scb.icsr.read();

                crate::clock::set_clock();
                // defmt::info!("wake up from wfe");
            };
        }
    }
}
