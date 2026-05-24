RM0456 Rev 6

RM0456

17.4.10

General purpose direct memory access controller (GPDMA)

GPDMA FIFO-based transfers
There is a single transfer operation mode: the FIFO mode. There are FIFO-based transfers.
Any channel x is implemented with a dedicated FIFO whose size is defined by
dma_fifo_size[x] (see Section 17.3.1 for more details).

GPDMA burst
A programmed transfer at the lowest level is a GPDMA burst.
A GPDMA burst is a burst of data received from the source, or a burst of data sent to the
destination. A source (and destination) burst is programmed with a burst length by the field
SBL_1[5:0] (respectively DBL_1[5:0]), and with a data width defined by the field
SDW_LOG2[1:0] (respectively DDW_LOG2[1:0]) in the GPDMA_CxTR1 register.
The addressing mode after each data (named beat) of a GPDMA burst is defined by SINC
and DINC in GPDMA_CxTR1, for source and destination respectively: either a fixed
addressing or an incremented addressing with contiguous data.
The start and next addresses of a GPDMA source/destination burst (defined by
GPDMA_CxSAR and GPDMA_CxDAR) must be aligned with the respective data width.
The table below lists the main characteristics of a GPDMA burst.
Table 140. Programmed GPDMA source/destination burst
SDW_LOG2[1:0]
DDW_LOG2[1:0]

Data
width
(bytes)

00

1

01

2

10

4

00

1

01

2

10

4

11

SINC/DINC

SBL_1[5:0]
DBL_1[5:0]

Burst
Next data/
length
beat
(data/beats) address

Next burst
address

Burst
address
alignment
1

+0

0 (fixed)
n = 0 to 63(1)

n+1

1
(contiguously
incremented)

+0

2
4

+1

+ (n + 1)

1

+2

+ 2 * (n + 1)

2

+4

+ 4 * (n + 1)

4

forbidden user setting, causing USEF generation and none burst to be issued.

1. When S/DBL_1[5:0] = 0, burst is of length 1. Then burst can be also named as single.

The next burst address in the above table is the next source/destination default address
pointed by GPDMA_CxSAR or GPDMA_CxDAR, once the programmed source/destination
burst is completed. This default value refers to the fixed/contiguously incremented address.

RM0456 Rev 6

<!-- pagebreak -->

763

General purpose direct memory access controller (GPDMA)

RM0456

GPDMA burst with 2D addressing (channel x = 12 to 15)
When the channel has additional 2D addressing feature, this default value refers to the
value without taking into account the two programmed incremented or decremented offsets.
These two additional offsets (with a null default value) are applied:
•

after each completed source/destination burst, as defined respectively by
GPDMA_CxTR2.SAO[12:0]/DAO[12:0] and GPDMA_CxBR1.SDEC/DDEC

•

after each completed block, as defined respectively by
GPDMA_CxBR2.BRSAO[15:0]/BRDAO[15:0] and
GPDMA_CxBR1.BRSDEC/BRDDEC)

Then, a 2D/repeated block can be addressed with a first programmed address jump after
each completed burst, and with a second programmed address jump after each block, as
depicted by Figure 67 with a 2D destination buffer.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

General purpose direct memory access controller (GPDMA)
Figure 67. Programmed 2D addressing
Memory
32b
Cx_DAR

Data0
Data1
...
DataI-1

Burst0

Data0
Data1
...
DataI-1

Burst1

+ DAO

+ DAO

Block0

+ DAO

Memory-mapped
Peripheral

Data0
Data1
...
DataI-1

BurstJ-1

...

Burst0

...

Burst1

+ DAO
+ BRDAO

32b
Cx_SAR

Data Register
(fixed addressing,
SINC=0)

+ DAO

Restore Cx_DAR

+ DAO

Blockk

2D/repeated block
LLIL

+ DAO
...

BurstJ-1

Data0
Data1
...
DataI-1

Burst0

Data0
Data1
...
DataI-1

Burst1

+ DAO
+ BRDAO

+ DAO

BlockK-1

+ DAO

+ DAO

Programmable address jumps 1) after burst and 2) after
+ DAO
block.
Example:
+ BRDAO
burst: I * words (DBL_1=I-1; DDW_LOG2=’b10)
block: J * bursts (BNDT=J*I*4)
LLI: K * blocks (BRC=K-1)

Data0
Data1
...
DataI-1

BurstJ-1

MSv63674V1

RM0456 Rev 6

<!-- pagebreak -->

763

General purpose direct memory access controller (GPDMA)

RM0456

GPDMA FIFO-based burst
In FIFO-mode, a transfer generally consists of two pipelined and separated burst transfers:
•

one burst from the source to the FIFO over the allocated source master port, as defined
by GPDMA_CxTR1.SAP

•

one burst from the FIFO to the destination over the allocated destination master port,
as defined by GPDMA_CxTR1.DAP

GPDMA source burst
The requested source burst transfer to the FIFO can be scheduled as early as possible over
the allocated port, depending on the current FIFO level versus the programmed burst size
(when the FIFO is ready to get one new burst from the source):
when FIFO level ≤ 2dma_fifo_size[x] - (SBL_1[5:0]+1) * 2SDW_LOG2[1:0]
where:
•

FIFO level is the current filling level of the FIFO, in bytes.

•

2dma_fifo_size[x] is the half of the FIFO size of the channel x, in bytes (see Section 17.3.1
for the implementation details and dma_fifo_size[x] value).

•

(SBL_1[5:0]+1) * 2SDW_LOG2[1:0] is the size of the programmed source burst transfer,
in bytes.

Based on the channel priority (GPDMA_CxCR.PRIO[1:0]), this ready FIFO-based source
transfer is internally arbitrated versus the other requested and active channels.

GPDMA destination burst
The requested destination burst transfer from the FIFO can be scheduled as early as
possible over the allocated port, depending on the current FIFO level versus the
programmed burst size (when the FIFO is ready to push one new burst to the destination):
when FIFO level ≥ (DBL_1[5:0]+1) * 2DDW_LOG2[1:0]
where:
•

FIFO level is the current filling level of the FIFO, in bytes.

•

(DBL_1[5:0]+1) * 2DDW_LOG2[1:0] is the size of the programmed destination burst
transfer, in bytes.

Based on the channel priority, this ready FIFO-based destination transfer is internally
arbitrated versus the other requested and active channels.

GPDMA burst vs source block size, 1-Kbyte address boundary and FIFO size
The programmed source/destination GPDMA burst is implemented with an AHB burst as is,
unless one of the following conditions is met:

<!-- pagebreak -->

•

When half of the FIFO size of the channel x is lower than the programmed
source/destination burst size, the programmed source/destination GPDMA burst is
implemented with a series of singles or bursts of a lower size, each transfer being of a
size that is lower or equal than half of the FIFO size, without any user constraint.

•

if the source block size (GPDMA_CxBR1.BNDT[15:0]) is not a multiple of the source
burst size but is a multiple of the data width of the source burst
(GPDMA_CxTR1.SDW_LOG2[1:0]), the GPDMA modifies and shortens bursts into
singles or bursts of lower length, in order to transfer exactly the source block size,
without any user constraint.

RM0456 Rev 6

RM0456

General purpose direct memory access controller (GPDMA)
•

if the source/destination burst transfer have crossed the 1-Kbyte address boundary on
a AHB transfer, the GPDMA modifies and shortens the programmed burst into singles
or bursts of lower length, to be compliant with the AHB protocol, without any user
constraint.

•

If the source/destination burst length exceeds 16 on a AHB transfer, the GPDMA
modifies and shortens the programmed burst into singles or bursts of lower length, to
be compliant with the AHB protocol, without any user constraint.

In any case, the GPDMA keeps ensuring source/destination data (and address) integrity
without any user constraint. The current FIFO level (software readable in GPDMA_CxSR) is
compared to and updated with the effective transfer size, and the GPDMA re-arbitrates
between each AHB single or burst transfer, possibly modified.
Based on the channel priority, each single or burst of a lower burst size versus the
programmed burst, is internally arbitrated versus the other requested and active channels.
Note:

In linked-list mode, the GPDMA read transfers related to the update of the linked-list
parameters from the memory to the internal GPDMA registers, are scheduled over the link
allocated port, as programmed by GPDMA_CxCR.LAP.

GPDMA data handling: byte-based reordering, packing/unpacking,
padding/truncation, sign extension and left/right alignment
The data handling is controlled by GPDMA_CxTR1. The source/destination data width of
the programmed burst is byte, half-word or word, as per the SDW_LOG2[1:0] and
DDW_LOG2[1:0] fields (see Table 141).
The user can configure the data handling between transferred data from the source and
transfer to the destination. More specifically, programmed data handling is orderly
performed with:
1.

Byte-based source reordering
–

2.

3.

Note:

If SBX = 1 and if source data width is a word, the two bytes of the unaligned
half-word at the middle of each source data word are exchanged.

Data width conversion by packing, unpacking, padding or truncation, if destination data
width is different than the source data width, depending on PAM[1:0]:
–

If destination data width > source data width, the post SBX source data is either
right-aligned and padded with 0 s, or sign extended up to the destination data
width, or is FIFO queued and packed up to the destination data width.

–

If destination data width < source data width, the post SBX data is either
right-aligned and left-truncated down to the destination data width, or is FIFO
queued and unpacked and streamed down to the destination data width.

Byte-based destination re-ordering:
–

If DBX = 1 and if the destination data width is not a byte, the two bytes are
exchanged within the aligned post PAM[1:0] half-words.

–

If DHX = 1 and if the destination data width is neither a byte nor a half-word, the
two aligned half-words are exchanged within the aligned post PAM[1:0] words.

Left-alignment with 0s-padding can be achieved by programming both a right-alignment with
a 0s-padding and a destination byte-based re-ordering.

RM0456 Rev 6

<!-- pagebreak -->

