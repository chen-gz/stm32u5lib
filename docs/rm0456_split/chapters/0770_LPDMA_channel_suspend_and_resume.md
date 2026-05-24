821

Low-power direct memory access controller (LPDMA)

RM0456

The figure below illustrates this LPDMA direct programming without any linked-list
(LPDMA_CxLLR = 0).
Figure 73. LPDMA channel direct programming without linked-list (LPDMA_CxLLR = 0)

Channel state = Idle
Initialize DMA channel
(keeping DMA_CxLLR[31:0] = 0)

Reconfigure DMA channel
(keeping DMA_CxLLR[31:0] = 0)

Enable DMA channel

Channel state = Active
Valid user
setting ?

N

Y
Setting USEF = 1
Disabling DMA channel
Executing the data transfer
from the register file

No transfer
error ?

N

Y

Setting DTEF = 1
Disabling DMA channel

Setting TCF = 1
Disabling DMA channel

End

MSv62626V1

18.4.3

LPDMA channel suspend and resume
The software can suspend on its own a channel still active, with the following sequence:

<!-- pagebreak -->

