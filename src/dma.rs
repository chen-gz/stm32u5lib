#![allow(unused)]

use crate::clock::delay_tick;
use crate::gpio::GpioPort;
use stm32_metapac::gpdma::vals;
use stm32_metapac::gpdma::vals::*;
use stm32_metapac::gpdma::Channel;
use stm32_metapac::gpdma::Gpdma;
use stm32_metapac::RCC;

pub struct DmaChannel {
    ins: Gpdma,
    ch: usize,
    request_source: u8,
    src_width: Dw, // ChTr1Dw,
    dst_width: Dw, // ChTr1Dw,
    tc_mode: Tcem, // ChTr2Tcem,
}
/// Determines the AP port based on the STM32U5 address space.
/// - PORT0 is used for peripherals: 0x4000_0000 to 0x5FFF_FFFF
/// - PORT1 is used for memory and other regions
fn get_ap_port_from_addr(addr: u32) -> Ap {
    if (0x4000_0000..=0x5FFF_FFFF).contains(&addr) {
        Ap::PORT0
    } else {
        Ap::PORT1
    }
}

// write a macro to define the dma channel
macro_rules! define_dma_channel {
    ($name:ident, $ins:ident, $ch:expr, $request_source:expr, $src_width:expr, $dst_width:expr, $tc_mode:expr) => {
        pub const $name: DmaChannel = DmaChannel {
            ins: stm32_metapac::$ins,
            ch: $ch,
            request_source: $request_source,
            src_width: $src_width,
            dst_width: $dst_width,
            tc_mode: $tc_mode,
        };
    };
}
// define the dma channel
define_dma_channel!(DMA_DCMI, GPDMA1, 0, 86, Dw::WORD, Dw::WORD, Tcem::LAST_LINKED_LIST_ITEM);
define_dma_channel!(DMA_USART1_RX, GPDMA1, 1, 24, Dw::BYTE, Dw::BYTE, Tcem::LAST_LINKED_LIST_ITEM);
define_dma_channel!(DMA_USART1_TX, GPDMA1, 2, 25, Dw::BYTE, Dw::BYTE, Tcem::LAST_LINKED_LIST_ITEM);
define_dma_channel!(DMA_USART2_RX, GPDMA1, 3, 26, Dw::BYTE, Dw::BYTE, Tcem::LAST_LINKED_LIST_ITEM);
define_dma_channel!(DMA_USART2_TX, GPDMA1, 4, 27, Dw::BYTE, Dw::BYTE, Tcem::LAST_LINKED_LIST_ITEM);
define_dma_channel!(DMA_USART3_RX, GPDMA1, 5, 28, Dw::BYTE, Dw::BYTE, Tcem::LAST_LINKED_LIST_ITEM);
define_dma_channel!(DMA_USART3_TX, GPDMA1, 6, 29, Dw::BYTE, Dw::BYTE, Tcem::LAST_LINKED_LIST_ITEM);
define_dma_channel!(DMA_USART4_RX, GPDMA1, 7, 30, Dw::BYTE, Dw::BYTE, Tcem::LAST_LINKED_LIST_ITEM);
define_dma_channel!(DMA_USART4_TX, GPDMA1, 8, 31, Dw::BYTE, Dw::BYTE, Tcem::LAST_LINKED_LIST_ITEM);
define_dma_channel!(DMA_USART5_RX, GPDMA1, 9, 32, Dw::BYTE, Dw::BYTE, Tcem::LAST_LINKED_LIST_ITEM);
define_dma_channel!(DMA_USART5_TX, GPDMA1, 10, 33, Dw::BYTE, Dw::BYTE, Tcem::LAST_LINKED_LIST_ITEM);

// pub const DCMI_DMA: DmaChannel = DmaChannel {
//     ins: stm32_metapac::GPDMA1,
//     ch: 0,
//     request_source: 86,
//     src_width: Dw::WORD, // 32 bits
//     dst_width: Dw::WORD,
//     // dst_width: ChTr1Dw::BYTE, // 8,
//     src_inc: false,
//     dst_inc: true,
//     src_type: Ap::PORT0,
//     dst_type: Ap::PORT1,
//     tc_mode: Tcem::LAST_LINKED_LIST_ITEM,
// };

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

const DMA_SINGLE_XFER: u32 = 65532; // single transfer max size
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
        });
        ch.tr2().modify(|v| {
            v.set_tcem(self.tc_mode);
            v.set_reqsel(self.request_source);
        });
    }
    pub async fn start(&self, src_addr: u32, src_inc: bool, dar_addr: u32,  dst_inc: bool, len: u32,) {
        self.init();
        let ch = self.ins.ch(self.ch);
        ch.tr1().modify(|v| {
            v.set_sinc(src_inc);
            v.set_dinc(dst_inc);
            v.set_sap(get_ap_port_from_addr(src_addr));
            v.set_dap(get_ap_port_from_addr(dar_addr));
        });
        assert!(len < DMA_SINGLE_XFER * 32);
        let link_list = unsafe { &mut LINK_LISTS[self.ch] };
        let num_link_list = (len + DMA_SINGLE_XFER - 1) / DMA_SINGLE_XFER;
        for i in 0..num_link_list as usize {
            link_list[i].sar = src_addr + (src_inc as u32) * (i as u32 * DMA_SINGLE_XFER);
            link_list[i].dar = dar_addr + (dst_inc as u32) * (i as u32 * DMA_SINGLE_XFER);
            link_list[i].llr = &link_list[i + 1] as *const _ as u32;
            link_list[i].llr &= 0x0000ffff;
            link_list[i].br1 = DMA_SINGLE_XFER;
            // let addr = &mut link_list.val[1] as *mut _ as u32;
            // set llr to update br(29), sar(28), dar(27), llr(16);
            link_list[i].llr |= (1 << 29) | (1 << 28) | (1 << 27) | (1 << 16);
            // handle last link list
            if i == num_link_list as usize - 1 {
                link_list[i].llr = 0;
                if len % DMA_SINGLE_XFER != 0 {
                    link_list[i].br1 = len % DMA_SINGLE_XFER;
                }
            }
        }
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
use embassy_sync::waitqueue::AtomicWaker;
static WAKER: AtomicWaker = AtomicWaker::new();
use stm32_metapac::interrupt;
use crate::hal::DMA;

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

use crate::hal;
impl hal::DMA for DmaChannel {

    async fn start(&self, src_addr: u32, src_inc: bool, dar_addr: u32, dst_inc: bool, len: u32) {
        self.start(src_addr, src_inc, dar_addr, dst_inc, len).await;
    }

    fn stop(&self) {
        self.stop();
    }
}