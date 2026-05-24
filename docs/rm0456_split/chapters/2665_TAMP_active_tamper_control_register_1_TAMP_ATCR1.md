2902

Universal synchronous/asynchronous receiver transmitter (USART/UART)

RM0456

Character reception
During an USART reception, data are shifted out least significant bit first (default
configuration) through the RX pin.
Character reception procedure
To receive a character, follow the sequence below:
1.

Program the M bits in USART_CR1 to define the word length.

2.

Select the desired baud rate using the baud rate register USART_BRR

3.

Program the number of stop bits in USART_CR2.

4.

Enable the USART by writing the UE bit in USART_CR1 register to 1.

5.

Select DMA enable (DMAR) in USART_CR3 if multibuffer communication is to take
place. Configure the DMA register as explained in Section 66.5.20: Continuous
communication using USART and DMA.

6.

Set the RE bit USART_CR1. This enables the receiver which begins searching for a
start bit.

When a character is received:
•

When FIFO mode is disabled, the RXNE bit is set to indicate that the content of the
shift register is transferred to the RDR. In other words, data have been received and
can be read (as well as their associated error flags).

•

When FIFO mode is enabled, the RXFNE bit is set to indicate that the RXFIFO is not
empty. Reading the USART_RDR returns the oldest data entered in the RXFIFO.
When a data is received, it is stored in the RXFIFO together with the corresponding
error bits.

•

An interrupt is generated if the RXNEIE (RXFNEIE when FIFO mode is enabled) bit is
set.

•

The error flags can be set if a frame error, noise, parity or an overrun error was
detected during reception.

•

In multibuffer communication mode:

•

<!-- pagebreak -->

–

When FIFO mode is disabled, the RXNE flag is set after every byte reception. It is
cleared when the DMA reads the Receive data Register.

–

When FIFO mode is enabled, the RXFNE flag is set when the RXFIFO is not
empty. After every DMA request, a data is retrieved from the RXFIFO. A DMA
request is triggered when the RXFIFO is not empty, that is when there are data to
be read from the RXFIFO.

In single-buffer mode:
–

When FIFO mode is disabled, clearing the RXNE flag is done by performing a
software read from the USART_RDR register. The RXNE flag can also be cleared
by programming RXFRQ bit to 1 in the USART_RQR register. The RXNE flag
must be cleared before the end of the reception of the next character to avoid an
overrun error.

–

When FIFO mode is enabled, the RXFNE is set when the RXFIFO is not empty.
After every read operation from USART_RDR, a data is retrieved from the
RXFIFO. When the RXFIFO is empty, the RXFNE flag is cleared. The RXFNE flag
can also be cleared by programming RXFRQ bit to 1 in USART_RQR. When the
RXFIFO is full, the first entry in the RXFIFO must be read before the end of the
reception of the next character, to avoid an overrun error. The RXFNE flag
generates an interrupt if the RXFNEIE bit is set. Alternatively, interrupts can be

RM0456 Rev 6

RM0456

Universal synchronous/asynchronous receiver transmitter (USART/UART)
generated and data can be read from RXFIFO when the RXFIFO threshold is
reached. In this case, the CPU can read a block of data defined by the
programmed threshold.

Break character
When a break character is received, the USART handles it as a framing error.

Idle character
When an idle frame is detected, it is handled in the same way as a data character reception
except that an interrupt is generated if the IDLEIE bit is set.

Overrun error
•

FIFO mode disabled
An overrun error occurs if a character is received and RXNE has not been reset.
Data can not be transferred from the shift register to the RDR register until the RXNE
bit is cleared. The RXN E flag is set after every byte reception.
An overrun error occurs if RXNE flag is set when the next data is received or the
previous DMA request has not been serviced. When an overrun error occurs:

•

–

the ORE bit is set;

–

the RDR content is not lost. The previous data is available by reading the
USART_RDR register.

–

the shift register is overwritten. After that, any data received during overrun is lost.

–

an interrupt is generated if either the RXNEIE or the EIE bit is set.

FIFO mode enabled
An overrun error occurs when the shift register is ready to be transferred and the
receive FIFO is full.
Data can not be transferred from the shift register to the USART_RDR register until
there is one free location in the RXFIFO. The RXFNE flag is set when the RXFIFO is
not empty.
An overrun error occurs if the RXFIFO is full and the shift register is ready to be
transferred. When an overrun error occurs:
–

The ORE bit is set.

–

The first entry in the RXFIFO is not lost. It is available by reading the
USART_RDR register.

–

The shift register is overwritten. After that point, any data received during overrun
is lost.

–

An interrupt is generated if either the RXFNEIE or EIE bit is set.

The ORE bit is reset by setting the ORECF bit in the USART_ICR register.
Note:

The ORE bit, when set, indicates that at least 1 data has been lost.
When the FIFO mode is disabled, there are two possibilities

•

if RXNE = 1, then the last valid data is stored in the receive register (RDR) and can be
read,

•

if RXNE = 0, the last valid data has already been read and there is nothing left to be
read in the RDR register. This case can occur when the last valid data is read in the
RDR register at the same time as the new (and lost) data is received.

RM0456 Rev 6

<!-- pagebreak -->

