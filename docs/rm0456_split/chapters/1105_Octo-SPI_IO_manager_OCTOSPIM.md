0

Res.

0x01940x019C

0

0

Res.

Reset value

0

Res.

Reserved
INSTRUCTION[31:0]

Res.

0x0190

Reserved
OCTOSPI_WIR

0

Res.

0x018C

0

Res.

Reset value

DCYC[4:0]

RM0456 Rev 6

RM0456

29

Octo-SPI I/O manager (OCTOSPIM)

Octo-SPI I/O manager (OCTOSPIM)
This section does not apply to STM32U535/545 devices.

29.1

Introduction
The Octo-SPI I/O manager is a low-level interface that enables an efficient OCTOSPI pin
assignment with a full I/O matrix (before alternate function map), and multiplex of
single/dual/quad/octal SPI interfaces over the same bus.

29.2

29.3

OCTOSPIM main features
•

Supports up to two single/dual/quad/octal SPI interfaces

•

Supports up to two ports for pin assignment

•

Fully programmable I/O matrix for pin assignment by function (data/control/clock)

OCTOSPIM implementation
The following table describes the OCTOSPIM implementation.
Table 260. OCTOSPIM implementation
OCTOSPI feature

Available on the devices

Supports up to two single/dual/quad interfaces

X

Fully I/O multiplexing capability

X

Supports time-multiplexed mode

X

Supports high-speed interface

-

Chip select selection if OCTOSPI provides dual chip select

-

Supports 16-bit data interface and dual-octal mode

-

29.4

OCTOSPIM functional description

29.4.1

OCTOSPIM block diagram
The block diagram of the OCTOSPI I/O manager is shown in Figure 163.

RM0456 Rev 6

<!-- pagebreak -->

