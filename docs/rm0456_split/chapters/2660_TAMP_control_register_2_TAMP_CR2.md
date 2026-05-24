RM0456 Rev 6

RM0456

66.5.5

Universal synchronous/asynchronous receiver transmitter (USART/UART)

USART FIFOs and thresholds
The USART can operate in FIFO mode.
The USART comes with a Transmit FIFO (TXFIFO) and a Receive FIFO (RXFIFO). The
FIFO mode is enabled by setting FIFOEN in USART_CR1 register (bit 29). This mode is
supported only in UART, SPI and smartcard modes.
Since the maximum data word length is 9 bits, the TXFIFO is 9-bit wide. However the
RXFIFO default width is 12 bits. This is due to the fact that the receiver does not only store
the data in the FIFO, but also the error flags associated to each character (Parity error,
Noise error and Framing error flags).

Note:

The received data is stored in the RXFIFO together with the corresponding flags. However,
only the data are read when reading the RDR.
The status flags are available in the USART_ISR register.
It is possible to configure the TXFIFO and RXFIFO levels at which the Tx and RX interrupts
are triggered. These thresholds are programmed through the RXFTCFG[2:0] and
TXFTCFG[2:0] bitfields of the USART_CR3 control register.
In this case:
•

The Rx interrupt is generated when the number of received data in the RXFIFO
reaches the threshold programmed in the RXFTCFG[2:0] bitfield.
In this case, the RXFT flag is set in the USART_ISR register. This means that
RXFTCFG[2:0] data have been received: 1 data in USART_RDR and
(RXFTCFG[2:0] – 1) data in the RXFIFO. As an example, when RXFTCFG[2:0] is
programmed to 101, the RXFT flag is set when a number of data corresponding to the
FIFO size has been received (FIFO size – 1 data in the RXFIFO and 1 data in the
USART_RDR). As a result, the next received data does not set the overrun flag.

•

66.5.6

The Tx interrupt is generated when the number of empty locations in the TXFIFO is
greater than the threshold programmed in the TXFTCFG[2:0] bitfield of the
USART_CR3 register.

USART transmitter
The transmitter can send data words of either 7 or 8 or 9 bits, depending on the M bit status.
The Transmit Enable bit (TE) must be set in order to activate the transmitter function. The
data in the transmit shift register is output on the TX pin while the corresponding clock
pulses are output on the CK pin.

Character transmission
During an USART transmission, data shifts out the least significant bit first (default
configuration) on the TX pin. In this mode, the USART_TDR register consists of a buffer
(TDR) between the internal bus and the transmit shift register.
When FIFO mode is enabled, the data written to the transmit data register (USART_TDR)
are queued in the TXFIFO.
Every character is preceded by a start bit which corresponds to a low logic level for one bit
period. The character is terminated by a configurable number of stop bits.
The number of stop bits can be configured to 0.5, 1, 1.5 or 2.

RM0456 Rev 6

<!-- pagebreak -->

2902

Universal synchronous/asynchronous receiver transmitter (USART/UART)
Note:

RM0456

The TE bit must be set before writing the data to be transmitted to the USART_TDR.
The TE bit must not be reset during data transmission. Resetting the TE bit during the
transmission corrupts the data on the TX pin as the baud rate counters get frozen. The
current data being transmitted are then lost.
An idle frame is sent when the TE bit is enabled.
Configurable stop bits
The number of stop bits to be transmitted with every character can be programmed in
USART_CR2, bits 13,12.
•

1 stop bit: This is the default value of number of stop bits.

•

2 stop bits: This is supported by normal USART, single-wire and modem modes.

•

1.5 stop bits: To be used in smartcard mode.

An idle frame transmission includes the stop bits.
A break transmission is 10 low bits (when M[1:0] = 00) or 11 low bits (when M[1:0] = 01) or 9
low bits (when M[1:0] = 10) followed by 2 stop bits (see Section 66.5.1: USART block
diagram). It is not possible to transmit long breaks (break of length greater than 9/10/11 low
bits).
Figure 811. Configurable stop bits
8-bit data, 1 Stop bit
Possible
parity bit

Data frame
Start bit

Bit0

Bit1

Bit2

Bit3

Bit4

Bit5

Bit6

CLOCK

Bit7

Stop
bit

Next
start
bit

Next data frame

**
** LBCL bit controls last data clock pulse

8-bit data, 1 1/2 Stop bits
Possible
parity bit

Data frame
Start bit

Bit0

Bit1

Bit2

Bit3

Bit4

Bit5

Bit6

Bit7

1.5
Stop
bits

Next
start
bit

Next data frame

8-bit data, 2 Stop bits
Possible
parity bit

Data frame
Start bit

Bit0

Bit1

Bit2

Bit3

Bit4

Bit5

Bit6

Bit7

2
Stop
bits

Next
start
bit

Next data frame

MSv31887V1

Character transmission procedure
To transmit a character, follow the sequence below:

<!-- pagebreak -->

