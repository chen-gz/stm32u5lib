use cortex_m::peripheral::NVIC;
use defmt::{trace};
use stm32_metapac::{PWR,  otg};
use crate::otg_fs::global_states::{regs, restore_irqs};

pub fn usb_power_down() {
    PWR.svmcr().modify(|w| {
        w.set_usv(false); // RM0456 (rev 4) p 404. Romove Vddusb isolation
    });
}

pub fn power_up_init() {
    trace!("init");
    PWR.svmcr().modify(|w| {
        w.set_usv(true); // RM0456 (rev 4) p 404. Romove Vddusb isolation
    });
    stm32_metapac::RCC
        .ahb2enr1()
        .modify(|w| w.set_usb_otg_fsen(true));

    // enable usb uvm
    stm32_metapac::PWR.svmcr().modify(|w| {
        w.set_uvmen(true);
    });

    //Wait for USB power to stabilize
    while !stm32_metapac::PWR.svmsr().read().vddusbrdy() {}
    trace!("USB power stabilized");

    // enable HSI48
    stm32_metapac::RCC.cr().modify(|w| w.set_hsi48on(true));
    while !stm32_metapac::RCC.cr().read().hsi48rdy() {}

    // Select HSI48 as USB clock source.
    #[cfg(stm32u575)]
    critical_section::with(|_| {
        stm32_metapac::RCC.ccipr1().modify(|w| {
            w.set_iclksel(stm32_metapac::rcc::vals::Iclksel::HSI48);
        })
    });

    defmt::info!("USB clock source selected");

    unsafe {
        NVIC::unpend(stm32_metapac::Interrupt::OTG_FS);
        NVIC::unmask(stm32_metapac::Interrupt::OTG_FS);
        // start_irq();
        // Self::restore_irqs();
        restore_irqs();
        trace!("USB IRQs start");
    }
    let r = regs();
    let core_id = r.cid().read().0;
    trace!("Core id {:08x}", core_id);

    // Wait for AHB ready.
    while !r.grstctl().read().ahbidl() {}

    r.gusbcfg().write(|w| {
        w.set_fdmod(true); // Force device mode TODO: no host mode support
        // w.set_physel(self.phy_type.internal() && !self.phy_type.high_speed());
        // Enable internal full-speed PHY
    });

            r.gccfg_v2().modify(|w| {
                w.set_pwrdwn(true); // Enable internal full-speed PHY,
                // w.set_vbden(val: true); // vbus detect. these can used to save power.
            });
    r.gotgctl().modify(|w| {
        w.set_bvaloen(true);
        w.set_bvaloval(true);
    });

    // Soft disconnect.
    r.dctl().write(|w| w.set_sdis(true));

    // Set speed.
    r.dcfg().write(|w| {
        w.set_pfivl(otg::vals::Pfivl::FRAME_INTERVAL_80); // set period frame interval TODO: figure out what is this
        // #[cfg(stm32u575)]
        w.set_dspd(otg::vals::Dspd::FULL_SPEED_INTERNAL);
        // todo: for u5a5, this is different. 11 is reserved
    });

    // r.diepmsk().write(|w| {
    //     w.set_xfrcm(true); // Unmask transfer complete EP interrupt
    // });
    r.gintsts()
        .write_value(stm32_metapac::otg::regs::Gintsts(0xFFFF_FFFF));

    // Unmask global interrupt
    r.gahbcfg().write(|w| {
        w.set_dmaen(true); // Enable DMA
        // w.set_gintmsk(true); // unmask global interrupt
        w.set_gint(true); // unmask global interrupt
        //
    });

    defmt::info!("USB power up done");
    // clock::delay_ms(100);
    // Connect
    r.dctl().write(|w| w.set_sdis(false));
}
