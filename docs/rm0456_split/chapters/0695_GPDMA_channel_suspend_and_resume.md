RM0456 Rev 6

RM0456

General purpose direct memory access controller (GPDMA)

Figure 52. GPDMA channel direct programming without linked-list (GPDMA_CxLLR = 0)

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

17.4.3

GPDMA channel suspend and resume
The software can suspend on its own a channel still active, with the following sequence:

Note:

1.

The software writes 1 into the GPDMA_CxCR.SUSP bit.

2.

The software polls the suspended flag GPDMA_CxSR.SUSPF until SUSPF = 1, or
waits for an interrupt previously enabled by writing 1 to GPDMA_CxCR.SUSPIE. Wait
for the channel to be effectively in suspended state means wait for the completion of
any ongoing GPDMA transfer over its master ports. Then the software can observe, in
a steady state, any read register or register field that is hardware modifiable.

An ongoing GPDMA transfer can be a data transfer (a source/destination burst transfer) or a
link transfer for the internal update of the linked-list register file from the next linked-list item.

RM0456 Rev 6

<!-- pagebreak -->

