1259

Secure digital input/output MultiMediaCard interface (SDMMC)

31.5.6

RM0456

SDMMC AHB master interface
The AHB master interface is used to transfer the data between a memory and the FIFO
using the SDMMC IDMA.

SDMMC IDMA
Direct memory access (DMA) is used to provide high-speed transfer between the SDMMC
FIFO and the memory. The AHB master optimizes the bandwidth of the system bus. The
SDMMC internal DMA (IDMA) provides one channel to be used either for transmit or
receive.
The IDMA is enabled by the IDMAEN bit and supports burst transfers of 8 beats.
•

In transmit burst transfer mode:
–

•

Data are fetched in burst from memory whenever the FIFO is empty for the
number of burst transfers, until all data according DATALENGTH has been
transfered. When the DATALENGTH is not an integer multiple of the burst size the
remaining, smaller then burst size data is transfered using single transfer mode.
When the DATALENGTH is not an integer multiple of 4, the last remaining data (1,
2 or 3 bytes) are fetched with a word transfer.

In receive burst transfer mode:
–

Data are stored in burst in to memory whenever the FIFO contains the number of
burst transfers, until all data according DATALENGTH has been transfered.
When the DATALENGTH is not an integer multiple of the burst transfer the
remaining, smaller then burst size data, is transfered using single transfer mode.
When the DATALENGTH is not an integer multiple of 4, the last remaining data (1,
2 or 3 bytes) are stored with halfword and or byte transfers.

In addition the IDMA provides the following channel configurations selected by bit
IDMABMODE:
•

single buffered channel

•

linked list channel

Single buffered channel
In single buffer configuration the data at the memory side is accessed in a linear matter
starting from the base address IDMABASE. When the IDMA has finished transferring all
data the and the DPSM has completed the transfer the DATAEND flag is set.

Linked list channel
In linked list configuration, IDMAMODE = 1, the data at the memory side is subsequently
accessed from linked buffers, located at base address IDMABASE. The size of the memory
buffers is defined by IDMABSIZE. The buffer size must be an integer multiple of the burst
size. The bit ULA is used to indicate if a new linked list buffer configuration has to be loaded
from the linked list table. A new linked list configuration is loaded when the ULA bit for the
current linked list item is set.
The first linked list item configuration is programmed by firmware directly in the SDMMC
registers.
When the IDMA has finished transferring all the data of one linked list buffer, according
IDMABSIZE, and when the linked list item ULA bit is set, the IDMA loads the new linked list
item from the linked list table, and continues transferring data from the next linked list buffer.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Secure digital input/output MultiMediaCard interface (SDMMC)
When the IDMA has finished transferring all data, according IDMABSIZE and ULA, and the
DPSM has completed the transfer, according DATALENGHT, the DATAEND flag is set.
In the following cases, the linked list provides more buffer space than the data to transfer
which means the current linked list buffer data has not completely be transfered:
•

the ULA bit is set, and all SDMMC data according DATALENGTH has been transfered
(DATAEND flag)

•

a transfer error (DCRCFAIL when DATACOUNT > 0, RXOVERR, TXUNDERR) occurs

•

a transfer is hold (DTHOLD)

In all above cases, the IDMA linked list is stopped and the FIFO is flushed/reset. Before
starting or restarting a new SDMMC transfer, the software must initialize a new linked list
with correct IDMABASE and IDMABSIZE.
When a IDMA transfer error occurs (see Section : IDMA transfer error management) or
when the linked list does not provide sufficient buffer space:
•

the linked list ends with ULA = 0 and all last linked list buffer data has been transfered,
and not all SDMMC data according DATALENGTH has been transfered. The SDMMC
transfer is stopped and an IDMA transfer error is generated (see Section : IDMA
transfer error management).

For a given linked list item, the base address is given by the linked list base IDMABA
register value plus the linked list offset IDMALA register value.
The content of each linked list item can be specified by the ULS bit, which makes possible to
optionally load the IDMABSIZE, resulting in a 3-word linked list structure. When the
IDMABSIZE is not to be loaded (fixed size buffers) a compacted reduced 2-word linked list
structure can be used containing only the IDMABASER and the IDMALAR values.

Registers
Linked
list 0

Linked
list 0

Linked
list 1

IDMABASE

D
M
A
AL
IDMABASE
IDMABSIZE

D

A
AL

I
A+

M

AB

ID

Linked
list last

ULA=0, ULS=0, ABR=1, IDMALA
IDMABASE

A
AL

M

IDMABASE
IDMABSIZE

M

IDMABASE

ID

A+
AB

A

AL

Linked
list last

I
A+
AB

A
AL

M

M
ULA=0, ULS=1, ABR=1, IDMALA

Linked list 2

A
AL

M

ID

M

ID

ID

A+

IDMABSIZE

ULA=0, ULS=0, ABR=1, IDMALA

M

A
AL
IDMABASE

D

IDMABASE

IDMABASE

D

M
I
A+
AB

AB

M

ID

Linked
list 1

IDMABASE

D
M

A

ID

AL

M

D

+I

ULA=1, ULS=1, ABR=1, IDMALA

ULA=1, ULS=0, ABR=1, IDMALA

A
I
A+
AB

I
A+
AB

A
AB

M

IDMABSIZE

ULA=1, ULS=1, ABR=1, IDMALA

AL

M

ID

M

ID

ID

IDMABASE

ULA=1, ULS=0, ABR=1, IDMALA

M

A

AL

M

A

AL

M
ULA=1, ULS=1, ABR=1, IDMALA

IDMABASE
IDMABSIZE

ID

A+

IDMABSIZE

ULA=1, ULS=0, ABR=X, IDMALA

Linked
list last

ULA=1, ULS=0, ABR=1, IDMALA

AB

ULA=1, ULS=0, ABR=1, IDMALA

M

ULA=1, ULS=0, ABR=1, IDMALA

IDMABASE

ID

A+

IDMABSIZE

ULA=1, ULS=0, ABR=X, IDMALA

ID

AB

M

ID

IDMABASE

Registers

ULA=1, ULS=1, ABR=X, IDMALA

Linked
list 0

Mixed size buffers
linked list structure

Linked
list 1

Fixed size buffers
Compacted linked list structure

Linked
list 2

Variable size buffers
Full linked list structure

ID

A+

AB

M

ID

Registers

Figure 201. Linked list structures

IDMABASE
MSv47491V2

There is no restriction on mixing both linked list item structures in a single list, this enables
the IDMABSIZE to be updated only when needed.

RM0456 Rev 6

<!-- pagebreak -->

1259

Secure digital input/output MultiMediaCard interface (SDMMC)

RM0456

Whenever a linked list buffer has been transfered and the current buffer ULA = 1, an end-oflinked-list-buffer-transfer-complete interrupt (IDMABTC) may be generated (if interrupt is
enabled).

Linked list acknowledgment
In the case where software dynamically updates the linked list, during the SDMMC transfer,
the availability of a new linked list buffer can be acknowledged by the acknowledge buffer
ready (ABR) bit.
When ABR acknowledges that the new linked list buffer is ready, the IDMA continues
transferring data from the new linked list buffer.
When ABR indicates that the new linked list buffer is not ready, an IDMA transfer error is
generated (see Section : IDMA transfer error management). Depending when the IDMA
transfer error occurs, it normally causes the generation of an TXUNDERR or RXOVERR
error. When a linked list buffer is not acknowledged in time the SDMMC transfer is stopped.
The ABR information is “don't care” when starting the linked list from software programmed
register information. The first linked list buffer must be ready to be used before starting the
SDMMC transfer.

IDMA transfer error management
An IDMA transfer error can occur:
•

When reading or writing a reserved address space (for data or linked list information).

•

When there is no more linked list buffer space to store received SDMMC data.

•

When all linked list buffer data has been transfered and still more SDMMC data needs
to be sent.

•

When the availability of a linked list buffer is not acknowledged.

On an IDMA transfer error subsequent IDMA transfers are disabled and an IDMATE flag is
set and hardware flow control is disabled. Depending when the IDMA transfer error occurs,
it normally causes the generation of a TXUNDERR or RXOVERR error.
The behavior of the IDMATE flag depend on when the IDMA transfer error occurs during the
SDMMC transfer:
•

•

•

An IDMA transfer error is detected before any SDMMC transfer error (TXUNDERR,
RXOVERR, DCRCFAIL, or DTIMEOUT):
–

The IDMATE flag is set at the same time as the SDMMC transfer error flag.

–

The TXUNDERR, RXOVERR, DCRCFAIL, or DTIMEOUT interrupt is generated.

An IDMA transfer error is detected during a STOP_TRANSNMISSION command:
–

The IDMATE flag is set at the same time as the DABORT flag.

–

The DABORT interrupt is generated.

An IDMA transfer error is detected at the end of the SDMMC transfer (DHOLD, or
DATAEND).
–

The IDMATE flag is set at the end of the SDMMC transfer.

–

A SDMMC transfer end interrupt is generated and a DHOLD or DATAEND flag is
set.

The IDMATE is generated on an other SDMMC transfer interrupt (TXUNDERR. RXOVERR,
DCRCFAIL, DTIMEOUT, DABORT, DHOLD, or DATAEND).

<!-- pagebreak -->

