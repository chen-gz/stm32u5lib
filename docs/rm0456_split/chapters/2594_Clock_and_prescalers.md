1.

I2C_CR2 register: START, STOP, PECBYTE, and NACK

2.

I2C_ISR register: BUSY, TXE, TXIS, RXNE, ADDR, NACKF, TCR, TC, STOPF, BERR,
ARLO, PECERR, TIMEOUT, ALERT, and OVR

RM0456 Rev 6

RM0456

Inter-integrated circuit interface (I2C)
PE must be kept low during at least three APB clock cycles to perform the I2C reset. To
ensure this, perform the following software sequence:

65.4.7

1.

Write PE = 0

2.

Check PE = 0

3.

Write PE = 1

I2C data transfer
The data transfer is managed through transmit and receive data registers and a shift
register.

Reception
The SDA input fills the shift register. After the eighth SCL pulse (when the complete data
byte is received), the shift register is copied into the I2C_RXDR register if it is empty
(RXNE = 0). If RXNE = 1, which means that the previous received data byte has not yet
been read, the SCL line is stretched low until I2C_RXDR is read. The stretch occurs
between the eighth and the ninth SCL pulse (before the acknowledge pulse).
Figure 783. Data reception
ACK pulse

ACK pulse

legend:

SCL
Shift register

SCL
stretch
xx

data1

xx

data2

xx

RXNE
rd data0 rd data1

I2C_RXDR

data0

data1

data2

MS19848V1

RM0456 Rev 6

<!-- pagebreak -->

2756

Inter-integrated circuit interface (I2C)

RM0456

Transmission
If the I2C_TXDR register is not empty (TXE = 0), its content is copied into the shift register
after the ninth SCL pulse (the acknowledge pulse). Then the shift register content is shifted
out on the SDA line. If TXE = 1, which means that no data is written yet in I2C_TXDR, the
SCL line is stretched low until I2C_TXDR is written. The stretch starts after the ninth SCL
pulse.
Figure 784. Data transmission
ACK pulse

ACK pulse
legend:

xx

xx

wr data1

wr data2

SCL
stretch

data2

Shift register

data1

SCL

xx

TXE

I2C_TXDR

data0

data1

data2

MS19849V1

Hardware transfer management
The I2C features an embedded byte counter to manage byte transfer and to close the
communication in various modes, such as:
–

NACK, STOP and ReSTART generation in controller mode

–

ACK control in target receiver mode

–

PEC generation/checking

In controller mode, the byte counter is always used. By default, it is disabled in target mode.
It can be enabled by software, by setting the SBC (target byte control) bit of the I2C_CR1
register.
The number of bytes to transfer is programmed in the NBYTES[7:0] bitfield of the I2C_CR2
register. If this number is greater than 255, or if a receiver wants to control the acknowledge
value of a received data byte, the reload mode must be selected, by setting the RELOAD bit
of the I2C_CR2 register. In this mode, the TCR flag is set when the number of bytes
programmed in NBYTES[7:0] is transferred (when the associated counter reaches zero),
and an interrupt is generated if TCIE is set. SCL is stretched as long as the TCR flag is set.
TCR is cleared by software when NBYTES[7:0] is written to a non-zero value.
When NBYTES[7:0] is reloaded with the last number of bytes to transfer, the RELOAD bit
must be cleared.

<!-- pagebreak -->

