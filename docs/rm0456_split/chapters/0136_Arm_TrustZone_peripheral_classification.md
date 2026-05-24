139

Memory and bus architecture

2.2.2

RM0456

Arm TrustZone peripheral classification
When the TrustZone security is active, a peripheral can be either securable or
TrustZone-aware type as follows:
•

Securable: peripheral protected by an AHB/APB firewall gate that is controlled from
TZSC controller to define security properties

•

TrustZone-aware: peripheral connected directly to AHB or APB bus and implementing
a specific TrustZone behavior such as a subset of registers being secure.

Refer to GTZC TrustZone system architecture for more details.
The tables below list the securable and TrustZone-aware peripherals within the system.
Table 4. Securable peripherals by TZSC
Bus

AHB3

AHB2

<!-- pagebreak -->

Peripheral

STM32U535/545

STM32U575/585 STM32U59x/5Ax STM32U5Fx/5Gx

ADF1

X

X

X

X

DAC1

X

X

X

X

ADC4

X

X

X

X

OCTOSPI2 registers

-

X

X

X

OCTOSPI1 registers

X

X

X

X

HSPI1 registers

-

-

X

X

FSMC registers

-

X

X

X

SDMMC1

X

X

X

X

SDMMC2

-

X

X

X

OCTOSPIM

-

X

X

X

SAES

X

X

X

X

PKA

X

X

X

X

RNG

X

X

X

X

HASH

X

X

X

X

AES

X

X

X

X

USB

X

-

-

-

OTG_FS

-

X

-

-

OTG_HS

-

-

X

X

DCMI

X

X

X

X

ADC12

X

X

X

X

RM0456 Rev 6

RM0456

Memory and bus architecture
Table 4. Securable peripherals by TZSC (continued)

Bus

Peripheral

STM32U535/545

GFXMMU registers

-

-

X

X

GPU2D registers

-

-

X

X

JPEG

-

-

-

X

DCACHE2 registers

-

-

X

X

(1)

X

X

X

X

ICACHE registers(1)

X

X

X

X

DMA2D

-

X

X

X

TSC

X

X

X

X

CRC

X

X

X

X

RAMCFG

X

X

X

X

MDF1

X

X

X

X

FMAC

X

X

X

X

CORDIC

X

X

X

X

VREFBUF

X

X

X

X

(2)

COMP

X

X

X

X

OPAMP

X

X

X

X

LPTIM4

X

X

X

X

LPTIM3

X

X

X

X

LPTIM1

X

X

X

X

I2C3

X

X

X

X

LPUART1

X

X

X

X

SPI3

X

X

X

X

DSI

-

-

X

X

LTDC

-

-

X

X

GFXTIM

-

-

-

X

SAI2

-

X

X

X

SAI1

X

X

X

X

TIM17

X

X

X

X

TIM16

X

X

X

X

TIM15

X

X

X

X

USART1

X

X

X

X

TIM8

X

X

X

X

SPI1

X

X

X

X

TIM1

X

X

X

X

DCACHE1 registers

AHB1

APB3

APB2

STM32U575/585 STM32U59x/5Ax STM32U5Fx/5Gx

RM0456 Rev 6

<!-- pagebreak -->

