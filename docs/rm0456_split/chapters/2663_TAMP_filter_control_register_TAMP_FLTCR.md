2902

Universal synchronous/asynchronous receiver transmitter (USART/UART)

RM0456

After writing the last data to the USART_TDR register, it is mandatory to wait until TC is set
before disabling the USART or causing the microcontroller to enter the low-power mode
(see Figure 812).
Figure 812. TC/TXE behavior when transmitting
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

USART_DR

F1

Set by hardware

Set by hardware
TC flag
Software
enables the
USART

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

ai17121b

Note:

When FIFO management is enabled, the TXFNF flag is used for data transmission.

Break characters
Setting the SBKRQ bit transmits a break character. The break frame length depends on the
M bit (see Figure 810).
If a 1 is written to the SBKRQ bit, a break character is sent on the TX line after completing
the current character transmission. The SBKF bit is set by the write operation and it is reset
by hardware when the break character is complete (during the stop bits after the break
character). The USART inserts a logic 1 signal (stop) for the duration of 2 bits at the end of
the break frame to guarantee the recognition of the start bit of the next frame.
When the SBKRQ bit is set, the break character is sent at the end of the current
transmission.
When FIFO mode is enabled, sending the break character has priority on sending data even
if the TXFIFO is full.

Idle characters
Setting the TE bit drives the USART to send an idle frame before the first data frame.

66.5.7

USART receiver
The USART can receive data words of either 7 or 8 or 9 bits depending on the M bits in the
USART_CR1 register.

Start bit detection
The start bit detection sequence is the same when oversampling by 16 or by 8.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Universal synchronous/asynchronous receiver transmitter (USART/UART)
In the USART, the start bit is detected when a specific sequence of samples is recognized.
This sequence is: 1 1 1 0 X 0 X 0X 0X 0 X 0X 0.
Figure 813. Start bit detection when oversampling by 16 or 8

RX state

Idle

Start bit

RX line
Ideal
sample
clock

1

2

3

Real
sample
clock

X

X

X X

4

5 6

7

X

X

X

8 9 10 11 12 13 14 15 16
Sampled values
X

9

10 11 12 13 14 15 16
6/16

7/16

7/16
One-bit time

Conditions
to validate 1
the start bit

1 1

0

Falling edge
detection

X

0

X

0

X

At least 2 bits
out of 3 at 0

0

0 0

0

X

X

X

X

X

X

At least 2 bits
out of 3 at 0
ai15471d

Note:

If the sequence has not completed, the start bit detection aborts and the receiver returns to
the idle state (no flag is set), where it waits for a falling edge.
The start bit is confirmed (RXNE flag set and interrupt generated if RXNEIE = 1, or RXFNE
flag set and interrupt generated if RXFNEIE = 1 if FIFO mode enabled) if the 3 sampled bits
are at 0 (first sampling on the 3rd, 5th and 7th bits finds the 3 bits at 0 and second sampling
on the 8th, 9th and 10th bits also finds the 3 bits at 0).
The start bit is validated but the NE noise flag is set if,
•

for both samplings, 2 out of the 3 sampled bits are at 0 (sampling on the 3rd, 5th and
7th bits and sampling on the 8th, 9th and 10th bits), or

•

for one of the samplings (sampling on the 3rd, 5th and 7th bits or sampling on the 8th,
9th and 10th bits), 2 out of the 3 bits are found at 0.
If neither of the above conditions are met, the start detection aborts and the receiver returns
to the idle state (no flag is set).

RM0456 Rev 6

<!-- pagebreak -->

