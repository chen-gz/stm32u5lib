//! STM32U5 内部 Flash 擦写与读取驱动。
//!
//! ### 功能概述
//! 1. **掉电持久化存储**：允许直接在 STM32U5 内部 Flash 上存储非易失性数据。
//! 2. **代码区保护机制**：
//!    - 自动引入链接器（Linker）符号 `_sidata` 和 `_eidata`，它们代表编译出的程序固件代码和已初始化数据在 Flash 中的实际终止边界。
//!    - `get_flash_safe_start()` 会将该边界向上舍入对齐到最近的 **8 KB 页面边界**。
//!    - 所有擦除（`erase_page`）和写入（`program_quad_word` / `write`）都会校验目标地址，严禁操作低于该安全边界的区域，从而防止覆盖或损坏程序固件。
//! 3. **数据读取功能**：
//!    - Flash 支持直接内存映射（Memory-mapped）寻址读取。
//!    - 驱动程序提供 `read()` 接口，利用 `read_volatile` 安全、高效地成块读取物理 Flash 内容到缓冲区。
//! 4. **物理读写与寿命限制**：
//!    - **擦写寿命限制**：整个 Flash 的标准擦写与编程寿命为 **10,000 次**；如果对高耐用性（High-endurance）进行特殊页配置，每个 Bank 最多支持 **256 Kbytes** 的区域达到 **100,000 次** 擦写寿命。
//!    - **读取寿命限制**：Flash 的数据读取次数是**无限制**的，读取操作不会对 Flash 寿命造成任何磨损。
//! 5. **自适应 Flash 尺寸检测**：
//!    - 运行时读取出厂固化的 Flash 大小寄存器（`0x08FFF80C`），自适应支持 1MB/2MB/4MB 的不同芯片型号。
//!    - 动态根据芯片尺寸计算 Bank 1 和 Bank 2 的分配比例，自动完成擦除时的页号和 Bank 切换。
//! 6. **四字对齐编程（128-bit Quad-Word）**：
//!    - STM32U5 硬件要求最小写入单元为 128 位（16 字节），且写入的目标首地址必须是 16 字节对齐的。
//!    - 每次编程需连续写入 4 个 32 位字（Word）到内部写入缓存，并等待 busy 清空。
//! 7. **指令缓存失效（ICACHE Invalidation）**：
//!    - 为了防止数据擦写后 CPU ICACHE 残留旧版本的数据副本，擦写成功后驱动会自动执行 `invalidate_icache()` 清空缓存，保证读回最新写入的数据。
//!
//! ### 使用示例
//! ```rust
//! use u5_lib::flash;
//!
//! fn save_data_example() {
//!     // 1. 获取安全的写入起始地址
//!     let safe_addr = flash::get_flash_safe_start();
//!     
//!     // 2. 擦除该 8 KB 页面（写入前必须擦除）
//!     flash::erase_page(safe_addr).unwrap();
//!     
//!     // 3. 准备写入的 16 字节数据（必须是 16 字节对齐）
//!     let my_data: [u8; 16] = [0xAA; 16];
//!     
//!     // 4. 写入数据到该安全地址
//!     flash::write(safe_addr, &my_data).unwrap();
//!
//!     // 5. 从该安全地址读回数据进行校验
//!     let mut read_buf = [0u8; 16];
//!     flash::read(safe_addr, &mut read_buf).unwrap();
//!     assert_eq!(read_buf, my_data);
//! }
//! ```

use stm32_metapac::{FLASH, ICACHE};

extern "C" {
    // 这些符号由 cortex-m-rt 链接器脚本提供，指向数据段在 Flash 中的起始和结束位置。
    static _sidata: u32;
    static _eidata: u32;
}

/// Flash 起始物理基地址（STM32 统一为 0x08000000）
pub const FLASH_START: usize = 0x08000000;
/// STM32U5 最小擦除扇区大小（8 KB）
pub const PAGE_SIZE: usize = 8192; // 8 KB

/// 驱动可能返回的错误枚举
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum FlashError {
    /// 擦写地址未对齐到规定的物理边界（擦除需 8KB 对齐，写入需 16字节对齐）
    AddressNotAligned,
    /// 批量写入的数据缓冲区长度不是 16 字节的整数倍
    LengthNotAligned,
    /// 目标读写地址超出了 Flash 的实际物理范围
    OutOfRange,
    /// 触发安全拦截：目标擦写地址位于固件代码或只读数据区内，拒绝操作
    OverwriteCode,
    /// 擦除页面时发生硬件错误（如写保护、对齐错误等）
    EraseError,
    /// 编程写入时发生硬件错误（如对齐、大小限制、校验错误等）
    ProgramError,
}

// 解锁 Flash 控制器所需的两步魔术密钥
const KEY1: u32 = 0x45670123;
const KEY2: u32 = 0xCDEF89AB;

/// 获取第一个安全且未被程序代码/数据占用的 Flash 物理地址。
/// 该地址向上对齐到了 8 KB (8192 字节) 的页面边界。
pub fn get_flash_safe_start() -> usize {
    let raw_end = unsafe { &_eidata as *const u32 as usize };
    // 向上舍入到 8 KB 的整倍数
    (raw_end + PAGE_SIZE - 1) & !(PAGE_SIZE - 1)
}

/// 读取出厂固化的签名寄存器，获取当前芯片的 Flash 物理容量（以字节为单位）。
/// 支持运行时自适应不同容量的 STM32U5 系列微控制器。
pub fn get_flash_size() -> usize {
    // 读取只读的 FLASHSIZE_BASE 地址 (0x08FFF80C)，其内容为以 KB 表示的 Flash 大小
    let flash_size_kb = unsafe { core::ptr::read_volatile(0x08FFF80C as *const u16) };
    (flash_size_kb as usize) * 1024
}

/// 获取当前芯片 Flash 的终止物理边界。
pub fn get_flash_end() -> usize {
    FLASH_START + get_flash_size()
}

/// 解锁非安全 Flash 控制寄存器 (FLASH_NSCR) 以允许擦写操作。
fn unlock_flash() {
    if FLASH.nscr().read().lock() {
        FLASH.nskeyr().write_value(KEY1);
        FLASH.nskeyr().write_value(KEY2);
    }
}

/// 重新锁定非安全 Flash 控制寄存器以保护 Flash 避免误操作。
fn lock_flash() {
    FLASH.nscr().modify(|w| w.set_lock(true));
}

/// 清空并失效指令缓存 (ICACHE)，避免 CPU 读到缓存在 ICACHE 中的旧 Flash 数据副本。
pub fn invalidate_icache() {
    // 等待 ICACHE 不处于 busy 状态
    while ICACHE.sr().read().busyf() {}
    // 设置 CACHEINV (Cache Invalidation) 启动缓存擦除失效
    ICACHE.cr().modify(|w| w.set_cacheinv(true));
    // 等待缓存重置失效完成
    while ICACHE.sr().read().busyf() {}
}

/// 擦除包含指定物理地址的整个 8 KB 页面。
///
/// ### 注意事项
/// - 传入的 `addr` 必须在通过 `get_flash_safe_start()` 认定的安全边界之外。
/// - `addr` 必须能够被 8192 (8 KB) 整除（即必须是页面起始地址）。
pub fn erase_page(addr: usize) -> Result<(), FlashError> {
    let safe_start = get_flash_safe_start();
    let flash_end = get_flash_end();

    // 1. 安全边界检查
    if addr < safe_start || addr >= flash_end {
        return Err(FlashError::OverwriteCode);
    }
    if !addr.is_multiple_of(PAGE_SIZE) {
        return Err(FlashError::AddressNotAligned);
    }

    // 根据动态获取的 Flash 总容量，计算 Bank 1 和 Bank 2 的分界线（双 Bank 对等分配）
    let flash_size_bytes = get_flash_size();
    let bank_size = flash_size_bytes / 2;

    let offset = addr - FLASH_START;
    // 自动判定属于哪个 Bank 以及该 Bank 内的相对页号 (PNB)
    let (bank, page_in_bank) = if offset < bank_size {
        (0u8, offset / PAGE_SIZE)
    } else {
        (1u8, (offset - bank_size) / PAGE_SIZE)
    };

    // 1. 确保当前无其他 Flash 擦写操作在进行
    while FLASH.nssr().read().bsy() {}

    // 2. 向 NSSR 寄存器所有错误位写 1 以清空之前的挂起错误标志
    FLASH.nssr().write(|w| {
        w.set_eop(true);
        w.set_operr(true);
        w.set_progerr(true);
        w.set_wrperr(true);
        w.set_pgaerr(true);
        w.set_sizerr(true);
        w.set_pgserr(true);
        w.set_optwerr(true);
    });

    // 3. 解锁寄存器控制权
    unlock_flash();

    // 4. 配置页面擦除参数 (PER = 1, 选择目标页号与 Bank，最后将 STRT 位置 1 触发擦除)
    FLASH.nscr().modify(|w| {
        w.set_per(true);
        w.set_pnb(page_in_bank as u8);
        w.set_bker(bank != 0);
        w.set_strt(true);
    });

    // 5. 阻塞等待擦除操作结束
    while FLASH.nssr().read().bsy() {}

    // 6. 清除页面擦除使能位 PER
    FLASH.nscr().modify(|w| {
        w.set_per(false);
    });

    // 7. 锁定寄存器保护 Flash
    lock_flash();

    // 8. 校验硬件状态寄存器，确保没有产生擦除错误
    let sr = FLASH.nssr().read();
    if sr.operr() || sr.progerr() || sr.wrperr() || sr.pgaerr() || sr.sizerr() || sr.pgserr() || sr.optwerr() {
        return Err(FlashError::EraseError);
    }

    // 9. 擦除完成后立即清理并失效 ICACHE
    invalidate_icache();

    Ok(())
}

/// 从指定的 Flash 物理地址直接内存映射读取数据到 `buf` 中。
pub fn read(addr: usize, buf: &mut [u8]) -> Result<(), FlashError> {
    let flash_end = get_flash_end();
    if addr < FLASH_START || addr + buf.len() > flash_end {
        return Err(FlashError::OutOfRange);
    }

    unsafe {
        let src = addr as *const u8;
        for i in 0..buf.len() {
            // 使用 volatile 防止编译器优化和重排读取指令
            buf[i] = core::ptr::read_volatile(src.add(i));
        }
    }
    Ok(())
}

/// 往指定 Flash 物理地址编程写入一个 128 位 (16 字节) 的四字（Quad-Word）。
///
/// ### 注意事项
/// - `addr` 必须位于安全边界内，且不能越界。
/// - `addr` 必须是 16 字节对齐的。
pub fn program_quad_word(addr: usize, data: &[u8; 16]) -> Result<(), FlashError> {
    let safe_start = get_flash_safe_start();
    let flash_end = get_flash_end();

    // 安全边界和对齐校验
    if addr < safe_start || addr + 16 > flash_end {
        return Err(FlashError::OverwriteCode);
    }
    if !addr.is_multiple_of(16) {
        return Err(FlashError::AddressNotAligned);
    }

    // 1. 等待硬件 Busy 清除
    while FLASH.nssr().read().bsy() {}

    // 2. 清除状态标志
    FLASH.nssr().write(|w| {
        w.set_eop(true);
        w.set_operr(true);
        w.set_progerr(true);
        w.set_wrperr(true);
        w.set_pgaerr(true);
        w.set_sizerr(true);
        w.set_pgserr(true);
        w.set_optwerr(true);
    });

    // 3. 解锁寄存器
    unlock_flash();

    // 4. 使能编程模式 (PG = 1)
    FLASH.nscr().modify(|w| {
        w.set_pg(true);
    });

    // 5. 按照硬件要求，以 32-bit (Word) 为单位，连续向目标地址写入 4 次以装填写缓存
    let words: &[u32; 4] = unsafe { core::mem::transmute(data) };
    let ptr = addr as *mut u32;
    unsafe {
        core::ptr::write_volatile(ptr.add(0), words[0]);
        core::ptr::write_volatile(ptr.add(1), words[1]);
        core::ptr::write_volatile(ptr.add(2), words[2]);
        core::ptr::write_volatile(ptr.add(3), words[3]);
    }

    // 6. 阻塞等待写入（Busy 位自动恢复）
    while FLASH.nssr().read().bsy() {}

    // 7. 关闭编程使能位 PG
    FLASH.nscr().modify(|w| {
        w.set_pg(false);
    });

    // 8. 锁定寄存器保护 Flash
    lock_flash();

    // 9. 校验写操作是否伴随硬件对齐/尺寸/保护错误
    let sr = FLASH.nssr().read();
    if sr.operr() || sr.progerr() || sr.wrperr() || sr.pgaerr() || sr.sizerr() || sr.pgserr() || sr.optwerr() {
        return Err(FlashError::ProgramError);
    }

    // 10. 写入结束后使 ICACHE 失效，保证一致性
    invalidate_icache();

    Ok(())
}

/// 往 Flash 指定地址写入任意缓冲区数据。
///
/// ### 注意事项
/// - `addr` 必须是 16 字节对齐的。
/// - `data` 的总长度也必须是 16 字节的整数倍（以装填 Quad-Word）。
pub fn write(addr: usize, data: &[u8]) -> Result<(), FlashError> {
    let safe_start = get_flash_safe_start();
    let flash_end = get_flash_end();

    if addr < safe_start {
        return Err(FlashError::OverwriteCode);
    }
    if addr + data.len() > flash_end {
        return Err(FlashError::OutOfRange);
    }
    if !addr.is_multiple_of(16) {
        return Err(FlashError::AddressNotAligned);
    }
    if !data.len().is_multiple_of(16) {
        return Err(FlashError::LengthNotAligned);
    }

    let mut current_addr = addr;
    let mut offset = 0;

    // 循环按 16 字节分块进行物理 Quad-Word 写入
    while offset < data.len() {
        let mut chunk = [0u8; 16];
        chunk.copy_from_slice(&data[offset..offset + 16]);
        program_quad_word(current_addr, &chunk)?;
        current_addr += 16;
        offset += 16;
    }

    Ok(())
}

// ///////////////////////////////////////////////////////////////////////////
// 未来规划与高耐用性（High-Endurance）擦写优化指南
// ///////////////////////////////////////////////////////////////////////////
// STM32U5 官方规范中，Flash 擦写物理特性不支持通过软件开关（如 Option Bytes）直接配置“高耐用性”模式。
// 该模式本质上是硬件电荷泵与物理存储阵列的物理耐受极限。为了支持部分页面的 100,000 次高频擦写寿命，
// 未来在该驱动上层可进一步实现以下规划方案：
//
// 1. 软件磨损均衡（Wear Leveling）实现：
//    - 限制频繁擦写的数据区总大小在单 Bank 内不超过 32 个页面（共 256 Kbytes）。
//    - 引入环形缓冲区（Ring Buffer）或日志型文件系统（LFS），让写操作在 32 个页面内循环滚动分布。
//    - 这样可以避免单个 8 KB 页面因频繁擦写过早物理损坏，使得整个 256 KB 区域整体寿命达到 100,000 次。
//
// 2. 擦除计数器（Erase Counter）：
//    - 在磨损均衡算法中，设计每个 Sector / Page 头部的擦写元数据，记录该页面的擦写次数。
//    - 写入时优先选择擦写次数最少的物理页面进行数据装填，以确保所有物理页寿命消耗均匀。
//
// 3. 坏块标记与数据保护：
//    - 监测 `erase_page` 与 `program_quad_word` 返回的 `EraseError` 与 `ProgramError`。
//    - 一旦检测到某个物理页物理损坏（写入/擦除校验失败），应将其标记为“坏页”并从可用地址链表中剔除，
//      同时将该页原存的重要配置数据迁移保存至备用页面中，保证系统数据高可用。

