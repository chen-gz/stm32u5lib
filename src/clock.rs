pub mod gg {
    use cortex_m::{self, peripheral::SYST};
    static mut SYSTEM_CLOCK: u32 = 4_000_000;

    pub fn delay_enable(system_clock: u32) {
        unsafe {
            SYSTEM_CLOCK = system_clock;
            let mut p = cortex_m::Peripherals::steal();
            p.DCB.enable_trace(); // enable trace
            let dwt = &mut p.DWT;
            dwt.enable_cycle_counter();
            dwt.cyccnt.modify(|_w| 0);
        }
    }

    pub fn delay_s(n: u32) {
        unsafe {
            let p = cortex_m::Peripherals::steal();
            let dwt = &p.DWT;
            let interval = SYSTEM_CLOCK;
            for _i in 0..n {
                let start = dwt.cyccnt.read();
                let end = start.wrapping_add(interval);
                let mut now = dwt.cyccnt.read();
                while (now >= start && now <= end)
                    || (now >= start && start > end)
                    || (now <= end && end < start)
                {
                    now = dwt.cyccnt.read();
                }
            }
        }
    }

    pub fn delay_ms(n: u32) {
        unsafe {
            let p = cortex_m::Peripherals::steal();
            let dwt = &p.DWT;
            let interval = SYSTEM_CLOCK / 1_000 * n;
            // 170 * (1e3 as u32) * n;
            let start = dwt.cyccnt.read();
            let end = start.wrapping_add(interval);
            let mut now = dwt.cyccnt.read();
            while (now >= start && now <= end)
                || (now >= start && start > end)
                || (now <= end && end < start)
            {
                now = dwt.cyccnt.read();
            }
        }
    }

    pub fn delay_us(n: u32) {
        unsafe {
            let p = cortex_m::Peripherals::steal();
            let dwt = &p.DWT;
            let interval = SYSTEM_CLOCK / 1_000_000 * n;
            let start = dwt.cyccnt.read();
            let end = start.wrapping_add(interval);
            let mut now = dwt.cyccnt.read();
            while (now >= start && now <= end)
                || (now >= start && start > end)
                || (now <= end && end < start)
            {
                now = dwt.cyccnt.read();
            }
        }
    }
}
