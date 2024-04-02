#![allow(unused)]

use core::ptr::read;

use crate::clock;
use crate::clock::{delay_ms, delay_tick, delay_us};
use cortex_m::delay;
use cortex_m::peripheral::NVIC;
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::signal;
use embassy_sync::signal::Signal;
use futures::future::poll_fn;
use futures::future::OkInto;
use sdio_host::common_cmd::cmd;
use stm32_metapac::rcc;
use stm32_metapac::RCC;
pub use stm32_metapac::SDMMC2;
use stm32_metapac::{common, sdmmc::Sdmmc};
// macro_rules! wait_for_event {
//     ($fn_name:ident, $event:ident) => {
//         pub async fn $fn_name(&self) -> Result<(), SdError> {
//             // enable interrupt
//             poll_fn(|cx| {
//                 self.set_waker(cx.waker().clone());
//                 match self.error_test() {
//                     Ok(_) => {}
//                     Err(err) => {
//                         self.clear_interrupt();
//                         return core::task::Poll::Ready(Err(err));
//                     }
//                 }
//                 if self.port.star().read().$event() {
//                     self.clear_interrupt();
//                     return core::task::Poll::Ready(Ok(()));
//                 }
//                 core::task::Poll::Pending
//             })
//             .await
//         }
//     };
// }

// Usage

#[derive(Copy, Clone)]
pub struct SdmmcPort {
    port: Sdmmc,
}

// import waker
use core::future::Future;

pub const SD1: SdmmcPort = SdmmcPort {
    port: stm32_metapac::SDMMC1,
};

use defmt::Format;

#[derive(Copy, Clone, Debug, Format)]
pub enum SdError {
    WriteBlockCountError,
    WriteAddressError,
    ReadBlockCountError,

    STATUSError,
    CmdCrcFail,
    DataCrcFail,
    CmdTimeout,
    DataTimeout,
    TxUnderrun,
    RxOverrun,
    // CmdRespEndBitErr,
    DataHold,
    BusyD0,
    AckFail,
    AckTimeOut,
}

use crate::gpio::*;
use sdio_host::{
    common_cmd::ResponseLen,
    common_cmd::{card_status, Resp},
    *,
};

#[derive(Copy, Clone)]
pub struct SdInstance {
    port: Sdmmc,
    cid: sd::CID<sd::SD>,
    csd: sd::CSD<sd::SD>,
    rca: sd::RCA<sd::SD>,
}

unsafe impl Send for SdInstance {}

unsafe impl Sync for SdInstance {}

impl SdInstance {
    pub fn new(sd: Sdmmc) -> Self {
        Self {
            port: sd,
            cid: sd::CID::default(),
            csd: sd::CSD::default(),
            rca: sd::RCA::default(),
        }
    }
    pub fn error_test(&self) -> Result<(), SdError> {
        let sta = self.port.star().read();
        if sta.ccrcfail() {
            return Err(SdError::CmdCrcFail);
        }
        if sta.dcrcfail() {
            return Err(SdError::DataCrcFail);
        }
        if sta.ctimeout() {
            return Err(SdError::CmdTimeout);
        }
        if sta.dtimeout() {
            return Err(SdError::DataTimeout);
        }
        if sta.txunderr() {
            return Err(SdError::TxUnderrun);
        }
        if sta.rxoverr() {
            return Err(SdError::RxOverrun);
        }
        if sta.ackfail() {
            return Err(SdError::AckFail);
        }
        if sta.acktimeout() {
            return Err(SdError::AckTimeOut);
        }
        Ok(())
    }

    pub fn error_test_async(&self, sta: stm32_metapac::sdmmc::regs::Star) -> Result<(), SdError> {
        if sta.ccrcfail() {
            return Err(SdError::CmdCrcFail);
        }
        if sta.dcrcfail() {
            return Err(SdError::DataCrcFail);
        }
        if sta.ctimeout() {
            return Err(SdError::CmdTimeout);
        }
        if sta.dtimeout() {
            return Err(SdError::DataTimeout);
        }
        if sta.txunderr() {
            return Err(SdError::TxUnderrun);
        }
        if sta.rxoverr() {
            return Err(SdError::RxOverrun);
        }
        if sta.ackfail() {
            return Err(SdError::AckFail);
        }
        if sta.acktimeout() {
            return Err(SdError::AckTimeOut);
        }
        Ok(())
    }
    pub fn init(&mut self, clk: GpioPort, cmd: GpioPort, d0: GpioPort, d1: GpioPort, d2: GpioPort, d3: GpioPort, d4: GpioPort, d5: GpioPort, d6: GpioPort, d7: GpioPort) {
        self.init_emmc(clk, cmd, d0, d1, d2, d3, d4, d5, d6, d7);
    }

    pub fn init_emmc(&mut self, clk: GpioPort, cmd: GpioPort, d0: GpioPort, d1: GpioPort, d2: GpioPort, d3: GpioPort, d4: GpioPort, d5: GpioPort, d6: GpioPort, d7: GpioPort) {
        clk.setup();
        cmd.setup();
        d0.setup();
        d1.setup();d2.setup();d3.setup();d4.setup();d5.setup();d6.setup();d7.setup();

        clock::set_sdmmc_clock(self.port, rcc::vals::Sdmmcsel::ICLK).unwrap();

        delay_ms(10); // TODO: chekc this value

        // setup gpio ports
        // check clock 48Mhz as input clock
        self.port.clkcr().modify(|v| {
            // v.set_clkdiv(60); // 48Mhz / (2 * clkdiv) = 48M / 120 = 400Khz
            // v.set_clkdiv(24); // 48Mhz / (2 * clkdiv) = 48M / 48 = 1Mhz
            // v.set_clkdiv(6); // 48Mhz / (2 * clkdiv) = 48M / 12 = 4Mhz
            // v.set_clkdiv(3); // 48Mhz / (2 * clkdiv) = 48M / 4 = 12Mhz
            v.set_clkdiv(1); // 48Mhz / (2 * clkdiv) = 48M / 2 = 24Mhz
        });

        self.port.power().modify(|v| v.set_pwrctrl(3));
        delay_ms(1); // 400khz, 74clk = 185us
        self.port.dtimer().modify(|v| {
            v.0 = 5 * 400_0000; // 400khz, 8000clk = 20ms
            // 12Mhz, 20_000_000 clk = 20/12 = 1.67s
        });
        defmt::info!("start init sd card");
        // initilize sd card
        match self.send_cmd(common_cmd::idle()) {
            Ok(_) => {}
            Err(err) => defmt::panic!("init sd card error: {:?}", err),
        }
        // let cmd = sd_cmd::send_if_cond(0xF, 0xa);
        let mut ok = false;
        loop {
            match self.send_cmd(emmc_cmd::send_op_cond((0x1FF << 15) | (1 << 7))) {
                Ok(_) => {
                    // read response
                    let resp0 = self.port.respr(0).read().0;
                    defmt::debug!("send if conf response: {:x}", resp0);
                    if resp0 & (1 << 31) != 0 {
                        defmt::info!("card is ready");
                        ok = true;
                        break;
                    }
                }
                Err(err) => defmt::panic!("send if cond error {}", err),
            }
        }

        match self.get_cid() {
            Ok(_) => {
                defmt::info!("cid: {}", self.cid.manufacturer_id());
            }
            Err(err) => defmt::error!(
                    "get cid error: {:?}, sta: {:x}",
                    err,
                    self.port.star().read().0
                ),
        }
        match self.get_rca() {
            Ok(_) => {}
            Err(err) => defmt::panic!("get rca error: {:?}", err),
        }
        defmt::info!("rca: {}", self.rca.address());

        match self.get_csd() {
            Ok(_) => {}
            Err(err) => defmt::panic!("get csd error: {:?}", err),
        }
        defmt::info!("csd: {}", self.csd.block_count());

        defmt::info!("select card");

        match self.send_cmd(common_cmd::select_card(self.rca.address())) {
            Ok(_) => {}
            Err(err) => defmt::panic!("select card error: {:?}", err),
        }

        // test cmd 23, whether
        defmt::info!("set block count");
        match self.send_cmd(sd_cmd::set_block_count(1)) {
            Ok(_) => {}
            Err(err) => defmt::panic!("set block count error: {:?}", err),
        }
        defmt::info!(
            "card version {}, The numbe of block of card: {}, self.port.dlenr: {}",
            self.csd.version(),
            self.csd.block_count(), // 125_042_688 * 500 =  62_521_344_000 = 62.5GB
            self.port.dlenr().read().0,
            // self.csd.block_length()
        );
        // self.send_cmd(common_cmd::send_csd(self.csd.rc))
        // self.send_cmd(sd_cmd::send_scr());

        // now let's switch to 8 bits mode
        self.send_cmd(sd_cmd::cmd6(0xcb70200)).unwrap();

    }

    pub fn init_sd_card(&mut self, clk: GpioPort, cmd: GpioPort, d0: GpioPort) {
        clock::set_sdmmc_clock(self.port, rcc::vals::Sdmmcsel::ICLK).unwrap();

        clk.setup();
        cmd.setup();
        d0.setup();
        delay_ms(100); // TODO: chekc this value

        // setup gpio ports
        // check clock 48Mhz as input clock
        self.port.clkcr().modify(|v| {
            // v.set_clkdiv(60); // 48Mhz / (2 * clkdiv) = 48M / 120 = 400Khz
            // v.set_clkdiv(24); // 48Mhz / (2 * clkdiv) = 48M / 48 = 1Mhz
            // v.set_clkdiv(6); // 48Mhz / (2 * clkdiv) = 48M / 12 = 4Mhz
            v.set_clkdiv(3); // 48Mhz / (2 * clkdiv) = 48M / 4 = 12Mhz
        });

        self.port.power().modify(|v| v.set_pwrctrl(3));
        delay_ms(10); // 400khz, 74clk = 185us
        self.port.dtimer().modify(|v| {
            v.0 = 5 * 400_0000; // 400khz, 8000clk = 20ms
            // 12Mhz, 20_000_000 clk = 20/12 = 1.67s
        });
        defmt::info!("start init sd card");
        // initilize sd card
        match self.send_cmd(common_cmd::idle()) {
            Ok(_) => {}
            Err(err) => defmt::panic!("init sd card error: {:?}", err),
        }
        // let cmd = sd_cmd::send_if_cond(0xF, 0xa);
        let mut ok = false;
        for i in 0..50 {
            defmt::debug!("send if conf {}", i);
            match self.send_cmd(sd_cmd::send_if_cond(0x1, 0xaa)) {
                Ok(_) => {
                    // read response
                    let resp0 = self.port.respr(0).read().0;
                    defmt::debug!("send if conf response: {:x}", resp0);
                    ok = true;
                    break;
                }
                Err(err) => defmt::error!("send if cond error {}", err),
            }
            delay_ms(10);
        }

        if !ok {
            defmt::panic!("init sd card error: send if cond error");
        }

        ///////// repeat this untill the card is ready
        //
        ok = false;
        for i in 0..20 {
            // card not initialized, rca = 0
            defmt::debug!("send app cmd");
            match self.send_cmd(common_cmd::app_cmd(0)) {
                Ok(_) => {
                    // get response and print it
                    let resp0 = self.port.respr(0).read().0;
                    ok = true;
                    defmt::debug!("app cmd response: {:x}", resp0);
                }
                Err(err) => {
                    defmt::error!("send app cmd error: {:?}", err);
                    ok = false
                }
            }
            if (ok) {
                ok = false;
                defmt::info!("send op cond");
                // match self.send_cmd(sd_cmd::sd_send_op_cond(true, false, false, 1 << 5)) {
                //     Ok(_) => {
                //         ok = true;
                //         defmt::info!("send op cmd success");
                //     }
                //     Err(err) => defmt::error!("send op cond error: {:?}", err),
                // }
                match self.send_cmd(sd_cmd::sd_send_op_cond(true, false, false, (1 << 5))) {
                    Ok(_) => {
                        let resp0 = self.port.respr(0).read().0;
                        let ocr: sd::OCR<sd::SD> = sd::OCR::from(resp0);
                        if !ocr.is_busy() {
                            ok = true;
                            defmt::info!("send op cmd success with resp: {:x}", resp0);
                            break;
                        }
                    }
                    Err(err) => defmt::error!("send op cond error: {:?}", err),
                }
            }
            if ok {
                break;
            }
            delay_ms(10);
        }
        if !ok {
            defmt::panic!("init sd card error: send op cond error");
        }

        ok = false;
        for i in 0..20 {
            match self.get_cid() {
                Ok(_) => {
                    ok = true;
                    break;
                }
                Err(err) => defmt::error!(
                    "get cid error: {:?}, sta: {:x}",
                    err,
                    self.port.star().read().0
                ),
            }
            delay_ms(10);
        }
        if (!ok) {
            defmt::panic!("init sd card error: get cid error");
        }
        defmt::info!("cid: {}", self.cid.manufacturer_id());

        match self.get_rca() {
            Ok(_) => {}
            Err(err) => defmt::panic!("get rca error: {:?}", err),
        }
        defmt::info!("rca: {}", self.rca.address());

        match self.get_csd() {
            Ok(_) => {}
            Err(err) => defmt::panic!("get csd error: {:?}", err),
        }
        defmt::info!("csd: {}", self.csd.block_count());

        defmt::info!("select card");

        match self.send_cmd(common_cmd::select_card(self.rca.address())) {
            Ok(_) => {}
            Err(err) => defmt::panic!("select card error: {:?}", err),
        }

        // test cmd 23, whether
        defmt::info!("set block count");
        match self.send_cmd(sd_cmd::set_block_count(1)) {
            Ok(_) => {}
            Err(err) => defmt::panic!("set block count error: {:?}", err),
        }
        defmt::info!(
            "card version {}, The numbe of block of card: {}, self.port.dlenr: {}",
            self.csd.version(),
            self.csd.block_count(), // 125_042_688 * 500 =  62_521_344_000 = 62.5GB
            self.port.dlenr().read().0
        );
        // self.send_cmd(common_cmd::send_csd(self.csd.rc))
        // self.send_cmd(sd_cmd::send_scr());
    }
    pub fn block_count(&self) -> u32 {
        self.csd.block_count() as u32
    }
    pub fn send_cmd<R: Resp>(&self, cmd: Cmd<R>) -> Result<(), SdError> {
        self.port.icr().write(|v| v.0 = 0x1FE00FFF);
        delay_us(1); // at least seven sdmmc_hcli clock peirod are needed between two write access
                     // 7clk, 160mhz, 44.4ns, 444ns
                     // to the cmdr register
        self.port.argr().write(|w| w.0 = cmd.arg);
        let res_len = match cmd.response_len() {
            ResponseLen::Zero => 0,
            ResponseLen::R48 => {
                let mut val = 1;
                if cmd.cmd == sd_cmd::sd_send_op_cond(true, false, false, 0).cmd
                    || cmd.cmd == emmc_cmd::send_op_cond(0).cmd
                {
                    val = 2; // no crc for this command
                }
                val
            }
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
        let mut stop = false;
        if cmd.cmd == common_cmd::idle().cmd || cmd.cmd == common_cmd::stop_transmission().cmd {
            stop = true;
        }
        self.port.cmdr().write(|v| {
            v.set_cmdindex(cmd.cmd);
            v.set_waitresp(res_len);
            v.set_cmdtrans(trans);
            v.set_cmdstop(stop);
            v.set_cpsmen(true); // ennable command path state machine
        });
        while !self.port.star().read().cmdsent() && !self.port.star().read().cmdrend() {
            self.error_test()?;
            delay_us(10);
        }
        if cmd.response_len() == ResponseLen::Zero {
            return Ok(());
        }

        while !self.port.star().read().cmdrend() {
            self.error_test()?;
            delay_ms(1);
        }
        while self.port.star().read().cpsmact() {}

        // handle R1 response
        // if not r1 response handle outsize
        let resp0 = self.port.respr(0).read().0;
        if res_len != 1 {
            return Ok(());
        }
        if cmd.cmd == common_cmd::set_block_length(0).cmd
            || cmd.cmd == common_cmd::card_status(0, false).cmd
            || cmd.cmd == common_cmd::write_single_block(0).cmd
            || cmd.cmd == common_cmd::write_multiple_blocks(0).cmd
            || cmd.cmd == common_cmd::read_single_block(0).cmd
            || cmd.cmd == common_cmd::read_multiple_blocks(0).cmd
            || cmd.cmd == common_cmd::app_cmd(0).cmd
        {
            let cs: sd::CardStatus<sd::SD> = sd::CardStatus::from(resp0);
            if (cs.error()) {
                defmt::info!(
                    "cmd: {}, param: {:x}, res_len: {}",
                    cmd.cmd,
                    cmd.arg,
                    res_len
                );
                defmt::info!("cmd: {}, param: {:x}", cmd.cmd, cmd.arg);
                defmt::error!("card status error: {:x}", resp0);
                defmt::error!(
                    "card status errs: out of range: {},
                address error: {},
                block len error: {},
                erase seq error: {},
                lock unlock failed: {},
                com crc error: {},
                illegal command: {},
                card ecc failed: {},
                cc error: {},
                error: {},
                csd overwrite: {},
                wp erase skip: {},
                erase reset: {}",
                    cs.out_of_range(),
                    cs.address_error(),
                    cs.block_len_error(),
                    cs.erase_seq_error(),
                    cs.lock_unlock_failed(),
                    cs.com_crc_error(),
                    cs.illegal_command(),
                    cs.card_ecc_failed(),
                    cs.cc_error(),
                    cs.error(),
                    cs.csd_overwrite(),
                    cs.wp_erase_skip(),
                    cs.erase_reset(),
                );
                return Err(SdError::STATUSError);
            }
        }
        Ok(())
    }

    pub async fn send_cmd_async<R: Resp>(&self, cmd: Cmd<R>) -> Result<(), SdError> {
        self.port.icr().write(|v| v.0 = 0x1FE00FFF);
        delay_us(1); // at least seven sdmmc_hcli clock peirod are needed between two write access
        // 7clk, 160mhz, 44.4ns, 444ns
        // to the cmdr register
        self.port.argr().write(|w| w.0 = cmd.arg);
        let res_len = match cmd.response_len() {
            ResponseLen::Zero => 0,
            ResponseLen::R48 => {
                let mut val = 1;
                if cmd.cmd == sd_cmd::sd_send_op_cond(true, false, false, 0).cmd {
                    val = 2; // no crc for this command
                }
                val
            }
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
        let mut stop = false;
        if cmd.cmd == common_cmd::idle().cmd || cmd.cmd == common_cmd::stop_transmission().cmd {
            stop = true;
        }
        self.port.cmdr().write(|v| {
            v.set_cmdindex(cmd.cmd);
            v.set_waitresp(res_len);
            v.set_cmdtrans(trans);
            v.set_cmdstop(stop);
            v.set_cpsmen(true); // ennable command path state machine
        });
        let mut stat = stm32_metapac::sdmmc::regs::Star(SIGNAL2.wait().await);

        while !stat.cmdrend() && !stat.cmdsent() {
            self.error_test_async(stat)?;
            stat = stm32_metapac::sdmmc::regs::Star(SIGNAL2.wait().await);
        }

        if cmd.response_len() == ResponseLen::Zero {
            return Ok(());
        }
        while !stat.cmdrend() {
            self.error_test_async(stat)?;
            stat = stm32_metapac::sdmmc::regs::Star(SIGNAL2.wait().await);
        }
        while self.port.star().read().cpsmact() {} // wait for cpsm idle -- TODO: check this

        if cmd.cmd == common_cmd::write_multiple_blocks(0).cmd
            || cmd.cmd == common_cmd::read_multiple_blocks(0).cmd
            || cmd.cmd == common_cmd::write_single_block(0).cmd
            || cmd.cmd == common_cmd::read_single_block(0).cmd
        {
            // defmt::info!("write multiple blocks");
            while (stat.dataend() == false) {
                self.error_test_async(stat)?;
                stat = stm32_metapac::sdmmc::regs::Star(SIGNAL2.wait().await);
            }
        }
        // handle R1 response
        // if not r1 response handle outsize
        let resp0 = self.port.respr(0).read().0;
        if res_len != 1 {
            return Ok(());
        }
        if cmd.cmd == common_cmd::set_block_length(0).cmd
            || cmd.cmd == common_cmd::card_status(0, false).cmd
            || cmd.cmd == common_cmd::write_single_block(0).cmd
            || cmd.cmd == common_cmd::write_multiple_blocks(0).cmd
            || cmd.cmd == common_cmd::read_single_block(0).cmd
            || cmd.cmd == common_cmd::read_multiple_blocks(0).cmd
            || cmd.cmd == common_cmd::app_cmd(0).cmd
        {
            let cs: sd::CardStatus<sd::SD> = sd::CardStatus::from(resp0);
            if (cs.error()) {
                defmt::info!(
                    "cmd: {}, param: {:x}, res_len: {}",
                    cmd.cmd,
                    cmd.arg,
                    res_len
                );
                defmt::info!("cmd: {}, param: {:x}", cmd.cmd, cmd.arg);
                defmt::error!("card status error: {:x}", resp0);
                defmt::error!(
                    "card status errs: out of range: {},
                address error: {},
                block len error: {},
                erase seq error: {},
                lock unlock failed: {},
                com crc error: {},
                illegal command: {},
                card ecc failed: {},
                cc error: {},
                error: {}
                csd overwrite: {},
                wp erase skip: {},
                erase reset: {}",
                    cs.out_of_range(),
                    cs.address_error(),
                    cs.block_len_error(),
                    cs.erase_seq_error(),
                    cs.lock_unlock_failed(),
                    cs.com_crc_error(),
                    cs.illegal_command(),
                    cs.card_ecc_failed(),
                    cs.cc_error(),
                    cs.error(),
                    cs.csd_overwrite(),
                    cs.wp_erase_skip(),
                    cs.erase_reset(),
                );
                return Err(SdError::STATUSError);
            }
        }
        Ok(())
    }

    pub fn get_cid(&mut self) -> Result<(), SdError> {
        defmt::info!(
            "get cid with resp len {}",
            match common_cmd::all_send_cid().response_len() {
                ResponseLen::Zero => 0,
                ResponseLen::R48 => 1,
                ResponseLen::R136 => 3,
            },
        );
        self.send_cmd(common_cmd::all_send_cid())?;
        // read response
        let resp0 = self.port.respr(0).read().0;
        let resp1 = self.port.respr(1).read().0;
        let resp2 = self.port.respr(2).read().0;
        let resp3 = self.port.respr(3).read().0;
        // self.cid = sd::CID::from([resp0, resp1, resp2, resp3]);
        self.cid = sd::CID::from([resp3, resp2, resp1, resp0]);
        defmt::info!("cid: {:?}, {:?}, {:?}, {:?}", resp0, resp1, resp2, resp3);
        Ok(())
    }
    pub fn get_csd(&mut self) -> Result<(), SdError> {
        self.send_cmd(common_cmd::send_csd(self.rca.address()))?;
        // read response
        let resp0 = self.port.respr(0).read().0;
        let resp1 = self.port.respr(1).read().0;
        let resp2 = self.port.respr(2).read().0;
        let resp3 = self.port.respr(3).read().0;
        self.csd = sd::CSD::from([resp0, resp1, resp2, resp3]);
        // self.csd = sd::CSD::from([resp3, resp2, resp1, resp0]);
        defmt::info!("csd: {:?}, {:?}, {:?}, {:?}", resp0, resp1, resp2, resp3);
        Ok(())
    }
    pub fn get_rca(&mut self) -> Result<(), SdError> {
        // self.send_cmd(sd_cmd::send_relative_address()).unwrap();
        let cmd: common_cmd::Cmd<sd_cmd::R6> = common_cmd::cmd(3, 0);
        self.send_cmd(cmd)?;
        // self.send_cmd(common_cmd::cmd(3, 0xA)).unwrap();
        let resp0 = self.port.respr(0).read().0;
        defmt::info!("rca: {:x}", resp0);
        self.rca = sd::RCA::from(resp0);
        Ok(())
    }
    pub fn read_single_block(&self, buf: &mut [u8], block_addr: u32) -> Result<(), SdError> {
        // TODO: check
        if block_addr > self.csd.block_count() as u32 {
            return Err(SdError::STATUSError);
        }
        self.port.idmabase0r().write(|v| v.0 = buf.as_ptr() as u32);
        self.port.dlenr().write(|v| v.0 = 512);
        self.port.dctrl().modify(|v| {
            v.set_fiforst(true); // reset fifo
            v.set_dblocksize(9); // 512 bytes
            v.set_dtmode(0); // block data transfer
            v.set_dtdir(true); // from card to controller
        });
        self.port.idmactrlr().modify(|v| {
            v.set_idmabmode(false); // single buffer mode
            v.set_idmaen(true); // enable dma
        });

        self.send_cmd(common_cmd::read_single_block(block_addr as u32))?;
        // wait for data transfer complete
        while !self.port.star().read().dataend() {
            self.error_test()?;
            // delay_ms(100);
        }
        Ok(())
    }

    /// the maximum block count is (1<<24) = 16777216 bytes = 16MB
    pub fn read_multiple_blocks(
        &self,
        buf: &[u8],
        block_addr: u32,
        block_count: u32,
    ) -> Result<(), SdError> {
        if (block_count + block_addr) > self.csd.block_count() as u32 {
            return Err(SdError::ReadBlockCountError);
        }
        // self.send_cmd(common_cmd::idle())?;
        // TODO: check
        self.port.idmabase0r().write(|v| v.0 = buf.as_ptr() as u32);
        self.port.dlenr().write(|v| v.0 = block_count * 512);
        self.port.dctrl().modify(|v| {
            v.set_fiforst(true);
            v.set_dblocksize(9);
            v.set_dtmode(0); // stream data transfer
            v.set_dtdir(true); // read ture, write false
        });
        self.port.idmactrlr().modify(|v| {
            v.set_idmabmode(false); // single buffer mode
            v.set_idmaen(true);
        });
        self.send_cmd(sd_cmd::set_block_count(block_count))?;
        self.send_cmd(common_cmd::read_multiple_blocks(block_addr))?;
        // delay_ms(10);
        while !self.port.star().read().dataend() {
            self.error_test()?;
        }
        Ok(())
    }

    pub fn write_multiple_blocks(
        &self,
        buf: &[u8],
        block_addr: u32,
        block_count: u32,
    ) -> Result<(), SdError> {
        if (block_count + block_addr) > self.csd.block_count() as u32 {
            return Err(SdError::WriteBlockCountError);
        }
        self.port.idmabase0r().write(|v| v.0 = buf.as_ptr() as u32);
        self.port.dlenr().write(|v| v.0 = block_count * 512);
        self.port.dctrl().modify(|v| {
            v.set_fiforst(true);
            v.set_dblocksize(9);
            v.set_dtmode(0); // block data transfer
            v.set_dtdir(false); // read ture, write false
        });
        self.port.idmactrlr().modify(|v| {
            v.set_idmabmode(false); // single buffer mode
            v.set_idmaen(true);
        });
        self.send_cmd(sd_cmd::set_block_count(block_count))?;
        self.send_cmd(common_cmd::write_multiple_blocks(block_addr))?;
        while !self.port.star().read().dataend() {
            self.error_test()?
        }
        Ok(())
    }

    pub fn write_single_block(&self, buf: &[u8], block_addr: u32) -> Result<(), SdError> {
        if block_addr as u64 > self.csd.block_count() {
            return Err(SdError::WriteAddressError);
        }
        // self.send_cmd(common_cmd::idle())?;
        self.port.idmabase0r().write(|v| v.0 = buf.as_ptr() as u32);
        self.port.dlenr().write(|v| v.0 = 512);
        self.port.dctrl().modify(|v| {
            v.set_fiforst(true); // reset fifo
            v.set_dblocksize(9); // 512 bytes
            v.set_dtmode(0); // block data transfer
            v.set_dtdir(false); // from controller to card
        });
        self.port.idmactrlr().modify(|v| {
            v.set_idmabmode(false); // single buffer mode
            v.set_idmaen(true); // enable dma
        });
        self.send_cmd(common_cmd::write_single_block(block_addr))?;
        while !self.port.star().read().dataend() {
            self.error_test()?;
        }
        Ok(())
    }

    pub async fn write_multiple_blocks_async(
        &self,
        buf: &[u8],
        block_addr: u32,
        block_count: u32,
    ) -> Result<(), SdError> {
        if (block_count + block_addr) > self.csd.block_count() as u32 {
            return Err(SdError::WriteBlockCountError);
        }
        unsafe {
            NVIC::unmask(stm32_metapac::Interrupt::SDMMC2);
        }
        // we should not clear interrupt here
        self.clear_interrupt();
        self.error_interrupt_enable();
        self.port.idmabase0r().write(|v| v.0 = buf.as_ptr() as u32);
        self.port.dlenr().write(|v| v.0 = block_count * 512);
        self.port.dctrl().modify(|v| {
            v.set_fiforst(true);
            v.set_dblocksize(9);
            v.set_dtmode(0); // block data transfer
            v.set_dtdir(false); // read ture, write false
        });
        self.port.idmactrlr().modify(|v| {
            v.set_idmabmode(false); // single buffer mode
            v.set_idmaen(true);
        });
        self.send_cmd_async(sd_cmd::set_block_count(block_count))
            .await?;
        self.send_cmd_async(common_cmd::write_multiple_blocks(block_addr))
            .await?;
        Ok(())
    }

    pub async fn write_single_block_async(
        &self,
        buf: &[u8],
        block_addr: u32,
    ) -> Result<(), SdError> {
        if block_addr as u64 > self.csd.block_count() {
            return Err(SdError::WriteAddressError);
        }
        unsafe {
            NVIC::unmask(stm32_metapac::Interrupt::SDMMC2);
        }
        // we should not clear interrupt here
        self.clear_interrupt();
        self.error_interrupt_enable();
        self.port.idmabase0r().write(|v| v.0 = buf.as_ptr() as u32);
        self.port.dlenr().write(|v| v.0 = 512);
        self.port.dctrl().modify(|v| {
            v.set_fiforst(true); // reset fifo
            v.set_dblocksize(9); // 512 bytes
            v.set_dtmode(0); // block data transfer
            v.set_dtdir(false); // from controller to card
        });
        self.port.idmactrlr().modify(|v| {
            v.set_idmabmode(false); // single buffer mode
            v.set_idmaen(true); // enable dma
        });
        // self.send_cmd(common_cmd::write_single_block(block_addr))?;
        self.send_cmd_async(common_cmd::write_single_block(block_addr))
            .await?;
        Ok(())
    }

    pub async fn read_single_block_async(
        &self,
        buf: &mut [u8],
        block_addr: u32,
    ) -> Result<(), SdError> {
        // TODO: check
        if block_addr > self.csd.block_count() as u32 {
            return Err(SdError::STATUSError);
        }
        unsafe {
            NVIC::unmask(stm32_metapac::Interrupt::SDMMC2);
        }
        self.clear_interrupt();
        self.error_interrupt_enable();
        self.port.idmabase0r().write(|v| v.0 = buf.as_ptr() as u32);
        self.port.dlenr().write(|v| v.0 = 512);
        self.port.dctrl().modify(|v| {
            v.set_fiforst(true); // reset fifo
            v.set_dblocksize(9); // 512 bytes
            v.set_dtmode(0); // block data transfer
            v.set_dtdir(true); // from card to controller
        });
        self.port.idmactrlr().modify(|v| {
            v.set_idmabmode(false); // single buffer mode
            v.set_idmaen(true); // enable dma
        });

        // self.send_cmd(common_cmd::read_single_block(block_addr as u32))?;
        self.send_cmd_async(common_cmd::read_single_block(block_addr as u32))
            .await?;
        Ok(())
    }

    pub async fn read_multiple_blocks_async(
        &self,
        buf: &[u8],
        block_addr: u32,
        block_count: u32,
    ) -> Result<(), SdError> {
        if (block_count + block_addr) > self.csd.block_count() as u32 {
            return Err(SdError::ReadBlockCountError);
        }
        // self.send_cmd(common_cmd::idle())?;
        // TODO: check
        self.port.idmabase0r().write(|v| v.0 = buf.as_ptr() as u32);
        self.port.dlenr().write(|v| v.0 = block_count * 512);
        self.port.dctrl().modify(|v| {
            v.set_fiforst(true);
            v.set_dblocksize(9);
            v.set_dtmode(0); // stream data transfer
            v.set_dtdir(true); // read ture, write false
        });
        self.port.idmactrlr().modify(|v| {
            v.set_idmabmode(false); // single buffer mode
            v.set_idmaen(true);
        });
        // self.send_cmd(sd_cmd::set_block_count(block_count))?;
        // self.send_cmd(common_cmd::read_multiple_blocks(block_addr))?;
        self.send_cmd_async(sd_cmd::set_block_count(block_count))
            .await?;
        self.send_cmd_async(common_cmd::read_multiple_blocks(block_addr))
            .await?;
        Ok(())
    }

    // pub fn write_multiple_blocks(
    //     &self,
    //     buf: &[u8],
    //     block_addr: u32,
    //     block_count: u32,
    // ) -> Result<(), SdError> {
    //     if (block_count + block_addr) > self.csd.block_count() as u32 {
    //         return Err(SdError::WriteBlockCountError);
    //     }
    //     self.port.idmabase0r().write(|v| v.0 = buf.as_ptr() as u32);
    //     self.port.dlenr().write(|v| v.0 = block_count * 512);
    //     self.port.dctrl().modify(|v| {
    //         v.set_fiforst(true);
    //         v.set_dblocksize(9);
    //         v.set_dtmode(0); // block data transfer
    //         v.set_dtdir(false); // read ture, write false
    //     });
    //     self.port.idmactrlr().modify(|v| {
    //         v.set_idmabmode(false); // single buffer mode
    //         v.set_idmaen(true);
    //     });
    //     self.send_cmd(sd_cmd::set_block_count(block_count))?;
    //     self.send_cmd(common_cmd::write_multiple_blocks(block_addr))?;
    //     while !self.port.star().read().dataend() {
    //         self.error_test()?
    //     }
    //     Ok(())
    // }

    pub fn clear_error(&self) {
        self.port.icr().modify(|v| {
            v.set_ctimeoutc(true);
            v.set_dtimeoutc(true);
            v.set_ccrcfailc(true);
            v.set_dcrcfailc(true);
            v.set_txunderrc(true);
            v.set_rxoverrc(true);
            v.set_ackfailc(true);
            v.set_acktimeoutc(true);
        })
    }
    pub fn clear_interrupt(&self) {
        self.port.icr().write(|v| v.0 = 0x1FE00FFF);
    }
    pub fn error_interrupt_enable(&self) {
        self.port.maskr().modify(|v| {
            v.set_cmdsentie(true);
            v.set_cmdrendie(true);
            v.set_dataendie(true);

            v.set_ccrcfailie(true);
            v.set_dcrcfailie(true);
            v.set_ctimeoutie(true);
            v.set_dtimeoutie(true);
            v.set_txunderrie(true);
            v.set_rxoverrie(true);
            v.set_cmdrendie(true);
            v.set_cmdsentie(true);
            v.set_dataendie(true);
            v.set_dbckendie(true);
            v.set_dabortie(true);
        });
    }
    async fn wait_for_event(&self) -> Result<stm32_metapac::sdmmc::regs::Star, SdError> {
        poll_fn(|cx| {
            unsafe {
                // if self.port == stm32_metapac::SDMMC1 {
                //     SDMMC1_WAKER = cx.waker().clone();
                // } else {
                //     SDMMC2_WAKER = cx.waker().clone();
                // }
                // SDMMC2_WAKER = cx.waker().clone();
            }
            // cause by interrupt?
            let star = self.port.star().read();
            let mask = self.port.maskr().read();
            defmt::info!("call wait for event");
            // list all interrupt
            let star_u32 = star.0;
            let mask_u32 = mask.0;
            if star_u32 & mask_u32 == 0 {
                self.error_interrupt_enable();
                return core::task::Poll::Pending; // call no by interrupt
            }

            let star = self.port.star().read();
            self.port.icr().write(|v| v.0 = 0x1FE00FFF);
            if self.error_test().is_err() {
                return core::task::Poll::Ready(Err(SdError::STATUSError));
            }
            return core::task::Poll::Ready(Ok(star));

            // if star.cmdsent() && mask.cmdsentie()
            // || star.cmdrend() && mask.cmdrendie()
            // || star.dataend() && mask.dataendie()
            // || star.dbckend() && mask.dbckendie()
            // {
            //     // read one more time of star and clear interrupt
            //     // clear all interrupt

            // }
            // // cuase by interrupt ?
            // // if yes, return the star register and clear interrupt
            // // if no, return pending
            // // todo!()
            // return core::task::Poll::Pending;
        })
        .await
    }
}

// impl Future for SdInstance {
//     type Output = Result<stm32_metapac::sdmmc::regs::Star, SdError>;
//     fn poll(
//         self: core::pin::Pin<&mut Self>,
//         cx: &mut core::task::Context<'_>,
//     ) -> core::task::Poll<Self::Output> {
//         unsafe {
//             if self.port == stm32_metapac::SDMMC1 {
//                 SDMMC1_WAKER = cx.waker().clone();
//             } else {
//                 SDMMC2_WAKER = cx.waker().clone();
//             }
//         }
//         // cause by interrupt?
//         let star = self.port.star().read();
//         let mask = self.port.maskr().read();
//         // list all interrupt
//         let star_u32 = star.0;
//         let mask_u32 = mask.0;
//         if star_u32 & mask_u32 == 0 {
//             return core::task::Poll::Pending; // call no by interrupt
//         }

//         let star = self.port.star().read();
//         self.port.icr().write(|v| v.0 = 0x1FE00FFF);
//         if self.error_test().is_err() {
//             return core::task::Poll::Ready(Err(SdError::STATUSError));
//         }
//         return core::task::Poll::Ready(Ok(star));

//         // if star.cmdsent() && mask.cmdsentie()
//         // || star.cmdrend() && mask.cmdrendie()
//         // || star.dataend() && mask.dataendie()
//         // || star.dbckend() && mask.dbckendie()
//         // {
//         //     // read one more time of star and clear interrupt
//         //     // clear all interrupt

//         // }
//         // // cuase by interrupt ?
//         // // if yes, return the star register and clear interrupt
//         // // if no, return pending
//         // // todo!()
//         // return core::task::Poll::Pending;
//     }
// }

// use core::task::Waker;
use stm32_metapac::interrupt;

// static mut SDMMC1_WAKER: Waker = Waker::noop();
// static mut SDMMC2_WAKER: Waker = Waker::noop();
//
// #[interrupt]
// fn SDMMC1() {
//     unsafe {
//         SDMMC1_WAKER.wake_by_ref();
//     }
// }

static SIGNAL: Signal<CriticalSectionRawMutex, u32> = Signal::new();
static SIGNAL2: Signal<CriticalSectionRawMutex, u32> = Signal::new();
static mut SIGNAL2_VAL: u32 = 0;
#[interrupt]
fn SDMMC2() {
    unsafe {
        // defmt::info!("SDMMC2 interrupt");
        // TODO: we need the old value if it is signaled
        let stat = stm32_metapac::SDMMC2.star().read().0;
        if (SIGNAL2.signaled()) {
            // SIGNAL2.signal(stm32_metapac::SDMMC2.star().read().0 | val);
            SIGNAL2.signal(stat | SIGNAL2_VAL);
        } else {
            SIGNAL2.signal(stat);
        }
        SIGNAL2_VAL = stat;
        stm32_metapac::SDMMC2.icr().write(|v| v.0 = 0x1FE00FFF);
    }
}
