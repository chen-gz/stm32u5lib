RM0456 Rev 6

RM0456

18.4.10

Low-power direct memory access controller (LPDMA)

LPDMA direct transfers
There is a single transfer operation mode called the direct mode. Any LPDMA channel is
used in direct mode. Any channel is implemented without any FIFO (for every channel x,
dma_fifo_size[x] = 0).

LPDMA single
A programmed transfer at the lowest level is called a LPDMA single.
A LPDMA single data width is 1, 2 or 4 bytes, as defined by the SDW_LOG2[1:0] and
DDW_LOG2[1:0] fields of LPDMA_CxTR1(respectively for source and destination).
Note:

The user must not assign a 8-byte data width (SDW_LOG2[1:0] = 0b11 or
DDW_LOG2[1:0] = 0b11) else a user setting is reported and no transfer is issued.
The addressing mode after each data of a LPDMA single is defined by the SINC and DINC
bits of LPDMA_CxTR1(respectively for source and destination): either a fixed addressing or
an incremented addressing with contiguous data.
The start and next addresses of a LPDMA source/destination single (defined by
LPDMA_CxSAR and LPDMA_CxDAR) must be aligned with the respective data width.
The table below lists the main characteristics of a LPDMA single.
The next single address in the table is the next source/destination address, pointed by
LPDMA_CxSAR and LPDMA_CxDAR, once the programmed source/destination single is
completed.
Table 152. Programmed LPDMA source/destination single

Programmed LPDMA
source/destination single

SDW_LOG2[1:0]
DDW_LOG2[1:0]

Data width
(bytes)

Fixed byte single

00

1

Fixed half-word single

01

2

Fixed word single

10

4

Incremented byte single

00

1

Incremented half-word single

01

2

Incremented word single

10

4

Forbidden

11

SINC/
DINC

Addressing
mode

SAR/DAR
next
Address
single
alignment
address
1

0

Fixed

+0

2
4

1

Incremented

+1

1

+2

2

+4

4

Causes USEF generation and none single to be issued.

In direct mode, a LPDMA single is an AHB single transfer.

RM0456 Rev 6

<!-- pagebreak -->

