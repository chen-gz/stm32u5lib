2756

Inter-integrated circuit interface (I2C)

RM0456

Figure 800. Transfer sequence flow for I2C controller receiver, N > 255 bytes
Controller
reception

Controller initialization

NBYTES = 0xFF; N=N-255
RELOAD =1
Configure target address
Set I2C_CR2.START

I2C_ISR.RXNE
=1?

No

Yes
Read I2C_RXDR

NBYTES
received?

No

Yes
Yes
I2C_ISR.TC =
1?
Set I2C_CR2.START with
target address NBYTES
...

No
No
I2C_ISR.TCR
= 1?
Yes
IF N< 256
NBYTES =N; N=0;RELOAD=0
AUTOEND=0 for RESTART; 1 for STOP
ELSE
NBYTES =0xFF;N=N-255
RELOAD=1

End
MSv19864V3

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Inter-integrated circuit interface (I2C)
Figure 801. Transfer bus diagrams for I2C controller receiver
(mandatory events only)
Example I2C controller receiver 2 bytes, automatic end mode (STOP)
RXNE

S

Address

A

data1

RXNE

data2

A

NA

legend:
P

transmission
reception

INIT

EV2

EV1

SCL stretch
NBYTES xx

2

INIT: program target address, program NBYTES = 2, AUTOEND=1, set START
EV1: RXNE ISR: rd data1
EV2: RXNE ISR: rd data2

Example I2C controller receiver 2 bytes, software end mode (RESTART)
RXNE

RXNE

legend:

TC

transmission
S

Address

A

data1

INIT

data2

A

EV1

NA

ReS

Address

EV2

reception
SCL stretch

NBYTES
xx

2

N

INIT: program target address, program NBYTES = 2, AUTOEND=0, set START
EV1: RXNE ISR: rd data1
EV2: RXNE ISR: read data2
EV3: TC ISR: program target address, program NBYTES = N, set START

MSv19865V2

65.4.10

I2C_TIMINGR register configuration examples
The following tables provide examples of how to program the I2C_TIMINGR register to
obtain timings compliant with the I²C-bus specification. To get more accurate configuration
values, use the STM32CubeMX tool (I2C Configuration window).

RM0456 Rev 6

<!-- pagebreak -->

2756

Inter-integrated circuit interface (I2C)

RM0456

Table 663. Timing settings for fI2CCLK of 8 MHz
Standard-mode (Sm)

Fast-mode (Fm)

Fast-mode Plus
(Fm+)

Parameter
10 kHz

100 kHz

400 kHz

500 kHz

PRESC[3:0]

0x1

0x1

0x0

0x0

SCLL[7:0]

0xC7

0x13

0x9

0x6

tSCLL

200 x 250 ns = 50 µs

20 x 250 ns = 5.0 µs

10 x 125 ns = 1250 ns

7 x 125 ns = 875 ns

SCLH[7:0]

0xC3

0xF

0x3

0x3

tSCLH

196 x 250 ns = 49 µs

(1)

16 x 250 ns = 4.0 µs

~2.0 µs(4)

0x2

0x2

0x1

0x0

tSDADEL

2 x 250 ns = 500 ns

2 x 250 ns = 500 ns

1 x 125 ns = 125 ns

0 ns

SCLDEL[3:0]

0x4

0x4

0x3

0x1

tSCLDEL

5 x 250 ns = 1250 ns

5 x 250 ns = 1250 ns

4 x 125 ns = 500 ns

2 x 125 ns = 250 ns

SDADEL[3:0]

(3)

4 x 125 ns = 500 ns

~2.5 µs

~100 µs

(2)

4 x 125 ns = 500 ns

~10 µs

tSCL

(2)

1. tSCL is greater than tSCLL + tSCLH due to SCL internal detection delay. Values provided for tSCL are examples only.
2. tSYNC1 + tSYNC2 minimum value is 4 x tI2CCLK = 500 ns. Example with tSYNC1 + tSYNC2 = 1000 ns.
3.

tSYNC1 + tSYNC2 minimum value is 4 x tI2CCLK = 500 ns. Example with tSYNC1 + tSYNC2 = 750 ns.

4.

tSYNC1 + tSYNC2 minimum value is 4 x tI2CCLK = 500 ns. Example with tSYNC1 + tSYNC2 = 655 ns.

Table 664. Timing settings for fI2CCLK of 16 MHz
Standard-mode (Sm)

Fast-mode (Fm)

Fast-mode Plus (Fm+)

Parameter
10 kHz

100 kHz

400 kHz

1000 kHz

PRESC[3:0]

0x3

0x3

0x1

0x0

SCLL[7:0]

0xC7

0x13

0x9

0x4

tSCLL

200 x 250 ns = 50 µs

20 x 250 ns = 5.0 µs

10 x 125 ns = 1250 ns

5 x 62.5 ns = 312.5 ns

SCLH[7:0]

0xC3

0xF

0x3

0x2

tSCLH

196 x 250 ns = 49 µs

(1)

16 x 250 ns = 4.0 µs

0x2

0x2

0x0

2 x 250 ns = 500 ns

2 x 250 ns = 500 ns

2 x 125 ns = 250 ns

0 ns

SCLDEL[3:0]

0x4

0x4

0x3

0x2

tSCLDEL

5 x 250 ns = 1250 ns

5 x 250 ns = 1250 ns

4 x 125 ns = 500 ns

3 x 62.5 ns = 187.5 ns

SDADEL[3:0]

0x2

tSDADEL

~10 µs

(3)

3 x 62.5 ns = 187.5 ns
~1.0 µs(4)

~100 µs

(2)

4 x 125 ns = 500 ns
~2.5 µs

tSCL

(2)

1. tSCL is greater than tSCLL + tSCLH due to SCL internal detection delay. Values provided for tSCL are examples only.
2. tSYNC1 + tSYNC2 minimum value is 4 x tI2CCLK = 250 ns. Example with tSYNC1 + tSYNC2 = 1000 ns.
3.

tSYNC1 + tSYNC2 minimum value is 4 x tI2CCLK = 250 ns. Example with tSYNC1 + tSYNC2 = 750 ns.

4. tSYNC1 + tSYNC2 minimum value is 4 x tI2CCLK = 250 ns. Example with tSYNC1 + tSYNC2 = 500 ns.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

65.4.11

Inter-integrated circuit interface (I2C)

SMBus specific features
Introduction
The system management bus (SMBus) is a two-wire interface through which various
devices can communicate with each other and with the rest of the system. It is based on
operation principles of the I²C-bus. The SMBus provides a control bus for system and power
management related tasks.
The I2C peripheral is compatible with the SMBus specification (http://smbus.org).
The system management bus specification refers to three types of devices:
•

Target is a device that receives or responds to a command.

•

Controller is a device that issues commands, generates clocks, and terminates the
transfer.

•

Host is a specialized controller that provides the main interface to the system CPU. A
host must be a controller-target and must support the SMBus host notify protocol. Only
one host is allowed in a system.

The I2C peripheral can be configured as a controller or a target device, and also as a host.

Bus protocols
There are eleven possible command protocols for any given device. The device can use any
or all of them to communicate. These are: Quick Command, Send Byte, Receive Byte, Write
Byte, Write Word, Read Byte, Read Word, Process Call, Block Read, Block Write, and Block
Write-Block Read Process Call. The protocols must be implemented by the user software.
For more details on these protocols, refer to the SMBus specification (http://smbus.org).
STM32CubeMX implements an SMBus stack thanks to X-CUBE-SMBUS, a downloadable
software pack that allows basic SMBus configuration per I2C instance.

Address resolution protocol (ARP)
SMBus target address conflicts can be resolved by dynamically assigning a new unique
address to each target device. To provide a mechanism to isolate each device for the
purpose of address assignment, each device must implement a unique 128-bit device
identifier (UDID). In the I2C peripheral, it is implemented by software.
The I2C peripheral supports the Address resolution protocol (ARP). The SMBus device
default address (0b1100 001) is enabled by setting the SMBDEN bit of the I2C_CR1
register. The ARP commands must be implemented by the user software.
Arbitration is also performed in target mode for ARP support.
For more details on the SMBus address resolution protocol, refer to the SMBus specification
(http://smbus.org).

Received command and data acknowledge control
An SMBus receiver must be able to NACK each received command or data. In order to
allow the ACK control in target mode, the target byte control mode must be enabled, by
setting the SBC bit of the I2C_CR1 register. Refer to Target byte control mode for more
details.

RM0456 Rev 6

<!-- pagebreak -->

