763

General purpose direct memory access controller (GPDMA)

RM0456

The table below lists the possible data handling from the source to the destination.
Table 141. Programmed data handling
SDW_
LOG2
[1:0]

Source
data

Source
data
stream(1)

SB
X

DDW_
LOG2
[1:0]

Destination
data

PAM[1:0](2)

DB
X

00

Byte

xx

x

B7,B6,B5,B4,B3,B2,B1,B0

0

0B3,0B2,0B1,0B0

1

B30,B20,B10,B00

00 (RA, 0P)

01

Half-word

01 (RA, SE)

1x (PACK)

0

00

Byte

00 (RA, 0P)

0

B7B6,B5B4,B3B2,B1B0

1

B6B7,B4B5,B2B3,B0B1

1
0

0
10

Word

01 (RA, SE)

1
0
1
0

1x (PACK)

1
0
1

01

<!-- pagebreak -->

Halfword

B7B6,B5B4
,B3B2,
B1B0

0

1

0

1

0

1

00 (RA, LT)
00

Byte

01 (LA, RT)
1x (UNPACK)

RM0456 Rev 6

SB3,SB2,SB1,SB0
B3S,B2S,B1S,B0S

1

x

x

Destination data
stream(1)

1

0
B7,B6,B5,
B4,B3,B2,
B1,B0

DH
X

000B1,000B0
00B10,00B00
0B100,0B000
B1000,B0000
SSSB1,SSSB0
SSB1S,SSB0S
SB1SS,SB0SS
B1SSS,B0SSS
B7B6B5B4,B3B2B1B0
B6B7B4B5,B2B3B0B1
B5B4B7B6,B1B0B3B2
B4B5B6B7,B0B1B2B3
B6,B4,B2,B0

x

x

B7,B5,B3,B1
B7,B6,B5,B4,B3,B2,B1,B0

RM0456

General purpose direct memory access controller (GPDMA)
Table 141. Programmed data handling (continued)

SDW_
LOG2
[1:0]

Source
data

Source
data
stream(1)

SB
X

DDW_
LOG2
[1:0]

Destination
data

PAM[1:0](2)

01

Half-word

xx

DB
X
0
1
0

00 (RA, 0P)

1
0
1

01

Halfword

B7B6,B5B4
,B3B2,
B1B0

0

x
10

Word

01 (RA, SE)

1
0
1
0

1x (PACK)

1
0
1

DH
X

x

0

1

0

1

0

1

00 (RA, LT)
00

Byte

01 (LA, RT)

10

Word

00 (RA, LT)

0
01

Half-word

01 (LA, RT)

1x (UNPACK)

RM0456 Rev 6

B7B6,B5B4,B3B2,B1B0
B6B7,B4B5,B2B3,B0B1
00B3B2,00B1B0
00B2B3,00B0B1
B3B200,B1B000
B2B300,B0B100
SSB3B2,SSB1B0
SSB2B3,SSB0B1
B3B2SS,B1B0SS
B2B3SS,B0B1SS
B7B6B5B4,B3B2B1B0
B6B7B4B5,B2B3B0B1
B5B4B7B6,B1B0B3B2
B4B5B6B7,B0B1B2B3
B12,B8,B4,B0

x

B15,B11,B7,B3

10 (UNPACK)
B7B6B5B4,
B3B2B1B0

Destination data
stream(1)

B7,B6,B5,B4,B3,B2,B1,B0
0
1

B5B4,B1B0
x

B4B5,B0B1

0

B7B6,B3B2

1

B6B7,B2B3

0

B7B6,B5B4,B3B2,B1B0

1

B6B7,B4B5,B2B3,B0B1

<!-- pagebreak -->

763

General purpose direct memory access controller (GPDMA)

RM0456

Table 141. Programmed data handling (continued)
SDW_
LOG2
[1:0]

Source
data
stream(1)

Source
data

SB
X

DDW_
LOG2
[1:0]

Destination
data

PAM[1:0](2)

DB
X
0

0

10

Word

xx

1
0
1

DH
X

0

1

00 (RA, LT)
00

Byte

01 (LA, RT)

10

Word

00 (RA, LT)

1

01

Half-word

01 (LA, RT)

1x (UNPACK)

x

Word

xx

B6B7B4B5,B2B3B0B1
B5B4B7B6,B1B0B3B2
B4B5B6B7,B0B1B2B3
B15,B11,B7,B3
B7,B5,B6,B4,B3,B1,B2,B0

0
1

B6B4,B2B0
x

B4B6,B0B2

0

B7B5,B3B1

1

B5B7,B1B3

0

B7B5,B6B4,B3B1,B2B0

1

B5B7,B4B6,B1B3,B0B2

0
10

B7B6B5B4,B3B2B1B0

B12,B8,B4,B0

1x (UNPACK)
B7B6B5B4,
B3B2B1B0

Destination data
stream(1)

1
0
1

0

1

B7B5B6B4,B3B1B2B0
B5B7B4B6,B1B3B0B2
B6B4B7B5,B2B0B3B1
B4B6B5B7,B0B2B1B3

1. Data stream is timely ordered starting from the byte with the lowest index (B0).
2. RA= right aligned, LA = left aligned, RT = right truncated, LT = left truncated, 0P = zero bit padding up to the destination
data width, SE = sign bit extended up to the destination data width.

17.4.11

GPDMA transfer request and arbitration
GPDMA transfer request
As defined by GPDMA_CxTR2, a programmed GPDMA data transfer is requested with one
of the following:

<!-- pagebreak -->

•

a software request if the control bit SWREQ = 1: This is used typically by the CPU for a
data transfer from a memory-mapped address to another memory mapped address
(memory-to-memory, GPIO to/from memory)

•

an input hardware request coming from a peripheral if SWREQ = 0: The selection of
the GPDMA hardware peripheral request is driven by the REQSEL[6:0] field
(see Section 17.3.3). The selected hardware request can be one of the following:
–

an hardware request from a peripheral configured in GPDMA mode (for a transfer
from/to the peripheral data register respectively to/from the memory)

–

an hardware request from a peripheral for its control registers update from the
memory

–

an hardware request from a peripheral for a read of its status registers transferred
to the memory

RM0456 Rev 6

RM0456
Caution:

General purpose direct memory access controller (GPDMA)
The user must not assign a same input hardware peripheral GPDMA request via
GPDMA_CxTR.REQSEL[6:0] to two different channels, if at a given time this request is
asserted by the peripheral and each channel is ready to execute this requested data
transfer. There is no user setting error reporting.

GPDMA transfer request for arbitration
A ready FIFO-based GPDMA source single/burst transfer (from the source address to the
FIFO) to be scheduled over the allocated master port (GPDMA_CxTR1.SAP) is arbitrated
based on the channel priority (GPDMA_CxCR.PRIO[1:0]) versus the other simultaneous
requested GPDMA transfers to the same master port.
A ready FIFO-based GPDMA destination single/burst transfer (from the FIFO to the
destination address) to be scheduled over the allocated master port (GPDMA_CxTR1.DAP)
is arbitrated based on the channel priority (GPDMA_CxCR.PRIO[1:0]) versus the other
simultaneous requested GPDMA transfers to the same master port.
An arbitrated GPDMA requested link transfer consists of one 32-bit read from the linked-list
data structure in memory to one of the linked-list registers (GPDMA_CxTR1,
GPDMA_CxTR2, GPDMA_CxBR1, GPDMA_CxSAR, GPDMA_CxDAR or GPDMA_CxLLR,
plus GPDMA_CxTR3, GPDMA_CxBR2). Each 32-bit read from memory is arbitrated with
the same channel priority as for data transfers, in order to be scheduled over the allocated
master port (GPDMA_CxCR.LAP).
Whatever the requested data transfer is programmed with a software request for a memoryto- memory transfer (GPDMA_CxTR2.SWREQ = 1), or with a hardware request
(GPDMA_CxTR2.SWREQ = 0) for a memory-to-peripheral transfer or a peripheral-tomemory transfer and whatever is the hardware request type, re-arbitration occurs after each
granted single/burst transfer.
When an hardware request is programmed from a destination peripheral
(GPDMA_CxTR2.SWREQ = 0 and GPDMA_CxTR2.DREQ = 1), the first memory read of a
(possibly 2D/repeated) block (the first ready FIFO-based source burst request), is gated by
the occurrence of the corresponding and selected hardware request. This first read request
to memory is not taken into account earlier by the arbiter (not as soon as the block transfer
is enabled and executable).

GPDMA arbitration
The GPDMA arbitration is directed from the 4-grade assigned channel priority
(GPDMA_CxCR.PRIO[1:0]). The arbitration policy, as illustrated in Figure 68, is defined by:
•

one high-priority traffic class (queue 3), dedicated to the assigned channels with
priority 3, for time-sensitive channels
This traffic class is granted via a fixed-priority arbitration against any other low-priority
traffic class. Within this class, requested single/burst transfers are round-robin
arbitrated.

•

three low-priority traffic classes (queues 0, 1, or 2) for non time-sensitive channels with
priority 0, 1, or 2
Each requested single/burst transfer within this class is round-robin arbitrated, with a
weight that is monotonically driven from the programmed priority:
–

Requests with priority 0 are allocated to the queue 0.

–

Requests with priority 1 are allocated and replicated to the queue 0 and queue 1.

RM0456 Rev 6

<!-- pagebreak -->

763

General purpose direct memory access controller (GPDMA)

RM0456

–

Requests with priority 2 are allocated and replicated to the queue 0, queue 1, and
queue 2.

–

Any queue 0, 1, or 2 equally grants any of its active input requests in a round-robin
manner, provided there are simultaneous requests.

–

Additionally, there is a second stage for the low-traffic with a round-robin arbiter
that fairly alternates between simultaneous selected requests from queue 0,
queue 1, and queue 2.
Figure 68. GPDMA arbitration policy

Request from any channel x
being assigned with GPDMA_CxCR.PRIO = 0

Queue 0
RRA

Request from any channel x
being assigned with GPDMA_CxCR.PRIO = 1

Queue 1
RRA

Request from any channel x
being assigned with GPDMA_CxCR.PRIO = 2

Queue 2
RRA

Request from any channel x
being assigned with GPDMA_CxCR.PRIO = 3

Queue 3
RRA

RRA = round-robin arbitration, FPA = fixed-priority arbitration

Low
RRA

FPA

Granted
request

High

GPDMA arbitration
MSv63647V1

GPDMA arbitration and bandwidth
With this arbitration policy, the following is guaranteed:
•

Equal maximum bandwidth between requests with same priority

•

Reserved bandwidth (noted as BQ3) to the time-sensitive requests (with priority 3)

•

Residual weighted bandwidth between different low-priority requests (priority 0 versus
priority 1 versus priority 2).

The two following examples highlight that the weighted round-robin arbitration is driven by
the programmed priorities:
•

Example 1: basic application with two non time-sensitive GPDMA requests: req0 and
req1. There are the following programming possibilities:
–

If they are assigned with same priority, the allocated bandwidth by the arbiter to
req0 (Breq0) is equal to the allocated bandwidth to req1(Breq1).
Breq0 = Breq1 = 1/2 * (1 - BQ3)

–

If req0 is assigned to priority 0 and req1 to priority 1, the allocated bandwidth to
req0 (BP0) is 3 times less than the allocated bandwidth to req1 (BP1).
Breq0 = BP0 = 1/2 * 1/2 * (1 - BQ3) = 1/4 * (1 - BQ3)
Breq1 = BP1 = (1/2 + 1) * 1/2 * (1 - BQ3) = 3/4 * (1 - BQ3)

–

If req0 is assigned to priority 0 and req1 to priority 2, the allocated bandwidth to
req0 (BP0) is 5 times less than the allocated bandwidth to req1 (BP2).
Breq0 = BP0 = 1/2 * 1/3 * (1 - BQ3) = 1/6 * (1 - BQ3)
Breq1 = BP2 = (1/2 + 1 +1) * 1/3 * (1 - BQ3) = 5/6 * (1 - BQ3)

The above computed bandwidth calculation is based on a theoretical input request,
always active for any GPDMA clock cycle. This computed bandwidth from the arbiter
must be weighted by the frequency of the request given by the application, that cannot
be always active and may be quite much variable from one GPDMA client (example
I2C at 400 kHz) to another one (PWM at 1 kHz) than the above x3 and x5 ratios.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

General purpose direct memory access controller (GPDMA)
•

Example 2: application where the user distributes a same non-null N number of
GPDMA requests to every non time-sensitive priority 0, 1 and 2. The bandwidth
calculation is then the following:
–

The allocated bandwidth to the set of requests of priority 0 (BP0) is
BP0 = 1/3 * 1/3 * (1 - BQ3) = 1/9 * (1 - BQ3)

–

The allocated bandwidth to the set of requests of priority 1(BP1) is
BP1 = (1/3 + 1/2) * 1/3 * (1 - BQ3) = 5/18 * (1 - BQ3)

–

The allocated bandwidth to the set of requests of priority 2(BP2) is
BP2 = (1/3 + 1/2 + 1) * 1/3 * (1 - BQ3) = 11/18 * (1 - BQ3)

–

The allocated bandwidth to any request n (Bn) among the N requests of that
priority Pi (i = 0 to 2) is Bn = 1/N * BPi

–

The allocated bandwidth to any request n of priority 0i (Bn, Pi) is
Bn, P0 = 1/N *1/9 * (1 - BQ3)
Bn, P1 = 1/N *5/18 * (1 - BQ3)
Bn, P2 = 1/N *11/18 * (1 - BQ3)

In this example, when the master port bus bandwidth is not totally consumed by the
time-sensitive queue 3, the residual bandwidth is such that 2.5 times less bandwidth is
allocated to any request of priority 0 versus priority 1, and 5.5 times less bandwidth is
allocated to any request of priority 0 versus priority 2.
More generally, assume that the following requests are present:
•

I requests (I ≥ 0) assigned to priority 0
If I > 0, these requests are noted from i = 0 to I-1.

•

J requests (J ≥ 0) assigned to priority 1
If J > 0, these requests are noted from j = 0 to J-1.

•

K requests (K > 0) assigned to priority 2
These requests are noted from k = 0 to K-1

•

L requests (L ≥ 0) assigned to priority 3
If L > 0, these requests are noted from l = 0 to L-1.

As BQ3 is the reserved bandwidth to time-sensitive requests, the bandwidth for each
request L with priority 3 is:
•

Bl = BQ3 / L for L > 0 (else: Bl = 0)

The bandwidth for each non-time sensitive queue is:
•

BQ0 = 1/3 * (1 - BQ3)

•

BQ1 = 1/3 * (1 - BQ3)

•

BQ2 = 1/3 * (1 - BQ3)

The bandwidth for the set of requests with priority 0 is:
•

BP0 = I / (I + J + K) * BQ0

The bandwidth for each request i with priority 0 is:
•

Bi = BP0 / I for L > 0 (else BP0 = 0)

The bandwidth for the set of requests with priority 1 and routed to queue 0 is:
•

BP1,Q0 = J / (I + J + K) * BQ0

RM0456 Rev 6

<!-- pagebreak -->

763

General purpose direct memory access controller (GPDMA)

RM0456

The bandwidth for the set of requests with priority 1 and routed to queue 1 is:
•

BP1,Q1 = J / (J + K) * BQ1

The total bandwidth for the set of requests with priority 1 is:
•

BP1 = BP1,Q0 + BP1,Q1

The bandwidth for each request j with priority 1 is:
•

Bj = BP1 / J for J > 0 (else Bj = 0)

The bandwidth for the set of requests with priority 2 and routed to queue 0 is:
•

BP2,Q0 = K / (I + J + K) * BQ0

The bandwidth for the set of requests with priority 2 and routed to queue 1 is:
•

BP2,Q1 = K / (J + K) * BQ1

The bandwidth for the set of requests with priority 2 and routed to queue 2 is:
•

BP2,Q2 = BQ2

The total bandwidth for the set of requests with priority 2 is:
•

BP2 = BP2,Q0 + BP2,Q1+ BP2,Q2

The bandwidth for each request k with priority 2 is:
•

Bk = BP2 / K (K>0 in the general case)

Thus finally the maximum allocated residual bandwidths for any i, j, k non-time sensitive
request are:
•

•

in the general case (when there is at least one request k with a priority 2 (K > 0)):
–

Bi = 1/I * 1/3 * I/(I + J + K) * (1 - BQ3)

–

Bj = 1/J * 1/3 *[J/(I + J + K) + J/(J + K)] * (1 - BQ3)

–

Bk = 1/K * 1/3 *[K/(I + J + K) + K/(J + K) + 1] * (1 - BQ3)

in the specific case (when there is no request k with a priority 2 (K = 0)):
–

Bi = 1/I * 1/2 * I/(I + J) * (1 - BQ3)

–

Bj = 1/J * 1/2 *[J/(I + J) + 1] * (1 - BQ3)

Consequently, the GPDMA arbiter can be used as a programmable weighted bandwidth
limiter, for each queue and more generally for each request/channel. The different weights
are monotonically resulting from the programmed channel priorities.

17.4.12

GPDMA triggered transfer
A programmed GPDMA transfer can be triggered by a rising/falling edge of a selected input
trigger event, as defined by GPDMA_CxTR2.TRIGPOL[1:0] and
GPDMA_CxTR2.TRIGSEL[6:0] (see Section 17.3.5 for the trigger selection).
The triggered transfer, as defined by the trigger mode in GPDMA_CxTR2.TRIGM[1:0], can
be at LLI data transfer level, to condition the first burst read of a block, the first burst read of
a 2D/repeated block for channel x (x = 12 to 15), or each programmed single read. The
trigger mode can also be programmed to condition the LLI link transfer (see TRIGM[1:0] in
GPDMA_CxTR2 for more details).

<!-- pagebreak -->

RM0456 Rev 6

RM0456

General purpose direct memory access controller (GPDMA)

Trigger hit memorization and trigger overrun flag generation
The GPDMA monitoring of a trigger for a channel x is started when the channel is
enabled/loaded with a new active trigger configuration: rising or falling edge on a selected
trigger (respectively TRIGPOL[1:0] = 01 or TRIGPOL[1:0] = 10).
The monitoring of this trigger is kept active during the triggered and uncompleted (data or
link) transfer. If a new trigger is detected, this hit is internally memorized to grant the next
transfer, as long as the defined rising/falling edge and TRIGSEL[6:0] are not modified, and
the channel is enabled.
Transferring a next LLIn+1, that updates the GPDMA_CxTR2 with a new value for any of
TRIGSEL[6:0] or TRIGPOL[1:0], resets the monitoring, trashing the possible memorized hit
of the formerly defined LLIn trigger.
Caution:

After a first new trigger hitn+1 is memorized, if another trigger hitn+2 is detected and if the hitn
triggered transfer is still not completed, hitn+2 is lost and not memorized. A trigger overrun
flag is reported (GPDMA_CxSR.TOF = 1) and an interrupt is generated if enabled
(if GPDMA_CxCR.TOIE = 1). The channel is not automatically disabled by hardware due to
a trigger overrun.
Figure 69 illustrates the trigger hit, memorization and overrun in the configuration example
with a block-level trigger mode and a rising edge trigger polarity.
Figure 69. Trigger hit, memorization, and overrun waveform

Channel state

IDLE

ACTIVE

Trigger

Peripheral
request
block transfer

DMA transfer
Trigger monitoring
state

Idle

Active (monitoring)

Trigger monitoring
action

Active

Hit and
fire

block transfer

Active

Active

Hit and
Fire Hit and
memorize
memorize

block transfer

Active

Active

Hit and
trash

Active

Fire

Trigger overrun
Hit and trash

Hit and fire (or fire alone)

Hit and memorize

MSv66923V1

Note:

The user can assign the same input trigger event to different channels. This can be used to
trigger different channels on a broadcast trigger event.

17.4.13

GPDMA circular buffering with linked-list programming
GPDMA circular buffering for memory-to-peripheral and
peripheral-to-memory transfers, with a linear addressing channel
For a circular buffering, with a continuous memory-to-peripheral (or peripheral-to-memory)
transfer, the software must set up a channel with half transfer and complete transfer

RM0456 Rev 6

<!-- pagebreak -->

763

General purpose direct memory access controller (GPDMA)

RM0456

events/interrupts generation (GPDMA_CxCR.HTIE = 1 and GPDMA_CxCR.TCIE = 1), in
order to enable a concurrent buffer software processing.
LLI0 is configured for the first block transfer with the linear addressing channel. A
continuously-executed LLI1 is needed to restore the memory source (or destination) start
address, for the memory-to-peripheral transfer (respectively the peripheral-to-memory
transfer). GPDMA automatically reloads the initially programmed
GPDMA_CxBR1.BNDT[15:0] when a block transfer is completed, and there is no need to
restore GPDMA_CxBR1.
Figure 70 illustrates this programming with a linear addressing GPDMA channel and a
source circular buffer.
Figure 70. GPDMA circular buffer programming: update of the memory start address
with a linear addressing channel
Req=PERIPH_TX

Req=PERIPH_TX

Reset
Restore
SAR/LLI1

Init/LLI0
Channel x
Ht+ tcf

Ht+ tcf

Linked-list register file
LLI0
DMA_CxTR1
DMA_CxTR2
DMA_CxBR1

CxLBA (LA = 0)
USA = 1
others Uxx = 0

DMA_CxSAR

Memory
LLI1
DMA_CxSAR

DMA_CxDAR
DMA_CxLLR
MSv62640V1

Note:

With a 2D addressing channel, the user may use a single LLI with
GPDMA_CxBR1.BRC[10:0] = 1, and program a negative memory block address offset with
GDMA_CxBR2 and GDMA_CxBR1, in order to jump back to the memory source or the
destination start address.
If circular buffering must be executed after some other transfers over the shared GPDMA
channel x, the before-last LLIN-1 in memory is needed to configure the first block transfer.
And the last LLIN restores the memory source (or destination) start address in
memory-to-peripheral transfer (respectively in peripheral-to-memory transfer).
Figure 71 illustrates this programming with a linear addressing shared GPDMA channel,
and a source circular buffer.

<!-- pagebreak -->

