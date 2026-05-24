2902

Universal synchronous/asynchronous receiver transmitter (USART/UART)

66.5.20

RM0456

Continuous communication using USART and DMA
The USART is capable of performing continuous communications using the DMA. The DMA
requests for Rx buffer and Tx buffer are generated independently.

Note:

Refer to Section 66.4: USART implementation to determine if the DMA mode is supported. If
DMA is not supported, use the USART as explained in Section 66.5.7. To perform
continuous communications when the FIFO is disabled, clear the TXE/ RXNE flags in the
USART_ISR register.

Transmission using DMA
DMA mode can be enabled for transmission by setting the DMAT bit in the USART_CR3
register. Data are loaded from an SRAM area configured using the DMA peripheral (refer to
section direct memory access controller (DMA)) to the USART_TDR register whenever the
TXE flag (TXFNF flag if FIFO mode is enabled) is set. To map a DMA channel for USART
transmission, use the following procedure (x denotes the channel number):
1.

Write the USART_TDR register address in the DMA control register to configure it as
the destination of the transfer. The data is moved to this address from memory after
each TXE (or TXFNF if FIFO mode is enabled) event.

2.

Write the memory address in the DMA control register to configure it as the source of
the transfer. The data is loaded into the USART_TDR register from this memory area
after each TXE (or TXFNF if FIFO mode is enabled) event.

3.

Configure the total number of bytes to be transferred to the DMA control register.

4.

Configure the channel priority in the DMA register

5.

Configure DMA interrupt generation after half/ full transfer as required by the
application.

6.

Clear the TC flag in the USART_ISR register by setting the TCCF bit in the
USART_ICR register.

7.

Activate the channel in the DMA register.

When the number of data transfers programmed in the DMA controller is reached, the DMA
controller generates an interrupt on the DMA channel interrupt vector.
In transmission mode, once the DMA has written all the data to be transmitted (DMA
transfer complete), the TC flag can be monitored to make sure that the USART
communication has completed. This is required to avoid corrupting the last transmission
before disabling the USART or before the system enters a low-power mode when the
peripheral clock is disabled. Software must wait until TC = 1. The TC flag remains cleared
during all data transfers and it is set by hardware at the end of transmission of the last
frame.
Note:

<!-- pagebreak -->

