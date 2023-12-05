#![allow(unused)]

use crate::clock::{delay_ms, delay_tick, delay_us};
use defmt::export::panic;
use futures::future::OkInto;
use stm32_metapac::{common, sdmmc::Sdmmc};
pub struct SdmmcPort {
    port: Sdmmc,
}
pub const SD1: SdmmcPort = SdmmcPort {
    port: stm32_metapac::SDMMC1,
};

#[derive(Copy, Clone, Debug)]
pub enum SdError {
    STATUSError,
    CmdTimeout,
}

use crate::gpio::*;
use sdio_host::{common_cmd::Resp, common_cmd::ResponseLen, *};

pub struct SdInstance {
    port: Sdmmc,
    cid: sd::CID<sd::SD>,
    csd: sd::CSD<sd::SD>,
    rca: sd::RCA<sd::SD>,
}

impl SdInstance {
    pub fn new(sd: Sdmmc) -> Self {
        Self {
            port: sd,
            cid: sd::CID::default(),
            csd: sd::CSD::default(),
            rca: sd::RCA::default(),
        }
    }
    pub fn init(&mut self, clk: GpioPort, cmd: GpioPort, d0: GpioPort) {
        clk.setup();
        cmd.setup();
        d0.setup();
        delay_ms(100);

        // setup gpio ports
        // check clock 48Mhz as input clock
        self.port.clkcr().modify(|v| {
            v.set_clkdiv(60); // 48Mhz / (2 * clkdiv) = 48M / 120 = 400Khz
        });

        self.port.power().modify(|v| v.set_pwrctrl(3));
        delay_us(200);
        self.port.dtimer().modify(|v| {
            v.0 = 4000; // TODO: check this value and add comment about it
        });
        delay_ms(20);
        defmt::info!("start init sd card");
        // initilize sd card
        self.send_cmd(common_cmd::idle());
        self.send_cmd(common_cmd::idle());
        self.send_cmd(common_cmd::idle());
        self.send_cmd(common_cmd::idle());
        defmt::info!("send if cond");
        // let cmd = sd_cmd::send_if_cond(0xF, 0xa);
        let mut ok = true;
        for i in 0..20 {
            defmt::info!("send if conf {}", i);
            match self.send_cmd(sd_cmd::send_if_cond(0x1, 0xaa)) {
                Ok(_) => {
                    // read response
                    let resp0 = self.port.respr(0).read().0;
                    defmt::info!("send if conf response: {:x}", resp0);
                }
                Err(_) => defmt::info!("send if cond error"),
            }
            delay_ms(10);
        }

        if !ok {
            defmt::panic!("init sd card error: send if cond error");
        }

        ///////// repeat this untill the card is ready
        //
        ok = false;
        for i in 0..10 {
            // card not initialized, rca = 0
            defmt::info!("send app cmd");
            match self.send_cmd(common_cmd::app_cmd(0)) {
                Ok(_) => {
                    ok = true;
                }
                Err(_) => defmt::info!("send app cmd error"),
            }
            if (ok) {
                defmt::info!("send op cond");
                self.send_cmd(sd_cmd::sd_send_op_cond(true, true, false, 0xffff))
                    .unwrap();
                break;
            }
            delay_ms(1);
        }

        self.get_cid();
        defmt::info!("cid: {}", self.cid.product_name());

        self.get_rca();
        defmt::info!("rca: {}", self.rca.address());

        self.get_csd();
        defmt::info!("csd: {}", self.csd.block_count());

        defmt::info!("select card");
        self.send_cmd(common_cmd::select_card(self.rca.address()));

        // test cmd 23, whether
        defmt::info!("set block count");
        self.send_cmd(sd_cmd::set_block_count(1));
        // self.send_cmd(common_cmd::send_csd(self.csd.rc))
        // self.send_cmd(sd_cmd::send_scr());
    }
    pub fn test_r6() {
        let tmp: sd_cmd::R6;
        // tmp.response_len();
    }
    pub fn send_cmd<R: Resp>(&self, cmd: Cmd<R>) -> Result<(), SdError> {
        self.port.argr().write(|w| {
            w.0 = cmd.arg;
        });
        let res_len = match cmd.response_len() {
            ResponseLen::Zero => 0,
            ResponseLen::R48 => 1,
            ResponseLen::R136 => 2,
        };
        let mut trans = false;
        if cmd.cmd == common_cmd::read_single_block(0).cmd
            || cmd.cmd == common_cmd::write_single_block(0).cmd
            || cmd.cmd == common_cmd::write_multiple_blocks(0).cmd
            || cmd.cmd == common_cmd::read_multiple_blocks(0).cmd
        {
            trans = true;
        }
        defmt::info!(
            "cmd: {}, arg: {:x}, trans: {}, resp_len: {}",
            cmd.cmd,
            cmd.arg,
            trans,
            res_len
        );

        self.port.cmdr().modify(|v| {
            v.set_cmdindex(cmd.cmd);
            v.set_waitresp(res_len);
            v.set_cmdtrans(trans);
        });
        delay_tick(10);
        defmt::info!("cmdr: 0x{:x}", self.port.cmdr().read().0);
        self.port.cmdr().modify(|v| {
            v.set_cpsmen(true); // ennable command path state machine
        });
        // read sta
        let mut sta = self.port.star().read();

        while !self.port.star().read().cmdsent() {
            // defmt::info!("cmd not sent");
            if self.port.star().read().ctimeout() {
                return Err(SdError::CmdTimeout);
            }
            delay_ms(100);
            defmt::info!("cmd not send with sat {:x}", sta.0);
            defmt::info!("cpsmact {}", sta.cpsmact());
        }
        if cmd.response_len() == ResponseLen::Zero {
            return Ok(());
        }

        while !self.port.star().read().cmdrend() {
            // defmt::info!("cmd response not received");
            // get stat and print it
            if self.port.star().read().ctimeout() {
                defmt::info!("cmd response timeout");
                return Err(SdError::CmdTimeout);
            }
        }
        while self.port.star().read().cpsmact() {}

        // if self.port.respcmdr().read().0 != cmd.cmd as u32 {
        defmt::info!(
            "cmd response received, the send cmd is {:x}, the response cmd is {:x} \n stat is {:x}",
            cmd.cmd,
            self.port.respcmdr().read().0,
            self.port.star().read().0,
        );
        // clear the stat register
        self.port.icr().write(|v| v.0 = 0x1FE00FFF);

        defmt::info!("stat register cleared: {:x}", self.port.star().read().0);
        // }
        Ok(())
    }
    pub fn get_cid(&mut self) {
        self.send_cmd(common_cmd::all_send_cid()).unwrap();
        // read response
        let resp0 = self.port.respr(0).read().0;
        let resp1 = self.port.respr(1).read().0;
        let resp2 = self.port.respr(2).read().0;
        let resp3 = self.port.respr(3).read().0;
        self.cid = sd::CID::from([resp0, resp1, resp2, resp3]);
    }
    pub fn get_csd(&mut self) {
        self.send_cmd(common_cmd::send_csd(0)).unwrap();
        // read response
        let resp0 = self.port.respr(0).read().0;
        let resp1 = self.port.respr(1).read().0;
        let resp2 = self.port.respr(2).read().0;
        let resp3 = self.port.respr(3).read().0;
        self.csd = sd::CSD::from([resp0, resp1, resp2, resp3]);
    }
    pub fn get_rca(&mut self) {
        // self.send_cmd(sd_cmd::send_relative_address()).unwrap();
        let cmd: common_cmd::Cmd<sd_cmd::R6> = common_cmd::cmd(3, 0);
        self.send_cmd(cmd).unwrap();
        // self.send_cmd(common_cmd::cmd(3, 0xA)).unwrap();
        let resp0 = self.port.respr(0).read().0;
        defmt::info!("rca: {:x}", resp0);
        self.rca = sd::RCA::from(resp0);
    }

    pub fn read_single_block(&self, buf: &mut [u8; 512], block_addr: u64) -> Result<(), SdError> {
        // TODO: check
        if block_addr > self.csd.block_count() {
            return Err(SdError::STATUSError);
        }
        // TODO: check the address does not match to c code.
        self.port.idmabase0r().write(|v| v.0 = buf.as_ptr() as u32);
        self.port.idmactrlr().modify(|v| {
            v.set_idmabmode(false); // single buffer mode
            v.set_idmaen(true); // enable dma
        });
        self.port.dlenr().write(|v| v.0 = 512);

        self.port.dctrl().modify(|v| {
            v.set_fiforst(true); // reset fifo
            v.set_dblocksize(9); // 512 bytes
            v.set_dtmode(0); // block data transfer
            v.set_dtdir(true); // from card to controller
        });

        let mut cmd = common_cmd::read_single_block(block_addr as u32);
        self.send_cmd(cmd);
        Ok(())
    }

    pub fn write_single_block(&self, buf: &[u8; 512], block_addr: u32) -> Result<(), SdError> {
        // todo check
        if block_addr as u64 > self.csd.block_count() {
            return Err(SdError::STATUSError);
        }
        // TODO: check the address does not match to c code.

        self.port.idmabase0r().write(|v| v.0 = buf.as_ptr() as u32);
        self.port.idmactrlr().modify(|v| {
            v.set_idmabmode(false); // single buffer mode
            v.set_idmaen(true); // enable dma
        });
        self.port.dlenr().write(|v| v.0 = 512);

        self.port.dctrl().modify(|v| {
            v.set_fiforst(true); // reset fifo
            v.set_dblocksize(9); // 512 bytes
            v.set_dtmode(0); // block data transfer
            v.set_dtdir(false); // from controller to card
        });
        let cmd = common_cmd::write_single_block(block_addr);
        self.send_cmd(cmd);
        Ok(())
    }
    /// the maximum block count is (1<<24) = 16777216 bytes = 16MB
    pub fn read_multiple_blocks(
        &self,
        buf: &mut [u8],
        block_addr: u32,
        block_count: u32,
    ) -> Result<(), SdError> {
        // TODO: check
        self.port.idmabase0r().write(|v| v.0 = buf.as_ptr() as u32);
        self.port.idmactrlr().modify(|v| {
            v.set_idmabmode(false); // single buffer mode
            v.set_idmaen(true);
        });
        self.port.dlenr().write(|v| v.0 = block_count * 512);
        self.port.dctrl().modify(|v| {
            v.set_fiforst(true);
            v.set_dblocksize(9);
            v.set_dtmode(1); // stream data transfer
            v.set_dtdir(false);
        });
        let cmd = sd_cmd::set_block_count(block_count);
        self.send_cmd(cmd);
        let cmd = common_cmd::read_multiple_blocks(block_addr);
        self.send_cmd(cmd);
        Ok(())
    }
    pub fn write_multiple_blocks(
        &self,
        buf: &[u8],
        block_addr: u32,
        block_count: u32,
    ) -> Result<(), SdError> {
        // TODO: check
        self.port.idmabase0r().write(|v| v.0 = buf.as_ptr() as u32);
        self.port.idmactrlr().modify(|v| {
            v.set_idmabmode(false); // single buffer mode
            v.set_idmaen(true);
        });
        self.port.dlenr().write(|v| v.0 = block_count * 512);
        self.port.dctrl().modify(|v| {
            v.set_fiforst(true);
            v.set_dblocksize(9);
            v.set_dtmode(1); // stream data transfer
            v.set_dtdir(true);
        });
        let cmd = common_cmd::write_multiple_blocks(block_addr);
        self.send_cmd(cmd);
        Ok(())
    }
}
