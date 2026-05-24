RM0456 Rev 6

Reserved
AHB3
Reserved
APB3
Reserved
AHB2
Reserved
AHB1
Reserved
APB2
Reserved
APB1
Reserved
AHB3
Reserved
APB3
Reserved
AHB2
Reserved
AHB1
Reserved
APB2
Reserved
APB1

External memories
Reserved
RSS
Reserved
SRAM6
SRAM5
SRAM3
SRAM2
SRAM1
Reserved
FLASH
Reserved
OTP
Reserved
System memory
Reserved
SRAM6
SRAM5
SRAM3
SRAM2
SRAM1
Reserved
FLASH
External memories remap
MSv70739V2

RM0456
All memory-map areas that are not allocated to on-chip memories and peripherals are
considered “Reserved”. The table below gives the boundary addresses of the peripherals
available in the devices.

APB3

AHB3

STM32U59x/5Ax

STM32U5Fx/5Gx

STM32U575/585

STM32U535/545

Secure boundary
address

Nonsecure boundary
address

Size (bytes)

Bus

Table 6. Memory map and peripheral register boundary addresses

Peripheral

0x5602 6000 - 0x5FFF FFFF

0x4602 6000 - 0x4FFF FFFF

164 M

Reserved

-

-

-

-

-

0x5602 5000 - 0x5602 5FFF

0x4602 5000 - 0x4602 5FFF

4K

LPDMA1

LPDMA register map

X

X

X

X

ADF register map

X

X

X

X

-

-

-

-

-

X

X

X

X

Peripheral register
map

0x5602 4000 - 0x5602 4FFF

0x4602 4000 - 0x4602 4FFF

4K

ADF1

0x5602 3C00 - 0x5602 3FFF

0x4602 3C00 - 0x4602 3FFF

1K

Reserved

0x5602 3800 - 0x5602 3BFF

0x4602 3800 - 0x4602 3BFF

1K

0x5602 3400 - 0x5602 37FF

0x4602 3400 - 0x4602 37FF

1K

GTZC2_TZIC

GTZC2 TZIC register map

X

X

X

X

0x5602 3000 - 0x5602 33FF

0x4602 3000 - 0x4602 33FF

1K

GTZC2_TZSC

GTZC2 TZSC register map

X

X

X

X

0x5602 2400 - 0x5602 2FFF

0x4602 2400 - 0x4602 2FFF

3K

Reserved

-

-

-

-

-

0x5602 2000 - 0x5602 23FF

0x4602 2000 - 0x4602 23FF

1K

EXTI

EXTI register map

X

X

X

X

0x5602 1C00 - 0x5602 1FFF

0x4602 1C00 - 0x4602 1FFF

1K

Reserved

0x5602 1800 - 0x5602 1BFF

0x4602 1800 - 0x4602 1BFF

1K

DAC1

0x5602 1400 - 0x5602 17FF

0x4602 1400 - 0x4602 17FF

1K

Reserved

0x5602 1000 - 0x5602 13FF

0x4602 1000 - 0x4602 13FF

1K

ADC4

0x5602 0C00 - 0x5602 0FFF

0x4602 0C00 - 0x4602 0FFF

1K

0x5602 0800 - 0x5602 0BFF

0x4602 0800 - 0x4602 0BFF

1K

0x5602 0400 - 0x5602 07FF

0x4602 0400 - 0x4602 07FF

1K

0x5602 0000 - 0x5602 03FF

0x4602 0000 - 0x4602 03FF

GTZC2_MPCBB4 GTZC2 MPCBB4 register map

-

-

-

-

-

DAC register map

X

X

X

X

-

-

-

-

-

ADC register map

X

X

X

X

RCC

RCC register map

X

X

X

X

PWR

PWR register map

X

X

X

X

Reserved

-

-

-

-

-

1K

LPGPIO1

LPGPIO register map

X

X

X

X

0x5600 8000 - 0x5601 FFFF

0x4600 8000 - 0x4601 FFFF

96 K

Reserved

-

-

-

-

-

0x5600 7C00 - 0x5600 7FFF

0x4600 7C00 - 0x4600 7FFF

1K

TAMP

TAMP register map

X

X

X

X

0x5600 7800 - 0x5600 7BFF

0x4600 7800 - 0x4600 7BFF

1K

RTC

RTC register map

X

X

X

X

0x5600 7400 - 0x5600 77FF

0x4600 7400 - 0x4600 77FF

1K

VREFBUF

VREFBUF register map

X

X

X

X

0x5600 5800 - 0x5600 73FF

0x4600 5800 - 0x4600 73FF

7K

Reserved

-

-

-

-

-

0x5600 5400 - 0x5600 57FF

0x4600 5400 - 0x4600 57FF

1K

COMP(1)

COMP register map

X

X

X

X

OPAMP register map

X

X

X

X

X

X

X

X

X

X

X

X

X

X

X

X

-

-

-

-

-

I2C register map

X

X

X

X

LPUART register map

X

X

X

X

SPI register map

X

X

X

X

0x5600 5000 - 0x5600 53FF

0x4600 5000 - 0x4600 53FF

1K

OPAMP

0x5600 4C00 - 0x5600 4FFF

0x4600 4C00 - 0x4600 4FFF

1K

LPTIM4

0x5600 4800 - 0x5600 4BFF

0x4600 4800 - 0x4600 4BFF

1K

LPTIM3

0x5600 4400 - 0x5600 47FF

0x4600 4400 - 0x4600 47FF

1K

LPTIM1

0x5600 2C00 - 0x5600 43FF

0x4600 2C00 - 0x4600 43FF

6K

Reserved

0x5600 2800 - 0x5600 2BFF

0x4600 2800 - 0x4600 2BFF

1K

I2C3

0x5600 2400 - 0x5600 27FF

0x4600 2400 - 0x4600 27FF

1K

LPUART1

0x5600 2000 - 0x5600 23FF

0x4600 2000 - 0x4600 23FF

1K

SPI3

0x5600 0800 - 0x5600 1FFF

0x4600 0800 - 0x4600 1FFF

6K

Reserved

-

-

-

-

-

0x5600 0400 - 0x5600 07FF

0x4600 0400 - 0x4600 07FF

1K

SYSCFG

SYSCFG register map

X

X

X

X

RM0456 Rev 6

LPTIM register map

<!-- pagebreak -->

150

RM0456

STM32U535/545

STM32U575/585

STM32U59x/5Ax

Peripheral

0x520D 3800 - 0x5600 03FF

0x420D 3800 - 0x4600 03FF

64.3 M

Reserved

-

-

-

-

-

0x520D 3400 - 0x520D 37FF

0x420D 3400 - 0x420D 37FF

1K

HSPI1

HSPI register map

-

-

X

X

0x520D 2800 - 0x520D 33FF

0x420D 2800 - 0x420D 33FF

3K

Reserved

-

-

-

-

-

OCTOSPI2
registers

OCTOSPI register map

-

X

X

X

Secure boundary
address

Nonsecure boundary
address

Peripheral register
map

0x520D 2400 - 0x520D 27FF

0x420D 2400 - 0x420D 27FF

1K

0x520D 1800 - 0x520D 23FF

0x420D 1800 - 0x420D 23FF

3K

Reserved

-

-

-

-

-

1K

OCTOSPI1
registers

OCTOSPI register map

X

X

X

X

-

-

-

-

-

FMC register map

-

X

X

X

0x520D 1400 - 0x520D 17FF

AHB2

STM32U5Fx/5Gx

Size (bytes)

Bus

Table 6. Memory map and peripheral register boundary addresses (continued)

0x420D 1400 - 0x420D 17FF

0x520D 0800 - 0x520D 13FF

0x420D 0800 - 0x420D 13FF

3K

Reserved

0x520D 0400 - 0x520D 07FF

0x420D 0400 - 0x420D 07FF

1K

FSMC registers

0x520C F800 - 0x520D 03FF

0x420C F800 - 0x420D 03FF

3K

Reserved

0x520C F400 - 0x520C F7FF

0x420C F400 - 0x420C F7FF

1K

DLYBOS2

-

0x520C F000 - 0x520C F3FF

0x420C F000 - 0x420C F3FF

1K

DLYBOS1

0x520C 9000 - 0x520C EFFF

0x420C 9000 - 0x420C EFFF

24 K

Reserved

-

0x520C 8C00 - 0x520C 8FFF

0x420C 8C00 - 0x420C 8FFF

1K

SDMMC2

SDMMC register map

0x520C 8800 - 0x520C 8BFF

0x420C 8800 - 0x420C 8BFF

1K

DLYBSD2

0x520C 8400 - 0x520C 87FF

0x420C 8400 - 0x420C 87FF

1K

DLYBSD1

DLYB register map

DLYB register map

-

-

-

-

-

X

X

X

X

X

X

X

-

-

-

-

-

X

X

X

-

X

X

X

X

X

X

X
X

0x520C 8000 - 0x520C 83FF

0x420C 8000 - 0x420C 83FF

1K

SDMMC1

SDMMC register map

X

X

X

0x520C 5800 - 0x520C 7FFF

0x420C 5800 - 0x420C 7FFF

10 K

Reserved

-

-

-

-

-

0x520C 5400 - 0x520C 57FF

0x420C 5400 - 0x420C 57FF

1K

OTFDEC2

-

X

X

X

0x520C 5000 - 0x520C 53FF

0x420C 5000 - 0x420C 53FF

1K

OTFDEC1

X

X

X

X

0x520C 4400 - 0x520C 4FFF

0x420C 4400 - 0x420C 4FFF

3K

Reserved

-

-

-

-

-

0x520C 4000 - 0x520C 43FF

0x420C 4000 - 0x420C 43FF

1K

OCTOSPIM

OCTOSPIM register map

-

X

X

X
X

0x520C 2000 - 0x520C 3FFF

0x420C 2000 - 0x420C 3FFF

8K

PKA

0x520C 1000 - 0x520C 1FFF

0x420C 1000 - 0x420C 1FFF

4K

Reserved

0x520C 0C00 - 0x520C 0FFF

0x420C 0C00 - 0x420C 0FFF

1K

0x520C 0800 - 0x520C 0BFF

0x420C 0800 - 0x420C 0BFF

0x520C 0400 - 0x520C 07FF

0x420C 0400 - 0x420C 07FF

0x520C 0000 - 0x520C 03FF
0x5204 0000 - 0x5205 FFFF

OTFDEC register map

PKA register map

X

X

X

-

-

-

-

-

SAES

SAES register map

X

X

X

X

1K

RNG

RNG register map

X

X

X

X

1K

HASH

HASH register map

X

X

X

X

0x420C 0000 - 0x420C 03FF

1K

AES

AES register map

X

X

X

X

0x4204 0000 - 0x4205 FFFF

128 K

OTG_HS

OTG_HS register map

-

-

X

X
-

0x5204 0000 - 0x520B FFFF

0x4204 0000 - 0x420B FFFF

512 K

OTG_FS

OTG_FS register map

-

X

-

0x5202 C800 - 0x5203 FFFF

0x4202 C800 - 0x4203 FFFF

78 K

Reserved

-

-

-

-

-

0x5202 C400 - 0x5202 C7FF

0x4202 C400 - 0x4202 C7FF

1K

PSSI

PSSI register map

X

X

X

X

0x5202 C000 - 0x5202 C3FF

0x4202 C000 - 0x4202 C3FF

1K

DCMI

DCMI register map

X

X

X

X

0x5202 8400 - 0x5202 BFFF

0x4202 8400 - 0x4202 BFFF

15 K

Reserved

-

-

-

-

-

0x5202 8000 - 0x5202 83FF

0x4202 8000 - 0x4202 83FF

1K

ADC12(2)

ADC register map

X

X

X

X

0x5202 2800 - 0x5202 7FFF

0x4202 2800 - 0x4202 7FFF

22 K

Reserved

-

-

-

-

-

<!-- pagebreak -->

RM0456 Rev 6

RM0456

AHB1

Nonsecure boundary
address

Peripheral

STM32U575/585

STM32U59x/5Ax

STM32U5Fx/5Gx

0x5202 2400 - 0x5202 27FF

0x4202 2400 - 0x4202 27FF

1K

GPIOJ

-

-

X

X

0x5202 2000 - 0x5202 23FF

0x4202 2000 - 0x4202 23FF

1K

GPIOI

-

X

X

X

0x5202 1C00 - 0x5202 1FFF

0x4202 1C00 - 0x4202 1FFF

1K

GPIOH

X

X

X

X

0x5202 1800 - 0x5202 1BFF

0x4202 1800 - 0x4202 1BFF

1K

GPIOG

X

X

X

X

0x5202 1400 - 0x5202 17FF

0x4202 1400 - 0x4202 17FF

1K

GPIOF

-

X

X

X

0x5202 1000 - 0x5202 13FF

0x4202 1000 - 0x4202 13FF

1K

GPIOE

X

X

X

X

0x5202 0C00 - 0x5202 0FFF

0x4202 0C00 - 0x4202 0FFF

1K

GPIOD

X

X

X

X

0x5202 0800 - 0x5202 0BFF

0x4202 0800 - 0x4202 0BFF

1K

GPIOC

X

X

X

X

0x5202 0400 - 0x5202 07FF

0x4202 0400 - 0x4202 07FF

1K

GPIOB

X

X

X

X

0x5202 0000 - 0x5202 03FF

0x4202 0000 - 0x4202 03FF

1K

GPIOA

X

X

X

X

0x5003 6C00 - 0x5201 FFFF

0x4003 6C00 - 0x4201 FFFF

32.7 M

Reserved

-

-

-

-

-

0x5003 6400 - 0x5003 6BFF

0x4003 6400 - 0x4003 6BFF

2K

BKPSRAM

-

X

X

X

X

-

-

-

-

-

-

-

-

X

0x5003 4000 - 0x5003 63FF

0x4003 4000 - 0x4003 63FF

9K

Reserved

0x5003 3C00 - 0x5003 3FFF

0x4003 3800 - 0x4003 3BFF

1K

GTZC1_MPCBB6

0x5003 3800 - 0x5003 3BFF

0x4003 3C00 - 0x4003 3FFF

1K

GTZC1_MPCBB5

0x5003 3400 - 0x5003 37FF

0x4003 3400 - 0x4003 37FF

1K

GTZC1_MPCBB3

Peripheral register
map

GPIO register map

GTZC1 MPCBBz register map
(z = 1, 2, 3, 5, 6)

STM32U535/545

Secure boundary
address

Size (bytes)

AHB2 (cont’d)

Bus

Table 6. Memory map and peripheral register boundary addresses (continued)

-

-

X

X

-

X

X

X
X

0x5003 3000 - 0x5003 33FF

0x4003 3000 - 0x4003 33FF

1K

GTZC1_MPCBB2

X

X

X

0x5003 2C00 - 0x5003 2FFF

0x4003 2C00 - 0x4003 2FFF

1K

GTZC1_MPCBB1

X

X

X

X

0x5003 2800 - 0x5003 2BFF

0x4003 2800 - 0x4003 2BFF

1K

GTZC1_TZIC

GTZC1 TZIC register map

X

X

X

X

GTZC1 TZSC register map

X

X

X

X

-

-

-

-

-

-

-

X

X
X

0x5003 2400 - 0x5003 27FF

0x4003 2400 - 0x4003 27FF

1K

GTZC1_TZSC

0x5003 1C00 - 0x5003 23FF

0x4003 1C00 - 0x4003 23FF

2K

Reserved

0x5003 1800 - 0x5003 1BFF

0x4003 1800 - 0x4003 1BFF

1K

DCACHE2

0x5003 1400 - 0x5003 17FF

0x4003 1400 - 0x4003 17FF

1K

DCACHE1

X

X

X

0x5003 0800 - 0x5003 13FF

0x4003 0800 - 0x4003 13FF

3K

Reserved

-

-

-

-

-

0x5003 0400 - 0x5003 07FF

0x4003 0400 - 0x4003 07FF

1K

ICACHE

ICACHE register map

X

X

X

X

0x5003 0000 - 0x5003 03FF

0x4003 0000 - 0x4003 03FF

18 K

Reserved

-

-

-

-

-

0x5002 F000 - 0x5002 FFFF

0x4002 F000 - 0x4002 FFFF

1K

GPU2D

-

-

-

X

X
X

DCACHE register map

0x5002 C000 - 0x5002 EFFF

0x4002 C000 - 0x4002 EFFF

1K

GFXMMU

GFXMMU register map

-

-

X

0x5002 BC00 - 0x5002 BFFF

0x4002 BC00 - 0x4002 BFFF

18 K

Reserved

-

-

-

-

-

0x5002 B000 - 0x5002 BBFF

0x4002 B000 - 0x4002 BBFF

3K

DMA2D

DMA2D register map

-

X

X

X

0x5002 A000 - 0x5002 AFFF

0x4002 A000 - 0x4002 AFFF

4K

JPEG

JPEG codec register map

-

-

-

X

0x5002 7000 - 0x5002 AFFF

0x4002 7000 - 0x4002 AFFF

16 K

Reserved

-

-

-

-

-

0x5002 6000 - 0x5002 6FFF

0x4002 6000 - 0x4002 6FFF

4K

RAMCFG

RAMCFG register map

X

X

X

X
X

0x5002 5000 - 0x5002 5FFF

0x4002 5000 - 0x4002 5FFF

4K

MDF1(3)

MDF register map

X

X

X

0x5002 4400 - 0x5002 4FFF

0x4002 4400 - 0x4002 4FFF

3K

Reserved

-

-

-

-

-

0x5002 4000 - 0x5002 43FF

0x4002 4000 - 0x4002 43FF

1K

TSC

TSC register map

X

X

X

X

0x5002 3400 - 0x5002 3FFF

0x4002 3400 - 0x4002 3FFF

3K

Reserved

0x5002 3000 - 0x5002 33FF

0x4002 3000 - 0x4002 33FF

1K

CRC

0x5002 2400 - 0x5002 2FFF

0x4002 2400 - 0x4002 2FFF

3K

Reserved

0x5002 2000 - 0x5002 23FF

0x4002 2000 - 0x4002 23FF

1K

FLASH registers

-

-

-

-

-

CRC register map

X

X

X

X

-

-

-

-

-

FLASH register map

X

X

X

X

0x5002 1800 - 0x5002 1FFF

0x4002 1800 - 0x4002 1FFF

2K

Reserved

0X5002 1400 - 0x5002 17FF

0X4002 1400 - 0x4002 17FF

1K

FMAC

0X5002 1000 - 0x5002 13FF

0X4002 1000 - 0x4002 13FF

1K

CORDIC

CORDIC register map

X

X

X

X

0x5002 0000 - 0x5002 0FFF

0x4002 0000 - 0x4002 0FFF

4K

GPDMA1

GPDMA register map

X

X

X

X

RM0456 Rev 6

-

-

-

-

-

FMAC register map

X

X

X

X

<!-- pagebreak -->

150

RM0456

APB1

STM32U59x/5Ax

STM32U5Fx/5Gx

STM32U575/585

STM32U535/545

Nonsecure boundary
address

Size (bytes)

APB2

Bus

Table 6. Memory map and peripheral register boundary addresses (continued)

Peripheral

0x5001 7C00 - 0x5001 FFFF

0x4001 7C00 - 0x4001 FFFF

33 K

Reserved

-

-

-

-

-

0x5001 6C00 - 0x5001 7BFF

0x4001 6C00 - 0x4001 7BFF

4K

DSI

DSI register map

-

-

X

X

0x5001 6800 - 0x5001 6BFF

0x4001 6800 - 0x4001 6BFF

1K

LTDC

LTDC register map

-

-

X

X

0x5001 6400 - 0x5001 67FF

0x4001 6400 - 0x4001 67FF

1K

GFXTIM

GFXTIM register map

-

-

-

X

0x5001 6400 - 0x5001 6BFF

0x4001 6400 - 0x4001 6BFF

2K

USB RAM

-

X

-

-

-

0x5001 6000 - 0x5001 63FF

0x4001 6000 - 0x4001 63FF

1K

USB

USB register map

X

-

-

-

0x5001 5C00 - 0x5001 5FFF

0x4001 5C00 - 0x4001 5FFF

1K

Reserved

0x5001 5800 - 0x5001 5BFF

0x4001 5800 - 0x4001 5BFF

1K

SAI2

0x5001 5400 - 0x5001 57FF

0x4001 5400 - 0x4001 57FF

1K

SAI1

0x5001 4C00 - 0x5001 53FF

0x4001 4C00 - 0x4001 53FF

2K

Reserved

0x5001 4800 - 0x5001 4BFF

0x4001 4800 - 0x4001 4BFF

1K

TIM17

0x5001 4400 - 0x5001 47FF

0x4001 4400 - 0x4001 47FF

1K

TIM16

0x5001 4000 - 0x5001 43FF

0x4001 4000 - 0x4001 43FF

1K

TIM15

0x5001 3C00 - 0x5001 3FFF

0x4001 3C00 - 0x4001 3FFF

1K

0x5001 3800 - 0x5001 3BFF

0x4001 3800 - 0x4001 3BFF

0x5001 3400 - 0x5001 37FF
0x5001 3000 - 0x5001 33FF

Secure boundary
address

Peripheral register
map

SAI register map
TIM16/TIM17 register map

-

-

-

-

-

X

X

X

X

X

X

X

-

-

-

-

X

X

X

X

X

X

X

X

TIM15 register map

X

X

X

X

Reserved

-

-

-

-

-

1K

USART1

USART register map

X

X

X

X

0x4001 3400 - 0x4001 37FF

1K

TIM8

TIMx register map

X

X

X

X

0x4001 3000 - 0x4001 33FF

1K

SPI1

SPI register map

X

X

X

X

0x5001 2C00 - 0x5001 2FFF

0x4001 2C00 - 0x4001 2FFF

1K

TIM1

TIMx register map

X

X

X

X

0x5000 E000 - 0x5001 2BFF

0x4000 E000 - 0x4001 2BFF

19 K

Reserved

-

-

-

-

-

0x5000 DC00 - 0x5000 DFFF

0x4000 DC00 - 0x4000 DFFF

1K

UCPD1

UCPD register map

-

X

X

X

0x5000 B000 - 0x5000 DBFF

0x4000 B000 - 0x4000 DBFF

11 K

Reserved

-

-

-

-

0x5000 AC00 - 0x5000 AFFF

0x4000 AC00 - 0x4000 AFFF

1K

FDCAN1 RAM

-

X

X

X

0x5000 A800 - 0x5000 ABFF

0x4000 A800 - 0x4000 ABFF

1K

Reserved

-

-

-

-

X

(4)

X

0x5000 A400 - 0x5000 A7FF

0x4000 A400 - 0x4000 A7FF

1K

FDCAN1

FDCAN register map

X

X

X

0x5000 A000 - 0x5000 A3FF

0x4000 A000 - 0x4000 A3FF

1K

Reserved

-

-

-

-

-

0x5000 9C00 - 0x5000 9FFF

0x4000 9C00 - 0x4000 9FFF

1K

I2C6

-

-

X

X

0x5000 9800 - 0x5000 9BFF

0x4000 9800 - 0x4000 9BFF

1K

I2C5

0x5000 9400 - 0x5000 97FF

0x4000 9400 - 0x4000 97FF

1K

LPTIM2

0x5000 8800 - 0x5000 93FF

0x4000 8800 - 0x4000 93FF

3K

Reserved

0x5000 8400 - 0x5000 87FF

0x4000 8400 - 0x4000 87FF

1K

I2C4

0x5000 6800 - 0x5000 83FF

0x4000 6800 - 0x4000 83FF

8K

0x5000 6400 - 0x5000 67FF

0x4000 6400 - 0x4000 67FF

1K

I2C register map
LPTIM register map

(4)

-

-

X

X

X

X

X

X

-

-

-

-

-

I2C register map

X

X

X

X

Reserved

-

-

-

-

-

USART6

USART register map

-

-

X

X

CRS register map

X

X

X

X

-

-

-

-

-

X

X

X

X

0x5000 6000 - 0x5000 63FF

0x4000 6000 - 0x4000 63FF

1K

CRS

0x5000 5C00 - 0x5000 5FFF

0x4000 5C00 - 0x4000 5FFF

1K

Reserved

0x5000 5800 - 0x5000 5BFF

0x4000 5800 - 0x4000 5BFF

1K

I2C2

0x5000 5400 - 0x5000 57FF

0x4000 5400 - 0x4000 57FF

1K

I2C1

X

X

X

X

0x5000 5000 - 0x5000 53FF

0x4000 5000 - 0x4000 53FF

1K

UART5

X

X

X

X

0x5000 4C00 - 0x5000 4FFF

0x4000 4C00 - 0x4000 4FFF

1K

UART4

X

X

X

X

0x5000 4800 - 0x5000 4BFF

0x4000 4800 - 0x4000 4BFF

1K

USART3

X

X

X

X

0x5000 4400 - 0x5000 47FF

0x4000 4400 - 0x4000 47FF

1K

USART2

-

X

X

X

0x5000 3C00 - 0x5000 43FF

0x4000 3C00 - 0x4000 43FF

2K

Reserved

-

-

-

-

-

0x5000 3800 - 0x5000 3BFF

0x4000 3800 - 0x4000 3BFF

1K

SPI2

SPI register map

X

X

X

X

0x5000 3400 - 0x5000 37FF

0x4000 3400 - 0x4000 37FF

1K

Reserved

-

-

-

-

-

<!-- pagebreak -->

