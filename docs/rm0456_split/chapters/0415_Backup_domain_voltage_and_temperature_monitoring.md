If VDDA is independent from VDD:
a)

Enable the AVM1 or AVM2 by setting AVM1EN or AVM2EN in PWR_SVMCR.

b)

Wait for the AVM wake-up time.

c)

Wait until VDDA1RDY or VDDA2RDY is set in PWR_SVMCR.

d)

Disable the AVM for consumption saving (optional).

Set the ASV in PWR_SVMCR to remove the VDDA power isolation.

PVM is not functional in Shutdown mode.

RM0456 Rev 6

RM0456

10.6.4

Power control (PWR)

Backup domain voltage and temperature monitoring
When the backup domain voltage and temperature monitoring is enabled (MONEN = 1
in PWR_DBPR), the backup domain voltage and the temperature are monitored. This
monitoring is not functional in Shutdown mode.
If the backup domain voltage monitoring internal tamper is enabled in the TAMP peripheral
(ITAMP1E = 1 in TAMP_CR1), a tamper event is generated when the backup domain
voltage is above the functional range. In case the backup domain voltage is below the
functional range, a brownout reset is generated, erasing all device including backup domain.

Note:

The backup domain voltage is VDD when present, VBAT otherwise.
If the temperature monitoring internal tamper is enabled in the TAMP peripheral
(ITAMP2E = 1 in TAMP_CR1), a tamper event is generated when the temperature is above
or below the functional range.

Note:

Backup domain voltage and temperature monitoring is not functional in Shutdown mode.

10.7

PWR power management

10.7.1

Power modes
By default, the microcontroller is in Run mode after a system or a power reset. Reducing the
power consumption in Run mode is done by configuring the voltage scaling according to
application performance needs. Refer to Section 10.5.4 for more details. Unused RAMs can
be powered-off with SRAMxPD bits in PWR_CR1. The power consumption is also reduced
by reducing SYSCLK, HCLK, and PCLK clocks speed, or gating unused peripherals clocks.
Refer to Section 11: Reset and clock control (RCC) for more details.
Several low-power modes are available to save power when the CPU does not need to be
kept running, for example when waiting for an external event. It is up to the user to select the
mode that gives the best compromise between low-power consumption, short startup time
and available wake-up sources.
The device features these low-power modes:
•

Sleep mode:
CPU clock off, all peripherals including Cortex-M33 core such as NVIC and SysTick
can run and wake up the CPU when an interrupt or an event occurs. Refer to
Section 10.7.5.

•

Stop 0, Stop 1, Stop 2, Stop 3 modes:
Stop mode achieves the lowest power consumption while retaining the content of
SRAM and registers. The SRAMs can be totally or partially switched off to further
reduce consumption. All clocks in the core domain are stopped. The PLL, the MSI
(MSIS and MSIK) RC, the HSI16 RC and the HSE crystal oscillators are disabled. The
LSE or LSI is still running.
The RTC can remain active (Stop mode with RTC, Stop mode without RTC).
Some peripherals are autonomous and can operate in Stop mode by requesting their
kernel clock and their bus (APB or AHB) when needed, in order to transfer data with
DMA (GPDMA1 or LPDMA1 depending on peripherals and power mode).
In Stop 2 and Stop 3 modes, most of the core domain is put in a lower leakage mode.
Stop 0 and Stop 1 offers the largest number of active peripherals and wake-up sources,

RM0456 Rev 6

<!-- pagebreak -->

482

Power control (PWR)

RM0456

a smaller wake-up time but a higher consumption than Stop 2.
In Stop 0 mode, the regulator remains in main regulator mode, allowing a very fast
wake-up time but with much higher consumption.
Stop 3 is the lowest power mode with full retention, but the functional peripherals and
sources of wake-up are reduced to the same ones than in Standby mode.
The system clock when exiting from Stop mode can be either MSIS up to 24 MHz or
HSI16, depending on software configuration.
Refer to Section 10.7.6, Section 10.7.7, Section 10.7.8 and Section 10.7.9.
•

Standby mode:
The Standby mode is used to achieve the lowest power consumption with BOR. The
internal regulator is switched off so that the core domain is powered off. The PLL, the
MSI (MSIS and MSIK) RC, the HSI16 RC and the HSE crystal oscillators are also
switched off.
The RTC can remain active (Standby mode with RTC, Standby mode without RTC).
The brownout reset (BOR) always remains active in Standby mode.
The state of each I/O during Standby mode can be selected by software: I/O with
internal pull-up, internal pull-down or floating.
After entering Standby mode, SRAMs and register contents are lost except for registers
and backup SRAM in the backup domain and Standby circuitry. Optionally, the full
SRAM2 or 8 Kbytes or 56 Kbytes can be retained in Standby mode, supplied by the
low-power regulator (standby with SRAM2 retention mode).
The BOR can be configured in ultra-low-power mode to further reduce power
consumption during standby mode and when the lowest threshold is selected (VBOR0).
The device exits Standby mode when an external reset (NRST pin), an IWDG reset,
WKUP pin event (configurable rising or falling edge), a RTC event occurs (alarm,
periodic wake-up, timestamp), or a tamper detection. The tamper detection can be
raised either due to external pins or due to an internal failure detection.
The system clock after wake-up is MSIS up to 4 MHz.
Refer to Section 10.7.10.

•

Shutdown mode:
The Shutdown mode allows the lowest power consumption. The internal regulator is
switched off so that the core domain is powered off. The PLL, the HSI16, the MSI
(MSIS and MSIK), the LSI and the HSE oscillators are also switched off.
The RTC can remain active (Shutdown mode with RTC, Shutdown mode without RTC).
The BOR is not available in Shutdown mode. No power voltage monitoring is possible
in this mode, therefore the switch to backup domain is not supported.
SRAMs and register contents are lost except for registers in the backup domain.
The device exits Shutdown mode when an external reset (NRST pin), a WKUP pin
event (configurable rising or falling edge), or a RTC event occurs (alarm, periodic
wake-up, timestamp), or a tamper detection.
The system clock after wake-up is MSIS at 4 MHz. Refer to Section 10.7.11.

<!-- pagebreak -->

