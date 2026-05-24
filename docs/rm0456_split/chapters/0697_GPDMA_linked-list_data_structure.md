1.

The software writes 1 into the GPDMA_CxCR.SUSP bit.

2.

The software polls suspended flag GPDMA_CxSR.SUSPF until SUSPF = 1, or waits
for an interrupt previously enabled by writing 1 to GPDMA_CxCR.SUSPIE. Wait for the
channel to be effectively in suspended state means wait for the completion of any
ongoing GPDMA transfer over its master port.

3.

The software resets the channel by writing 1 to GPDMA_CxCR.RESET. This causes
the reset of the FIFO, the reset of the channel internal state, the reset of the
GPDMA_CxCR.EN bit, and the reset of the GPDMA_CxCR.SUSP bit.

4.

The software safely reconfigures the channel. The software must reprogram the
hardware-modified GPDMA_CxBR1, GPDMA_CxSAR, and GPDMA_CxDAR
registers.

5.

In order to restart the aborted then reprogrammed channel, the software enables it
again by writing 1 to the GPDMA_CxCR.EN bit.

RM0456 Rev 6

RM0456

General purpose direct memory access controller (GPDMA)
Figure 54. GPDMA channel abort and restart sequence

Channel state = Active
Suspend the DMA channel
(write 1 to CxCR.SUSP)
or

SUSPF=1 ?

Channel state = Suspended
(and Idle)
Receiving

N

Y

suspended
interrupt

Reset the DMA channel
(write 1 to CxCR.RESET)

Channel state = Idle
Reconfigure the DMA channel

Enable the DMA channel

Channel state = Active
MSv62628V1

17.4.5

GPDMA linked-list data structure
Alternatively to the direct programming mode, a channel can be programmed by a list of
transfers, known as a list of linked-list items (LLI). Each LLI is defined by its data structure.
The base address in memory of the data structure of a next LLIn+1 of a channel x is the sum
of the following:
•

the link base address of the channel x (in GPDMA_CxLBAR)

•

the link address offset (LA[15:2] field in GPDMA_CxLLR). The linked-list register
GPDMA_CxLLR is the updated result from the data structure of the previous LLIn of
the channel x.

The data structure for each LLI may be specific.
A linked-list data structure is addressed following the value of the UT1, UT2, UB1, USA,
UDA and ULL bits, plus UB2 and UT3, in GPDMA_CxLLR.
In linked-list mode, each GPDMA linked-list register (GPDMA_CxTR1, GPDMA_CxTR2,
GPDMA_CxBR1, GPDMA_CxSAR, GPDMA_CxDAR or GPDMA_CxLLR, plus
GPDMA_CxTR3 or GPDMA_CxBR2) is conditionally and automatically updated from the
next linked-list data structure in the memory, following the current value of the

RM0456 Rev 6

<!-- pagebreak -->

763

General purpose direct memory access controller (GPDMA)

RM0456

GPDMA_CxLLR register that was conditionally updated from the linked-list data structure of
the previous LLI.

Static linked-list data structure
For example, when the update bits (UT1, UT2, UB1, USA, UDA and ULL, plus UB2 and
UT3) in GPDMA_CxLLR are all asserted, the linked-list data structure in the memory is
maximal with:
•

channel x (x = 0 to 11) contiguous 32-bit locations, including GPDMA_CxTR1,
GPDMA_CxTR2, GPDMA_CxBR1, GPDMA_CxSAR, GPDMA_CxDAR and
GPDMA_CxLLR (see Figure 55) and including the first linked-list register file (LLI0) and
the next LLIs (such as LLI1, LLI2) in the memory

•

channel x (x = 12 to 15), contiguous 32-bit locations, including GPDMA_CxTR1,
GPDMA_CxTR2, GPDMA_CxBR1, GPDMA_CxSAR, GPDMA_CxDAR, and
GPDMA_CxLLR, plus GPDMA_CxTR3 and GPDMA_CxBR2 (see Figure 56), and
including the first linked-list register file (LLI0) and the next LLIs (such as LLI1, LLI2) in
the memory
Figure 55. Static linked-list data structure (all Uxx = 1)
of a linear addressing channel x
DMA register file

Memory from link base address
DMA_CxLBAR

Channel x linked-list register file
(LLI0)
DMA_CxTR1

LLI1
All Uxx=1

DMA_CxTR1
DMA_CxTR2

DMA_CxTR2
DMA_CxBR1

DMA_CxBR1

DMA_CxSAR

DMA_CxSAR

DMA_CxDAR

DMA_CxDAR

DMA_CxLLR

DMA_CxLLR

Channel x other registers

LLI2

All Uxx=1

DMA_CxTR1
Other channels registers

DMA_CxTR2

Global registers

DMA_CxBR1
DMA_CxSAR
DMA_CxDAR
DMA_CxLLR

MSv62629V1

<!-- pagebreak -->

RM0456 Rev 6

RM0456

General purpose direct memory access controller (GPDMA)
Figure 56. Static linked-list data structure (all Uxx = 1)
of a 2D addressing channel x
DMA register file

Memory from link base address
DMA_CxLBAR

Channel x linked-list register file
(LLI0)
DMA_CxTR1

LLI1
All Uxx=1

DMA_CxTR1
DMA_CxTR2

DMA_CxTR2
DMA_CxBR1

DMA_CxBR1

DMA_CxSAR

DMA_CxSAR

DMA_CxDAR

DMA_CxDAR

DMA_CxTR3

DMA_CxTR3

DMA_CxBR2

DMA_CxBR2

DMA_CxLLR

DMA_CxLLR
All Uxx=1

Channel x other registers
Other channels registers

LLI2
DMA_CxTR1
DMA_CxTR2

Global registers

DMA_CxBR1
DMA_CxSAR
DMA_CxDAR
DMA_CxTR3
DMA_CxBR2
DMA_CxLLR
MSv63645V1

Dynamic linked-list data structure
Alternatively, the memory organization for the full list of LLIs can be compacted with specific
data structure for each LLI.
If UT1 = 0 and UT2 = 1, the link address offset of the register GPDMA_CxLLR is pointing to
the updated value of the GPDMA_CxTR2 instead of the GPDMA_CxTR1 which is not to be
modified (see Figure 57).
Example: if UT1 = UB1 = USA = 0 and if UT3 = UB2 = 0, when channel x is with 2D
addressing, and if UT2 = UDA = ULL = 1, the next LLI does not contain an (updated) value
for GPDMA_CxTR1, nor GPDMA_CxBR1, nor GPDMA_CxSAR, nor GPDMA_CxTR3, nor
GPDMA_CxBR2 when channel x is with 2D addressing. The next LLI contains an updated
value for GPDMA_CxTR2, GPDMA_CxDAR, and GPDMA_CxLLR, as shown in Figure 58.

RM0456 Rev 6

<!-- pagebreak -->

