use crate::usb::reg::vals::Dspd;
use crate::usb::reg::Otg;
pub use crate::usb::Config;
use crate::usb::{
    bus::Bus as OtgBus, controlpipe::ControlPipe, driver::Driver as OtgDriver, endpoint::Endpoint,
    interrupt::on_interrupt as on_interrupt_impl, In, OtgInstance, Out, PhyType, State,
};
use cortex_m::peripheral::NVIC;
use defmt::info;
use stm32_metapac::interrupt;

use crate::clock::delay_us;
use embassy_usb_driver::{EndpointAddress, EndpointAllocError, EndpointType, Event, Unsupported};

const MAX_EP_COUNT: usize = 9;
#[cfg(stm32u5a5)]
pub mod fifo_const {
    pub const MAX_EP_COUNT: usize = 9;
    pub const FIFO_DEPTH_WORDS: u16 = 1024;
    pub const RX_FIFO_SIZE_EACH: u16 = 128;
    pub const RX_FIFO_SIZE_SIZE_WORD: u16 = 288;
    pub const TX_FIFO_SIZE_WORDS: [u16; MAX_EP_COUNT] = [64, 64, 64, 64, 64, 64, 80, 80, 96];
}
#[cfg(stm32u575)]
pub mod fifo_const {
    // todo!("fix thsese constants");
    pub const MAX_EP_COUNT: usize = 6;
    pub const FIFO_DEPTH_WORDS: u16 = 1024;
    pub const RX_FIFO_SIZE_EACH: u16 = 128;
    pub const RX_FIFO_SIZE_SIZE_WORD: u16 = 288;
    pub const TX_FIFO_SIZE_WORDS: [u16; MAX_EP_COUNT] = [16, 16, 16, 16, 16, 16];
    //  80, 80, 96];
}

static STATE: State<{ MAX_EP_COUNT }> = State::new();

fn regs() -> stm32_metapac::otg::Otg {
    #[cfg(stm32u575)]
    unsafe {
        stm32_metapac::USB_OTG_FS
    }
    #[cfg(stm32u5a5)]
    unsafe {
        stm32_metapac::USB_OTG_HS
    }
}

#[cfg(stm32u575)]
#[interrupt]
unsafe fn OTG_FS() {
    let r = regs();
    let state = &STATE;
    on_interrupt_impl(r, state, MAX_EP_COUNT);
}
#[cfg(stm32u5a5)]
#[interrupt]
unsafe fn OTG_HS() {
    defmt::trace!("OTG_HS interrupt");
    let r = regs();
    let state = &STATE;
    on_interrupt_impl(r, state, MAX_EP_COUNT);
}

// From `synopsys-usb-otg` crate:
// This calculation doesn't correspond to one in a Reference Manual.
// In fact, the required number of words is higher than indicated in RM.
// The following numbers are pessimistic and were figured out empirically.
const RX_FIFO_EXTRA_SIZE_WORDS: u16 = 30;

/// USB driver.
pub struct Driver<'d> {
    // phantom: PhantomData<&'d mut T>,
    inner: OtgDriver<'d, MAX_EP_COUNT>,
}

use crate::hal::Pin;
impl<'d> Driver<'d> {
    /// Initializes USB OTG peripheral with internal Full-Speed PHY.
    ///
    /// # Arguments
    ///
    /// * `ep_out_buffer` - An internal buffer used to temporarily store received packets.
    /// Must be large enough to fit all OUT endpoint max packet sizes.
    /// Endpoint allocation will fail if it is too small.
    pub fn new_fs<P: Pin>(dp: P, dm: P, ep_out_buffer: &'d mut [u8], config: Config) -> Self {
        dp.setup();
        dm.setup();
        let regs = unsafe { Otg::from_ptr(regs().as_ptr()) };

        let instance = OtgInstance {
            regs,
            state: &STATE,
            fifo_depth_words: fifo_const::FIFO_DEPTH_WORDS,
            extra_rx_fifo_words: fifo_const::RX_FIFO_SIZE_SIZE_WORD,
            // RX_FIFO_EXTRA_SIZE_WORDS,,
            endpoint_count: fifo_const::MAX_EP_COUNT,
            phy_type: PhyType::InternalHighSpeed,
            calculate_trdt_fn: calculate_trdt,
        };

        Self {
            inner: OtgDriver::new(ep_out_buffer, instance, config),
        }
    }
}

impl<'d> embassy_usb_driver::Driver<'d> for Driver<'d> {
    type EndpointOut = Endpoint<'d, Out>;
    type EndpointIn = Endpoint<'d, In>;
    type ControlPipe = ControlPipe<'d>;
    type Bus = Bus<'d>;

    fn alloc_endpoint_in(
        &mut self,
        ep_type: EndpointType,
        ep_addr: Option<EndpointAddress>,
        max_packet_size: u16,
        interval_ms: u8,
    ) -> Result<Self::EndpointIn, EndpointAllocError> {
        self.inner
            .alloc_endpoint_in(ep_type, ep_addr, max_packet_size, interval_ms)
    }

    fn alloc_endpoint_out(
        &mut self,
        ep_type: EndpointType,
        ep_addr: Option<EndpointAddress>,
        max_packet_size: u16,
        interval_ms: u8,
    ) -> Result<Self::EndpointOut, EndpointAllocError> {
        self.inner
            .alloc_endpoint_out(ep_type, ep_addr, max_packet_size, interval_ms)
    }

    fn start(self, control_max_packet_size: u16) -> (Self::Bus, Self::ControlPipe) {
        let (bus, cp) = self.inner.start(control_max_packet_size);

        (
            Bus {
                inner: bus,
                inited: false,
            },
            cp,
        )
    }
}

/// USB bus.
pub struct Bus<'d> {
    inner: OtgBus<'d, MAX_EP_COUNT>,
    inited: bool,
}
fn common_init() {
    stm32_metapac::PWR.svmcr().modify(|v| v.set_usv(true));
    stm32_metapac::PWR.svmcr().modify(|v| v.set_uvmen(true));

    critical_section::with(|_| {
        stm32_metapac::PWR.vosr().modify(|v| {
            v.0 |= (1 << 19) | (1 << 20);
            v.boosten();
            v.usbpwren();
            v.usbboosten();
        });
        delay_us(100);
        stm32_metapac::RCC.cr().modify(|v| v.set_hseon(true));
        // wait for HSE to stabilize
        while !stm32_metapac::RCC.cr().read().hserdy() {}
    });

    stm32_metapac::RCC.apb3enr().modify(|w| {
        w.set_syscfgen(true);
    });
    stm32_metapac::RCC.ahb2enr1().modify(|w| {
        w.set_usb_otg_hs_phyen(true);
        w.set_usb_otg_hsen(true);
    });
    stm32_metapac::SYSCFG.otghsphycr().modify(|v| {
        v.set_clksel(stm32_metapac::syscfg::vals::Usbrefcksel::MHZ16);
        v.set_en(true);
    });
    stm32_metapac::RCC.ccipr2().modify(|w| {
        w.set_otghssel(stm32_metapac::rcc::vals::Otghssel::HSE);
    });

    defmt::trace!("Waiting for USB power to stabilize");
    while !stm32_metapac::PWR.svmsr().read().vddusbrdy() {}
    defmt::trace!("USB power stabilized");



    #[cfg(stm32u575)]
    unsafe {
        NVIC::unmask(stm32_metapac::interrupt::OTG_FS);
        NVIC::unpend(stm32_metapac::interrupt::OTG_FS);
    }
    #[cfg(stm32u5a5)]
    unsafe {
        NVIC::unmask(stm32_metapac::interrupt::OTG_HS);
        NVIC::unpend(stm32_metapac::interrupt::OTG_HS);
    }
    info!("USB OTG initialized, interrupt enabled");
}

impl<'d> Bus<'d> {
    fn init(&mut self) {
        common_init();
        let phy_type = self.inner.phy_type();
        // let r = stm32_metapac::USB_OTG_FS;
        let r = regs();
        let core_id = r.cid().read().0;
        // trace!("Core id {:08x}", core_id);

        // Wait for AHB ready.
        while !r.grstctl().read().ahbidl() {}

        // Configure as device.
        self.inner.configure_as_device();

        // Configuring Vbus sense and SOF output
        match core_id {
            0x0000_1200 | 0x0000_1100 | 0x0000_1000 => self.inner.config_v1(),
            0x0000_2000 | 0x0000_2100 | 0x0000_2300 | 0x0000_3000 | 0x0000_3100 => {
                self.inner.config_v2v3()
            }
            0x0000_5000 => self.inner.config_v5(),
            _ => unimplemented!("Unknown USB core id {:X}", core_id),
        }
    }

    fn disable(&mut self) {
        // disable interrupts
        // #[cfg(stm32u575)]
        // NVIC::mask(stm32_metapac::interrupt::OTG_FS);
        // #[cfg(stm32u5a5)]
        // NVIC::mask(stm32_metapac::interrupt::OTG_HS);
        // disable clock

        // rcc::disable::<T>();
        self.inited = false;

        // #[cfg(stm32l4)]
        // crate::pac::PWR.cr2().modify(|w| w.set_usv(false));
        // Cannot disable PWR, because other peripherals might be using it
    }
}

impl<'d> embassy_usb_driver::Bus for Bus<'d> {
    async fn poll(&mut self) -> Event {
        if !self.inited {
            self.init();
            self.inited = true;
        }

        self.inner.poll().await
    }

    fn endpoint_set_stalled(&mut self, ep_addr: EndpointAddress, stalled: bool) {
        self.inner.endpoint_set_stalled(ep_addr, stalled)
    }

    fn endpoint_is_stalled(&mut self, ep_addr: EndpointAddress) -> bool {
        self.inner.endpoint_is_stalled(ep_addr)
    }

    fn endpoint_set_enabled(&mut self, ep_addr: EndpointAddress, enabled: bool) {
        self.inner.endpoint_set_enabled(ep_addr, enabled)
    }

    async fn enable(&mut self) {
        self.inner.enable().await
    }

    async fn disable(&mut self) {
        // NOTE: inner call is a no-op
        self.inner.disable().await
    }

    async fn remote_wakeup(&mut self) -> Result<(), Unsupported> {
        self.inner.remote_wakeup().await
    }
}

impl<'d> Drop for Bus<'d> {
    fn drop(&mut self) {
        Bus::disable(self);
    }
}

fn calculate_trdt(speed: Dspd) -> u8 {
    // todo: fix this constant
    return 0x9;
    let ahb_freq = 160_000_000;
    match speed {
        Dspd::HIGH_SPEED => {
            // From RM0431 (F72xx), RM0090 (F429), RM0390 (F446)
            if ahb_freq >= 30_000_000 || cfg!(stm32h7rs) {
                0x9
            } else {
                panic!("AHB frequency is too low")
            }
        }
        Dspd::FULL_SPEED_EXTERNAL | Dspd::FULL_SPEED_INTERNAL => {
            // From RM0431 (F72xx), RM0090 (F429)
            match ahb_freq {
                0..=14_199_999 => panic!("AHB frequency is too low"),
                14_200_000..=14_999_999 => 0xF,
                15_000_000..=15_999_999 => 0xE,
                16_000_000..=17_199_999 => 0xD,
                17_200_000..=18_499_999 => 0xC,
                18_500_000..=19_999_999 => 0xB,
                20_000_000..=21_799_999 => 0xA,
                21_800_000..=23_999_999 => 0x9,
                24_000_000..=27_499_999 => 0x8,
                27_500_000..=31_999_999 => 0x7, // 27.7..32 in code from CubeIDE
                32_000_000..=u32::MAX => 0x6,
            }
        }
        _ => unimplemented!(),
    }
}
