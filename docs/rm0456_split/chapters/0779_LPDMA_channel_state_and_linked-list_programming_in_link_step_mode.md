RM0456 Rev 6

RM0456

18.4.8

Low-power direct memory access controller (LPDMA)

LPDMA channel state and linked-list programming in link step mode
When LPDMA_CxCR.LSM = 1, a channel transfer is executed and completed after each
single execution of a LLI, including its (conditional) data transfer and its (conditional) link
transfer.
A LPDMA channel transfer can be programmed at LLI level, started by writing 1 into
LPDMA_CxCR.EN, and after completed at LLI level:
•

•

Note:

The current LLIn transfer is described with:
–

LPDMA_CxTR1 defines the source/destination elementary single transfers.

–

LPDMA_CxBR1 defines the number of bytes at a block level (BNDT[15:0]).

–

LPDMA_CxTR2 defines the input control (request, trigger) and the output control
(transfer complete event) of the transfer.

–

LPDMA_CxSAR/LPDMA_CxDAR defines the source/destination transfer start
address.

–

LPDMA_CxLLR defines the data structure and the address offset of the next
LLIn+1 in the memory.

The current LLIn transfer is completed after the single execution of the current LLIn:
–

after the (conditional) data transfer completion (when
LPDMA_CxBR1.BNDT[15:0] = 0)

–

after the (conditional) update of the LPDMA link register file from the data structure
of the next LLIn+1 in memory

If a LLI is recursive (pointing to itself as a next LLI, either LPDMA_CxLLR.ULL = 1 and
LPDMA_CxLLR.LA[15:2] is updated by the same value, or LPDMA_CxLLR.ULL = 0), a
channel in link step mode is completed after each repeated single execution of this LLI.
Figure 80 depicts the LPDMA channel execution mode, and its programming in link step
mode.

Note:

Figure 80 is not intended to illustrate how often a TCEF can be raised, depending on the
programmed value of TCEM[1:0] in LPDMA_CxTR2. It can be raised at (each) block
completion, at (each) 2D block completion, at (each) LLI completion, or only at the last LLI
data transfer completion. In link step mode, the channel is disabled after each single
execution of a LLI, and depending on the value of TCEM[1:0] a TCEF is raised or not.

RM0456 Rev 6

<!-- pagebreak -->

821

Low-power direct memory access controller (LPDMA)

RM0456

Figure 80. LPDMA channel execution and linked-list programming
in link step mode (LPDMA_CxCR.LSM = 1)

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

Low-power direct memory access controller (LPDMA)
The link step mode can be used to elaborate dynamically LLIs in memory during run-time.
The software can be facilitated by using a static data structure for any LLIn (all update bits of
LPDMA_CxLLR have a static value, LLIn.LLR.LA = LLIn-1.LLR.LA + constant).

Run-time adding a LLIn+1 in link step mode
During run-time, the software can defer the elaboration of the LLIn+1 (and next LLIs),
until/after LPDMA executed the transfer from the LLIn-1 and loaded the LLIn from the
memory, as shown in the figure below.
Figure 81. Building LLIn+1: LPDMA dynamic linked-lists in link step mode

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
LPDMA executed the transfer from the LLIn-1 and loaded a formerly elaborated LLIn from
the memory by overwriting directly the linked-list register file with the new LLIn’, as shown in
Figure 82.

RM0456 Rev 6

<!-- pagebreak -->

821

Low-power direct memory access controller (LPDMA)

RM0456

Figure 82. Replace with a new LLIn’ in register file in link step mode

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
The software can build and insert a new LLIn’ and LLIn+1’ in the memory, after LPDMA
executed the transfer from the LLIn-1 and loaded a formerly elaborated LLIn from the
memory, by overwriting partly the linked-list register file (LPDMA_CxBR1.BNDT[15:0] to be
null and LPDMA_CxLLR to point to new LLIn’) as shown in Figure 83.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Low-power direct memory access controller (LPDMA)

Figure 83. Replace with a new LLIn’ and LLIn+1’ in memory in link step mode (option 1)

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

821

Low-power direct memory access controller (LPDMA)

RM0456

Run-time replacing a LLIn with a new LLIn’ in link step mode
Other software implementations exist. Meanwhile LPDMA executes the transfer from the
LLIn-1 and loads a formerly elaborated LLIn from the memory (or even earlier), the software
can do the following:
1.

Disable the NVIC for not being interrupted by the interrupt handling.

2.

Build a new LLIn’ and a new LLIn+1’.

3.

Enable again the NVIC for the channel interrupt (transfer complete) notification.

The software in the interrupt handler for LLIn-1 is then restricted to overwrite
LPDMA_CxBR1.BNDT[15:0] to be null and LPDMA_CxLLR to point to new LLIn’, as shown
in Figure 84.
Figure 84. Replace with a new LLIn’ and LLIn+1’ in memory in link step mode (option 2)

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

