1177

Hexadeca-SPI interface (HSPI)

30.4.3

RM0456

HSPI interface to memory modes
The HSPI supports the following protocols:
•

regular-command protocol

•

HyperBus protocol

The HSPI uses from 6 to 21 signals to interface with a memory, depending
on the functional mode:
•

NCS: chip-select.

•

CLK: communication clock

•

NCLK: inverted clock used only in the 1.8 V HyperBus protocol

•

DQS0, DQS1: data strobe used only in regular-command protocol

•

IO[3:0]: data bus LSB

•

IO[7:4]:

•

30.4.4

–

data bus MSB used in dual-quad and octal configurations

–

data bus used as possible remap for quad-SPI mode

IO[15:8]:
–

data bus MSB used in dual-quad, dual-octal and 16-bit configurations

–

data bus used as possible remap for octal-SPI mode

–

IO[15:12] and IO[11:8] can also be used as possible remap for quad-SPI mode

HSPI regular-command protocol
When in regular-command protocol, the HSPI communicates with the external device using
commands. Each command can include the following phases:
•

instruction phase

•

address phase

•

alternate-byte phase

•

dummy-cycle phase

•

data phase

Only the data phase uses 16 bits. Instruction, address, and alternate phases use only the
eight LSB of the bus as for octal configuration.
Any of these phases can be configured to be skipped, but single-phase commands
supported are only those with instruction phase.
The NCS falls before the start of each command and rises again after each command
finishes.
In memory-mapped mode, both read and write operation are supported: as a consequence,
some of the configuration registers are duplicated to specify write operations (read
operations are configured using regular registers).

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Hexadeca-SPI interface (HSPI)
Figure 169. SDR read command in 16-bit configuration

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

1. Data (such as D0, D1, D2) are sent in 16-bit configuration mode over IO[15:0]. Only the command and
address are sent over IO[7:0] as for octal mode.

The specific regular-command protocol features are configured through the registers in the
0x0100-0x01FC offset range.

Instruction phase
During this phase, a 1- to 4-byte instruction is sent to the external device specifying the type
of operation to be performed. The size of the instruction to be sent is configured in
ISIZE[1:0] of HSPI_CCR and the instruction is programmed in INSTRUCTION[31:0] of
HSPI_IR.
The instruction phase can optionally send:
•

1 bit at a time (over IO0, SO signal in single-SPI mode)

•

2 bits at a time (over IO0/IO1 in dual-SPI mode)

•

4 bits at a time (over IO0 to IO3 in quad-SPI mode)

•

8 bits at a time (over IO0 to IO7 in octal-SPI mode, or in 16-bit SPI mode)

This can be configured using IMODE[2:0] of HSPI_CCR.
The instruction can be sent in DTR (double-transfer rate) mode on each rising and falling
edge of the clock, by setting IDTR in HSPI_CCR.
When IMODE[2:0] = 000 in HSPI_CCR, the instruction phase is skipped, and the command
sequence starts with the address phase, if present.
When in memory-mapped mode, the instruction used for the write operation is specified in
HSPI_WIR and the instruction format is specified in HSPI_WCCR. The instruction used for
the read operation and the instruction format are specified in HSPI_IR and HSPI_CCR.

Address phase
In the address phase, 1 to 4 bytes are sent to the external device, to indicate the address of
the operation. The number of address bytes to be sent is configured in ADSIZE[1:0] of
HSPI_CCR.
In indirect and automatic status-polling modes, the address bytes to be sent are specified in
ADDRESS[31:0] of HSPI_AR. In memory-mapped mode, the address is given directly via
the AHB (from any master in the system).
The address phase can send:
•

1 bit at a time (over IO0, SO signal in single-SPI mode)

•

2 bits at a time (over IO0/IO1 in dual-SPI mode)

•

4 bits at a time (over IO0 to IO3 in quad-SPI mode)

•

8 bits at a time (over IO0 to IO7 in octal-SPI mode, or in 16-bit SPI mode)

RM0456 Rev 6

<!-- pagebreak -->

1177

Hexadeca-SPI interface (HSPI)

RM0456

This can be configured using ADMODE[2:0] of HSPI_CCR.
The address can be sent in DTR mode (on each rising and falling edge of the clock) setting
ADDTR in HSPI_CCR.
When ADMODE[2:0] = 000, the address phase is skipped and the command sequence
proceeds directly to the next phase, if any.
In memory-mapped mode, the address format for the write operation is specified in
HSPI_WCCR. The address format for the read operation is specified in HSPI_CCR.

Warning:

Some memory specifications consider that each address
corresponds to a 16-bit value. HSPI considers that each
address corresponds to an 8-bit value.So the software needs
to multiply the address by two when accessing the memory
registers.

Alternate-byte phase
In the alternate-bytes phase, 1 to 4 bytes are sent to the external device, generally to control
the mode of operation. The number of alternate bytes to be sent is configured in
ABSIZE[1:0] of HSPI_CCR. The bytes to be sent are specified in HSPI_ABR.
The alternate-byte phase can send:
•

1 bit at a time (over IO0, SO signal in single-SPI mode)

•

2 bits at a time (over IO0/IO1 in dual-SPI mode)

•

4 bits at a time (over IO0 to IO3 in quad-SPI mode)

•

8 bits at a time (over IO0 to IO7 in octal-SPI mode, or in 16-bit SPI mode)

This can be configured using ABMODE[2:0] of HSPI_CCR.
The alternate bytes can be sent in DTR mode (on each rising and falling edge of the clock)
setting ABDTR of HSPI_CCR.
When ABMODE[2:0] = 000, the alternate-bytes phase is skipped and the command
sequence proceeds directly to the next phase, if any.
Only a single nibble may need to be sent during the alternate-byte phase rather than a full
byte, such as when the dual-SPI mode is used and only two cycles are used for the
alternate bytes.
In this case, the firmware can use the quad-SPI mode (ABMODE[2:0] = 011), and send
a byte with bits 7 and 3 of ALTERNATE[31:0] set to 1 (keeping the IO3 line high), and bits 6
and 2 set to 0 (keeping the IO2 line low), in HSPI_IR.
The upper two bits of the nibble to be sent are then placed in bits 5:4 of ALTERNATE[31:0],
while the lower two bits are placed in bits 1:0. For example, if the nibble 2 (0010) is to be
sent over IO0/IO1, then ALTERNATE[31:0] must be set to 0x8A (1000_1010).
In memory-mapped mode, the alternate bytes used for the write operation are specified in
HSPI_WABR, and the alternate byte format is specified in HSPI_WCCR. The alternate
bytes used for read operation and the alternate byte format are specified in HSPI_ABR and
HSPI_CCR.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Hexadeca-SPI interface (HSPI)

Dummy-cycle phase (memory latency)
In the dummy-cycle phase, 1 to 31 cycles are given without any data being sent or received,
in order to give the external device, the time to prepare for the data phase when higher clock
frequencies are used. The number of cycles given during this phase is specified in
DCYC[4:0] of HSPI_TCR. In both SDR and DTR modes, the duration is specified as a
number of full CLK cycles.
When DCYC[4:0] = 00000, the dummy-cycle phase is skipped, and the command sequence
proceeds directly to the data phase, if present.
In order to assure enough “turn-around” time for changing the data signals from the output
mode to the input mode, there must be at least one dummy cycle when using the dual-,
quad-, octal-, or 16-bit SPI mode, to receive data from the external device.
In memory-mapped mode, the dummy cycles for the write operations are specified in
HSPI_WTCR. The dummy cycles for the read operation are specified in HSPI_TCR.

Data phase
During the data phase, any number of bytes can be sent to or received from the external
device.
In indirect mode, the number of bytes to be sent/received is specified in HSPI_DLR. In this
mode, the data to be sent to the external device must be written to HSPI_DR. In
indirect-read mode, the data received from the external device is obtained by reading
HSPI_DR.
In automatic status-polling mode, the number of bytes to be received is specified in
HSPI_DLR, and the data received from the external device can be obtained by reading
HSPI_DR.
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

•

16 bits at a time (over IO0 to IO15 in 16-bit SPI mode)

This can be configured using DMODE[2:0] of HSPI_CCR.
The data can be sent or received in DTR mode (on each rising and falling edge of the clock)
setting DDTR of HSPI_CCR.
When DMODE[2:0] = 000, the data phase is skipped, and the command sequence finishes
immediately by raising the NCS. This configuration must be used only in indirect-write
mode.
In memory-mapped mode, the data format for the write operation is specified in
HSPI_WCCR. The data format for the read operation is specified in HSPI_CCR.

DQS use
The DQS signal can be used for data strobing during the read transactions when the device
toggles the DQS aligned with the data.

RM0456 Rev 6

<!-- pagebreak -->

