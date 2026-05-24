2756

Inter-integrated circuit interface (I2C)

RM0456

The SMBus device default address (0b1100 001) is enabled by setting the SMBDEN bit of
the I2C_CR1 register.
The SMBus host address (0b0001 000) is enabled by setting the SMBHEN bit of the
I2C_CR1 register.
The alert response address (0b0001100) is enabled by setting the ALERTEN bit of the
I2C_CR1 register.

Packet error checking
PEC calculation is enabled by setting the PECEN bit of the I2C_CR1 register. Then the PEC
transfer is managed with the help of the hardware byte counter associated with the
NBYTES[7:0] bitfield of the I2C_CR2 register. The PECEN bit must be configured before
enabling the I2C.
The PEC transfer is managed with the hardware byte counter, so the SBC bit must be set
when interfacing the SMBus in target mode. The PEC is transferred after transferring
NBYTES[7:0] - 1 data bytes, if the PECBYTE bit is set and the RELOAD bit is cleared. If
RELOAD is set, PECBYTE has no effect.
Caution:

Changing the PECEN configuration is not allowed when the I2C peripheral is enabled.
Table 666. SMBus with PEC configuration
Mode

SBC bit RELOAD bit AUTOEND bit PECBYTE bit

Controller Tx/Rx NBYTES + PEC+ STOP

X

0

1

1

Controller Tx/Rx NBYTES + PEC +
ReSTART

X

0

0

1

Target Tx/Rx with PEC

1

0

X

1

Timeout detection
The timeout detection is enabled by setting the TIMOUTEN and TEXTEN bits of the
I2C_TIMEOUTR register. The timers must be programmed in such a way that they detect a
timeout before the maximum time given in the SMBus specification.
tTIMEOUT check
To check the tTIMEOUT parameter, load the 12-bit TIMEOUTA[11:0] bitfield with the timer
reload value. Keep the TIDLE bit at 0 to detect the SCL low level timeout.
Then set the TIMOUTEN bit of the I2C_TIMEOUTR register, to enable the timer.
If SCL is tied low for longer than the (TIMEOUTA + 1) x 2048 x tI2CCLK period, the TIMEOUT
flag of the I2C_ISR register is set.
Refer to Table 667.
Caution:

Changing the TIMEOUTA[11:0] bitfield and the TIDLE bit values is not allowed when the
TIMEOUTEN bit is set.
tLOW:SEXT and tLOW:MEXT check

A 12-bit timer associated with the TIMEOUTB[11:0] bitfield allows checking tLOW:SEXT for the
I2C peripheral operating as a target, or tLOW:MEXT when it operates as a controller. As the
standard only specifies a maximum, the user can choose the same value for both. The timer
is then enabled by setting the TEXTEN bit in the I2C_TIMEOUTR register.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Inter-integrated circuit interface (I2C)
If the SMBus peripheral performs a cumulative SCL stretch for longer than the (TIMEOUTB
+ 1) x 2048 x tI2CCLK period, and within the timeout interval described in Bus idle detection
section, the TIMEOUT flag of the I2C_ISR register is set.
Refer to Table 668.

Caution:

Changing the TIMEOUTB[11:0] bitfield value is not allowed when the TEXTEN bit is set.

Bus idle detection
To check the tIDLE period, the TIMEOUTA[11:0] bitfield associated with 12-bit timer must be
loaded with the timer reload value. Keep the TIDLE bit at 1 to detect both SCL and SDA high
level timeout. Then set the TIMOUTEN bit of the I2C_TIMEOUTR register to enable the
timer.
If both the SCL and SDA lines remain high for longer than the
(TIMEOUTA + 1) x 4 x tI2CCLK period, the TIMEOUT flag of the I2C_ISR register is set.
Refer to Table 669.
Caution:

Changing the TIMEOUTA[11:0] bitfield and the TIDLE bit values is not allowed when the
TIMEOUTEN bit is set.

65.4.13

SMBus I2C_TIMEOUTR register configuration examples
The following tables provide examples of settings to reach desired tTIMEOUT, tLOW:SEXT,
tLOW:MEXT, and tIDLE timings at different fI2CCLK frequencies.
Table 667. TIMEOUTA[11:0] for maximum tTIMEOUT of 25 ms
tTIMEOUT

fI2CCLK

TIMEOUTA[11:0]

TIDLE

TIMEOUTEN

8 MHz

0x61

0

1

98 x 2048 x 125 ns = 25 ms

16 MHz

0xC3

0

1

196 x 2048 x 62.5 ns = 25 ms

Table 668. TIMEOUTB[11:0] for maximum tLOW:SEXT and tLOW:MEXT of 8 ms
tLOW:SEXT
tLOW:MEXT

fI2CCLK

TIMEOUTB[11:0]

TEXTEN

8 MHz

0x1F

1

32 x 2048 x 125 ns = 8 ms

16 MHz

0x3F

1

64 x 2048 x 62.5 ns = 8 ms

Table 669. TIMEOUTA[11:0] for maximum tIDLE of 50 µs
tIDLE

fI2CCLK

TIMEOUTA[11:0]

TIDLE

TIMEOUTEN

8 MHz

0x63

1

1

100 x 4 x 125 ns = 50 µs

16 MHz

0xC7

1

1

200 x 4 x 62.5 ns = 50 µs

RM0456 Rev 6

<!-- pagebreak -->

2756

Inter-integrated circuit interface (I2C)

65.4.14

RM0456

SMBus target mode
In addition to I2C target transfer management (refer to Section 65.4.8: I2C target mode), this
section provides extra software flowcharts to support SMBus.

SMBus target transmitter
When using the I2C peripheral in SMBus mode, set the SBC bit to enable the PEC
transmission at the end of the programmed number of data bytes. When the PECBYTE bit
is set, the number of bytes programmed in NBYTES[7:0] includes the PEC transmission. In
that case, the total number of TXIS interrupts is NBYTES[7:0] - 1, and the content of the
I2C_PECR register is automatically transmitted if the controller requests an extra byte after
the transfer of the NBYTES[7:0] - 1 data bytes.
Caution:

The PECBYTE bit has no effect when the RELOAD bit is set.
Figure 803. Transfer sequence flow for SMBus target transmitter N bytes + PEC

SMBus target
transmission

Target initialization

No
I2C_ISR.ADDR
= 1?
Yes
Read ADDCODE and DIR in I2C_ISR
I2C_CR2.NBYTES = N + 1
PECBYTE = 1
Set I2C_ICR.ADDRCF

I2C_ISR.TXIS
= 1?

SCL
stretched

No

Yes
Write I2C_TXDR.TXDATA

MSv19867V3

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Inter-integrated circuit interface (I2C)
Figure 804. Transfer bus diagram for SMBus target transmitter (SBC = 1)
legend:

Example SMBus target transmitter 2 bytes + PEC

transmission
TXIS TXIS

ADDR

S

Address

reception

A

data1

EV1

EV2 EV3

NBYTES

A

data2

A

PEC

NA

P

SCL stretch

3

EV1: ADDR ISR: check ADDCODE, program NBYTES = 3, set PECBYTE, set ADDRCF
EV2: TXIS ISR: wr data1
EV3: TXIS ISR: wr data2
MSv19869V3

SMBus target receiver
When using the I2C peripheral in SMBus mode, set the SBC bit to enable the PEC checking
at the end of the programmed number of data bytes. To allow the ACK control of each byte,
the reload mode must be selected (RELOAD = 1). Refer to Target byte control mode for
more details.
To check the PEC byte, the RELOAD bit must be cleared and the PECBYTE bit must be set.
In this case, after the receipt of NBYTES[7:0] - 1 data bytes, the next received byte is
compared with the internal I2C_PECR register content. A NACK is automatically generated
if the comparison does not match, and an ACK is automatically generated if the comparison
matches, whatever the ACK bit value. Once the PEC byte is received, it is copied into the
I2C_RXDR register like any other data, and the RXNE flag is set.
Upon a PEC mismatch, the PECERR flag is set and an interrupt is generated if the ERRIE
bit of the I2C_CR1 register is set.
If no ACK software control is required, the user can set the PECBYTE bit and, in the same
write operation, load NBYTES[7:0] with the number of bytes to receive in a continuous flow.
After the receipt of NBYTES[7:0] - 1 bytes, the next received byte is checked as being the
PEC.
Caution:

The PECBYTE bit has no effect when the RELOAD bit is set.

RM0456 Rev 6

<!-- pagebreak -->

2756

Inter-integrated circuit interface (I2C)

RM0456

Figure 805. Transfer sequence flow for SMBus target receiver N bytes + PEC
SMBus target
reception

Target initialization

No
I2C_ISR.ADDR
= 1?
Yes
Read ADDCODE and DIR in I2C_ISR
I2C_CR2.NBYTES = 1, RELOAD = 1
PECBYTE = 1
Set I2C_ICR.ADDRCF

I2C_ISR.RXNE =1?
I2C_ISR.TCR = 1?

SCL
stretched

No

Yes
Read I2C_RXDR.RXDATA
Program I2C_CR2.NACK = 0
I2C_CR2.NBYTES = 1
N=N-1

N = 1?

No

Yes
Read I2C_RXDR.RXDATA
Program RELOAD = 0
NACK = 0 and NBYTES = 1

No
I2C_ISR.RXNE
= 1?
Yes
Read I2C_RXDR.RXDATA

End
MSv19868V3

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Inter-integrated circuit interface (I2C)
Figure 806. Bus transfer diagrams for SMBus target receiver (SBC = 1)
legend:

Example SMBus target receiver 2 bytes + PEC

transmission
ADDR

RXNE

S Address

A

data1

A

EV1

data2

A

EV2

NBYTES

RXNE

RXNE

reception

PEC

A

EV3

EV4

SCL stretch

P

3

EV1: ADDR ISR: check ADDCODE and DIR, program NBYTES = 3, PECBYTE = 1, RELOAD = 0, set ADDRCF
EV2: RXNE ISR: rd data1
EV3: RXNE ISR: rd data2
EV4: RXNE ISR: rd PEC

legend :

Example SMBus target receiver 2 bytes + PEC, with ACK control
(RELOAD = 1/0)
ADDR
S Address

RXNE,TCR
A

data1

EV1
NBYTES

A

RXNE,TCR
data2

EV2

A
EV3

transmission

RXNE
PEC

A

reception
P

SCL stretch

EV4

1

EV1: ADDR ISR: check ADDCODE and DIR, program NBYTES = 1, PECBYTE = 1, RELOAD = 1, set ADDRCF
EV2: RXNE-TCR ISR: rd data1, program NACK=0 and NBYTES = 1
EV3: RXNE-TCR ISR: rd data2, program NACK=0, NBYTES = 1 and RELOAD = 0
EV4: RXNE-TCR ISR: rd PEC
MSv19870V3

65.4.15

SMBus controller mode
In addition to I2C controller transfer management (refer to Section 65.4.9: I2C controller
mode), this section provides extra software flowcharts to support SMBus.

SMBus controller transmitter
When the SMBus controller wants to transmit the PEC, the PECBYTE bit must be set and
the number of bytes must be loaded in the NBYTES[7:0] bitfield, before setting the START
bit. In this case, the total number of TXIS interrupts is NBYTES[7:0] - 1. So if the PECBYTE
bit is set when NBYTES[7:0] = 0x1, the content of the I2C_PECR register is automatically
transmitted.
If the SMBus controller wants to send a STOP condition after the PEC, the automatic end
mode must be selected (AUTOEND = 1). In this case, the STOP condition automatically
follows the PEC transmission.
RM0456 Rev 6

<!-- pagebreak -->

2756

Inter-integrated circuit interface (I2C)

RM0456

When the SMBus controller wants to send a RESTART condition after the PEC, the
software mode must be selected (AUTOEND = 0). In this case, once NBYTES[7:0] - 1 are
transmitted, the I2C_PECR register content is transmitted. The TC flag is set after the PEC
transmission, stretching the SCL line low. The RESTART condition must be programmed in
the TC interrupt subroutine.
Caution:

The PECBYTE bit has no effect when the RELOAD bit is set.
Figure 807. Bus transfer diagrams for SMBus controller transmitter

Example SMBus controller transmitter 2 bytes + PEC, automatic end mode (STOP)
TXIS TXIS
S Address A

legend:

data1

A

data2

A

PEC

A

P

transmission
reception

EV1

INIT

EV2
SCL stretch

TXE

NBYTES xx

3

INIT: program target address, program NBYTES = 3, AUTOEND=1, set PECBYTE, set START
EV1: TXIS ISR: wr data1
EV2: TXIS ISR: wr data2

Example SMBus controller transmitter 2 bytes + PEC, software end mode (RESTART)
legend:

TC

TXIS TXIS

transmission
S Address A
INIT

xx

data1

A

data2

A

PEC

A

EV1 EV2

Rstart Address
EV3

reception
SCL stretch

N

3

NBYTES
INIT: program target address, program NBYTES = 3, AUTOEND=0, set PECBYTE, set START
EV1: TXIS ISR: wr data1
EV2: TXIS ISR: wr data2
EV3: TC ISR: program target address, program NBYTES = N, set START

MSv19871V3

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Inter-integrated circuit interface (I2C)

SMBus controller receiver
When the SMBus controller wants to receive, at the end of the transfer, the PEC followed by
a STOP condition, the automatic end mode can be selected (AUTOEND = 1). The
PECBYTE bit must be set and the target address programmed before setting the START bit.
In this case, after the receipt of NBYTES[7:0] - 1 data bytes, the next received byte is
automatically checked versus the I2C_PECR register content. A NACK response is given to
the PEC byte, followed by a STOP condition.
When the SMBus controller receiver wants to receive, at the end of the transfer, the PEC
byte followed by a RESTART condition, the software mode must be selected (AUTOEND =
0). The PECBYTE bit must be set and the target address programmed before setting the
START bit. In this case, after the receipt of NBYTES[7:0] - 1 data bytes, the next received
byte is automatically checked versus the I2C_PECR register content. The TC flag is set
after the PEC byte reception, stretching the SCL line low. The RESTART condition can be
programmed in the TC interrupt subroutine.
Caution:

The PECBYTE bit has no effect when the RELOAD bit is set.

RM0456 Rev 6

<!-- pagebreak -->

2756

Inter-integrated circuit interface (I2C)

RM0456

Figure 808. Bus transfer diagrams for SMBus controller receiver
Example SMBus controller receiver 2 bytes + PEC, automatic end mode (STOP)
RXNE

Address

S

A

data1

A

RXNE

data2

A

RXNE
PEC

NA

legend:
P

transmission
reception

INIT

EV2

EV1

EV3
SCL stretch

NBYTES xx

3

INIT: program target address, program NBYTES = 3, AUTOEND=1, set PECBYTE, set START
EV1: RXNE ISR: rd data1
EV2: RXNE ISR: rd data2
EV3: RXNE ISR: rd PEC

Example SMBus controller receiver 2 bytes + PEC, software end mode (RESTART)
RXNE

RXNE

legend:

RXNE TC

transmission
S Address A

data1

data2

A
EV1

INIT

A

PEC

EV2

NA
EV3

Restart Address
EV4

reception
SCL stretch

NBYTES
xx

3

N

INIT: program target address, program NBYTES = 3, AUTOEND = 0, set PECBYTE, set START
EV1: RXNE ISR: rd data1
EV2: RXNE ISR: rd data2
EV3: RXNE ISR: read PEC
EV4: TC ISR: program target address, program NBYTES = N, set START
MSv19872V3

65.4.16

Autonomous mode
The I2C peripheral can autonomously operate as controller or target in Stop mode as long
as NOSTRETCH = 0. The autonomous mode is not supported when NOSTRETCH = 1.
Besides Stop mode, the autonomous mode can also be used in Run and Sleep modes.
The APB clock is requested by the peripheral each time the I2C status needs to be updated.
Once the APB clock is received by the peripheral, either an interrupt or a DMA request is
generated, depending on the I2C configuration.
If an interrupt is generated, the device wakes up from Stop mode.
If there is no interrupt, the device remains in Stop mode, but the kernel and AHB/APB clocks
are available for the I2C peripheral and all the autonomous peripherals enabled in the RCC.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Inter-integrated circuit interface (I2C)
If the DMA requests are enabled, the data are DMA-transferred to or from the SRAM, while
the device remains in Stop mode.

Target mode
In target mode, the autonomous mode is enabled in Stop mode if WUPEN = 1 in the
I2C_CR1. This mode is supported only when NOSTRETCH = 0.
The kernel clock is requested by the peripheral on START detection during all the transfer
until the STOP condition occurs, when the target is addressed. If the target is not
addressed, the kernel clock request is released after the address phase.
To optimize the functionality in target mode by using only DMA transfers, it is possible to set
the ADDRACLR and STOPFACLR bits in the I2C_CR1 in order to avoid to serve the
Address match (ADDR) and STOP detection (STOPF) events.
Note:

If the I2C clock is the system clock, or if WUPEN = 0, the internal oscillator is not switched
on after a START is received in target mode.

Controller mode
In controller mode, a transfer can be automatically launched when an asynchronous trigger
is detected in Run, Sleep or Stop mode. The trigger, selected by TRIGSEL in the
I2C_AUTOCR register, generates a kernel clock request to allow the transfer, and launches
a START condition and the I2C transfer as defined in the I2C_CR2 register. The kernel clock
is requested until the STOP condition occurs.
To avoid waking up the CPU too often, it is possible to replace some interrupts by DMA
requests, to make some control register write actions. In controller mode, transfer complete
(TC) and transfer complete reload (TCR) events can generate a DMA request when the
corresponding TCDMAEN or TCRDMAEN bits of the I2C_AUTOCR register are set.
Consequently, the I2C_CR2 register can be written thanks to a DMA transfer, to program a
new transfer (in TC event) or to reload the number of bytes (in TCR event).
In case a trigger is enabled, but the application needs to take the control back to start a
different transfer, the following steps are required before writing in the I2C_CR2 register to
launch the new transfer:
1.

Wait for BUSY = 0 in the I2C_ISR register

2.

Clear the TRIGEN bit of the I2C_AUTOCR register

3.

Wait for longer than tBUF (bus free time between a STOP and a START condition)

4.

Wait for BUSY = 0 in the I2C_ISR register

Caution:

When the device is in Stop mode, the I2C peripheral receives its kernel clock only when it is
implicated in the transfer. Consequently, features such as timeouts and bus idle detection
are not reliable in Stop mode.

Caution:

The digital filter is not compatible with the functionality in Stop mode. Before entering Stop
mode with the WUPEN bit set, deactivate the digital filter, by writing zero to the DNF[3:0]
bitfield.

Caution:

Clock stretching must be enabled (NOSTRETCH = 0) to ensure proper operation of the
wake-up from Stop mode feature.

Caution:

If wake-up from Stop mode is disabled (WUPEN = 0), the I2C peripheral must be disabled
before entering Stop mode (PE = 0).

RM0456 Rev 6

<!-- pagebreak -->

2756

Inter-integrated circuit interface (I2C)

65.4.17

RM0456

Error conditions
The following errors are the conditions that can cause the communication to fail.

Bus error (BERR)
A bus error is detected when a START or a STOP condition is detected and is not located
after a multiple of nine SCL clock pulses. START or STOP condition is detected when an
SDA edge occurs while SCL is high.
The bus error flag is set only if the I2C peripheral is involved in the transfer as controller or
addressed target (that is, not during the address phase in target mode).
In case of a misplaced START or RESTART detection in target mode, the I2C peripheral
enters address recognition state like for a correct START condition.
When a bus error is detected, the BERR flag of the I2C_ISR register is set, and an interrupt
is generated if the ERRIE bit of the I2C_CR1 register is set.

Arbitration loss (ARLO)
An arbitration loss is detected when a high level is sent on the SDA line, but a low level is
sampled on the SCL rising edge.
In controller mode, arbitration loss is detected during the address phase, data phase and
data acknowledge phase. In this case, the SDA and SCL lines are released, the START
control bit is cleared by hardware and the controller switches automatically to target mode.
In target mode, arbitration loss is detected during data phase and data acknowledge phase.
In this case, the transfer is stopped and the SCL and SDA lines are released.
When an arbitration loss is detected, the ARLO flag of the I2C_ISR register is set and an
interrupt is generated if the ERRIE bit of the I2C_CR1 register is set.

Overrun/underrun error (OVR)
An overrun or underrun error is detected in target mode when NOSTRETCH = 1 and:
•

In reception when a new byte is received and the RXDR register has not been read yet.
The new received byte is lost, and a NACK is automatically sent as a response to the
new byte.

•

In transmission:
–

When STOPF = 1 and the first data byte must be sent. The content of the
I2C_TXDR register is sent if TXE = 0, 0xFF if not.

–

When a new byte must be sent and the I2C_TXDR register has not been written
yet, 0xFF is sent.

When an overrun or underrun error is detected, the OVR flag of the I2C_ISR register is set
and an interrupt is generated if the ERRIE bit of the I2C_CR1 register is set.

Packet error checking error (PECERR)
A PEC error is detected when the received PEC byte does not match the I2C_PECR
register content. A NACK is automatically sent after the wrong PEC reception.
When a PEC error is detected, the PECERR flag of the I2C_ISR register is set and an
interrupt is generated if the ERRIE bit of the I2C_CR1 register is set.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Inter-integrated circuit interface (I2C)

Timeout error (TIMEOUT)
A timeout error occurs for any of these conditions:
•

TIDLE = 0 and SCL remains low for the time defined in the TIMEOUTA[11:0] bitfield:
this is used to detect an SMBus timeout.

•

TIDLE = 1 and both SDA and SCL remains high for the time defined in the TIMEOUTA
[11:0] bitfield: this is used to detect a bus idle condition.

•

Controller cumulative clock low extend time reaches the time defined in the
TIMEOUTB[11:0] bitfield (SMBus tLOW:MEXT parameter).

•

Target cumulative clock low extend time reaches the time defined in the
TIMEOUTB[11:0] bitfield (SMBus tLOW:SEXT parameter).

When a timeout violation is detected in controller mode, a STOP condition is automatically
sent.
When a timeout violation is detected in target mode, the SDA and SCL lines are
automatically released.
When a timeout error is detected, the TIMEOUT flag is set in the I2C_ISR register and an
interrupt is generated if the ERRIE bit of the I2C_CR1 register is set.

Alert (ALERT)
The ALERT flag is set when the I2C peripheral is configured as a host (SMBHEN = 1), the
SMBALERT# signal detection is enabled (ALERTEN = 1), and a falling edge is detected on
the SMBA pin. An interrupt is generated if the ERRIE bit of the I2C_CR1 register is set.

65.5

I2C in low-power modes
Table 670. Effect of low-power modes to I2C
Mode

Description

Sleep

No effect. I2C interrupts cause the device to exit the Sleep mode.

Stop(1)

The contents of I2C registers are kept. If the autonomous mode is enabled and
I2C is clocked by an internal oscillator available in Stop mode: transfers in
controller and in target modes are functional. DMA requests are functional, and
the interrupts cause the device to exit the Stop mode. If WUPEN = 0: the I2C must
be disabled before entering Stop mode.

Standby

The I2C peripheral is powered down. It must be reinitialized after exiting Standby
mode.

1. Refer to Section 65.3: I2C implementation for information about the Stop modes supported by each
instance. If the wake-up from a specific Stop mode is not supported, the instance must be disabled before
entering that specific Stop mode.

RM0456 Rev 6

<!-- pagebreak -->

2756

Inter-integrated circuit interface (I2C)

65.6

RM0456

I2C interrupts
The following table gives the list of I2C interrupt requests.
Table 671. I2C interrupt requests

Interrupt
acronym

Interrupt event

Event flag

Enable
control bit

Interrupt clear
method

Receive buffer not
empty

RXNE

RXIE

Read I2C_RXDR
register

Transmit buffer
interrupt status

TXIS

TXIE

Write I2C_TXDR
register

STOP detection
interrupt flag

STOPF

STOPIE

Write
STOPCF = 1

Transfer complete
reload

TCR
TCIE

I2C_EV

I2C

Write I2C_CR2
with
NBYTES[7:0] = 0

Exit the
Sleep
mode

Exit the
Stop
mode

Exit the
Stop3,
Standby,
Shutdown
mode

Yes

Yes(1)

No

Yes

Yes(1)

No

Write START = 1
or STOP = 1

Transfer complete

TC

Address matched

ADDR

ADDRIE

Write
ADDRCF=1

NACK reception

NACKF

NACKIE

Write NACKCF=1

Address matched

ADDR

ADDRIE

Write
ADDRCF=1

NACK reception

NACKF

NACKIE

Write NACKCF=1

Bus error

BERR

Write BERRCF=1

Arbitration loss

ARLO

Write ARLOCF=1

Overrun/underrun

OVR

Write OVRCF=1
Write
PECERRCF=1

I2C_ERR PEC error

PECERR

Timeout/
tLOW error

TIMEOUT

Write
TIMEOUTCF=1

SMBus alert

ALERT

Write
ALERTCF=1

ERRIE

1. Refer to the I2C implementation table for information about wake-up from Stop mode support per instance.

65.7

I2C DMA requests

65.7.1

Transmission using DMA
DMA (direct memory access) can be enabled for transmission by setting the TXDMAEN bit
of the I2C_CR1 register. Data is loaded from an SRAM area configured through the DMA

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Inter-integrated circuit interface (I2C)
peripheral (see Section 17: General purpose direct memory access controller (GPDMA)) to
the I2C_TXDR register whenever the TXIS bit is set.
Only the data are transferred with DMA.
In controller mode, the initialization, the target address, direction, number of bytes and
START bit are programmed by software (the transmitted target address cannot be
transferred with DMA). When all data are transferred using DMA, DMA must be initialized
before setting the START bit. The end of transfer is managed with the NBYTES counter.
Refer to Controller transmitter.
In target mode:
•

With NOSTRETCH = 0, when all data are transferred using DMA, DMA must be
initialized before the address match event, or in ADDR interrupt subroutine, before
clearing ADDR.

•

With NOSTRETCH = 1, the DMA must be initialized before the address match event.

The PEC transfer is managed with the counter associated to the NBYTES[7:0] bitfield. Refer
to SMBus target transmitter and SMBus controller transmitter.
Note:

If DMA is used for transmission, it is not required to set the TXIE bit.

65.7.2

Reception using DMA
DMA (direct memory access) can be enabled for reception by setting the RXDMAEN bit of
the I2C_CR1 register. Data is loaded from the I2C_RXDR register to an SRAM area
configured through the DMA peripheral (refer to Section 17: General purpose direct memory
access controller (GPDMA)) whenever the RXNE bit is set. Only the data (including PEC)
are transferred with DMA.
In controller mode, the initialization, the target address, direction, number of bytes and
START bit are programmed by software. When all data are transferred using DMA, DMA
must be initialized before setting the START bit. The end of transfer is managed with the
NBYTES counter.
In target mode with NOSTRETCH = 0, when all data are transferred using DMA, DMA must
be initialized before the address match event, or in the ADDR interrupt subroutine, before
clearing the ADDR flag.
The PEC transfer is managed with the counter associated to the NBYTES[7:0] bitfield. Refer
to SMBus target receiver and SMBus controller receiver.

Note:

If DMA is used for reception, it is not required to set the RXIE bit.

65.7.3

Controller event control using DMA
In controller mode, the transfer can be automatically managed while the device is in Run,
Sleep or Stop mode, thanks to TC and TCR DMA requests.
If the TCDMAEN bit of the I2C_AUTOCR register is set, the I2C_EVC (I2C control event)
DMA request is generated when the TC bit of the I2C_ISR register is set. The DMA
controller must be programmed to write the next command in the I2C_CR2 register. If both
STOP and START bits are set in the new I2C_CR2 command, a STOP condition followed by
a START condition is sent, followed by the address, direction and number of bytes defined
in I2C_CR2. If only START bit is set in the new I2C_CR2 command, a RESTART condition
is sent, followed by the address, direction and number of bytes defined in I2C_CR2.

RM0456 Rev 6

<!-- pagebreak -->

2756

Inter-integrated circuit interface (I2C)

RM0456

If the TCRDMAEN bit of the I2C_AUTOCR register is set, the I2C_EVC (I2C control event)
DMA request is generated when TCR bit of the I2C_ISR register is set. The DMA controller
must be programmed to write the remaining number of bytes to transfer in the I2C_CR2
register.

65.8

I2C debug modes
When the device enters debug mode (core halted), the SMBus timeout either continues
working normally or stops, depending on the DBG_I2Cx_STOP bits in the DBG block.

65.9

I2C registers
Refer to Section 1.2 for the list of abbreviations used in register descriptions.
The registers are accessed by words (32-bit).

65.9.1

I2C control register 1 (I2C_CR1)
Address offset: 0x00
Reset value: 0x0000 0000
Access: no wait states, except if a write access occurs while a write access is ongoing. In
this case, wait states are inserted in the second write access, until the previous one is
completed. The latency of the second write access can be up to
2 x i2c_pclk + 6 x i2c_ker_ck.

31

30

STOPF ADDRA
ACLR
CLR
rw

rw

15

14

RXDM TXDMA
AEN
EN
rw

rw

29

28

27

26

25

Res.

Res.

Res.

Res.

Res.

13

12

11

10

9

Res.

ANF
OFF
rw

24
FMP

rw

rw

22

ALERT
PECEN
EN

21

20

SMBD
EN

SMBH
EN

19
GCEN

18

17

WUPE NOSTR
N
ETCH

16
SBC

rw

rw

rw

rw

rw

rw

rw

rw

rw

8

7

6

5

4

3

2

1

0

ERRIE

TCIE

STOP
IE

NACKI
E

ADDRI
E

RXIE

TXIE

PE

rw

rw

rw

rw

rw

rw

rw

rw

DNF[3:0]
rw

23

rw

Bit 31 STOPFACLR: STOP detection flag (STOPF) automatic clear
0: STOPF flag is set by hardware, cleared by software by setting STOPCF bit.
1: STOPF flag remains cleared by hardware. This mode can be used in NOSTRETCH target
mode, to avoid the overrun error if the STOPF flag is not cleared before next data
transmission. This allows a target data management by DMA only, without any interrupt from
peripheral.
Bit 30 ADDRACLR: Address match flag (ADDR) automatic clear
0: ADDR flag is set by hardware, cleared by software by setting ADDRCF bit.
1: ADDR flag remains cleared by hardware. This mode can be used in target mode, to avoid
the ADDR clock stretching if the I2C enables only one target address. This allows a target
data management by DMA only, without any interrupt from peripheral.
Bits 29:25 Reserved, must be kept at reset value.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Inter-integrated circuit interface (I2C)

Bit 24 FMP: Fast-mode Plus 20 mA drive enable
0: 20 mA I/O drive disabled
1: 20 mA I/O drive enabled
Bit 23 PECEN: PEC enable
0: PEC calculation disabled
1: PEC calculation enabled
Bit 22 ALERTEN: SMBus alert enable
0: The SMBALERT# signal on SMBA pin is not supported in host mode (SMBHEN = 1). In
device mode (SMBHEN = 0), the SMBA pin is released and the alert response address
header is disabled (0001100x followed by NACK).
1: The SMBALERT# signal on SMBA pin is supported in host mode (SMBHEN = 1). In
device mode (SMBHEN = 0), the SMBA pin is driven low and the alert response address
header is enabled (0001100x followed by ACK).
Note: When ALERTEN = 0, the SMBA pin can be used as a standard GPIO.
Bit 21 SMBDEN: SMBus device default address enable
0: Device default address disabled. Address 0b1100001x is NACKed.
1: Device default address enabled. Address 0b1100001x is ACKed.
Bit 20 SMBHEN: SMBus host address enable
0: Host address disabled. Address 0b0001000x is NACKed.
1: Host address enabled. Address 0b0001000x is ACKed.
Bit 19 GCEN: General call enable
0: General call disabled. Address 0b00000000 is NACKed.
1: General call enabled. Address 0b00000000 is ACKed.
Bit 18 WUPEN: Wake-up from Stop mode enable
0: Wake-up from Stop mode disabled.
1: Wake-up from Stop mode enabled.
Note: WUPEN can be set only when DNF[3:0] = 0000.
Bit 17 NOSTRETCH: Clock stretching disable
This bit is used to disable clock stretching in target mode. It must be kept cleared in
controller mode.
0: Clock stretching enabled
1: Clock stretching disabled
Note: This bit can be programmed only when the I2C peripheral is disabled (PE = 0).
Bit 16 SBC: Target byte control
This bit is used to enable hardware byte control in target mode.
0: Target byte control disabled
1: Target byte control enabled
Bit 15 RXDMAEN: DMA reception requests enable
0: DMA mode disabled for reception
1: DMA mode enabled for reception
Bit 14 TXDMAEN: DMA transmission requests enable
0: DMA mode disabled for transmission
1: DMA mode enabled for transmission
Bit 13 Reserved, must be kept at reset value.

RM0456 Rev 6

<!-- pagebreak -->

2756

Inter-integrated circuit interface (I2C)

RM0456

Bit 12 ANFOFF: Analog noise filter OFF
0: Analog noise filter enabled
1: Analog noise filter disabled
Note: This bit can be programmed only when the I2C peripheral is disabled (PE = 0).
Bits 11:8 DNF[3:0]: Digital noise filter
These bits are used to configure the digital noise filter on SDA and SCL input. The digital
filter, filters spikes with a length of up to DNF[3:0] * tI2CCLK
0000: Digital filter disabled
0001: Digital filter enabled and filtering capability up to one tI2CCLK
...
1111: digital filter enabled and filtering capability up to fifteen tI2CCLK
Note: If the analog filter is enabled, the digital filter is added to it. This filter can be
programmed only when the I2C peripheral is disabled (PE = 0).
Bit 7 ERRIE: Error interrupts enable
0: Error detection interrupts disabled
1: Error detection interrupts enabled
Note: Any of these errors generates an interrupt:
- arbitration loss (ARLO)
- bus error detection (BERR)
- overrun/underrun (OVR)
- timeout detection (TIMEOUT)
- PEC error detection (PECERR)
- alert pin event detection (ALERT)
Bit 6 TCIE: Transfer complete interrupt enable
0: Transfer complete interrupt disabled
1: Transfer complete interrupt enabled
Note: Any of these events generates an interrupt:
Transfer complete (TC)
Transfer complete reload (TCR)
Bit 5 STOPIE: STOP detection interrupt enable
0: STOP detection (STOPF) interrupt disabled
1: STOP detection (STOPF) interrupt enabled
Bit 4 NACKIE: Not acknowledge received interrupt enable
0: Not acknowledge (NACKF) received interrupts disabled
1: Not acknowledge (NACKF) received interrupts enabled
Bit 3 ADDRIE: Address match interrupt enable (target only)
0: Address match (ADDR) interrupts disabled
1: Address match (ADDR) interrupts enabled
Bit 2 RXIE: RX interrupt enable
0: Receive (RXNE) interrupt disabled
1: Receive (RXNE) interrupt enabled
Bit 1 TXIE: TX interrupt enable
0: Transmit (TXIS) interrupt disabled
1: Transmit (TXIS) interrupt enabled

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Inter-integrated circuit interface (I2C)

Bit 0 PE: Peripheral enable
0: Peripheral disabled
1: Peripheral enabled
Note: When PE = 0, the I2C SCL and SDA lines are released. Internal state machines and
status bits are put back to their reset value. When cleared, PE must be kept low for at
least three APB clock cycles.

65.9.2

I2C control register 2 (I2C_CR2)
Address offset: 0x04
Reset value: 0x0000 0000
Access: no wait states, except if a write access occurs while a write access is ongoing. In
this case, wait states are inserted in the second write access until the previous one is
completed. The latency of the second write access can be up to 2 x i2c_pclk +
6 x i2c_ker_ck.

31

30

29

28

27

Res.

Res.

Res.

Res.

Res.

15

14

13

12

11

NACK

STOP

START

rs

rs

rs

HEAD1
ADD10
0R
rw

rw

26

25

24

23

22

21

PECBY AUTOE RELOA
TE
ND
D

20

19

18

17

16

NBYTES[7:0]

rs

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

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

rw

rw

rw

rw

RD_W
RN
rw

SADD[9:0]
rw

rw

rw

rw

rw

rw

Bits 31:27 Reserved, must be kept at reset value.
Bit 26 PECBYTE: Packet error checking byte
This bit is set by software, and cleared by hardware when the PEC is transferred, or when a
STOP condition or an Address matched is received, also when PE = 0.
0: No PEC transfer
1: PEC transmission/reception is requested
Note: Writing 0 to this bit has no effect.
This bit has no effect when RELOAD is set, and in target mode when SBC = 0.
Bit 25 AUTOEND: Automatic end mode (controller mode)
This bit is set and cleared by software.
0: software end mode: TC flag is set when NBYTES data are transferred, stretching SCL low.
1: Automatic end mode: a STOP condition is automatically sent when NBYTES data are
transferred.
Note: This bit has no effect in target mode or when the RELOAD bit is set.
Bit 24 RELOAD: NBYTES reload mode
This bit is set and cleared by software.
0: The transfer is completed after the NBYTES data transfer (STOP or RESTART follows).
1: The transfer is not completed after the NBYTES data transfer (NBYTES is reloaded). TCR
flag is set when NBYTES data are transferred, stretching SCL low.
Bits 23:16 NBYTES[7:0]: Number of bytes
The number of bytes to be transmitted/received is programmed there. This field is don’t care
in target mode with SBC = 0.
Note: Changing these bits when the START bit is set is not allowed.

RM0456 Rev 6

<!-- pagebreak -->

2756

Inter-integrated circuit interface (I2C)

RM0456

Bit 15 NACK: NACK generation (target mode)
The bit is set by software, cleared by hardware when the NACK is sent, or when a STOP
condition or an Address matched is received, or when PE = 0.
0: an ACK is sent after current received byte.
1: a NACK is sent after current received byte.
Note: Writing 0 to this bit has no effect.
This bit is used only in target mode: in controller receiver mode, NACK is automatically
generated after last byte preceding STOP or RESTART condition, whatever the NACK
bit value.
When an overrun occurs in target receiver NOSTRETCH mode, a NACK is
automatically generated, whatever the NACK bit value.
When hardware PEC checking is enabled (PECBYTE = 1), the PEC acknowledge value
does not depend on the NACK value.
Bit 14 STOP: STOP condition generation
This bit only pertains to controller mode. It is set by software and cleared by hardware when
a STOP condition is detected or when PE = 0.
0: No STOP generation
1: STOP generation after current byte transfer
Note: Writing 0 to this bit has no effect.
Bit 13 START: START condition generation
This bit is set by software. It is cleared by hardware after the START condition followed by
the address sequence is sent, by an arbitration loss, by an address matched in target mode,
by a timeout error detection, or when PE = 0.
0: No START generation
1: RESTART/START generation:
If the I2C is already in controller mode with AUTOEND = 0, setting this bit generates a
repeated START condition when RELOAD = 0, after the end of the NBYTES transfer.
Otherwise, setting this bit generates a START condition once the bus is free.
Note: Writing 0 to this bit has no effect.
The START bit can be set even if the bus is BUSY or I2C is in target mode.
This bit has no effect when RELOAD is set.
Bit 12 HEAD10R: 10-bit address header only read direction (controller receiver mode)
0: The controller sends the complete 10-bit target address read sequence: START + 2 bytes
10-bit address in write direction + RESTART + first seven bits of the 10-bit address in read
direction.
1: The controller sends only the first seven bits of the 10-bit address, followed by read
direction.
Note: Changing this bit when the START bit is set is not allowed.
Bit 11 ADD10: 10-bit addressing mode (controller mode)
0: The controller operates in 7-bit addressing mode
1: The controller operates in 10-bit addressing mode
Note: Changing this bit when the START bit is set is not allowed.
Bit 10 RD_WRN: Transfer direction (controller mode)
0: Controller requests a write transfer
1: Controller requests a read transfer
Note: Changing this bit when the START bit is set is not allowed.

<!-- pagebreak -->

