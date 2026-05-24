482

Power control (PWR)

RM0456

Entering Sleep mode
The MCU enters the Sleep mode as described in Entering into a low-power mode, when the
SLEEPDEEP bit in the Cortex-M33 system control register is clear (see Table 101 for details
on how to enter Sleep mode).

Exiting Sleep mode
The MCU exits the Sleep mode as described in Exiting a low-power mode (see Table 101
for details on how to exit Sleep mode).
Table 101. Sleep mode
Sleep mode

Description
WFI (wait for interrupt) or WFE (wait for event) while:
– SLEEPDEEP = 0
– No interrupt (for WFI) or event (for WFE) pending
Refer to the Cortex-M33 system control register.

Mode entry

On return from ISR while:
– SLEEPDEEP = 0 and
– SLEEPONEXIT = 1
– No interrupt pending
Refer to the Cortex-M33 system control register.
If WFI or Return from ISR was used for entry
Interrupt (see Table 187: STM32U5 series vector table)

Mode exit

If WFE was used for entry and SEVONPEND = 0:
wake-up event (see Section 23.3: EXTI functional description)
If WFE was used for entry and SEVONPEND = 1:
Interrupt even when disabled in NVIC (see Table 187: STM32U5 series vector table) or
wake-up event (see Section 23.3: EXTI functional description)

wake-up latency

10.7.6

None

Stop 0 mode
The Stop 0 mode is based on the Cortex-M33 Deepsleep mod combined with the peripheral
clock gating. The voltage regulator is configured in main regulator mode. In Stop 0 mode, all
clocks in the core domain are stopped. The PLL, MSIS, MSIK, HSI16 and HSE oscillators
are disabled.
Some peripherals with the autonomous mode capability can switch on HSI16 or MSIS or
MSIK for transferring data (see Section 10.7.2 for details).
All SRAMs and register contents are preserved, but the SRAMs can be totally or partially
switched off to further reduced consumption.
The BOR is always available in Stop 0 mode.

I/O states in Stop 0 mode
In Stop 0 mode, all I/O pins keep the same state as in the Run mode.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Power control (PWR)

Entering Stop 0 mode
The MCU enters the Stop 0 mode as described in Entering into a low-power mode, when the
SLEEPDEEP bit in the Cortex-M33 system control register is set (see Table 102 for details
on how to enter Stop 0 mode).
If the flash memory programming is ongoing, the Stop 0 mode entry is delayed until the
memory access is finished.
If an access to the APB domain is ongoing, the Stop 0 mode entry is delayed until the APB
access is finished.
In Stop 0 mode, the following features can be selected by programming the individual
control bits:
•

The independent watchdog (IWDG) is started by writing to its key register or by
hardware option. Once started, it cannot be stopped except by a reset
(see Section 61.4: IWDG functional description).

•

The real-time clock (RTC) and Tamper (TAMP) kernel clock enabled by RTCEN in
RCC_BDCR.

•

The internal RC oscillator LSI clock or LSI clock divided by 128, is configured by LSION
and LSIPREDIV bits in RCC_BDCR.

•

The external 32.768 kHz oscillator (LSE) is configured by LSEON bit in RCC_BDCR.

Several peripherals can be autonomous in Stop 0 mode and can add consumption if they
are enabled (see Section 10.7.2 for more details).
OPAMPs, COMPs, the PVM, and the PVD can be used in Stop 0 mode. If they are not
needed, they must be disabled by software to save their power consumptions.
The ADCx (x = 1, 4), the DAC1 (two channels), the temperature sensor and the VREFBUF
can consume power during Stop 0 mode, unless they are disabled before entering
this mode.

Exiting Stop 0 mode
The MCU exits Stop 0 mode as described in Exiting a low-power mode (see Table 102 for
details on how to exit Stop 0 mode).
When exiting Stop 0 mode by issuing an interrupt or a wake-up event, HSI16 is selected as
system clock if STOPWUCK is set in RCC_CFGR1. The MSIS oscillator is selected as
system clock if STOPWUCK is cleared. The MSIS selection allows a wake-up at higher
frequency (up to 24 MHz).
Several peripherals are autonomous in Stop mode, and can generate interrupts with
wake-up from Stop capability. All peripheral clocks must be enabled to allow a wake-up
from Stop interrupt (see Peripheral clock gating).
When exiting the Stop 0 mode, the MCU is in Run mode, range 4.

RM0456 Rev 6

<!-- pagebreak -->

