RM0456 Rev 6

RM0456

Inter-integrated circuit interface (I2C)
Figure 798. Transfer bus diagrams for I2C controller transmitter
(mandatory events only)

Example I2C controller transmitter 2 bytes, automatic end mode (STOP)
legend:
transmission

TXIS TXIS

reception
S

Address

A

data1

A

data2

A

P

SCL stretch
INIT

EV1 EV2

TXE
NBYTES xx

2

INIT: program target address, program NBYTES = 2, AUTOEND = 1, set START
EV1: TXIS ISR: wr data1
EV2: TXIS ISR: wr data2

Example I2C controller transmitter 2 bytes, software end mode (RESTART)
TXIS TXIS

legend:

TC

transmission
S Address

INIT

A

data1

A

data2

EV1 EV2

A

ReS

Address

EV3

reception
SCL stretch

TXE

NBYTES

xx

2

INIT: program target address, program NBYTES = 2, AUTOEND = 0, set START
EV1: TXIS ISR: wr data1
EV2: TXIS ISR: wr data2
EV3: TC ISR: program target address, program NBYTES = N, set START

MSv19862V3

RM0456 Rev 6

<!-- pagebreak -->

2756

Inter-integrated circuit interface (I2C)

RM0456

Controller receiver
In the case of a read transfer, the RXNE flag is set after each byte reception, after the eighth
SCL pulse. An RXNE event generates an interrupt if the RXIE bit of the I2C_CR1 register is
set. The flag is cleared when I2C_RXDR is read.
If the total number of data bytes to receive is greater than 255, select the reload mode, by
setting the RELOAD bit of the I2C_CR2 register. In this case, when the NBYTES[7:0]
number of data bytes is transferred, the TCR flag is set and the SCL line is stretched low
until NBYTES[7:0] is written with a non-zero value.
When RELOAD = 0 and he number of data bytes defined in NBYTES[7:0] is transferred:

<!-- pagebreak -->

