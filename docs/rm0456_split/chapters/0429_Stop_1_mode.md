Longest wake-up time between: MSIS or HSI16 wake-up time and FLASH wake-up time from
Stop 0 mode.

RM0456 Rev 6

RM0456

10.7.7

Power control (PWR)

Stop 1 mode
The Stop 1 mode is the same as Stop 0 mode except that the regulator is in low-power
mode (see the table below for details on how to enter and exit Stop 1 mode).
Table 103. Stop 1 mode

Stop 1 mode

Description
WFI (wait for interrupt) or WFE (wait for event) while:
– SLEEPDEEP bit is set in Cortex-M33 system control register
– No interrupt (for WFI) or event (for WFE) pending
– LPMS = 001 in PWR_CR1

Mode entry

On Return from ISR while:
– SLEEPDEEP bit is set in Cortex-M33 system control register
– SLEEPONEXIT = 1
– No interrupt pending
– LPMS = 001 in PWR_CR1
Note: To enter Stop 1 mode, all EXTI line pending bits (in EXTI_RPR1), and the peripheral flags
generating wake-up interrupts must be cleared. Otherwise, the Stop 1 mode entry
procedure is ignored and the program execution continues.
If WFI or Return from ISR was used for entry
- any EXTI line configured in interrupt mode (the corresponding EXTI interrupt vector must be
enabled in the NVIC). The interrupt source can be external interrupts or peripherals with
wake-up capability (see Table 187: STM32U5 series vector table).
- RTC, TAMP, IWDG interrupts, or any other peripheral interrupt occurring when the AHB/APB
clocks are present due to an autonomous peripheral clock request (the peripheral vector must
be enabled in the NVIC)
If WFE was used for entry and SEVONPEND = 0:
- any EXTI line configured in event mode (see Section 23.3: EXTI functional description)

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

wake-up latency

10.7.8

Longest wake-up time between: MSIS or HSI16 wake-up time and regulator wake-up time from
low-power mode + FLASH wake-up time from Stop 1 mode.

Stop 2 mode
The Stop 2 mode is similar to Stop 1 except that most of the core domain is put in a lower
leakage mode. Only the part of the core domain embedding AHB3 and APB3 peripherals
remains fully powered, allowing those peripherals to be functional.
The AHB3 and APB3 peripherals with the autonomous mode capability can switch on
HSI16, or MSIS, or MSIK for transferring data (see Section 10.7.2 for more details).

RM0456 Rev 6

<!-- pagebreak -->

482

Power control (PWR)

RM0456

All SRAMs and register contents are preserved, but the SRAMs can be totally or partially
switched off to further reduced consumption.
The BOR is always available in Stop 2 mode.

I/O states in Stop 2 mode
In Stop 2 mode, all I/O pins keep the same state as in Run mode.

Entering Stop 2 mode
The MCU enters Stop 2 mode as described in Entering into a low-power mode, when the
SLEEPDEEP bit in the Cortex-M33 system control register is set (see Table 104 for details
on how to enter the Stop 2 mode).
If the flash memory programming is ongoing, the Stop 2 mode entry is delayed until the
memory access is finished.
If an access to the APB domain is ongoing, the Stop 2 mode entry is delayed until the APB
access is finished.
In Stop 2 mode, the following features can be selected by programming individual control
bits:
•

The independent watchdog (IWDG) is started by writing to its key register or by
hardware option. Once started it cannot be stopped except by a reset
(see Section 61.4: IWDG functional description).

•

The real-time clock (RTC) and Tamper (TAMP) kernel clock enabled by the RTCEN bit
in RCC_BDCR.

•

The internal RC oscillator LSI clock or LSI clock divided by 128, is configured by the
LSION and LSIPREDIV bits in RCC_BDCR.

•

The external 32.768 kHz oscillator (LSE) is configured by LSEON in RCC_BDCR.

Several peripherals can be autonomous in Stop 2 mode and can add consumption if they
are enabled (see Section 10.7.2 for more details).
OPAMPs, COMPs, the PVM, and the PVD can be used in Stop 2 mode. If they are not
needed, they must be disabled by software to save their power consumptions.
The ADCx (x = 1, 2, 4), the DAC1 (two channels), the temperature sensor and the
VREFBUF can consume power during Stop 2 mode, unless they are disabled before
entering this mode.
Caution:

All the peripherals that cannot be enabled in Stop 2 mode must be either disabled by
clearing the enable bit in the peripheral itself, or put under reset state by configuring RCC
registers.

Exiting Stop 2 mode
The MCU exits Stop 2 mode as defined in Exiting a low-power mode (see Table 104 for
details on how to exit Stop 2 mode).
When exiting Stop 2 mode by issuing an interrupt or a wake-up event, HSI16 is selected as
system clock if the bit STOPWUCK is set in RCC_CFGR1. MSIS is selected as system
clock if STOPWUCK is cleared. The MSI selection allows a wake-up at higher frequency
(up to 24 MHz).

<!-- pagebreak -->

