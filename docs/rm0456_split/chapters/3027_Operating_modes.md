3134

Universal serial bus full-speed host/device interface (USB)

71.7.3

RM0456

Channel/endpoint receive buffer descriptor n
(USB_CHEP_RXTXBD_n)
Address offset: 0x4 + 0x8 * n, (n = 0 to 7)
Reset value: 0xXXXX XXXX
This register description applies when corresponding CHEPnR register does not program
use of double buffering in the transmit mode (otherwise refer to following register
description).
This table location is used to store two different values, both required during packet
reception. The most significant bits contains the definition of allocated buffer size, to allow
buffer overflow detection, while the least significant part of this location is written back by the
USB peripheral at the end of reception to give the actual number of received bytes. Due to
the restrictions on the number of available bits, buffer size is represented using the number
of allocated memory blocks, where block size can be selected to choose the trade-off
between fine-granularity/small-buffer and coarse-granularity/large-buffer. The size of
allocated buffer is a part of the endpoint/channel descriptor and it is normally defined during
the enumeration process according to its maxPacketSize parameter value (see “Universal
Serial Bus Specification”).

31

30

BLSIZE

29

28

27

26

25

24

23

22

NUM_BLOCK[4:0]

21

20

19

18

17

16

COUNT_RX[9:0]

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

rw

rw

rw

rw

rw

rw

rw

ADDR_RX[15:0]
rw

rw

rw

rw

rw

rw

rw

rw

rw

Bit 31 BLSIZE: Block size
This bit selects the size of memory block used to define the allocated buffer area.
–
If BLSIZE = 0, the memory block is 2-byte large, which is the minimum block
allowed in a half-word wide memory. With this block size the allocated buffer size
ranges from 2 to 62 bytes.
–
If BLSIZE = 1, the memory block is 32-byte large, which permits to reach the
maximum packet length defined by USB specifications. With this block size the
allocated buffer size theoretically ranges from 32 to 1024 bytes, which is the longest
packet size allowed by USB standard specifications. However, the applicable size is
limited by the available buffer memory.
Bits 30:26 NUM_BLOCK[4:0]: Number of blocks
These bits define the number of memory blocks allocated to this packet buffer. The actual
amount of allocated memory depends on the BLSIZE value as illustrated in Table 746.
Bits 25:16 COUNT_RX[9:0]: Reception byte count
These bits contain the number of bytes received by the endpoint/channel associated with the
USB_CHEPnR register during the last OUT/SETUP transaction addressed to it.
Note: Although the application only needs to read this value, it is writable.
Bits 15:0 ADDR_RX[15:0]: Reception buffer address
These bits point to the starting address of the packet buffer, which contains the data received
by the endpoint/channel associated with the USB_CHEPnR register at the next OUT/SETUP
token addressed to it. Bits 1 and 0 must always be written as “00” since packet memory is
word wide and all packet buffers must be word aligned.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Universal serial bus full-speed host/device interface (USB)

71.7.4

Channel/endpoint transmit buffer descriptor n [alternate]
(USB_CHEP_RXTXBD_n)
Address offset: 0x4 + 0x8 * n, (n = 0 to 7)
Reset value: 0xXXXX XXXX
This register description applies when corresponding CHEPnR register programs use of
double buffering and activates transmit buffers (otherwise refer to previous register
description).

31

30

29

28

27

26

Res.

Res.

Res.

Res.

Res.

Res.

25

24

23

22

21

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

rw

rw

rw

rw

rw

rw

rw

rw

rw

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

COUNT_TX[9:0]

ADDR_TX[15:0]
rw

rw

Bits 31:26 Reserved, must be kept at reset value.
Bits 25:16 COUNT_TX[9:0]: Transmission byte count
These bits contain the number of bytes to be transmitted by the endpoint/channel associated
with the USB_CHEPnR register at the next IN token addressed to it.
Bits 15:0 ADDR_TX[15:0]: Transmission buffer address
These bits point to the starting address of the packet buffer containing data to be transmitted
by the endpoint/channel associated with the USB_CHEPnR register at the next IN token
addressed to it. Bits 1 and 0 must always be written as “00” since packet memory is word
wide and all packet buffers must be word aligned.

RM0456 Rev 6

<!-- pagebreak -->

3134

Universal serial bus full-speed host/device interface (USB)

71.7.5

RM0456

USBSRAM register map
The table below provides the USB register map and reset values.

0x3C

<!-- pagebreak -->

Res.

Res.

Res.

Res.

Res.

Res.
BLSIZE
BLSIZE

X

X

X

Res.

Res.

Res.

Res.

Res.

Res.

X

X

X

X

X

X

Res.

Res.

Res.

Res.

Res.

Res.
BLSIZE
BLSIZE

X

X

X

X

X

X

X

X

NUM_
BLOCK[4:0]
X

X

X

X

Res.

Res.

Res.

Res.

X

X

X

X

Res.

Res.

Res.

X

X

X

NUM_
BLOCK[4:0]
X

X

X

X

X

X

X

X

X

X

X

X

X

X

X

X

X

X

X

X

X

X

X

X

X

X

X

X

X

X

X

X

X

X

X

X

X

X

X

X

X

X

X

X

X

X

X

X

X

X

X

X

X

X

X

X

X

X

X

X

X

X

X

X

X

X

X

X

X

X

X

X

X

X

X

X

X

X

X

X

X

X

X

X

X

X

X

X

X

X

X

X

X

X

X

X

X

X

X

X

X

X

X

X

X

X

X

X

X

X

X

X

X

X

X

X

X

X

X

X

X

X

X

X

X

X

X

X

X

X

X

X

X

X

X

X

X

X

X

X

X

X

X

X

X

X

X

X

X

X

X

X

X

X

X

X

X

X

X

X

X

X

X

X

X

X

X

X

X

X

X

X

X

X

X

X

X

X

ADDR_TX[15:0]
X

X

X

X

X

X

X

X

X

X

X

X

X

ADDR_RX[15:0]
X

X

X

X

X

X

X

X

X

X

X

X

X

ADDR_RX[15:0]
X

X

X

X

X

X

X

X

X

X

X

X

X

ADDR_TX[15:0]
X

X

X

.
.
.
.

X

X

X

X

X

X

X

X

X

X

ADDR_TX[15:0]
X

X

X

X

X

X

X

X

X

X

X

X

X

ADDR_RX[15:0]
X

X

X

X

X

X

X

X

X

X

X

X

X

ADDR_RX[15:0]
X

X

X

X

X

X

X

X

X

COUNT_TX[9:0]
X

X

ADDR_TX[15:0]

COUNT_RX[9:0]

X

X

ADDR_RX[15:0]

COUNT_RX[9:0]

NUM_
BLOCK[4:0]

X

ADDR_RX[15:0]

COUNT_TX[9:0]
X

X

X

COUNT_TX[9:0]
X

X

X

COUNT_RX[9:0]

X

X

X

COUNT_RX[9:0]

X

X

X

NUM_
BLOCK[4:0]
X

X

COUNT_TX[9:0]
X

X

X

COUNT_TX[9:0]
X

Res.

0x3C

X

Res.

0x38

X

X

X

COUNT_RX[9:0]

X

Res.

0x38

X

Res.

.
.
.
.

X

Res.

0x0C

X

NUM_
BLOCK[4:0]

Res.

0x0C

X

Res.

0x08

X

Res.

0x08

X

ADDR_TX[15:0]

COUNT_RX[9:0]

X

Res.

0x04

X

X

NUM_
BLOCK[4:0]

BLSIZE

0x04

COUNT_TX[9:0]
X

BLSIZE

0x00

USB_
CHEP_
TXRXBD_0
Reset value
USB_
CHEP_
TXRXBD_0
[alternate]
Reset value
USB_
CHEP_
RXTXBD_0
Reset value
USB_
CHEP_
RXTXBD_0
[alternate]
Reset value
USB_
CHEP_
TXRXBD_1
Reset value
USB_
CHEP_
TXRXBD_1
[alternate]
Reset value
USB_
CHEP_
RXTXBD_1
Reset value
USB_
CHEP_
RXTXBD_1
[alternate]
Reset value
.
.
.
.
USB_
CHEP_
TXRXBD_7
Reset value
USB_
CHEP_
TXRXBD_7
[alternate]
Reset value
USB_
CHEP_
RXTXBD_7
Reset value
USB_
CHEP_
RXTXBD_7
[alternate]
Reset value

Res.

0x00

Register

Res.

Offset

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

Table 747. USBSRAM register map and reset values

X

X

X

X

ADDR_TX[15:0]
X

X

RM0456 Rev 6

X

X

X

X

X

X

X

X

X

X

X

RM0456

72

USB on-the-go full-speed (OTG_FS)

USB on-the-go full-speed (OTG_FS)
This section applies to STM32U575/585 devices only.

72.1

Introduction
Portions Copyright (c) Synopsys, Inc. All rights reserved. Used with permission.
This section presents the architecture and the programming model of the OTG_FS
controller.
The following acronyms are used throughout the section:
FS

Full-speed

LS

Low-speed

MAC

Media access controller

OTG

On-the-go

PFC

Packet FIFO controller

PHY

Physical layer

USB

Universal serial bus

UTMI

USB 2.0 Transceiver Macrocell interface (UTMI)

LPM

Link power management

BCD

Battery charging detector

HNP

Host negotiation protocol

SRP

Session request protocol

References are made to the following documents:
•

USB On-The-Go Supplement, Revision 2.0

•

Universal Serial Bus Revision 2.0 Specification

•

USB 2.0 Link Power Management Addendum Engineering Change Notice to the USB
2.0 specification, July 16, 2007

•

Errata for USB 2.0 ECN: Link Power Management (LPM) - 7/2007

•

Battery Charging Specification, Revision 1.2

The USB OTG is a dual-role device (DRD) controller that supports both device and host
functions and is fully compliant with the On-The-Go Supplement to the USB 2.0
Specification. It can also be configured as a host-only or device-only controller, fully
compliant with the USB 2.0 Specification. OTG_FS supports the speeds defined in the
Table 748: OTG_FS speeds supported below. The USB OTG supports both HNP and SRP.
The only external device required is a charge pump for VBUS in OTG mode.

RM0456 Rev 6

<!-- pagebreak -->

3291

USB on-the-go full-speed (OTG_FS)

RM0456

Table 748. OTG_FS speeds supported
-

<!-- pagebreak -->

HS (480 Mb/s)

FS (12 Mb/s)

LS (1.5 Mb/s)

Host mode

-

X

X

Device mode

-

X

-

RM0456 Rev 6

RM0456

72.2

USB on-the-go full-speed (OTG_FS)

OTG_FS main features
The main features can be divided into three categories: general, host-mode and devicemode features.

72.2.1

General features
The OTG_FS interface general features are the following:
•

It is USB-IF certified to the Universal Serial Bus Specification Rev 2.0

•

OTG_FS supports the following PHY interface:
–

•

•

•

An on-chip full-speed PHY

It includes full support (PHY) for the optional On-The-Go (OTG) protocol detailed in the
On-The-Go Supplement Rev 2.0 specification
–

Integrated support for A-B device identification (ID line)

–

Integrated support for host Negotiation protocol (HNP) and session request
protocol (SRP)

–

It allows host to turn VBUS off to conserve battery power in OTG applications

–

It supports OTG monitoring of VBUS levels with internal comparators

–

It supports dynamic host-peripheral switch of role

It is software-configurable to operate as:
–

SRP capable USB FS Peripheral (B-device)

–

SRP capable USB FS/LS host (A-device)

–

USB On-The-Go Full-Speed Dual Role device

It supports FS SOF and LS Keep-alives with
–

SOF pulse PAD connectivity

–

SOF pulse internal connection to timer (TIMx)

–

Configurable framing period

–

Configurable end of frame interrupt

•

It includes power saving features such as system stop during USB suspend, switch-off
of clock domains internal to the digital core, PHY and DFIFO power management.

•

It features a dedicated RAM of 1.25 Kbytes with advanced FIFO control:
–

Configurable partitioning of RAM space into different FIFOs for flexible and
efficient use of RAM

–

Each FIFO can hold multiple packets

–

Dynamic memory allocation

–

Configurable FIFO sizes that are not powers of 2 to allow the use of contiguous
memory locations

•

It guarantees max USB bandwidth for up to one frame (1 ms) without system
intervention.

•

It supports charging port detection as described in Battery Charging Specification
Revision 1.2.

RM0456 Rev 6

<!-- pagebreak -->

3291

USB on-the-go full-speed (OTG_FS)

72.2.2

RM0456

Host-mode features
The OTG_FS interface main features and requirements in host-mode are the following:
•

External charge pump for VBUS voltage generation.

•

Up to 12 host channels (pipes): each channel is dynamically reconfigurable to allocate
any type of USB transfer.

•

Built-in hardware scheduler holding:

•

72.2.3

–

Up to 12 interrupt plus isochronous transfer requests in the periodic hardware
queue

–

Up to 12 control plus bulk transfer requests in the non-periodic hardware queue

Management of a shared Rx FIFO, a periodic Tx FIFO and a nonperiodic Tx FIFO for
efficient usage of the USB data RAM.

Peripheral-mode features
The OTG_FS interface main features in peripheral-mode are the following:

<!-- pagebreak -->

•

1 bidirectional control endpoint0

•

5 IN endpoints (EPs) configurable to support bulk, interrupt or isochronous transfers

•

5 OUT endpoints configurable to support bulk, interrupt or isochronous transfers

•

Management of a shared Rx FIFO and a Tx-OUT FIFO for efficient usage of the USB
data RAM

•

Management of up to 6 dedicated Tx-IN FIFOs (one for each active IN EP) to put less
load on the application

•

Support for the soft disconnect feature.

RM0456 Rev 6

RM0456

72.3

USB on-the-go full-speed (OTG_FS)

OTG_FS implementation
Table 749. OTG_FS implementation(1)
USB features

OTG_FS for
STM32U575/585

Device bidirectional endpoints (including EP0)

6

Host mode channels

12

Size of dedicated SRAM

1.2 Kbytes

USB 2.0 link power management (LPM) support

X

OTG revision supported

2.0

Battery charging detection (BCD) support

X

Integrated PHY

FS

1. “X” = supported, “-” = not supported, “FS” = supported in FS mode, “HS” = supported in HS mode.

RM0456 Rev 6

<!-- pagebreak -->

3291

USB on-the-go full-speed (OTG_FS)

RM0456

72.4

OTG_FS functional description

72.4.1

OTG_FS block diagram
Figure 893. OTG_FS full-speed block diagram

AHB peripheral

Cortex core

USB Interrupt

DP
DM
USB2.0
OTG FS
core

Power
and
clock control

OTG FS
PHY

UTMIFS

ID
VBUS
Universal serial bus

USB suspend
SOF

USB clock at 48 MHz
USB clock
domain

NOE

RAM bus

System clock
domain

1.25 Kbyte
USB data
FIFOs
MSv47468V1

72.4.2

OTG_FS pin and internal signals
Table 750. OTG_FS input/output pins
Signal name

<!-- pagebreak -->

