use cortex_m::peripheral::NVIC;
use defmt::{trace};
use stm32_metapac::{PWR, RCC, SYSCFG, otg};
use crate::usb_otg_hs::global_states::{regs, restore_irqs};

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
    #[cfg(otg_hs)]
    {
        critical_section::with(|_| {
            PWR.vosr().modify(|v| {
                v.0 |= (1 << 19) | (1 << 20);
                // SBPWREN and USBBOOSTEN in PWR_VOSR.
                // v.boosten();
            });
            crate::clock::delay_us(100);
            // delay_ms(100);
            // wait fo USBBOOSTRDY
            // while !pwr.vosr().read().usbboostrdy() {}
            // enable hse
            RCC.cr().modify(|w| {
                w.set_hseon(true);
            });
            // wait for hse ready
            while !RCC.cr().read().hserdy() {}

            RCC.ccipr2().modify(|w| {
                w.set_otghssel(stm32_metapac::rcc::vals::Otghssel::HSE);
            });

            RCC.apb3enr().modify(|w| {
                w.set_syscfgen(true);
            });
            RCC.ahb2enr1().modify(|w| {
                w.set_usb_otg_hs_phyen(true);
                w.set_usb_otg_hsen(true);
            });
            // TODO: update this clock settings
            SYSCFG.otghsphycr().modify(|v| {
                let hse_freq = unsafe { crate::clock::HSE_FREQ };
                if hse_freq == 26_000_000 {
                    v.set_clksel(0b1110);   // 26Mhz HSE
                } else if hse_freq == 16_000_000 {
                    v.set_clksel(0b0011); // 16Mhz HSE
                } else {
                    defmt::panic!("HSE frequency not supported");
                }

                v.set_en(true);
            });
        });
    }
    // Wait for USB power to stabilize
    while !stm32_metapac::PWR.svmsr().read().vddusbrdy() {}
    trace!("USB power stabilized");

    // Select HSI48 as USB clock source.
    // #[cfg(stm32u575)]
    // critical_section::with(|_| {
    //     stm32_metapac::RCC.ccipr1().modify(|w| {
    //         w.set_iclksel(stm32_metapac::rcc::vals::Iclksel::HSI48);
    //     })
    // });
    #[cfg(otg_hs)]
    critical_section::with(|_| {
        stm32_metapac::RCC.ccipr2().modify(|w| {
            w.set_otghssel(stm32_metapac::rcc::vals::Otghssel::HSE);
        })
    });
    // #[cfg(stm32u575)]
    // stm32_metapac::RCC
    //     .ahb2enr1()
    //     .modify(|w| w.set_usb_otg_fsen(true));
    //
    // #[cfg(stm32u575)]
    // unsafe {
    //     NVIC::unpend(stm32_metapac::Interrupt::OTG_FS);
    //     NVIC::unmask(stm32_metapac::Interrupt::OTG_FS);
    //     // start_irq();
    //     Self::restore_irqs();
    //     trace!("USB IRQs start");
    // }

    #[cfg(otg_hs)]
    unsafe {
        NVIC::unpend(stm32_metapac::Interrupt::OTG_HS);
        NVIC::unmask(stm32_metapac::Interrupt::OTG_HS);
        // start_irq();
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

    match core_id {
        // this is used to distinguish differnet stm32 chips
        0x0000_2000 | 0x0000_2100 | 0x0000_2300 | 0x0000_3000 | 0x0000_3100 => {
            // F446-like chips have the GCCFG.VBDEN bit with the opposite meaning
            r.gccfg_v2().modify(|w| {
                w.set_pwrdwn(true); // Enable internal full-speed PHY,
                // w.set_vbden(val: true); // vbus detect. these can used to save power.
            });
            // todo: vbus detection

            // Force B-peripheral session
            // r.gotgctl().modify(|w| {
            //     w.set_bvaloen(!self.config.vbus_detection); // B-peripheral session valid. Only  used as device
            //     w.set_bvaloval(true);
            // });
        }
        0x0000_5000 => {
            // U5A5
            r.gccfg_v2().modify(|w| {
                // w.set_pwrdwn(true);
                // w.set_vbden(self.config.vbus_detection);   // vbus detect. these can used to save power.
                // w.set_vbden(true);
                // w.set_phyhsen(true);
                w.0 = (1 << 24) | (1 << 23);
            });

            // Force B-peripheral session
            r.gusbcfg().modify(|w| w.set_trdt(0x09));
        }
        _ => unimplemented!("Unknown USB core id {:X}", core_id),
    }
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
        // w.set_dspd(phy_type.to_dspd()); // todo: for u5a5, this is different. 11 is reserved
        #[cfg(otg_hs)]
        // w.set_dspd(otg::vals::Dspd::FULL_SPEED_EXTERNAL);
        w.set_dspd(otg::vals::Dspd::HIGH_SPEED); // todo: for u5a5, this is different. 11 is reserved
    });

    // r.diepmsk().write(|w| {
    //     w.set_xfrcm(true); // Unmask transfer complete EP interrupt
    // });
    r.gintsts()
        .write_value(stm32_metapac::otg::regs::Gintsts(0xFFFF_FFFF));

    // Unmask global interrupt
    r.gahbcfg().write(|w| {
        w.set_dmaen(true); // Enable DMA
        w.set_gintmsk(true); // unmask global interrupt
        //
    });

    // Connect
    r.dctl().write(|w| w.set_sdis(false));
}

