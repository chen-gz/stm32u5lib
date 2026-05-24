1443

Analog-to-digital converter (ADC4)

34.4.20

RM0456

End of conversion sequence (EOS flag)
The ADC notifies the application of each end of sequence (EOS) event.
The ADC sets the EOS flag in the ADC_ISR register as soon as the last data result of a
conversion sequence is available in the ADC_DR register. An interrupt can be generated if
the EOSIE bit is set in the ADC_IER register. The EOS flag is cleared by software by writing
1 to it.

34.4.21

Example timing diagrams (single/continuous modes
hardware/software triggers)
Figure 287. Single conversions of a sequence, software trigger

ADSTART(1)
EOC
EOS
SCANDIR
ADC state(2)

RDY

ADC_DR
by S/W

CH0

CH9

CH10 CH17

D0

D9

RDY

D10

D17

CH17 CH10 CH9

CH0

RDY

D17

D9

D0

D10

by H/W
MSv30338V3

1. EXTEN = 00, CONT = 0.
2. CHSEL = 0x20601, WAIT = 0, AUTOFF = 0.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Analog-to-digital converter (ADC4)
Figure 288. Continuous conversion of a sequence, software trigger

ADSTART(1)
EOC
EOS
ADSTP
SCANDIR
ADC state(2)

RDY

CH0

ADC_DR

by S/W

CH9

CH10 CH17

CH0

CH9

D0

D9

D17

D0

D10

RDY

CH10 STP

CH17

D9

CH10
D17

by H/W
MSv30339V2

1. EXTEN = 00, CONT = 1.
2. CHSEL = 0x20601, WAIT = 0, AUTOFF = 0.

Figure 289. Single conversions of a sequence, hardware trigger

ADSTART(1)
EOC
EOS
TRGx(1)
ADC state(2)

RDY

ADC_DR

by S/W

by H/W

triggered

ignored

CH0

CH1

CH2

CH3

RDY CH0

CH1

CH2

CH3

RDY

D0

D1

D2

D3

D0

D1

D2

D3

MSv30340V2

1. EXTSEL = TRGx (over-frequency), EXTEN = 01 (rising edge), CONT = 0.
2. CHSEL = 0xF, SCANDIR = 0, WAIT = 0, AUTOFF = 0.

RM0456 Rev 6

<!-- pagebreak -->

1443

Analog-to-digital converter (ADC4)

RM0456

Figure 290. Continuous conversions of a sequence, hardware trigger

ADSTART(1)
EOC
EOS
ADSTP
TRGx(1)
ADC state(2)

RDY

CH0

CH1

CH2

CH3

CH0

CH1

CH2

CH3

CH0 STOP

D0

D1

D2

D3

D0

D1

D2

D3

ADC_DR

by S/W

by H/W

triggered

ignored

RDY

MSv30341V2

1. EXTSEL = TRGx, EXTEN = 10 (falling edge), CONT = 1.
2. CHSEL = 0xF, SCANDIR = 0, WAIT = 0, AUTOFF = 0.

34.4.22

Low-frequency trigger mode
If the application has to support a time longer than the maximum tIDLE value (between one
trigger to another for single conversion mode or between the ADC enable and the first ADC
conversion), then the ADC internal state needs to be rearmed. This mechanism can be
enabled by setting LFTRIG bit to 1 in ADC_CFGR2 register. By setting this bit, any trigger
(software or hardware) sends a rearm command to ADC. The conversion is started after a
two ADC clock cycle delay compared to LFTRIG set to 0.
It is not necessary to use this mode when AUTOFF bit is set to 1. For wait mode, only the
first trigger generates an internal rearm command.

34.4.23

Data management
Data register and data alignment (ADC_DR, ALIGN)
At the end of each conversion (when an EOC event occurs), the result of the converted data
is stored in the ADC_DR data register which is 16-bit wide.
The format of the ADC_DR depends on the configured data alignment and resolution.
The ALIGN bit in the ADC_CFGR1 register selects the alignment of the data stored after
conversion. Data can be right-aligned (ALIGN = 0) or left-aligned (ALIGN = 1) as shown in
Figure 291.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Analog-to-digital converter (ADC4)
Figure 291. Data alignment and resolution (oversampling disabled: OVSE = 0)
15

ALIGN RES
0

14

13

12

11

9

8

7

6

0x0

0x0

5

4

2

1

0

DR[9:0]
0x00

0x2

3

DR[11:0]
0x00

0x1

DR[7:0]
0x00

0x3
1

10

DR[5:0]
DR[11:0]

0x0

0x0
0x00

DR[9:0]

0x1
0x2

DR[7:0]

0x3

0x00

0x00
DR[5:0]

0x0
MS30342V1

ADC overrun (OVR, OVRMOD)
The overrun flag (OVR) indicates a data overrun event, when the converted data was not
read in time by the CPU or the DMA, before the data from a new conversion is available.
The OVR flag is set in the ADC_ISR register if the EOC flag is still at 1 at the time when a
new conversion completes. An interrupt can be generated if the OVRIE bit is set in the
ADC_IER register.
When an overrun condition occurs, the ADC keeps operating and can continue to convert
unless the software decides to stop and reset the sequence by setting the ADSTP bit in the
ADC_CR register.
The OVR flag is cleared by software by writing 1 to it.
It is possible to configure if the data is preserved or overwritten when an overrun event
occurs by programming the OVRMOD bit in the ADC_CFGR1 register:
•

OVRMOD = 0
–

•

An overrun event preserves the data register from being overwritten: the old data
is maintained and the new conversion is discarded. If OVR remains at 1, further
conversions can be performed but the resulting data is discarded.

OVRMOD = 1
–

The data register is overwritten with the last conversion result and the previous
unread data is lost. If OVR remains at 1, further conversions can be performed
and the ADC_DR register always contains the data from the latest conversion.

RM0456 Rev 6

<!-- pagebreak -->

1443

Analog-to-digital converter (ADC4)

RM0456

Figure 292. Example of overrun (OVR)

ADSTART(1)
EOC
EOS
OVR
ADSTP
TRGx(1)
ADC state(2)

RDY

CH0

CH1

CH2

CH0

CH1

CH0 STOP

RDY

OVERRUN

ADC_DR read
access
ADC_DR
(OVRMOD=0)

D0

D1

D2

D0

ADC_DR
(OVRMOD=1)

D0

D1

D2

D0

by S/W

CH2

D1

D2

by H/W

triggered
MSv30343V3

Managing a sequence of data converted without using the DMA
If the conversions are slow enough, the conversion sequence can be handled by software.
In this case the software must use the EOC flag and its associated interrupt to handle each
data result. Each time a conversion is complete, the EOC bit is set in the ADC_ISR register
and the ADC_DR register can be read. The OVRMOD bit in the ADC_CFGR1 register must
be configured to 0 to manage overrun events as an error.

Managing converted data without using the DMA without overrun
It may be useful to let the ADC convert one or more channels without reading the data after
each conversion. In this case, the OVRMOD bit must be configured at 1 and the OVR flag
must be ignored by the software. When OVRMOD is et to 1, an overrun event does not
prevent the ADC from continuing to convert and the ADC_DR register always contains the
latest conversion data.

Managing converted data using the DMA
Since all converted channel values are stored in a single data register, it is efficient to use
DMA when converting more than one channel. This avoids losing the conversion data
results stored in the ADC_DR register.
When DMA mode is enabled (DMAEN bit set to 1 in the ADC_CFGR1 register), a DMA
request is generated after the conversion of each channel. This allows the transfer of the

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Analog-to-digital converter (ADC4)
converted data from the ADC_DR register to the destination location selected by the
software.

Note:

The DMAEN bit in the ADC_CFGR1 register must be set after the ADC calibration phase.
Despite this, if an overrun occurs (OVR = 1) because the DMA did not serve the DMA
transfer request in time, the ADC stops generating DMA requests and the data
corresponding to the new conversion is not transferred by the DMA. Which means that all
the data transferred to the RAM can be considered as valid.
Depending on the configuration of OVRMOD bit, the data is either preserved or overwritten
(refer to ADC overrun (OVR, OVRMOD)).
The DMA transfer requests are blocked until the software clears the OVR bit.
Two different DMA modes are proposed depending on the application use and are
configured with bit DMACFG in the ADC_CFGR1 register:
•

DMA one-shot mode (DMACFG = 0).
This mode must be selected when the DMA is programmed to transfer a fixed number
of data words.

•

DMA circular mode (DMACFG = 1)
This mode must be selected when programming the DMA in circular mode or double
buffer mode.

DMA one-shot mode (DMACFG = 0)
In this mode, the ADC generates a DMA transfer request each time a new conversion data
word is available and stops generating DMA requests once the DMA has reached the last
DMA transfer (when a transfer complete interrupt occurs - refer to DMA section), even if a
conversion has been started again.
When the DMA transfer is complete (all the transfers configured in the DMA controller have
been done):
•

The content of the ADC data register is frozen.

•

Any ongoing conversion is aborted and its partial result discarded

•

No new DMA request is issued to the DMA controller. This avoids generating an
overrun error if there are still conversions which are started.

•

The scan sequence is stopped and reset

•

The DMA is stopped

DMA circular mode (DMACFG = 1)
In this mode, the ADC generates a DMA transfer request each time a new conversion data
word is available in the data register, even if the DMA has reached the last DMA transfer.
This allows the DMA configuration in circular mode in order to handle a continuous analog
input data stream.

RM0456 Rev 6

<!-- pagebreak -->

1443

Analog-to-digital converter (ADC4)

34.4.24

RM0456

Low-power features
Wait conversion mode (WAIT)
Wait conversion mode can be used to simplify the software as well as optimizing the
performance of applications clocked at low frequency where there might be a risk of ADC
overrun occurring.
When the WAIT bit is set to 1 in the ADC_CFGR1 register, a new conversion can start only
if the previous data has been treated, once the ADC_DR register has been read or if the
EOC bit has been cleared.
This is a way to automatically adapt the speed of the ADC to the speed of the system that
reads the data.

Note:

Any hardware triggers which occur while a conversion is ongoing or during the wait time
preceding the read access are ignored.
Figure 293. Wait conversion mode (continuous mode, software trigger)

ADSTART
EOC
EOS
ADSTP
ADC_DR Read access
ADC state

RDY

ADC_DR

by S/W

CH1

CH2

DLY

DLY CH3
D2

D1

DLY
D3

CH1

DLY

STOP

RDY

D1

by H/W

MSv30344V2

1. EXTEN = 00, CONT = 1.
2. CHSEL = 0x3, SCANDIR = 0, WAIT = 1, AUTOFF = 0.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Analog-to-digital converter (ADC4)

ADC power-saving modes
The ADC embeds two power-saving modes, the auto-off and the autonomous modes.
Auto-off mode (AUTOFF)
The auto-off mode is enabled by setting the AUTOFF bit to 1 in the ADC_PWRR register.
Below the auto-off mode operating sequence:
1.

When AUTOFF is set to 1, the ADC is always powered off when no conversion is
ongoing.

2.

It then automatically wakes up when a conversion is triggered by software or by
hardware, and a startup time is inserted between the trigger event and the ADC
sampling time.

3.

The ADC is then automatically disabled once the conversion or sequence of
conversions is complete.

4.

When consecutive hardware or software triggers occur, the ADC is automatically
enabled and the conversion is processed.

Refer to Figure 294 for a description of auto-off mode state diagram.
The auto-off mode dramatically reduces power consumption in applications requiring a
limited number of conversions or conversion requests far between enough (for example with
a low-frequency hardware trigger) to justify the extra power and time used for switching the
ADC on and off.
Auto-off mode can be combined with wait mode (WAIT = 1) for applications clocked at low
frequency. This combination can achieve significant power saving if the ADC is
automatically powered off during the wait phase and restarted as soon as the ADC_DR
register is read by the application (see Figure 295: ADC behavior with WAIT = 0 and
AUTOFF = 1 and Figure 296: ADC behavior with WAIT = 1 and AUTOFF = 1).
The auto-off mode is compatible with the low-power background autonomous mode
(LPBAM).
Note:

Refer to the Section Reset and clock control (RCC) for the description of how to manage
the dedicated internal oscillators. The ADC interface can automatically switch on/off these
internal oscillators to save power.

RM0456 Rev 6

<!-- pagebreak -->

1443

Analog-to-digital converter (ADC4)

RM0456

Figure 294. Auto-off mode state diagram

Consecutive
SW or HW
trigger

Voltage
regulator ON
ADC disabled

ADVREGEN = 1
and LDORDY = 1

ADEN = 1

SCAN
conversion

1st SW or
HW
trigger

Calibration
ADC
enable

Conversion

ADDIS = 1

ADC in DeepPower-Down

ADVREGEN=0

EOS and
AUTOFF = 1

MSv62486V2

Figure 295. ADC behavior with WAIT = 0 and AUTOFF = 1

TRGx

EOC
EOS
ADC_DR Read
access
ADC state

RDY Startup

ADC_DR
by S/W

CH1

CH2

CH3

CH4

D1

D2

D3

OFF

Startup

D4

by H/W

triggered
MSv30345V2

1. EXTSEL = TRGx, EXTEN = 01 (rising edge), CONT = x, ADSTART = 1, CHSEL = 0xF, SCANDIR = 0, WAIT = 0,
AUTOFF = 1.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Analog-to-digital converter (ADC4)
Figure 296. ADC behavior with WAIT = 1 and AUTOFF = 1

TRGx
EOC

ADC state

RDY

Startup

CH1

DLY

DLY

OFF

Startup CH2

D1

ADC_DR

by S/W

DLY

DLY

Startup

CH3

D2

OFF

Startup

CH1

D3

OFF

ADC_DR Read
access

OFF

EOS

CH2

D4

by H/W

triggered

MSv30346V2

1. EXTSEL = TRGx, EXTEN = 01 (rising edge), CONT = x, ADSTART = 1, CHSEL = 0xF, SCANDIR = 0, WAIT = 1,
AUTOFF = 1.

Autonomous mode (AUTOFF, DPD)
The autonomous mode is enabled by setting both AUTOFF and DPD bits to 1 in
ADC_PWRR register. In addition, the autonomous mode must be enabled in the RCC.
Below the autonomous mode operating sequence:
1.

When AUTOFF and DPD are both set to 1, the ADC is powered off when no conversion
is ongoing.

2.

Upon hardware trigger reception, the ADC requests the adc_ker_ck and adc_hclk
clocks to the RCC, the ADC voltage regulator is enabled, the calibration factor is
loaded, the ADC is enabled and the conversion starts.

3.

Once the ADC conversion is complete, the ADC can either generate an AWDx interrupt
or a DMA request, depending on peripheral configuration:
–

When DMA mode is enabled, the ADC generates a DMA request to transfer data
to memory or to another peripherals.

–

When an analog watchdog is enabled, ADC data do not need to be transferred.
The analog watchdog compares the data to the threshold value and generates an
AWDx interrupt to wake up the device if the data is under or over the programmed
threshold.

4.

When the ADC conversion/sequence or conversion is complete, the ADC and the ADC
voltage regulator are automatically disabled as well as VREFINT buffer and the
temperature sensor, and further clock requests are deasserted. This allows the
minimization of current consumption.

5.

When consecutive hardware triggers occur, the ADC is automatically enabled and the
conversion is processed.

Refer to Figure 297 for a description of autonomous mode state diagram.
The autonomous mode enables the ADC peripheral to operate when the device is in Stop
mode. However it can also be used in Run or Sleep mode.
RM0456 Rev 6

<!-- pagebreak -->

1443

Analog-to-digital converter (ADC4)

RM0456

It is compatible with the low-power background autonomous mode (LPBAM).
Figure 297. Autonomous mode state diagram

Consecutive
HW trigger

ADC in DeepPower-Down

Voltage regulator
ON
Wait LDORDY
ADC enabled
Wait ADRDY

1st HW
trigger

ADDIS
Wait ADEN = 0
Voltage regulator
disabled

SCAN
conversion
Conversion
(DMA mode or
analog
watchdog)

EOS
HW event
Automatic state change

MSv62487V2

34.4.25

Analog window watchdog
The three AWD analog watchdogs monitor whether some channels remain within a
configured voltage range (window).

Description of analog watchdog 1
AWD1 analog watchdog is enabled by setting the AWD1EN bit in the ADC_CFGR1 register.
It is used to monitor that either one selected channel or all enabled channels (see Table 335:
Analog watchdog 1 channel selection) remain within a configured voltage range (window) as
shown in Figure 298.
The AWD1 analog watchdog status bit is set if the analog voltage converted by the ADC is
below a lower threshold or above a higher threshold. These thresholds are programmed in
HT1[11:0] and LT1[11:0] bits of ADC_AWD1TR register. An interrupt can be enabled by
setting the AWD1IE bit in the ADC_IER register.
The AWD1 flag is cleared by software by programing it to 1.
When converting data with a resolution of less than 12-bit (according to bits DRES[1:0]), the
LSB of the programmed thresholds must be kept cleared because the internal comparison
is always performed on the full 12-bit raw converted data (left aligned).
Table 334 describes how the comparison is performed for all the possible resolutions.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Analog-to-digital converter (ADC4)
Table 334. Analog watchdog comparison

Resolution

Analog Watchdog comparison between:

bits
RES[1:0]

Raw converted
data, left aligned(1)

Thresholds

00: 12-bit

DATA[11:0]

LTx[11:0] and HTx[11:0] -

01: 10-bit

DATA[11:2],00

LTx[11:0] and HTx[11:0] The user must configure LTx[1:0] and HTx[1:0] to “00”

10: 8-bit

DATA[11:4],0000

LTx[11:0] and HTx[11:0]

The user must configure LTx[3:0] and HTx[3:0] to
“0000”

11: 6-bit

DATA[11:6],000000 LTx[11:0] and HTx[11:0]

The user must configure LTx[5:0] and HTx[5:0] to
“000000”

Comments

1. The watchdog comparison is performed on the raw converted data before any alignment calculation.

Table 335 shows how to configure the AWD1SGL and AWD1EN bits in the ADC_CFGR1
register to enable the analog watchdog on one or more channels.
Figure 298. Analog watchdog guarded area
Analog voltage
Higher threshold

HTx
Guarded area

Lower threshold

LTx
MS45396V1

Table 335. Analog watchdog 1 channel selection
Channels guarded by the analog watchdog
None
All channels
(1)

Single

channel

AWD1SGL bit

AWD1EN bit

x

0

0

1

1

1

1. Selected by the AWD1CH[4:0] bits

Description of analog watchdog 2 and 3
The second and third analog watchdogs are more flexible and can guard several selected
channels by programming the AWDxCHy in ADC_AWDxCR (x = 2, 3).
The corresponding watchdog is enabled when any AWDxCHy bit (x = 2,3) is set in
ADC_AWDxCR register.
When converting data with a resolution of less than 12 bits (configured through DRES[1:0]
bits), the LSB of the programmed thresholds must be kept cleared because the internal
comparison is always performed on the full 12-bit raw converted data (left aligned).
Table 334 describes how the comparison is performed for all the possible resolutions.
The AWD2/3 analog watchdog status bit is set if the analog voltage converted by the ADC is
below a low threshold or above a high threshold. These thresholds are programmed in
RM0456 Rev 6

<!-- pagebreak -->

1443

Analog-to-digital converter (ADC4)

RM0456

HTx[11:0] and LTx[11:0] of ADC_AWDxTR registers (x = 2 or 3). An interrupt can be
enabled by setting the AWDxIE bit in the ADC_IER register.
The AWD2 and AWD3 flags are cleared by software by programming them to 1.

ADC_AWDx_OUT signal output generation
Each analog watchdog is associated to an internal hardware signal, ADC_AWDx_OUT (x
being the watchdog number) that is directly connected to the ETR input (external trigger) of
some on-chip timers (refer to the timers section for details on how to select the
ADC_AWDx_OUT signal as ETR).
ADC_AWDx_OUT is activated when the associated analog watchdog is enabled:
•

ADC_AWDx_OUT is set when a guarded conversion is outside the programmed
thresholds.

•

ADC_AWDx_OUT is reset after the end of the next guarded conversion which is inside
the programmed thresholds. It remains at 1 if the next guarded conversions are still
outside the programmed thresholds.

•

ADC_AWDx_OUT is also reset when disabling the ADC (when setting ADDIS to 1).
Note that stopping conversions (ADSTP set to 1), might clear the ADC_AWDx_OUT
state.

•

ADC_AWDx_OUT state does not change when the ADC converts the none-guarded
channel (see Figure 301)

AWDx flag is set by hardware and reset by software: AWDx flag has no influence on the
generation of ADC_AWDx_OUT (as an example, ADC_AWDx_OUT can toggle while AWDx
flag remains at 1 if the software has not cleared the flag).
The ADC_AWDx_OUT signal is generated by the ADC_CLK domain. This signal can be
generated even the bus clock is stopped.
The AWD comparison is performed at the end of each ADC conversion. The
ADC_AWDx_OUT rising edge and falling edge occurs two ADC_CLK clock cycles after the
comparison.
As ADC_AWDx_OUT is generated by the ADC_CLK domain and AWD flag is generated by
the bus clock domain, the rising edges of these signals are not synchronized.
Figure 299. ADC_AWDx_OUT signal generation

ADC STATE

RDY

Conversion1

Conversion2

Conversion3

Conversion4

Conversion5

Conversion6

Conversion7

inside

outside

inside

outside

outside

outside

inside

Cleared
by SW

Cleared
by SW

Cleared
by SW

EOC FLAG
Cleared
by SW

AWDx FLAG

ADC_AWDx_OUT

- Converted channels: 1,2,3,4,5,6,7
- Guarded converted channels: 1,2,3,4,5,6,7
MSv45362V1

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Analog-to-digital converter (ADC4)

Figure 300. ADC_AWDx_OUT signal generation (AWDx flag not cleared by software)

ADC STATE RDY

Conversion1

Conversion2

Conversion3

inside

outside

inside

Conversion4

Conversion5

Conversion6

Conversion7

outside

outside

outside

inside

EOC FLAG
not cleared by SW
AWDx FLAG

ADC_AWDx_OUT

- Converted channels: 1,2,3,4,5,6,7
- Guarded converted channels: 1,2,3,4,5,6,7
MSv45363V1

Figure 301. ADC_AWDx_OUT signal generation (on a single channel)

ADC STATE

Conversion1

Conversion2

outside

Conversion1

Conversion2

inside

Conversion1

Conversion2

outside

Conversion1

Conversion2

outside

EOC FLAG

EOS FLAG
AWDx FLAG

Cleared
by SW

Cleared
by SW

ADCy_AWDx_OUT
- Converted channels: 1 and 2
- Only channel 1 is guarded
MSv45364V1

Analog watchdog threshold control
LTx[11:0] and HTx[11:0] can be changed during an analog-to-digital conversion (that is
between the start of the conversion and the end of conversion of the ADC internal state). If
HTx and LTx bits are programmed during the ADC guarded channel conversion, the
watchdog function is masked for this conversion. This mask is cleared when starting a new
conversion, and the resulting new AWD threshold is applied starting the next ADC
conversion result. AWD comparison is performed at each end of conversion. If the current
ADC data are out of the new threshold interval, this does not generated any interrupt or an
ADC_AWDx_OUT signal. The Interrupt and the ADC_AWDx_OUT generation only occurs
at the end of the ADC conversion that started after the threshold update. If
ADC_AWDx_OUT is already asserted, programming the new threshold does not deassert
the ADC_AWDx_OUT signal.

RM0456 Rev 6

<!-- pagebreak -->

1443

Analog-to-digital converter (ADC4)

RM0456

Figure 302. Analog watchdog threshold update

ADC state

Conversion

Conversion

Conversion

Conversion

Threshold updated
LTx, HTx

XXXX

XXXY

XXXZ
Masked

Active

Comparison

Active
MSv45365V1

34.4.26

Oversampler
The oversampling unit performs data preprocessing to offload the CPU. It can handle
multiple conversions and average them into a single data with increased data width, up to
16-bit.
It provides a result with the following form, where N and M can be adjusted:

1
Result = ---- ×
M

n = N–1

∑ Conversion ( tn )
n=0

It allows the following functions to be performed by hardware: averaging, data rate
reduction, SNR improvement, basic filtering.
The oversampling ratio N is defined using the OVSR[2:0] bits in the ADC_CFGR2 register. It
can range from 2x to 256x. The division coefficient M consists of a right bit shift up to 8 bits.
It is configured through the OVSS[3:0] bits in the ADC_CFGR2 register.
The summation unit can yield a result up to 20 bits (256 x 12-bit), which is first shifted right.
The lower bits of the result are then truncated, keeping only the 16 least significant bits
rounded to the nearest value using the least significant bits left apart by the shifting, before
being finally transferred into the ADC_DR data register.
Note:

If the intermediate result after the shifting exceeds 16 bits, the upper bits of the result are
simply truncated.
Figure 303. 20-bit to 16-bit result truncation
19

15

11

7

3

0

Raw 20-bit data

Shifting
15

0

Truncation
and rounding
MS31928V2

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Analog-to-digital converter (ADC4)
The Figure 304 gives a numerical example of the processing, from a raw 20-bit
accumulated data to the final 16-bit result.
Figure 304. Numerical example with 5-bits shift and rounding
19
Raw 20-bit data:

15
3

11

7

3

B

7

D

7

1

D

B

F

15
Final result after 5-bits shift
and rounding to nearest

0

MS31929V1

The Table 336 below gives the data format for the various N and M combination, for a raw
conversion data equal to 0xFFF.
Table 336. Maximum output results vs N and M. Grayed values indicates truncation
1-bit
No-shift
Oversa
shift
Max
mpling
OVSS =
OVSS
=
Raw
data
ratio
0000
0001

2-bit
shift

3-bit
shift

4-bit
shift

5-bit
shift

6-bit
shift

7-bit
shift

8-bit
shift

OVSS =
0010

OVSS =
0011

OVSS =
0100

OVSS =
0101

OVSS =
0110

OVSS = OVSS =
0111
1000

2x

0x1FFE

0x1FFE

0x0FFF

0x0800

0x0400

0x0200

0x0100

0x0080

0x0040

0x0020

4x

0x3FFC

0x3FFC

0x1FFE

0x0FFF

0x0800

0x0400

0x0200

0x0100

0x0080

0x0040

8x

0x7FF8

0x7FF8

0x3FFC

0x1FFE

0x0FFF

0x0800

0x0400

0x0200

0x0100

0x0080

16x

0xFFF0

0xFFF0

0x7FF8

0x3FFC

0x1FFE

0x0FFF

0x0800

0x0400

0x0200

0x0100

32x

0x1FFE0

0xFFE0

0xFFF0

0x7FF8

0x3FFC

0x1FFE

0x0FFF

0x0800

0x0400

0x0200

64x

0x3FFC0

0xFFC0

0xFFE0

0xFFF0

0x7FF8

0x3FFC

0x1FFE

0x0FFF

0x0800

0x0400

128x

0x7FF80

0xFF80

0xFFC0

0xFFE0

0xFFF0

0x7FF8

0x3FFC

0x1FFE

0x0FFF

0x0800

256x

0xFFF00

0xFF00

0xFF80

0xFFC0

0xFFE0

0xFFF0

0x7FF8

0x3FFC

0x1FFE

0x0FFF

The conversion timings in oversampler mode do not change compared to standard
conversion mode: the sample time is maintained equal during the whole oversampling
sequence. New data are provided every N conversion, with an equivalent delay equal to
N x tCONV = N x (tSMPL + tSAR). The flags features are raised as following:
•

the end of the sampling phase (EOSMP) is set after each sampling phase

•

the end of conversion (EOC) occurs once every N conversions, when the oversampled
result is available

•

the end of sequence (EOCSEQ) occurs once the sequence of oversampled data is
completed (i.e. after N x sequence length conversions total)

RM0456 Rev 6

<!-- pagebreak -->

1443

Analog-to-digital converter (ADC4)

RM0456

ADC operating modes supported when oversampling
In oversampling mode, most of the ADC operating modes are available:

Note:

•

Single or continuous mode conversions, forward or backward scanned sequences and
up to 8 channels programmed sequence

•

ADC conversions start either by software or with triggers

•

ADC stop during a conversion (abort)

•

Data read via CPU or DMA with overrun detection

•

Low-power modes (WAIT, AUTOFF)

•

Programmable resolution: in this case, the reduced conversion values (as per RES[1:0]
bits in ADC_CFGR1 register) are accumulated, truncated, rounded and shifted in the
same way as 12-bit conversions are

The alignment mode is not available when working with oversampled data. The ALIGN bit in
ADC_CFGR1 is ignored and the data are always provided right-aligned.

Analog watchdog
The analog watchdog functionality is available, with the following differences:

Note:

•

the RES[1:0] bits are ignored, comparison is always done on using the full 12-bits
values HTx[11:0] and LTx[11:0]

•

the comparison is performed on the most significant 12 bits of the 16 bits oversampled
results ADC_DR[15:4]

Care must be taken when using high shifting values. This reduces the comparison range.
For instance, if the oversampled result is shifted by 4 bits thus yielding a 12-bit data rightaligned, the effective analog watchdog comparison can only be performed on 8 bits. The
comparison is done between ADC_DR[11:4] and HTx[7:0] / LTx[[7:0], and HTx[11:8] /
LTx[11:8] must be kept reset.

Triggered mode
The averager can also be used for basic filtering purposes. Although not a very efficient filter
(slow roll-off and limited stop band attenuation), it can be used as a notch filter to reject
constant parasitic frequencies (typically coming from the mains or from a switched mode
power supply). For this purpose, a specific discontinuous mode can be enabled with TOVS
bit in ADC_CFGR2, to be able to have an oversampling frequency defined by a user and
independent from the conversion time itself.
Figure 305 below shows how conversions are started in response to triggers in
discontinuous mode.
If the TOVS bit is set, the content of the DISCEN bit is ignored and considered as 1.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Analog-to-digital converter (ADC4)
Figure 305. Triggered oversampling mode (TOVS bit = 1)
Trigger
CONT = 0
(DISCEN = 1)*
TOVS = 0

Trigger

Ch(N) Ch(N) Ch(N) Ch(N)
0
1
2
3

Ch(N) Ch(N) Ch(N) Ch(N)
0
1
2
3

EOC flag set
Trigger Trigger Trigger
CONT = 0
(DISCEN = 1)*
TOVS = 1

Ch(N)
0

Ch(N)
1

Ch(N)
2

Trigger Trigger Trigger Trigger
Ch(N)
3

Ch(N)
0

Ch(N)
1

Ch(N)
2

EOC flag set
(DISCEN = 1)*: DISCEN bit is forced to 1 by software when TOVS bit is set
MS33700V1

34.4.27

Temperature sensor and internal reference voltage
The temperature sensor can be used to measure the junction temperature (TJ) of the
device. The temperature sensor is internally connected an ADC internal input channel which
is used to convert the sensor’s output voltage to a digital value. The sampling time for the
temperature sensor analog pin must be greater than the minimum TS_temp value specified in
the datasheet. When not in use, the sensor can be put in Power-down mode.
The internal voltage reference (VREFINT) provides a stable (bandgap) voltage output for the
ADC and the comparators.
Refer to Table ADC interconnection in Section 34.4.2: ADC pins and internal signals for
details on the ADC internal input channel to which the above voltages are connected.
Figure 306 shows the block diagram of connections between the temperature sensor, the
internal voltage reference and the ADC.
The VSENSESEL bit must be set to enable the conversion of VSENSE while VREFEN bit
must be set to enable the conversion of VREFINT.
When the ADC operates in autonomous mode, these signals are controlled automatically to
reduce power consumption (VSENSESEL and VREFEN must be set to measure the voltage
in autonomous mode).
The temperature sensor output voltage linearly changes with the temperature. The offset of
this line varies from chip to chip due to process variation (up to 45 °C from one chip to
another).
The uncalibrated internal temperature sensor is more suited for applications that detect
temperature variations instead of absolute temperatures. To improve the accuracy of the
temperature sensor measurement, calibration values are stored in system memory for each
device by STMicroelectronics during production.
During the manufacturing process, the calibration data of the temperature sensor and the
internal voltage reference are stored in the system memory area. The user application can
then read them and use them to improve the accuracy of the temperature sensor or the
internal reference. Refer to the datasheet for additional information.

RM0456 Rev 6

<!-- pagebreak -->

1443

Analog-to-digital converter (ADC4)

RM0456

Main features
•

Linearity: ±2 °C max., precision depending on calibration
Figure 306. Temperature sensor and VREFINT channel block diagram
VREFEN control bit
VREFINT
+
-

VIN[0]

converted data
VSENSESEL
control bit
Temperature
sensor

+
-

VSENSE

Address/data bus

Internal power
block

VIN[13]

MSv62488V5

Reading the temperature
1.

Select the input channel connected to VSENSE (refer to Table ADC interconnection in
Section 34.4.2: ADC pins and internal signals).

2.

Select an appropriate sampling time specified in the device datasheet (TS_temp).

3.

Set the VSENSESEL bit in the ADC_CCR register to wake up the temperature sensor
from power-down mode and wait for its stabilization time (tSTART).

4.

Start the ADC conversion by setting the ADSTART bit in the ADC_CR register (or by
external trigger)

5.

Read the resulting VSENSE data in the ADC_DR register

6.

Calculate the temperature using the following formula

TS_CAL2_TEMP – TS_CAL1_TEMP
Temperature ( in °C ) = --------------------------------------------------------------------------------------------------- × ( TS_DATA – TS_CAL1 ) + TS_CAL1_TEMP
TS_CAL2 – TS_CAL1

Where:
–

TS_CAL2 is the temperature sensor calibration value acquired at
TS_CAL2_TEMP.

–

TS_CAL1 is the temperature sensor calibration value acquired at
TS_CAL1_TEMP.

–

TS_DATA is the actual temperature sensor output value converted by the ADC.

Refer to Section 34.3: ADC implementation for more information on TS_CAL1 and
TS_CAL2 calibration points.
Note:

<!-- pagebreak -->

The sensor has a startup time after waking up from power-down mode before it can output
VSENSE at the correct level. The ADC also has a startup time after power-on, so to minimize
the delay, the ADEN and VSENSESEL bits must be set at the same time.

RM0456 Rev 6

RM0456

Analog-to-digital converter (ADC4)

Calculating the actual VREF+ voltage using the internal reference voltage
The VDDA power supply voltage applied to the microcontroller may be subject to variation or
not precisely known. The embedded internal voltage reference (VREFINT) and its
calibration data acquired by the ADC during the manufacturing process at VREF+ = 3.0 V
can be used to evaluate the actual VREF+ voltage level, if VREF+ pin is connected to a
variable VDDA power supply.
The following formula gives the actual VREF+ voltage supplying the device:
V REF+ = 3.0 V × VREFINT_CAL ⁄ VREFINT_DATA

Where:
•

VREFINT_CAL is the VREFINT calibration value

•

VREFINT_DATA is the actual VREFINT output value converted by ADC

Converting a supply-relative ADC measurement to an absolute voltage value
The ADC is designed to deliver a digital value corresponding to the ratio between the
voltage reference VREF+ and the voltage applied on the converted channel. For most
application use cases, it is necessary to convert this ratio into a voltage independent from
VREF+. For applications where VREF+ is known and ADC converted values are right-aligned,
the following formula can be used to calculate this absolute value:
V REF+
V CHANNELx = ------------------------------------- × ADC_DATA x
FULL_SCALE

For applications where VREF+ value is not known, the internal voltage reference and VREF+
can be replaced by the expression provided in Section : Calculating the actual VREF+
voltage using the internal reference voltage, resulting in the following formula:
3.0 V × VREFINT_CAL × ADC_DATA
V CHANNELx = -------------------------------------------------------------------------------------------------------x
VREFINT_DATA × FULL_SCALE

Where:
•

VREFINT_CAL is the VREFINT calibration value (refer to Section 34.3: ADC
implementation for the value of VREFINT_CAL).

•

ADC_DATAx is the value measured by the ADC on channelx (right-aligned).

•

VREFINT_DATA is the actual VREFINT output value converted by the ADC.

•

FULL_SCALE is the maximum digital value of the ADC output. For example with 12-bit
resolution, it is 212 - 1 = 4095 or with 8-bit resolution, 28 - 1 = 255.

Note:

If ADC measurements are done using an output format other than 12 bit right-aligned, all the
parameters must first be converted to a compatible format before the calculation is done.

34.4.28

Battery voltage monitoring
The VBATEN bit in the ADC_CCR register allows the application to measure the battery
voltage on the VBAT pin. As the VBAT voltage may be higher than VDDA, to ensure the
correct operation of the ADC, the VBAT pin is internally connected to a bridge divider by 4.
This bridge is automatically enabled when VBATEN is set, to connect VBAT/4 to the
corresponding ADC input channel (refer to Table ADC interconnection in Section 34.4.2:
ADC pins and internal signals). As a consequence, the converted digital value is VBAT/4. To

RM0456 Rev 6

<!-- pagebreak -->

