2902

Low-power universal asynchronous receiver transmitter (LPUART)

RM0456

Configurable stop bits during reception
The number of stop bits to be received can be configured through the control bits of
LPUART_CR2: it can be either 1 or 2 in normal mode.

67.4.8

•

1 stop bit: sampling for 1 stop bit is done on the 8th, 9th, and 10th samples.

•

2 stop bits: sampling for the 2 stop bits is done in the middle of the second stop bit.
The RXNE and FE flags are set just after this sample, that is, during the second stop
bit. The first stop bit is not checked for framing error.

LPUART baud rate generation
The baud rate for the receiver and transmitter (Rx and Tx) are both set to the value
programmed in the LPUART_BRR register.

256 × lpuart_ker_ck_pres
Tx/Rx baud = ------------------------------------------------------------------LPUARTDIV

LPUARTDIV is defined in the LPUART_BRR register.
Note:

The baud counters are updated to the new value in the baud registers after a write operation
to LPUART_BRR. Hence, the baud rate register value must not be changed during
communication.
It is forbidden to write values lower than 0x300 in the LPUART_BRR register.
lpuart_ker_ck_pres must range from 3 x baud rate to 4096 x baud rate.
The maximum baud rate that can be reached when the LPUART clock source is the LSE, is
9600 bauds. Higher baud rates can be reached when the LPUART is clocked by clock
sources different from the LSE clock.

Table 690. Error calculation for programmed baud rates at lpuart_ker_ck_pres= 32.768 kHz
lpuart_ker_ck_pres= 32.768 kHz

Baud rate

S.No

Desired

Actual

Value programmed in the baud
rate register

% Error = (Calculated – Desired)
B.rate / Desired B.rate

1

0.3 kbaud

300 baud

0x6D3A

0

2

0.6 kbaud

600 baud

0x369D

0

3

1200 bauds

1200.087 bauds

0x1B4E

0.007

4

2400 bauds

2400.17 bauds

0xDA7

0.007

5

4800 bauds

4801.72 bauds

0x6D3

0.035

6

9600 kbauds

9608.94 bauds

0x369

0.093

<!-- pagebreak -->

