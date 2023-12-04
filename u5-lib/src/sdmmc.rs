#![allow(unused)]

use defmt::export::panic;
use crate::clock::{delay_ms, delay_tick, delay_us};
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
    pub fn init(&mut self) {
        // setup gpio ports
        // check clock 48Mhz as input clock
        self.port.clkcr().modify(|v| {
            v.set_clkdiv(60); // 48Mhz / (2 * clkdiv) = 48M / 120 = 400Khz
        });

        self.port.power().modify(|v| v.set_pwrctrl(3));
        delay_us(200);
        self.port.dtimer().modify(|v| {
            v.0 = 8000;
        });
        delay_ms(20);
        // initilize sd card
        let mut cmd = common_cmd::idle();
        self.send_cmd_ctrl(cmd);
        let cmd = sd_cmd::send_if_cond(0xF, 0xa);
        self.send_cmd_ctrl(cmd);

        ///////// repeat this untill the card is ready
        //
        // card not initialized, rca = 0
        self.send_cmd_ctrl(common_cmd::app_cmd(0));
        let cmd = sd_cmd::sd_send_op_cond(true, true, false, 0xffff);
        self.send_cmd_ctrl(cmd);
        ////////// end of repeat

        self.send_cmd_ctrl(common_cmd::all_send_cid());

        self.send_cmd_ctrl(sd_cmd::send_relative_address()); // the length of R6 is not defined. the
                                                        // function may cause issue

        self.send_cmd_ctrl(common_cmd::send_csd(self.rca.address()));

        self.send_cmd_ctrl(common_cmd::select_card(self.rca.address()));

        // test cmd 23, whether
        self.send_cmd_ctrl(sd_cmd::set_block_count(1));
        // self.send_cmd(common_cmd::send_csd(self.csd.rc))
        // self.send_cmd(sd_cmd::send_scr());
    }
    pub fn test_r6() {
        let tmp: sd_cmd::R6;
        // tmp.response_len();
    }
    // the cmd should be common_cmd::read_single_block, common_cmd::write_single_block,
    // common_cmd::write_multiple_blocks, common_cmd::read_multiple_blocks
    pub fn send_cmd_data<R: Resp>(&self, cmd: Cmd<R>) -> Result<(), SdError> {
        self.port.argr().write(|w| {
            w.0 = cmd.arg;
        });
        let res_len = match cmd.response_len() {
            ResponseLen::Zero => 0,
            ResponseLen::R48 => 1,
            ResponseLen::R136 => 3,
        };
        let mut trans = false;
        if cmd.cmd == common_cmd::read_single_block(0).cmd
            || cmd.cmd == common_cmd::write_single_block(0).cmd
            || cmd.cmd == common_cmd::write_multiple_blocks(0).cmd
            || cmd.cmd == common_cmd::read_multiple_blocks(0).cmd
        {
            trans = true;
        }

        self.port.cmdr().modify(|v| {
            v.set_cmdindex(cmd.cmd);
            v.set_waitresp(res_len);
            v.set_cmdtrans(trans); // TODO: update this
        });
        // ennable command path state machine
        self.port.cmdr().modify(|v| {
            v.set_cmdtrans(true);
        });
        // read sta
        let mut sta = self.port.star().read();
        match cmd.response_len() {
            ResponseLen::Zero => {
                // wait for cmdsent
                while !self.port.star().read().cmdsent() {}
            }
            ResponseLen::R48 => {
                panic!("wrong cmd");
            }
            ResponseLen::R136 => {
                panic!("wrong cmd");
            }
        }
        Ok(())
    }
    pub fn send_cmd_ctrl<R: Resp>(&mut self, cmd: Cmd<R>) -> Result<(), SdError> {
        self.port.argr().write(|w| {
            w.0 = cmd.arg;
        });
        let res_len = match cmd.response_len() {
            ResponseLen::Zero => 0,
            ResponseLen::R48 => 1,
            ResponseLen::R136 => 3,
        };
        let mut trans = false;
        if cmd.cmd == common_cmd::read_single_block(0).cmd
            || cmd.cmd == common_cmd::write_single_block(0).cmd
            || cmd.cmd == common_cmd::write_multiple_blocks(0).cmd
            || cmd.cmd == common_cmd::read_multiple_blocks(0).cmd
        {
            trans = true;
        }

        self.port.cmdr().modify(|v| {
            v.set_cmdindex(cmd.cmd);
            v.set_waitresp(res_len);
            v.set_cmdtrans(trans); // TODO: update this
        });
        // ennable command path state machine
        self.port.cmdr().modify(|v| {
            v.set_cmdtrans(true);
        });
        // read sta
        let mut sta = self.port.star().read();
        match cmd.response_len() {
            ResponseLen::Zero => {
                // wait for cmdsent if no response
                while !self.port.star().read().cmdsent() {}
            }
            ResponseLen::R48 => {
                while !self.port.star().read().cmdrend() {}
                // handle short response
                let resp = self.port.respr(0).read().0;
                let cs: sd::CardStatus<sd::SD> = sd::CardStatus::from(resp);
                if cs.error() {
                    panic!("sd card error");
                }
                // TODO: check r3 response ?
                if cmd.cmd == sd_cmd::send_relative_address().cmd {
                    defmt::info!("receive r3 response! ");
                    self.rca = sd::RCA::from(resp);
                }
            }
            ResponseLen::R136 => {
                while !self.port.star().read().cmdrend() {}
                let resp0 = self.port.respr(0).read().0;
                let resp1 = self.port.respr(1).read().0;
                let resp2 = self.port.respr(2).read().0;
                let resp3 = self.port.respr(3).read().0;
                if cmd.cmd == common_cmd::send_csd(0).cmd {
                    let csd: sd::CSD<sd::SD> = sd::CSD::from([resp0, resp1, resp2, resp3]);
                    defmt::info!("csd: {:?}", csd);
                    self.csd = csd;
                } else if cmd.cmd == common_cmd::send_cid(0).cmd {
                    let cid: sd::CID<sd::SD> = sd::CID::from([resp0, resp1, resp2, resp3]);
                    defmt::info!("cid: {:?}", cid);
                    self.cid = cid;
                }
            }
        }
        Ok(())
    }

    pub fn read_single_block(
        &self,
        buf: &mut [u8; 512],
        block_addr: u64,
    ) -> Result<(), SdError> {
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
        self.send_cmd_data(cmd);
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
        self.send_cmd_data(cmd);
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
        self.send_cmd_data(cmd);
        let cmd = common_cmd::read_multiple_blocks(block_addr);
        self.send_cmd_data(cmd);
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
        self.send_cmd_data(cmd);
        Ok(())
    }
}
