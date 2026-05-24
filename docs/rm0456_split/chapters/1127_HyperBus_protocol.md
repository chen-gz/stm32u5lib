RM0456 Rev 6

RM0456

30.4.6

Hexadeca-SPI interface (HSPI)

HyperBus protocol
The HSPI can communicate with the external device using the HyperBus protocol.
The HyperBus uses 11 to 12 pins in 8-bit data mode, or 19 to 20 pins in 16-bit data mode
depending on the operating voltage:
•

IO[7:0] as bidirectional data bus for 8-bit data mode and IO[15:0] as bidirectional data
bus for 16-bit data mode

•

DQS for read and write data strobe and latency insertion

•

NCS

•

CLK

•

NCLK for 1.8 V operations (to support this mode, the device must be powered
with 1.8 V)

The HyperBus does not require any command specification nor any alternate bytes. As a
consequence, a separate register set is used to define the timing of the transaction.
The HyperBus frame is composed of the following phases:
•

command/address phase

•

data phase

The NCS falls before the start of a transaction and rises again after each transaction
finishes.
Figure 173. Example of HyperBus read operation (8-bit data mode)
NCS
t RWR =Read write recovery

t ACC = Initial access

CLK

DQS0

High = 2x Latency count
Low = 1x Latency count

DQS0 and data
are edge aligned.

Latency count

IO[7:0]

47:40

39:32

31:24

23:16

15:8

7:0

Command-Address
Host drives IO[7:0] and memory drives DQS0

Dn
A

Dn
B

Dn+1
A

Dn+1
B

Memory drives IO[7:0]
and DQS0
MSv43492V2

The specific HyperBus features are configured through the registers in the 0x0200-0x02FC
offset range.

Command/address phase
During this initial phase, the HSPI sends 48 bits over IO[7:0] to specify the operations to be
performed with the external device.

RM0456 Rev 6

<!-- pagebreak -->

1177

Hexadeca-SPI interface (HSPI)

RM0456
Table 267. Command/address phase description

CA bit

Bit name

47

R/W#

46

Address space

45

Burst type

44-16

Description
Identifies the transaction as a read or a write
Indicates if the transaction accesses the memory or the register
space
Indicates if the burst is linear or wrapped

Row and upper
Selects the row and the upper column addresses
column address

15-3

Reserved

2-0

Lower column
address

Selects the starting 16-bit word within the half page

The address space is configured through the memory type MTYP[2:0] of HSPI_DCR1.
The total size of the device must be considered through DEVSIZE[4:0] of HSPI_DCR1. In
case of multi-chip product (MCP), the device size is the sum of all the sizes of all the MCP
dies.

Warning:

Some memory specifications consider that each address
corresponds to a 16-bit value. The HSPI considers that each
address corresponds to an 8-bit value. So, the software
needs to multiple the address by two when accessing the
memory registers.

Read/write operation with initial latency
The HyperBus read and write operations need to respect two timings:
•

tRWR: minimal read/write recovery time for the device (defined by TRWR[7:0]
in HSPI_HLCR)

•

tACC: access time for the device (defined in TACC[7:0] of HSPI_HLCR) according to the
memory latency

During the read operation, the DQS0/1 is used by the device, in two ways (see Figure 173):

<!-- pagebreak -->

•

during the command/address phase, to request an additional latency

•

during the data phase, for data strobing

RM0456 Rev 6

RM0456

Hexadeca-SPI interface (HSPI)
During the write operation, the DQS0/1 is used:
•

by the device, during the command/address phase, to request an additional latency.

•

by the HSPI, during the data phase, for write data masking.
Figure 174. HyperBus write operation with initial latency (8-bit data mode)
NCS
t RWR =Read Write Recovery

tACC= Access

CLK

DQS0

High = 2x Latency count
Low = 1x Latency count

CLK and data
are center aligned.

Latency count
IO[7:0]

47:40 39:32 31:24 23:16

15:8

Dn
A

7:0

Command-Address

Dn
B

Dn+1
A

Dn+1
B

Host drives IO[7:0]
and DQS0

Host drives IO[7:0] and memory drives DQS0

MSv43494V2

Read/write operation with additional latency
If the device needs an additional latency (during refresh period of an SDRAM for example),
DQS0/1 must be tied to one during one of the DQS signals, during the command/address
phase.
An additional tACC duration is added by the HSPI to meet the device request.
Figure 175. HyperBus read operation with additional latency (8-bit data mode)
NCS
tRWR=Read write recovery

Additional latency

tACC = Access

Latency count 1

Latency count 2

CLK

DQS0

IO[7:0]

High = 2x Latency count
Low = 1x Latency count

47:40 39:32 31:24 23:16 15:8

DQS0 and data
are edge aligned.
Dn
A

7:0

Command-Address

Dn
B

Dn+1
A

Dn+1
B

Memory drives IO[7:0]
and DQS0

Host drives IO[7:0] and memory drives DQS0

RM0456 Rev 6

MSv43495V2

<!-- pagebreak -->

1177

Hexadeca-SPI interface (HSPI)

RM0456

Figure 176. HyperBus write operation with additional latency (8-bit data mode)
NCS
Additional latency
tRWR= Read write recovery

t ACC = Initial access

CLK
DQS0

High = 2x Latency count
Low = 1x Latency count
Latency count 1

IO[7:0]

47:40 39:32 31:24 23:16 15:8

CLK and data
are center aligned.

Latency count 2
Dn
A

7:0

Dn
B

Dn+1 Dn+1
B
A

Host drives IO[7:0]
and DQS0

Command-Address
Host drives IO[7:0] and memory drives DQS0

MSv43496V2

Fixed latency mode
Some devices or some applications may not want to operate with a variable latency time as
described above.
The latency can be forced to 2 x tACC by setting LM of HSPI_HLCR.
In this HSPI latency mode, the state of the DQS signal is not taken into account by the HSPI
and an additional latency is always added leading to a fixed 2 x tACC latency time.

Write operation with no latency
Some devices can also require a zero latency for the write operations. This write-zero
latency can be forced by setting WZL in HSPI_HLCR.
Figure 177. HyperBus write operation with no latency (register write)
NCS
CLK
DQS0
IO[7:0]

Memory drives DQS0 but master ignores
47:40

39:32

31:24

23:16

Command-Address

15:8

7:0

15:8

7:0
Data
MSv43497V2

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Hexadeca-SPI interface (HSPI)

Latency on page-crossing during the read operations
An additional latency can be needed by some devices for the read operation when crossing
pages.
The initial latency must be respected for any page access, as a consequence, when the first
access is close to the page boundary, a latency is automatically added at the page crossing
to respect the tACC time.
Figure 178. HyperBus read operation page crossing with latency (8-bit data mode)
12 clock
initial latency

NCS

9 words
data

CLK
3 clock initial page
crossing latency

DQS0
IO[7:0]

A0 02

46 8A 80

07

Read from Address = 123457h

dd

dd dd dd

dd dd dd dd dd dd

dd dd dd dd

Address Address
123457 123458

Address Address Address
12345D 12345E 12345F

Address Address
123460 123461

MSv43498V3

16-bit data transfer using HyperBus
In HyperBus protocol, the HSPI supports a dual-octal configuration (16-bit data transfers)
when DMM = 1 in HSPI_CR: one octal HyperBus memory is connected to IO0-IO7 and
another is connected to IO8-IO15. These memories share all signals except DQS that are
dedicated.
For 16-bit data transfers, DMODE[2:0] must be to equal to 101. Any other value in
DMODE[2:0] correspond to 8-bit data transfer (quad-, dual-, and single-bit data transfer do
not exist in HyperBus protocol). Command-address phase is always using 8 bits in
HyperBus protocol (from IO8-IO15). Only the data is on 16-bit for write or read operations
accessing the memory space (from IO0-IO15) as shown for instance in Figure 179. For the
memory register accesses, the data is on 8-bit (from IO0 to IO7, IO8 to IO15 being not used
but driven by the controller) as shown in Figure 177.
The behavior of the interface at protocol-level is exactly the same as for HyperBus octal
configuration, as described above, except that the variable latency is not supported
in dual-octal HyperBus configuration. LM in HSPI_HLCR must be set.

RM0456 Rev 6

<!-- pagebreak -->

