RM0456 Rev 6

RM0456

Power control (PWR)
Several peripherals are autonomous in Stop mode, and can generate interrupts with
wake-up from Stop capability. All peripheral clocks must be enabled to allow a wake-up from
Stop interrupt (see Peripheral clock gating).
When exiting the Stop 2 mode, the MCU is in Run mode, range 4.
Table 104. Stop 2 mode

Stop 2 mode

Description
WFI (wait for interrupt) or WFE (wait for event) while:
– SLEEPDEEP bit is set in Cortex-M33 system control register
– No interrupt (for WFI) or event (for WFE) pending
– LPMS = 010 in PWR_CR1

Mode entry

On Return from ISR while:
– SLEEPDEEP bit is set in Cortex-M33 system control register
– SLEEPONEXIT = 1
– No interrupt pending
– LPMS = 010 in PWR_CR1
Note: To enter Stop 2 mode, all EXTI line pending bits (in EXTI_RPR2), and the peripheral flags
generating wake-up interrupts must be cleared. Otherwise, the Stop mode entry
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
- any EXTI line configured in interrupt mode (even if the corresponding EXTI Interrupt vector is
disabled in the NVIC). The interrupt source can be external interrupts or peripherals with
wake-up capability (see Table 187: STM32U5 series vector table).
- any EXTI line configured in event mode (see Section 23.3: EXTI functional description).
- RTC, TAMP, IWDG interrupts, or any other peripheral interrupt occurring when the AHB/APB
clocks are present due to an autonomous peripheral clock request.
Note: All peripheral clocks must be enabled to allow this peripheral to generate a wake-up from
Stop interrupt ([PERIPH]EN, [PERIPH]SMEN and [PERIPH]AMEN bits must be set in the
RCC, and a functional independent clock must be selected).

Wake-up
latency

10.7.9

Longest wake-up time between: MSIS or HSI16 wake-up time and regulator wake-up time from
low-power mode + FLASH wake-up time from Stop 2 mode.

Stop 3 mode
The Stop 3 mode is based on the Cortex-M33 Deepsleep mode combined with peripheral
clock gating. In Stop 3 mode, all clocks in the core domain are stopped. The PLL, MSIS,
MSIK, HSI16, and HSE oscillators are disabled.
All SRAMs and register contents are preserved, but the SRAMs can be totally or partially
switched off to further reduce consumption.
RM0456 Rev 6

<!-- pagebreak -->

482

Power control (PWR)

RM0456

The BOR is always available in Stop 3 mode.
All other peripherals must be either disabled by clearing the enable bit in the peripheral
itself, or put under reset state by configuring RCC registers.

I/O states in Stop 3 mode
In the Stop 3 mode, the I/Os are by default in floating state. If the APC bit in the PWR_APCR
register is set, the I/Os can be configured either with a pull-up (see PWR_PUCRx registers),
or with a pull-down (see PWR_PDCRx registers), or can be kept in analog state if none of
the PWR_PUCRx or PWR_PDCRx register is set. The pull-down configuration has highest
priority over pull-up configuration in case both PWR_PUCRx and PWR_PDCRx are set for
the same I/O. After wake-up from Stop 3 mode, the pull-up/pull-down I/O configuration
remains retained based on the PWR_PUCRx/PWR_PDCRx registers as long as the APC
bit is set.
Some I/Os (listed in Section 13: General-purpose I/Os (GPIO)) are used for JTAG/SW
debug and can only be configured to their respective reset pull-up or pull-down state during
Stop 3 mode setting their respective bit to 1 in the PWR_PUCRx or PWR_PDCRx registers,
or to be configured to floating state if the bit is kept at 0.
The RTC outputs on PC13 and PB2 are functional in Stop 3 mode. PC14 and PC15 used for
LSE are also functional. The 24 wake-up pins multiplexed on eight events
(WKUPx, x = 1 to 8) and the eight RTC tampers pins are available.

Entering Stop 3 mode
The MCU enters the Stop 3 mode as described in Entering into a low-power mode, when the
SLEEPDEEP bit in the Cortex-M33 System Control register is set (see Table 105 for details
on how to enter the Stop 3 mode).
If the flash memory programming is ongoing, the Stop 3 mode entry is delayed until the
memory access is finished.
If an access to the APB domain is ongoing, the Stop 3 mode entry is delayed until the APB
access is finished.
In Stop 3 mode, the following features can be selected by programming individual control
bits:
•

The independent watchdog (IWDG) is started by writing to its key register or by
hardware option. Once started it cannot be stopped except by a reset (see
Section 23.3: IWDG functional description).

•

The real-time clock (RTC) and Tamper (TAMP) kernel clock enabled by the RTCEN bit
in RCC_BDCR.

•

The internal RC oscillator LSI clock or LSI clock divided by 128, is configured by the
LSION and LSIPREDIV bits in RCC_BDCR.

•

The external 32.768 kHz oscillator (LSE) is configured by the LSEON bit in
RCC_BDCR.

Exiting Stop 3 mode
The MCU exits the Stop 3 mode as described in Exiting a low-power mode (see Table 105
for details on how to exit Stop 3 mode).
When exiting Stop 3 mode by issuing an interrupt or a wake-up event, HSI16 is selected as
system clock if the STOPWUCK bit is set in RCC_CFGR1. MSIS is selected as system

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Power control (PWR)
clock if STOPWUCK is cleared. The MSIS selection allows a wake-up at higher frequency
(up to 24 MHz).
When exiting Stop 3 mode, I/Os that were configured with pull-up or pull-down during Stop 3
mode through PWR_PUCRx or PWR_PDCRx registers keep this configuration upon exiting
Stop 3 mode until the APC bit in PWR_CR3 is cleared by software. Once APC is cleared,
the I/Os pull-up/pull-down state is configured according to the GPIOx_PUPDR registers.
The content of the PWR_PUCRx or PWR_PDCRx registers is not lost and can be re-used
for a sub-sequent entering into Stop 3 mode.
Figure 35. I/O states in Stop 3 mode
IO state retention disabled
GPIO mode

Normal

Floating

Normal

System mode

Run

Stop 3

Run

Stop 3 entry

WFI/WFE/Sleep on exit

APC (PWR)
Wakeup request
IO state retention enabled
GPIO mode

Normal

State retained (PU/PD)

System mode

Run

Stop 3 entry

WFI/WFE/Sleep on exit

APC (PWR)

Normal

Stop 3

Set

Run

Clear

Wakeup request
MSv66126V1

When exiting the Stop 3 mode, the MCU is in Run mode, range 4.

RM0456 Rev 6

<!-- pagebreak -->

