2902

Universal synchronous/asynchronous receiver transmitter (USART/UART)

RM0456

Determining the maximum USART baud rate that enables to correctly wake
up the microcontroller from low-power mode
The maximum baud rate that enables to correctly wake up the microcontroller from lowpower mode depends on the wake-up time parameter (refer to the device datasheet) and on
the USART receiver tolerance (see Section 66.5.9: Tolerance of the USART receiver to
clock deviation).
Let us take the example of OVER8 = 0, M bits = 01, ONEBIT = 0 and BRR [3:0] = 0000.
In these conditions, according to Table 679: Tolerance of the USART receiver when BRR
[3:0] = 0000, the USART receiver tolerance equals 3.41%.
DTRA + DQUANT + DREC + DTCL + DWU < USART receiver tolerance
DWUmax = tWUUSART/ (11 x Tbitmin)
Tbitmin = tWUUSART/ (11 x DWUmax)
where tWUUSART is the wake-up time from low-power mode.
If we consider the ideal case where DTRA, DQUANT, DREC, and DTCL parameters are at
0%, the maximum value of DWU is 3.41%. In fact, we need to consider at least the
usart_ker_ck inaccuracy (DREC).
For example, if HSI is used as usart_ker_ck, and the HSI inaccuracy is of 1%, then we
obtain:
tWUUSART = 3 µs (values provided only as examples; for correct values, refer to the
device datasheet).
DWUmax = USART receiver tolerance – DREC = 3.41% – 1% = 2.41%
Tbitmin = 3 µs/ (11 x 2.41%) = 11.32 µs.
As a result, the maximum baud rate enables to wake up correctly from low-power mode
is: 1/11.32 µs = 88.36 kbauds.

66.6

USART in low-power modes
Table 682. Effect of low-power modes on the USART
Mode

Description

Sleep

No effect. USART interrupts cause the device to exit Sleep mode.

Stop(1)

The content of the USART registers is kept.
If the USART is clocked by an oscillator available in Stop mode, transfers
in asynchronous and SPI master modes are functional. DMA requests are
functional, and the interrupts cause the device to exit Stop mode.

Standby

The USART peripheral is powered down and must be reinitialized after
exiting Standby mode.

1. Refer to Section 66.4 to know if the wake-up from Stop mode is supported for a given peripheral instance.
If an instance is not functional in a given Stop mode, it must be disabled before entering this Stop mode.

66.7

USART interrupts
Refer to Table 683 for a detailed description of all USART interrupt requests.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Universal synchronous/asynchronous receiver transmitter (USART/UART)
Table 683. USART interrupt requests

Interrupt
vector

USART or
UART

Event
flag

Enable
Control
bit

Transmit data
register empty

TXE

TXEIE

Write TDR

Yes

Transmit FIFO not
full

TXFNF

TXFNFIE TXFIFO full

Yes

Transmit FIFO
empty

TXFE

TXFEIE

Write TDR or write 1
in TXFRQ

Transmit FIFO
threshold reached

TXFT

TXFTIE

Write TDR(2)

CTS interrupt

CTSIF

CTSIE

Write 1 in CTSCF

No

Transmission
complete

TC

TCIE

Write TDR or write 1
in TCCF

Yes

Transmission
complete before
guard time

TCBGT

TCBGTIE

Write TDR or write 1
in TCBGT

No

Interrupt event

Interrupt clear
method

RM0456 Rev 6

Exit from Exit from
Sleep
Stop(1)
modes
mode

Exit from
Standby
mode

Yes
Yes

Yes

No

<!-- pagebreak -->

