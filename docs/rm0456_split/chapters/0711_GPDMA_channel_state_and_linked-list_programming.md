RM0456 Rev 6

MSv62637V1

RM0456

17.4.9

General purpose direct memory access controller (GPDMA)

GPDMA channel state and linked-list programming
The software can reconfigure a channel when the channel is disabled
(GPDMA_CxCR.EN = 0) and update the execution mode (GPDMA_CxCR.LSM) to change
from/to run-to-completion mode to/from link step mode.
In any execution mode, the software can:
•

reprogram LLIn+1 in the memory to finally complete the channel by this LLIn+1 (clear the
GPDMA_CxLLR of this LLIn+1), before that this LLIn+1 is loaded/used by the GPDMA
channel

•

abort and reconfigure the channel with a LSM update (see Section 17.4.4.)

In link step mode, the software can clear LSM after each a single execution of any LLI,
during LLIn-1.
Figure 66 shows the overall and unified GPDMA linked-list programming, whatever is the
execution mode.
Note:

Figure 66 is not intended to illustrate how often a TCEF can be raised, depending on the
programmed value of TCEM[1:0] in GPDMA_CxTR2. It can be raised at (each) block
completion, at (each) 2D block completion, at (each) LLI completion, or only at the last LLI
data transfer completion. In run-to-completion mode, whatever is the value of TCEM[1:0], at
the channel completion the hardware always set TCEF = 1 and disables the channel. In link
step mode, the channel is disabled after each single execution of a LLI, and depending on
the value of TCEM[1:0] a TCEF is raised or not.

RM0456 Rev 6

<!-- pagebreak -->

763

General purpose direct memory access controller (GPDMA)

RM0456

Figure 66. GPDMA channel execution and linked-list programming

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
Setting ULEF = 1
Disabling DMA channel

Y
Valid user
setting ?
Y
N

N
Setting USEF = 1
Disabling DMA channel

LSM = 1 ?
Y

Setting TCF = 1
Disabling DMA channel

End
MSv62638V1

<!-- pagebreak -->

