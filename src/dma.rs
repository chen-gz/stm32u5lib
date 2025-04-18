#![allow(unused)]

use crate::clock::delay_tick;
use crate::gpio::GpioPort;
use heapless::arc_pool;
pub use stm32_metapac::gpdma::vals;
pub use stm32_metapac::gpdma::vals::*;
use stm32_metapac::gpdma::Channel;
use stm32_metapac::gpdma::Gpdma;

use stm32_metapac::RCC;

pub struct DmaChannel {
    ins: Gpdma,
    ch: usize,
    request_source: u8,
    src_width: Dw, // ChTr1Dw,
    dst_width: Dw, // ChTr1Dw,
    src_inc: bool,
    dst_inc: bool,
    src_type: Ap, //   // port0 for prepherial, port1 for memory
    dst_type: Ap,
    tc_mode: vals::Tcem, // ChTr2Tcem,
}
pub const DCMI_DMA: DmaChannel = DmaChannel {
    ins: stm32_metapac::GPDMA1,
    ch: 0,
    request_source: 86,
    src_width: Dw::WORD, // 32 bits
    dst_width: Dw::WORD,
    // dst_width: ChTr1Dw::BYTE, // 8,
    src_inc: false,
    dst_inc: true,
    src_type: Ap::PORT0,
    dst_type: Ap::PORT1,
    tc_mode: Tcem::LAST_LINKED_LIST_ITEM,
};

use crate::gpio::*;

#[repr(C)]
#[derive(Copy, Clone)]
struct ListNode {
    br1: u32,
    sar: u32,
    dar: u32,
    llr: u32,
}
type DmaList = [ListNode; 32];

type DmaLists = [DmaList; 10];

const DMA_MAX_TRANSFER_SIZE: u32 = 65532; // single transfer max size
static mut LINK_LISTS: DmaLists = [[ListNode {
    br1: 0,
    sar: 0,
    dar: 0,
    llr: 0,
}; 32]; 10];

impl DmaChannel {
    pub fn init(&self) {
        RCC.ahb1enr().modify(|v| v.set_gpdma1en(true));
        // setup gpio ports
        let ch = self.ins.ch(self.ch);
        ch.tr1().modify(|v| {
            v.set_sdw(self.src_width);
            v.set_ddw(self.dst_width);
            v.set_sinc(self.src_inc);
            v.set_dinc(self.dst_inc);
            v.set_sap(self.src_type);
            v.set_dap(self.dst_type);
        });
        ch.tr2().modify(|v| {
            v.set_tcem(self.tc_mode);
            v.set_reqsel(self.request_source);
        });
    }
    pub async fn start(&self, src_addr: u32, dar_addr: u32, len: u32) {
        // buf: &[u8]) {
        self.init();
        assert!(len < DMA_MAX_TRANSFER_SIZE * 32);
        let link_list = unsafe { &mut LINK_LISTS[self.ch] };
        let num_link_list = (len + DMA_MAX_TRANSFER_SIZE - 1) / DMA_MAX_TRANSFER_SIZE;
        for i in 0..num_link_list as usize {
            link_list[i].sar = src_addr + (self.src_inc as u32) * (i as u32 * DMA_MAX_TRANSFER_SIZE);
            link_list[i].dar = dar_addr + (self.dst_inc as u32) * (i as u32 * DMA_MAX_TRANSFER_SIZE);
            link_list[i].llr = &link_list[i + 1] as *const _ as u32;
            link_list[i].llr &= 0x0000ffff;
            link_list[i].br1 = DMA_MAX_TRANSFER_SIZE;
            // let addr = &mut link_list.val[1] as *mut _ as u32;
            // set llr to update br(29), sar(28), dar(27), llr(16);
            link_list[i].llr |= (1 << 29) | (1 << 28) | (1 << 27) | (1 << 16);
            // handle last link list
            if i == num_link_list as usize - 1 {
                link_list[i].llr = 0;
                if len % DMA_MAX_TRANSFER_SIZE != 0 {
                    link_list[i].br1 = len % DMA_MAX_TRANSFER_SIZE;
                }
            }
        }
        let ch = self.ins.ch(self.ch);
        ch.lbar() .write(|v| v.set_lba(((link_list.as_ptr() as u32) >> 16) as u16));
        ch.sar().write_value(link_list[0].sar);
        ch.dar().write_value(link_list[0].dar);
        ch.br1().modify(|v| v.0 = link_list[0].br1);
        ch.llr().modify(|v| v.0 = link_list[0].llr);

        ch.cr().modify(|v| {
            v.set_en(true);
        });

        // enable interrupt
        ch.cr().modify(|v| {
            v.set_tcie(true);
        });
        // wait for finished interrupt and return
        poll_fn(|cx| {
            WAKER.register(cx.waker()); // register waker
            // wait for the transfer complete
            if ch.sr().read().tcf() {
                ch.sr().write(|v| v.set_tcf(true));
                // clear the interrupt
                return core::task::Poll::Ready(());
            } else {
                return core::task::Poll::Pending;
            }
        });
    }
    pub fn stop(&self) {
        let ch = self.ins.ch(self.ch);
        ch.cr().modify(|v| {
            v.set_susp(true);
        });
        ch.cr().modify(|v| {
            v.set_reset(true);
        });
        while ch.cr().read().en() {}
    }
}
use core::future::poll_fn;

// waker
use core::cell::Cell;
use embassy_sync::waitqueue::AtomicWaker;
static WAKER: AtomicWaker = AtomicWaker::new();
use stm32_metapac::interrupt;
/// DMA interrupt handler
#[interrupt]
fn GPDMA1_CHANNEL0() {
    let ch = unsafe { stm32_metapac::GPDMA1.ch(0) };
    if ch.sr().read().tcf() {
        // ch.fcr().write(|v| v.set_tcf(true));
        // clear the interrupt
        // disable the interrupt
        ch.cr().modify(|v| {
            v.set_en(false);
            v.set_tcie(false);
        });
        WAKER.wake();
    }
}
