482

Power control (PWR)

RM0456
Table 102. Stop 0 mode

Stop 0 mode

Description
WFI (wait for interrupt) or WFE (wait for event) while:
– SLEEPDEEP bit is set in Cortex-M33 system control register
– No interrupt (for WFI) or event (for WFE) pending
– LPMS = 000 in PWR_CR1

Mode entry

On Return from ISR while:
– SLEEPDEEP bit is set in Cortex-M33 system control register
– SLEEPONEXIT = 1
– No interrupt pending
– LPMS = 000 in PWR_CR1
Note: To enter Stop 0 mode, all EXTI line pending bits (in EXTI_RPR2), and the peripheral flags
generating wake-up interrupts must be cleared. Otherwise, the Stop 0 mode entry
procedure is ignored and the program execution continues.
If WFI or Return from ISR was used for entry:
- any EXTI line configured in interrupt mode (the corresponding EXTI interrupt vector must be
enabled in the NVIC). The interrupt source can be external interrupts or peripherals with
wake-up capability (see Table 187: STM32U5 series vector table).
- RTC, TAMP, IWDG interrupts, or any other peripheral interrupt occurring when the AHB/APB
clocks are present due to an autonomous peripheral clock request (the peripheral vector must
be enabled in the NVIC)
If WFE was used for entry and SEVONPEND = 0:
- any EXTI line configured in event mode (see Section 23.3: EXTI functional description).

Mode exit

If WFE was used for entry and SEVONPEND = 1:
- any EXTI line configured in interrupt mode (even if the corresponding EXTI interrupt vector is
disabled in the NVIC). The interrupt source can be external interrupts or peripherals with
wake-up capability (see Table 187: STM32U5 series vector table).
- any EXTI line configured in event mode (see Section 23.3: EXTI functional description)
- RTC, TAMP, IWDG interrupts, or any other peripheral interrupt occurring when the AHB/APB
clocks are present due to an autonomous peripheral clock request
Note: All peripheral clocks must be enabled to allow this peripheral to generate a wake-up from
Stop interrupt ([PERIPH]EN, [PERIPH]SMEN and [PERIPH]AMEN bits must be set in the
RCC, and a functional independent clock must be selected).

Wake-up
latency

<!-- pagebreak -->

