1.

The software writes 1 into the LPDMA_CxCR.SUSP bit.

2.

The software polls the suspended flag LPDMA_CxSR.SUSPF until SUSPF = 1, or
waits for an interrupt previously enabled by writing 1 to LPDMA_CxCR.SUSPIE. Wait
for the channel to be effectively in suspended state means wait for the completion of

RM0456 Rev 6

RM0456

Low-power direct memory access controller (LPDMA)
any ongoing LPDMA transfer over its master port. Then the software can observe, in a
steady state, any read register or register field that is hardware modifiable.
Note that an ongoing LPDMA transfer can be a data transfer (a read-followed-by-write
single transfer) or a link transfer for the internal update of the linked-list register file
from the next linked-list item.
3.

The software safely resumes the suspended channel by writing 0 to
LPDMA_CxCR.SUSP.

The suspend and resume sequence is illustrated in the figure below.
Figure 74. LPDMA channel suspend and resume sequence

Channel state = Active
Suspend the DMA channel
(write 1 to CxCR.SUSP)
or

SUSPF=1 ?

N

Channel state = Suspended and Idle Y
Receiving
suspended
interrupt

Resume the DMA channel
(write 0 to CxCR.SUSP)

Channel state = Active
MSv62627V1

Note:

A suspend and resume sequence does not impact the LPDMA_CxCR.EN bit. Suspending a
channel (transfer) does not suspend a started trigger detection.

18.4.4

LPDMA channel abort and restart
Alternatively, like for aborting a continuous LPDMA transfer with a circular buffering or a
double buffering, the software can abort, on its own, a still active channel with the following
sequence:
1.

The software writes 1 into the LPDMA_CxCR.SUSP bit.

2.

The software polls suspended flag LPDMA_CxSR.SUSPF until SUSPF = 1, or waits for
an interrupt previously enabled by writing 1 to LPDMA_CxCR.SUSPIE. Wait for the
channel to be effectively in suspended state means wait for the completion of any
ongoing LPDMA transfer over its master port.

3.

The software resets the channel by writing 1 to LPDMA_CxCR.RESET. This causes
the reset of the channel internal state, the reset of the LPDMA_CxCR.EN bit, and the
reset of the LPDMA_CxCR.SUSP bit.

RM0456 Rev 6

<!-- pagebreak -->

