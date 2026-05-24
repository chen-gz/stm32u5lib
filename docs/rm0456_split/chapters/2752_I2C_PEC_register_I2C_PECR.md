RM0456 Rev 6

RM0456

Low-power universal asynchronous receiver transmitter (LPUART)
LPUART_TDR register stores the data in the TXFIFO. Data are copied from the
TXFIFO to the shift register at the end of the current transmission.
When the TXFIFO is not full, the TXFNF flag stays at 1 even after a write in
LPUART_TDR. It is cleared when the TXFIFO is full. This flag generates an interrupt if
the TXFNEIE bit is set.
Alternatively, interrupts can be generated and data can be written to the TXFIFO when
the TXFIFO threshold is reached. In this case, the CPU can write a block of data
defined by the programmed threshold.
If a frame is transmitted (after the stop bit) and the TXE flag (TXFE in case of FIFO
mode) is set, the TC bit goes high. An interrupt is generated if the TCIE bit is set in the
LPUART_CR1 register.
After writing the last data in the LPUART_TDR register, it is mandatory to wait for
TC = 1 before disabling the LPUART or causing the microcontroller to enter the lowpower mode (see Figure 837: TC/TXE behavior when transmitting).
Figure 837. TC/TXE behavior when transmitting
Idle preamble

Frame 1

Frame 2

Set by hardware
cleared by software

Set by hardware
cleared by software

F2

F3

Frame 3

TX line
TXE flag

LPUART_DR

F1

Set by hardware

Set by hardware
TC flag
Software
enables the
LPUART

Software waits until TXE=1
and writes F2 into DR

Software waits until TXE=1
and writes F1 into DR

TC is not set
because TXE=0

Software waits until
TXE=1 and writes
F3 into DR

TC is not set
because TXE=0

TC is set
because TXE=1

Software waits until TC=1

MSv31889V1

Note:

When FIFO management is enabled, the TXFNF flag is used for data transmission.

Break characters
Setting the SBKRQ bit transmits a break character. The break frame length depends on the
M bits (see Figure 835).
If a 1 is written to the SBKRQ bit, a break character is sent on the TX line after completing
the current character transmission. The SBKF bit is set by the write operation and it is reset
by hardware when the break character is complete (during the stop bits after the break
character). The LPUART inserts a logic 1 signal (STOP) for the duration of 2 bits at the end
of the break frame to guarantee the recognition of the start bit of the next frame.
When the SBKRQ bit is set, the break character is sent at the end of the current
transmission.

RM0456 Rev 6

<!-- pagebreak -->

2902

Low-power universal asynchronous receiver transmitter (LPUART)

RM0456

When FIFO mode is enabled, sending the break character has priority on sending data even
if the TXFIFO is full.

Idle characters
Setting the TE bit drives the LPUART to send an idle frame before the first data frame.

67.4.7

LPUART receiver
The LPUART can receive data words of either 7 or 8 or 9 bits depending on the M bits in the
LPUART_CR1 register.

Start bit detection
In the LPUART, the start bit is detected when a falling edge occurs on the Rx line, followed
by a sample taken in the middle of the start bit to confirm that it is still 0. If the start sample is
at 1, then the noise error flag (NE) is set, then the start bit is discarded and the receiver
waits for a new start bit. Else, the receiver continues to sample all incoming bits normally.

Character reception
During an LPUART reception, data are shifted with the least significant bit first (default
configuration) through the RX pin. In this mode, the LPUART_RDR register consists of a
buffer (RDR) between the internal bus and the received shift register.
Character reception procedure
To receive a character, follow the sequence below:
1.

Program the M bits in LPUART_CR1 to define the word length.

2.

Select the desired baud rate using the baud rate register LPUART_BRR

3.

Program the number of stop bits in LPUART_CR2.

4.

Enable the LPUART by writing the UE bit in LPUART_CR1 register to 1.

5.

Select DMA enable (DMAR) in LPUART_CR3 if multibuffer communication is to take
place. Configure the DMA register as explained in Section 67.4.13: Continuous
communication using DMA and LPUART.

6.

Set the RE bit LPUART_CR1. This enables the receiver, which begins searching for a
start bit.

When a character is received

<!-- pagebreak -->

•

When FIFO mode is disabled, the RXNE bit is set. It indicates that the content of the
shift register is transferred to the RDR. In other words, data has been received and can
be read (as well as its associated error flags).

•

When FIFO mode is enabled, the RXFNE bit is set indicating that the RXFIFO is not
empty. Reading the LPUART_RDR returns the oldest data entered in the RXFIFO.

RM0456 Rev 6

RM0456

Low-power universal asynchronous receiver transmitter (LPUART)
When a data is received, it is stored in the RXFIFO, together with the corresponding
error bits.
•

An interrupt is generated if the RXNEIE (RXFNEIE in case of FIFO mode) bit is set.

•

The error flags can be set if a frame error, noise, or an overrun error has been detected
during reception.

•

In multibuffer communication mode:

•

–

When FIFO mode is disabled, the RXNE flag is set after every byte received and
is cleared by the DMA read of the receive data register.

–

When FIFO mode is enabled, the RXFNE flag is set when the RXFIFO is not
empty. After every DMA request, a data is retrieved from the RXFIFO. DMA
request is triggered by RXFIFO is not empty, that is, there is a data in the RXFIFO
to be read.

In single-buffer mode:
–

When FIFO mode is disabled, clearing the RXNE flag is done by performing a
software read from the LPUART_RDR register. The RXNE flag can also be
cleared by writing 1 to the RXFRQ in the LPUART_RQR register. The RXNE bit
must be cleared before the end of the reception of the next character to avoid an
overrun error.

–

When FIFO mode is enabled, the RXFNE flag is set when the RXFIFO is not
empty. After every read operation from the LPUART_RDR register, a data is
retrieved from the RXFIFO. When the RXFIFO is empty, the RXFNE flag is
cleared. The RXFNE flag can also be cleared by writing 1 to the RXFRQ bit in the
LPUART_RQR register. When the RXFIFO is full, the first entry in the RXFIFO
must be read before the end of the reception of the next character to avoid an
overrun error. The RXFNE flag generates an interrupt if the RXFNEIE bit is set.
Alternatively, interrupts can be generated and data can be read from RXFIFO
when the RXFIFO threshold is reached. In this case, the CPU can read a block of
data defined by the programmed threshold.

Break character
When a break character is received, the LPUART handles it as a framing error.

Idle character
When an idle frame is detected, it is handled in the same way as a data character reception
except that an interrupt is generated if the IDLEIE bit is set.

RM0456 Rev 6

<!-- pagebreak -->

2902

Low-power universal asynchronous receiver transmitter (LPUART)

RM0456

Overrun error
•

FIFO mode disabled
An overrun error occurs when a character is received when RXNE has not been reset.
Data cannot be transferred from the shift register to the RDR register until the RXNE bit
is cleared. The RXNE flag is set after every byte received.
An overrun error occurs if the RXNE flag is set when the next data is received or the
previous DMA request has not been serviced. When an overrun error occurs:

•

–

The ORE bit is set;

–

The RDR content is not lost. The previous data is available when a read to
LPUART_RDR is performed.

–

The shift register is overwritten. After that, any data received during overrun is lost.

–

An interrupt is generated if either the RXNEIE bit or EIE bit is set.

FIFO mode enabled
An overrun error occurs when the shift register is ready to be transferred when the
receive FIFO is full.
Data cannot be transferred from the shift register to the LPUART_RDR register until
there is one free location in the RXFIFO. The RXFNE flag is set when the RXFIFO is
not empty.
An overrun error occurs if the RXFIFO is full and the shift register is ready to be
transferred. When an overrun error occurs:
–

The ORE bit is set;

–

The first entry in the RXFIFO is not lost. It is available when a read to
LPUART_RDR is performed.

–

The shift register is overwritten. After that, any data received during overrun is lost.

–

An interrupt is generated if either the RXFNEIE bit or EIE bit is set.

The ORE bit is reset by setting the ORECF bit in the ICR register.
Note:

The ORE bit, when set, indicates that at least 1 data has been lost. T
When the FIFO mode is disabled, there are two possibilities

•

If RXNE = 1, then the last valid data is stored in the receive register (RDR) and can be
read,

•

If RXNE = 0, then the last valid data has already been read and there is nothing left to
be read in the RDR. This case can occur when the last valid data is read in the RDR at
the same time as the new (and lost) data is received.

Selecting the clock source
The choice of the clock source is done through the clock control system (see Section Reset
and clock controller (RCC)). The clock source must be selected through the UE bit, before
enabling the LPUART.
The clock source must be selected according to two criteria:
•

Possible use of the LPUART in low-power mode

•

Communication speed.

The clock source frequency is lpuart_ker_ck.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Low-power universal asynchronous receiver transmitter (LPUART)
When the dual clock domain and the wake-up from low-power mode features are supported,
the lpuart_ker_ck clock source can be configured in the RCC (see Section Reset and clock
controller (RCC)). Otherwise, the lpuart_ker_ck is the same as lpuart_pclk.
The lpuart_ker_ck can be divided by a programmable factor in the LPUART_PRESC
register.
Figure 838. lpuart_ker_ck clock divider block diagram

lpuart_ker_ck_pres
lpuart_ker_ck

LPUARTx_PRESC[3:0]

LPUARTx_BRR
register and
oversampling

MSv40859V1

Some lpuart_ker_ck sources enable the LPUART to receive data while the MCU is in lowpower mode. Depending on the received data and wake-up mode selection, the LPUART
wakes up the MCU, when needed, in order to transfer the received data by software reading
the LPUART_RDR register or by DMA.
For the other clock sources, the system must be active to enable LPUART communications.
The communication speed range (specially the maximum communication speed) is also
determined by the clock source.
The receiver samples each incoming bit as close as possible to the middle of the bit-period.
Only a single sample is taken of each of the incoming bits.
Note:

There is no noise detection for data.

Framing error
A framing error is detected when the stop bit is not recognized on reception at the expected
time, following either a desynchronization or excessive noise.
When the framing error is detected:
•

The FE bit is set by hardware.

•

The invalid data is transferred from the Shift register to the LPUART_RDR register.

•

No interrupt is generated in case of single byte communication. However, this bit rises
at the same time as the RXNE bit which itself generates an interrupt. In case of
multibuffer communication, an interrupt is issued if the EIE bit is set in the
LPUART_CR3 register.

The FE bit is reset by writing 1 to the FECF in the LPUART_ICR register.

RM0456 Rev 6

<!-- pagebreak -->

