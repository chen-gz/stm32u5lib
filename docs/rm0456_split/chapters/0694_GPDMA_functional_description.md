763

General purpose direct memory access controller (GPDMA)

17.4

GPDMA functional description

17.4.1

GPDMA block diagram

RM0456

Figure 51. GPDMA block diagram

Channel datapath and
transfer input control
DMA
requests

Data transfer
generation

Channel x (1)
...

AHB master
port 0
interface

Arbitration

Channel 1
Link transfer
generation

Channel 0

DMA
clock

AHB master
port 1
interface

Interrupt generation
DMA channel registers

Stop DMA
channel in
debug mode

Events generation
Channel x (1)
DMA global
registers

Channel state
management

...
Channel 1
Channel 0

AHB slave interface

Security and privilege
management

32-bit AHB bus

DMA
triggers

Transfer output control

32-bit AHB bus

GPDMA

DMA channel
interrupt
DMA channel
transfer complete
(gpdma_chx_tc)
DMA channel state
(vs privilege
and security)
DMA illegal event
(vs security)

Clock management
DMA clock request

(1) Refer to the device implementation table for the number of channels.

32-bit AHB bus

17.4.2

MSv63644V2

GPDMA channel state and direct programming without any linked-list
After a GPDMA reset, a GPDMA channel x is in idle state. When the software writes 1 in
GPDMA_CxCR.EN, the channel takes into account the value of the different channel
configuration registers (GPDMA_CxXXX), switches to the active/non-idle state and starts to
execute the corresponding requested data transfers.
After enabling/starting a GPDMA channel transfer by writing 1 in GPDMA_CxCR.EN, a
GPDMA channel interrupt on a complete transfer notifies the software that the GPDMA
channel is back in idle state (EN is then deasserted by hardware) and that the channel is
ready to be reconfigured then enabled again.
Figure 52 illustrates this GPDMA direct programming without any linked-list
(GPDMA_CxLLR = 0).

<!-- pagebreak -->

