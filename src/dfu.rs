//! USB DFU (Device Firmware Upgrade v1.1) Class Implementation
//!
//! Provides DFU Functional Descriptor, Control Request Handling,
//! State Machine Management, and Application Jump routine.

use embassy_usb::control::{InResponse, OutResponse, Request};
use embassy_usb::driver::Driver;
use embassy_usb::Builder;
use crate::flash::{self, FLASH_BASE, FLASH_PAGE_SIZE};

/// Default Base Address for User Application Partition (64KB offset)
pub const APP_BASE_ADDRESS: u32 = FLASH_BASE + (64 * 1024);

/// DFU Requests
pub const DFU_DETACH: u8 = 0;
pub const DFU_DNLOAD: u8 = 1;
pub const DFU_UPLOAD: u8 = 2;
pub const DFU_GETSTATUS: u8 = 3;
pub const DFU_CLRSTATUS: u8 = 4;
pub const DFU_GETSTATE: u8 = 5;
pub const DFU_ABORT: u8 = 6;

/// DFU States according to DFU 1.1 Specification
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum DfuState {
    AppIdle = 0,
    AppDetach = 1,
    DfuIdle = 2,
    DfuDnloadSync = 3,
    DfuDnlBusy = 4,
    DfuDnloadIdle = 5,
    DfuManifestSync = 6,
    DfuManifest = 7,
    DfuManifestWaitReset = 8,
    DfuUploadIdle = 9,
    DfuError = 10,
}

/// DFU Error Status Codes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum DfuStatus {
    Ok = 0x00,
    ErrTarget = 0x01,
    ErrFile = 0x02,
    ErrWrite = 0x03,
    ErrErase = 0x04,
    ErrCheckErase = 0x05,
    ErrProg = 0x06,
    ErrVerify = 0x07,
    ErrAddress = 0x08,
    ErrNotDone = 0x09,
    ErrFirmware = 0x0A,
    ErrVendor = 0x0B,
    ErrUsbr = 0x0C,
    ErrPor = 0x0D,
    ErrUnknown = 0x0E,
    ErrStalledPkt = 0x0F,
}

/// DFU Functional Descriptor attributes
pub mod dfu_attributes {
    pub const CAN_DOWNLOAD: u8 = 0x01;
    pub const CAN_UPLOAD: u8 = 0x02;
    pub const MANIFESTATION_TOLERANT: u8 = 0x04;
    pub const WILL_DETACH: u8 = 0x08;
}

/// USB DFU 1.1 Class Handler
pub struct DfuClass<'a> {
    pub state: DfuState,
    pub status: DfuStatus,
    pub poll_timeout_ms: u32,
    pub transfer_size: u16,
    pub app_address: u32,
    pub current_address: u32,
    pub detach_requested: bool,
    buffer: &'a mut [u8],
    buffer_len: usize,
}

impl<'a> DfuClass<'a> {
    /// Create a new DFU Class Handler with a provided download buffer
    pub fn new(buffer: &'a mut [u8], transfer_size: u16, app_address: u32) -> Self {
        Self {
            state: DfuState::DfuIdle,
            status: DfuStatus::Ok,
            poll_timeout_ms: 50,
            transfer_size,
            app_address,
            current_address: app_address,
            detach_requested: false,
            buffer,
            buffer_len: 0,
        }
    }

    /// Add DFU Interface and DFU Functional Descriptor to Embassy USB Builder
    pub fn register<'d, D: Driver<'d>>(
        builder: &mut Builder<'d, D>,
        transfer_size: u16,
        detach_timeout_ms: u16,
        attributes: u8,
    ) {
        let mut function = builder.function(0xFE, 0x01, 0x02);
        let mut interface = function.interface();
        let mut alt = interface.alt_setting(0xFE, 0x01, 0x02, None);

        // Append DFU Functional Descriptor
        let descriptor = [
            0x09, // bLength
            0x21, // bDescriptorType (DFU FUNCTIONAL)
            attributes,
            (detach_timeout_ms & 0xFF) as u8,
            ((detach_timeout_ms >> 8) & 0xFF) as u8,
            (transfer_size & 0xFF) as u8,
            ((transfer_size >> 8) & 0xFF) as u8,
            0x10, // bcdDFUVersion (1.1 in BCD: 0x0110)
            0x01,
        ];
        alt.descriptor(0x21, &descriptor[2..]);
    }

    /// Process DFU Control Requests (Endpoint 0)
    pub fn handle_control_out(&mut self, req: Request, buf: &[u8]) -> Option<OutResponse> {
        if req.request_type != embassy_usb::control::RequestType::Class {
            return None;
        }

        match req.request {
            DFU_DETACH => {
                self.detach_requested = true;
                self.state = DfuState::AppDetach;
                Some(OutResponse::Accepted)
            }
            DFU_DNLOAD => {
                if self.state != DfuState::DfuIdle
                    && self.state != DfuState::DfuDnloadIdle
                    && self.state != DfuState::DfuDnloadSync
                {
                    return Some(OutResponse::Rejected);
                }

                let block_num = req.value;
                if buf.is_empty() {
                    // End of download -> Manifest Sync
                    self.state = DfuState::DfuManifestSync;
                } else {
                    // Buffer received data
                    let len = buf.len().min(self.buffer.len());
                    self.buffer[..len].copy_from_slice(&buf[..len]);
                    self.buffer_len = len;

                    if block_num == 0 {
                        self.current_address = self.app_address;
                    }

                    // Program Flash
                    match self.program_buffered_chunk() {
                        Ok(()) => {
                            self.state = DfuState::DfuDnloadSync;
                            self.status = DfuStatus::Ok;
                        }
                        Err(status) => {
                            self.state = DfuState::DfuError;
                            self.status = status;
                        }
                    }
                }
                Some(OutResponse::Accepted)
            }
            DFU_CLRSTATUS => {
                self.state = DfuState::DfuIdle;
                self.status = DfuStatus::Ok;
                Some(OutResponse::Accepted)
            }
            DFU_ABORT => {
                self.state = DfuState::DfuIdle;
                self.status = DfuStatus::Ok;
                self.current_address = self.app_address;
                Some(OutResponse::Accepted)
            }
            _ => Some(OutResponse::Rejected),
        }
    }

    /// Process DFU Control Reads (Endpoint 0)
    pub fn handle_control_in<'b>(&mut self, req: Request, buf: &'b mut [u8]) -> Option<InResponse<'b>> {
        if req.request_type != embassy_usb::control::RequestType::Class {
            return None;
        }

        match req.request {
            DFU_GETSTATUS => {
                // State transitions on GETSTATUS
                match self.state {
                    DfuState::DfuDnloadSync => {
                        self.state = DfuState::DfuDnloadIdle;
                    }
                    DfuState::DfuManifestSync => {
                        self.state = DfuState::DfuManifest;
                    }
                    DfuState::DfuManifest => {
                        self.state = DfuState::DfuManifestWaitReset;
                    }
                    _ => {}
                }

                if buf.len() >= 6 {
                    buf[0] = self.status as u8;
                    // bwPollTimeout (24-bit LSB first)
                    buf[1] = (self.poll_timeout_ms & 0xFF) as u8;
                    buf[2] = ((self.poll_timeout_ms >> 8) & 0xFF) as u8;
                    buf[3] = ((self.poll_timeout_ms >> 16) & 0xFF) as u8;
                    buf[4] = self.state as u8;
                    buf[5] = 0; // iString
                    Some(InResponse::Accepted(&buf[..6]))
                } else {
                    Some(InResponse::Rejected)
                }
            }
            DFU_GETSTATE => {
                if !buf.is_empty() {
                    buf[0] = self.state as u8;
                    Some(InResponse::Accepted(&buf[..1]))
                } else {
                    Some(InResponse::Rejected)
                }
            }
            DFU_UPLOAD => {
                if self.state != DfuState::DfuIdle && self.state != DfuState::DfuUploadIdle {
                    return Some(InResponse::Rejected);
                }
                self.state = DfuState::DfuUploadIdle;

                let req_len = req.length as usize;
                let send_len = req_len.min(buf.len());
                let flash_ptr = self.current_address as *const u8;

                unsafe {
                    core::ptr::copy_nonoverlapping(flash_ptr, buf.as_mut_ptr(), send_len);
                }
                self.current_address += send_len as u32;

                Some(InResponse::Accepted(&buf[..send_len]))
            }
            _ => Some(InResponse::Rejected),
        }
    }

    /// Internal Flash page erase and quad-word programming helper
    fn program_buffered_chunk(&mut self) -> Result<(), DfuStatus> {
        let chunk_len = self.buffer_len;
        if chunk_len == 0 {
            return Ok(());
        }

        let addr = self.current_address;

        // Erase page if chunk starts on page boundary
        if addr % (FLASH_PAGE_SIZE as u32) == 0 {
            if let Err(_) = flash::erase_page(addr as usize) {
                return Err(DfuStatus::ErrErase);
            }
        }

        // Convert slice bytes into u32 words aligned to 4 words (16 bytes)
        let mut u32_buf = [0u32; 512]; // Up to 2048 bytes
        let words_count = (chunk_len + 3) / 4;
        let quad_words_count = (words_count + 3) & !3; // Round up to multiple of 4 words

        unsafe {
            let src_ptr = self.buffer.as_ptr();
            let dst_ptr = u32_buf.as_mut_ptr() as *mut u8;
            core::ptr::copy_nonoverlapping(src_ptr, dst_ptr, chunk_len);
        }

        if let Err(_) = flash::write_quad_words(addr, &u32_buf[..quad_words_count]) {
            return Err(DfuStatus::ErrWrite);
        }

        self.current_address += chunk_len as u32;
        Ok(())
    }
}

impl<'a> embassy_usb::Handler for DfuClass<'a> {
    fn control_out(&mut self, req: Request, buf: &[u8]) -> Option<OutResponse> {
        self.handle_control_out(req, buf)
    }

    fn control_in<'b>(&'b mut self, req: Request, buf: &'b mut [u8]) -> Option<InResponse<'b>> {
        self.handle_control_in(req, buf)
    }
}

/// Jump to Application at `app_address` (Default `0x0801_0000`)
///
/// # Safety
/// Relocates SCB VTOR vector table, resets Main Stack Pointer (MSP),
/// and executes Application Reset Handler.
pub unsafe fn jump_to_application(app_address: u32) -> ! {
    cortex_m::interrupt::disable();

    // 1. Relocate SCB VTOR
    let scb = &*cortex_m::peripheral::SCB::PTR;
    scb.vtor.write(app_address);

    // 2. Load MSP and Reset Handler
    let sp = *(app_address as *const u32);
    let reset_handler_ptr = *((app_address + 4) as *const u32);

    core::arch::asm!(
        "msr msp, {}",
        "bx {}",
        in(reg) sp,
        in(reg) reset_handler_ptr,
        options(noreturn)
    );
}
