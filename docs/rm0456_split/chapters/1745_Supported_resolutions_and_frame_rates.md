Signal name

Signal type

Description

DSI_D0P/D0N

Input/Output

Differential Data lane 0

DSI_D1P/D1N

Output

Differential Data lane 1

RM0456 Rev 6

RM0456

DSI Host (DSI)
Table 435. DSI pins (continued)
Signal name

Signal type

Description

DSI_CKP/CKN

Output

Differential clock

DSI_TE

Input

DSI tearing effect pin

Table 436. DSI internal input/output signals

44.4.3

Signal name

Signal type

dsi_it

Output

Description
DSI global interrupt

Supported resolutions and frame rates
The DSI specification does not define supported standard resolutions or frame rates.
Display resolution, blanking periods, synchronization events duration, frame rates, and pixel
color depth play a fundamental role in the required bandwidth. In addition, other link-related
attributes can influence the ability of the link to support a DSI-specific device, namely
display input buffering capabilities, video transmission mode (burst or non-burst), bus
turn-around (BTA) time, concurrent command mode traffic in a video mode transmission, or
display device specifics. All these variables make it difficult to define a standard procedure
to estimate the minimum lane rate and the minimum number of lanes that support a specific
display device.
The basic assumptions for estimates are:

44.4.4

•

clock lane frequency is 250 MHz, resulting in a bandwidth of 500 Mbit/s for each data
lane

•

the display must be capable of buffering the pixel data at the speed at which it is
delivered in the DSI link

•

no significant control traffic is present on the link when the pixel data are transmitted.

System level architecture
Figure 413 shows the architecture of the DSI Host.

RM0456 Rev 6

<!-- pagebreak -->

1861

DSI Host (DSI)

RM0456
Figure 413. DSI Host architecture

Bias
PLL
DSI Host

Ctrl
LTDC

RGB

DATAP1
DSI
Wrapper

RGB

LTDC
I/F
APB to
Generic

APB

DATAN1
Packet
handler

D-PHY
I/F

PPI

DATAP0
D-PHY

DATAN0
CLKP

Register
bank

APB

Error
management

CLKN

MS53588V1

The different parts have the following functions:

<!-- pagebreak -->

•

The DSI Wrapper ensures the interfacing between the LTDC and the DSI Host kernel.
It can adapt the color mode, and manages the tearing effect (TE) for automatic frame
buffer update in adapted command mode. The DSI Wrapper also controls the DSI bias,
the DSI PLL and specific functions of the MIPI® D-PHY.

•

The LTDC interface captures the data and control signals from the LTDC and conveys
them to a FIFO for video control signals and another one for the pixel data. This data is
then used to build one of the following:
–

Video packets, when in video mode (see Section 44.5)

–

The memory_write_start and memory_write_continue DCS commands, when in
adapted command mode (see Section 44.6)

•

The register bank is accessible through a standard APB interface, providing access to
the DSI Host registers for configuration and control. There is also a fully programmable
interrupt generator to inform the system about certain events.

•

The PHY interface control is responsible for managing the D-PHY interface. It
acknowledges the current operation and enables low-power transmission/reception or
a high-speed transmission. It also performs data splitting between available D-PHY
lanes for high-speed transmission.

•

The packet handler schedules the activities inside the link. It performs several functions
based on the currently operational interfaces and the used video transmission mode
(burst mode or non-burst mode with sync pulses or sync events). It builds long or short
packets generating correspondent ECC and CRC codes. This block also performs the
following functions:
–

packet reception

–

validation of packet header by checking the ECC

–

header correction and notification for single-bit errors

–

termination of reception

–

multiple header error notification

RM0456 Rev 6

RM0456

DSI Host (DSI)
–
•

•

depending on the virtual channel of the incoming packet, the handler routes the
output data to the respective port.

The APB-to-generic block bridges the APB operations into FIFOs holding the generic
commands. The block interfaces with the following FIFOs:
–

Command FIFO

–

Write payload FIFO

–

Read payload FIFO

The error management notifies and monitors the error conditions on the DSI link. It
controls the timers used to determine if a timeout condition occurred, performing an
internal soft reset and triggering an interruption notification.

RM0456 Rev 6

<!-- pagebreak -->

