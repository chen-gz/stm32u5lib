482

Power control (PWR)

RM0456
Table 107. Shutdown mode

Shutdown mode

Description
WFI (wait for interrupt) or WFE (wait for event) while:
– SLEEPDEEP bit is set in Cortex-M33 system control register
– No interrupt (for WFI) or event (for WFE) pending
– LPMS = 11X in PWR_CR1 with BREN = 0 in PWR_BDCR1
– WUFx bits cleared in PWR_WUSR

Mode entry

On Return from ISR while:
– SLEEPDEEP bit is set in Cortex-M33 system control register
– SLEEPONEXT = 1
– No interrupt pending
– LPMS = 11X in PWR_CR1
– WUFx bits cleared in PWR_WUSR
– RTC/TAMP flags corresponding to the chosen wake-up source, cleared

Mode exit

WKUPx pin edge, RTC/TAMP event/interrupt, NRST pin external reset

Wake-up latency

10.7.12

Reset phase

USB power management in low-power modes
(STM32U59x/5Ax/5Fx/5Gx only)
In Stop 0 and Stop 1 modes, it is possible to keep the OTG_HS configuration by leaving the
USBPWREN bit set. This allows the OTG_HS to wake up the MCU from Stop mode.
However, in order to decrease the power consumption, it is recommended to shut off the
OTG_HS before entering Stop 0 or Stop 1 mode.
In Stop 2 and Stop 3 modes, it is not possible to keep the OTG_HS configuration. The
OTG_HS must be shut off before entering Stop 2 or Stop 3 mode.
The following steps are needed to shut off the OTG_HS before entering Stop mode:
1.

Clear USBPWREN and USBBOOSTEN bits in PWR_VOSR.

2.

Request entry in Stop mode.

Upon wake-up from Stop mode, and before configuring the OTG_HS:
1.

Make sure the voltage scaling is in range 1 or in range 2 (using VOS[1:0]
in PWR_VOSR).

2.

Make sure the EPOD booster clock is enabled (using PLL1MBOOST[3:0]
in RCC_PLL1CFGR)

3.

Enable the USB internal power by setting USBPWREN and USBBOOSTEN bits
in PWR_VOSR.

4.

Wait for USBBOOSTRDY in PWR_VOSR to be set.

Using PA11 and PA12 GPIOs
When PA11 and PA12 are used as OTG_HS_DM and OTG_HS_DP additional functions,
GPIOs must be configured in analog mode (default setting).
When PA11 and PA12 are used as standard GPIOs, set first the USV bit in the
PWR_SVMCR register, then the USBPWREN and VDD11USBDIS bits in the PWR_VOSR
register must be set prior to configure the GPIOs in a mode other than analog.

<!-- pagebreak -->

