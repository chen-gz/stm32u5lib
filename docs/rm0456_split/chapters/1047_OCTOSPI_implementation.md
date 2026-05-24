•

the regular-command frame format with the command, address, alternate byte, dummy
cycles, and data phase

•

the HyperBus™ frame format

OCTOSPI main features
•

Functional modes: indirect, automatic status-polling, and memory-mapped

•

Read and write support in memory-mapped mode

•

External (P)SRAM memory support

•

Support for single, dual, quad, and octal communication

•

Dual memory configuration, where eight bits can be sent/received simultaneously by
accessing two quad memories in parallel

•

SDR (single-data rate) and DTR (double-transfer rate) support

•

Data strobe support

•

Fully programmable opcode

•

Fully programmable frame format

•

Support wrapped-type access to memory in read direction

•

HyperBus support

•

Integrated FIFO for reception and transmission

•

Asynchronous bus clock versus kernel clock support

•

8-, 16-, and 32-bit data accesses allowed

•

DMA protocol support

•

DMA channel for indirect mode operations

•

Interrupt generation on FIFO threshold, timeout, operation complete, and access error

•

AHB interface with transaction acceptance limited to one: the interface accepts the next
transfer on AHB bus only once the previous is completed on memory side.

RM0456 Rev 6

RM0456

28.3

Octo-SPI interface (OCTOSPI)

OCTOSPI implementation
Table 250. Instances on STM32U5 Series devices
Devices

OCTOSPI1

OCTOSPI2

OCTOSPIM

HSPI1

STM32U535/545

X

-

-

-

STM32U575/585

X

X

X

-

STM32U59x/5Ax

X

X

X

X

STM32U5Fx/5Gx

X

X

X

X

Table 251. OCTOSPI/HSPI implementation
OCTOSPI feature

OCTOSPI1/2

HSPI1

HyperBus standard compliant

X

X

Xccela standard compliant

X

X

XSPI (JESD251C) standard compliant

X

X

AMBA® AHB compliant data interface

X

X

Dual AHB interface

X

X

Asynchronous AHB clock versus kernel clock

X

X

Functional modes: indirect, automatic status-polling, and memory-mapped

X

X

Read and write support in memory-mapped mode

X

X

Dual-quad configuration

X

X

Dual-octal configuration

-

X

SDR (single-data rate) and DTR (double-transfer rate)

X

X

Data strobe (DS,DQS)

X

X

Fully programmable opcode

X

X

Fully programmable frame format

X

X

Integrated FIFO for reception and transmission

X

X

8-, 16-, and 32-bit data accesses

X

X

Interrupt on FIFO threshold, timeout, operation complete, and access error

X

X

Compliant with dual-OCTOSPI arbiter (communication regulation)

X

-

Extended CSHT timeout

X

X

Memory-mapped write

X

X

Refresh counter

X

X

GPDMA interface

X

X

High-speed interface

-

X

Dual chip select

-

-

Extended external memory

-

-

RM0456 Rev 6

<!-- pagebreak -->

