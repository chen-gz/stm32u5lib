RM0456 Rev 6

RM0456

Low-power universal asynchronous receiver transmitter (LPUART)
Figure 839. Mute mode using Idle line detection
RXNE

Data 1 Data 2 Data 3 Data 4

RX

RWU
MMRQ written to 1

IDLE

Mute mode

RXNE

Data 5 Data 6

Normal mode
Idle frame detected

MSv31154V1

Note:

If the MMRQ is set while the IDLE character has already elapsed, mute mode is not entered
(RWU is not set).
If the LPUART is activated while the line is IDLE, the idle state is detected after the duration
of one IDLE frame (not only after the reception of one character frame).

4-bit/7-bit address mark detection (WAKE = 1)
In this mode, bytes are recognized as addresses if their MSB is a 1 otherwise they are
considered as data. In an address byte, the address of the targeted receiver is put in the 4
or 7 LSBs. The choice of 7- or 4-bit address detection is done using the ADDM7 bit. This 4bit/7-bit word is compared by the receiver with its own address, which is programmed in the
ADD bits in the LPUART_CR2 register.
Note:

In 7-bit and 9-bit data modes, address detection is done on 6-bit and 8-bit addresses
(ADD[5:0] and ADD[7:0]) respectively.
The LPUART enters mute mode when an address character is received which does not
match its programmed address. In this case, the RWU bit is set by hardware. The RXNE
flag is not set for this address byte and no interrupt or DMA request is issued when the
LPUART enters mute mode.
The LPUART also enters mute mode when the MMRQ bit is written to 1. The RWU bit is
also automatically set in this case.
The LPUART exits from mute mode when an address character is received which matches
the programmed address. Then the RWU bit is cleared and subsequent bytes are received
normally. The RXNE/RXFNE bit is set for the address character since the RWU bit has been
cleared.

Note:

When FIFO management is enabled, when MMRQ bit is set while the receiver is sampling
the last bit of a data, this data may be received before effectively entering in mute mode.
An example of mute mode behavior using address mark detection is given in Figure 840.

RM0456 Rev 6

<!-- pagebreak -->

2902

Low-power universal asynchronous receiver transmitter (LPUART)

RM0456

Figure 840. Mute mode using address mark detection
In this example, the current address of the receiver is 1
(programmed in the LPUART_CR2 register)
RXNE

IDLE

RX

Addr=0 Data 1 Data 2

RWU

IDLE

RXNE

Addr=1 Data 3 Data 4 Addr=2 Data 5

Mute mode

Normal mode
Matching address

MMRQ written to 1
(RXNE was cleared)

RXNE

Mute mode

Non-matching address

Non-matching address
MSv31888V2

67.4.11

LPUART parity control
Parity control (generation of parity bit in transmission and parity checking in reception) can
be enabled by setting the PCE bit in the LPUART_CR1 register. Depending on the frame
length defined by the M bits, the possible LPUART frame formats are as listed in Table 692.
Table 692: LPUART frame formats
M bits

PCE bit

LPUART frame(1)

00

0

| SB | 8 bit data | STB |

00

1

| SB | 7-bit data | PB | STB |

01

0

| SB | 9-bit data | STB |

01

1

| SB | 8-bit data PB | STB |

10

0

| SB | 7bit data | STB |

10

1

| SB | 6-bit data | PB | STB |

1. Legends: SB: start bit, STB: stop bit, PB: parity bit.
2. In the data register, the PB is always taking the MSB position (8th or 7th, depending on the M bit value).

Even parity
The parity bit is calculated to obtain an even number of “1s” inside the frame, which is made
of the 6, 7 or 8 LSB bits (depending on M bit values) and the parity bit.
As an example, if data = 00110101, and 4 bits are set, then the parity bit is equal to 0 if even
parity is selected (PS bit in LPUART_CR1 = 0).

Odd parity
The parity bit is calculated to obtain an odd number of “1s” inside the frame made of the 6, 7
or 8 LSB bits (depending on M bit values) and the parity bit.
As an example, if data = 00110101 and 4 bits set, then the parity bit is equal to 1 if odd parity
is selected (PS bit in LPUART_CR1 = 1).

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Low-power universal asynchronous receiver transmitter (LPUART)

Parity checking in reception
If the parity check fails, the PE flag is set in the LPUART_ISR register and an interrupt is
generated if PEIE is set in the LPUART_CR1 register. The PE flag is cleared by software
writing 1 to the PECF in the LPUART_ICR register.

Parity generation in transmission
If the PCE bit is set in LPUART_CR1, then the MSB bit of the data written in the data
register is transmitted but is changed by the parity bit (even number of “1s” if even parity is
selected (PS = 0) or an odd number of “1s” if odd parity is selected (PS = 1)).

67.4.12

LPUART single-wire half-duplex communication
Single-wire half-duplex mode is selected by setting the HDSEL bit in the LPUART_CR3
register.
The LPUART can be configured to follow a single-wire half-duplex protocol where the TX
and RX lines are internally connected. The selection between half- and full-duplex
communication is made with a control bit HDSEL in LPUART_CR3.
As soon as HDSEL is written to 1:
•

The TX and RX lines are internally connected.

•

The RX pin is no longer used

•

The TX pin is always released when no data is transmitted. Thus, it acts as a standard
I/O in idle or in reception. It means that the I/O must be configured so that TX is
configured as alternate function open-drain with an external pull-up.

Apart from this, the communication protocol is similar to normal LPUART mode. Any conflict
on the line must be managed by software (for instance by using a centralized arbiter). In
particular, the transmission is never blocked by hardware and continues as soon as data is
written in the data register while the TE bit is set.
Note:

In LPUART communications, in the case of 1-stop bit configuration, the RXNE flag is set in
the middle of the stop bit.

67.4.13

Continuous communication using DMA and LPUART
The LPUART is capable of performing continuous communication using the DMA. The DMA
requests for Rx buffer and Tx buffer are generated independently.

Note:

Refer to Section 67.3: LPUART implementation to determine if the DMA mode is supported.
If DMA is not supported, use the LPUSRT as explained in Section 67.4.7. To perform
continuous communication. When FIFO is disabled, clear the TXE/ RXNE flags in the
LPUART_ISR register.

Transmission using DMA
DMA mode can be enabled for transmission by setting the DMAT bit in the LPUART_CR3
register. Data are loaded from an SRAM area configured using the DMA peripheral (refer to
Section direct memory access controller) to the LPUART_TDR register whenever the TXE
flag (TXFNF flag if FIFO mode is enabled) is set. To map a DMA channel for LPUART
transmission, use the following procedure (x denotes the channel number):

RM0456 Rev 6

<!-- pagebreak -->

