RM0456 Rev 6

RM0456

Inter-integrated circuit interface (I2C)
When RELOAD = 0 in controller mode, the counter can be used in two modes:

Caution:

•

Automatic end (AUTOEND = 1 in the I2C_CR2 register). In this mode, the controller
automatically sends a STOP condition once the number of bytes programmed in the
NBYTES[7:0] bitfield is transferred.

•

Software end (AUTOEND = 0 in the I2C_CR2 register). In this mode, a software action
is expected once the number of bytes programmed in the NBYTES[7:0] bitfield is
transferred; the TC flag is set and an interrupt is generated if the TCIE bit is set. The
SCL signal is stretched as long as the TC flag is set. The TC flag is cleared by software
when the START or STOP bit of the I2C_CR2 register is set. This mode must be used
when the controller wants to send a RESTART condition.

The AUTOEND bit has no effect when the RELOAD bit is set.
Table 661. I2C configuration

65.4.8

Function

SBC bit

RELOAD bit

AUTOEND bit

Controller Tx/Rx NBYTES + STOP

X

0

1

Controller Tx/Rx + NBYTES + RESTART

X

0

0

Target Tx/Rx, all received bytes ACKed

0

X

X

Target Rx with ACK control

1

1

X

I2C target mode
I2C target initialization
To work in target mode, the user must enable at least one target address. The I2C_OAR1
and I2C_OAR2 registers are available to program the target own addresses OA1 and OA2,
respectively.
OA1 can be configured either in 7-bit (default) or in 10-bit addressing mode, by setting the
OA1MODE bit of the I2C_OAR1 register.
OA1 is enabled by setting the OA1EN bit of the I2C_OAR1 register.
If an additional target addresses are required, the second target address OA2 can be
configured. Up to seven OA2 LSBs can be masked, by configuring the OA2MSK[2:0] bitfield
of the I2C_OAR2 register. Therefore, for OA2MSK[2:0] configured from 1 to 6, only
OA2[7:2], OA2[7:3], OA2[7:4], OA2[7:5], OA2[7:6], or OA2[7] are compared with the
received address. When OA2MSK[2:0] is other than 0, the address comparator for OA2
excludes the I2C reserved addresses (0000 XXX and 1111 XXX) and they are not
acknowledged. If OA2MSK[2:0] = 7, all received 7-bit addresses are acknowledged (except
reserved addresses). OA2 is always a 7-bit address.
When enabled through the specific bit, the reserved addresses can be acknowledged if they
are programmed in the I2C_OAR1 or I2C_OAR2 register with OA2MSK[2:0] = 0.
OA2 is enabled by setting the OA2EN bit of the I2C_OAR2 register.
The general call address is enabled by setting the GCEN bit of the I2C_CR1 register.
When the I2C peripheral is selected by one of its enabled addresses, the ADDR interrupt
status flag is set, and an interrupt is generated if the ADDRIE bit is set.
By default, the target uses its clock stretching capability, which means that it stretches the
SCL signal at low level when required, to perform software actions. If the controller does not
RM0456 Rev 6

<!-- pagebreak -->

2756

Inter-integrated circuit interface (I2C)

RM0456

support clock stretching, I2C must be configured with NOSTRETCH = 1 in the I2C_CR1
register.
After receiving an ADDR interrupt, if several addresses are enabled, the user must read the
ADDCODE[6:0] bitfield of the I2C_ISR register to check which address matched. The DIR
flag must also be checked to know the transfer direction.

Target with clock stretching
As long as the NOSTRETCH bit of the I2C_CR1 register is zero (default), the I2C peripheral
operating as an I²C-bus target stretches the SCL signal in the following situations:
•

The ADDR flag is set and the received address matches with one of the enabled target
addresses.
The stretch is released when the software clears the ADDR flag by setting the
ADDRCF bit.

•

In transmission, the previous data transmission is completed and no new data is written
in I2C_TXDR register, or the first data byte is not written when the ADDR flag is cleared
(TXE = 1).
The stretch is released when the data is written to the I2C_TXDR register.

•

In reception, the I2C_RXDR register is not read yet and a new data reception is
completed.
The stretch is released when I2C_RXDR is read.

•

In target byte control mode (SBC bit set) with reload (RELOAD bit set), the last data
byte transfer is finished (TCR bit set).
The stretch is released when then TCR is cleared by writing a non-zero value in the
NBYTES[7:0] bitfield.

•

After SCL falling edge detection.
The stretch is released after [(SDADEL + SCLDEL + 1) x (PRESC+ 1) + 1] x tI2CCLK
period.

Target without clock stretching
As long as the NOSTRETCH bit of the I2C_CR1 register is set, the I2C peripheral operating
as an I²C-bus target does not stretch the SCL signal.
The SCL clock is not stretched while the ADDR flag is set.
In transmission, the data must be written in the I2C_TXDR register before the first SCL
pulse corresponding to its transfer occurs. If not, an underrun occurs, the OVR flag is set in
the I2C_ISR register and an interrupt is generated if the ERRIE bit of the I2C_CR1 register
is set. The OVR flag is also set when the first data transmission starts and the STOPF bit is
still set (has not been cleared). Therefore, if the user clears the STOPF flag of the previous
transfer only after writing the first data to be transmitted in the next transfer, it ensures that
the OVR status is provided, even for the first data to be transmitted.
In reception, the data must be read from the I2C_RXDR register before the ninth SCL pulse
(ACK pulse) of the next data byte occurs. If not, an overrun occurs, the OVR flag is set in the
I2C_ISR register, and an interrupt is generated if the ERRIE bit of the I2C_CR1 register is
set.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Inter-integrated circuit interface (I2C)

Target byte control mode
To allow byte ACK control in target reception mode, the target byte control mode must be
enabled, by setting the SBC bit of the I2C_CR1 register. This is required to comply with
SMBus standards.
The reload mode must be selected to allow byte ACK control in target reception mode
(RELOAD = 1). To get control of each byte, NBYTES[7:0] must be initialized to 0x1 in the
ADDR interrupt subroutine, and reloaded to 0x1 after each received byte. When the byte is
received, the TCR bit is set, stretching the SCL signal low between the eighth and the ninth
SCL pulse. The user can read the data from the I2C_RXDR register, and then decide to
acknowledge it or not by configuring the ACK bit of the I2C_CR2 register. The SCL stretch is
released by programming NBYTES to a non-zero value: the acknowledge or
not-acknowledge is sent and the next byte can be received.
NBYTES[7:0] can be loaded with a value greater than 0x1. Receiving then continues until
the corresponding number of bytes are received.
Note:

The SBC bit must be configured when the I2C peripheral is disabled, when the target is not
addressed, or when ADDR = 1.
The RELOAD bit value can be changed when ADDR = 1, or when TCR = 1.

Caution:

The target byte control mode is not compatible with NOSTRETCH mode. Setting SBC when
NOSTRETCH = 1 is not allowed.
Figure 785. Target initialization flow
Target
initialization

Initial settings

Clear OA1EN and OA2EN in I2C_OAR1/I2C_OAR2

Configure OA1[9:0], OA1MODE, OA1EN, OA2[6:0],
OA2MSK[2:0], OA2EN, and GCEN

Optional: Configure SBC in I2C_CR1(1)

Enable interrupts and/or DMA in I2C_CR1

End
MSv19850V4

1. SBC must be set to support SMBus features.

RM0456 Rev 6

<!-- pagebreak -->

2756

Inter-integrated circuit interface (I2C)

RM0456

Target transmitter
A transmit interrupt status (TXIS) flag is generated when the I2C_TXDR register becomes
empty. An interrupt is generated if the TXIE bit of the I2C_CR1 register is set.
The TXIS flag is cleared when the I2C_TXDR register is written with the next data byte to
transmit.
When NACK is received, the NACKF flag is set in the I2C_ISR register and an interrupt is
generated if the NACKIE bit of the I2C_CR1 register is set. The target automatically
releases the SCL and SDA lines to let the controller perform a STOP or a RESTART
condition. The TXIS bit is not set when a NACK is received.
When STOP is received and the STOPIE bit of the I2C_CR1 register is set, the STOPF flag
of the I2C_ISR register is set and an interrupt is generated. In most applications, the SBC bit
is usually programmed to 0. In this case, if TXE = 0 when the target address is received
(ADDR = 1), the user can choose either to send the content of the I2C_TXDR register as the
first data byte, or to flush the I2C_TXDR register, by setting the TXE bit in order to program
a new data byte.
In target byte control mode (SBC = 1), the number of bytes to transmit must be programmed
in NBYTES[7:0] in the address match interrupt subroutine (ADDR = 1). In this case, the
number of TXIS events during the transfer corresponds to the value programmed in
NBYTES[7:0].
Caution:

When NOSTRETCH = 1, the SCL clock is not stretched while the ADDR flag is set, so the
user cannot flush the I2C_TXDR register content in the ADDR subroutine to program the
first data byte. The first data byte to send must be previously programmed in the I2C_TXDR
register:
•

This data can be the one written in the last TXIS event of the previous transmission
message.

•

If this data byte is not the one to send, the I2C_TXDR register can be flushed, by
setting the TXE bit, to program a new data byte. The STOPF bit must be cleared only
after these actions. This guarantees that they are executed before the first data
transmission starts, following the address acknowledge.
If STOPF is still set when the first data transmission starts, an underrun error is
generated (the OVR flag is set).
If a TXIS event (transmit interrupt or transmit DMA request) is required, the user must
set the TXIS bit in addition to the TXE bit, to generate the event.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Inter-integrated circuit interface (I2C)
Figure 786. Transfer sequence flow for I2C target transmitter, NOSTRETCH = 0

Target
transmission

Target initialization

No
I2C_ISR.ADDR
=1?
Yes

SCL
stretched

Read ADDCODE and DIR in I2C_ISR
Optional: Set I2C_ISR.TXE = 1
Set I2C_ICR.ADDRCF

No
I2C_ISR.TXIS
=1?
Yes
Write I2C_TXDR.TXDATA

MSv19851V3

RM0456 Rev 6

<!-- pagebreak -->

2756

Inter-integrated circuit interface (I2C)

RM0456

Figure 787. Transfer sequence flow for I2C target transmitter, NOSTRETCH = 1

Target
transmission

Target initialization

No
No
I2C_ISR.TXIS
=1?

I2C_ISR.STOPF
=1?

Yes

Yes

Write I2C_TXDR.TXDATA

Optional: Set I2C_ISR.TXE = 1
and I2C_ISR.TXIS=1

Set I2C_ICR.STOPCF
MSv19852V3

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Inter-integrated circuit interface (I2C)

Figure 788. Transfer bus diagrams for I2C target transmitter (mandatory events only)
legend:

Example I2C target transmitter 3 bytes with 1st data flushed,
NOSTRETCH=0:
TXIS TXIS

TXIS

A

A

ADDR

S

Address

data1

reception
data3

A

data2

EV2 EV3

EV1

transmission

TXIS

NA

P

SCL stretch

EV4 EV5

TXE

EV1: ADDR ISR: check ADDCODE and DIR, set TXE, set ADDRCF
EV2: TXIS ISR: wr data1
EV3: TXIS ISR: wr data2
EV4: TXIS ISR: wr data3
EV5: TXIS ISR: wr data4 (not sent)

legend :

Example I2C target transmitter 3 bytes without 1st data flush,
NOSTRETCH=0:

S

TXIS

TXIS

ADDR

Address

A

EV1

data1

A

transmission

TXIS

data2

reception

A

EV2

data3

EV3

NA

SCL stretch

P

EV4

TXE

EV1: ADDR ISR: check ADDCODE and DIR, set ADDRCF
EV2: TXIS ISR: wr data2
EV3: TXIS ISR: wr data3
EV4: TXIS ISR: wr data4 (not sent)

legend:

Example I2C target transmitter 3 bytes, NOSTRETCH=1:

transmission
TXIS

TXIS

S

Address

EV1

data1

A

EV2

A

STOPF

TXIS

data2

EV3

A

data3

EV4

reception
SCL stretch

NA P

EV5

TXE

EV1: wr data1
EV2: TXIS ISR: wr data2
EV3: TXIS ISR: wr data3
EV4: TXIS ISR: wr data4 (not sent)
EV5: STOPF ISR: (optional: set TXE and TXIS), set STOPCF
MSv19853V3

RM0456 Rev 6

<!-- pagebreak -->

2756

Inter-integrated circuit interface (I2C)

RM0456

Target receiver
The RXNE bit of the I2C_ISR register is set when the I2C_RXDR is full, which generates an
interrupt if the RXIE bit of the I2C_CR1 register is set. RXNE is cleared when I2C_RXDR is
read.
When STOP condition is received and the STOPIE bit of the I2C_CR1 register is set, the
STOPF flag in the I2C_ISR register is set and an interrupt is generated.
Figure 789. Transfer sequence flow for I2C target receiver, NOSTRETCH = 0

Target reception

Target initialization

No
I2C_ISR.ADDR
=1?
Yes
SCL
stretched
Read ADDCODE and DIR in I2C_ISR
Set I2C_ICR.ADDRCF

No
I2C_ISR.RXNE
=1?
Yes
Write I2C_RXDR.RXDATA

MSv19855V3

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Inter-integrated circuit interface (I2C)
Figure 790. Transfer sequence flow for I2C target receiver, NOSTRETCH = 1
Target reception

Target initialization

No
No
I2C_ISR.RXNE
=1?

I2C_ISR.STOPF
=1?

Yes

Yes
Set I2C_ICR.STOPCF

Read I2C_RXDR.RXDATA

MSv19856V3

Figure 791. Transfer bus diagrams for I2C target receiver
(mandatory events only)
Legend

Example I2C target receiver 3 bytes, NOSTRETCH = 0:
ADDR

RXNE

RXNE

RXNE

Transmission
Reception

S

Address

A

data1

A

data2

data3

A

EV1

EV2

EV3

A

SCL stretch

EV4

RXNE

EV1: ADDR ISR: check ADDCODE and DIR, set ADDRCF
EV2: RXNE ISR: rd data1
EV3: RXNE ISR: rd data2
EV4: RXNE ISR: rd data3

Legend

Example I2C target receiver 3 bytes, NOSTRETCH = 1:

Transmission

RXNE
S

Address

A

data 1

RXNE
data 2

A

EV1

RXNE
data 3

A

EV2

A

Reception
P

SCL stretch

EV3

RXNE

EV1: RXNE ISR: rd data1
EV2: RXNE ISR: rd data2
EV3: RXNE ISR: rd data3
MSv19857V5

RM0456 Rev 6

<!-- pagebreak -->

2756

Inter-integrated circuit interface (I2C)

65.4.9

RM0456

I2C controller mode
I2C controller initialization
Before enabling the peripheral, the I2C controller clock must be configured, by setting the
SCLH and SCLL bits in the I2C_TIMINGR register.
The STM32CubeMX tool calculates and provides the I2C_TIMINGR content in the I2C
Configuration window.
A clock synchronization mechanism is implemented in order to support multicontroller
environment and target clock stretching.
In order to allow clock synchronization:
•

The low level of the clock is counted using the SCLL counter, starting from the SCL low
level internal detection.

•

The high level of the clock is counted using the SCLH counter, starting from the SCL
high level internal detection.

I2C detects its own SCL low level after a tSYNC1 delay depending on the SCL falling edge,
SCL input noise filters (analog and digital), and SCL synchronization to the I2CxCLK clock.
I2C releases SCL to high level once the SCLL counter reaches the value programmed in the
SCLL[7:0] bitfield of the I2C_TIMINGR register.
I2C detects its own SCL high level after a tSYNC2 delay depending on the SCL rising edge,
SCL input noise filters (analog and digital), and SCL synchronization to the I2CxCLK clock.
I2C ties SCL to low level once the SCLH counter reaches the value programmed in the
SCLH[7:0] bitfield of the I2C_TIMINGR register.
Consequently the controller clock period is:
tSCL = tSYNC1 + tSYNC2 + {[(SCLH+ 1) + (SCLL+ 1)] x (PRESC+ 1) x tI2CCLK}
The duration of tSYNC1 depends upon:
•

SCL falling slope

•

input delay induced by the analog filter (when enabled)

•

input delay induced by the digital filter (when enabled): DNF[3:0] x tI2CCLK

•

delay due to SCL synchronization with the i2c_ker_ck clock (two to three i2c_ker_ck
periods)

The duration of tSYNC2 depends upon:

<!-- pagebreak -->

•

SCL rising slope

•

input delay induced by the analog filter (when enabled)

•

input delay induced by the digital filter (when enabled): DNF[3:0] x tI2CCLK

•

delay due to SCL synchronization with the i2c_ker_ck clock (two to three i2c_ker_ck
periods)

RM0456 Rev 6

RM0456

Inter-integrated circuit interface (I2C)
Figure 792. Controller clock generation
SCL controller clock generation

SCL high level detected
SCLH counter starts

tSYNC2

SCLH
tSYNC1

SCLL

SCL

SCL low level detected
SCLL counter starts

SCL released

SCL driven low

SCL controller clock synchronization
SCL high level detected
SCLH counter starts

SCL high level detected
SCLH counter starts

SCLH

SCL high level detected
SCLH counter starts

SCLH

SCLL

SCLH

SCLL
SCL driven low by
another device

SCL driven low by
another device

SCL low level detected
SCLL counter starts
SCL low level detected
SCLL counter starts

SCL released

MSv19858V2

RM0456 Rev 6

<!-- pagebreak -->

