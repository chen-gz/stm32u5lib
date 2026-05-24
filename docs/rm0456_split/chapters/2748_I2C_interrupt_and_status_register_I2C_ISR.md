RM0456 Rev 6

RM0456

Low-power universal asynchronous receiver transmitter (LPUART)
Figure 835. LPUART word length programming
9-bit word length (M = 01 ), 1 Stop bit
Possible
Parity
bit

Data frame
Start
bit

Bit0

Bit1

Bit2

Bit3

Bit4

Bit5

Bit6

Bit7

Clock

Bit8

Stop
bit

Next
Start
bit

**
Start
bit

Idle frame

Stop
bit

Stop
bit

Stop
bit

Stop
bit

Start
bit

Stop
bit

Start
bit

Break frame

Start
bit

8-bit word length (M = 00 ), 1 Stop bit
Possible
Parity
bit

Data frame
Start
bit

Bit0

Bit1

Bit2

Bit3

Bit4

Bit5

Bit6

Bit7

Clock

Stop
bit

Next
Start
bit

**
Start
bit

Idle frame
Break frame

7-bit word length (M = 10 ), 1 Stop bit
Possible
Parity
bit

Data frame
Start
bit

Bit0

Bit1

Bit2

Bit3

Bit4

Bit5

Bit6

Stop
bit

Next
Start
bit

**

Clock

Idle frame
Break frame

Start
bit
Stop
bit

** LBCL bit controls last data clock pulse
MS33194V2

RM0456 Rev 6

<!-- pagebreak -->

2902

Low-power universal asynchronous receiver transmitter (LPUART)

67.4.5

RM0456

LPUART FIFOs and thresholds
The LPUART can operate in FIFO mode.
The LPUART comes with a transmit FIFO (TXFIFO) and a Receive FIFO (RXFIFO). The
FIFO mode is enabled by setting FIFOEN bit (bit 29) in the LPUART_CR1 register.
Since 9 bits the maximum data word length is 9 bits, the TXFIFO is 9-bit wide. However, the
RXFIFO default width is 12 bits. This is due to the fact that the receiver does not only store
the data in the FIFO, but also the error flags associated to each character (Parity error,
Noise error and Framing error flags).

Note:

The received data is stored in the RXFIFO together with the corresponding flags. However,
only the data are read when reading the RDR.
The status flags are available in the LPUART_ISR register.
It is possible to define the TXFIFO and RXFIFO levels at which the Tx and RX interrupts are
triggered. These thresholds are programmed through the RXFTCFG[2:0] and
TXFTCFG[2:0] bitfields of the LPUART_CR3 control register.
In this case:
•

The Rx interrupt is generated when the number of received data in the RXFIFO
reaches the threshold programmed in the RXFTCFG[2:0] bitfield.
In this case, the RXFT flag is set in the LPUART_ISR register. This means that
RXFTCFG[2:0] data have been received: 1 data in LPUART_RDR and
(RXFTCFG[2:0] – 1) data in the RXFIFO. As an example, when the RXFTCFG[2:0] is
programmed to 101, the RXFT flag is set when a number of data corresponding to the
FIFO size has been received: FIFO size – 1 data in the RXFIFO and 1 data in the
LPUART_RDR. As a result, the next received data does not set the overrun flag.

•

67.4.6

The Tx interrupt is generated when the number of empty locations in the TXFIFO is
greater than the threshold programmed in the TXFTCFG[2:0] bitfield of the
LSART_CR3 register.

LPUART transmitter
The transmitter can send data words of either 7 or 8 or 9 bits, depending on the M bit status.
The transmit enable bit (TE) must be set in order to activate the transmitter function. The
data in the transmit shift register is output on the TX pin.

Character transmission
During an LPUART transmission, data shifts out least significant bit first (default
configuration) on the TX pin. In this mode, the LPUART_TDR register consists of a buffer
(TDR) between the internal bus and the transmit shift register (see Section 67.4.1: LPUART
block diagram).
When FIFO mode is enabled, the data written to the LPUART_TDR register are queued in
the TXFIFO.
Every character is preceded by a start bit bit, which corresponds to a low logic level for one
bit period. The character is terminated by a configurable number of stop bits.
The number of stop bits can be 1 or 2.

<!-- pagebreak -->

RM0456 Rev 6

RM0456
Note:

Low-power universal asynchronous receiver transmitter (LPUART)
The TE bit must be set before writing the data to be transmitted to the LPUART_TDR.
The TE bit must not be reset during data transmission. Resetting the TE bit during the
transmission corrupts the data on the TX pin as the baud rate counters is frozen. The
current data being transmitted are lost.
An idle frame is sent after the TE bit is enabled.
Configurable stop bits
The number of stop bits to be transmitted with every character can be programmed in
LPUART_CR2 (bits 13, 12).
•

1 stop bit: This is the default value of the number of stop bits.

•

2 stop bits: This is supported by normal LPUART, single-wire, and modem modes.

An idle frame transmission includes the stop bits.
A break transmission is 10 low bits (when M[1:0] = 00) or 11 low bits (when M[1:0] = 01) or 9
low bits (when M[1:0] = 10) followed by 2 stop bits. It is not possible to transmit long breaks
(break of length greater than 9/10/11 low bits).
Figure 836. Configurable stop bits
8-bit Word length (M[1:0]=00 bit is reset)
a) 1 Stop bit
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

CLOCK

Stop
bit

Next
start
bit

Next data frame

**
** LBCL bit controls last data clock pulse

b) 2 Stop bits
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

MS31885V1

Character transmission procedure
To transmit a character, follow the sequence below:
1.

Program the M bits in LPUART_CR1 to define the word length.

2.

Select the desired baud rate using the LPUART_BRR register.

3.

Program the number of stop bits in LPUART_CR2.

4.

Enable the LPUART by writing the UE bit in LPUART_CR1 register to 1.

5.

Select DMA enable (DMAT) in LPUART_CR3 if multibuffer communication is to take
place. Configure the DMA register as explained in Section 67.4.13: Continuous
communication using DMA and LPUART.

6.

Set the TE bit in LPUART_CR1 to send an idle frame as first transmission.

RM0456 Rev 6

<!-- pagebreak -->

