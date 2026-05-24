1599

Multi-function digital filter (MDF)

RM0456

Table 371. MDF trigger connections (continued)
Trigger name

Trigger source

mdf_trgi5

tim4_trgo

mdf_trgi6

tim16_oc1

mdf_trgi7

tim6_trgo

mdf_trgi8

tim7_trgo

mdf_trgi9

adf1_sad_det
(sound activity detection signal from ADF1)

mdf_trgi10

exti11

mdf_trgi11

exti15

mdf_trgi12

lptim1_ch1

mdf_trgi13

adf1_trgo signal from ADF1

The table below shows the way the break outputs of the MDF are connected.
Table 372. MDF break connections
Trigger name

Trigger source

mdf_break0

tim1_brk_cmp7

mdf_break1

tim1_brk2_cmp7

mdf_break2

tim8_brk_cmp7

mdf_break3

tim8_brk2_cmp7

The table below shows the way the ADC data are connected to the MDF.
Table 373. MDF ADC data connections
ADC data bus name

ADC source

mdf_adcitf1_dat[15:0]

adc1_dat

mdf_adcitf2_dat[15:0]

adc2_dat(1)

1. Only available in STM32U59x/5Ax/5Fx/5Gx. It is not connected in STM32U535/545/575/585.

39.4.3

Serial input interfaces (SITF)
The SITFx input interfaces allow the connection of the external sensors to the digital filters,
via the bitstream matrix (BSMX). The SITFx serial interface can be configured in the
following modes:
•

LF_MASTER SPI mode (low-frequency)

•

normal SPI mode

•

Manchester mode

The amount of SITFx instances is equal to the amount of filters.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Multi-function digital filter (MDF)
The data from each serial interface can be routed to any filter in order to perform:
•

the PDM to PCM conversion

•

the out-off limit detection

•

the short detection

The serial interfaces are enabled by setting the corresponding SITFEN bit to 1. Once the
interface is enabled, it receives serial data from the external Σ∆ modulator.
Note:

Before enabling the serial interface, the user must insure that the mdf_proc_ck is already
enabled (see Section 39.4.5 for details).
The SITFx are controlled via the MDF serial interface control register x (MDF_SITFxCR).
As shown in Figure 328, for each SITF, there is a large choice of clocking possibilities:
•

If the serial interface is programmed in SPI mode, the selected clock source is a copy
of the clock present on MDF_CCK0 or MDF_CCK1 or MDF_CKIx pin (see 2 in
Figure 328).

•

If the serial interface is programmed in LF_MASTER SPI mode, the selected clock
source must be the clock directly provided by the CCKDIV to the MDF_CCK0 or
MDF_CCK1 pin (see 1 in Figure 328). In this case MDF_CKIx must not be selected.

See Table 374 for additional information.
Note:

Using the common clock (MDF_CCK0 or MDF CCK1) can be helpful to share the same
clock between several SITFx.

RM0456 Rev 6

<!-- pagebreak -->

1599

Multi-function digital filter (MDF)

RM0456
Figure 328. SITFx overview

MDF

To BSMX

SITF0
bs0_r

bs0_f

Serial
RX

MDF_SDI0

MDF_SITF0CR.SCKSRC

MDF_CKI0

2
1
0

MDF_SITF0CR.SITFMOD

...

...

To BSMX

SITFx
bsx_r

bsx_f

Serial
RX

MDF_SDIx

MDF_SITFxCR.SCKSRC
2

MDF_CKIx

1
0

MDF_SITFxCR.SITFMOD

mdf_proc_ck
mdf_ker_ck

CCK1EN
MDF_CCK1

CKGEN

MDF_CCK0
(1) (2)
CCK0EN
mdf_ker_ck clock domain

(1) SITF0 in SPI mode
(2) SITF0 in LF_MASTER SPI mode

Note: refer to the MDF implementation section to define the ‘x’ value.

MSv62654V1

LF_MASTER and normal SPI modes
The LF_MASTER SPI mode is a special mode allowing the usage of a mdf_proc_ck clock
frequency, only two times higher than the sensor clock. This mode is dedicated to low-power
use-cases, using low-speed sensors.
In LF_MASTER SPI mode, the MDF must provide the bitstream clock to the external
sensors via MDF_CCK0 and MDF_CCK1 pins, and receives the bitstream data via the
serial data input MDF_SDIx.
For each SITFx, the application must select the same clock than the one provided to the
external sensor (MDF_CCK0 or MDF_CCK1), in order to guarantee optimal timing
performances. This selection is done via SCKSRC[1:0].

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Multi-function digital filter (MDF)

Warning:

The MDF_CKIx pin cannot be used in LF_MASTER SPI mode.

The normal SPI interface is a more flexible interface than the LF_MASTER SPI, but the
mdf_proc_ck frequency must be at least four times higher than the sensor clock.
The application can select MDF_CCK0, MDF_CCK1 or MDF_CKIx clock for the capture of
the data received via the MDF_SDIx pin.
The MDF can generate a clock to the sensors via MDF_CCK0 or MDF_CCK1 if needed.
For all SPI modes, all SITFs can share the same clock input (MDF_CCK0 or MDF_CCK1),
in order to optimize the amount of requested I/Os.
For all SPI modes, the serial data is captured using the rising and the falling edge of the
selected clock. The SITFx always provides the following bitstreams:
•

bitstream received using the bitstream clock falling edge (bsx_f)

•

bitstream received using the bitstream clock rising edge (bsx_r)

According to the sensors connected, one of the two bitstreams may not be available.
The application can select the wanted stream via the BSMX matrix.
Figure 329. SPI timing example
FBS
MDF_CCKy or
MDF_CKIx
MDF_SDIx

L0

R0

L1

R1

L2

R2

L3

R3

bsx_ck
bsx_r

L0

L1

L2

L3

bsx_f

R0

R1

R2

R3

Sampling point
MSv62655V1

To properly synchronize/receive the data stream, the frequency of the mdf_proc_ck clock
must be adjusted according to the constraints listed in Table 375.

Clock absence detection
A no-clock-transition period may be detected when the serial interface works in normal SPI
mode. This feature can be used to detect a clock failure in the SPI link.
The application can program a timeout value via the STH[4:0] bitfield of the corresponding
SITFx. If the MDF does not detect clock transitions for a duration of STH[4:0] x Tmdf_proc_ck,
then the CKABF flag is set.
An interrupt can be generated if CKABIE is set to 1. The STH[4:0] bitfield is in the MDF
serial interface control register x (MDF_SITFxCR).

RM0456 Rev 6

<!-- pagebreak -->

1599

Multi-function digital filter (MDF)

RM0456

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

If CKABF i= 0, a clock has been detected. The CKABIE bit can be set to 1 if the
application wants an interrupt on detection of a clock absence.

The clock absence detection feature is not available in the LF_MASTER SPI mode.

Manchester mode
In Manchester coded format, the MDF receives data stream from the external sensor via the
MDF_SDIx pin only.
The MDF_CKIx pins are not needed in this mode.
Decoded data and clock signals are recovered from serial stream after Manchester
decoding. They are available on bsx_r. There are two possible settings of Manchester
codings:

<!-- pagebreak -->

•

signal rising edge decoded as 0 and signal falling edge decoded as 1

•

signal rising edge decoded as 1 and signal falling edge decoded as 0

RM0456 Rev 6

RM0456

Multi-function digital filter (MDF)
Figure 330. Manchester timing example (SITFMOD = 11)

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

MDF_SDIx
mdf_proc_ck
10

STH[4:0] = 5

5

MCNT

bsx_ck

bsx_r

X

CKABF
Clear of the
CKABF
Signal absence if STH counter is higher than 2*STH
Long transitions if STH counter is between STH and 2*STH
Short transitions if STH counter lower than STH

MSv62656V1

To decode the incoming Manchester stream, the user must program the STH[4:0] bitfield in
the MDF serial interface control register x (MDF_SITFxCR). The STH[4:0] bitfield is used by
the SITFx to estimate the Manchester symbol length and to detect a clock absence. An
internal counter (MCNT) is restarted every time a transition is detected in the MDF_SDIx
input. It is used to detect short transitions, long transitions or clock absence. A long
transition indicates that the data value changed. Figure 330 shows a case where the OVR is
around height and STH[4:0] = 5.
The estimated Manchester symbol rate (TSYMB) must respect the following formula:
(STH + 1)×Tmdf_proc_ck < TSYMB < (2 × STH × Tmdf_proc_ck)
It is recommended to compute STH as follows:
( 2 × OVR ) – 1
STH [ 4:0 ] = round ⎛⎝ ------------------------------------- ⎞⎠
3

where OVR represents the ratio between the mdf_proc_ck frequency and the expected
Manchester symbol frequency. OVR must be higher than five, and the mdf_proc_ck clock
must be adjusted according to the constraints listed in Table 375.
The clock absence flag CKABF is set to 1 when no transition is detected during more than
2 x STH[4:0] x Tmdf_proc_ck, or when the SITFx is not yet synchronized to the incoming
Manchester stream. In addition, an interrupt can be generated if the bit CKABIE is set to 1.

RM0456 Rev 6

<!-- pagebreak -->

