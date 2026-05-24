1177

Hexadeca-SPI interface (HSPI)

RM0456

Figure 179. HyperBus write operation with initial latency (16-bit mode)
NCS
t RWR = Read Write Recovery

tACC = Access

CLK
High = 2x Latency count
Low = 1x Latency count

DQS[1:0]

CLK and data are
center aligned.

Latency count
IO[15:0]

47:40 39:32 31:24 23:16

15:8

7:0

Command-Address
Host drives IO[15:0] and memory drives DQS[1:0]
Command-Address are sent on IO[7:0]
IO[15:8] are unused and drived H or L by the host

Dn
A

Dn
B

Dn+1
A

Dn+1
B

Host drives data on IO[15:0]
and DQS[1:0]

MSv71532V2

30.4.7

Specific features
The HSPI supports some specific features, such as:
•

wrap support

•

NCS boundary and refresh

Wrap support
The HSPI supports an hybrid wrap as defined by the HyperBus protocol. A hybrid wrap is
also supported in the regular-command protocol.
In hybrid wrap, the transaction can continue after the initial wrap with an incremental
access.
The wrap size supported by the target memory is configured by WRAPSIZE in HSPI_DCR2.
Wrap is supported only in memory-read direction and only for data size = 4 bytes. Wrapped
reads are supported for both HyperBus and regular-command protocols. To enable
wrapped-read accesses, the dedicated registers HSPI_WPxxx must be programmed
according to the wrapped-read access characteristics. The dedicated HSPI_WPxxx
registers apply for both HyperBus and regular-command protocols.
If the target memory is not supporting the hybrid wrap, WRAPSIZE must be set to 0.
Note:

Hybrid wrap requires that the nonwrapped registers (HSPI_CCR, HSPI_TCR, HSPI_IR) are
set according to the memory configuration to satisfy its correct data prefetch (initiated after
the wrap command).
The wrap operation cannot be interrupted by a refresh. The refresh event is only considered
after the wrap completion.

NCS boundary and refresh
Two processes can be activated to regulate the HSPI transactions:

<!-- pagebreak -->

