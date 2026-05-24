RM0456 Rev 6

RM0456

Audio digital filter (ADF)
Table 391. ADF internal signals (continued)

Signal name

Signal type

adf_flt0_dma

Remarks

Input/output DMA request/acknowledge signals for the ADF processing chain

adf_flt0_it

Output

Global interrupt signals

adf_bus_ckreq

Output

Bus interface clock request output

adf_ker_ckreq

Output

Kernel clock request output

adf_ker_ck

Input

Kernel clock input

adf_hclk

Input

AHB bus interface clock input

adf_sad_det

Output

SAD sound detection: 1 means that detecting sound

adf_adcitf1_dat[15:0]

Input

ADCITF1 data input

adf_adcitf2_dat[15:0]

Input

ADCITF2 data input

Table 392. ADF trigger connections

40.4.3

Trigger name

Direction

Trigger source/destination

adf_trgi

Input

From exti15

adf_trgo

Output

To mdf_trgi13

Serial input interface (SITF)
The SITF0 input interface allows the connection of the external sensor to the digital filter via
the bitstream matrix (BSMX). The SITF0 can be configured in the following modes:
•

LF_MASTER SPI mode (low-frequency)

•

normal SPI mode

•

Manchester mode

The data from the serial interface is routed to the filter in order to perform the PDM to PCM
conversion and the sound activity detection.
The serial interface is enabled by setting the SITFEN bit to 1. Once the interface is enabled,
it receives serial data from the external Σ∆ modulator.
Note:

Before enabling the serial interface, the user must insure that the adf_proc_ck is already
enabled (see Section 40.4.5: Clock generator (CKGEN) for details).
The SITF0 is controlled via the ADF serial interface control register 0 (ADF_SITF0CR).
As shown in the Figure 362, ADF_CCK0 or ADF_CCK1 can be selected as clock source, in
order to sample the incoming bitstream:
•

If the serial interface is programmed in SPI mode, the selected clock source is a copy
of the clock present on the ADF_CCK0 or ADF_CCK1 pin.

•

If the serial interface is programmed in LF_MASTER SPI mode, the selected clock
source is the clock directly provided by the CCKDIV to the ADF_CCK0 or ADF_CCK1
pin.

See Table 393 for additional information.

RM0456 Rev 6

<!-- pagebreak -->

1674

Audio digital filter (ADF)

RM0456
Figure 362. SITF overview

ADF

To BSMX

SITF0
ADF_SDI0

bs0_r

bs0_f

Serial
RX

SCKSRC
1
0

SITFMOD

adf_proc_ck
adf_ker_ck

CCK1EN
ADF_CCK1

CKGEN

ADF_CCK0
(1)

(2)

CCK0EN
adf_ker_ck clock domain

(1) SITF0 in SPI mode
(2) SITF0 in LF_MASTER SPI mode

MSv63651V1

LF_MASTER and normal SPI modes
The LF_MASTER SPI mode is a special mode allowing the use of an adf_proc_ck clock
frequency, only two times bigger than the sensor clock. This mode is dedicated to low-power
use-cases, using low-speed sensors.
In LF_MASTER SPI mode, the ADF must provide the bitstream clock to the external
sensors via ADF_CCK0 and ADF_CCK1 pins. The ADF receives the bitstream data via the
serial data input ADF_SDI0.
For the SITF0, the application must select the same clock than the one provided to the
external sensor (ADF_CCK0 or ADF_CCK1), in order to guarantee optimal timing
performances. This selection is done via SCKSRC[1:0].
The normal SPI interface is a more flexible interface than the LF_MASTER SPI, but the
adf_proc_ck frequency must be at least four times higher than the sensor clock.
The application can select ADF_CCK0 or ADF_CCK1 clock for the capture of the data
received via the ADF_SDI0 pin.
The ADF can generate a clock to the sensors via ADF_CCK0 or ADF_CCK1 if needed.
For all SPI modes, the serial data is captured using the rising and the falling edge of the
selected clock. The SITF0 always provides the following bitstreams:
•

bitstream received using the bitstream clock falling edge (bs0_f)

•

bitstream received using the bitstream clock rising edge (bs0_r)

According to the sensors connected, one of the two bitstreams may not be available.
The application can select the wanted stream via the BSMX matrix.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Audio digital filter (ADF)
Figure 363. SPI timing example
FBS
ADF_CCK0 or
ADF_CCK1
ADF_SDI0

L0

R0

L1

R1

L2

R2

L3

R3

bs0_ck
bs0_r

L0

L1

L2

L3

bs0_f

R0

R1

R2

R3

Sampling point
MSv63670V1

To properly synchronize/receive the data stream, the adf_proc_ck frequency must be
adjusted according to the constraints listed in Table 394.

Clock absence detection
A no-clock-transition period may be detected when the serial interface works in normal SPI
mode. This feature can be used to detect a clock failure in the SPI link.
The application can program a timeout value via the STH[4:0] bitfield of the SITF0. If the
ADF does not detect clock transitions for a duration of STH[4:0] x Tadf_proc_ck, then the
CKABF flag is set.
An interrupt can be generated if CKABIE is set to 1. The STH[4:0] bitfield is in the ADF serial
interface control register 0 (ADF_SITF0CR).
When the serial interface is enabled, the CKABF flag remains to 1 until a first clock transition
is detected.
To avoid spurious clock absence detection, the following sequence must be respected:
1.

Configure the serial interface in normal SPI mode and enable it.

2.

Clear the CKABF flag by writing CKABF bit to 1.
If no clock transition is detected on the serial interface, the hardware immediately sets
the CKABF flag to 1.

3.

Note:

Read the CKABF flag:
–

If CKABF = 1, go back to step 2.

–

If CKABF = 0, a clock has been detected. The CKABIE bit can be set to 1 if the
application wants an interrupt on detection of a clock absence.

The clock absence detection feature is not available in the LF_MASTER SPI mode.

Manchester mode
In Manchester coded format, the ADF receives data stream from the external sensor via the
ADF_SDI0 pin only.
The ADF_CCK0 and ADF_CCK1 pins are not needed in this mode.

RM0456 Rev 6

<!-- pagebreak -->

1674

Audio digital filter (ADF)

RM0456

Decoded data and clock signals are recovered from serial stream after Manchester
decoding. They are available on bs0_r. There are two possible settings of Manchester
codings:
•

signal rising edge decoded as 0 and signal falling edge decoded as 1

•

signal rising edge decoded as 1 and signal falling edge decoded as 0
Figure 364. Manchester timing example (SITFMOD = 11)

Data
transferred

1

1

0

0

1

0

0

TSYMB

ADF_SDI0
adf_proc_ck
10

STH[4:0] = 5

5

MCNT

bs0_ck

bs0_r

X

CKABF
Clear of the
CKABF
Signal absence if STH counter is higher than 2*STH
Long transitions if STH counter is between STH and 2*STH
Short transitions if STH counter lower than STH

MSv63652V1

To decode the incoming Manchester stream, the user must program STH[4:0] in the ADF
serial interface control register 0 (ADF_SITF0CR). The STH[4:0] bitfield is used by the
SITF0 to estimate the Manchester symbol length and to detect a clock absence. An internal
counter (MCNT) is restarted every time a transition is detected in the ADF_SDI0 input. It is
used to detect short transitions, long transitions or clock absence. A long transition indicates
that the data value changed. Figure 364 shows a case where the OVR is around height and
STH[4:0] = 5.
The estimated Manchester symbol rate (TSYMB) must respect the following formula:
(STH + 1)×Tadf_proc_ck < TSYMB < (2 × STH × Tadf_proc_ck)
It is recommended to compute STH as follows:
( 2 × OVR ) – 1
STH [ 4:0 ] = round ⎛⎝ ------------------------------------- ⎞⎠
3

<!-- pagebreak -->

