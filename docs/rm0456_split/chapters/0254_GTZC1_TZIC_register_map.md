275

Global TrustZone controller (GTZC)

RM0456

Bit 14 CTZSC1F: clear the illegal access flag for GTZC1 TZSC registers
0: no action
1: status flag cleared
Bits 13:5 Reserved, must be kept at reset value.
Bit 4 COTFDEC2F: clear the illegal access flag for OTFDEC2
0: no action
1: status flag cleared
Note: This bit is only available on some devices in the STM32U5 series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and kept at reset value.
Bit 3 COTFDEC1F: clear the illegal access flag for OTFDEC1
0: no action
1: status flag cleared
Bit 2 CFLASHF: clear the illegal access flag for flash memory
0: no action
1: status flag cleared
Bit 1 CFLASH_REGF: clear the illegal access flag for FLASH registers
0: no action
1: status flag cleared
Bit 0 CGPDMA1F: clear the illegal access flag for GPDMA1
0: no action
1: status flag cleared

5.7.13

GTZC1 TZIC register map

<!-- pagebreak -->

TIM3IE

TIM2IE

Res.

Res.

Res.

Res.

0

0

0

0

0

0

0

0

0

AESIE

0

OTGIE

0

HASHIE

0

RNGIE

0

PKAIE

0

SAESIE

TIM1IE

Res.

0

MDF1IE

TIM4IE

Res.

0

OCTOSPIMIE

0

0

SDMMC1IE

0

0

SDMMC2IE

0

0

FSMC_REGIE

GPU2DIE

0

0

OCTOSPI1_REGIE

GFXMMUIE

0

0

RAMCFGIE

GFXMMU_REGIE

0

0
OCTOSPI2_REGIE

HSPI1_REGIE

Res.

DCACHE2_REGIE

Reset value

JPEGIE

GTZC1_TZIC_IER3

Res.

0x008

Res.

Reset value

SPI1IE

0

CORDICIE

TIM5IE

0

TIM8IE

0

FMACIE

TIM6IE

0
USART1IE

0

CRCIE

TIM7IE
0

TIM15IE

0

TSCIE

WWDGIE

0

TIM16IE

0

TIM17IE

0

DMA2DIE

0

ICACHE_REGIE

SPI2IE

IWDGIE

0

SAI1IE

USART2IE

0

DCACHE1_REGIE

UART4IE

USART3IE

0

SAI2IE

I2C1IE

UART5IE

0

LTDCUSBIE

I2C2IE

0

DCMIIE

CRSIE

0

ADC12IE

I2C4IE

0

DSIIE

LPTIM2IE

0

GFXTIMIE

FDCAN1IE

0

Res.

Res.

Res.

UCPD1IE

0

Res.

0

Res.

I2C5IE

USART6IE

0

Res.

Res.

I2C6IE

Res.

Res.

Res.

Res.

Res.

Res.

GTZC1_TZIC_IER2

Res.

0x004

Res.

Reset value

Res.

Res.

Res.

Res.

Res.

Res.

31
30
29
28
27
26
25
24
23
22
21
20
19
18
17
16
15
14
13
12
11
10
9
8
7
6
5
4
3
2
1
0

GTZC1_TZIC_IER1

Res.

Offset

0x000

Register name

Res.

Table 40. GTZC1 TZIC register map and reset values

0

0

0

0

0

0

0

0

0

0

0

0

0

RM0456 Rev 6

0x028

GTZC1_TZIC_
FCR3

0

0

0

0

0

0

0
CTIM2F

0
0
0

Reset value

0

0

0

0

0

0

0

0

RM0456 Rev 6
Res.
Res.
Res.
Res.
Res.

CTIM1F

0

0
0
0
0
0
0
0
0
0
0
0
0

CMDF1F

CTIM3F

0

CSPI1F

CTIM4F

0

CCORDICF

CTIM5F

0

CTIM8F

0

CUSART1F

CTIM6F

0

CCRCF

CTIM7F

0

CFMACF

CWWDGF

0

CTIM15F

CIWDGF

0

CTSCF

CSPI2F

0

CTIM16F

CUSART2F

0

CDMA2DF

CUSART3F

0

CTIM17F

CUART4F

0

CICACHEF

CI2C1F
CUART5F

0

CSAI1F

DCACHE1F
ICACHEF
DMA2DF
TSCF
CRCF
FMACF
CORDICF
MDF1F

0
0
0
0
0

Res.
Res.
Res.
Res.
Res.
Res.
Res.
Res.
Res.
OTFDEC2F
OTFDEC1F
FLASHF
FLASH_REGF
GPDMA1F

TZSC1F

CI2C2F

0

CDCACHE1F

ADC12F

0

TZIC1F

CCRSF

0

CSAI2F

0

OCTOSPI1_MEMFE

CI2C4F

0

ADC12F

DCMIF
0

0

CLTDCUSBF

0

0

CDCMIF

OTGF
0

0

CLPTIM2F

TIM3F
TIM2F

0
0
0
0

Res.

TIM1F

TIM4F

0
SPI1F

TIM5F

0
TIM8F

TIM6F

0
USART1F

TIM7F

0
TIM15F

WWDGF

0
TIM16F

IWDGF

0
TIM17F

SPI2F

0
SAI1F

USART2F

0
SAI2F

USART3F

0
LTDCUSBF

UART4F

0
DSIF

I2C1F
UART5F

0
Res.

OTFDEC2IE
OTFDEC1IE
FLASHIE
FLASH_REGIE
GPDMA1IE

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

TZIC1IE
I2C2F

0
Res.

TZSC1IE

CRSF

0
Res.

OCTOSPI1_MEMIE

I2C4F

0
Res.

BKPSRAMIE
0
Res.

FSMC_MEMIE

LPTIM2F

0
GFXTIMF

Reset value

CDSIF

0

0

Res.

OCTOSPI2_MEMIE

UCPD1F

0

FDCAN1F

0

Res.

0

Res.

0

COTGF

AESF
0

0

CUCPD1F

Res.
HSPI1_MEMIE

Res.

0

Res.

I2C5F
USART6F

Res.
Res.

I2C6F

0

CGFXTIMF

RNGF
HASHF
0

0

CFDCAN1F

Res.

0

CAESF

CRNGF

PKAF

0

FSMC_MEMFE

0

CHASHF

SAESF

0

BKPSRAMIF

0

CPKAF

OCTOSPIMF

0

OCTOSPI2_MEMIF

0

CSAESF

0

0

Res.

Res.

0

Res.

Res.

0

HSPI1_MEMIF

Res.

SRAM6IE

Res.

0

Res.

CI2C5F

Res.

0

COCTOSPIMF

SDMMC1F

0

0

CUSART6F

Res.

0

CSDMMC1F

FSMCF
SDMMC2F

0

0

Res.

GTZC1_TZIC_SR1
0

CFSMCF
0

0

Res.

0

SRAM1IE

0

MPCBB6_REGIE

SRAM3IE
MPCBB2_REGIE

0

Res.

MPCBB3_REGIE

0

Res.

Res.

SRAM5IE

0

SRAM2IE

MPCBB5_REGIE

Reset value
MPCBB1_REGIE

31
30
29
28
27
26
25
24
23
22
21
20
19
18
17
16
15
14
13
12
11
10
9
8
7
6
5
4
3
2
1
0

GTZC1_TZIC_IER4

0

CSDMMC2F

OCTOSPI1F

0

0

CI2C6F

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Register name

0

Res.

0

0

Res.

Reset value

COCTOSPI1F

0

SRAM6F

0

SRAM1F
0

MPCBB6_REGF

0

Res.

0

SRAM2F

0

MPCBB1_REGF

RAMCFGF

Res.

OCTOSPI2F

Res.

0

Res.

Res.

Res.

GPU2DF

Res.

GFXMMUF

GTZC1_TZIC_
FCR1

Res.

0
HSPI1_REGF

0
GFXMMU_REGF

0

CRAMCFGF

CGPU2DF

Res.

Res.

MPCBB2_REGF

0
DCACHE2_REGF

SRAM3F

0
JPEGF

MPCBB3_REGF

Reset value

Res.

Res.

Reset value

COCTOSPI2F

CGFXMMUF

0

CHSPI1_REGF

0

CGFXMMU_REGF

Res.

Res.

Res.

Reset value

CDCACHE2_REGF

GTZC1_TZIC_
FCR2

Res.

GTZC1_TZIC_SR3
Res.

GTZC1_TZIC_SR2

CJPEGF

0x024
SRAM5F

0x020
GTZC1_TZIC_SR4
MPCBB5_REGF

0x01C

Res.

0x018

Res.

0x014

Res.

0x010

Res.

0x00C

Res.

Offset

Res.

RM0456
Global TrustZone controller (GTZC)

Table 40. GTZC1 TZIC register map and reset values (continued)

0
0
0
0
0

0
0
0
0
0
0
0
0
0
0
0
0

0
0
0
0
0

Reset value

0

0

0

0

0

0

0

0

0

0

0

0

<!-- pagebreak -->

