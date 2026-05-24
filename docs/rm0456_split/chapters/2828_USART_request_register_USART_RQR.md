RM0456 Rev 6

RM0456

68.5.3

Serial peripheral interface (SPI)

CRC computation
Two separate CRC calculators are implemented to check the reliability of transmitted and
received data. The CRC polynomial configuration depends on the instance. Refer to
Section 68.3: SPI implementation for more information.
The length of the polynomial is defined by the most significant bit of the value stored in the
SPI_CRCPOLY register. It must be greater than the data frame size (in bits) defined in the
DSIZE[4:0] bitfield of the SPI_CFG1 register. To obtain a full-size polynomial, the polynomial
length must exceed the maximum data size of the peripheral instance, and the CRC33_17
bit of the SPI_CR1 register must be set to select the most significant bit of the polynomial
string.
For example, to select the standard CRC16-CCITT (XMODEM) polynomial x^16 + x^12 +
x^5 + 1:
•

For a 32-bit instance: write 0x11021 to the SPI_CRCPOLY register and clear the
CRC33_17 bit.

•

For a 16-bit instance: to obtain the full size, write 0x1021, and set the CRC33_17 bit.

The CRCSIZE field of the SPI_CFG1 register then defines how many most significant bits
from the CRC calculation registers are transferred and compared as CRC frame. It is
defined independently from the data frame length, but it must be either equal or an integer
multiple of the data frame size. Its size cannot exceed the maximum data size of the
instance.
To fully benefit from the CRC calculation capability, the polynomial length setting must
correspond to the CRC frame size, else the bits unused at the calculation are transferred
and expected all zero at the end of the CRC frame if its size is set greater than the
polynomial length.

CRC principle
The CRC calculation is enabled by setting the CRCEN bit of the SPI_CFG1 register before
the SPI is enabled (SPE = 1). The CRC value is then calculated using the CRC polynomial
defined by the CRCPOLY register and CRC33_17 bit. When SPI is enabled, the CRC
polynomial can be changed but only in the case when there is no traffic on the bus.
The CRC computation is done, bit by bit, on the sampling clock edge defined by the CPHA
and CPOL bits of the SPI_CR1 register. The calculated CRC value is checked automatically
at the end of the data block defined by the SPI_CR2 register exclusively.
When a mismatch is detected between the CRC calculated internally on the received data
and the CRC received from the transmitter, a CRCE flag is set to indicate a data corruption
error. The right procedure for handling the CRC depends on the SPI configuration and the
chosen transfer management.

CRC transfer management
Communication starts and continues normally until the last data frame has been sent or
received in the SPI_DR register.
The length of the transfer must be defined by TSIZE. When the desired number of data is
transferred, the TXCRC is transmitted and the data received on the line are compared to the
RXCRC value.
Whatever the CRCSIZE configuration, TSIZE cannot be set to 0xFFFF for a full-featured
instance, and to 0x3FF for a limited-featured instance, if CRC is enabled.

RM0456 Rev 6

<!-- pagebreak -->

2952

Serial peripheral interface (SPI)

RM0456

In transmission, the CRC computation is frozen during the CRC transfer and the TXCRC is
transmitted, in a frame of length equal to the CRCSIZE field value.
In reception, the RXCRC is also frozen when the desired number of data is transferred.
Information to be compared with the RXCRC register content is then received in a frame of
length equal to the CRCSIZE value.
Once the CRC frame is complete, an automatic check is performed comparing the received
CRC value and the value calculated in the SPI_RXCRC register. The software has to check
the CRCE flag of the SPI_SR register to determine if the data transfers were corrupted or
not. Software clears the CRCE flag by writing 1 to the CRCEC.
The user takes no care about any flushing redundant CRC information, it is done
automatically.

Resetting the SPI_TXCRC and SPI_RXCRC values
The SPI_TXCRC and SPI_RXCRC values are initialized automatically when new data is
sampled after a CRC phase. This allows the use of DMA circular mode to transfer data
without any interruption (several data blocks covered by intermediate CRC checking
phases). Initialization patterns for receiver and transmitter can be configured either to zero
or to all ones in dependency on setting bits TCRCINI and RCRCINI of the SPI_CR1 register.
The CRC values are reset when the SPI is disabled.

68.6

SPI in low-power modes
The SPI supports autonomous operation down to Stop mode, refer to Section 68.4.15:
Autonomous mode.
Table 701. Effect of low-power modes on the SPI
Mode

Description

Sleep

No effect. SPI interrupts cause the device to exit Sleep mode.

Stop(1)

The SPI registers content is kept.
If the autonomous mode is enabled at RCC configuration, and SPI is clocked by an
internal oscillator available in Stop mode, transfers are functional. The DMA
requests are functional, and the interrupts in this mode cause the device to exit
Stop mode.

Standby

The SPI instance is not functional in this mode. It is powered down, and must be
reinitialized after exiting Standby mode.

1. Refer to Section 68.3: SPI implementation for information about wake-up from Stop mode support per
instance as well as Standby mode availability. If an instance is not functional in a Stop mode, it must be
disabled before entering this Stop mode.

68.7

SPI interrupts
Table 702 gives an overview of the SPI events capable of generating interrupts if enabled.
Some of them feature wake-up from low-power mode capability, additionally. Most of them
can be enabled and disabled independently while using specific interrupt enable control bits.
The flags associated with the events are cleared by specific methods. Refer to the
description of SPI registers for more details about the event flags. When SPI is disabled, all

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Serial peripheral interface (SPI)
the pending interrupt requests are blocked to prevent their propagation into the interrupt
services, except the MODF interrupt request.
Table 702. SPI wake-up and interrupt requests
Interrupt event

Event
flag

Enable
Control
bit

Event clear method

Exit from
Stop and Standby
modes capability(1)(2)

TxFIFO ready to be loaded (space
available for one data packet FIFO threshold)

TXP

TXPIE

TXP cleared by hardware when
TxFIFO contains less than
FTHLV empty locations

Yes

Data received in RxFIFO (one data
packet available - FIFO threshold)

RXP

RXPIE

RXP cleared by hardware when
RxFIFO contains less than
FTHLV samples

Yes

Both TXP and RXP active

DXP

DXPIE

When TXP or RXP are cleared

Yes

Transmission Transfer Filled

TXTF

TXTFIE

Writing TXTFC to 1

No

Underrun

UDR

UDRIE

Writing UDRC to 1

Yes

Overrun

OVR

OVRIE

Writing OVRC to 1

Yes

CRC Error

CRCE

CRCEIE

Writing CRCEC to 1

Yes

TI Frame Format Error

TIFRE

TIFREIE

Writing TIFREC to 1

No

Mode Fault

MODF

MODFIE

Writing MODFC to 1

No

End Of Transfer (full transfer
sequence completed - based on
TSIZE value)

EOT

Writing EOTC to 1

Yes

Master mode suspended

SUSP

Writing SUSPC to 1

Yes

TxFIFO transmission complete
(TxFIFO empty)

TXC(3)

TXC cleared by hardware when
a transmission activity starts on
the bus

No

Interrupt
vector

SPI

EOTIE

1. All the interrupt events can wake up the system from Sleep mode at each instance. For detailed information about
instances capabilities to exit from concrete Stop and Standby mode refer to the Functionalities depending on the working
mode table.
2. Refer to Section 68.3: SPI implementation for information about Standby mode availability.
3. The TXC flag behavior depends on the TSIZE setting. When TSIZE>0, the flag fully follows the EOT one including its
clearing by EOTC.

RM0456 Rev 6

<!-- pagebreak -->

2952

Serial peripheral interface (SPI)

RM0456

68.8

SPI registers

68.8.1

SPI control register 1 (SPI_CR1)
Address offset: 0x000
Reset value: 0x0000 0000

31

30

29

28

27

26

25

24

23

22

21

20

19

18

17

16

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

IOLOCK

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

Res.

Res.

Res.

Res.

Res.

Res.

Res.

SPE

rs

TCRCINI RCRCINI CRC33_17
rw

rw

rw

SSI
rw

HDDIR CSUSP CSTART MASRX
rw

w

rs

rw

rw

Bits 31:17 Reserved, must be kept at reset value.
Bit 16 IOLOCK: locking the AF configuration of associated I/Os
This bit can be set by software only when SPI is disabled (SPE = 0). It is cleared by hardware
exclusively whenever the SPE bit is changed from 1 to 0, either by hardware or software.
When this bit is set, the SPI_CFG2 register content cannot be modified. This bit is writeprotected when SPI is enabled (SPE = 1).
0: AF configuration is not locked
1: AF configuration is locked
Bit 15 TCRCINI: CRC calculation initialization pattern control for transmitter
0: all zero pattern is applied
1: all ones pattern is applied
Bit 14 RCRCINI: CRC calculation initialization pattern control for receiver
0: All zero pattern is applied
1: All ones pattern is applied
Bit 13 CRC33_17: Full size (33-bit or 17-bit) CRC polynomial configuration
0: Full size (33-bit or 17-bit) CRC polynomial is not used
1: Full size (33-bit or 17-bit) CRC polynomial is used
Bit 12 SSI: internal SS signal input level
This bit has an effect only when the SSM bit is set. The value of this bit is forced onto the
peripheral SS input internally and the I/O value of the SS pin is ignored.
Bit 11 HDDIR: Rx/Tx direction at half-duplex mode
In half-duplex configuration the HDDIR bit establishes the Rx/Tx direction of the data transfer.
This bit is ignored in Full-Duplex or any Simplex configuration.
0: SPI is receiver
1: SPI is transmitter

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Serial peripheral interface (SPI)

Bit 10 CSUSP: master suspend request
This bit reads as zero.
In master mode, when this bit is set by software, the CSTART bit is reset at the end of the
current frame and communication is suspended. The user has to check SUSP flag to check
end of the frame transfer.
The master mode communication must be suspended (using this bit or keeping TXDR
empty) before going to Low-power mode.
After software suspension, SUSP flag must be cleared and SPI disabled and re-enabled
before the next transfer starts.
Bit 9 CSTART: master transfer start
This bit can be set by software if SPI is enabled only to start an SPI communication. it is
cleared by hardware when end of transfer (EOT) flag is set or when a transfer suspend
request is accepted.
In SPI mode, the bit is taken into account at master mode only. If transmission is enabled,
communication starts or continues only if any data is available in the transmission FIFO.
0: master transfer is at idle
1: master transfer is ongoing or temporary suspended by automatic suspend
Bit 8 MASRX: master automatic suspension in Receive mode
This bit is set and cleared by software to control continuous SPI transfer in master receiver
mode and automatic management in order to avoid overrun condition.
When SPI communication is suspended by hardware automatically, it may happen that few
bits of next frame are already clocked out due to internal synchronization delay.
This is why, the automatic suspension is not quite reliable when size of data drops below 8
bits. In this case, a safe suspension can be achieved by combination with delay inserted
between data frames applied when MIDI parameter keeps a non zero value; sum of data size
and the interleaved SPI cycles must always produce interval at length of 8 SPI clock periods
at minimum. After software clearing of the SUSP bit, the communication resumes and
continues by subsequent bits transfer without any next constraint. Before the SUSP bit is
cleared, the user must release the RxFIFO space as much as possible by reading out all the
data packets available at RxFIFO based on the RXP flag indication to prevent any
subsequent suspension.
0: SPI flow/clock generation is continuous, regardless of overrun condition. (data are lost)
1: SPI flow is suspended temporary on RxFIFO full condition, before reaching overrun
condition. The SUSP flag is set when the SPI communication is suspended.
Bits 7:1 Reserved, must be kept at reset value.
Bit 0 SPE: serial peripheral enable
This bit is set by and cleared by software.
When SPE = 1, SPI data transfer is enabled, the SPI_CFG1 and SPI_CFG2 configuration
registers, CRCPOLY, UDRDR, part of the SPI_AUTOCR register and IOLOCK bit in the
SPI_CR1 register are write protected. They can be changed only when SPE = 0.
When SPE = 0 any SPI operation is stopped and disabled, all the pending requests of the
events with enabled interrupt are blocked except the MODF interrupt request (but their
pending still propagates the request of the spi_plck clock), the SS output is deactivated at
master, the RDY signal keeps not ready status at slave, the internal state machine is reseted,
all the FIFOs content is flushed, CRC calculation initialized, receive data register is read zero.
SPE is cleared and cannot be set when MODF error flag is active.
0: Serial peripheral disabled.
1: Serial peripheral enabled

RM0456 Rev 6

<!-- pagebreak -->

2952

Serial peripheral interface (SPI)

68.8.2

RM0456

SPI control register 2 (SPI_CR2)
Address offset: 0x004
Reset value: 0x0000 0000

31

30

29

28

27

26

25

24

23

22

21

20

19

18

17

16

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

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

rw

rw

rw

TSIZE[15:0]
rw

rw

rw

rw

rw

rw

rw

rw

rw

Bits 31:16 Reserved, must be kept at reset value.
Bits 15:0 TSIZE[15:0]: number of data at current transfer
When these bits are changed by software, the SPI must be disabled.
Endless transfer is initialized when CSTART is set while zero value is stored in TSIZE. TSIZE
cannot be set to 0xFFFF for a full-featured instance and 0x3FF for a limited-featured
instance, when CRC is enabled.
Note: TSIZE[15:10] bits are reserved for limited-featured instances, and must be kept at reset
value.

68.8.3

SPI configuration register 1 (SPI_CFG1)
Address offset: 0x008
Reset value: 0x0007 0007
The content of this register is write protected when SPI is enabled, except TXDMAEN and
RXDMAEN bits.

31

30

29

BPASS

28

MBR[2:0]

rw

rw

rw

rw

15

14

13

12

TXDMA RXDMA
EN
EN
rw

rw

Res.

Res.

27

26

25

24

23

22

21

Res.

Res.

Res.

Res.

Res.

CRCEN

Res.

11

10

9

8

7

Res.

UDRCF
G

rw

Res.

rw

6

5

20

rw

rw

RM0456 Rev 6

17

16

rw

rw

rw

rw

rw

4

3

2

1

0

rw

rw

FTHLV[3:0]
rw

18
CRCSIZE[4:0]

DSIZE[4:0]
rw

rw

Bit 31 BPASS: bypass of the prescaler at master baud rate clock generator
0: bypass is disabled
1: bypass is enabled

<!-- pagebreak -->

19

rw

rw

RM0456

Serial peripheral interface (SPI)

Bits 30:28 MBR[2:0]: master baud rate prescaler setting
000: SPI master clock/2
001: SPI master clock/4
010: SPI master clock/8
011: SPI master clock/16
100: SPI master clock/32
101: SPI master clock/64
110: SPI master clock/128
111: SPI master clock/256
Note: MBR setting is considered at slave working at TI mode, too (see Section 68.5.1: TI
mode).
Bits 27:23 Reserved, must be kept at reset value.
Bit 22 CRCEN: hardware CRC computation enable
0: CRC calculation disabled
1: CRC calculation enabled
Bit 21 Reserved, must be kept at reset value.
Bits 20:16 CRCSIZE[4:0]: length of CRC frame to be transferred and compared
Most significant bits are taken into account from polynomial calculation when CRC result is
transferred or compared. The length of the polynomial is not affected by this setting.
The value must be equal or a multiple of the data size configured through the DSIZE bitfield.
Its maximum size corresponds to the instance maximum data size.
00000: not used
00001: not used
00010: not used
00011: 4 bits
00100: 5 bits
00101: 6 bits
00110: 7 bits
00111: 8 bits
.....
11101: 30 bits
11110: 31 bits
11111: 32 bits
Note: The most significant bit at CRCSIZE bit field is reserved for the peripheral instances
where data size is limited to 16 bits.
The CRCSIZE[2:0] bits are fixed to 1 for the peripheral instances with a limited set of
features.
Bit 15 TXDMAEN: Tx DMA stream enable
0: Tx DMA disabled
1: Tx DMA enabled
Bit 14 RXDMAEN: Rx DMA stream enable
0: Rx-DMA disabled
1: Rx-DMA enabled
Bits 13:10 Reserved, must be kept at reset value.

RM0456 Rev 6

<!-- pagebreak -->

2952

Serial peripheral interface (SPI)

RM0456

Bit 9 UDRCFG: behavior of slave transmitter at underrun condition
For more details see Figure 858: Optional configurations of the slave behavior when an
underrun condition is detected.
0: slave sends a constant pattern defined by the user in the SPI_UDRDR register
1: Slave repeats lastly received data from master. When slave is configured at transmit only
mode (COMM[1:0] = 01), all zeros pattern is repeated.
Bits 8:5 FTHLV[3:0]: FIFO threshold level
Defines number of data frames in a single data packet. It is recommended that the size of the
packet does not exceed 1/2 of FIFO space.
SPI interface is more efficient if configured packet sizes are aligned with data register access
parallelism:
–If SPI data register is accessed as a 16-bit register and DSIZE ≤ 8 bits, better to select
FTHLV = 2, 4, 6.
–If SPI data register is accessed as a 32-bit register and DSIZE > 8 bits, better to select
FTHLV = 2, 4, 6, while if DSIZE ≤ 8bit, better to select FTHLV = 4, 8, 12.
0000: 1-data
0001: 2-data
0010: 3-data
0011: 4-data
0100: 5-data
0101: 6-data
0110: 7-data
0111: 8-data
1000: 9-data
1001: 10-data
1010: 11-data
1011: 12-data
1100: 13-data
1101: 14-data
1110: 15-data
1111: 16-data
Note: FTHLV[3:2] bits are reserved for instances with a limited set of features.
Bits 4:0 DSIZE[4:0]: number of bits in a single SPI data frame
00000: not used
00001: not used
00010: not used
00011: 4 bits
00100: 5 bits
00101: 6 bits
00110: 7 bits
00111: 8 bits
.....
11101: 30 bits
11110: 31 bits
11111: 32 bits
Note: The most significant bit DSIZE[4] is reserved for instances which data size is limited to
16 bits.
DSIZE[2:0] bits are fixed to 1 for instances with a limited set of features.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Serial peripheral interface (SPI)

68.8.4

SPI configuration register 2 (SPI_CFG2)
Address offset: 0x00C
Reset value: 0x0000 0000
The content of this register is write-protected when SPI is enabled or the IOLOCK bit is set
in the SPI_CR1 register.

31

30

AFCNTR SSOM

29

28

27

26

25

24

SSOE

SSIOP

Res.

SSM

CPOL

CPHA

23

22

21

LSBFRST MASTER

20

19

SP[2:0]

18

17

16

COMM[1:0]

Res.

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

rw

rw

rw

rw

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

Res.

Res.

Res.

Res.

Res.

IOSWP RDIOP RDIOM
rw

rw

rw

MIDI[3:0]
rw

rw

0

MSSI[3:0]
rw

rw

rw

rw

rw

rw

Bit 31 AFCNTR: alternate function GPIOs control
This bit is taken into account when SPE = 0 only.
When SPI must be disabled temporary for a specific configuration reason (for example CRC
reset, CPHA or HDDIR change) setting this bit prevents any glitches on the associated
outputs configured at alternate function mode by keeping them forced at state corresponding
the current SPI configuration.
0: The peripheral takes no control of GPIOs while it is disabled
1: The peripheral keeps always control of all associated GPIOs
Bit 30 SSOM: SS output management in master mode
This bit is taken into account in master mode when SSOE is enabled. It allows the SS output
to be configured between two consecutive data transfers.
0: SS is kept at active level until data transfer is complete, it becomes inactive with EOT flag
1: SPI data frames are interleaved with SS nonactive pulses when MIDI[3:0] > 1
Bit 29 SSOE: SS output enable
This bit is taken into account in master mode only
0: SS output is disabled and the SPI can work in multimaster configuration
1: SS output is enabled. The SPI cannot work in a multimaster environment. It forces the SS
pin at inactive level after the transfer is complete or SPI is disabled with respect to SSOM,
MIDI, MSSI, SSIOP bits setting
Bit 28 SSIOP: SS input/output polarity
0: low level is active for SS signal
1: high level is active for SS signal
Bit 27 Reserved, must be kept at reset value.
Bit 26 SSM: software management of SS signal input
0: SS input value is determined by the SS PAD
1: SS input value is determined by the SSI bit
Note: When master uses hardware SS output (SSM = 0 and SSOE = 1) the SS signal input is
forced to not active state internally to prevent master mode fault error.
Bit 25 CPOL: clock polarity
0: SCK signal is at 0 when idle
1: SCK signal is at 1 when idle

RM0456 Rev 6

<!-- pagebreak -->

2952

Serial peripheral interface (SPI)

RM0456

Bit 24 CPHA: clock phase
0: the first clock transition is the first data capture edge
1: the second clock transition is the first data capture edge
Bit 23 LSBFRST: data frame format
0: MSB transmitted first
1: LSB transmitted first
Bit 22 MASTER: SPI master
0: SPI slave
1: SPI master
Bits 21:19 SP[2:0]: serial protocol
000: SPI Motorola
001: SPI TI
others: reserved, must not be used
Bits 18:17 COMM[1:0]: SPI Communication Mode
00: full-duplex
01: simplex transmitter
10: simplex receiver
11: half-duplex
Bit 16 Reserved, must be kept at reset value.
Bit 15 IOSWP: swap functionality of MISO and MOSI pins
When this bit is set, the function of MISO and MOSI pins alternate functions are inverted.
Original MISO pin becomes MOSI and original MOSI pin becomes MISO.
0: no swap
1: MOSI and MISO are swapped
Bit 14 RDIOP: RDY signal input/output polarity
0: high level of the signal means the slave is ready for communication
1: low level of the signal means the slave is ready for communication
Bit 13 RDIOM: RDY signal input/output management
0: RDY signal is defined internally fixed as permanently active (RDIOP setting has no effect)
1: RDY signal is overtaken from alternate function input (at master case) or output (at slave
case) of the dedicated pin (RDIOP setting takes effect)
Note: When DSIZE in the SPI_CFG1 register is configured shorter than 8 bits, the RDIOM bit
must be kept at zero.
Bits 12:8 Reserved, must be kept at reset value.
Bits 7:4 MIDI[3:0]: Master Inter-Data Idleness
Specifies minimum time delay (expressed in SPI clock cycles periods) inserted between two
consecutive data frames in master mode.
0000: no delay
0001: 1 clock cycle period delay
...
1111: 15 clock cycle periods delay
Note: This feature is not supported in TI mode.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Serial peripheral interface (SPI)

Bits 3:0 MSSI[3:0]: Master SS Idleness
Specifies an extra delay, expressed in number of SPI clock cycle periods, inserted
additionally between active edge of SS opening a session and the beginning of the first data
frame of the session in master mode when SSOE is enabled.
0000: no extra delay
0001: 1 clock cycle period delay added
...
1111: 15 clock cycle periods delay added
Note: This feature is not supported in TI mode.
To include the delay, the SPI must be disabled and re-enabled between sessions.

68.8.5

SPI interrupt enable register (SPI_IER)
Address offset: 0x010
Reset value: 0x0000 0000

31

30

29

28

27

26

25

24

23

22

21

20

19

18

17

16

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

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

Res.

Res.

Res.

Res.

Res.

Res.

MODFIE TIFREIE CRCEIE OVRIE
rw

rw

rw

rw

UDRIE TXTFIE EOTIE
rw

rw

rw

DXPIE
rw

TXPIE RXPIE
rw

rw

Bits 31:10 Reserved, must be kept at reset value.
Bit 9 MODFIE: mode Fault interrupt enable
0: MODF interrupt disabled
1: MODF interrupt enabled
Bit 8 TIFREIE: TIFRE interrupt enable
0: TIFRE interrupt disabled
1: TIFRE interrupt enabled
Bit 7 CRCEIE: CRC error interrupt enable
0: CRC interrupt disabled
1: CRC interrupt enabled
Bit 6 OVRIE: OVR interrupt enable
0: OVR interrupt disabled
1: OVR interrupt enabled
Bit 5 UDRIE: UDR interrupt enable
0: UDR interrupt disabled
1: UDR interrupt enabled
Bit 4 TXTFIE: TXTF interrupt enable
0: TXTF interrupt disabled
1: TXTF interrupt enabled
Bit 3 EOTIE: EOT, SUSP and TXC interrupt enable
0: EOT/SUSP/TXC interrupt disabled
1: EOT/SUSP/TXC interrupt enabled

RM0456 Rev 6

<!-- pagebreak -->

2952

Serial peripheral interface (SPI)

RM0456

Bit 2 DXPIE: DXP interrupt enabled
DXPIE is set by software and cleared by TXTF flag set event.
0: DXP interrupt disabled
1: DXP interrupt enabled
Bit 1 TXPIE: TXP interrupt enable
TXPIE is set by software and cleared by TXTF flag set event.
0: TXP interrupt disabled
1: TXP interrupt enabled
Bit 0 RXPIE: RXP interrupt enable
0: RXP interrupt disabled
1: RXP interrupt enabled

68.8.6

SPI status register (SPI_SR)
Address offset: 0x014
Reset value: 0x0000 1002
All the flags of this register are not cleared automatically when the SPI is reenabled. They
require specific clearing access exclusively via the flag clearing register as noted in the bits
descriptions below.

31

30

29

28

27

26

25

24

23

22

21

20

19

18

17

16

CTSIZE[15:0]
r

r

r

r

r

r

r

r

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

RXPLVL[1:0]

TXC

SUSP

Res.

MODF

TIFRE

CRCE

OVR

UDR

TXTF

EOT

DXP

TXP

RXP

r

r

r

r

r

r

r

r

r

r

r

r

RXWNE
r

r

r

Bits 31:16 CTSIZE[15:0]: number of data frames remaining in current TSIZE session
The value is not quite reliable when traffic is ongoing on bus or during autonomous operation
in low-power mode.
Note: CTSIZE[15:0] bits are not available in instances with limited set of features.
Bit 15 RXWNE: RxFIFO word not empty
0: less than four bytes of RxFIFO space is occupied by data
1: at least four bytes of RxFIFO space is occupied by data
Note: This bit value does not depend on DSIZE setting and keeps together with RXPLVL[1:0]
information about RxFIFO occupancy by residual data.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Serial peripheral interface (SPI)

Bits 14:13 RXPLVL[1:0]: RxFIFO packing level
When RXWNE = 0 and data size is set up to 16 bits, the value gives number of remaining
data frames persisting at RxFIFO.
When data size is greater than 16 bits, these bits are always read as 00. As a result, the
single data frame received in the FIFO cannot be detected neither by RWNE nor by RXPLVL
bits if the data size is set from 17 to 24 bits. The user must then apply other methods to
detect the number of data received, such as monitor the EOT event when TSIZE > 0 or RXP
events when FTHLV = 0.
00: no next frame is available at RxFIFO
01: 1 frame is available
10: 2 frames are available*
11: 3 frames are available*
Note: (*): Possible value when data size is set up to 8 bits only.
Bit 12 TXC: TxFIFO transmission complete
The flag behavior depends on TSIZE setting.
When TSIZE = 0, the TXC is changed by hardware exclusively and it raises each time the
TxFIFO becomes empty and there is no activity on the bus.
If TSIZE ≠ 0 there is no specific reason to monitor TXC as it just copies the EOT flag value
including its software clearing. The TXC generates an interrupt when EOTIE is set.
This flag is set when SPI is reset or disabled.
0: current data transfer is still ongoing, data is available in TxFIFO or last frame transmission
is on going.
1: last TxFIFO frame transmission complete
Bit 11 SUSP: suspension status
In master mode, SUSP is set by hardware either as soon as the current frame is complete
after CSUSP request is done or at master automatic suspend receive mode (the MASRX bit
is set in the SPI_CR1 register) on RxFIFO full condition.
SUSP generates an interrupt when EOTIE is set.
This bit must be cleared prior to disabling the SPI. This is done by setting the SUSPC bit of
SPI_IFCR exclusively.
0: SPI not suspended (master mode active or other mode).
1: master mode is suspended (current frame completed).
Bit 10 Reserved, must be kept at reset value.
Bit 9 MODF: mode fault
When MODF is set, SPE and IOLOCK bits of the SPI_CR1 register are reset and setting
SPE again is blocked until MODF is cleared.
This bit is cleared by writing 1 to the MODFC bit of the SPI_IFCR exclusively.
0: no mode fault
1: mode fault detected.
Bit 8 TIFRE: TI frame format error
This bit is cleared by writing 1 to the TIFREC bit of the SPI_IFCR exclusively.
0: no TI Frame Error
1: TI frame error detected
Bit 7 CRCE: CRC error
This bit is cleared when SPI is re-enabled or by writing 1 to the CRCEC bit of the SPI_IFCR
optionally.
0: no CRC error
1: CRC error detected

RM0456 Rev 6

<!-- pagebreak -->

2952

Serial peripheral interface (SPI)

RM0456

Bit 6 OVR: overrun
This bit is cleared when SPI is re-enabled or by writing 1 to the OVRC bit of the SPI_IFCR
optionally.
0: no overrun
1: overrun detected
Bit 5 UDR: underrun
This bit is cleared when SPI is re-enabled or by writing 1 to the UDRC bit of the SPI_IFCR
optionally.
0: no underrun
1: underrun detected
Note: In SPI mode, the UDR flag applies to slave mode only.
Bit 4 TXTF: transmission transfer filled
TXTF is set by hardware as soon as all of the data packets in a transfer have been submitted
for transmission by application software or DMA, that is when TSIZE number of data have
been pushed into the TxFIFO.
This bit is cleared by software write 1 to the TXTFC bit of the SPI_IFCR exclusively.
The TXTF flag triggers an interrupt if the TXTFIE bit is set.
TXTF setting clears the TXPIE and DXPIE masks so to off-load application software from
calculating when to disable TXP and DXP interrupts.
0: upload of TxFIFO is ongoing or not started
1: TxFIFO upload is finished
Bit 3 EOT: end of transfer
EOT is set by hardware as soon as a full transfer is complete, that is when SPI is re-enabled
or when TSIZE number of data have been transmitted and/or received on the SPI. EOT is
cleared when SPI is re-enabled or by writing 1 to the EOTC bit of the SPI_IFCR register
optionally.
EOT flag triggers an interrupt if the EOTIE bit is set.
If DXP flag is used until TXTF flag is set and DXPIE is cleared, EOT can be used to
download the last packets contained into RxFIFO in one-shot.
In master, EOT event terminates the data transfer and handles SS output optionally. When
CRC is applied, the EOT event is extended over the CRC frame transfer.
To restart the internal state machine properly, SPI is strongly suggested to be disabled and
re-enabled before next transfer starts despite its setting is not changed.
0: transfer is ongoing or not started
1: transfer complete
Bit 2 DXP: duplex packet
DXP flag is set whenever both TXP and RXP flags are set regardless SPI mode.
0: TxFIFO is Full and/or RxFIFO is Empty
1: both TxFIFO has space for write and RxFIFO contains for read a single packet at least
Bit 1 TXP: Tx-packet space available
TXP flag can be changed only by hardware. Its value depends on the physical size of the
FIFO and its threshold (FTHLV[3:0]), data frame size (DSIZE[4:0] in SPI mode), and actual
communication flow. If the data packet is stored by performing consecutive write operations
to SPI_TXDR, TXP flag must be checked again once a complete data packet is stored at
TxFIFO. TXP is set despite SPI TxFIFO becomes inaccessible when SPI is reset or
disabled.
0: not enough free space at TxFIFO to host next data packet
1: enough free space at TxFIFO to host at least one data packet

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Serial peripheral interface (SPI)

Bit 0 RXP: Rx-packet available
The flag is changed by hardware. It monitors the total number of data currently available at
RxFIFO if SPI is enabled. RXP value depends on the FIFO threshold (FTHLV[3:0]), data
frame size (DSIZE[4:0] in SPI mode), and actual communication flow. If the data packet is
read by performing consecutive read operations from SPI_RXDR, RXP flag must be
checked again once a complete data packet is read out from RxFIFO.
0: RxFIFO is empty or an incomplete data packet is received
1: RxFIFO contains at least one data packet

68.8.7

SPI interrupt/status flags clear register (SPI_IFCR)
Address offset: 0x018
Reset value: 0x0000 0000

31

30

29

28

27

26

25

24

23

22

21

20

19

18

17

16

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

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

Res.

Res.

Res.

Res.

SUSPC

Res.

UDRC

TXTFC

EOTC

Res.

Res.

Res.

w

w

w

w

MODFC TIFREC CRCEC OVRC
w

w

w

w

Bits 31:12 Reserved, must be kept at reset value.
Bit 11 SUSPC: Suspend flag clear
Writing a 1 into this bit clears SUSP flag of the SPI_SR register
Bit 10 Reserved, must be kept at reset value.
Bit 9 MODFC: mode fault flag clear
Writing a 1 into this bit clears MODF flag of the SPI_SR register
Bit 8 TIFREC: TI frame format error flag clear
Writing a 1 into this bit clears TIFRE flag of the SPI_SR register
Bit 7 CRCEC: CRC error flag clear
Writing a 1 into this bit clears CRCE flag of the SPI_SR register
Bit 6 OVRC: overrun flag clear
Writing a 1 into this bit clears OVR flag of the SPI_SR register
Bit 5 UDRC: underrun flag clear
Writing a 1 into this bit clears UDR flag of the SPI_SR register
Bit 4 TXTFC: transmission transfer filled flag clear
Writing a 1 into this bit clears TXTF flag of the SPI_SR register
Bit 3 EOTC: end of transfer flag clear
Writing a 1 into this bit clears EOT flag of the SPI_SR register
Bits 2:0 Reserved, must be kept at reset value.

RM0456 Rev 6

<!-- pagebreak -->

2952

Serial peripheral interface (SPI)

68.8.8

RM0456

SPI autonomous mode control register (SPI_AUTOCR)
Address offset: 0x01C
Reset value: 0x0000 0000

31

30

29

28

27

26

25

24

23

22

21

20

19

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

rw

rw

rw

rw

rw

rw

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

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

TRIGEN TRIGPOL

18

17

16

TRIGSEL[3:0]

Bits 31:22 Reserved, must be kept at reset value.
Bit 21 TRIGEN: Hardware control of CSTART triggering enable
0: Hardware control disabled
1: Hardware control enabled
Note: if user cannot prevent trigger event during write, the TRIGEN must be changed when
SPI is disabled
Bit 20 TRIGPOL: Trigger polarity
0: trigger is active on raising edge
1: trigger is active on falling edge
Note: This bit can be written only when SPE = 0.
Bits 19:16 TRIGSEL[3:0]: Trigger selection (refer Section : Description of SPI interconnections).
0000: spi_trg0 is selected
0001: spi_trg1 is selected
...
1111: spi_trg15 is selected
Note: these bits can be written only when SPE = 0.
Bits 15:0 Reserved, must be kept at reset value.

68.8.9

SPI transmit data register (SPI_TXDR)
Address offset: 0x020
Reset value: 0x0000 0000

31

30

29

28

27

26

25

24

23

22

21

20

19

18

17

16

TXDR[31:16]
w

w

w

w

w

w

w

w

w

w

w

w

w

w

w

w

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

w

w

w

w

w

w

w

w

w

w

w

w

w

w

TXDR[15:0]

<!-- pagebreak -->

w

w

RM0456 Rev 6

RM0456

Serial peripheral interface (SPI)

Bits 31:0 TXDR[31:0]: transmit data register
The register serves as an interface with TxFIFO. A write to it accesses TxFIFO.
Note: data is always right-aligned. Unused bits are ignored when writing to the register, and
read as zero when the register is read.
Note: DR can be accessed byte-wise (8-bit access): in this case only one data-byte is written
by single access.
half-word-wise (16 bit access) in this case 2 data-bytes or 1 half-word-data can be
written by single access.
word-wise (32 bit access). In this case 4 data-bytes or 2 half-word-data or word-data
can be written by single access.
Write access of this register less than the configured data size is forbidden.

68.8.10

SPI receive data register (SPI_RXDR)
Address offset: 0x030
Reset value: 0x0000 0000

31

30

29

28

27

26

25

24

23

22

21

20

19

18

17

16

r

r

r

r

r

r

r

r

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

r

r

r

r

r

r

r

RXDR[31:16]

RXDR[15:0]
r

r

r

r

r

r

r

r

r

Bits 31:0 RXDR[31:0]: receive data register
The register serves as an interface with RxFIFO. When it is read, RxFIFO is accessed.
Note: data is always right-aligned. Unused bits are read as zero when the register is read.
Writing to the register is ignored.
Note: DR can be accessed byte-wise (8-bit access): in this case only one data-byte is read by
single access
half-word-wise (16 bit access) in this case 2 data-bytes or 1 half-word data can be read
by single access
word-wise (32 bit access). In this case 4 data-bytes or 2 half-word data or word-data
can be read by single access.
Read access of this register less than the configured data size is forbidden.

68.8.11

SPI polynomial register (SPI_CRCPOLY)
Address offset: 0x040
Reset value: 0x0000 0107
The content of this register is write protected when SPI is enabled.

31

30

29

28

27

26

25

24

rw

rw

rw

rw

rw

rw

rw

rw

15

14

13

12

11

10

9

8

23

22

21

20

19

18

17

16

rw

rw

rw

rw

rw

rw

rw

rw

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

rw

rw

rw

CRCPOLY[31:16]

CRCPOLY[15:0]
rw

rw

rw

rw

rw

rw

rw

rw

rw

RM0456 Rev 6

<!-- pagebreak -->

2952

Serial peripheral interface (SPI)

RM0456

Bits 31:0 CRCPOLY[31:0]: CRC polynomial register
This register contains the polynomial for the CRC calculation.
The default 9-bit polynomial setting 0x107 corresponds to default 8-bit setting of DSIZE. It is
compatible with setting 0x07 used in other ST products with fixed length of the polynomial
string, where the most significant bit of the string is always kept hidden.
Length of the polynomial is given by the most significant bit of the value stored in this
register. It must be set greater than DSIZE. CRC33_17 bit must be set additionally with
CRCPOLY register when DSIZE is configured to maximum data size and CRC is enabled (to
keep polynomial length grater than data size).
Note: CRCPOLY[31:16] bits are reserved for instances with data size limited to 16 bits. There
is no constrain when 32-bit access is applied at these addresses. Reserved bits 31-16
are always read zero while any write to them is ignored.

68.8.12

SPI transmitter CRC register (SPI_TXCRC)
Address offset: 0x044
Reset value: 0x0000 0000

31

30

29

28

27

26

25

24

23

22

21

20

19

18

17

16

TXCRC[31:16]
r

r

r

r

r

r

r

r

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

r

r

r

r

r

r

r

r

r

r

r

r

r

r

TXCRC[15:0]
r

r

Bits 31:0 TXCRC[31:0]: CRC register for transmitter
When CRC calculation is enabled, the TXCRC[31:0] bits contain the computed CRC value
of the subsequently transmitted bytes. CRC calculation is initialized when the CRCEN bit of
the SPI_CR1 register is set or when a data block is transferred completely. The CRC is
calculated serially using the polynomial programmed in the SPI_CRCPOLY register.
The number of bits considered at calculation depends on the SPI_CRCPOLY register and
CRCSIZE bits settings in the SPI_CFG1 register.
Note: A read to this register when the communication is ongoing may return an incorrect
value.
Note: TXCRC[31-16] bits are reserved for instances with data size limited to 16 bits. There is
no constrain when 32-bit access is applied at these addresses. Reserved bits 31-16
are always read zero while any write to them is ignored.
Note: The configuration of the CRCSIZE bitfield is not taken into account when the content of
this register is read by software. No masking is applied for unused bits in this case.

<!-- pagebreak -->

