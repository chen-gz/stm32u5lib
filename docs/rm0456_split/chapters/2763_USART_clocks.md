2902

Low-power universal asynchronous receiver transmitter (LPUART)

RM0456

1.

Write the LPUART_TDR register address in the DMA control register to configure it as
the destination of the transfer. The data is moved to this address from memory after
each TXE (or TXFNF if FIFO mode is enabled) event.

2.

Write the memory address in the DMA control register to configure it as the source of
the transfer. The data is loaded into the LPUART_TDR register from this memory area
after each TXE (or TXFNF if FIFO mode is enabled) event.

3.

Configure the total number of bytes to be transferred to the DMA control register.

4.

Configure the channel priority in the DMA register

5.

Configure DMA interrupt generation after half/ full transfer as required by the
application.

6.

Clear the TC flag in the LPUART_ISR register by setting the TCCF bit in the
LPUART_ICR register.

7.

Activate the channel in the DMA register.

When the number of data transfers programmed in the DMA controller is reached, the DMA
controller generates an interrupt on the DMA channel interrupt vector.
In transmission mode, once the DMA has written all the data to be transmitted (DMA
transfer complete), the TC flag can be monitored to make sure that the LPUART
communication has completed. This is required to avoid corrupting the last transmission
before disabling the LPUART or entering low-power mode. Software must wait until TC = 1.
The TC flag remains cleared during all data transfers and it is set by hardware at the end of
transmission of the last frame.
Note:

The DMAT bit must not be cleared before the DMA end of transfer.
Figure 841. Transmission using DMA
Idle preamble

Frame 2

Frame 1

Frame 3

TX line
Set by hardware
cleared by DMA write

TXE flag

Set by hardware
cleared by DMA write

Set by hardware
Ignored by the DMA because the
transfer is complete

DMA request

a

LPUART_TDR

F1

F2

F3
Set by
hardware

TC flag
DMA writes
LPUART_TDR

Cleared by software

DMA transfer
complete flag
Software
configures DMA
to send 3 data
blocks and
enables
LPUART

Set by hardware

DMA writes
F1 into
LPUART_
TDR

DMA writes
F2 into
LPUART_
TDR

DMA writes
F3 into
LPUART_
TDR

The DMA
transfer is
complete

Software waits until TC = 1

MSv31890V4

Note:

<!-- pagebreak -->

When FIFO management is enabled, the DMA request is triggered by transmit FIFO not full
(that is, TXFNF = 1).

RM0456 Rev 6

RM0456

Low-power universal asynchronous receiver transmitter (LPUART)

Reception using DMA
DMA mode can be enabled for reception by setting the DMAR bit in the LPUART_CR3
register. Data are loaded from the LPUART_RDR register to a SRAM area configured using
the DMA peripheral (refer to section direct memory access controller (DMA)) whenever a
data byte is received. To map a DMA channel for LPUART reception, use the following
procedure:
1.

Write the LPUART_RDR register address in the DMA control register to configure it as
the source of the transfer. The data is moved from this address to the memory after
each RXNE (RXFNE in case FIFO mode is enabled) event.

2.

Write the memory address in the DMA control register to configure it as the destination
of the transfer. The data is loaded from LPUART_RDR to this memory area after each
RXNE (RXFNE in case FIFO mode is enabled) event.

3.

Configure the total number of bytes to be transferred to the DMA control register.

4.

Configure the channel priority in the DMA control register

5.

Configure interrupt generation after half/ full transfer as required by the application.

6.

Activate the channel in the DMA control register.

When the number of data transfers programmed in the DMA controller is reached, the DMA
controller generates an interrupt on the DMA channel interrupt vector.
Note:

The DMAR bit must not be cleared before the DMA end of transfer.
Figure 842. Reception using DMA
Frame 2

Frame 1

Frame 3

RX line
Set by hardware
cleared by DMA read

RXNE flag
DMA request

F3

F2

F1

LPUART_RDR
DMA reads
LPUART_RDR
DMA transfer
complete flag

Software configures the
DMA to receive 3
datablocks and enables
the LPUART

Set by hardware

DMA reads F1
from
LPUART_RDR

DMA reads F2
from
LPUART_RDR

DMA reads F3
from
LPUART_RDR

Cleared by
software

DMA transfer is
complete

MSv31891V5

Note:

When FIFO management is enabled, the DMA request is triggered by receive FIFO not
empty (that is, RXFNE = 1).

Error flagging and interrupt generation in multibuffer communication
If any error occurs during a transaction In multibuffer communication mode, the error flag is
asserted after the current byte. An interrupt is generated if the interrupt enable flag is set.

RM0456 Rev 6

<!-- pagebreak -->

