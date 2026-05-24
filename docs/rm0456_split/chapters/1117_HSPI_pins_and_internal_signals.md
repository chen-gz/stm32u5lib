Quad-SPI
memory(1)

RM0456 Rev 6

MSv65075V5

RM0456

Hexadeca-SPI interface (HSPI)
Figure 168. HSPI block diagram for dual-quad configuration

STM32
AHB

Registers/
control

Clock
management

Signals from
ALT function
HSPI_NCLK

RX/TX data
FIFO

AHB

Shift register
DMA signals

High-speed interface

hspi_ker_clk
hspi_nrst

Quad-SPI
memory 1(1)

Interrupts
6

HSPI_CLK

CLK

HSPI_IO0

IO0

HSPI_IO1

IO1

HSPI_IO2

IO2

HSPI_IO3

IO3

HSPI_IO4

NCS

HSPI_IO5
HSPI_IO6
HSPI_IO7

Quad-SPI
memory 2

HSPI_NCS
HSPI_DQS0

CLK

HSPI_IO8

IO0

HSPI_IO9

IO1

HSPI_IO10

IO2

HSPI_IO11

IO3

HSPI_IO12

NCS

HSPI_IO13
HSPI_IO14
HSPI_IO15

HSPI

HSPI_DQS1

(1) The Quad-SPI memories are connected to HSPI_IO[0:7], but can also be connected to HSPI_IO[8:15].

30.4.2

MSv65076V4

HSPI pins and internal signals
Table 265. HSPI input/output pins

Pin name

Type

Description

HSPI_NCLK

Output

HSPI inverted clock to support 1.8 V HyperBus protocol

HSPI_CLK

Output

HSPI clock

HSPI_IOn (n = 0 to 15) Input/output HSPI data pins
HSPI_NCS
HSPI_DQS0,1

Caution:

Output

Chip select for the memory

Input/output Data strobe/write mask signal from/to the memory

Use the same configuration (output speed, HSLV) for all HSPI input/output pins to avoid any
data corruption.
Table 266. HSPI internal signals

Signal name

Type

Description

hspi_hclk

Input

HSPI AHB clock

hspi_ker_ck

Input

HSPI kernel clock

hspi_dma

NA

DMA request signal

hspi_it

Output

Global interrupt line (see Table 270 for the multiple sources of interrupt)

RM0456 Rev 6

<!-- pagebreak -->

