2902

Universal synchronous/asynchronous receiver transmitter (USART/UART)

RM0456

Figure 816. Data sampling when oversampling by 8

RX line
sampled values
Sample
clock (x8)

1

2

3

4

5

6

7

8

2/8
3/8

3/8
One bit time

MSv31153V1

Table 678. Noise detection from sampled data
Sampled value

NE status

Received bit value

000

0

0

001

1

0

010

1

0

011

1

1

100

1

0

101

1

1

110

1

1

111

0

1

Framing error
A framing error is detected when the stop bit is not recognized on reception at the expected
time, following either a de-synchronization or excessive noise.
When the framing error is detected:
•

the FE bit is set by hardware.

•

the invalid data is transferred from the Shift register to the USART_RDR register
(RXFIFO in case FIFO mode is enabled).

•

no interrupt is generated in case of single byte communication. However this bit rises at
the same time as the RXNE bit (RXFNE in case FIFO mode is enabled) which itself
generates an interrupt. In case of multibuffer communication an interrupt is issued if the
EIE bit is set in the USART_CR3 register.

The FE bit is reset by writing 1 to the FECF in the USART_ICR register.
Note:

<!-- pagebreak -->

Framing error is not supported in SPI mode.

RM0456 Rev 6

RM0456

Universal synchronous/asynchronous receiver transmitter (USART/UART)

Configurable stop bits during reception
The number of stop bits to be received can be configured through the control bits of
USART_CR: it can be either 1 or 2 in normal mode and 0.5 or 1.5 in smartcard mode.
•

0.5 stop bit (reception in smartcard mode): no sampling is done for 0.5 stop bit. As a
consequence, no framing error and no break frame can be detected when 0.5 stop bit
is selected.

•

1 stop bit: sampling for 1 stop bit is done on the 8th, 9th and 10th samples.

•

1.5 stop bits (smartcard mode)
When transmitting in smartcard mode, the device must check that the data are
correctly sent. The receiver block must consequently be enabled (RE = 1 in
USART_CR1) and the stop bit is checked to test if the smartcard has detected a parity
error.
In the event of a parity error, the smartcard forces the data signal low during the
sampling (NACK signal), which is flagged as a framing error. The FE flag is then set
through RXNE flag (RXFNE if the FIFO mode is enabled) at the end of the 1.5 stop bit.
Sampling for 1.5 stop bits is done on the 16th, 17th and 18th samples (1 baud clock
period after the beginning of the stop bit). The 1.5 stop bit can be broken into 2 parts:
one 0.5 baud clock period during which nothing happens, followed by 1 normal stop bit
period during which sampling occurs halfway through (refer to Section 66.5.17: USART
receiver timeout for more details).

•

2 stop bits
Sampling for 2 stop bits is done on the 8th, 9th and 10th samples of the first stop bit.
The framing error flag is set if a framing error is detected during the first stop bit.
The second stop bit is not checked for framing error. The RXNE flag (RXFNE if the
FIFO mode is enabled) is set at the end of the first stop bit.

66.5.8

USART baud rate generation
The baud rate for the receiver and transmitter (Rx and Tx) are both set to the value
programmed in the USART_BRR register.
Equation 1: baud rate for standard USART (SPI mode included) (OVER8 = 0 or 1)
In case of oversampling by 16, the baud rate is given by the following formula:
Tx/Rx baud = usart_ker_ck_pres
-------------------------------------------------USARTDIV

In case of oversampling by 8, the baud rate is given by the following formula:
× usart_ker_ck_pres
Tx/Rx baud = 2
------------------------------------------------------------USARTDIV

Equation 2: baud rate in smartcard, LIN and IrDA modes (OVER8 = 0)
The baud rate is given by the following formula:
Tx/Rx baud = usart_ker_ck_pres
-------------------------------------------------USARTDIV

RM0456 Rev 6

<!-- pagebreak -->

2902

Universal synchronous/asynchronous receiver transmitter (USART/UART)

RM0456

USARTDIV is an unsigned fixed point number that is coded on the USART_BRR register.

Note:

•

When OVER8 = 0, BRR = USARTDIV.

•

When OVER8 = 1
–

BRR[2:0] = USARTDIV[3:0] shifted 1 bit to the right.

–

BRR[3] must be kept cleared.

–

BRR[15:4] = USARTDIV[15:4]

The baud counters are updated to the new value in the baud registers after a write operation
to USART_BRR. Hence the baud rate register value must not be changed during
communication.
In case of oversampling by 16 and 8, USARTDIV must be greater than or equal to 16.

How to derive USARTDIV from USART_BRR register values
Example 1
To obtain 9600 bauds with usart_ker_ck_pres= 8 MHz:
•

In case of oversampling by 16:
USARTDIV = 8 000 000/9600
BRR[3:0] = USARTDIV = 0d833 = 0x0341

•

In case of oversampling by 8:
USARTDIV = 2 * 8 000 000/9600
USARTDIV = 1666,66 (0d1667 = 0x683)
BRR[3:0] = 0x3 >>1 = 0x1
BRR[3:0] = 0x681

Example 2
To obtain 921.6 kbauds with usart_ker_ck_pres = 48 MHz:
•

In case of oversampling by 16:
USARTDIV = 48 000 000/921 600
BRR[3:0] = USARTDIV = 52 = 0x34

•

In case of oversampling by 8:
USARTDIV = 2 * 48 000 000/921 600
USARTDIV = 104 (0d104 = 0x68)
BRR[3:0] = USARTDIV[3:0] >> 1 = 0x8 >> 1 = 0x4
BRR[3:0] = 0x64

<!-- pagebreak -->

