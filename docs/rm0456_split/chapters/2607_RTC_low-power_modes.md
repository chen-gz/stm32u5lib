2756

Inter-integrated circuit interface (I2C)
Caution:

RM0456

For compliance with the I²C-bus or SMBus specification, the controller clock must respect
the timings in the following table.
Table 662. I²C-bus and SMBus specification clock timings

Symbol

Parameter

Standardmode (Sm)

Fast-mode Fast-mode
(Fm)
Plus (Fm+)

Min

Max

Min

Max

Min

Max

Min

Max

SMBus
Unit

fSCL

SCL clock frequency

-

100

-

400

-

1000

-

100

tHD:STA

Hold time (repeated) START condition

4.0

-

0.6

-

0.26

-

4.0

-

tSU:STA

Set-up time for a repeated START
condition

4.7

-

0.6

-

0.26

-

4.7

-

tSU:STO

Set-up time for STOP condition

4.0

-

0.6

-

0.26

-

4.0

-

tBUF

Bus free time between a STOP and
START condition

4.7

-

1.3

-

0.5

-

4.7

-

tLOW

Low period of the SCL clock

4.7

-

1.3

-

0.5

-

4.7

-

tHIGH

High period of the SCL clock

4.0

-

0.6

-

0.26

-

4.0

50

tr

Rise time of both SDA and SCL signals

-

1000

-

300

-

120

-

1000

tf

Fall time of both SDA and SCL signals

-

300

-

300

-

120

-

300

Note:

kHz

µs

ns

The SCLL[7:0] bitfield also determines the tBUF and tSU:STA timings and SCLH[7:0] the
tHD:STA and tSU:STO timings.
Refer to Section 65.4.10 for examples of I2C_TIMINGR settings versus the i2c_ker_ck
frequency.

Controller communication initialization (address phase)
To initiate the communication with a target to address, set the following bitfields of the
I2C_CR2 register:

Note:

•

ADD10: addressing mode (7-bit or 10-bit)

•

SADD[9:0]: target address to send

•

RD_WRN: transfer direction

•

HEAD10R: in case of 10-bit address read, this bit determines whether the header only
(for direction change) or the complete address sequence is sent.

•

NBYTES[7:0]: the number of bytes to transfer; if equal to or greater than 255 bytes, the
bitfield must initially be set to 0xFF.

Changing these bitfields is not allowed as long as the START bit is set.
Before launching the communication, make sure that the I²C-bus is idle. This can be
checked using the bus idle detection function or by verifying that the IDR bits of the GPIOs
selected as SDA and SCL are set. Any low-level incident on the I²C-bus lines that coincides
with the START condition asserted by the I2C peripheral may cause its deadlock if not
filtered out by the input filters. If such incidents cannot be prevented, design the software so
that it restores the normal operation of the I2C peripheral in case of a deadlock, by toggling
the PE bit of the I2C_CR1 register.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Inter-integrated circuit interface (I2C)
To launch the communication, set the START bit of the I2C_CR2 register. The controller
then automatically sends a START condition followed by the target address, either
immediately if the BUSY flag is low, or tBUF time after the BUSY flag transits from high to low
state. The BUSY flag is set upon sending the START condition.
In case of an arbitration loss, the controller automatically switches back to target mode and
can acknowledge its own address if it is addressed as a target.

Note:

The START bit is reset by hardware when the target address is sent on the bus, whatever
the received acknowledge value. The START bit is also reset by hardware upon arbitration
loss.
In 10-bit addressing mode, the controller automatically keeps resending the target address
in a loop until the first address byte (first seven address bits) is acknowledged by the target.
Setting the ADDRCF bit makes I2C quit that loop.
If the I2C peripheral is addressed as a target (ADDR = 1) while the START bit is set, the I2C
peripheral switches to target mode and the START bit is cleared.

Note:

The same procedure is applied for a repeated START condition. In this case, BUSY = 1.
Figure 793. Controller initialization flow
Controller
initialization

Initial settings

Enable interrupts and/or DMA in I2C_CR1

End

MSv19859V3

Initialization of a controller receiver addressing a 10-bit address target
If the target address is in 10-bit format, the user can choose to send the complete read
sequence, by clearing the HEAD10R bit of the I2C_CR2 register. In this case, the controller
automatically sends the following complete sequence after the START bit is set:
(RE)START + Target address 10-bit header Write + Target address second byte +
(RE)START + Target address 10-bit header Read.
Figure 794. 10-bit address read access with HEAD10R = 0
11110XX
S

Target address
1st 7 bits

0
R/W

A1

Target address
2nd byte

A2

Sr

11110XX

1

Target address
1st 7 bits

R/W

A3

DATA

A

DATA

A

P

Read

Write

MSv41066V2

RM0456 Rev 6

<!-- pagebreak -->

