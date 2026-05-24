2756

Inter-integrated circuit interface (I2C)

RM0456

Host notify protocol
To enable the host notify protocol, set the SMBHEN bit of the I2C_CR1 register. The I2C
peripheral then acknowledges the SMBus host address (0b0001 000).
When this protocol is used, the device acts as a controller and the host as a target.

SMBus alert
The I2C peripheral supports the SMBALERT# optional signal through the SMBA pin. With
the SMBALERT# signal, an SMBus target device can signal to the SMBus host that it wants
to talk. The host processes the interrupt and simultaneously accesses all SMBALERT#
devices through the alert response address (0b0001 100). Only the device/devices which
pulled SMBALERT# low acknowledges/acknowledge the alert response address.
When the I2C peripheral is configured as an SMBus target device (SMBHEN = 0), the
SMBA pin is pulled low by setting the ALERTEN bit of the I2C_CR1 register. The alert
response address is enabled at the same time.
When the I2C peripheral is configured as an SMBus host (SMBHEN = 1), the ALERT flag of
the I2C_ISR register is set when a falling edge is detected on the SMBA pin and ALERTEN
= 1. An interrupt is generated if the ERRIE bit of the I2C_CR1 register is set. When
ALERTEN = 0, the alert line is considered high even if the external SMBA pin is low.
Note:

If the SMBus alert pin is not required, keep the ALERTEN bit cleared. The SMBA pin can
then be used as a standard GPIO.

Packet error checking
A packet error checking mechanism introduced in the SMBus specification improves
reliability and communication robustness. The packet error checking is implemented by
appending a packet error code (PEC) at the end of each message transfer. The PEC is
calculated by using the C(x) = x8 + x2 + x + 1 CRC-8 polynomial on all the message bytes
(including addresses and read/write bits).
The I2C peripheral embeds a hardware PEC calculator and allows a not acknowledge to be
sent automatically when the received byte does not match the hardware calculated PEC.

Timeouts
To comply with the SMBus timeout specifications, the I2C peripheral embeds hardware
timers.
Table 665. SMBus timeout specifications
Limits
Symbol
tTIMEOUT

Parameter

Unit
Min

Max

Detect clock low timeout

25

35

tLOW:SEXT(1)

Cumulative clock low extend time (target device)

-

25

tLOW:MEXT(2)

Cumulative clock low extend time (controller
device)

-

10

ms

1. tLOW:SEXT is the cumulative time a given target device is allowed to extend the clock cycles in one message
from the initial START to the STOP. It is possible that another target device or the controller also extends
the clock causing the combined clock low extend time to be greater than tLOW:SEXT. The value provided
applies to a single target device connected to a full-target controller.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Inter-integrated circuit interface (I2C)

2. tLOW:MEXT is the cumulative time a controller device is allowed to extend its clock cycles within each byte of
a message as defined from START-to-ACK, ACK-to-ACK, or ACK-to-STOP. It is possible that a target
device or another controller also extends the clock, causing the combined clock low time to be greater than
tLOW:MEXT on a given byte. The value provided applies to a single target device connected to a full-target
controller.

Figure 802. Timeout intervals for tLOW:SEXT, tLOW:MEXT
Start

Stop
tLOW:SEXT
ClkAck
tLOW:MEXT

ClkAck
tLOW:MEXT

tLOW:MEXT

SMBCLK

SMBDAT

MS19866V1

Bus idle detection
A controller can assume that the bus is free if it detects that the clock and data signals have
been high for tIDLE > tHIGH(max) (refer to the table in Section 65.4.9).
This timing parameter covers the condition where a controller is dynamically added to the
bus, and may not have detected a state transition on the SMBCLK or SMBDAT lines. In this
case, the controller must wait long enough to ensure that a transfer is not currently in
progress. The I2C peripheral supports a hardware bus idle detection.

65.4.12

SMBus initialization
In addition to the I2C initialization for the I²C-bus, the use of the peripheral for the SMBus
communication requires some extra initialization steps.

Received command and data acknowledge control (target mode)
An SMBus receiver must be able to NACK each received command or data. To allow ACK
control in target mode, the target byte control mode must be enabled, by setting the SBC bit
of the I2C_CR1 register. Refer to Target byte control mode for more details.

Specific addresses (target mode)
The specific SMBus addresses must be enabled if required. Refer to Bus idle detection for
more details.

RM0456 Rev 6

<!-- pagebreak -->

