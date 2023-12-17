#![allow(unused)]

use core::ptr::read;

use crate::clock::{delay_ms, delay_tick, delay_us};
use cortex_m::delay;
use futures::future::poll_fn;
use futures::future::OkInto;
use sdio_host::common_cmd::cmd;
use stm32_metapac::{common, sdmmc::Sdmmc};
macro_rules! wait_for_event {
    ($fn_name:ident, $event:ident) => {
        pub async fn $fn_name(&self) -> Result<(), SdError> {
            poll_fn(|cx| {
                self.set_waker(cx.waker().clone());
                match self.error_test() {
                    Ok(_) => {}
                    Err(err) => {
                        self.clear_interrupt();
                        return core::task::Poll::Ready(Err(err));
                    }
                }
                if self.port.star().read().$event() {
                    self.clear_interrupt();
                    return core::task::Poll::Ready(Ok(()));
                }
                core::task::Poll::Pending
            })
            .await
        }
    };
}

// Usage

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
    pub fn init(&mut self, clk: GpioPort, cmd: GpioPort, d0: GpioPort) {
        clk.setup();
        cmd.setup();
        d0.setup();
        delay_ms(100); // TODO: chekc this value

        // setup gpio ports
        // check clock 48Mhz as input clock
        self.port.clkcr().modify(|v| {
            v.set_clkdiv(24); // 48Mhz / (2 * clkdiv) = 48M / 120 = 400Khz
        });

        self.port.power().modify(|v| v.set_pwrctrl(3));
        delay_ms(10); // 400khz, 74clk = 185us
        self.port.dtimer().modify(|v| {
            v.0 = 5*400_000; // 400khz, 8000clk = 20ms
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
        while !self.port.star().read().cmdsent() && !self.port.star().read().cmdrend() {
            self.error_test()?;
delay_ms(1);
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
        // self.csd = sd::CSD::from([resp0, resp1, resp2, resp3]);
        self.csd = sd::CSD::from([resp3, resp2, resp1, resp0]);
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

    pub fn read_single_block_retry(
        &self,
        buf: &mut [u8],
        block_addr: u32,
        try_times: u32,
    ) -> Result<(), SdError> {
        for i in 0..try_times {
            match self.read_single_block(buf, block_addr) {
                Ok(_) => {
                    return Ok(());
                }
                Err(err) => {
                    defmt::error!("read single block error: {:?}, try times: {}", err, i);
                }
            }
        }
        Err(SdError::STATUSError)
    }

    pub fn write_single_block_retry(
        &self,
        buf: &[u8],
        block_addr: u32,
        try_times: u32,
    ) -> Result<(), SdError> {
        for i in 0..try_times {
            match self.write_single_block(buf, block_addr) {
                Ok(_) => {
                    return Ok(());
                }
                Err(err) => {
                    defmt::error!("write single block error: {:?}, try times: {}", err, i);
                }
            }
            delay_ms(50);
        }
        Err(SdError::STATUSError)
    }

    pub async fn write_single_blocks_async(
        &self,
        buf: &[u8],
        block_addr: u32,
    ) -> Result<(), SdError> {
        if block_addr as u64 > self.csd.block_count() {
            return Err(SdError::STATUSError);
        }
        // stop all current transmission
        self.port.cmdr().modify(|v| v.set_cmdstop(true));
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
            v.set_idmaen(true); // enable jdma
        });
        self.send_cmd(common_cmd::write_single_block(block_addr))?;
        self.wait_for_dataend().await?;
        Ok(())
    }
    wait_for_event!(wait_for_dataend, dataend);
    wait_for_event!(wait_for_cmdrend, cmdrend);
    wait_for_event!(wait_for_cmdsent, cmdsent);
    pub fn set_waker(&self, waker: core::task::Waker) {
        unsafe {
            if self.port == stm32_metapac::SDMMC1 {
                SDMMC1_WAKER = waker;
            } else {
                SDMMC2_WAKER = waker;
            }
        }
    }
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
}

impl Future for SdInstance {
    type Output = Result<(), SdError>;
    fn poll(
        self: core::pin::Pin<&mut Self>,
        cx: &mut core::task::Context<'_>,
    ) -> core::task::Poll<Self::Output> {
        unsafe {
            if self.port == stm32_metapac::SDMMC1 {
                SDMMC1_WAKER = cx.waker().clone();
            } else {
                SDMMC2_WAKER = cx.waker().clone();
            }
        }
        return core::task::Poll::Pending;
        todo!()
    }
}

use core::task::Waker;
use stm32_metapac::interrupt;

static mut SDMMC1_WAKER: Waker = Waker::noop();
static mut SDMMC2_WAKER: Waker = Waker::noop();

#[interrupt]
fn SDMMC1() {
    unsafe {
        SDMMC1_WAKER.wake_by_ref();
    }
}

#[interrupt]
fn SDMMC2() {
    unsafe {
        SDMMC2_WAKER.wake_by_ref();
    }
}
