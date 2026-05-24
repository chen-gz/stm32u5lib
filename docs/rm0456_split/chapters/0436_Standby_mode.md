482

Power control (PWR)

RM0456
Figure 36. I/O states in Standby mode

IO state retention disabled
GPIO mode

Normal

Floating

Normal (default after reset)

System mode

Run

Standby (Vcore off)

Run

Standby entry

WFI/WFE/Sleep on exit

APC (PWR)
Wakeup request
IO state retention enabled
GPIO mode

Normal

State retained (PU/PD)
Standby (Vcore off)

System mode

Run

Standby entry

WFI/WFE/Sleep on exit

APC (PWR)

Normal
Run

Clear

Set

Wakeup request
MSv66125V1

Table 106. Standby mode
Standby mode

Description
WFI (wait for interrupt) or WFE (wait for event) while:
– SLEEPDEEP bit is set in Cortex-M33 system control register
– No interrupt (for WFI) or event (for WFE) pending
– LPMS = 10x in PWR_CR1 (1)
– WUFx bits cleared in PWR_WUSR

Mode entry

On Return from ISR while:
– SLEEPDEEP bit is set in Cortex-M33 system control register
– SLEEPONEXIT = 1
– No interrupt pending
– LPMS = 10x in PWR_CR1(1)
– WUFx bits cleared in PWR_WUSR
– RTC/TAMP flags corresponding to the chosen wake-up source, cleared

Mode exit

WKUPx pin edge, RTC/TAMP event/interrupt, NRST pin external reset, IWDG reset, BOR reset

Wake-up latency Reset phase
1. The Standby mode is also entered if LPMS = 11X in PWR_CR1 with BREN = 1 in PWR_BDCR1.

10.7.11

Shutdown mode
The lowest power consumption is reached in Shutdown mode. It is based on the Deepsleep
mode with the voltage regulator disabled. The core domain is consequently powered off.
The PLL, HSI16, MSIS, MSIK and HSE oscillators are also switched off.
The SRAMs and register contents are lost except for registers in the backup domain. The
BOR is not available in Shutdown mode. No power voltage monitoring is possible in this
mode, therefore the switch to backup domain is not supported: VBAT mode is not

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Power control (PWR)
supported, i.e the RTC, and TAMP are not functional, and the backup SRAM content is not
guaranteed if VDD is powered off during Shutdown mode.

I/O states in Shutdown mode
In the Shutdown mode, I/Os are by default in floating state. If the APC bit in the PWR_APCR
register is set, the I/Os can be configured either with a pull-up (see PWR_PUCRx registers
(x = A to J)), or with a pull-down (see PWR_PDCRx registers (x = A to J)), or can be kept in
analog state if none of PWR_PUCRx or PWR_PDCRx register is set. The pull-down
configuration has highest priority over pull-up configuration in case both PWR_PUCRx and
PWR_PDCRx are set for the same I/O. However this configuration is lost when exiting the
Shutdown mode due to the power-on reset.
Some I/Os (listed in Section 13: General-purpose I/Os (GPIO)) are used for JTAG/SW
debug and can only be configured to their respective reset pull-up or pull-down state during
Shutdown mode setting to 1 their respective bit in the PWR_PUCRx or PWR_PDCRx
registers, or to be configured to floating state if the bit is kept at 0.
The RTC outputs on PC13 and PB2 are functional in Shutdown mode. PC14 and PC15
used for LSE are also functional. The 24 wake-up pins multiplexed on eight events (WKUPx,
x = 1 to 8) and the eight RTC tampers pins are available.

Entering Shutdown mode
The MCU enters the Shutdown mode as described in Entering into a low-power mode, when
the SLEEPDEEP bit in the Cortex-M33 system control register is set (see Table 107 for
details on how to enter Shutdown mode).
In Shutdown mode, the following features can be selected by programming individual
control bits:

Caution:

•

The real-time clock (RTC) and Tamper (TAMP) kernel clock enabled by the RTCEN bit
in the backup domain control register (RCC_BDCR). Caution: in case of VDD
power-down, the RTC content is lost.

•

The external 32.768 kHz oscillator (LSE) is configured by the LSEON bit in the backup
domain control register (RCC_BDCR).

The Shutdown mode cannot be entered if the BREN bit is set in the PWR backup domain
control register 1 (PWR_BDCR1). If BREN = 1, the Standby mode is entered instead of
Shutdown mode.

Exiting Shutdown mode
The MCU exits the Shutdown mode as described in Exiting a low-power mode. A power-on
reset occurs when exiting from Shutdown mode. All registers (except for the ones in the
backup domain) are reset after a wake-up from Shutdown (see Table 107 for more details
on how to exit Shutdown mode).
When exiting Shutdown mode, I/Os that were configured with pull-up or pull-down during
Shutdown through registers PWR_PUCRx or PWR_PDCRx lose their configuration and are
configured in floating state or to their pull-up pull-down reset value (for some I/Os listed
in Section 13: General-purpose I/Os (GPIO)).

RM0456 Rev 6

<!-- pagebreak -->

