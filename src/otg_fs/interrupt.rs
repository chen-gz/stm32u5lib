use core::sync::atomic::Ordering;
// use crate::otg_fs::mod_new::SETUP_DATA;
use defmt::{info, trace};
use stm32_metapac::interrupt;
use crate::otg_fs::global_states::{regs, State, state};
use crate::otg_fs::global_states::fifo_const::{RX_FIFO_SIZE_SIZE_WORD, TX_FIFO_SIZE_WORDS};

pub fn wakeup_all() {
    let state = state();
    for waker in state.ep_in_wakers.iter() {
        waker.wake();
    }
    for waker in state.ep_out_wakers.iter() {
        waker.wake();
    }
    state.bus_waker.wake();
}

pub static mut RESET: bool = false;

pub unsafe fn on_interrupt() {
    let r = regs();
    // defmt::info!("OTG_HS interrupt with ints {:08x}  and mask {:08x}, and {:08x}", r.gintsts().read().0, r.gintmsk().read().0, r.gintsts().read().0 & r.gintmsk().read().0);
    let ints = r.gintsts().read();
    if ints.wkupint() || ints.usbsusp() || ints.enumdne() || ints.otgint() || ints.srqint() || ints.usbrst()
    {
        if ints.wkupint() {
            info!("wkupint");
            r.gintsts().write(|w| w.set_wkupint(true)); // clear
        }
        else if  ints.usbsusp() {
            info!("usbsusp");
            wakeup_all();
            r.gintsts().write(|w| w.set_usbsusp(true)); // clear
        }
        else if ints.enumdne() {
            info!("enumdne");
            init_enumeration_done();

            r.gintsts().write(|w| w.set_enumdne(true)); // clear
        }
        else if ints.otgint() {
            info!("otgint");
            let otgints = r.gotgint().read();
            r.gotgint().write_value(otgints); // clear all
        }
        else if ints.srqint() {
            info!("srqint");
            r.gintsts().write(|w| w.set_srqint(true)); // clear
        }
        else if ints.usbrst() {
            info!("usbrst");
            unsafe {RESET = true};
            init_reset();
            // restart the control pipe task
            wakeup_all();
            r.gintsts().write(|w| w.set_usbrst(true)); // clear
            // mask this and
            crate::otg_fs::endpoint::init_endpoint();
        }
    }
    // let state = &STATE;
    let state: &mut State<6> = state();

    // Handle RX
    #[cfg(otg_fs)]
    while r.gintsts().read().rxflvl() {
        // RX FIFO non-empty
        let status = r.grxstsp().read();

        // status read and popo pop register
        let ep_num = status.epnum() as usize;
        let len = status.bcnt() as usize;
        info!("rxflvl with ep_num: {}, len: {}", ep_num, len);

        match status.pktstsd() {
            stm32_metapac::otg::vals::Pktstsd::SETUP_DATA_RX => {
                // get SETUP_DATA
                let data: u32 = r.fifo(0).read().0;
                let data2: u32 = r.fifo(0).read().0;
                for i in 0..4 {
                    SETUP_DATA[i] = (data >> (i * 8)) as u8;
                    SETUP_DATA[i + 4] = (data2 >> (i * 8)) as u8;
                }
                trace!("SETUP_DATA_RX, with data {:x}, {:x}, {:x}", data, data2, SETUP_DATA[0..8]);
                state.ep_out_wakers[ep_num].wake();
                state.ep0_setup_ready.store(true, Ordering::Release);
            }
            stm32_metapac::otg::vals::Pktstsd::OUT_DATA_RX => {
                // received OUT data
                // state.ep_out_size[ep_num].store(len as u16, Ordering::Release);
                state.ep_out_wakers[ep_num].wake();
                let len_words = (len + 3) / 4;
                let mut data = [0u8; 64];
                let mut index = 0;
                for _ in 0..len_words {
                    let tmp = r.fifo(0).read().data();
                    for i in 0..4 {
                        data[index] = (tmp >> (i * 8)) as u8;
                        index += 1;
                    }
                }
                trace!("OUT_DATA_RX ep={} len={}, data={:x}", ep_num, len, data[0..len]);
            }
            stm32_metapac::otg::vals::Pktstsd::OUT_DATA_DONE => {
                trace!("OUT_DATA_DONE ep={}", ep_num);
                r.doepctl(0).modify(|w| w.set_cnak(true));
            }
            stm32_metapac::otg::vals::Pktstsd::SETUP_DATA_DONE => {
                trace!("SETUP_DATA_DONE ep={}", ep_num);
                r.doepctl(0).modify(|w| w.set_cnak(true));
            }
            x => {
                trace!("unknown PKTSTS: {}", x.to_bits());
            }
        }
    }

    // IN endpoint interrupt
    if ints.iepint() {
        info!("iepint");
        let mut ep_mask = r.daint().read().iepint();
        let mut ep_num = 0;

        // Iterate over endpoints while there are non-zero bits in the mask
        while ep_mask != 0 {
            if ep_mask & 1 != 0 {
                // get the interrupt mask
                info!(
                    "device in endpoint interrupt mask: {:08x}",
                    r.diepmsk().read().0
                );

                // mask
                r.diepmsk().modify(|w| w.set_xfrcm(false));
                // r.diepmsk().modify(|w| w.set_tom(false));
                let ep_ints = r.diepint(ep_num).read();
                r.diepint(ep_num).write(|w| w.set_toc(true));

                // clear all
                // r.diepint(ep_num).write_value(ep_ints);

                // TXFE is cleared in DIEPEMPMSK
                // if ep_ints.txfe() {
                //     critical_section::with(|_| {
                //         r.diepempmsk().modify(|w| {
                //             w.set_ineptxfem(w.ineptxfem() & !(1 << ep_num));
                //         });
                //     });
                // }
                state.ep_in_wakers[ep_num].wake();
                trace!("in ep={} irq val={:08x}", ep_num, ep_ints.0);
            }

            ep_mask >>= 1;
            ep_num += 1;
        }
    }


    // OUT endpoint interrupt
    if ints.oepint() {
        let mut ep_mask = r.daint().read().oepint();
        let mut ep_num = 0;
        while ep_mask != 0 {
            if ep_mask & 1 != 0 {
                // show setup package

                // defmt::info!("------------------------------------------");
                // defmt::info!("oepint, ep_num: {},  intsts: {:08x}", ep_num, r.doepint(ep_num).read().0);
                // defmt::info!("setup data: {:x}", SETUP_DATA);
                // defmt::info!("doepctl: {:x}", regs().doepctl(ep_num).read().0);
                // defmt::info!("doeptsiz: {:x}", regs().doeptsiz(ep_num).read().0);
                let ep_ints = r.doepint(ep_num).read();
                if ep_ints.stup() {
                    state.ep_out_wakers[ep_num].wake();
                    state.ep0_setup_ready.store(true, Ordering::Release);
                    r.doepint(ep_num).write(|w| w.set_stup(true));
                    defmt::info!("setup package");
                } else if ep_ints.stsphsrx() {
                    // let status = r.grxstsp().read();
                    r.doepint(ep_num).write(|w| w.set_stsphsrx(true));
                    defmt::info!("clear stsphsrx+++++++++++++++++++++++++++++++++++++++");
                }
                // donothing if is setup and xfrc
                // clear all
                // r.doepint(ep_num).write_value(ep_ints);

                if ep_ints.xfrc() {
                    r.doepmsk().modify(|w| w.set_xfrcm(false)); // mask the interrupt and wake up the waker
                    // pop ?
                }
                state.ep_out_wakers[ep_num].wake();
            }
            ep_mask >>= 1;
            ep_num += 1;
        }
    }
}


#[cfg(otg_hs)]
#[interrupt]
fn OTG_HS() {
    unsafe {
        on_interrupt();
    }
}

pub fn init_enumeration_done() {
    // RM0456 Rev 5, p3423
    // 1. On the enumeration done interrupt, (ENUMDNE bit in OTG_GINTSTS register), read the otg_dsts register to get the speed of the enumeration.
    let _spd = regs().dsts().read().enumspd();
    // 2. program the mpsiz field in the otg_diepctl0 to set the maximum packet size. This step configures control endpoint 0.. The maximum packet size for a control depends on the enumeration speed.
    // todo: for now, we only support full speed and high speed
    // let mpsiz = match spd {
    //     otg::vals::Enumspd::HIGH_SPEED => 64,
    //     otg::vals::Enumspd::FULL_SPEED => 64,
    //     otg::vals::Enumspd::LOW_SPEED => 8,
    // };
    regs().diepctl(0).modify(|w| w.set_mpsiz(00));
    //
    // trace!("doepctl0: {:x}", regs().doepctl(0).read().0);
    // trace!("doepdma0: {:x}", regs().doepdma(0).read().0);
    // trace!("doeptsiz0: {:x}", regs().doeptsiz(0).read().0);
    // trace!("irq mask 0: {:x}", regs().gintmsk().read().0);
    // restore_irqs();
    // 3. for usb otg_hs in dma mode,  program the otg_doepctl0 register to enable control OUT endpoint 0, to receive a setup packet.
    // regs().doepctl(0).modify(|w| {
    //     w.set_cnak(true);
    //     w.set_epena(true);
    // });
}


pub fn init_reset() {
    // wake up all related task and send stall to all endpoints

    // in control pipe task, the control pipe will be waked up and set again


    // Rm0456 Rev 5, p3423
    let r = regs();
    // 1. set the NAK -- SNAK = 1 in OTG_DOEPCTLx register
    // r.doepctl(0).modify(|w| w.set_snak(true));
    // r.diepctl(0).modify(|w| w.set_snak(true));
    // for i in 0..6{
    //     r.doepctl(i).modify(|w| w.set_snak(true));
    //     r.diepctl(i).modify(|w| w.set_snak(true));
    // }
    // disable all the endpoint
    // r.dctl().modify(|w| w.set_sgonak(true));
    // for i in 1..6 {
    //     r.doepctl(i).modify(|w| w.set_epdis(true));
    //     r.diepctl(i).modify(|w| w.set_epdis(true));
    //     // r.diepctl(i).modify(|w| w.set_epdis(true));
    // }
    // // wait for the endpoint to be disabled
    // for i in 1..6 {
    //     while r.doepctl(i).read().epena() {}
    //     while r.diepctl(i).read().epena() {}
    // }
    // r.dctl().modify(|w| w.set_cgonak(false));

    // defmt::info!("ep=2, doepctl: {:x}", r.doepctl(2).read().0);
    // 2. unmask interrupt bits
    r.daintmsk().modify(|w| {
        w.set_iepm(w.iepm() | 1);
        w.set_oepm(w.oepm() | 1);
    });
    r.doepmsk().write(|w| {
        w.set_stupm(true);
        w.set_xfrcm(true);
    });
    r.diepmsk().write(|w| {
        w.set_xfrcm(true);
        w.set_tom(true);
    });
    // 3. set up data fifo ram for each of the fifo
    init_fifo();
    // the task should be start after this
}

pub fn init_fifo() {
    trace!("init_fifo");

    // let r = T::regs();
    let r = regs();

    r.grxfsiz().modify(|w| w.set_rxfd(RX_FIFO_SIZE_SIZE_WORD));

    for i in 0..TX_FIFO_SIZE_WORDS.len() {
        r.dieptxf(i).write(|w| {
            w.set_fd(TX_FIFO_SIZE_WORDS[i]);
            w.set_sa(RX_FIFO_SIZE_SIZE_WORD + TX_FIFO_SIZE_WORDS[0..i].iter().sum::<u16>());
        });
    }

    // Flush fifos
    r.grstctl().write(|w| {
        w.set_rxfflsh(true);
        w.set_txfflsh(true);
        w.set_txfnum(0x10);
    });
    // loop {
    //     let x = r.grstctl().read();
    //     if !x.rxfflsh() && !x.txfflsh() {
    //         break;
    //     }
    // }
}
