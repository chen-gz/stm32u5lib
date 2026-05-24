1104

Octo-SPI interface (OCTOSPI)

28.4.4

RM0456

OCTOSPI regular-command protocol
When in regular-command protocol, the OCTOSPI communicates with the external device
using commands. Each command can include the following phases:
•

Instruction phase

•

Address phase

•

Alternate-byte phase

•

Dummy-cycle phase

•

Data phase

Any of these phases can be configured to be skipped but, in case of single-phase
command, the only use case supported is instruction-phase-only.
The NCS falls before the start of each command and rises again after each command
finishes.
In memory-mapped mode, both read and write operations are supported: as a
consequence, some of the configuration registers are duplicated to specify write operations
(read operations are configured using regular registers).
Figure 147. SDR read command in octal configuration

§ §

NCS
CLK
ECh

13h

A[31:24] A[23:16] A[15:8]

A[7:0]

Address

§

Pre-drive

IO[7:0]

D0

D1

D2

D3

Dummy
MSv43488V1

The specific regular-command protocol features are configured through the registers in the
0x0100-0x01FC offset range.

Instruction phase
During this phase, a 1- to 4-byte instruction is sent to the external device specifying the type
of operation to be performed. The size of the instruction to be sent is configured
by ISIZE[1:0] in OCTOSPI_CCR and the instruction is programmed in INSTRUCTION[31:0]
of OCTOSPI_IR.
The instruction phase can optionally send:
•

1 bit at a time (over IO0, SO single in single-SPI mode)

•

2 bits at a time (over IO0/IO1 in dual-SPI mode)

•

4 bits at a time (over IO0 to IO3 in quad-SPI mode)

•

8 bits at a time (over IO0 to IO7 in octal-SPI mode)

This can be configured using IMODE[2:0] of OCTOSPI_CCR.
The instruction can be sent in DTR mode on each rising and falling edge of the clock, by
setting IDTR in OCTOSPI_CCR.
When IMODE[2:0] = 000 in OCTOSPI_CCR, the instruction phase is skipped, and the
command sequence starts with the address phase, if present.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Octo-SPI interface (OCTOSPI)
In memory-mapped mode, the instruction used for the write operation is specified
in OCTOSPI_WIR, and the instruction format is specified in OCTOSPI_WCCR. The
instruction used for the read operation and the instruction format are specified in
OCTOSPI_IR and OCTOSPI_CCR.

Address phase
In the address phase, 1 to 4 bytes are sent to the external device, to indicate the address of
the operation. The number of address bytes to be sent is configured by ADSIZE[1:0]
in OCTOSPI_CCR.
In indirect and automatic status-polling modes, the address bytes to be sent are specified
by ADDRESS[31:0] in OCTOSPI_AR. In memory-mapped mode, the address is given
directly via the AHB (from any master in the system).
The address phase can send:
•

1 bit at a time (over IO0, SO single in single-SPI mode)

•

2 bits at a time (over IO0/IO1 in dual-SPI mode)

•

4 bits at a time (over IO0 to IO3 in quad-SPI mode)

•

8 bits at a time (over IO0 to IO7 in octal-SPI mode)

This can be configured using ADMODE[2:0] in OCTOSPI_CCR.
The address can be sent in DTR mode (on each rising and falling edge of the clock) setting
ADDTR in OCTOSPI_CCR.
When ADMODE[2:0] = 000, the address phase is skipped and the command sequence
proceeds directly to the next phase, if any.
In memory-mapped mode, the address format for the write operation is specified in
OCTOSPI_WCCR. The address format for the read operation is specified in
OCTOSPI_CCR.

Alternate-byte phase
In the alternate-byte phase, 1 to 4 bytes are sent to the external device, generally to control
the mode of operation. The number of alternate bytes to be sent is configured
by ABSIZE[1:0] in OCTOSPI_CCR. The bytes to be sent are specified in OCTOSPI_ABR.
The alternate-byte phase can send:
•

1 bit at a time (over IO0, SO single in single-SPI mode)

•

2 bits at a time (over IO0/IO1 in dual-SPI mode)

•

4 bits at a time (over IO0 to IO3 in quad-SPI mode)

•

8 bits at a time (over IO0 to IO7 in octal-SPI mode)

This can be configured using ABMODE[2:0] in OCTOSPI_CCR.
The alternate bytes can be sent in DTR mode (on each rising and falling edge of the clock)
setting ABDTR in OCTOSPI_CCR.
When ABMODE[2:0] = 000, the alternate-byte phase is skipped and the command
sequence proceeds directly to the next phase, if any.
There may be times when only a single nibble needs to be sent during the alternate-byte
phase rather than a full byte, such as when the dual-SPI mode is used and only two cycles
are used for the alternate bytes.

RM0456 Rev 6

<!-- pagebreak -->

1104

Octo-SPI interface (OCTOSPI)

RM0456

In this case, the firmware can use the quad-SPI mode (ABMODE[2:0] = 011) and send a
byte with bits 7 and 3 of ALTERNATE[31:0] set to 1 (keeping the IO3 line high), and bits 6
and 2 set to 0 (keeping the IO2 line low), in OCTOSPI_IR.
The upper two bits of the nibble to be sent are then placed in bits 5:4 of ALTERNATE[31:0]
while the lower two bits are placed in bits 1:0. For example, if the nibble 2 (0010) is to be
sent over IO0/IO1, then ALTERNATE[31:0] must be set to 0x8A (1000_1010).
In memory-mapped mode, the alternate bytes used for the write operation are specified in
OCTOSPI_WABR, and the alternate byte format is specified in OCTOSPI_WCCR. The
alternate bytes used for read operation and the alternate byte format are specified in
OCTOSPI_ABR and OCTOSPI_CCR.

Dummy-cycle phase (memory latency)
In the dummy-cycle phase, 1 to 31 cycles are given without any data being sent or received,
in order to give the external device, the time to prepare for the data phase when higher clock
frequencies are used. The number of cycles given during this phase is specified
by DCYC[4:0] in OCTOSPI_TCR. In both SDR and DTR modes, the duration is specified as
a number of full CLK cycles.
When DCYC[4:0] = 00000, the dummy-cycle phase is skipped, and the command sequence
proceeds directly to the data phase, if present.
In order to assure enough “turn-around” time for changing the data signals from the output
mode to the input mode, there must be at least one dummy cycle when using the dual-SPI,
the quad-SPI, or the octal-SPI mode, to receive data from the external device.
In memory-mapped mode, the dummy cycles for the write operations are specified
in OCTOSPI_WTCR. The dummy cycles for the read operation are specified
in OCTOSPI_TCR.

Data phase
During the data phase, any number of bytes can be sent to or received from the external
device.
In indirect mode, the number of bytes to be sent/received is specified in OCTOSPI_DLR. In
this mode, the data to be sent to the external device must be written to OCTOSPI_DR, while
in indirect-read mode the data received from the external device is obtained by reading
OCTOSPI_DR.
In automatic status-polling mode, the number of bytes to be received is specified in
OCTOSPI_DLR, and the data received from the external device can be obtained by reading
OCTOSPI_DR.
In memory-mapped mode, the data read or written, is sent or received directly over the AHB
to the Cortex core or to a DMA.
The data phase can send/receive:
•

1 bit at a time (over IO0/IO1 (SO/SI respectively) in single-SPI mode)

•

2 bits at a time (over IO0/IO1 in dual-SPI mode)

•

4 bits at a time (over IO0 to IO3 in quad-SPI mode)

•

8 bits at a time (over IO0 to IO7 in octal-SPI mode)

This can be configured using DMODE[2:0] in OCTOSPI_CCR.

<!-- pagebreak -->

