RM0456 Rev 6

RM0456

Reset and clock control (RCC)

Low-power mode security reset
To avoid that critical applications mistakenly enter a low-power mode, the following
low-power mode security resets are available. If enabled in option bytes, the resets are
generated in any of the following conditions:
•

Entering Standby mode: this type of reset is enabled by resetting NRST_STDBY bit in
user option bytes. In this case, whenever a Standby mode entry sequence is
successfully executed, the device is reset instead of entering Standby mode.

•

Entering Stop mode: this type of reset is enabled by resetting NRST_STOP bit in user
option bytes. In this case, whenever a Stop mode entry sequence is successfully
executed, the device is reset instead of entering Stop mode.

•

Entering Shutdown mode: this type of reset is enabled by resetting NRST_SHDW bit in
user option bytes. In this case, whenever a Shutdown mode entry sequence is
successfully executed, the device is reset instead of entering Shutdown mode.

For further information on the user option bytes, refer to Section 7.4.1: Option bytes
description.

Option byte loader reset
The option byte loader reset is generated when the OBL_LAUNCH bit is set in the
FLASH_NSCR register. This bit is used to launch the option byte loading by software.

11.3.3

Backup domain reset
The backup domain has two specific resets.
A backup domain reset is generated when one of the following events occurs:
•

a software reset, triggered by setting BDRST bit RCC_BDCR

•

a VDD or VBAT power on, if both supplies have previously been powered off

A backup domain reset affects the LSE oscillator, the RTC, the TAMP, the backup registers,
RCC_BDCR, and PWR_BDCR1. The reset of PWR_BDCR1 affects the backup SRAM.

11.4

RCC clock functional description
Four different clock sources can be used to drive the system clock (SYSCLK):
•

HSI16: high-speed internal 16 MHz RC oscillator clock

•

MSIS: multi-speed internal RC oscillator clock

•

HSE: high-speed external crystal or clock, from 4 to 50 MHz

•

PLL1 clock

The MSIS is used as system clock source after startup from reset, configured at 4 MHz.
The devices have the following additional clock sources:
•

MSIK: multi-speed internal RC oscillator clock used for peripherals kernel clocks

•

LSI: 32 kHz/250 Hz low-speed internal RC that drives the independent watchdog and
optionally the RTC used for auto-wake-up from Stop and Standby modes

•

LSE: 32.768 kHz low-speed external crystal or clock that optionally drives the real-time
clock (rtc_ck)

RM0456 Rev 6

<!-- pagebreak -->

609

Reset and clock control (RCC)

RM0456

•

HSI48: internal 48 MHz RC that potentially drives the OTG_FS, the USB, the SDMMC,
and the RNG

•

SHSI: secure high-speed internal 48 MHz RC that drives the SAES

•

PLL2 and PLL3 clocks

Each clock source can be switched on or off independently when it is not used, to optimize
power consumption.
Several prescalers can be used to configure the AHB frequency, the APB1, APB2, and
APB3 domains. The maximum frequency of the AHB and APB domains is 160 MHz.
All the peripheral clocks are derived from their bus clock (HCLK, PCLK1, PCLK2, or PCLK3)
except the following ones that receive an independent kernel clock. This kernel clock can be
selected by software between several sources thanks to RCC_CCIPRx registers (x = 1,2,3):
OTG_FS, USB, or OTG_HS, SDMMCx (x = 1,2), RNG, ADCx (x = 1, 2, 4), DAC1,
U(S)ARTx (x = 1 to 6), LPUART1, I2Cx (x = 1 to 6), SPIx (x = 1 to 3), OCTOSPIx (x = 1,2),
SAIx (x = 1,2), MDF1, ADF1, FDCAN1, LPTIMx (x = 1 to 4), SAES, DSI, LTDC, HSPI1.
In addition, the RTC kernel clock is selected by software in RCC_BDCR. The IWDG clock is
always the LSI 32 kHz clock.
The RCC feeds the Cortex system timer (SysTick) external clock with the AHB clock (HCLK)
divided by eight, or LSE or LSI. The SysTick can work either with this clock or directly with
the Cortex clock (HCLK), configurable in the SysTick control and status register.
FCLK acts as Cortex-M33 free-running clock.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Reset and clock control (RCC)
Figure 38. Clock tree for STM32U5 series
LSI RC
32 kHz or 250 Hz

LSCO

To IWDG

LSI
HSI16

LSE OSC
32.768 kHz

OSC32_OUT

Clock
detector

OSC32_IN

MCO

LSE
LSI
MSIS
HSI16
HSE
SYSCLK
pll1_r_ck
HSI48
MSIK

ĺ

OSC_OUT

HSE OSC
4-50 MHz

OSC_IN

To RTC

LSI
LSE
MSIK
HSI16

/32

To UCPD1

x2

To LPTIM1, LPTIM3, LPTIM4

To AHB bus, core, memory and DMA
AHB
PRESC
/ 1,2,..512

HCLK

FCLK Cortex free running clock
LSE
LSI

To Cortex system timer

/8

Clock
source
control

APB1
PRESC
/ 1,2,4,8,16

HSE

Clock
detector

PCLK1
To APB1 peripherals
x1 or x2
To TIMx
(x = 2 to 7)

SYSCLK
HSI RC 16 MHz

HSI16

MSI RC
MSIS 100 kHz – 48 MHz

MSIS

MSIK 100 kHz – 48 MHz

MSIK

LSE
HSI16
SYSCLK

PLL1

/M

pll1_r_ck

/R

PLL2

MSIS
HSI16
HSE

/M
pll2_p_ck

/P

VCO

pll2_q_ck

/Q

To OCTOSPIx
(x = 1, 2)

CRS clock

To SAES

To APB2 peripherals
x1 or x2

To TIMx
(x = 1,8,15,16,17)

pll3_q_ck

/Q

pll3_r_ck

/R

/N

SYSCLK
MSIK
pll1_q_ck
pll2_q_ck

MSIS
HSI16
HSE

/M

VCO

To LPTIM2

PCLK2

pll3_p_ck

/P

To I2Cx
(x = 1,2,4,5,6)

To FDCAN1

APB2
PRESC
/ 1,2,4,8,16

PLL3

x5

HSE
pll1_q_ck
pll2_p_ck

pll2_r_ck

/R

/N

To SPI2

LSI
LSE
HSI16

pll1_q_ck

/Q
/N

HSI16
SYSCLK
MSIK

MSIS
HSI16
HSE

pll1_p_ck

/P

VCO

To USARTx
(x = 2 to 6)

MSIK
HSI16
SYSCLK

HSI48

HSI48 RC 48 MHz

x5

SHSI RC

/2
pll1_p_ck
pll3_q_ck
MSIK

LSE
HSI16
SYSCLK

To USART1

MSIK
HSI16
SYSCLK

To SPI1

x2
To ADF1 and MDF1

AUDIOCLK

pll1_p_ck
MSIK
HSI48
pll1_q_ck
pll2_q_ck

pll1_p_ck
pll2_p_ck
pll3_p_ck
HSI16

x2

To SAIx
(x = 1, 2)
To SDMMCx
(x = 1,2)

ICLK

To 48 MHz OTG_FS or USB
HSI16
/2

SYSCLK
pll1_q_ck
pll2_q_ck
pll3_r_ck

To HSPI

pll2_r_ck
pll3_r_ck

To RNG
APB3
PRESC
/ 1,2,4,8,16

HSE
pll1_p_ck
HSE / 2
pll1_p_ck / 2

To APB3 peripherals

MSIK
HSI16

To I2C3

MSIK
HSI16

To SPI3

MSIK
HSI16
LSE

To LPUART1

To LTDC

pll3_p_ck
HSE

PCLK3

To DSI

DSI PHY
PLL

OTG_HS
PHY PLL

To OTG_HS

pll2_r_ck
HSE
HSI16
MSIK

LSI
LSE

Highlighted connections or peripheral may not be present in all devices of the STM32U5 Series.
Refer to the device datasheet for more information.

RM0456 Rev 6

To ADC1, ADC2,
ADC4 and DAC1
DAC1 sample and hold clock
MSv71110V2

<!-- pagebreak -->

