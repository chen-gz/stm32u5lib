RM0456 Rev 6

RM0456

Analog-to-digital converter (ADC12)
The ADC also notifies the end of Sampling phase by setting the status bit EOSMP (for
regular conversions only). EOSMP flag is cleared by software by writing 1 to it. An interrupt
can be generated if bit EOSMPIE is set.

33.4.24

End of conversion sequence (EOS, JEOS)
The ADC notifies the application for each end of regular sequence (EOS) and for each end
of injected sequence (JEOS) event.
The ADC sets the EOS flag as soon as the last data of the regular conversion sequence is
available in the ADC_DR register. An interrupt can be generated if bit EOSIE is set. EOS
flag is cleared by the software either by writing 1 to it.
The ADC sets the JEOS flag as soon as the last data of the injected conversion sequence is
complete. An interrupt can be generated if bit JEOSIE is set. JEOS flag is cleared by the
software either by writing 1 to it.

33.4.25

Timing diagrams example (single/continuous modes,
hardware/software triggers)
Figure 233. Single conversions of a sequence, software trigger

ADSTART(1)
EOC
EOS
ADC state(2)

RDY

CH1

ADC_DR
by SW

CH9

CH10

CH17

D1

D9

D10

RDY

by HW

D17

CH1

CH9

CH10

CH17

RDY

D1

D9

D10

D17

Indicative timings
MS30549V1

1. EXTEN[1:0] = 0x0, CONT = 0
2. Channels selected = 1,9, 10, 17; AUTDLY = 0.

RM0456 Rev 6

<!-- pagebreak -->

1377

Analog-to-digital converter (ADC12)

RM0456

Figure 234. Continuous conversion of a sequence, software trigger

ADCSTART(1)
EOC
EOS
ADSTP
ADC state(2) READY

CH1

ADC_DR

by SW

CH9

CH10

CH17

CH1

CH9

D1

D9

D10

D17

D1

CH10

STP

CH1

READY

CH9

D9

D1
Indicative timings

by HW

MS30550V1

1. EXTEN[1:0] = 0x0, CONT = 1
2. Channels selected = 1,9, 10, 17; AUTDLY = 0.

Figure 235. Single conversions of a sequence, hardware trigger
ADSTART
EOC
EOS
TRGX(1)
ADC state(2)

RDY

CH1

ADC_DR
by s/w

by h/w

CH2

CH3

CH4

READY CH1

CH2

CH3

CH4

RDY

D1

D2

D3

D4

D1

D2

D3

D4

triggered

ignored

Indicative timings
MS31013V2

1. TRGx (over-frequency) is selected as trigger source, EXTEN[1:0] = 01, CONT = 0
2. Channels selected = 1, 2, 3, 4; AUTDLY = 0.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Analog-to-digital converter (ADC12)
Figure 236. Continuous conversions of a sequence, hardware trigger

ADSTART
EOC
EOS
ADSTP
TRGx(1)
ADC(2)

RDY

ADC_DR
by s/w

CH1

CH2

CH3

CH4

CH1

CH2

CH3

CH4

CH1

D1

D2

D3

D4

D1

D2

D3

D4

by h/w

triggered

ignored

STOP

RDY

Not in scale timings
MS31014V2

1. TRGx is selected as trigger source, EXTEN[1:0] = 10, CONT = 1
2. Channels selected = 1, 2, 3, 4; AUTDLY = 0.

33.4.26

Low-frequency trigger mode (LFTRIG)
Once the ADC is enabled or when the last ADC conversion is complete, the ADC is ready to
start the next conversion. The ADC needs to be started at a defined time (Tidle = 150 μs),
otherwise the converted data can be corrupted due to transistor leakage.
If the user application requires a time higher than Tidle maximum value between one trigger
to another for single conversion mode, or between ADC enable and the first conversion,
then the ADC converter input multiplexer must be reinitialized. This operation is selected by
setting the LFTRIG bit of the ADC_CFGR2 register. It is recommended to always set
LFTRIG as there is no impact on timing and performance.

33.4.27

Data management
Data register, data alignment and offset (ADC_DR, ADC_JDRy, OFFSETy,
OFFSETy_CH, OVSS, LSHIFT, USAT, SSAT)
Data and alignment
At the end of each regular conversion channel (when the EOC event occurs), the result of
the converted data is stored into the ADC_DR data register which is 32 bits wide.
At the end of each injected conversion channel (when the JEOC event occurs), the result of
the converted data is stored into the corresponding ADC_JDRy data register which is 32 bits
wide.
The OVSS[3:0] and LSHIFT[3:0] bitfields in the ADC_CFGR2 register selects the alignment
of the data stored after conversion. By default, data are right-aligned. Refer to Figure 237,
Figure 238, Figure 239 and Figure 240 for examples of data alignment.

Note:

The data can be realigned in normal and in oversampling mode.

Offset
An offset y (y = 1,2,3,4) can be applied to a channel by programming a value different from
0 in the OFFSETy[23:0] bitfield of the ADC_OFRy register. The channel to which the offset
is applied is programmed into the bits OFFSETy_CH[4:0] of ADC_OFRy register. The offset
RM0456 Rev 6

<!-- pagebreak -->

1377

Analog-to-digital converter (ADC12)

RM0456

can be positive or negative depending on the value of the POSOFF bit. When POSOFF is
cleared, the converted value is subtracted by the user-defined offset written in
OFFSETy[23:0] bits. The result can be a negative value. The read data is consequently
signed and the SEXT bit represents the extended sign value.
The offset value must be lower than the maximum conversion value (for example, in 14-bit
mode, the maximum offset value is 0x3FFF).
The offset can be used to convert unsigned data to signed data (for example, in 14-bit
mode, the offset value is equal to 0x2000).
The offset correction is also supported in oversampling mode. For the oversampling mode,
offset is subtracted before OVSS right shift applied.
Table 314 describes how the comparison is performed for all the possible resolutions for
analog watchdog 1, 2, 3.
Table 314. Offset computation versus data resolution
Resolution
(bits
RES[1:0])

Subtraction/addition between
raw converted data and offset
Raw converted left
-aligned data

Result

Comments

Offset

00: 14-bit

DATA[13:0]

Signed or unsigned
OFFSET[13:0] 24-bit data, right
aligned to [13:0]

-

01: 12-bit

DATA[13:2],00

Signed or unsigned
OFFSET[13:2] 24-bit data, right
aligned to [11:0]

The user must configure
OFFSET[11:0] to
0b0000 0000 0000.

10: 10-bit

DATA[13:4],00
00

Signed or unsigned
OFFSET[13:4] 24-bit data, right
aligned to [9:0]

The user must configure
OFFSET[3:0] to 0b0000.

11: 8-bit

DATA[13:6],00
0000

Signed or unsigned
OFFSET[13:6] 24-bit data, right
aligned to [7:0]

The user must configure
OFFSET[5:0] to 0b000000.

Figure 237, Figure 238, Figure 239 and Figure 240 show alignments for signed and
unsigned data together with corresponding OVSS and LSHIFT values.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Analog-to-digital converter (ADC12)
Figure 237. Right alignment (offset disabled, unsigned value)

31

15 13

0

0000

14-bit data

D13..D0

31

15 11

12-bit data

0

0000

D11..D0

31

7

15

8-bit data

0000

23

31
14-bit data
OSR=1024

00

0
D7..D0

15

0

00

D23..D0

31

13

15

16-bit data
OSR=1024

OVSS = 0000

0

0000

D13..D0

OVSS = 1010
MSv62491V1

Figure 238. Right alignment (offset enabled, signed value)

31
14-bit data

15 13
SEXT

15

31
SEXT

0
D7..D0

SEXT

SEXT

7

0

15

8-bit data

31

0
D11..D0

15

31

14-bit data
OSR=1024

11

SEXT

8-bit data

23

Signed 32-bit or
16-bit format

D13..D0

31
12-bit data

0

SSAT = 1

D6..D0

15
D23..D0

Signed 8-bit
format

0
OVSS = 0000

MSv62492V1

RM0456 Rev 6

<!-- pagebreak -->

1377

Analog-to-digital converter (ADC12)

RM0456

Figure 239. Left alignment (offset disabled, unsigned value)
31

23

14-bit data

0000

31

3

15

0000

D11..D0

23

31

15

23

31

7

LSHIFT = 4

0
LSHIFT = 0

D7..D0

15

LSHIFT = 2

0
0

SEXT

8-bit data

14-bit data
OSR=1024

00

D13..D0

23

12-bit data

10

15

7

0

D23..D0

0

LSHIFT = 8

MSv62493V1

Figure 240. Left alignment (offset enabled, signed value)
31
14-bit data

S

D13..D0

0

31
14-bit data

D13..D0

31

15

8-bit data

SEXT

31
14-bit data
OSR=1024

S

23

LSHIFT = 1

Signed 16-bit
format

LSHIFT = 3

Signed 16-bit
format

LSHIFT = 7

Signed 16-bit
format

SSAT = 1

Signed 8-bit
format

LSHIFT = 7

Signed 32-bit
format

0
0

D7..D0

31

0

7

15
SEXT

Signed 32-bit
format

0

D11..D0

31
8-bit data

0

3

15
SEXT

LSHIFT = 17
0

15
SEXT

12-bit data

0

15

7

0
D6..D0

0

15
D23..D0

0

MSv62492V1

Management of signed and unsigned saturation format (SSAT, USAT)
The offset correction might results in the data width to be wider than the original data.
To limit the original width, the data saturation can be enabled through the SSAT and USAT
bits of the ADC_OFRy register.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Analog-to-digital converter (ADC12)
Unsigned 14-bit data can be extended to 15-bit signed data by using an offset value
different from 0x2000.
The original data width can be preserved by setting the SSAT bit to limit the data width to 14
bits.
Unsigned data can be saturated to the original data width by setting the USAT bit.
Table 315 shows the sign-extended data format corresponding to different resolutions.
Table 315. 14-bit data formats
SSAT

USAT

Format

Data range (offset = 0x2000)

0

0

Sign-extended 15-bit significant data:
SEXT[31:14]
DATA[13:0]

0x0000 1FFF - 0xFFFF E000

1

0

Sign-extended 14-bit significant data:
SEXT[31:13]
DATA[12:0]

0x1FFF - 0xE000

0

1

Unsigned saturation14-bit significant data:
DATA[13:0]

0x3FFF - 0x0000

1

1

Reserved

-

Table 316 provides numerical examples for three different offset values.
Table 316. Numerical examples for 16-bit format
Result

Result

Result

SSAT = 0

SSAT = 0

SSAT = 1

USAT = 0

USAT = 1

USAT = 0

0x0000 1FFF

1FFF

1FFF

0x0000 0000

0000

0000

0x0000

0xFFFF E000

E000

E000

0x3FFF

0x0000 1FDF

1FDF

1FDF

0xFFFF FFE0

0000

FFE0

0x0000

0xFFFF DFE0

0000

DFE0

0x3FFF

0x0000 201F

201F

1FFF

0x0000 0020

0020

0020

0x0000

0xFFFF E020

0000

E020

0x3FFF

0x0000 3FDF

3FDF

3FDF

0x0000 1FE0

1FE0

1FE0

0xFFFF FFE0

0000

FFE0

Raw conversion
result

Offset value

0x3FFF
0x2000

0x2000

0x2000

0x2000
0x0000

Caution:

0x2000

0x2020

0x1FE0

0x20

SSAT must not be used in conjunction with USAT. No hardware check is performed to
ensure that this recommendation is respected.

RM0456 Rev 6

<!-- pagebreak -->

1377

Analog-to-digital converter (ADC12)

RM0456

Gain compensation
When the GCOMP bit is set in the ADC_CFGR2 register, the gain compensation is activated
on all converted data. After each conversion, data is calculated using the following formula.
DATA = DATA ( adc result ) × ( GCOMPCOEFF ) ⁄ 4096
As GCOMPCOEFF can be programmed from 0 to 16383, the actual gain compensation
factor can range from 0 to 3.999756.
Before storing the resulting data in the RDATA or JDATAx registers, the LSB−1 value is
evaluated to round up the data and minimize the error.
The gain compensation is also effective for the oversampling. When the gain compensation
is used for the oversampling mode, the gain calculation is performed after the accumulation
and right-shift operations to minimize the power consumption (the gain calculation is done
only once instead of at each conversion). The internal multiplier width is 32 bits and the
input data width for the gain compensation must be less than 18 bits. When using
oversampling with injected and regular conversion mode the bit ADC_CFGR2.ROVSM bit
must be set to resume the pending conversion with the correct value.

ADC overrun (OVRMOD)
The overrun flag (OVR) notifies of that a buffer overrun event occurred when the regular
converted data has not been read (by the CPU or the DMA) before ADC_DR FIFO (eight
stages) is overflowed.
The OVR flag is set when a new conversion completes while the ADC_CR register FIFO
was full. An interrupt is generated if the OVRIE bit is set.
When an overrun event occurs, the ADC is still operating and can continue converting
unless the software decides to stop and reset the sequence by setting the ADSTP bit.
OVR flag is cleared by software by writing 1 to it.
Data can be configured to be preserved or overwritten when an overrun event occurs by
programming the OVRMOD control bit of the ADC_CFGR1 register:
•

OVRMOD = 0
The overrun event preserves the data register from being overwritten: the old data is
maintained up to ADC_DR FIFO depth (eight data) and the new conversion is
discarded and lost. If OVR remains at 1, further conversions occur but the result data
are also discarded.

•

OVRMOD = 1
The data register is overwritten with the last conversion result and the previous unread
data is lost. In this mode, ADC_DR FIFO is disabled. If OVR remains at 1, further
conversions operate normally and the ADC_DR register always contains the latest
converted data.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Analog-to-digital converter (ADC12)
Figure 241. Example of overrun (OVRMOD = 0)

ADSTART
EOC
OVR
ADSTP
TRGx
ADC state

RDY

CH1

CH2

CH3

CH4

D1

D2

D3

---

CH11

CH12

CH13

STOP

RDY

ADC_DR read
access
ADC_DR

D4
D5

D4
ADC_DR
(FIFO_DATA)

D10

D9
na

D10
by s/w

by h/w

triggered

D12
Indicative timings

MSv69549V1

Figure 242. Example of overrun (OVRMOD = 1)

ADSTART
EOC

OVR
ADSTP
TRGx
ADC state

RDY

CH1

CH2

CH3

CH4

D1

D2

D3

ADC_DR read access
ADC_DR
(OVRMOD =1)

by s/w

by h/w

CH5
CH6
Overun

triggered

D4

D5

CH7

STOP

RDY

D6

Indicative timings
MSv65301V1

Note:

There is no overrun detection on the injected channels since there is a dedicated data
register for each of the four injected channels.

RM0456 Rev 6

<!-- pagebreak -->

1377

Analog-to-digital converter (ADC12)

RM0456

Managing a sequence of conversions without using the DMA
If the conversions are slow enough, the conversion sequence can be handled by the
software. In this case, the software must use the EOC flag and its associated interrupt to
handle each data. Each time a conversion is complete, EOC is set and the ADC_DR
register can be read. OVRMOD must be configured to 0 to manage overrun events as an
error.

Managing conversions without using the DMA and without overrun
It may be useful to let the ADC convert one or more channels without reading the data each
time (for example if the device features an analog watchdog). In this case, the OVRMOD bit
must be configured to 1 and the OVR flag must be ignored by the software. An overrun
event does not prevent the ADC from continuing to convert and the ADC_DR register
always contains the latest conversion.

Managing conversions using the DMA
Since converted channel values are stored in a unique data register, it is useful to use DMA
to convert more than one channel. This avoids the loss of the data already stored in the
ADC_DR register.
When the DMA mode is enabled (DMNGT[1:0] = 01 or 11 in the ADC_CFGR register in
single ADC mode or DAMDF different from 00 in dual ADC mode), a DMA request is
generated after each conversion of a channel. This allows the transfer of the converted data
from the ADC_DR register to the destination location selected by the software.
Despite this, if an overrun occurs (OVR = 1) because the DMA cannot serve the DMA
transfer request in time, the ADC stops generating DMA requests and the data
corresponding to the new conversion is not transferred by the DMA. This means that all the
data transferred to the RAM can be considered as valid.
Depending on the configuration of the OVRMOD bit, the data is either preserved or
overwritten.
The DMA transfer requests are blocked until the software clears the OVR bit.
Two different DMA modes are proposed depending on the application. They can be
configured through the DMNGT[1:0] bitfield of the ADC_CFGR1 register and the DAMDF bit
of the ADC12_CCR register, in single ADC mode and in dual ADC mode, respectively:
•

DMA one shot mode (DMNGT[1:0] = 01).
This mode is suitable when the DMA is programmed to transfer a fixed number of data.

•

DMA circular mode (DMNGT[1:0] = 11)
This mode is suitable when programming the DMA in circular mode.

DMA one shot mode (DMNGT[1:0] = 01)
In this mode, the ADC generates a DMA transfer request each time a new conversion data
is available and stops generating DMA requests once the DMA has reached the last DMA
transfer (when a transfer complete interrupt occurs - refer to DMA section) even if a
conversion has been started again.
When the DMA transfer is complete (all the transfers configured in the DMA controller have
been done):

<!-- pagebreak -->

•

The content of the ADC data register is frozen.

•

Any ongoing conversion is aborted with partial result discarded.

RM0456 Rev 6

RM0456

Analog-to-digital converter (ADC12)
•

No new DMA request is issued to the DMA controller. This avoids generating an
overrun error if there are still conversions, which are started.

•

Scan sequence is stopped and reset.

•

The DMA is stopped.

DMA circular mode (DMNGT[1:0] = 11)
In this mode, the ADC generates a DMA transfer request each time a new conversion data
is available in the data register, even if the DMA has reached the last DMA transfer. This
allows configuring the DMA in circular mode to handle a continuous analog input data
stream.
DMA with FIFO
The output data register features an eight-stage FIFO. Two different DMA requests are
generated in parallel. When a data is available, an “SREQ single request” is generated.
When four data are available, a “BREQ burst request” is generated. DMA can be
programmed either single transfer mode or incremental burst mode (four beats), according
to this mode, correct request line is selected by the DMA. Refer to the DMA chapter for
further information.

33.4.28

Managing conversions using the MDF
The ADC conversion results can be transferred directly to the MDF.
In this case, the DMNGT[1:0] bits must be set to 10.
The ADC transfers the 16 least significant bits of the regular data register to the MDF
through adcx_dat[15:0] bus, which in turn resets the EOC flag once the transfer is effective.
The data format must be in 16-bit signed format:
ADC_DR[31:16] = don’t care
ADC_DR[15] = sign
ADC_DR[14:0] = data
Any value above 16-bit signed format is truncated.

33.4.29

Dynamic low-power features
Auto-delayed conversion mode (AUTDLY)
The ADC implements an auto-delayed conversion mode controlled by the AUTDLY
configuration bit. Auto-delayed conversions are useful to simplify the software as well as to
optimize performance of an application clocked at low frequency where there would be risk
of encountering an ADC overrun.
When AUTDLY = 1, a new conversion can start only if all the previous data of the same
group has been treated:
•

For a regular conversion: once the ADC_DR register has been read or if the EOC bit
has been cleared (see Figure 243).

•

For an injected conversion: when the JEOS bit has been cleared (see Figure 244).

The auto-delayed conversion mode enables to automatically adapt the speed of the ADC to
the speed of the system which reads the data.

RM0456 Rev 6

<!-- pagebreak -->

1377

Analog-to-digital converter (ADC12)

RM0456

The delay is inserted after each regular conversion (whatever DISCEN = 0 or 1) and after
each sequence of injected conversions (whatever JDISCEN = 0 or 1).
Note:

There is no delay inserted between each conversion of the injected sequence, except after
the last one.
During a conversion, a hardware trigger event (for the same group of conversions) occurring
during this delay is ignored.

Note:

This is not true for software triggers where it remains possible during this delay to set the
bits ADSTART or JADSTART to restart a conversion: it is up to the software to read the data
before launching a new conversion.
No delay is inserted between conversions of different groups (a regular conversion followed
by an injected conversion or conversely):
•

If an injected trigger occurs during the automatic delay of a regular conversion, the
injected conversion starts immediately (see Figure 244).

•

Once the injected sequence is complete, the ADC waits for the delay (if not ended) of
the previous regular conversion before launching a new regular conversion (see
Figure 246).

The behavior is slightly different in auto-injected mode (JAUTO = 1) where a new regular
conversion can start only when the automatic delay of the previous injected sequence of
conversion has ended (when JEOS has been cleared). This is to ensure that the software
can read all the data of a given sequence before starting a new sequence (see Figure 247).
To stop a conversion in continuous auto-injection mode combined with auto-delay mode
(JAUTO = 1, CONT = 1 and AUTDLY = 1), follow the following procedure:
1.

Wait until JEOS = 1 (no more conversions are restarted).

2.

Clear JEOS.

3.

Set ADSTP = 1.

4.

Read the regular data.

If this procedure is not respected, a new regular sequence can restart if JEOS is cleared
after ADSTP has been set.
In AUTDLY mode, a hardware regular trigger event is ignored if it occurs during an already
ongoing regular sequence or during the delay that follows the last regular conversion of the
sequence. It is however considered pending if it occurs after this delay, even if it occurs
during an injected sequence of the delay that follows it. The conversion then starts at the
end of the delay of the injected sequence.
In AUTDLY mode, a hardware injected trigger event is ignored if it occurs during an already
ongoing injected sequence or during the delay that follows the last injected conversion of
the sequence.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Analog-to-digital converter (ADC12)

Figure 243. AUTODLY = 1, regular conversion in continuous mode, software trigger

ADSTART (1)
EOC
EOS
ADSTP
ADC_DR read access
CH1

RDY

ADC state

DLY

ADC_DR

CH2
D1

by SW

DLY

CH3

DLY

D2

CH1

DLY

STOP

D3

RDY

D1

by HW
Indicative timings
MS31020V1

1. AUTDLY = 1.
2. Regular configuration: EXTEN[1:0] = 0x0 (software trigger), CONT = 1, CHANNELS = 1,2,3.
3. Injected configuration DISABLED.

Figure 244. AUTODLY = 1, regular hardware conversions interrupted by injected conversions
(DISCEN = 0; JDISCEN = 0)
Ignored

Regular
trigger
ADC state

RDY

CH1
regular

DLY

CH2

DLY (CH1)

Not ignored
(occurs during injected sequence)

CH1
CH6
DLY
CH2
DLY
CH3
regular
injected regular injected regular
DLY (CH1)
DLY (CH3)

DLY
CH5
regular
DLY (CH2)

EOC
EOS
ADC_DR
read access
D1

ADC_DR
Injected
trigger

D2

D3

D1

Ignored
DLY (inj)

JEOS
D5

ADC_JDR1

D6

ADC_JDR2
by s/w

by h/w

Indicative timings

MS31021V2

1. AUTDLY = 1.
2. Regular configuration: EXTEN[1:0] = 0x1 (hardware trigger), CONT = 0, DISCEN = 0, CHANNELS = 1, 2, 3.
3. Injected configuration: JEXTEN[1:0] = 0x1 (hardware Trigger), JDISCEN = 0, CHANNELS = 5,6.

RM0456 Rev 6

<!-- pagebreak -->

1377

Analog-to-digital converter (ADC12)

RM0456

Figure 245. AUTODLY = 1, regular hardware conversions interrupted by injected conversions
(DISCEN = 1, JDISCEN = 1)

Not ignored (occurs during
injected sequence)

Ignored
Regular trigger
ADC state

RDY

CH1

DLY

RDY

CH2

DLY RDY

CH5

RDY

CH6

CH3

DLY RDY

CH1

DLY

RDY CH2

regular
regular
injected regular
DLY (CH1)
DLY (CH3)

regular
injected
regular
DLY (CH1)
DLY (CH2)

EOC
EOS
ADC_DR
read access
ADC_DR

D2

D1
Ignored

Injected trigger

D3

D1

Ignored
DLY (inj)

JEOS
ADC_JDR1

D5
D6

ADC_JDR2
by SW

Indicative timings

by HW

MS31022V1

1. AUTDLY = 1.
2. Regular configuration: EXTEN[1:0] = 0x1 ( hardware trigger), CONT = 0, DISCEN = 1, DISCNUM = 1, CHANNELS = 1, 2,
3.
3. Injected configuration: JEXTEN[1:0] = 0x1 (hardware Trigger), JDISCEN = 1, CHANNELS = 5,6.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Analog-to-digital converter (ADC12)

Figure 246. AUTODLY = 1, regular continuous conversions interrupted by injected conversions
ADSTART(1)
ADC
state

RDY

CH1
regular

CH2
regular

DLY

DLY

CH6
CH5
injected injected

DLY CH1
CH3
regular
regular
DLY (CH3)

DLY

DLY (CH2)

DLY (CH1)
EOC
EOS
ADC_DR read
access
ADC_DR

D1

D2

D3
Ignored

Injected
trigger

DLY (inj)

JEOS
ADC_JDR1

D5
D6

ADC_JDR2
by s/w

by h/w

Indicative timings
MS31023V3

1. AUTDLY = 1.
2. Regular configuration: EXTEN[1:0] = 0x0 (software trigger), CONT = 1, DISCEN = 0, CHANNELS = 1, 2, 3.
3. Injected configuration: JEXTEN[1:0] = 0x1 (hardware trigger), JDISCEN = 0, CHANNELS = 5,6.

Figure 247. AUTODLY = 1 in auto- injected mode (JAUTO = 1)
ADSTART(1)
ADC state

No delay
RDY

CH1
regular

DLY (CH1)

CH2
CH6
DLY (inj)
CH5
CH3
regular
regular injected injected

DLY

CH1
regular

EOC
EOS
ADC_DR read access
ADC_DR

D1

D2

D3

JEOS
ADC_JDR1

D5
D6

ADC_JDR2
by s/w

by h/w

Indicative timings
MS31024V4

1. AUTDLY = 1.
2. Regular configuration: EXTEN[1:0] = 0x0 (software trigger), CONT = 1, DISCEN = 0, CHANNELS = 1, 2.
3. Injected configuration: JAUTO = 1, CHANNELS = 5,6.

RM0456 Rev 6

<!-- pagebreak -->

