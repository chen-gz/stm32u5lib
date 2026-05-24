1104

Octo-SPI interface (OCTOSPI)

RM0456

Write operation with no latency
Some devices can also require a zero latency for the write operations. This write-zero
latency can be forced by setting WZL in OCTOSPI_HLCR.
Figure 155. HyperBus write operation with no latency (register write)
NCS
CLK
DQS

Memory drives DQS but master ignores
39:32

47:40

IO[7:0]

31:24

23:16

15:8

7:0

Command-Address

15:8

7:0
Data
MSv71587V1

Latency on page-crossing during the read operations
An additional latency can be needed by some devices for the read operation when crossing
pages.
The initial latency must be respected for any page access, as a consequence, when the first
access is close to the page boundary, a latency is automatically added at the page crossing
to respect the tACC time.
Figure 156. HyperBus read operation page crossing with latency
12 clock
initial latency

NCS

9 words
data

CLK
3 clock initial page
crossing latency

DQS
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

MSv71588V1

28.4.7

Specific features
The OCTOSPI supports some specific features, such as:
•

Wrap support

•

NCS boundary and refresh

•

Communication regulation

Wrap support
The OCTOSPI supports a hybrid wrap as defined by the HyperBus protocol. A hybrid wrap
is also supported in the regular-command protocol.
In hybrid wrap, the transaction can continue after the initial wrap with an incremental
access.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Octo-SPI interface (OCTOSPI)
The wrap size supported by the target memory is configured by WRAPSIZE
in OCTOSPI_DCR2.
Wrap is supported only in memory-read direction and only for data size = 4 bytes. Wrapped
reads are supported for both HyperBus and regular-command protocols. To enable
wrapped-read accesses, the dedicated OCTOSPI_WPxxx registers must be programmed
according to the wrapped-read access characteristics. These registers apply for both
HyperBus and regular-command protocols.
If the target memory is not supporting the hybrid wrap, WRAPSIZE must be set to 0.

Note:

Hybrid wrap requires that the nonwrapped registers (OCTOSPI_CCR, OCTOSPI_TCR,
OCTOSPI_IR) are set according to the memory configuration to satisfy its correct data
prefetch (initiated after the wrap command).
The wrap operation cannot be interrupted by a refresh. The refresh event is only considered
after the wrap completion.

NCS boundary and refresh
Two processes can be activated to regulate the OCTOSPI transactions:
•

NCS boundary

•

Refresh

The NCS boundary feature limits a transaction to a boundary of aligned addresses. The size
of the address to be aligned with, is configured by CSBOUND[4:0] in OCTOSPI_DCR3: it is
equal to 2CSBOUND.
As an example, if CSBOUND(4:0] = 0x4, the boundary is set to 24 = 16 bytes. The NCS is
then released each time the LSB address is equal to 0xF, and each time a new transaction
is issued to address the next data.
If CSBOUND[4:0] = 0, the feature is disabled. A minimum value of three is recommended.
The NCS boundary feature cannot be used for flash memory devices in write mode since a
command is necessary to program another page of the flash memory.
The refresh feature limits the duration of the transactions to the value programmed
by REFRESH[31:0] in OCTOSPI_DCR4. The duration is expressed in number of cycles.
This allows an external RAM to perform its internal refresh operation regularly.
The refresh value must be greater than the minimal transaction size in terms of number of
cycles including the command/address/alternate/dummy phases.
If NCS boundary and refresh are enabled at the same time, the NCS is released on the first
condition met.

Communication regulation
The communication regulation feature limits the maximum length of a transaction to the
value programmed by MAXTRAN[7:0] in OCTOSPI_DCR3.
If the number of clock cycles reaches the MAXTRAN + 1 value, and if the second OCTOSPI
requests access, the NCS is released and a new transaction is issued to address the next
data. If the second OCTOSPI does not request an access, the transaction is not stopped
and the NCS is not released.
If MAXTRAN[7:0] = 0, no limitation occurs.

RM0456 Rev 6

<!-- pagebreak -->

