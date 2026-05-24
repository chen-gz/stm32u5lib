2756

Inter-integrated circuit interface (I2C)

RM0456

Bit 31 TEXTEN: Extended clock timeout enable
0: Extended clock timeout detection is disabled
1: Extended clock timeout detection is enabled. When a cumulative SCL stretch for more
than tLOW:EXT is done by the I2C interface, a timeout error is detected (TIMEOUT = 1).
Bits 30:28 Reserved, must be kept at reset value.
Bits 27:16 TIMEOUTB[11:0]: Bus timeout B
This field is used to configure the cumulative clock extension timeout:
–
Controller mode: the controller cumulative clock low extend time (tLOW:MEXT) is
detected
–
Target mode: the target cumulative clock low extend time (tLOW:SEXT) is detected
tLOW:EXT = (TIMEOUTB + TIDLE = 01) x 2048 x tI2CCLK
Note: These bits can be written only when TEXTEN = 0.
Bit 15 TIMOUTEN: Clock timeout enable
0: SCL timeout detection is disabled
1: SCL timeout detection is enabled. When SCL is low for more than tTIMEOUT (TIDLE = 0) or
high for more than tIDLE (TIDLE = 1), a timeout error is detected (TIMEOUT = 1).
Bits 14:13 Reserved, must be kept at reset value.
Bit 12 TIDLE: Idle clock timeout detection
0: TIMEOUTA is used to detect SCL low timeout
1: TIMEOUTA is used to detect both SCL and SDA high timeout (bus idle condition)
Note: This bit can be written only when TIMOUTEN = 0.
Bits 11:0 TIMEOUTA[11:0]: Bus timeout A
This field is used to configure:
The SCL low timeout condition tTIMEOUT when TIDLE = 0
tTIMEOUT= (TIMEOUTA + 1) x 2048 x tI2CCLK
The bus idle condition (both SCL and SDA high) when TIDLE = 1
tIDLE= (TIMEOUTA + 1) x 4 x tI2CCLK
Note: These bits can be written only when TIMOUTEN = 0.

65.9.7

I2C interrupt and status register (I2C_ISR)
Address offset: 0x18
Reset value: 0x0000 0001
Access: no wait states

31

30

29

28

27

26

25

24

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.
r

r

r

r

r

r

r

r

15

14

13

12

11

10

9

8

7

6

5

4

3

2

1

0

BUSY

Res.

ALERT

OVR

ARLO

BERR

TCR

TC

ADDR

RXNE

TXIS

TXE

r

r

r

r

r

r

r

rs

rs

r

r

TIMEO PECER
UT
R
r

r

23

21

20

19

18

17

ADDCODE[6:0]

Bits 31:24 Reserved, must be kept at reset value.

<!-- pagebreak -->

22

RM0456 Rev 6

STOPF NACKF
r

r

16
DIR

RM0456

Inter-integrated circuit interface (I2C)

Bits 23:17 ADDCODE[6:0]: Address match code (target mode)
These bits are updated with the received address when an address match event occurs
(ADDR = 1). In the case of a 10-bit address, ADDCODE provides the 10-bit header followed
by the two MSBs of the address.
Bit 16 DIR: Transfer direction (target mode)
This flag is updated when an address match event occurs (ADDR = 1).
0: Write transfer, target enters receiver mode.
1: Read transfer, target enters transmitter mode.
Bit 15 BUSY: Bus busy
This flag indicates that a communication is in progress on the bus. It is set by hardware
when a START condition is detected, and cleared by hardware when a STOP condition is
detected, or when PE = 0.
Bit 14 Reserved, must be kept at reset value.
Bit 13 ALERT: SMBus alert
This flag is set by hardware when SMBHEN = 1 (SMBus host configuration), ALERTEN = 1
and an SMBALERT# event (falling edge) is detected on SMBA pin. It is cleared by software
by setting the ALERTCF bit.
Note: This bit is cleared by hardware when PE = 0.
Bit 12 TIMEOUT: Timeout or tLOW detection flag
This flag is set by hardware when a timeout or extended clock timeout occurred. It is cleared
by software by setting the TIMEOUTCF bit.
Note: This bit is cleared by hardware when PE = 0.
Bit 11 PECERR: PEC error in reception
This flag is set by hardware when the received PEC does not match with the PEC register
content. A NACK is automatically sent after the wrong PEC reception. It is cleared by
software by setting the PECCF bit.
Note: This bit is cleared by hardware when PE = 0.
Bit 10 OVR: Overrun/underrun (target mode)
This flag is set by hardware in target mode with NOSTRETCH = 1, when an
overrun/underrun error occurs. It is cleared by software by setting the OVRCF bit.
Note: This bit is cleared by hardware when PE = 0.
Bit 9 ARLO: Arbitration lost
This flag is set by hardware in case of arbitration loss. It is cleared by software by setting the
ARLOCF bit.
Note: This bit is cleared by hardware when PE = 0.
Bit 8 BERR: Bus error
This flag is set by hardware when a misplaced START or STOP condition is detected
whereas the peripheral is involved in the transfer. The flag is not set during the address
phase in target mode. It is cleared by software by setting the BERRCF bit.
Note: This bit is cleared by hardware when PE = 0.
Bit 7 TCR: Transfer complete reload
This flag is set by hardware when RELOAD = 1 and NBYTES data have been transferred. It
is cleared by software when NBYTES is written to a non-zero value.
Note: This bit is cleared by hardware when PE = 0.
This flag is only for controller mode, or for target mode when the SBC bit is set.

RM0456 Rev 6

<!-- pagebreak -->

2756

Inter-integrated circuit interface (I2C)

RM0456

Bit 6 TC: Transfer complete (controller mode)
This flag is set by hardware when RELOAD = 0, AUTOEND = 0 and NBYTES data have
been transferred. It is cleared by software when START bit or STOP bit is set.
Note: This bit is cleared by hardware when PE = 0.
Bit 5 STOPF: STOP detection flag
This flag is set by hardware when a STOP condition is detected on the bus and the
peripheral is involved in this transfer:
–
as a controller, provided that the STOP condition is generated by the peripheral.
–
as a target, provided that the peripheral has been addressed previously during this
transfer.
It is cleared by software by setting the STOPCF bit.
Note: This bit is cleared by hardware when PE = 0.
Bit 4 NACKF: Not acknowledge received flag
This flag is set by hardware when a NACK is received after a byte transmission. It is cleared
by software by setting the NACKCF bit.
Note: This bit is cleared by hardware when PE = 0.
Bit 3 ADDR: Address matched (target mode)
This bit is set by hardware as soon as the received target address matched with one of the
enabled target addresses. It is cleared by software by setting ADDRCF bit.
Note: This bit is cleared by hardware when PE = 0.
Bit 2 RXNE: Receive data register not empty (receivers)
This bit is set by hardware when the received data is copied into the I2C_RXDR register, and
is ready to be read. It is cleared when I2C_RXDR is read.
Note: This bit is cleared by hardware when PE = 0.
Bit 1 TXIS: Transmit interrupt status (transmitters)
This bit is set by hardware when the I2C_TXDR register is empty and the data to be
transmitted must be written in the I2C_TXDR register. It is cleared when the next data to be
sent is written in the I2C_TXDR register.
This bit can be written to 1 by software only when NOSTRETCH = 1, to generate a TXIS
event (interrupt if TXIE = 1 or DMA request if TXDMAEN = 1).
Note: This bit is cleared by hardware when PE = 0.
Bit 0 TXE: Transmit data register empty (transmitters)
This bit is set by hardware when the I2C_TXDR register is empty. It is cleared when the next
data to be sent is written in the I2C_TXDR register.
This bit can be written to 1 by software in order to flush the transmit data register I2C_TXDR.
Note: This bit is set by hardware when PE = 0.

<!-- pagebreak -->

