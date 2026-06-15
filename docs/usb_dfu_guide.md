# STM32U5 USB DFU Usage Guide

This guide explains how to integrate and use the `u5-lib` USB DFU 1.1 Class driver in both **Bootloader Mode** and **Runtime (Application) Mode**, as well as how to flash firmware using standard host tools like `dfu-util`.

---

## 1. Bootloader Mode (Standalone DFU Bootloader)

In **Bootloader Mode**, the device runs a small binary located at `0x0800_0000` (64KB reservation). Its primary job is to handle USB DFU requests from host, erase/write pages to Application Flash (`0x0801_0000`), and jump to Application upon completion.

### Code Example (`src/bin/dfu_bootloader.rs` or `main.rs` of Bootloader)

```rust
#![no_std]
#![no_main]

use u5_lib::otg::{Driver, Config};
use u5_lib::gpio::{USB_DM_PA11, USB_DP_PA12};
use u5_lib::dfu::{DfuClass, APP_BASE_ADDRESS, dfu_attributes, jump_to_application};
use embassy_usb::Builder;

#[embassy_executor::main]
async fn main(_spawner: embassy_executor::Spawner) {
    // 1. Check if user app is valid and no forced DFU button is pressed
    let app_sp = unsafe { *(APP_BASE_ADDRESS as *const u32) };
    let boot_dfu_forced = false; // Optionally check GPIO button or RTC backup flag here

    // If valid stack pointer (0x2000_0000 .. SRAM) and not forced, jump straight to App
    if app_sp & 0x2E00_0000 == 0x2000_0000 && !boot_dfu_forced {
        unsafe {
            jump_to_application(APP_BASE_ADDRESS);
        }
    }

    // 2. Initialize 160MHz System Clock
    u5_lib::clock::init_clock(true, u5_lib::clock::ClockFreqs::KernelFreq160Mhz);

    // 3. Setup USB Driver
    static mut EP_OUT_BUFFER: [u8; 1024] = [0; 1024];
    let config = Config::default();
    let driver = Driver::new_fs(USB_DP_PA12, USB_DM_PA11, unsafe { &mut *(&raw mut EP_OUT_BUFFER) }, config);

    // 4. USB Configuration
    let mut usb_config = embassy_usb::Config::new(0x0483, 0xdf11); // ST Microelectronics DFU VID/PID
    usb_config.manufacturer = Some("STMicroelectronics");
    usb_config.product = Some("STM32U5 DFU Bootloader");
    usb_config.serial_number = Some("DFU-001");
    usb_config.max_power = 100;
    usb_config.max_packet_size_0 = 64;

    // Static descriptors and DFU transfer buffer
    static mut CONFIG_DESC: [u8; 256] = [0; 256];
    static mut BOS_DESC: [u8; 256] = [0; 256];
    static mut MSOS_DESC: [u8; 256] = [0; 256];
    static mut CONTROL_BUF: [u8; 64] = [0; 64];
    static mut DFU_BUFFER: [u8; 2048] = [0; 2048];

    let dfu_buf = unsafe { &mut *(&raw mut DFU_BUFFER) };
    let mut dfu_class = DfuClass::new(dfu_buf, 2048, APP_BASE_ADDRESS);

    let mut builder = Builder::new(
        driver,
        usb_config,
        unsafe { &mut *(&raw mut CONFIG_DESC) },
        unsafe { &mut *(&raw mut BOS_DESC) },
        unsafe { &mut *(&raw mut MSOS_DESC) },
        unsafe { &mut *(&raw mut CONTROL_BUF) },
    );

    // 5. Register DFU Functional Descriptor
    DfuClass::register(
        &mut builder,
        2048, // Transfer size (2KB)
        1000, // Detach timeout (1s)
        dfu_attributes::CAN_DOWNLOAD | dfu_attributes::CAN_UPLOAD | dfu_attributes::WILL_DETACH,
    );

    builder.handler(&mut dfu_class);
    let mut usb = builder.build();

    // 6. Run USB background task
    let usb_fut = usb.run();

    let dfu_monitor_fut = async {
        loop {
            embassy_time::Timer::after_millis(100).await;
            // Check if DFU manifestation finished or detach requested
            if dfu_class.state == u5_lib::dfu::DfuState::DfuManifestWaitReset {
                // Flash programming complete, jump to application
                unsafe {
                    jump_to_application(APP_BASE_ADDRESS);
                }
            }
        }
    };

    embassy_futures::select::select(usb_fut, dfu_monitor_fut).await;
}
```

---

## 2. Runtime Mode (Application Mode)

In **Runtime Mode**, your main application runs at `0x0801_0000`. You can include the DFU descriptor so host tools know the device can detach into DFU mode.

### Linker Script (`memory.x`) for Application
Ensure your Application `memory.x` starts at `0x0801_0000`:
```ld
MEMORY
{
  FLASH : ORIGIN = 0x08010000, LENGTH = 1920K
  RAM   : ORIGIN = 0x20000000, LENGTH = 768K
}
```

### Detach Request Handling in Application
When the host executes `dfu-util -e` or requests detach, `DfuClass` sets `detach_requested = true`. The Application can catch this and trigger a system reset:
```rust
if dfu_class.detach_requested {
    cortex_m::peripheral::SCB::sys_reset();
}
```

---

## 3. Host Operations (`dfu-util`)

You can use the standard open-source utility `dfu-util` on macOS, Linux, or Windows.

### List Connected DFU Devices
```bash
dfu-util -l
```
*Expected Output:*
```text
Found DFU: [0483:df11] ver=0110, devnum=5, cfg=1, intf=0, alt=0, name="STMicroelectronics", serial="DFU-001"
```

### Flash New Firmware (`.bin`)
To download `app.bin` into Application Flash at `0x0801_0000` and automatically reset/jump:
```bash
dfu-util -a 0 -s 0x08010000:leave -D target/thumbv8m.main-none-eabihf/release/app.bin
```

### Read Back / Verify Firmware
To upload (read out) 128KB of firmware from Flash:
```bash
dfu-util -a 0 -s 0x08010000:131072 -U dump_app.bin
```

---

## 4. Flash Driver Direct Usage (`u5_lib::flash`)

If you want to perform standalone Flash erase or write operations without USB:

```rust
use u5_lib::flash;

// Erase 8KB Flash page at 0x0801_0000
flash::erase_page(0x0801_0000).expect("Erase page failed");

// Write 4x u32 words (16-byte aligned)
let data: [u32; 4] = [0x12345678, 0x9ABCDEF0, 0x00112233, 0x44556677];
flash::write_quad_words(0x0801_0000, &data).expect("Flash write failed");
```
