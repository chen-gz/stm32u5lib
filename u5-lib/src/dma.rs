#![allow(unused)]
use crate::clock::delay_tick;
use crate::gpio::GpioPort;
pub use stm32_metapac::gpdma::vals::*;
use stm32_metapac::gpdma::Channel;
use stm32_metapac::gpdma::Gpdma;

pub struct DmaChannel {
    ins: Gpdma,
    ch: usize,
    request_source: u8,
    src_width: ChTr1Dw,
    dst_width: ChTr1Dw,
    src_inc: bool,
    dst_inc: bool,
    src_type: ChTr1Ap, // port0 for prepherial, port1 for memory
    dst_type: ChTr1Ap,
    tc_mode: ChTr2Tcem,
}
pub const DCMI_DMA: DmaChannel = DmaChannel {
    ins: stm32_metapac::GPDMA1,
    ch: 0,
    request_source: 86,
    src_width: ChTr1Dw::WORD, // 32 bits
    dst_width: ChTr1Dw::WORD,
    // dst_width: ChTr1Dw::BYTE, // 8,
    src_inc: false,
    dst_inc: true,
    src_type: ChTr1Ap::PORT0,
    dst_type: ChTr1Ap::PORT1,
    tc_mode: ChTr2Tcem::LASTLINKEDLISTITEM,
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
    pub fn start(&self, src_addr: u32, dar_addr: u32, len: u32) {
        // buf: &[u8]) {
        self.init();
        assert!(len < DMA_MAX_TRANSFER_SIZE * 32);
        defmt::info!(
            "dma src_addr = {}, dst_addr = {}, len = {}",
            src_addr,
            dar_addr,
            len
        );
        let cnts = (len + DMA_MAX_TRANSFER_SIZE - 1) / DMA_MAX_TRANSFER_SIZE; // round up

        unsafe {
            let link_list = &mut LINK_LISTS[self.ch];
            // let link_list = &mut LINK_LISTS[self.ch];
            for i in 0..(len / DMA_MAX_TRANSFER_SIZE) as usize {
                if self.src_inc {
                    link_list[i].sar = src_addr + (i as u32 * DMA_MAX_TRANSFER_SIZE);
                } else {
                    link_list[i].sar = src_addr;
                }

                if self.dst_inc {
                    link_list[i].dar = dar_addr + (i as u32 * DMA_MAX_TRANSFER_SIZE);
                } else {
                    link_list[i].dar = dar_addr;
                }
                link_list[i].llr = &link_list[i + 1] as *const _ as u32;
                link_list[i].llr &= 0x0000ffff;
                link_list[i].br1 = DMA_MAX_TRANSFER_SIZE;
                // let addr = &mut link_list.val[1] as *mut _ as u32;
                // set llr to update br(29), sar(28), dar(27), llr(16);
                link_list[i].llr |= (1 << 29) | (1 << 28) | (1 << 27) | (1 << 16);
            }

            let mut last: usize = (len / DMA_MAX_TRANSFER_SIZE) as usize;
            if (len % DMA_MAX_TRANSFER_SIZE) != 0 {
                if self.src_inc {
                    link_list[last].sar = src_addr + (last as u32 * DMA_MAX_TRANSFER_SIZE);
                } else {
                    link_list[last].sar = src_addr;
                }
                if self.dst_inc {
                    link_list[last].dar = dar_addr + (last as u32 * DMA_MAX_TRANSFER_SIZE);
                } else {
                    link_list[last].dar = dar_addr;
                }
                link_list[last].br1 = len % DMA_MAX_TRANSFER_SIZE;
                // link_list[last] = ListNode {
                //     br1: 0,
                //     sar: 0,
                //     dar: 0,
                //     llr: 0,
                // };
                link_list[last].llr = 0;
            } else {
                link_list[last - 1].llr = 0;
            }
            let ch = self.ins.ch(self.ch);
            ch.lbar()
                .write(|v| v.set_lba(((link_list.as_ptr() as u32) >> 16) as u16));
            ch.sar().write_value(link_list[0].sar);
            ch.dar().write_value(link_list[0].dar);
            ch.br1().modify(|v| v.0 = link_list[0].br1);
            ch.llr().modify(|v| v.0 = link_list[0].llr);
            defmt::info!("dma sar = 0x{:x}", ch.sar().read());
            defmt::info!("dma dar = 0x{:x}", ch.dar().read());

            ch.cr().modify(|v| {
                v.set_en(true);
            });
        }
    }
    pub fn stop(&self) {
        let ch = self.ins.ch(self.ch);
        // ch.cr().modify(|v| {
        //     v.set_susp(true);
        // });
        ch.cr().modify(|v| {
            v.set_reset(true);
        });
        while ch.cr().read().en() {}
    }
}
