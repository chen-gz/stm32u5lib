609

Reset and clock control (RCC)

RM0456

The system clock maximum frequency is 160 MHz. After a system reset, the MSIS
oscillator, at 4 MHz, is selected as system clock. When a clock source is used directly or
through the PLL as a system clock, it is not possible to stop it.
A switch from one clock source to another occurs only if the target clock source is ready
(clock stable after startup delay or PLL locked). If a clock source that is not yet ready is
selected, the switch occurs when the clock source becomes ready. Status bits in RCC_CR
indicate which clocks are ready and which clock is currently used as a system clock.
The table below gives the different bus frequencies depending on the product voltage range.
Table 114. Bus maximum frequency

11.4.10

Product voltage range

AHB1/AHB2/AHB3/APB1/APB2/APB3

Range 1

160 MHz

Range 2

110 MHz

Range 3

55 MHz

Range 4

25 MHz

Clock source frequency versus voltage scaling
The table below gives the different clock source frequencies depending on the product
voltage range.
Table 115. Clock source maximum frequency

Voltage
range

Clock frequency
MSIS, MSIK

HSI16

HSI48

SHSI

HSE

PLL outputs (VCO min to max)

Range 1

All ranges

Allowed

Allowed

Allowed

50 MHz

208 MHz(1) (128 to 544 MHz)

Range 2

All ranges

Allowed

Allowed

Allowed

50 MHz

110 MHz (128 to 544 MHz)

Range 3

All ranges

Allowed

Allowed

Allowed

50 MHz

55 MHz (128 to 330 MHz)

Range 4

Up to 24 MHz
range

Allowed

Allowed
Allowed
25 MHz
(divided by 2) (divided by 2)

Not allowed

1. The maximum frequency depends on peripherals connected to PLL outputs.

11.4.11

Clock security system (CSS)
The CSS can be activated by software. In this case, the clock detector is enabled after
the HSE oscillator wake-up time, and disabled when this oscillator is stopped.
If a failure is detected on the HSE clock, the HSE oscillator is automatically disabled. A clock
failure event is sent to some timers break input and an interrupt is generated to inform the
software about the failure (clock security system interrupt CSSI). This allows the MCU to
perform rescue operations. The CSSI is linked to the Cortex-M33 NMI (nonmaskable
interrupt) exception vector.

Note:

<!-- pagebreak -->

Once the CSS is enabled and if the HSE clock fails, the CSSI occurs and an NMI is
automatically generated. The NMI is executed indefinitely unless CSSI bit is cleared. As a
consequence, in the NMI ISR, the user must clear the CSSI by setting CSSC in RCC_CICR.

RM0456 Rev 6

RM0456

Reset and clock control (RCC)
If the HSE oscillator is used directly or indirectly as the system clock (indirectly means: it is
used as PLL input clock and the PLL clock is used as system clock), a detected failure
causes a switch of the system clock to the MSIS or the HSI16 oscillator depending on
STOPWUCK configuration in RCC_CR, and the disabling of the HSE oscillator. If the HSE
clock (divided or not) is the clock entry of the PLL used as system clock when the failure
occurs, the PLL is disabled too.

11.4.12

Clock security system on LSE
A clock security system on LSE can be activated by software writing LSECSSON
in RCC_BDCR. This bit can be disabled only by a hardware reset or RTC software reset, or
after a failure detection on LSE. LSECSSON must be written after LSE is enabled (LSEON
enabled) and ready (LSERDY set by hardware), and after the RTC clock has been selected
by RTCSEL.
The CSS on LSE works in all modes, including VBAT mode. It works also under system reset
(excluding power-on reset).
The CSS on LSE detects when the LSE disappears or in case of over frequency. In addition,
the glitches on LSE can be filtered by setting LSEGFON. LSEGFON must be written when
the LSE is disabled (LSEON = 0 and LSERDY = 0).
If a failure is detected on the external 32 kHz oscillator, the LSE clock is no longer supplied
to the RTC, but no hardware action is made to the registers. If the MSI was in PLL-mode,
this mode is disabled.
The CSS on LSE detection event is connected to the internal tamper 3 of the TAMP:
•

On STM32U575/585 rev. X devices, the internal tamper 3 must be enabled
(ITAMP3E = 1 in TAMP_CR1) and the associated interrupt enabled (ITAMP3IE in
TAMP_IER) in order to wake up from the low-power modes. This erases also the TAMP
backup registers and backup SRAM unless ITAMP3NOER = 1 in TAMP_CR3
(see Section 64: Tamper and backup registers (TAMP) for more details).

•

On all other STM32U575/585 revisions, and the other STM32U5 devices, the CSS on
LSE detection event is also connected to an EXTI line, allowing to generate an event or
interrupt supporting wake-up from Stop 0, Stop 1, or Stop 2 mode, without requiring to
enable tamper detection (see Table 118: Interrupt sources and control and Table 190:
EXTI line connections).

In case of CSS on LSE detection event (LSECSSD = 1 in RCC_BDCR), the software must
then disable the LSECSSON bit, stop the defective 32 kHz oscillator (disabling LSEON),
and change the RTC clock source (no clock or LSI or HSE, with RTCSEL), or take any
required action to secure the application.
Refer to datasheet for CSS on LSE electrical characteristics.

11.4.13

ADC and DAC clocks
The ADC and DAC kernel clock source is selected thanks to ADCDACSEL[2:0]
in RCC_CCIPR3. The ADC clock ratio must be around 50 %. For this reason, the AHB
clock, when selected as ADC clock, must not be divided with HPRE prescaler. If pll2_r_ck is
selected as ADC clock, the PLL2R division factor must be even (division by 2 or 4
for example).
If the application requires that the ADC or DAC is precisely triggered by a TIMx timer without
any uncertainty, the HCLK must be selected as ADC and DAC kernel clock source.

RM0456 Rev 6

<!-- pagebreak -->

609

Reset and clock control (RCC)

RM0456

The other clock sources are asynchronous to TIMx timers therefore an uncertainty of
the trigger instant is added by the resynchronization between the two clock domains.
LPTIMx timers are also asynchronous.
The DAC requires an additional low-power clock (LSI or LSE) to operate in sample and hold
mode, available in Stop mode. This clock is selected with DAC1SEL in the RCC_CCIPR3.

11.4.14

RTC and TAMP clock
The RTCCLK clock source is used by RTC and TAMP, and can be either the HSE / 32, LSE,
or LSI clock. It is selected by programming RTCSEL[1:0] in RCC_BDCR. This selection
cannot be modified without resetting the backup domain. The system must always be
configured so as to get a PCLK frequency greater than or equal to the RTCCLK frequency
for a proper operation of the RTC. The TAMP does not require any kernel clock if only
backup registers are used, with tampers in edge detection mode. All other tamper detection
modes require a kernel clock (refer to Section 64: Tamper and backup registers (TAMP) for
more details).
LSE and LSI clocks are in the backup domain, whereas the HSE clock is not. Consequently:
•

If LSE or LSI is selected as RTC and TAMP clock, these peripherals continue to work
even if the VDD supply is switched off, provided the VBAT supply is maintained.

•

If the HSE clock divided by a prescaler is used as the RTC or TAMP clock, the RTC
state is not guaranteed if the VDD supply is powered off, or if the internal voltage
regulator is powered off (removing power from the core domain). Depending on the
TAMP configuration, this one can remain functional if used in a mode that does not
need any kernel clock.

When the RTC and TAMP clock is LSE or LSI, the RTC remains clocked and functional
under system reset.
If the LSE is needed only for the RTC or TAMP, LSESYSEN must be kept at reset value to
get the lowest consumption.

11.4.15

Timer clock
The timer clock frequencies are automatically defined by hardware.
There are two cases:

11.4.16

•

If the APB prescaler equals 1, the timer clock frequencies are set to the APB domain
frequency.

•

Otherwise, they are set to twice (×2) the APB domain frequency.

Watchdog clock
If the independent watchdog (IWDG) is started by either hardware option or software
access, the LSI oscillator is forced on and cannot be disabled. After the LSI oscillator
temporization, the LSI 32 kHz clock is provided to the IWDG.

11.4.17

OCTOSPI clock
The OCTOSPIx kernel clock, selected by OCTOSPIxSEL[1:0], can be up to 200 MHz when
pll1_q_ck or pll2_q_ck are used.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

11.4.18

Reset and clock control (RCC)

HSPI1 clock
The HSPI1 kernel clock, selected by HSPI1SEL[1:0], can be up to 200 MHz when
pll1_q_ck, pll2_q_ck or pll3_r_ck are used.

11.4.19

OTG_HS clock
The OTG_HS kernel clock is generated by the OTG_HS PHY. This PHY can accept only
frequencies of following list (16, 19.2, 20, 24, 26 or 32 MHz), with an accuracy of ± 400 ppm.
Those frequencies can be achieved using either HSE, HSE/2, PLL1_P or PLL1_P/2, and
selected by the OTGHSSEL[1:0] multiplexer. Refer to the OTGHSSEL description
concerning some limitations that apply when using the PLL as its input.

11.4.20

DSI clock
The DSI interface clock can be derived from the internal DSI PHY PLL or by the pll3_p_ck
clocks, selected by DSISEL multiplexer.

11.4.21

LTDC clock
The LTDC interface clock can be derived from the pll2_r_ck or pll3_r_ck clocks, selected by
LTDCSEL multiplexer.

11.4.22

Clock-out capability
•

MCO
The microcontroller clock output (MCO) capability allows the clock to be output onto the
external MCO pin. One of the following clock signals can be selected as MCO clock.
–

LSI

–

LSE

–

SYSCLK

–

HSI16

–

HSI48

–

HSE

–

PLLCLK

–

MSIS

–

MSIK

The selection is controlled by MCOSEL[3:0] in RCC_CR. The selected clock can be
divided with MCOPRE[2:0] in RCC_CR.
•

LSCO
Another output (LSCO) allows one of the low-speed clocks below to be output onto
the external LSCO pin:
–

LSI

–

LSE

This output remains available in all Stop modes, Standby, and Shutdown modes.
This output is not available in VBAT mode. The selection is controlled by LSCOSEL bit
and enabled with LSCOEN in RCC_BDCR.
The MCO clock output requires the corresponding alternate function selected on MCO pin.

RM0456 Rev 6

<!-- pagebreak -->

609

Reset and clock control (RCC)

11.4.23

RM0456

Internal/external clock measurement with TIM15/TIM16/TIM17
The frequency of all on-board clock sources can be indirectly measured by means of
the TIM15, TIM16, or TIM17 channel 1 input capture, and LPTIM1 or LPTIM2 channel 2
input capture.

HSI16 and MSI calibration using LSE
The primary purpose of connecting the LSE to the channel 1 input capture of TIM15, TIM16,
and TIM17, and to the channel 2 input capture of LPTIM2, is to be able to precisely measure
the HSI16 and MSI system clocks (for this, either HSI16 or MSIS must be used as system
clock source). The number of HSI16 (MSIS respectively) clock counts between consecutive
edges of the LSE signal provides a measure of the internal clock period. Taking advantage
of the high precision of LSE crystals (typically a few tens of ppms), the internal clock
frequency can be determined with the same resolution, and the source can be trimmed
to compensate the manufacturing, process, temperature and/or voltage related frequency
deviations.
The four oscillators of MSI and HSI16 oscillator have dedicated user-accessible calibration
bits for this purpose.
The basic concept consists in providing a relative measurement (such as HSI16/LSE ratio).
The precision is therefore closely related to the ratio between the two clock sources.
The higher the ratio is, the better the measurement is.
Note:

When the LSE is available, the MSI can be automatically trimmed by LSE using PLL-mode.

HSI16 and MSI calibration using HSE
If the HSE is available, it can be used as system clock, and the timer input capture must be
connected either to MSI (divided by 1024 or by 4) or to HSI/256. TIM16 and TIM17 channel
1 input capture, as well and the LPTIM2 input capture 2, are connected to the divided
oscillator only when TIMICSEL[2:0] ≠ 0 in RCC_CCIPR1.
Considering that the timer counter is 16-bit, and that the ratio between HSE and the input
capture signal must be the highest possible, a division by 1024 must be selected when
MSIRC0, MSIRC1, or MSIRC2 is measured, and a division by 4 when MSIRC4 is
measured.

LSI calibration
The calibration of the LSI follows the same principle, but changing the reference clock.
The LSI clock must be connected to the channel 1 input capture of the TIM16 or TIM17, or
to the channel 2 input capture of the LPTIM1. Then defining the HSE as system clock
source, the number of its clock counts between consecutive edges of the LSI signal,
provides a measure of the internal low-speed clock period.
The basic concept consists in providing a relative measurement (such as HSE/LSI ratio).
The precision is therefore closely related to the ratio between the two clock sources.
The higher the ratio is, the better the measurement is.

11.4.24

Peripherals clock gating and autonomous mode
Peripherals clock gating in Run mode
Each peripheral clock can be enabled by the corresponding EN bit in RCC_AHBxENR and
RCC_APBxENR registers.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Reset and clock control (RCC)
When the peripheral clock is not active, read or write accesses to the peripheral registers
are not supported.
The enable bit has a synchronization mechanism to create a glitch-free clock for the
peripheral. After the enable bit is set, there the clock is active after 2 cycles of the peripheral
bus clock.

Caution:

Just after enabling the clock for a peripheral, the software must wait for these two clock
cycles before accessing the peripheral registers.

Peripherals clock gating in Sleep and Stop modes
When a peripheral is enabled, its clock can be automatically gated off when the device is in
Sleep mode, by clearing the peripheral SMEN bit in RCC_AHBxSMENR and
RCC_APBxSMENR. Both EN and SMEN of the peripheral must be set to keep the clock on
in Sleep mode.
The SMEN bit of the peripheral is also used to allow peripheral clocking in Stop 0 and Stop 1
modes, upon peripheral request.
When the clock is requested by a peripheral, this clock is distributed to all enabled
peripherals. Therefore, the SMEN bit must be cleared before entering Stop mode, if the
peripheral is not used in Stop mode.
Caution:

The SMEN bit of the peripheral must be set to allow the generation of an interrupt capable to
wake up the device from Stop mode. This is not necessary when the peripheral wake-up
interrupt is generated though the EXTI.

Peripherals clock gating and autonomous mode in Stop 0/1/2 modes
Some peripherals support autonomous mode (refer to Table 116). These peripherals are
able to generate a kernel clock request and a AHB/APB bus clock request when they need,
in order to operate and update their status register even in Stop mode. Depending on the
peripheral configuration, either a DMA request or an interrupt can be associated to the
peripheral event.
Upon an AHB or APB bus clock request from an autonomous peripheral, either MSI or
HSI16 oscillator is woken up, depending on the oscillator selected by STOPWUCK
in RCC_CFGR1.
If the autonomous peripheral is configured with DMA requests enabled, a data transfer is
performed thanks to the AHB/APB clock. The bus clocks as well as the oscillator (HSI16 or
MSI) are automatically switched off as soon as the transfer is finished, if no other peripheral
requests it. The device automatically goes back in Stop mode.
If the autonomous peripheral is configured with interrupt enabled, the interrupt wakes up
the device into Run mode.
The autonomous peripherals mapped on AHB3 or APB3 belong to the SmartRun domain
and are autonomous in Stop 0, Stop 1, and Stop 2 with the LPDMA1 and SRAM4.
The autonomous peripherals mapped on AHB1, AHB2, APB1, and APB2, belong to
the CPU domain, and are autonomous in Stop 0 and Stop 1 mode, only with GPDMA1
and SRAM1/2/3/4/5/6.

RM0456 Rev 6

<!-- pagebreak -->

