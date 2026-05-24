821

Low-power direct memory access controller (LPDMA)

RM0456

The next LLI contains an updated value for LPDMA_CxTR2, LPDMA_CxDAR, and
LPDMA_CxLLR, as shown in the figure below.
Figure 77. LPDMA dynamic linked-list data structure of an addressing channel x
LLIn
All Uxx = 1

DMA_CxTR1
DMA_CxTR2
DMA_CxBR1

UT1 = UB1 = USA = 0
UT2 = UDA = ULL = 1

DMA_CxSAR
DMA_CxDAR

LLIn+1
DMA_CxTR2
DMA_CxDAR
DMA_CxLLR

DMA_CxLLR
MSv62630V1

The user must program LPDMA_CxLLR for each LLIn to be 32-bit aligned and not to exceed
the 64-Kbyte addressable space pointed by LPDMA_CxLBAR.

18.4.6

Linked-list item transfer execution
A LLIn transfer is the sequence of:

Note:

1.

a data transfer: LPDMA executes the data transfer as described by the LPDMA internal
register file (this data transfer can be void/null for LLI0)

2.

a conditional link transfer: LPDMA automatically and conditionally updates its internal
register file by the data structure of the next LLIn+1, as defined by the LPDMA_CxLLR
value of the LLIn.

The initial data transfer as defined by the internal register file (LLI0) can be null
(LPDMA_CxBR1.BNDT[15:0] = 0) provided that the conditional update bit UB1 in
LPDMA_CxLLR is set (meaning there is a non-null data transfer described by the next LLI1
in the memory to be executed).
Depending on the intended LPDMA usage, a LPDMA channel x can be executed as
described by the full linked-list (run-to-completion mode, LPDMA_CxCR.LSM = 0) or a
LPDMA channel x can be programmed for a single execution of a LLI (link step mode,
LPDMA_CxCR.LSM = 1), as described in the next paragraphs.

18.4.7

LPDMA channel state and linked-list programming
in run-to-completion mode
When LPDMA_CxCR.LSM = 0, a LPDMA channel x is initially programmed, started by
writing 1 to LPDMA_CxCR.EN, and after (possibly) completed at channel level. The channel
transfer is:
•

•

<!-- pagebreak -->

configured with at least the following:
–

the first LLI0, internal linked-list register file: LPDMA_CxTR1, LPDMA_CxTR2,
LPDMA_CxBR1, LPDMA_CxSAR, LPDMA_CxDAR and LPDMA_CxLLR

–

the last LLIN, described by the linked-list data structure in memory, as defined by
the LPDMA_CxLLR reflecting the before last LLIN-1

completed when LPDMA_CxLLR[31:0] = 0 and LPDMA_CxBR1.BNDT[15:0] = 0, at
the end of the last LLIN-1 transfer

RM0456 Rev 6

RM0456

Low-power direct memory access controller (LPDMA)
LPDMA_CxLLR[31:0] = 0 is the condition of a linked-list based channel completion and
means the following:
•

The 16 low significant bits LPDMA_CxLLR.LA[15:0] of the next link address are null.

•

All the update bits Uxx of LPDMA_CxLLR are null (UT1, UT2, UB1, USA, UDA, ULL).

The channel may never be completed when LPDMA_CxLLR.LSM = 0:
•

•

If the last LLIN is recursive, pointing to itself as a next LLI:
–

either LPDMA_CxLLR.ULL = 1 and LPDMA_CxLLR.LA[15:2] is updated by the
same value

–

or LPDMA_CxLLR.ULL = 0

If LLIN is pointing to a previous LLI.

In the typical run-to-completion mode, the allocation of a LPDMA channel, including its fine
programming, is done once during the LPDMA initialization. In order to have a reserved data
communication link and LPDMA service during run-time, for continuously repeated transfers
(from/to a peripheral respectively to/from memory or for memory-to-memory transfers). This
reserved data communication link can consist of a channel, or the channel can be shared
and a repeated transfer consists of a sequence of LLIs.
Figure 78 depicts the LPDMA channel execution and its registers programming in run-tocompletion mode.
Note:

Figure 78 is not intended to illustrate how often a TCEF can be raised, depending on the
programmed value of TCEM[1:0] in LPDMA_CxTR2. It can be raised at (each) block
completion, at (each) 2D block completion, at (each) LLI completion, or only at channel
completion. In run-to-completion mode, whatever is the value of TCEM[1:0], at the channel
completion, the hardware always set TCEF = 1 and disables the channel.

RM0456 Rev 6

<!-- pagebreak -->

821

Low-power direct memory access controller (LPDMA)

RM0456

Figure 78. LPDMA channel execution and linked-list programming
in run-to-completion mode (LPDMA_CxCR.LSM = 0)

Channel state = Idle
Initialize DMA channel

Reconfigure DMA channel
Enable DMA channel

Channel state = Active
Valid user
setting ?

N

Y
Setting USEF = 1
Disabling DMA channel

N

BNDT0 ?

Y
Executing once the data
transfer from the register file

No transfer
error ?

N

Y
N

Setting DTEF = 1
Disabling DMA channel

LLR 0 ?
Y

Loading next LLI
into the register file

No transfer
error ?

N

Y

Y
Setting TCF = 1
Disabling DMA channel

Valid user
setting ?

Setting ULEF = 1
Disabling DMA channel
N
Setting USEF = 1
Disabling DMA channel

End
MSv62631V1

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Low-power direct memory access controller (LPDMA)

Run-time inserting a LLIn via an auxiliary channel, in run-to-completion mode
The start of the link transfer of the LLIn-1 (start of the LLIn loading) can be conditioned by the
occurrence of a trigger, when programming the following fields of the LPDMA_CxTR2 in the
data structure of the LLIn-1:
•

TRIGM[1:0] = 10 (link transfer triggering mode)

•

TRIGPOL[1:0] = 01 or 10 (rising or falling edge)

•

TRIGSEL[4:0] (see Section 18.3.5 for the trigger selection details)

Another auxiliary channel y can be used to store the channel x LLIn in the memory and to
generate a transfer complete event lpdma_chy_tc. By selecting this event as the input
trigger of the link transfer of the LLIn-1 of the channel x, the software can pause the primary
channel x after its LLIn-1 data transfer, until it is indeed written the LLIn.
Figure 79 depicts such a dynamic elaboration of a linked-list of a primary channel x, via
another auxiliary channel y.
Caution:

This use case is restricted to an application with a LLIn-1 data transfer that does not need a
trigger. The triggering mode of this LLIn-1 is used to load the next LLIn.

RM0456 Rev 6

<!-- pagebreak -->

821

Low-power direct memory access controller (LPDMA)

RM0456

Figure 79. Inserting a LLIn with an auxiliary LPDMA channel y

LLIn-1 transfer

DMA primary channel x

DMA auxiliary channel y

CPU

Executing LLIn-2 data transfer
Loading LLIn-1
(with DMA_CxTR2: TRIGM[1:0] = 10
TRIGPOL[1:0] = 01
TRIGSEL= dma_chy_tc
and TCEM[1:0] = 01)

Executing LLIn-1 data transfer
Transfer complete interrupt

LLIn transfer

Build new LLIn
Configure channel Y

Executing data transfer
(Memcopy of new LLIn)
dma_chy_tc

LLIn+1
transfer

Loading new LLIn

Executing LLIn data transfer
Loading LLIn+1
MSv62632V2

<!-- pagebreak -->

