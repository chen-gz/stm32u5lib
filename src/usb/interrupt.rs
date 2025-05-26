use core::sync::atomic::Ordering;
use defmt::{error, trace};
use stm32_metapac::otg::{vals, Otg};
use crate::usb::{State, EP_OUT_BUFFER_EMPTY};

/// Handle interrupts.
pub unsafe fn on_interrupt<const MAX_EP_COUNT: usize>(r: Otg, state: &State<MAX_EP_COUNT>, ep_count: usize) {
    trace!("irq");

    let ints = r.gintsts().read();
    if ints.wkupint() || ints.usbsusp() || ints.usbrst() || ints.enumdne() || ints.otgint() || ints.srqint() {
        // Mask interrupts and notify `Bus` to process them
        r.gintmsk().write(|w| {
            w.set_iepint(true);
            w.set_oepint(true);
            w.set_rxflvlm(true);
        });
        state.bus_waker.wake();
    }

    // Handle RX
    while r.gintsts().read().rxflvl() {
        let status = r.grxstsp().read();
        trace!("=== status {:08x}", status.0);
        let ep_num = status.epnum() as usize;
        let len = status.bcnt() as usize;

        assert!(ep_num < ep_count);

        match status.pktstsd() {
            vals::Pktstsd::SETUP_DATA_RX => {
                trace!("SETUP_DATA_RX");
                assert!(len == 8, "invalid SETUP packet length={}", len);
                assert!(ep_num == 0, "invalid SETUP packet endpoint={}", ep_num);

                // flushing TX if something stuck in control endpoint
                if r.dieptsiz(ep_num).read().pktcnt() != 0 {
                    r.grstctl().modify(|w| {
                        w.set_txfnum(ep_num as _);
                        w.set_txfflsh(true);
                    });
                    while r.grstctl().read().txfflsh() {}
                }

                let data = &state.cp_state.setup_data;
                data[0].store(r.fifo(0).read().data(), Ordering::Relaxed);
                data[1].store(r.fifo(0).read().data(), Ordering::Relaxed);
            }
            vals::Pktstsd::OUT_DATA_RX => {
                trace!("OUT_DATA_RX ep={} len={}", ep_num, len);

                if state.ep_states[ep_num].out_size.load(Ordering::Acquire) == EP_OUT_BUFFER_EMPTY {
                    // SAFETY: Buffer size is allocated to be equal to endpoint's maximum packet size
                    // We trust the peripheral to not exceed its configured MPSIZ
                    let buf =
                        unsafe { core::slice::from_raw_parts_mut(*state.ep_states[ep_num].out_buffer.get(), len) };

                    for chunk in buf.chunks_mut(4) {
                        // RX FIFO is shared so always read from fifo(0)
                        let data = r.fifo(0).read().0;
                        chunk.copy_from_slice(&data.to_ne_bytes()[0..chunk.len()]);
                    }

                    state.ep_states[ep_num].out_size.store(len as u16, Ordering::Release);
                    state.ep_states[ep_num].out_waker.wake();
                } else {
                    error!("ep_out buffer overflow index={}", ep_num);

                    // discard FIFO data
                    let len_words = (len + 3) / 4;
                    for _ in 0..len_words {
                        r.fifo(0).read().data();
                    }
                }
            }
            vals::Pktstsd::OUT_DATA_DONE => {
                trace!("OUT_DATA_DONE ep={}", ep_num);
            }
            vals::Pktstsd::SETUP_DATA_DONE => {
                trace!("SETUP_DATA_DONE ep={}", ep_num);
            }
            x => trace!("unknown PKTSTS: {}", x.to_bits()),
        }
    }

    // IN endpoint interrupt
    if ints.iepint() {
        let mut ep_mask = r.daint().read().iepint();
        let mut ep_num = 0;

        // Iterate over endpoints while there are non-zero bits in the mask
        while ep_mask != 0 {
            if ep_mask & 1 != 0 {
                let ep_ints = r.diepint(ep_num).read();

                // clear all
                r.diepint(ep_num).write_value(ep_ints);

                // TXFE is cleared in DIEPEMPMSK
                if ep_ints.txfe() {
                    critical_section::with(|_| {
                        r.diepempmsk().modify(|w| {
                            w.set_ineptxfem(w.ineptxfem() & !(1 << ep_num));
                        });
                    });
                }

                state.ep_states[ep_num].in_waker.wake();
                trace!("in ep={} irq val={:08x}", ep_num, ep_ints.0);
            }

            ep_mask >>= 1;
            ep_num += 1;
        }
    }

    // out endpoint interrupt
    if ints.oepint() {
        trace!("oepint");
        let mut ep_mask = r.daint().read().oepint();
        let mut ep_num = 0;

        // Iterate over endpoints while there are non-zero bits in the mask
        while ep_mask != 0 {
            if ep_mask & 1 != 0 {
                let ep_ints = r.doepint(ep_num).read();
                // clear all
                r.doepint(ep_num).write_value(ep_ints);

                if ep_ints.stup() {
                    state.cp_state.setup_ready.store(true, Ordering::Release);
                }
                state.ep_states[ep_num].out_waker.wake();
                trace!("out ep={} irq val={:08x}", ep_num, ep_ints.0);
            }

            ep_mask >>= 1;
            ep_num += 1;
        }
    }
}

