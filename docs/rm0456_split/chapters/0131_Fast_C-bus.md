SRD peripherals and SRAM4, including AHB to APB bridge,
and APB peripherals (connected to APB3)

RM0456 Rev 6

RM0456

Memory and bus architecture
The bus matrix provides access from a master to a slave, enabling concurrent access and
efficient operation even when several high-speed peripherals work simultaneously. This
architecture is shown in the figure below.
Figure 1. System architecture
APB1 peripherals
APB2 peripherals
USB
OTG
HS

LTDC

GPU2D

GFXMMU

Legend
Bus multiplexer

Master Interface

Fast bus multiplexer

Slave Interface

Fast bus multiplexer on STM32U59x/5Ax/5Fx/5Gx
Fast bus multiplexer on STM32U575/585

M1 port

SD
MMC2

M0 port

Port 0

Fast-bus

Slow-bus

Kbyte)
32-Kbyte

SD
MMC1

Port 1

ICACHE (8/32-

GPDMA1 DMA2D

S-bus

C-bus

Cortex-M33
with TrustZone mainline and FPU

MPCBBx: Block-based memory protection controller
MPCWMx: Watermark-based memory protection controller

DCACHE1

DCACHE2

(4/16-Kbyte)

(16-Kbyte)

Peripheral not present in STM32U535/545
Peripheral not present in STM32U535/545/575/585
Peripheral present only in STM32U5Fx/5Gx

128-bit cache refill
FLASH
(512-Kbyte/
2/4-Mbyte)

MPCBB1

SRAM1

MPCBB2

SRAM2

MPCBB3

SRAM3

MPCBB5

SRAM5

MPCBB6

SRAM6

MPCWM4

BKPSRAM

MPCWM1

OTFDEC1

OCTOSPI1

MPCWM5

OTFDEC2

OCTOSPI2

AHB1
peripherals
AHB2
peripherals

MPCWM6

HSPI1

MPCWM2
MPCWM3

FSMC
MPCBB4

SRD
32-bit bus matrix

SRAM4
AHB3
peripherals
MSv70738V1

2.1.1

Fast C-bus
This bus connects the C-bus of the Cortex-M33 core to the internal flash memory and
to the bus matrix via the instruction cache. This bus is used for instruction fetch and data
access to the internal memories mapped in the code region. This bus targets the internal
flash memory and the internal SRAMs (SRAM1, SRAM2, SRAM3, SRAM5, and SRAM6).
SRAM1, SRAM2, SRAM3, SRAM5, and SRAM6 are accessible on this bus with a
continuous mapping.

2.1.2

Slow C-bus
This bus connects the C-bus of the Cortex-M33 core to the bus matrix via the instruction
cache. This bus is used for instruction fetch and data access to the external memories
mapped in the code region. This bus targets the external memories (FSMC, HSPI1,
and OCTOSPIs).
RM0456 Rev 6

<!-- pagebreak -->

