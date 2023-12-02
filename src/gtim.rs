pub mod gg {
    use crate::clock::gg::delay_tick;
    use stm32_metapac::tim::Tim;
    pub struct TimIns {
        ins: Tim,
    }
    pub const TIM1: TimIns = TimIns {
        ins: stm32_metapac::TIM1,
    };

    use crate::gpio::gg::GpioPort;
    use crate::gpio::*;
    impl TimIns {
        pub fn init(&self, freq: u32){
            // always use upcounter mode

        }
    }
}

}

