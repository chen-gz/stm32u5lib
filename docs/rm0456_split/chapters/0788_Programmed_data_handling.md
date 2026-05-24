821

Low-power direct memory access controller (LPDMA)

RM0456

LPDMA data handling: byte-based padding/truncation, sign extension and
left/right alignment
The user can configure some data manipulation between a transferred data from the source
and its transfer to the destination. Data handling is controlled by the LPDMA_CxTR1
register:
•

If destination data width = source data width (DDW_LOG2[1:0] = SDW_LOG2[1:0]), the
source data is copied as is and transferred to the destination.

•

Else, depending on PAM:
–

If destination data width > source data width, the source data can be either rightaligned and padded with 0 s, or sign extended up to the destination data width.

–

If destination data width < source data width, the source data can be either rightaligned and left-truncated down to the destination data width, or left-aligned and
right-truncated down to the destination data width.

There is no data manipulation between two distinct transferred data from the source, before
the generation of the destination transfer.
The table below lists possible data handling from the source to the destination.
Table 153. Programmed data handling
SDW_
LOG2[1:0]

00

01

10

Source
data

Byte

Half-word

Word

Source
data
stream(1)

B7,B6,B5,B4,
B3,B2,B1,B0

B7B6,B5B4,
B3B2,B1B0

B7B6B5B4,
B3B2B1B0

DDW_
LOG2[1:0]

Destination
data

PAM[0]

Destination data
stream(1)

00

Byte

x

B7,B6,B5,B4,B3,B2,B1,B0

01

Half-word

10

Word

00

Byte

01

Half-word

10

Word

00

Byte
Half-word

01

Word

10

0 (RA, 0P)(2)(3)

0B3,0B2,0B1,0B0

1 (RA, SE)(2)(4)

SB3,SB2,SB1,SB0

0 (RA, 0P)

000B1,000B0

1 (RA, SE)

SSSB1,SSSB0

0 (RA, LT)(2)

B6,B4,B2,B0

1 (LA, RT)(2)

B7,B5,B3,B1

xx
0 (RA, 0P)

00B3B2,00B1B0

1 (RA, SE)

SSB3B2,SSB1B0

0 (RA, LT)

B12,B8,B4,B0

1 (LA, RT)

B15,B11,B7,B3

0 (RA, LT)

B5B4,B1B0

1 (LA, RT)

B7B6,B3B2

1. Data stream is timely ordered starting from the byte with the lowest index a.k.a B0.
2. RA = right aligned. LA = left aligned. RT= right truncated. LT = left truncated.
3. 0P = zero bit padding up to the destination data width.
4. SE = sign bit extended up to the destination data width.

<!-- pagebreak -->

RM0456 Rev 6

B7B6,B5B4,B3B2,B1B0

xx

B7B6B5B4,B3B2B1B0

RM0456

18.4.11

Low-power direct memory access controller (LPDMA)

LPDMA transfer request and arbitration
LPDMA transfer request
As defined by LPDMA_CxTR2, a programmed LPDMA data transfer is requested with one
of the following:

Caution:

•

a software request if the control bit SWREQ = 1: This is used typically by the CPU for a
data transfer from a memory-mapped address to another memory mapped address
(memory-to-memory, GPIO to/from memory)

•

an input hardware request coming from a peripheral if SWREQ = 0: The selection of
the LPDMA hardware peripheral request is driven by the REQSEL[4:0] field
(see Section 18.3.4). The selected hardware request can be one of the following:
–

a hardware request from a peripheral configured in LPDMA mode (for a transfer
from/to the peripheral data register respectively to/from the memory)

–

a hardware request from a peripheral for its control registers update from the
memory

–

a hardware request from a peripheral for a read of its status registers transferred
to the memory

The user must not assign a same input hardware peripheral LPDMA request via
LPDMA_CxTR.REQSEL[4:0] to two different channel if at a given time this request is
asserted by the peripheral and each channel is ready to execute this requested data
transfer. There is no user setting error reporting.

LPDMA transfer request for arbitration
For a given channel, a LPDMA requested data transfer from the source address to the
destination address is arbitrated versus simultaneous requested LPDMA transfers from
other channels, in order to be scheduled over the AHB master port. A LPDMA data transfer
is atomic to the LPDMA arbitration: it consists of an AHB read single, immediately followed
by an AHB write single. It is granted by the arbiter once for both AHB transfers, based on the
channel priority defined by LPDMA_CxCR.PRIO[1:0].
An arbitrated LPDMA requested link transfer consists of one 32-bit read from the linked-list
data structure in the memory to one of the linked-list registers (LPDMA_CxTR1,
LPDMA_CxTR2, LPDMA_CxBR1, LPDMA_CxSAR, LPDMA_CxDAR or LPDMA_CxLLR).
Each 32-bit read from the memory is arbitrated with the same channel priority as for data
transfers, in order to be scheduled over the master port.
The re arbitration occurs after each granted single transfer:
•

•

whatever how the requested data transfer is programmed:
–

with a software request for a memory-to-memory transfer
(LPDMA_CxTR2.SWREQ = 1)

–

with a hardware request (LPDMA_CxTR2.SWREQ = 0) for a memory-toperipheral transfer or a peripheral-to-memory transfer

whatever the hardware request type

When the requested data transfer is programmed with a hardware request from a peripheral
(LPDMA_CxTR2.SWREQ = 0), the first memory read of a block is gated by the occurrence
of the corresponding and selected hardware request, whatever the peripheral is source or
destination of the transfer. This first read request to the memory is not taken into account
earlier by the arbiter (not as soon as the block transfer is enabled and executable).

RM0456 Rev 6

<!-- pagebreak -->

821

Low-power direct memory access controller (LPDMA)

RM0456

LPDMA arbitration
The LPDMA arbitration is directed from the 4-grade assigned channel priority
(LPDMA_CxCR.PRIO[1:0]). The arbitration policy, as illustrated in Figure 86, is defined by:
•

one high-priority traffic class (queue 3), dedicated to the assigned channels with
priority 3, for time-sensitive channels
This traffic class is granted via a fixed-priority arbitration against any other low-priority
traffic class. Within this class, requested single transfers are round-robin arbitrated.

•

three low-priority traffic classes (queues 0, 1 or 2) for non time-sensitive channels with
priority 0, 1 or 2
Each requested single transfer within this class is round-robin arbitrated, with a weight
that is monotonically driven from the programmed priority:
–

Requests with priority 0 are allocated to the queue 0.

–

Requests with priority 1 are allocated and replicated to the queue 0 and queue 1.

–

Requests with priority 2 are allocated and replicated to the queue 0, queue 1, and
queue 2.

–

Any queue 0, 1 or 2 equally grants any of its active input requests in a round-robin
manner, provided there are simultaneous requests.

–

Additionally, there is a second stage for the low-traffic with a round-robin arbiter
that fairly alternates between simultaneous selected requests from queue 0,
queue 1 and queue 2.
Figure 86. LPDMA arbitration policy

Request from any channel x
being assigned with LPDMA_CxCR.PRIO = 0

Queue 0
RRA

Request from any channel x
being assigned with LPDMA_CxCR.PRIO = 1

Queue 1
RRA

Request from any channel x
being assigned with LPDMA_CxCR.PRIO = 2

Queue 2
RRA

Request from any channel x
being assigned with LPDMA_CxCR.PRIO = 3

Queue 3
RRA

RRA = round-robin arbitration, FPA = fixed-priority arbitration

Low
RRA

FPA

Granted
request

High

LPDMA arbitration
MSv62639V1

1. RRA: round-robin arbitration
2. FPA: fixed-priority arbitration

LPDMA arbitration and bandwidth
With this arbitration policy, the following is guaranteed:
•

<!-- pagebreak -->

Equal maximum bandwidth between requests with same priority

•

Reserved bandwidth (noted as BQ3) to the time-sensitive requests (with priority 3)

•

Residual weighted bandwidth between different low-priority requests (priority 0 versus
priority 1 versus priority 2).

RM0456 Rev 6

RM0456

Low-power direct memory access controller (LPDMA)
The two following examples highlight that the weighted round-robin arbitration is driven by
the programmed priorities:
•

Example 1: basic application with two non time-sensitive LPDMA requests: req0 and
req1. There are the following programming possibilities:
–

If they are assigned with same priority, the allocated bandwidth by the arbiter to
req0 (Breq0) is equal to the allocated bandwidth to req1(Breq1).

–

If req0 is assigned to priority 0 and req1 to priority 1, the allocated bandwidth to
req0 (BP0) is 3 times less than the allocated bandwidth to req1 (BP1).

Breq0 = Breq1 = 1/2 * (1 - BQ3)

Breq0 = BP0 = 1/2 * 1/2 * (1 - BQ3) = 1/4 * (1 - BQ3)
Breq1 = BP1 = (1/2 + 1) * 1/2 * (1 - BQ3) = 3/4 * (1 - BQ3)
–

If req0 is assigned to priority 0 and req1 to priority 2, the allocated bandwidth to
req0 (BP0) is 5 times less than the allocated bandwidth to req1 (BP2).
Breq0 = BP0 = 1/2 * 1/3 * (1 - BQ3) = 1/6 * (1 - BQ3)
Breq1 = BP2 = (1/2 + 1 +1) * 1/3 * (1 - BQ3) = 5/6 * (1 - BQ3)

The above computed bandwidth calculation is based on a theoretical input request,
always active for any LPDMA clock cycle. This computed bandwidth from the arbiter
must be weighted by the frequency of the request given by the application, that cannot
be always active and may be quite much variable from one LPDMA client (example I2C
at 400 kHz) to another one (PWM at 1 kHz) than the above x3 and x5 ratios.
•

Example 2: application where the user distributes a same non-null N number of
LPDMA requests to every non time-sensitive priority 0, 1 and 2. The bandwidth
calculation is then the following:
–

The allocated bandwidth to the set of requests of priority 0 (BP0) is
BP0 = 1/3 * 1/3 * (1 - BQ3) = 1/9 * (1 - BQ3)

–

The allocated bandwidth to the set of requests of priority 1(BP1) is

–

The allocated bandwidth to the set of requests of priority 2(BP2) is

BP1 = (1/3 + 1/2) * 1/3 * (1 - BQ3) = 5/18 * (1 - BQ3)
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

RM0456 Rev 6

<!-- pagebreak -->

821

Low-power direct memory access controller (LPDMA)

RM0456

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

As BQ3 is the reserved bandwidth to time-sensitive requests, the bandwidth for each request
L with priority 3 is:
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

<!-- pagebreak -->

Bk = BP2 / K (K>0 in the general case)

RM0456 Rev 6

RM0456

Low-power direct memory access controller (LPDMA)
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

Consequently, the LPDMA arbiter can be used as a programmable weighted bandwidth
limiter, for each queue and more generally for each request/channel. The different weights
are monotonically resulting from the programmed channel priorities.

18.4.12

LPDMA triggered transfer
A programmed LPDMA transfer can be triggered by a rising/falling edge of a selected input
trigger event, as defined by LPDMA_CxTR2.TRIGPOL[1:0] and
LPDMA_CxTR2.TRIGSEL[4:0] (see Section 18.3.5 for the trigger selection).
The triggered transfer, as defined by the trigger mode in LPDMA_CxTR2.TRIGM[1:0], can
be at LLI data transfer level, to condition the first single read of a block, or each
programmed single read. The trigger mode can also be programmed to condition the LLI
link transfer (see the TRIGM[1:0] description in LPDMA channel x transfer register 2
(LPDMA_CxTR2) for more details).

Trigger hit memorization and trigger overrun flag generation
The LPDMA monitoring of a trigger for a channel x is started when the channel is
enabled/loaded with a new active trigger configuration: rising or falling edge on a selected
trigger (respectively TRIGPOL[1:0] = 01 or TRIGPOL[1:0] = 10).
The monitoring of this trigger is kept active during the triggered and uncompleted (data or
link) transfer. If a new trigger is detected, this hit is internally memorized to grant the next
transfer, as long as the defined rising/falling edge and TRIGSEL[4:0] are not modified, and
the channel is enabled.
Transferring a next LLIn+1, that updates the LPDMA_CxTR2 with a new value for any of
TRIGSEL[4:0] or TRIGPOL[1:0], resets the monitoring, trashing the possible memorized hit
of the formerly defined LLIn trigger.
Caution:

After a first new trigger hitn+1 is memorized, if another trigger hitn+2 is detected and if the hitn
triggered transfer is still not completed, hitn+2 is lost and not memorized. A trigger overrun
flag is reported (LPDMA_CxSR.TOF = 1) and an interrupt is generated if enabled (if
LPDMA_CxCR.TOIE = 1). The channel is not automatically disabled by hardware due to a
trigger overrun.

RM0456 Rev 6

<!-- pagebreak -->

821

Low-power direct memory access controller (LPDMA)

RM0456

The figure below illustrates the trigger hit, memorization and overrun in the configuration
example with a block-level trigger mode and a rising edge trigger polarity.
Figure 87. Trigger hit, memorization and overrun waveform
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

18.4.13

LPDMA circular buffering with linked-list programming
LPDMA circular buffering for memory-to-peripheral and peripheral-tomemory transfers
For a circular buffering, with a continuous memory-to-peripheral (or peripheral-to-memory)
transfer, the software must set up a channel with half transfer and complete transfer
events/interrupts generation (LPDMA_CxCR.HTIE = 1 and LPDMA_CxCR.TCIE = 1), in
order to enable a concurrent buffer software processing.
LLI0 is configured for the first block transfer. A continuously-executed LLI1 is needed to
restore the memory source (or destination) start address, for the memory-to-peripheral
transfer (respectively the peripheral-to-memory transfer). LPDMA automatically reloads the
initially programmed LPDMA_CxBR1.BNDT[15:0] when a block transfer is completed, and
there is no need to restore LPDMA_CxBR1.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Low-power direct memory access controller (LPDMA)
The figure below illustrates this programming with a LPDMA channel and a source circular
buffer.
Figure 88. LPDMA circular buffer programming: update of the memory start address
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

If circular buffering must be executed after some other transfers over the shared LPDMA
channel x, the before-last LLIN-1 in memory is needed to configure the first block transfer.
And the last LLIN restores the memory source (or destination) start address in memory-toperipheral transfer (respectively in peripheral-to-memory transfer).

RM0456 Rev 6

<!-- pagebreak -->

821

Low-power direct memory access controller (LPDMA)

RM0456

The figure below illustrates this programming with a shared LPDMA channel, and a source
circular buffer.
Figure 89. Shared LPDMA channel with circular buffering:
update of the memory start address
Req=PERIPH_TX Req=PERIPH_TX

Reset
Init/LLI0

LLI1

...

Channel X

LLIN-1

LLIN

Ht+ tcf

Ht+ tcf

Memory

LLIN-1

LLIN-2
All Uxx=1

DMA_Cx...

DMA_CxTR1

DMA_Cx...

DMA_CxTR2

DMA_Cx...

DMA_CxBR1

DMA_CxLLR

LLIN

DMA_CxSAR
DMA_CxDAR
DMA_CxLLR

USA = 1, others Uxx = 0 LA+ = 0xC

18.4.14

MSv62641V1

LPDMA secure/nonsecure channel
The LPDMA controller is compliant with the TrustZone hardware architecture at channel
level, partitioning all its resources so that they exist in one of the secure and nonsecure
worlds at any given time.
Any channel x is a secure or a nonsecure hardware resource, as configured by
LPDMA_SECCFGR.SECx.
When a channel x is configured in secure state by a secure and privileged agent,
the following access control rules are applied:
•

A nonsecure read access to a register field of this channel is forced to return 0, except
for LPDMA_SECCFGR, LPDMA_PRIVCFGR, LPDMA_RCFGLOCKR that are
readable by a nonsecure agent.

•

A nonsecure write access to a register field of this channel has no impact.

When a channel x is configured in secure state, a secure agent can configure separately as
secure or nonsecure the LPDMA data transfer from the source (LPDMA_CxTR1.SSEC) and
the LPDMA data transfer to the destination (LPDMA_CxTR1.DSEC).
When a channel x is configured in secure state and in linked-list mode, the loading of the
next linked-list data structure from the LPDMA memory into its register file, is automatically
performed with secure transfers via the master port.
LPDMA generates the DMA channel state versus security, reflecting LPDMA_SECCFGR, to
keep the other peripherals informed of the secure/nonsecure state of each LPDMA
channel x.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Low-power direct memory access controller (LPDMA)
LPDMA also generates a security illegal access pulse signal on an illegal nonsecure access
to a secure LPDMA register. This signal is routed to the TrustZone interrupt controller.
When the secure software must switch a channel from a secure state to a nonsecure state,
the secure software must abort the channel or wait until the secure channel is completed
before switching. This is needed to dynamically re-allocate a channel to a next nonsecure
transfer as a nonsecure software is not allowed to do so and must have
LPDMA_CxCR.EN = 0 before the nonsecure software can reprogram the LPDMA_CxCR for
a next transfer. The secure software may reset not only the channel x
(LPDMA_CxCR.RESET = 1) but also the full channel x register file to its reset value.

18.4.15

LPDMA privileged/unprivileged channel
Any channel x is a privileged or unprivileged hardware resource, as configured by a
privileged agent via LPDMA_PRIVCFGR.PRIVx.
When a channel x is configured in a privileged state by a privileged agent, the following
access control rules are applied:
•

An unprivileged read access to a register field of this channel is forced to return 0,
except for LPDMA_PRIVCFGR, LPDMA_SECCFGR, that are readable by an
unprivileged agent.

•

An unprivileged write access to a register field of this channel has no impact.

When a channel is configured in a privileged (or unprivileged) state, the source and
destination data transfers are privileged (respectively unprivileged) transfers over the AHB
master port.
When a channel is configured in a privileged (or unprivileged) state and in linked-list mode,
the loading of the next linked-list data structure from the LPDMA memory into its register file,
is automatically performed with privileged (respectively unprivileged) transfers, via the
master port.
LPDMA generates a DMA channel state versus privilege, reflecting LPDMA_PRIVCFGR,
to keep the other peripherals informed of the privileged/unprivileged state of each DMA
channel x.
Additionally, the LPDMA generates the privileged illegal access pulse signal on an illegal
unprivileged access to a privileged LPDMA register. This signal may be used or not,
depending on the product (see the system security section for more details).
When the privileged software must switch a channel from a privileged state to
an unprivileged state, the privileged software must abort the channel or wait until that the
privileged channel is completed before switching. This is needed to dynamically re-allocate
a channel to a next unprivileged transfer as an unprivileged software is not allowed to do so,
and must have LPDMA_CxCR.EN = 0 before the unprivileged software can reprogram the
LPDMA_CxCR for a next transfer. The privileged software may reset not only the channel x
(LPDMA_CxCR.RESET = 1) but also the full channel x register file to its reset value.

RM0456 Rev 6

<!-- pagebreak -->

821

Low-power direct memory access controller (LPDMA)

18.4.16

RM0456

LPDMA error management
LPDMA is able to manage and report to the user a transfer error, as follows, depending on
the root cause.

Data transfer error
On a bus access (as a AHB single) to the source or the destination:
•

The source or destination target reports an AHB error.

•

The programmed channel transfer is stopped (LPDMA_CxCR.EN cleared by the
LPDMA hardware). The channel status register reports an idle state
(LPDMA_CxSR.IDLEF = 1) and the data error (LPDMA_CxSR.DTEF = 1).

•

After a LPDMA data transfer error, the user must perform a debug session, taking care
of the product-defined memory mapping of the source and destination, including the
protection attributes.

•

After a LPDMA data transfer error, the user must issue a channel reset
(set LPDMA_CxCR.RESET) to reset the hardware LPDMA channel data path before
the user enables again the same channel for a next transfer.

Link transfer error
On a tentative update of a LPDMA channel register from the programmed LLI in the
memory:
•

The linked-list memory reports an AHB error.

•

The programmed channel transfer is stopped (LPDMA_CxCR.EN cleared by the
LPDMA hardware), the channel status register reports an idle state
(LPDMA_CxSR.IDLEF = 1) and the link error (LPDMA_CxSR.ULEF = 1).

•

After a LPDMA link error, the user must perform a debug session, taking care of the
product-defined memory mapping of the linked-list data structure (LPDMA_CxLBAR
and LPDMA_CxLLR), including the protection attributes.

•

After a LPDMA link error, the user must explicitly write the linked-list register file
(LPDMA_CxTR1, LPDMA_CxTR2, LPDMA_CxBR1, LPDMA_CxSAR,
LPDMA_CxDAR and LPDMA_CxLLR), before the user enables again the same
channel for a next transfer.

User setting error
On a tentative execution of a LPDMA transfer with an unauthorized user setting:

<!-- pagebreak -->

•

The programmed channel transfer is disabled (LPDMA_CxCR.EN forced and cleared
by the LPDMA hardware) preventing the next unauthorized programmed data transfer
from being executed. The channel status register reports an idle state
(LPDMA_CxSR.IDLEF = 1) and a user setting error (LPDMA_CxSR.USEF = 1).

•

After a LPDMA user setting error, the user must perform a debug session, taking care
of the LPDMA channel programming. A user setting error can be caused by one of the
following:
–

a programmed null source block size without a programmed update of this value
from the next LLI1 (LPDMA_CxBR1.BNDT[15:0] = 0 and
LPDMA_CxLLR.UB1 = 0)

–

a programmed non-null source block size being not a multiple of the programmed
data width of a source single transfer (LPDMA_CxBR1.BNDT[2:0] versus
LPDMA_CxTR1.SDW_LOG2[1:0])

RM0456 Rev 6

RM0456

18.4.17

Low-power direct memory access controller (LPDMA)
–

a programmed unaligned source start address, being not a multiple of the
programmed data width of a source single transfer (LPDMA_CxSAR[2:0] versus
LPDMA_CxTR1.SDW_LOG2[1:0])

–

a programmed unaligned destination start address, being not a multiple of the
programmed data width of a destination single transfer (LPDMA_CxDAR[2:0]
versus LPDMA_CxTR1.DDW_LOG2[1:0])

–

a programmed double-word source data width
(LPDMA_CxTR1.SDW_LOG2[1:0] = 0b11)

–

a programmed double-word destination data width
(LPDMA_CxTR1.DDW_LOG2[1:0] = 0b11

–

a programmed linked-list item LLIn+1 with a null data transfer
(LPDMA_CxLLR.UB1 = 1 and LPDMA_CxBR1. BNDT = 0)

LPDMA autonomous mode
To save dynamic power consumption while LPDMA executes the programmed linked-list
transfers, LPDMA hardware automatically manages its own clock gating and generates a
clock request output signal to the RCC, whenever the device is in Run, Sleep or Stop mode,
provided that the RCC is programmed with the corresponding LPDMA enable control bits.
For more details about the RCC programming, refer to the RCC section of the reference
manual.
For mode details about the availability of the LPDMA autonomous feature vs the device lowpower modes, refer to Section 18.3.2.

Note:

design/firmware: In low power modes, DMA clock request is asserted upon a DMA
request/trigger from an autonomous peripheral or upon a DMA trigger from an EXTI line.
The user can program and schedule the execution of a given LPDMA transfer at a LLIn level
of a LPDMA channel x, with LPDMA_CxTR2 as follows:
•

The software controls and conditions the input of a transfer with TRIGM[1:0],
TRIGPOL[1:0], TRIGSEL[y(a):0], SWREQ, and REQSEL[z(a):0] for the input trigger and
request.

•

The software controls and signals the output of a transfer with TCEM[1:0] for
generating or not a transfer complete event, and generating or not an associated half
data transfer event).

See LPDMA channel x transfer register 2 (LPDMA_CxTR2) for more details.
The output channel x transfer complete event, lpdma_chx_tc, can be programmed as a
selected input trigger for a channel if this event is looped-back and connected at the LPDMA
level (see Section 18.3.5), allowing autonomous and fine DMA inter-channel transfer
scheduling, without needing a cleared transfer complete flag (TCF).

a. Refer to LPDMA channel x transfer register 2 (LPDMA_CxTR2) for y and z values.

RM0456 Rev 6

<!-- pagebreak -->

