609

Reset and clock control (RCC)

RM0456
Figure 40. MSI block diagram

/1
/2

MSIRC0
48 MHz

/3
/4

/1

MSIK

/2

MSIRC1
4 MHz

/3
/4

/1

MSIS

/2

MSIRC2
3.072 MHz

/3
/4

/1
/2

MSIRC3
400 kHz

/3
/4
MSv64329V1

Refer to datasheet for complete MSI frequency characteristics in MSI-mode and in PLLmode. MSIS and MSIK frequency ranges can be adjusted by software, by using respectively
MSISRANGE[3:0] and MSIKRANGE[3:0] in RCC_ICSCR1, with MSIRGSEL = 1.
Sixteen frequency ranges are available, generated from the four internal RCs, as shown
in the table below.
Table 113. MSIS and MSIK ranges per internal MSIRCs (PLL_mode disabled)(1)
MSIRC0

MSIRC1

MSIRC2

MSIRC3

Range 0: 48 MHz

Range 4: 4 MHz

Range 8: 3.072 MHz

Range 12: 400 kHz

Range 1: 24 MHz

Range 5: 2 MHz

Range 9: 1.536 MHz

Range 13: 200 kHz

Range 2: 16 MHz

Range 6: 1.33 MHz

Range 10: 1.024 MHz

Range 14: 133 kHz

Range 3: 12 MHz

Range 7: 1 MHz

Range 11: 0.768 MHz

Range 15: 100 kHz

1. Refer to datasheet for complete MSI frequency characteristics in MSI-mode and in PLL-mode.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Reset and clock control (RCC)
The MSIS clock is used as system clock after restart from reset, wake-up from Standby, and
Shutdown low-power modes. After restart from reset or when exiting Shutdown mode, MSIS
and MSIK frequencies are set to their default value 4 MHz. The frequency range at wake-up
from Standby mode can be adjusted by software, using respectively MSISSRANGE[3:0] and
MSIKSRANGE[3:0] with MSIRGSEL = 0 (refer to RCC_CSR).
The MSIS clock can be selected as system clock after a wake-up from Stop mode (Stop 0,
Stop 1, Stop 2, or Stop 3) depending on STOPWUCK in RCC_CR. It can also be used as a
backup clock source (auxiliary clock) if the HSE crystal oscillator fails. See Section 11.4.11.
The MSI advantage is to provide a low-cost (no external components) low-power clock
source. In addition, when used in PLL-mode with the LSE, the MSI provides a very accurate
clock source that can be used by the OTG_FS, or the USB, and feeds the PLL.
MSISRDY and MSIKRDY in RCC_CR indicate whether the MSIS and MSIK RC are stable
or not. At startup, MSIS and MSIK RC output clocks are not released until their respective
bit is set by hardware. The MSIS and MSIK RC can be switched on and off by using
MSISON and MSIKON in RCC_CR.

Hardware auto calibration with LSE (PLL-mode)
When a 32.768 kHz external oscillator is present in the application, it is possible to configure
either the MSIS or the MSIK in a PLL-mode. This mode is enabled:
•

for MSIS by setting MSIPLLEN with MSIPLLSEL = 1 in RCC_CR

•

for MSIK by setting MSIPLLEN with MSIPLLSEL = 0

In case MSIS and MSIK ranges are generated from the same MSIRC source, the PLL-mode
is applied on both MSIS and MSIK. When configured in PLL-mode, the MSIS or MSIK
automatically calibrates itself thanks to the LSE. This mode is available for all MSI frequency
ranges. At 48 MHz, the MSIK in PLL-mode can be used for the OTG_FS, or the USB,
avoiding the need of an external high-speed crystal.
If LSE clocks pulses are stopped, the MSI PLL-mode is automatically unlocked, and
the MSI accuracy is consequently degraded. On all STM32U5 devices except
STM32U575/585 rev. X, the MSI PLL-mode unlock event is connected to an EXTI line: this
is used to generate an event or interrupt supporting wake-up from Stop 0, Stop 1, or Stop 2
mode (see Table 118: Interrupt sources and control and Table 190: EXTI line connections).

MSI PLL-mode stabilization time
When MSIPLLEN = 1, the final accuracy after enabling the MSI (by writing MSISON = 1 or
MSIKON = 1 or following a peripheral clock request in Stop mode) is reached after a
stabilization time tSTAB(MSI) when MSIPLLFAST = 0. This stabilization time is needed even
if the LSE is kept enable. Refer to datasheet for tSTAB(MSI) value.
If MSIPLLEN = 1 with MSIPLLFAST =1 , the MSI oscillator is kept powered on when a
request to switch it off is received (either by writing MSISON = 0 and MSIKON =0 , or
because no peripheral requests this clock in Stop mode). In this case the MSI PLL-mode
accuracy is kept when the MSI is switched on again, providing that the tSTAB(MSI)
stabilization time is reached before switching off the MSI. This mode can be used for
autonomous peripherals requiring accuracy in Stop mode, with an extra consumption as the
oscillator remains powered on, but gated off when disabled.

RM0456 Rev 6

<!-- pagebreak -->

