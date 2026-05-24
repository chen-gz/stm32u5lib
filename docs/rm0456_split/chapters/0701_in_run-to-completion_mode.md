RM0456 Rev 6

RM0456

17.4.7

General purpose direct memory access controller (GPDMA)

GPDMA channel state and linked-list programming
in run-to-completion mode
When GPDMA_CxCR.LSM = 0 (in full list execution mode, execution of the full sequence of
LLIs, named run-to-completion mode), a GPDMA channel x is initially programmed, started
by writing 1 to GPDMA_CxCR.EN, and after completed at channel level. The channel
transfer is:
•

•

configured with at least the following:
–

the first LLI0, internal linked-list register file: GPDMA_CxTR1, GPDMA_CxTR2,
GPDMA_CxBR1, GPDMA_CxSAR, GPDMA_CxDAR, and GPDMA_CxLLR, plus
GPDMA_CxTR3 and GPDMA_CxBR2

–

the last LLIN, described by the linked-list data structure in memory, as defined by
the GPDMA_CxLLR reflecting the before last LLIN-1

completed when GPDMA_CxLLR[31:0] = 0, GPDMA_CxBR1.BRC[10:0] = 0, and
GPDMA_CxBR1.BNDT[15:0] = 0, at the end of the last LLIN-1 transfer

GPDMA_CxLLR[31:0] = 0 is the condition of a linked-list based channel completion and
means the following:
•

The 16 low significant bits GPDMA_CxLLR.LA[15:0] of the next link address are null.

•

All the update bits GPDMA_CxLLR.Uxx are null (UT1, UT2, UB1, USA, UDA and ULL,
plus UB2 and UT3).

The channel may never be completed when GPDMA_CxLLR.LSM = 0:
•

•

If the last LLIN is recursive, pointing to itself as a next LLI:
–

either GPDMA_CxLLR.ULL = 1 and GPDMA_CxLLR.LA[15:2] is updated by the
same value

–

or GPDMA_CxLLR.ULL = 0

If LLIN is pointing to a previous LLI

In the typical run-to-completion mode, the allocation of a GPDMA channel, including its fine
programming, is done once during the GPDMA initialization. In order to have a reserved
data communication link and GPDMA service during run-time, for continuously repeated
transfers (from/to a peripheral respectively to/from memory or for memory-to-memory
transfers). This reserved data communication link can consist of a channel, or the channel
can be shared and a repeated transfer consists of a sequence of LLIs.
Figure 59 depicts the GPDMA channel execution and its registers programming in run-tocompletion mode.
Note:

Figure 59 is not intended to illustrate how often a TCEF can be raised, depending on the
programmed value of TCEM[1:0] in GPDMA_CxTR2. It can be raised at (each) block
completion, at (each) 2D block completion, at (each) LLI completion, or only at channel
completion. In run-to-completion mode, whatever is the value of TCEM[1:0], at the channel
completion, the hardware always set TCEF = 1 and disables the channel.

RM0456 Rev 6

<!-- pagebreak -->

763

General purpose direct memory access controller (GPDMA)

RM0456

Figure 59. GPDMA channel execution and linked-list programming
in run-to-completion mode (GPDMA_CxCR.LSM = 0)

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

General purpose direct memory access controller (GPDMA)

Run-time inserting a LLIn via an auxiliary channel, in run-to-completion mode
The start of the link transfer of the LLIn-1 (start of the LLIn loading) can be conditioned by the
occurrence of a trigger, when programming the following fields of the GPDMA_CxTR2 in the
data structure of the LLIn-1:
•

TRIGM[1:0] = 10 (link transfer triggering mode)

•

TRIGPOL[1:0] = 01 or 10 (rising or falling edge)

•

TRIGSEL[6:0] (see Section 17.3.5 for the trigger selection details)

Another auxiliary channel y can be used to store the channel x LLIn in the memory and to
generate a transfer complete event gpdma_chy_tc. By selecting this event as the input
trigger of the link transfer of the LLIn-1 of the channel x, the software can pause the primary
channel x after its LLIn-1 data transfer, until it is indeed written the LLIn.
Figure 60 depicts such a dynamic elaboration of a linked-list of a primary channel x, via
another auxiliary channel y.
Caution:

This use case is restricted to an application with a LLIn-1 data transfer that does not need a
trigger. The triggering mode of this LLIn-1 is used to load the next LLIn.

RM0456 Rev 6

<!-- pagebreak -->

763

General purpose direct memory access controller (GPDMA)

RM0456

Figure 60. Inserting a LLIn with an auxiliary GPDMA channel y

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

17.4.8

GPDMA channel state and linked-list programming in link step mode
When GPDMA_CxCR.LSM = 1 (in link step execution mode, single execution of one LLI), a
channel transfer is executed and completed after each single execution of a LLI, including
its (conditional) data transfer and its (conditional) link transfer.
A GPDMA channel transfer can be programmed at LLI level, started by writing 1 into
GPDMA_CxCR.EN, and after completed at LLI level:
•

<!-- pagebreak -->

The current LLIn transfer is described with:
–

GPDMA_CxTR1 defines the source/destination elementary single/burst transfers.

–

GPDMA_CxBR1 defines the number of bytes at a block level (BNDT[15:0]) and,
for channel x (x = 12 to 15), the number of blocks at a 2D/repeated block level
(BRC[10:0]+1) and the incrementing/decrementing mode for address offsets.

RM0456 Rev 6

RM0456

General purpose direct memory access controller (GPDMA)

•

Note:

–

GPDMA_CxTR2 defines the input control (request, trigger) and the output control
(transfer complete event) of the transfer.

–

GPDMA_CxSAR/GPDMA_CxDAR define the source/destination transfer start
address.

–

GPDMA_CxTR3 for channel x (x = 12 to 15) defines the source/destination
additional address offset between burst transfers.

–

GPDMA_CxBR2 for channel x (x = 12 to 15) defines the source/destination
additional address offset between blocks at a 2D/repeated block level.

–

GPDMA_CxLLR defines the data structure and the address offset of the next
LLIn+1 in the memory.

The current LLIn transfer is completed after the single execution of the current LLIn:
–

after the (conditional) data transfer completion (when
GPDMA_CxBR1.BRC[10:0] = 0, and GPDMA_CxBR1.BNDT[15:0] = 0

–

after the (conditional) update of the GPDMA link register file from the data
structure of the next LLIn+1 in memory

If a LLI is recursive (pointing to itself as a next LLI, either GPDMA_CxLLR.ULL = 1 and
GPDMA_CxLLR.LA[15:2] is updated by the same value, or GPDMA_CxLLR.ULL = 0), a
channel in link step mode is completed after each repeated single execution of this LLI.
The link step mode can be used to elaborate dynamically LLIs in memory during run-time.
The software can be facilitated by using a static data structure for any LLIn (all update bits of
GPDMA_CxLLR have a static value, LLIn.LLR.LA = LLIn-1.LLR.LA + constant).
Figure 61 depicts the GPDMA channel execution mode, and its programming in link step
mode.

Note:

Figure 61 is not intended to illustrate how often a TCEF can be raised, depending on the
programmed value of TCEM[1:0] in GPDMA_CxTR2. It can be raised at (each) block
completion, at (each) 2D block completion, at (each) LLI completion, or only at the last LLI
data transfer completion. In link step mode, the channel is disabled after each single
execution of a LLI, and depending on the value of TCEM[1:0] a TCEF is raised or not.

RM0456 Rev 6

<!-- pagebreak -->

763

General purpose direct memory access controller (GPDMA)

RM0456

Figure 61. GPDMA channel execution and linked-list programming
in link step mode (GPDMA_CxCR.LSM = 1)

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

BNDT  0 ?
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
Valid user
setting ?

Setting ULEF = 1
Disabling DMA channel
N

Y
Setting TCF = 1
Disabling DMA channel

Setting USEF = 1
Disabling DMA channel

End
MSv62633V1

<!-- pagebreak -->

RM0456 Rev 6

RM0456

General purpose direct memory access controller (GPDMA)

Run-time adding a LLIn+1 in link step mode
During run-time, the software can defer the elaboration of the LLIn+1 (and next LLIs),
until/after GPDMA executed the transfer from the LLIn-1 and loaded the LLIn from the
memory, as shown in Figure 62.
Figure 62. Building LLIn+1: GPDMA dynamic linked-lists in link step mode

LSM = 1 with 2-stage linked-list programming:
DMA executes LLIn-1 and loads LLIn while CPU builds LLIn+1

DMA Channel

CPU

LLIn-2
transfer
Transfer complete interrupt

LLIn-1
transfer

Enable DMA channel

Executing LLIn-1 data transfer
Build and store LLIn+1
Loading LLIn
Transfer complete interrupt

Enable DMA channel

LLIn
transfer

MSv62634V1

Run-time replacing a LLIn with a new LLIn’ in link step mode (in linked-list
register file)
In this link step mode, during run-time, the software can build and insert a new LLIn’, after
GPDMA executed the transfer from the LLIn-1 and loaded a formerly elaborated LLIn from
the memory by overwriting directly the linked-list register file with the new LLIn’, as shown in
Figure 63.

RM0456 Rev 6

<!-- pagebreak -->

763

General purpose direct memory access controller (GPDMA)

RM0456

Figure 63. Replace with a new LLIn’ in register file in link step mode

LSM = 1 with 1-stage linked-list programming:
Overwriting the (pre)loaded LLIn linked-list register file with
a new LLIn’ directly in linked-list register file.
DMA executes LLIn-1 and load LLIn, then CPU builds and overwrites LLIn'

LLIn-1
transfer

DMA channel

CPU

Executing LLIn-1 data transfer
Loading LLIn
Transfer complete interrupt

Build LLIn' and overwrite
linked-list register file

LLIn'
transfer

Enable DMA channel

Executing LLIn' data transfer
Loading LLIn+1’
Transfer complete interrupt

Build LLIn+1’' and overwrite
linked-list register file

Enable DMA channel

LLIn+1"
transfer
Transfer complete interrupt
MSv62635V1

Run-time replacing a LLIn with a new LLIn’ in link step mode (in the memory)
The software can build and insert a new LLIn’ and LLIn+1’ in the memory, after GPDMA
executed the transfer from the LLIn-1 and loaded a formerly elaborated LLIn from the
memory, by overwriting partly the linked-list register file (GPDMA_CxBR1.BNDT[15:0] to be
null and GPDMA_CxLLR to point to new LLIn’) as shown in Figure 64.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

General purpose direct memory access controller (GPDMA)

Figure 64. Replace with a new LLIn’ and LLIn+1’ in memory in link step mode (option 1)

LSM = 1 with 1-stage linked-list programming:
Overwriting the (pre)loaded LLIn linked-list register file with a new LLIn' and LLIn+1' in memory and
overwrite partly linked-list register file
(DMA_CxBR1.BNDT = 0 and DMA_CxLLR to point to new LLIn')
DMA executes LLIn-1 and load LLIn then CPU builds (LLIn' and LLIn+1') and overwrite (BR1 and LLR)

LLIn-1
transfer

DMA Channel

CPU

Executing LLIn-1 data transfer
Loading LLIn
Transfer complete interrupt

Build LLIn' and LLIn+1' in memory
Write DMA_CxBR1.BNDT = 0
Write DMA_CxLLR to point to new LLIn'

LLIn'
transfer

Enable DMA channel

Loading LLIn’
Transfer complete interrupt

LLIn+1'
transfer

Enable DMA channel

Executing LLIn+1' data transfer
Loading LLIn+1'
Transfer complete interrupt
MSv62636V1

RM0456 Rev 6

<!-- pagebreak -->

763

General purpose direct memory access controller (GPDMA)

RM0456

Run-time replacing a LLIn with a new LLIn’ in link step mode
Other software implementations exist. Meanwhile GPDMA executes the transfer from the
LLIn-1 and loads a formerly elaborated LLIn from the memory (or even earlier), the software
can do the following:
1.

Disable the NVIC for not being interrupted by the interrupt handling.

2.

Build a new LLIn’ and a new LLIn+1’.

3.

Enable again the NVIC for the channel interrupt (transfer complete) notification.

The software in the interrupt handler for LLIn-1 is then restricted to overwrite
GPDMA_CxBR1.BNDT[15:0] to be null and GPDMA_CxLLR to point to new LLIn’, as shown
in Figure 65.
Figure 65. Replace with a new LLIn’ and LLIn+1’ in memory in link step mode (option 2)

LSM = 1 with 1-stage linked-list programming:
Overwriting the (pre)loaded LLIn linked-list register file by building new LLIn' and LLIn+1' in memory
while disabling (temporary) channel interrupt at NVIC level, and overwriting DMA_CxBR1.BNDT = 0
and DMA_CxLLR to point to new LLIn'
DMA executes LLIn-1 and loading LLIn while CPU builds (LLIn' and LIn+1'), then CPU overwrites
(BR1 and LLR)

LLIn-1
transfer

DMA channel

CPU

Disable NVIC DMA irq channel

Executing LLIn-1 data transfer

Build LLIn' & LLIn+1' in memory

Loading LLIn

Enable NVIC DMA irq channel
Transfer complete interrupt

Write DMA_CxBR1.BNDT = 0
Write DMA_CxLLR to point to new LLIn'

LLIn'
transfer

Enable DMA channel

Loading LLIn’
Transfer complete interrupt

LLIn+1'
transfer

Enable DMA channel

Executing LLIn+1' data transfer
Loading LLIn+1'
Transfer complete interrupt

<!-- pagebreak -->

