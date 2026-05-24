482

Power control (PWR)

RM0456
Table 105. Stop 3 mode

Stop 3 mode

Description
WFI (wait for interrupt) or WFE (wait for event) while:
– SLEEPDEEP bit is set in Cortex-M33 system control register
– No interrupt (for WFI) or event (for WFE) pending
– LPMS =“011 in PWR_CR1
– WUFx bits cleared in PWR_WUSR

Mode entry

On Return from ISR while:
– SLEEPDEEP bit is set in Cortex-M33 system control register
– SLEEPONEXIT = 1
– No interrupt pending
– LPMS = 011 in PWR_CR1
– WUFx bits cleared in PWR_WUSR
– RTC/TAMP flags corresponding to the chosen wake-up source, cleared
Note: To enter Stop 3 mode, all WUFx, and the RTC/TAMP flags generating wake-up interrupts
must be cleared. Otherwise, the Stop 3 mode entry procedure is completed but the Stop
3 is exited immediately after entry.

Mode exit

WKUPx pin edge, RTC/TAMP event/interrupt, NRST pin external reset, IWDG reset, BOR reset

wake-up latency

Longest wake-up time between: MSIS or HSI16 wake-up time and regulator wake-up time from
low-power mode + FLASH wake-up time from Stop 3 mode.

10.7.10

Standby mode
The lowest power mode in which the BOR is active is the Standby mode. It is based on the
Cortex-M33 Deepsleep mode, with the voltage regulators disabled (except when SRAM2
content is preserved). The PLL, HSI16, MSIS, MSIK and HSE oscillators are also
switched off.
The SRAMs and register contents are lost except for registers in the backup domain and
Standby circuitry (see Figure 32). SRAM2 content can be partially or fully preserved
depending on RRSB1 and RRSB2 bits configuration in PWR_CR1. In this case,
the low-power regulator is ON and provides the supply to SRAM2 only.
The BOR is always available in Standby mode. ULPMEN in PWR_CR1 must be configured
to 1 to reach the lowest power consumption by forcing the BOR in ultra-low-power mode
(only available when BOR level 0 is selected).

I/O states in Standby mode
In the Standby mode, the I/Os are by default in floating state. If APC bit is set
in PWR_APCR, the I/Os can be configured either with a pull-up (see PWR_PUCRx), or with
a pull-down (see PWR_PDCRx), or can be kept in analog state if none of the PWR_PUCRx
or PWR_PDCRx register is set. The pull-down configuration has highest priority over pull-up
configuration in case both PWR_PUCRx and PWR_PDCRx are set for the same I/O. After
wake-up from Standby mode, the pull-up/pull-down I/O configuration remains retained
based on PWR_PUCRx/PWR_PDCRx registers as long as the APC bit is set.
Some I/Os (listed in Section 13: General-purpose I/Os (GPIO)) are used for JTAG/SW
debug and can only be configured to their respective reset pull-up or pull-down state during

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Power control (PWR)
Standby mode setting their respective bit to 1 in the PWR_PUCRx or PWR_PDCRx
registers, or to be configured to floating state if the bit is kept at 0.
The RTC outputs on PC13 and PB2 are functional in Standby mode. PC14 and PC15 used
for LSE are also functional. The 24 wake-up pins multiplexed on eight events
(WKUPx, x = 1 to 8) and the eight RTC tampers pins are available.

Entering Standby mode
The MCU enters the Standby mode as described in Entering into a low-power mode, when
the SLEEPDEEP bit in the Cortex-M33 system control register is set (see Table 106 for
details on how to enter Standby mode).
In Standby mode, the following features can be selected by programming individual control
bits:
•

The independent watchdog (IWDG) is started by writing to its Key register or by
hardware option. Once started it cannot be stopped except by a reset
(see Section 61.4: IWDG functional description).

•

The real-time clock (RTC) and Tamper (TAMP) kernel clock enabled by the RTCEN bit
in RCC_BDCR.

•

The internal RC oscillator LSI clock or LSI clock divided by 128, is configured by the
LSION and LSIPRE bits in RCC_BDCR.

•

The external 32.768 kHz oscillator (LSE) is configured by the LSEON bit in
RCC_BDCR.

Exiting Standby mode
The MCU exits the Standby mode as described in Exiting a low-power mode. The SBF
status flag in PWR status register (PWR_SR) indicates that the MCU was in Standby mode.
All registers are reset after wake-up from Standby except for PWR control register 3
(PWR_CR3) (see Table 106 for more details on how to exit Standby mode).
When exiting Standby mode, I/Os that were configured with pull-up or pull-down during
Standby through PWR_PUCRx or PWR_PDCRx, keep this configuration upon exiting
Standby mode until the APC bit in PWR_CR3 is cleared by software. The application can
release the retained I/O state (clear the retained pull-up/pull-down) by clearing the APC bit
after reconfiguring the GPIOs and related peripherals. Once APC is cleared, the I/Os state
is configured according to GPIOx registers. The content of the PWR_PUCRx or
PWR_PDCRx registers is not lost and can be re-used for a sub-sequent entering into
Standby mode.
Some I/Os (listed in Section 13: General-purpose I/Os (GPIO)) are used for JTAG/SW
debug and have internal pull-up or pull-down activated after reset so is configured at this
reset value, as well when exiting Standby mode.
For I/Os, with a pull-up or pull-down pre-defined after reset (some JTAG/SW I/Os) or with
the GPIOx_PUPDR programming done after exiting from Standby, in case those
programming is different from the PWR_PUCRx or PWR_PDCRx programmed value during
Standby, both a pull-down and pull-up are applied until APC is cleared, releasing the
PWR_PUCRx or PWR_PDCRx programmed value.

RM0456 Rev 6

<!-- pagebreak -->

