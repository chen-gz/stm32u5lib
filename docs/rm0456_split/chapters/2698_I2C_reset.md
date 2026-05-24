RM0456 Rev 6

RM0456

Universal synchronous/asynchronous receiver transmitter (USART/UART)

USART transmission mode
In transmission, the APB clock is requested only when the TE bit is set and in the following
cases:
•
If the FIFO mode is enabled, the APB clock is requested when

•

–

The TxFIFO is empty (TXFE = 1) and the corresponding interrupt is enabled
(TXFEIE = 1)

–

The TxFIFO threshold is reached (TXFT = 1) and the corresponding interrupt is
enabled (TXFTIE = 1)

–

The TxFIFO is not full (TXFNF = 1) and the corresponding interrupt or DMA is
enabled (TXFNFIE = 1 or DMAT = 1)

If the FIFO mode is disabled, the APB clock is requested as soon as data are
transferred to the shift register. The DMA or associated interrupt must be enabled.

The TE bit is set by hardware if an asynchronous trigger is detected.
A transmission is automatically launched when an asynchronous trigger is detected in Run,
Sleep, or Stop mode. The trigger is selected through the TRIGSEL bit in the
USART_AUTOCR register. It sets the TE bit in the USART_CR1 register and generates an
APB clock request to enable the transfer. The APB clock is requested until the transmission
completes and the TE bit is cleared by hardware when the programmed number of data to
be transmitted (TDN bitfield in the USART_AUTOCR register) is reached. In this case, the
TC flag is set when the number of data to be transmitted is reached and the last byte is
transmitted.

USART reception mode
•

If the FIFO mode is enabled, the APB clock is requested when
–

The RxFIFIO is full (RXFF = 1) and the corresponding interrupt is enabled
(RXFFIE = 1)

–

The RxFIFO threshold is reached (RXFT = 1) and the corresponding interrupt is
enabled (RXFTIE = 1)

–
•

Note:

The RxFIFO is not empty (RXFNE = 1) and the corresponding interrupt or DMA is
enabled (RXFNEIE= 1 or DMAR = 1)
If the FIFO mode is disabled, the APB clock is requested when the USART finishes
sampling data and it is ready to be written in the USART_RDR. The DMA or the
associated interrupt must be enabled.

The APB clock is requested in reception mode when an overrun error occurs (ORE = 1).
The EIE bit must be set to enable the generation of an interrupt and waking up the MCU,
and the OVRDIS bit must remain cleared. The APB clock request is kept until the interrupt
flag is cleared.
The APB clock is also requested in reception mode when a Parity/Noise/Framing error
occurs and the DMA is used for reception. The APB clock request is kept until the interrupt
flag is cleared.
Only UART and SPI master modes support the autonomous mode.

RM0456 Rev 6

<!-- pagebreak -->

