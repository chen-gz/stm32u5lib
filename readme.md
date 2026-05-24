## Introduction


This is a HAL library for stm32. The code are writing for u5 and it should be
easy to extend to other stm32 chips.


## Design

This library use stateless design. That means you can call any function at any
time without worrying about the state of the peripheral. The library will use
the state based on the hardware to determine what to do next. If the function is
not applicable in the current state, it will return an error code.

There are some function can not write as stateless design. In that case, we will
have document and put warning in the function description.

I am plan to redesign and updateh library with self-loop test for the future.
Currently the USB function does not work well. I will make it work when I have time.

**redesign** Redesign for the interface are planned. A lot of aysnc function with be supported and tested.
All function with async will default with timeout function to ensure the function will not block the system.

### Known Issues

In the STM32U575, the RTC does not work at 160MHz with 1.8V.

The frequency range between 16MHz and 160MHz has not been tested. Frequencies below 16MHz and at 160MHz have been tested.

## 功能与测试覆盖情况

对代码库进行了扫描，以下是当前功能的测试覆盖情况：

### 已覆盖并有测试用例的功能
这些功能在 `tests/` 目录下有对应的测试文件，具备基本的集成测试：
*   **时钟系统 (Clock)**：测试文件 `tests/clock.rs`
*   **通用输入输出 (GPIO)**：测试文件 `tests/gpio.rs`
*   **I2C 通信**：测试文件 `tests/i2c.rs` (支持主从模式及异步)
*   **低功耗定时器 (LPTIM)**：测试文件 `tests/lptim.rs`
*   **实时时钟 (RTC)**：测试文件 `tests/rtc.rs`
*   **通用定时器 (TIM)**：测试文件 `tests/tim.rs`
*   **外部中断 (EXTI)**：测试文件 `tests/exti_btn.rs`
*   **USB 相关**：测试文件 `tests/usb_cdc_acm.rs`, `tests/usb_enumeration.rs`

### 缺失独立测试的功能
以下模块在 `src/` 中有实现，但在 `tests/` 中没有对应的独立测试文件：
*   `adc.rs` (模数转换器)
*   `camera.rs` / `dcmi.rs` (摄像头及数字摄像头接口)
*   `low_power.rs` / `pwr.rs` (低功耗和电源管理)
*   `sdmmc.rs` / `sd_device.rs` (SD卡接口)
*   `usart.rs` (串口通信)
*   `utils.rs` (工具函数)

### 已知缺失或不完整的功能
*   **DMA 异步实现不完整 (`src/dma.rs`)**：
    *   中断处理函数被注释，导致异步等待可能永久挂起。
    *   所有通道共享同一个全局 `WAKER`，并发时会互相覆盖。
