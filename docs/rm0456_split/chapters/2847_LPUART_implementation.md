2952

Serial peripheral interface (SPI)

68.8.15

RM0456

SPI register map

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

CSUSP

CSTART

MASRX

Res.

Res.

Res.

Res.

Res.

Res.

Res.

SPE

0

0

0

0

0

0

0

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

SUSPC

0x0240x02C

0

0

0

0

0

0

0

0

0

0

0

0

0

0x034 0x03C

0

0

0

0

0

0

0

0

0

0x048
0x04C

0

0

0

0

0

UDRCFG

Res.

MODFIE

TIFREIE

CRCEIE

OVRIE

UDRIE

TXTFIE

EOTIE

DXPIE

TXPIE

RXPIE
0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

1

1

1

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

CRCPOLY[15:0]
0

0

0

0

0

0

TXCRC[31:16](2)
0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

1

0

0

0
0

0

0

0

0

RXCRC[15:0]
0

0

0

0

0

0

0

0

0

0

0

UDRDR[31:16](2)
0

0

TXCRC[15:0]

RXCRC[31:16](2)

SPI_UDRDR
Reset value

1

Res.

SPI_RXCRC
Reset value

0

0

CRCPOLY[31:16](2)

SPI_TXCRC
Reset value

0

RXDR[15:0]
0

Reserved

0x044

0

0

RXDR[31:16]
0

SPI_CRCPOLY
Reset value

0

Res.

SPI_RXDR
Reset value

0

TXDR[15:0]

Reserved

0x030

0x040

0

0

Res.

0

TXDR[31:16]
0

0

Res.

Reset value

0

Res.

0x020

0

Res.

0

0

Res.

0

0

Res.

TRIGPOL

0

0

Res.

TRIGEN

Res.

Res.

Res.

0

0

RXP

Res.
Res.

0

0

Res.

Res.
Res.

SPI_TXDR

0

Res.

Res.
Res.

0

Reset value

0

TXP

Res.
Res.

TRIGSEL[3:0]

0

Res.

Res.
Res.

SPI_AUTOCR

Res.

0x01C

0
Res.

Reset value

0

Res.

SPI_IFCR

Res.

0x018

0

DPXP

0

0

Res.

1

0

Res.

0

0

EOT

0

0

EOTC

0

0

Res.

0

0

TXTF

0

0

TXTFC

0

0

0

0

0

UDRDR[15:0]
0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

1.

The configuration of this bitfield depends on the features of the SPI instance. For more details, refer to Section 68.3: SPI implementation.

2.

The bits 31-16 are reserved for the peripheral instances with data size limited to 16 bits. There is no constraint when the 32-bit access is
applied at these addresses. The bits 31-16, when reserved, are always read to zero while any write to them is ignored.

Refer to Section 2.3: Memory organization for the register boundary addresses.

<!-- pagebreak -->

1

Res.

0

1

UDR

0

1

MSSI[3:0]

UDRC

0

MIDI[3:0]

0

OVR

0

0

OVRC

0

0

Res.

0

0

CRCE

0

0

CRCEC

0

0

0

DSIZE[4:0](1)

Res.

0

0

0

TIFRE

SUSP

0

0

0

MODF

TXC

0

Res.

RXPLVL
[1:0]

0

Res.

RXWNE

0

CTSIZE[15:0](1)

SPI_SR

0x014

FTHLV[3:0](1)

0

TIFREC

Res.

Reset value

Res.

Reset value

0

MODFC

Res.

0

0

Res.

Res.

0

0

Res.

Res.

Res.

0

Res.

0

Res.

Res.

0

Res.

0

Res.

Res.

0

Res.

0

Res.

Res.

0

RDIOM

0

Res.

RXDMAEN

0

RDIOP

0

Res.

TXDMAEN

0

IOSWP

SP[2:0]

Res.

1

MASTER

1

Res.

0

0

Res.

0

Res.

Res.

Res.

1

Res.

0

CRCEN
0

Res.

Res.

CRCSIZE[4:0](1)

LSBFRST

Res.

0

Res.

Res.

Res.

SPI_IER

0

CPHA

0

0

Res.

0

Res.

0

0

CPOL

SSIOP

0

0

Res.

SSOE

Reset value

Res.

SPI_CFG2

0

SSM

0

0

Res.

0

Res.

0

Res.

0

Res.

BPASS

Reset value

SSOM

0x010

MBR[2:0]

AFCNTR

0x00C

SPI_CFG1

Res.

0x008

Res.

Reset value

TSIZE[15:0]
0

COMM[1:0]

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

SPI_CR2

0x004

0
(1)

Res.

18

12

19

Res.

HDDIR

20

Res.

13

21

Res.

SSI

22

Res.

14

23

Res.

CRC33_17

24

Res.

15

25

Res.

RCRCINI

26

Res.

16

27

Res.

TCRCINI

28

Res.

17

29

Res.

IOLOCK

30

Res.

0

Reset value

Res.

31

SPI_CR1

0x000

Res.

Register name

Res.

Offset

Res.

Table 703. SPI register map and reset values

RM0456 Rev 6

RM0456

Serial audio interface (SAI)

69

Serial audio interface (SAI)

69.1

Introduction
The SAI interface (serial audio interface) offers a wide set of audio protocols due to its
flexibility and wide range of configurations. Many audio protocols can be addressed thanks
to SAI “free protocol mode”. The free protocol mode allows the application to define the
audio frame shape, number of slots, slot size, data size, and so on. For example, the SAI
supports I2S standards, LSB- or MSB-justified, PCM/DSP, TDM, and AC’97 protocols.
SPDIF output is offered when the audio block is configured as a transmitter.
To bring this level of flexibility and reconfigurability, the SAI contains two independent audio
subblocks. Each block has it own clock generator and I/O line controller.
The SAI works in master or slave configuration. The audio subblocks are either receiver or
transmitter and work synchronously or not (with respect to the other one).
The SAI can be connected with other SAIs to work synchronously.

69.2

SAI main features
•

Two independent audio subblocks, which can be transmitters or receivers with their
respective FIFO

•

8-word integrated FIFOs for each audio subblock

•

Synchronous or asynchronous mode between the audio subblocks

•

Possible synchronization between multiple SAIs

•

Master or slave configuration independent for both audio subblocks

•

Clock generator for each audio block to target independent audio frequency sampling
when both audio subblocks are configured in master mode

•

Data size configurable: 8-, 10-, 16-, 20-, 24-, 32-bit

•

Audio protocol: I2S, LSB- or MSB-justified, PCM/DSP, TDM, AC’97

•

PDM interface, supporting up to 4 microphone pairs

•

SPDIF output available if required

•

Up to 16 slots available with configurable size

•

Number of bits by frame can be configurable

•

Frame synchronization active level configurable (offset, bit length, level)

•

First active bit position in the slot is configurable

•

LSB first or MSB first for data transfer

•

Mute mode

•

Stereo/Mono audio frame capability

•

Communication clock strobing edge configurable (SCK)

•

Error flags with associated interrupts if enabled respectively
–

Overrun and underrun detection

–

Anticipated frame synchronization signal detection in slave mode

–

Late frame synchronization signal detection in slave mode

–

Codec not ready for the AC’97 mode in reception

RM0456 Rev 6

<!-- pagebreak -->

