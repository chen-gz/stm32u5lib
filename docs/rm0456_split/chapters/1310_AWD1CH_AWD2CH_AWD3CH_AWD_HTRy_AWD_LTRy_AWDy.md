1377

Analog-to-digital converter (ADC12)

33.4.30

RM0456

Analog window watchdog (AWD1EN, JAWD1EN, AWD1SGL,
AWD1CH, AWD2CH, AWD3CH, AWD_HTRy, AWD_LTRy, AWDy)
The three AWD analog watchdogs monitor whether some channels remain within a
configured voltage range (window).
Figure 248. Analog watchdog guarded area
Analog voltage
Higher threshold

HTR
Guarded area

Lower threshold

LTR
ai16048

AWDx flag and interrupt
An interrupt can be enabled for each of the three analog watchdogs by setting AWDyIE in
the ADC_IER register (y = 1, 2, 3).
AWDy (y = 1, 2, 3) flag is cleared by software by writing 1 to it.
The ADC conversion result is compared to the lower and higher thresholds before
alignment.
Description of analog watchdog 1
The AWD analog watchdog 1 is enabled by setting the AWD1EN bit in the ADC_CFGR1
register. This watchdog monitors whether either one selected channel or all enabled
channels remain within a configured voltage range (window).
Table 317 shows how the ADC_CFGR1 registers must be configured to enable the analog
watchdog on one or more channels.
Table 317. Analog watchdog channel selection
Channels guarded by the analog
watchdog

AWD1SGL bit

AWD1EN bit

JAWD1EN bit

None

x

0

0

All injected channels

0

0

1

All regular channels

0

1

0

All regular and injected channels

0

1

1

Single(1) injected channel

1

0

1

Single(1) regular channel

1

1

0

Single(1) regular or injected channel

1

1

1

1. Selected by the AWDyCH[4:0] bits. The channels must also be programmed to be converted in the
appropriate regular or injected sequence.

The AWD1 analog watchdog status bit is set if the analog voltage converted by the ADC is
below a lower threshold or above a higher threshold.
These thresholds are programmed in the HTR1[24:0] bits of the ADC_HTR1 register and
LTR1[24:0] of the ADC_LTR1 register for the analog watchdog 1.
The threshold can be up to 25-bits (14-bit resolution with oversampling, OSR=256).

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Analog-to-digital converter (ADC12)
When converting data with a resolution of less than 14 bits (according to bits RES[1:0]), the
LSBs of the programmed thresholds must be kept cleared, the internal comparison being
performed on the full 14-bit converted data (left aligned to the half-word boundary).
Table 318 describes how the comparison is performed for all the possible resolutions for
analog watchdog 1,2,3.
Table 318. Analog watchdog 1,2,3 comparison
Resolution
(bit
RES[1:0])

Analog watchdog comparison
between:
Comments
Raw converted
data, left aligned(1)

Thresholds

00: 14-bit

DATA[13:0]

LTR1[24:0] and
HTR1[24:0]

01: 12-bit

DATA[13:2],00

LTR1[24:0] and
HTR1[24:0]

User must configure LTR1[1:0] and
HTR1[1:0] to 00

10: 10-bit

DATA[13:4],0000

LTR1[24:0] and
HTR1[24:0]

User must configure LTR1[3:0] and
HTR1[3:0] to 0000

11: 8-bit

DATA[13:6],000000

LTR1[24:0] and
HTR1[24:0]

User must configure LTR1[5:0] and
HTR1[5:0] to 000000

-

1. The watchdog comparison is performed when the oversampling, the gain compensation and the offset
compensation are complete (the data that are compared can be either signed or unsigned).

Analog watchdog filter for watchdog 1
When the ADC is configured with only one input channel (selecting several channels in scan
mode not allowed), a valid ADC conversion data range can be configured through the
ADC_LTR1 and ADC_HTR1 register:
•

When converted data belong to the range defined in ADC_LTR1 and ADC_HTR1, a
DMA request is generated.

•

Otherwise, no DMA request is issued. RDATA register is updated at each conversion. If
data are out-of-range a number of times higher than the value specified in the
AWDFILT bit of ADC_HTR1, the AWDx flag is set and the corresponding interrupt is
issued.

Description of analog watchdog 2 and 3
The second and third analog watchdogs are more flexible and can guard several selected
channels by programming the corresponding bits in AWDCHy[19:0] (y=2,3).
The corresponding watchdog is enabled when any bit of AWDCHy[19:0] (y=2,3) is set.
The threshold can be up to 25 bits (14-bit resolution with oversampling, OSR = 1024, and
offset conversion in signed format) and are programmed with the ADC_HTR2, ADC_LTR2,
ADC_LTR3, and ADC_HTR3 registers.
When converting data with a resolution of less than 14 bits (according to bits RES[1:0]), the
LSBs of the programmed thresholds must be kept cleared, the internal comparison being
performed on the full 14-bit converted data (left aligned).

RM0456 Rev 6

<!-- pagebreak -->

1377

Analog-to-digital converter (ADC12)

RM0456

ADC_AWDy_OUT signal output generation
Each analog watchdog is associated to an internal hardware signal ADC_AWDy_OUT (y
being the watchdog number) which is directly connected to the ETR input (external trigger)
of some on-chip timers. Refer to the on-chip timers section to understand how to select the
ADC_AWDy_OUT signal as ETR.
ADC_AWDy_OUT is activated when the associated analog watchdog is enabled:

Note:

•

ADC_AWDy_OUT is set when a guarded conversion is outside the programmed
thresholds.

•

ADC_AWDy_OUT is reset after the end of the next guarded conversion which is inside
the programmed thresholds (it remains at 1 if the next guarded conversions are still
outside the programmed thresholds).

•

ADC_AWDy_OUT is also reset when disabling the ADC (when setting ADDIS = 1).
Note that stopping regular or injected conversions (setting ADSTP = 1 or JADSTP = 1)
has no influence on the generation of ADCy_AWDx_OUT.

AWDx flag is set by hardware and reset by software: AWDy flag has no influence on the
generation of ADC_AWDy_OUT (ex: ADCy_AWDy_OUT can toggle while AWDx flag
remains at 1 if the software did not clear the flag).
Figure 249. ADCy_AWDx_OUT signal generation (on all regular channels)

ADC
STATE

RDY

Conversion1

Conversion2

Conversion3

inside

outside

inside

Conversion4 Conversion5
outside

Conversion6

Conversion7

outside

outside

inside

cleared
by S/W

cleared
by S/W

EOC FLAG

AWDx FLAG

cleared
by S/W

cleared
by S/W

ADCy_AWDx_OUT

- Converting regular channels 1,2,3,4,5,6,7
- Regular channels 1,2,3,4,5,6,7 are all guarded
MS31025V1

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Analog-to-digital converter (ADC12)

Figure 250. ADC_AWDx_OUT signal generation (AWDx flag not cleared by software)
ADC
STATE

RDY

Conversion1

Conversion2

Conversion3

inside

outside

inside

Conversion4 Conversion5
outside

Conversion6

Conversion7

outside

inside

outside

EOC FLAG
not cleared by S/W
AWDx FLAG
ADCy_AWDx_OUT

- Converting regular channels 1,2,3,4,5,6,7
- Regular channels 1,2,3,4,5,6,7 are all guarded
MS31026V1

Figure 251. ADC_AWDx_OUT signal generation (on a single regular channel)
ADC
Conversion1
STATE
outside

Conversion2

Conversion1

Conversion2 Conversion1

inside

outside

Conversion2

Conversion1 Conversion2
outside

EOC FLAG
EOS FLAG
cleared
by S/W

cleared
by S/W

AWDx FLAG
ADCy_AWDx_OUT

- Converting regular channels 1 and 2
- Only channel 1 is guarded
MS31027V1

Figure 252. ADC_AWDx_OUT signal generation (on all injected channels)
ADC
STATE

RDY

Conversion1
inside

Conversion2
outside

Conversion3 Conversion4
inside
outside

Conversion
outside

Conversion
outside

cleared
by S/W

cleared
by S/W

Conversion
inside

JEOS FLAG

AWDx FLAG

cleared
by S/W

cleared
by S/W

ADCy_AWDx_OUT

- Converting the injected channels 1, 2, 3, 4
- All injected channels 1, 2, 3, 4 are guarded
MS31028V1

RM0456 Rev 6

<!-- pagebreak -->

1377

Analog-to-digital converter (ADC12)

RM0456

Analog watchdog threshold control
LTRx[24:0] and HTRx[24:0] can be changed when an analog-to-digital conversion is
ongoing (that is between the start of conversion and the end of conversion of the ADC
internal state). If LTRx[24:0] and HTRx[24:0] are updated during the ADC conversion of the
ADC guarded channel, the watchdog function is masked for this conversion. This masking is
removed at the next start of conversion, resulting in an analog watchdog thresholds to be
applied from the next ADC conversion. The analog watchdog comparison is performed at
each end of conversion. If the current ADC data is out of the new interval, no interrupt and
AWDx_OUT signal are issued. The interrupt and the AWD generation only happen at the
end of the conversion which started after the threshold update. If AWD_xOUT is already
asserted, programming the new thresholds does not deassert the AWD_OUT signal.

Analog watchdog with gain and offset compensation
When gain and offset compensation are enabled, the analog watchdog compares the
threshold after the compensated data.

33.4.31

Oversampler
The oversampling unit performs data preprocessing to offload the CPU. It is able to handle
multiple conversions and average them into a single data with increased data width, up to
24-bit (14-bit values and OSR = 1024).
It provides a result with the following form, where N and M can be adjusted:

n = N–1

1
Result = ---- ×
M

∑ Conversion(tn)
n=0

It enables the following functions to be performed by hardware: averaging, data rate
reduction, SNR improvement, basic filtering.
The oversampling ratio N is defined using the OSR[9:0] bits in the ADC_CFGR2 register,
and can range from 2x to 1024x. The division coefficient M consists of a right bit shift up to
10 bits, and is defined using the OVSS[3:0] bits in the ADC_CFGR2 register.
The summation unit can yield a result up to 24 bits (1024 x 14-bit results), which can be left
or right shifted. When right shifting is selected, it is rounded to the nearest value using the
least significant bits left apart by the shifting, before being transferred into the ADC_DR data
register.
Figure 253 gives a numerical example of the processing, from a raw 24-bit accumulated
data to the final 14-bit result.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Analog-to-digital converter (ADC12)
Figure 253. 14-bit result oversampling with 10-bits right shift and rounding
31

23

24-bit data
OSR=1024

0

31

7

23

31

23

0

D13..D0

15

0

31

OVSS[3:0]=0

Right shifting and rounding
15
7

0

24-bit data
OSR=1024

0

D23..D0

14-bit data
OSR=1024

14-bit data
OSR=1024

15

7

OVSS[3:0]=1010

0

0xFFE258

23

Right shifting and rounding
15
7

0

0x3FF8

OVSS[3:0]=0

0
OVSS[3:0]=1010
MSv62476V2

The conversion timings in oversampling mode do not change: the sample time is maintained
during the whole oversampling sequence. A new data is provided every N conversions with
an equivalent delay equal to N x TCONV = N x (tSMPL + tSAR). The flags are set as follows:
•

The end of the sampling phase (EOSMP) is set after each sampling phase.

•

The end of conversion (EOC) occurs once every N conversions, when the
oversampled result is available.

•

The end of sequence (EOS) occurs once the sequence of oversampled data is
completed (i.e. after N x sequence length conversions total).

Operating modes supported during oversampling
In oversampling mode, most of the ADC operating modes are maintained:

Note:

•

Single or continuous mode conversions

•

ADC conversions start either by software or with triggers

•

ADC stop during a conversion (abort)

•

Data read via CPU or DMA with overrun detection

•

Low-power modes (AUTDLY)

•

Programmable resolution: in this case, the reduced conversion values (configured
through the RES[1:0] bits of the ADC_CFGR1 register) are accumulated, truncated,
rounded, and shifted in the same way as 14-bit conversions.

The alignment mode is not available when working with oversampled data. The ALIGN bit in
ADC_CFGR is ignored and the data are always provided right-aligned.

Analog watchdog
The analog watchdog functionality is maintained, with the following differences:
•

The RES[1:0] bits are ignored. The comparison is always done using the full 25-bit
values, HTRx[24:0] and LTRx[24:0].

•

The comparison is performed on the oversampled accumulated value before shifting.

RM0456 Rev 6

<!-- pagebreak -->

1377

Analog-to-digital converter (ADC12)
Note:

RM0456

Care must be taken when using high shifting values, since this reduces the comparison
range. For instance, if the oversampled result is shifted by 4 bits, thus yielding an 8-bit rightaligned data, the effective analog watchdog comparison can only be performed on 8 bits.
The comparison is done between ADC_DR[11:4] and HT[7:0]/LT[[7:0]. HT[11:8]/LT[11:8]
must be kept reset.

Triggered mode
The averager can also be used for basic filtering purpose. Although not a very powerful filter
(slow roll-off and limited stop band attenuation), it can be used as a notch filter to reject
constant parasitic frequencies (typically coming from the mains or from a switched mode
power supply). For this purpose, a specific discontinuous mode can be enabled with the
TROVS bit of the DC_CFGR2, to be able to have an oversampling frequency defined by a
user and independent from the conversion time itself.
The Figure 254 below shows how conversions are started in response to triggers during
discontinuous mode.
If the TROVS bit is set, the content of the DISCEN bit is ignored and considered as 1.
Figure 254. Triggered regular oversampling mode (TROVS bit = 1)
CONT=0
DISCEN = 1
TROVS = 0

Trigger

Trigger

Ch(N)0 Ch(N)1 Ch(N)2 Ch(N)3

Ch(N)0 Ch(N)1 Ch(N)2 Ch(N)3

EOC flag set
CONT=0
DISCEN = 1
TROVS = 1

Trigger

Ch(N)0

Trigger

Ch(N)1

Trigger

Ch(N)2

Trigger

Ch(N)3

Trigger

Trigger

Ch(N)0

Ch(N)1

Trigger

Ch(N)2

EOC flag set
MS34455V2

Injected and regular sequencer management when oversampling
In oversampling mode, injected and regular sequencers can have different behaviors. The
oversampling can be enabled for both sequencers with some limitations if they have to be
used simultaneously (this is related to a unique accumulation unit).

Oversampling regular channels only
The regular oversampling mode bit, ROVSM, defines how the regular oversampling
sequence is resumed if it is interrupted by injected conversion:

<!-- pagebreak -->

•

In continued mode, the accumulation restarts from the last valid data (prior to the
conversion abort request due to the injected trigger). This ensures that oversampling is
complete whatever the injection frequency (providing at least one regular conversion
can be completed between triggers);

•

In resumed mode, the accumulation restarts from 0 (previous conversions results are
ignored). This mode guarantees that all data used for oversampling were converted
back-to-back within a single time-slot. Care must be taken to have an injection trigger

RM0456 Rev 6

RM0456

Analog-to-digital converter (ADC12)
period above the oversampling period length. If this condition is not respected, the
oversampling cannot be completed and the regular sequencer is blocked.
Figure 255 gives examples for an oversampling ratio of 4x.
Figure 255. Regular oversampling modes (4x ratio)
Oversampling
stopped

Oversampling
continued

Regular channels Ch(N)0 Ch(N)1 Ch(N)2 Ch(N)3 Ch(M)0 Ch(M)1
Abort
Trigger
Injected channels Ch(J)

Ch(M)1 Ch(M)2 Ch(M)3 Ch(O)0

Ch(K)

JEOC
Continued mode: ROVSE = 1, JOVSE = 0, ROVSM = 0, TROVS = X

Oversampling
aborted

Oversampling
resumed

Regular channels Ch(N)0 Ch(N)1 Ch(N)2 Ch(N)3 Ch(M)0 Ch(M)1
Abort
Trigger
Injected channels Ch(J)

Ch(M)0 Ch(M)1 Ch(M)2 Ch(M)3

Ch(K)

JEOC
Resumed mode: ROVSE = 1, JOVSE = 0, ROVSM = 1, TROVS = X
MS34456V1

Oversampling injected channels only
The injected oversampling mode bit, JOVSE, enables oversampling solely for conversions
in the injected sequencer.

RM0456 Rev 6

<!-- pagebreak -->

1377

Analog-to-digital converter (ADC12)

RM0456

Oversampling regular and injected channels
Both ROVSE and JOVSE bits can be set simultaneously. In this case, the regular
oversampling mode is forced to resumed mode (ROVSM bit ignored), as shown in
Figure 256.
Figure 256. Regular and injected oversampling modes used simultaneously
Oversampling
aborted

Oversampling
resumed

Regular channels Ch(N)0 Ch(N)1 Ch(N)2 Ch(N)3 Ch(M)0 Ch(M)1
Abort
Trigger

Ch(M)0 Ch(M)1

Injected channels Ch(J)0 Ch(J)1 Ch(J)2 Ch(J)3

JEOC
ROVSE = 1, JOVSE = 1, ROVSM = 1, TROVS = 0
MS34457V2

Triggered regular oversampling with injected conversions
Injected conversions can be performed in triggered regular mode. In this case, the injected
mode oversampling mode must be disabled, and the ROVSM bit is ignored (resumed mode
forced). The JOVSE bit must be reset. The behavior is shown in Figure 257.
Figure 257. Triggered regular oversampling with injection
Oversampling
resumed
Trigger

Regular channels Ch(N)0

Trigger

Trigger

Ch(N)1
Trigger

Trigger

Ch(N)2
Abort

Injected channels Ch(J)

Trigger

Ch(N)0 Ch(N)1
Resumed
Ch(K)

ROVSE = 1, JOVSE = 0, ROVSM = 1, TROVS = 1
MS34458V4

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Analog-to-digital converter (ADC12)

Auto-injected mode
It is possible to oversample auto-injected sequences and have all conversions results stored
in registers. This enables to save a DMA resource. This mode is available only when both
regular and injected oversampling active: JAUTO = 1, ROVSE = 1 and JOVSE = 1. Other
combinations are not supported. The ROVSM bit is ignored in auto-injected mode.
Figure 258 shows how the conversions are sequenced.
Figure 258. Oversampling in auto-injected mode
Regular channels N0 N1 N2 N3

N0 N1 N2 N3

Injected channels I0 I1 I2 I3 J0 J1 J2 J3 K0 K1 K2 K3 L0 L1 L2 L3
JAUTO =1, ROVSE = 1, JOVSE = 1, ROVSM = X, TROVS = 0
MS34459V1

It is possible to have also the triggered mode enabled, using the TROVS bit. In this case,
the ADC must be configured as follows: JAUTO = 1, DISCEN = 0, JDISCEN = 0,
ROVSE = 1, JOVSE = 1 and TROVSE = 1.

Dual ADC modes support when oversampling
It is possible to have oversampling enabled when working in dual ADC configuration, for the
injected simultaneous mode and regular simultaneous mode. In this case, the two ADCs
must be programmed with the very same settings (including oversampling).
All other dual ADC modes are not supported when either regular or injected oversampling is
enabled (ROVSE = 1 or JOVSE = 1).

Summary of combined modes
Table 319 summarizes all mode combinations, including non-supported modes.
Table 319. Summary of oversampler operating modes
Regular over- Injected oversampling:
sampling:
ROVSE bit

JOVSE bit

Oversampler
mode:
ROVSM bit

Triggered regular mode:

0 = continued

TROVS bit

Comment

1 = resumed
1

0

0 (continued)

0

Regular continued mode

1

0

0 (continued)

1

Not supported

1

0

1 (resumed)

0

Regular resumed mode

1

0

1 (resumed)

1

Triggered regular resumed
mode

1

1

0 (continued)

X

Not supported

1

1

1 (resumed)

0

Injected and regular resumed
mode

1

1

1 (resumed)

1

Not supported

0

1

X

X

Injected oversampling

RM0456 Rev 6

<!-- pagebreak -->

1377

Analog-to-digital converter (ADC12)

33.4.32

RM0456

Dual ADC modes
In devices with two ADCs or more, dual ADC modes can be used (see Figure 33.4.33):
•

ADC1 and ADC2 can be used together in dual mode (ADC1 is master)

In dual ADC mode the start of conversion is triggered alternately or simultaneously by the
ADCx master to the ADC slave, depending on the mode selected by the DUAL[4:0] bits of
the ADC12_CCR register.
Four possible modes are implemented:
•

Injected simultaneous mode

•

Regular simultaneous mode

•

Interleaved mode

•

Alternate trigger mode

It is also possible to use these modes combined in the following ways:
•

Injected simultaneous mode + regular simultaneous mode

•

Regular simultaneous mode + alternate trigger mode

•

Injected simultaneous mode + interleaved mode

In dual ADC mode (when bits DUAL[4:0] in the ADC12_CCR register are not equal to zero),
the bits CONT, AUTDLY, DISCEN, DISCNUM[2:0], JDISCEN, JAUTO of the ADC_CFGR
register are shared between the master and slave ADC: the bits in the slave ADC are
always equal to the corresponding bits of the master ADC.
To start a conversion in dual mode, the user must program the bits EXTEN[1:0], EXTSEL,
JEXTEN[1:0], JEXTSEL of the master ADC only, to configure a software or hardware
trigger, and a regular or injected trigger. (the bits EXTEN[1:0] and JEXTEN[1:0] of the slave
ADC are don’t care).
In regular simultaneous or interleaved modes: once the user sets bit ADSTART or bit
ADSTP of the master ADC, the corresponding bit of the slave ADC is also automatically
set. However, bit ADSTART or bit ADSTP of the slave ADC is not necessary cleared at the
same time as the master ADC bit.
In injected simultaneous or alternate trigger modes: once the user sets bit JADSTART or bit
JADSTP of the master ADC, the corresponding bit of the slave ADC is also automatically
set. However, bit JADSTART or bit JADSTP of the slave ADC is not necessary cleared at
the same time as the master ADC bit.
In dual ADC mode, the converted data of the master and slave ADC can be read in parallel,
by reading the ADC common data register (ADC12_CDR). The status bits can be also read
in parallel by reading the dual-mode status register (ADC12_CSR).

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Analog-to-digital converter (ADC12)
Figure 259. Dual ADC block diagram(1)
Regular data register
(32-bits)

Regular
channels
Slave ADC

Injected
channels
ADCx_INP0,
ADCx_INN0
ADCx_INP2,
ADCx_INN2

Internal triggers

Address/data bus

Injected data registers
(4 x32-bits)

Internal analog inputs

Regular data register
(32-bits)

GPIO
ports

ADCx_INP19,
ADCx_INN19

Injected data registers
(4 x32-bits)
Regular
channels

Internal analog inputs
Injected
channels

Dual mode
control

Start trigger mux.
(regular group)

Master ADC
Start trigger mux.
(injected group)

MSv41029V2

1. External triggers also exist on slave ADC but are not shown for the purposes of this diagram.
2. The ADC common data register (ADC12_CDR) contains both the master and slave ADC regular converted
data.

RM0456 Rev 6

<!-- pagebreak -->

1377

Analog-to-digital converter (ADC12)

RM0456

Injected simultaneous mode
This mode is selected by programming bits DUAL[4:0] = 00101
This mode converts an injected group of channels. The external trigger source comes from
the injected group multiplexer of the master ADC (selected by the JEXTSEL[4:0] bits in the
ADC_JSQR register).
Note:

Do not convert the same channel on the two ADCs (no overlapping sampling times for the
two ADCs when converting the same channel).
In simultaneous mode, one must convert sequences with the same length and inside a
sequence, the N-th conversion in master and slave must be configured with the same
sampling time.
Regular conversions can be performed on one or all ADCs. In that case, they are
independent of each other and are interrupted when an injected event occurs. They are
resumed at the end of the injected conversion group.
•

At the end of injected sequence of conversion event (JEOS) on the master ADC, the
converted data is stored into the master ADC_JDRy registers and a JEOS interrupt is
generated (if enabled)

•

At the end of injected sequence of conversion event (JEOS) on the slave ADC, the
converted data is stored into the slave ADC_JDRy registers and a JEOS interrupt is
generated (if enabled)

•

As the duration of the master injected sequence is equal to the duration of the slave
injected one (like in Figure 260), it is possible for the software to enable only one of the
two JEOS interrupts (for example master JEOS) and read both converted data (from
master ADC_JDRy and slave ADC_JDRy registers).
Figure 260. Injected simultaneous mode on four channels: dual ADC mode

MASTER ADC
SLAVE ADC

CH1

CH2

CH3

CH4

CH15

CH14

CH13

CH12

Trigger
Sampling

End of injected sequence on
MASTER and SLAVE ADC

Conversion
MS31900V1

If JDISCEN = 1, each simultaneous conversion of the injected sequence requires an
injected trigger event to occur.
This mode can be combined with AUTDLY mode:

<!-- pagebreak -->

•

Once a simultaneous injected sequence of conversions has ended, a new injected
trigger event is accepted only if both JEOS bits of the master and the slave ADC have
been cleared (delay phase). Any new injected trigger events occurring during the
ongoing injected sequence and the associated delay phase are ignored.

•

Once a regular sequence of conversions of the master ADC has ended, a new regular
trigger event of the master ADC is accepted only if the master data register (ADC_DR)
has been read. Any new regular trigger events occurring for the master ADC during the
ongoing regular sequence and the associated delay phases are ignored.
There is the same behavior for regular sequences occurring on the slave ADC.

RM0456 Rev 6

RM0456

Analog-to-digital converter (ADC12)

Regular simultaneous mode with independent injected
This mode is selected by programming bits DUAL[4:0] = 00110.
This mode is performed on a regular group of channels. The external trigger source comes
from the regular group multiplexer of the master ADC (selected by the EXTSEL[4:0] bits in
the ADC_CFGR register). A simultaneous trigger is provided to the slave ADC.
In this mode, independent injected conversions are supported. An injection request (either
on master or on the slave) aborts the current simultaneous conversions, which are restarted
once the injected conversion is completed.
Note:

Do not convert the same channel on the two ADCs (no overlapping sampling times for the
two ADCs when converting the same channel).
In regular simultaneous mode, one must convert sequences with the same length and
inside a sequence, the N-th conversion in master and slave must be configured with the
same sampling time.
Software is notified by interrupts when it can read the data:
•

At the end of each conversion event (EOC) on the master ADC, a master EOC interrupt
is generated (if EOCIE is enabled) and software can read the ADC_DR of the master
ADC.

•

At the end of each conversion event (EOC) on the slave ADC, a slave EOC interrupt is
generated (if EOCIE is enabled) and software can read the ADC_DR of the slave ADC.

•

As the duration of the master regular sequence is equal to the duration of the slave one
(like in Figure 261), it is possible for the software to enable only one of the two EOC
interrupt (ex: master EOC) and read both converted data from the Common Data
register (ADC12_CDR).

It is also possible to read the regular data using the DMA. Two methods are possible:
•

•

Note:

Using two DMA channels (one for the master and one for the slave). In this case bits
DAMDF[1:0] must be kept cleared.
–

Configure the DMA master ADC channel to read ADC_DR from the master. DMA
requests are generated at each EOC event of the master ADC.

–

Configure the DMA slave ADC channel to read ADC_DR from the slave. DMA
requests are generated at each EOC event of the slave ADC.

Configuring dual ADC mode data format DAMDF[1:0] bits, which leaves one DMA
channel free for other uses:
–

Configure DAMDF[1:0] = 10 or 11 (depending on resolution).

–

A single DMA channel is used (the one of the master). Configure the DMA master
ADC channel to read the common ADC register (ADC12_CDR)

–

A single DMA request is generated each time both master and slave EOC events
have occurred. At that time, the slave ADC converted data is available in the
upper half-word of the ADC12_CDR 32-bit register and the master ADC converted
data is available in the lower half-word of the ADC12_CDR register.

–

both EOC flags are cleared when the DMA reads the ADC12_CDR register.

When DAMDF[1:0] = 10 or 11, the user must program the same number of conversions in
the master’s sequence as in the slave’s sequence. Otherwise, the remaining conversions do
not generate a DMA request.

RM0456 Rev 6

<!-- pagebreak -->

1377

Analog-to-digital converter (ADC12)

RM0456

Figure 261. Regular simultaneous mode on 16 channels: dual ADC mode

MASTER ADC
SLAVE ADC

...

CH1

CH2

CH3

CH4

CH16

CH14

CH13

CH12 ...

Trigger
Sampling

CH16
CH1
End of regular sequence on
MASTER and SLAVE ADC

Conversion
ai16054b

If DISCEN = 1 then each “n” simultaneous conversions of the regular sequence require a
regular trigger event to occur (“n” is defined by DISCNUM).
This mode can be combined with AUTDLY mode:
•

Once a simultaneous conversion of the sequence has ended, the next conversion in
the sequence is started only if the common data register, ADC12_CDR (or the regular
data register of the master ADC) has been read (delay phase).

•

Once a simultaneous regular sequence of conversions has ended, a new regular
trigger event is accepted only if the common data register (ADC12_CDR) has been
read (delay phase). Any new regular trigger events occurring during the ongoing
regular sequence and the associated delay phases are ignored.

It is possible to use the DMA to handle data in regular simultaneous mode combined with
AUTDLY mode, assuming that multi-DMA mode is used: DAMDF[1:0] bits must be set to 10
or 11.
When regular simultaneous mode is combined with AUTDLY mode, it is mandatory for the
user to ensure that:

Note:

•

The number of conversions in the master’s sequence is equal to the number of
conversions in the slave sequence.

•

For each simultaneous conversion of the sequence, the length of the conversion of the
slave ADC is inferior to the length of the conversion of the master ADC. Note that the
length of the sequence depends on the number of channels to convert and the
sampling time and the resolution of each channel.

This combination of regular simultaneous mode and AUTDLY mode is restricted to the use
case when only regular channels are programmed: it is forbidden to program injected
channels in this combined mode.

Interleaved mode with independent injected
This mode is selected by programming bits DUAL[4:0] = 00111.
This mode can be started only on a regular group (usually one channel). The external
trigger source comes from the regular channel multiplexer of the master ADC.
After an external trigger occurs:
•

The master ADC starts immediately.

•

The slave ADC starts after a delay of several ADC clock cycles after the sampling
phase of the master ADC has complete.

The minimum delay which separates two conversions in interleaved mode is configured in
the DELAY bits in the ADC12_CCR register. This delay starts to count after the end of the
sampling phase of the master conversion. This way, an ADC cannot start a conversion if the

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Analog-to-digital converter (ADC12)
complementary ADC is still sampling its input (only one ADC can sample the input signal at
a given time).
•

The minimum possible DELAY is 1 to ensure that there is at least one cycle time
between the opening of the analog switch of the master ADC sampling phase and the
closing of the analog switch of the slave ADC sampling phase.

•

The maximum DELAY is equal to the number of cycles corresponding to the selected
resolution. However, the user must properly calculate this delay to ensure that an ADC
does not start a conversion while the other ADC is still sampling its input.

If the CONT bit is set on both master and slave ADCs, the selected regular channels of both
ADCs are continuously converted.
The software is notified by interrupts when it can read the data at the end of each
conversion event (EOC) on the slave ADC. A slave and master EOC interrupts are
generated (if EOCIE is enabled) and the software can read the ADC_DR of the slave/master
ADC.
Note:

In 16-bit data format, enable only the slave EOC interrupt and read the common data
register (ADC12_CDR). For 32-bit data format, enable both the slave and master EOC
interrupts and read ADC12_CDR2 register. But in this case, the user must ensure that the
duration of the conversions are compatible to ensure that inside the sequence, a master
conversion is always followed by a slave conversion before a new master conversion
restarts. It is recommended to use the MDMA mode.
It is also possible to have the regular data transferred by DMA. In this case, individual DMA
requests on each ADC cannot be used and it is mandatory to use the DAMDF mode, as
following:
•

Configure DAMDF[1:0] = 10 or 11 (depending on resolution).

•

A single DMA channel is used (the one of the masters). Configure the DMA master
ADC channel to read the common ADC register (ADC12_CDR).

•

A single DMA request is generated each time both master and slave EOC events have
occurred. At that time, the slave ADC converted data is available in the upper half-word
of the ADC12_CDR 32-bit register and the master ADC converted data is available in
the lower half-word of the ADC12_CCR register.

•

Both EOC flags are cleared when the DMA reads the ADC12_CCR register.

RM0456 Rev 6

<!-- pagebreak -->

1377

Analog-to-digital converter (ADC12)

RM0456

Figure 262. Interleaved mode on one channel in continuous conversion mode: dual
ADC mode

MASTER ADC

CH1

CH1

SLAVE ADC
Trigger

CH1

CH1

6 ADCCLK
cycles

6 ADCCLK
cycles

End of conversion on master and
slave ADC

Sampling
Conversion

MSv65310V1

Figure 263. Interleaved mode on one channel in single conversion mode: dual ADC
mode

MASTER ADC

CH1

CH1

SLAVE ADC
Trigger

CH1

CH1

6 ADCCLK
cycles

End of conversion on
master and slave ADC

6 ADCCLK
cycles

End of conversion on
master and slave ADC

Sampling
Conversion

MSv65311V1

If DISCEN = 1, each “n” simultaneous conversions (“n” is defined by DISCNUM) of the
regular sequence require a regular trigger event to occur.
In this mode, injected conversions are supported. When injection is done (either on master
or on slave), both the master and the slave regular conversions are aborted and the
sequence is restarted from the master (see Figure 264 below).

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Analog-to-digital converter (ADC12)
Figure 264. Interleaved conversion with injection
Injected trigger

Resume (always on master)

CH11
ADC1 (master)

CH1

ADC2 (slave)

CH1
CH2

CH2
read
CDR

Legend:

Sampling

CH1

CH1
CH2

read
CDR

CH1
CH2

conversions
aborted

CH1
CH2

read
CDR

CH0
read
CDR

Conversion

MS34460V1

Alternate trigger mode
This mode is selected by programming bits DUAL[4:0] = 01001.
This mode can be started only on an injected group. The source of external trigger comes
from the injected group multiplexer of the master ADC.
This mode is only possible when selecting hardware triggers: JEXTEN[1:0] must not be 0x0.
Injected discontinuous mode disabled (JDISCEN = 0 for both ADC)
1.

When the 1st trigger occurs, all injected master ADC channels in the group are
converted.

2.

When the 2nd trigger occurs, all injected slave ADC channels in the group are
converted.

3.

And so on.

A JEOS interrupt, if enabled, is generated after all injected channels of the master ADC in
the group have been converted.
A JEOS interrupt, if enabled, is generated after all injected channels of the slave ADC in the
group have been converted.
JEOC interrupts, if enabled, can also be generated after each injected conversion.
If another external trigger occurs after all injected channels in the group have been
converted then the alternate trigger process restarts by converting the injected channels of
the master ADC in the group.

RM0456 Rev 6

<!-- pagebreak -->

1377

Analog-to-digital converter (ADC12)

RM0456

Figure 265. Alternate trigger: injected group of each ADC

1st trigger

JEOC on
master ADC

JEOC on
master ADC

JEOC,JEOS on
master ADC

MASTER ADC
SLAVE ADC
2nd trigger JEOC on
slave ADC

3rd trigger

JEOC on
slave ADC

JEOC, JEOS
on slave ADC

JEOC on
JEOC on
master ADC master ADC

JEOC,JEOS on
master ADC

MASTER ADC
SLAVE ADC
4th trigger

JEOC on
slave ADC

Sampling
Conversion

JEOC on JEOC, JEOS
slave ADC on slave ADC
ai16059-m

Note:

Regular conversions can be enabled on one or all ADCs. In this case, the regular
conversions are independent of each other. A regular conversion is interrupted when the
ADC has to perform an injected conversion. It is resumed when the injected conversion is
finished.
The time interval between two trigger events must be greater than or equal to one ADC
clock period. The minimum time interval between two trigger events that start conversions
on the same ADC is the same as in the single ADC mode.
Injected discontinuous mode enabled (JDISCEN = 1 for both ADC)
If the injected discontinuous mode is enabled for both master and slave ADCs:
•

When the 1st trigger occurs, the first injected channel of the master ADC is converted.

•

When the 2nd trigger occurs, the first injected channel of the slave ADC is converted.

•

And so on.

A JEOS interrupt, if enabled, is generated after all injected channels of the master ADC in
the group have been converted.
A JEOS interrupt, if enabled, is generated after all injected channels of the slave ADC in the
group have been converted.
JEOC interrupts, if enabled, can also be generated after each injected conversions.
If another external trigger occurs after all injected channels in the group have been
converted, then the alternate trigger process restarts.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Analog-to-digital converter (ADC12)

Figure 266. Alternate trigger: Four injected channels (each ADC) in discontinuous mode
1st trigger

3rd trigger
JEOC on
master ADC

5th trigger
7th trigger
JEOC on
JEOC on
master ADC
master ADC

JEOC, JEOS on
master ADC

MASTER ADC
SLAVE ADC

Sampling
Conversion

JEOC on slave
JEOC on slave
JEOC on slave
ADC
ADC
ADC
2nd trigger
4th trigger
6th trigger

JEOC, JEOS on
slave ADC
8th trigger
ai16060V3

Combined regular/injected simultaneous mode
This mode is selected by programming bits DUAL[4:0] = 00001.
It is possible to interrupt the simultaneous conversion of a regular group to start the
simultaneous conversion of an injected group.
Note:

The sequences must be converted with the same length, the N-th conversion in master and
slave mode must be configured with the same sampling time inside a given sequence, or
the interval between triggers has to be longer than the long conversion time of the two
sequences. If the above conditions are not respected, the ADC with the shortest sequence
may restart while the ADC with the longest sequence is completing the previous
conversions.

Combined regular simultaneous + alternate trigger mode
This mode is selected by programming bits DUAL[4:0] = 00010.
It is possible to interrupt the simultaneous conversion of a regular group to start the alternate
trigger conversion of an injected group. Figure 267 shows the behavior of an alternate
trigger interrupting a simultaneous regular conversion.
The injected alternate conversion is immediately started after the injected event. If a regular
conversion is already running, in order to ensure synchronization after the injected
conversion, the regular conversion of all (master/slave) ADCs is stopped and resumed
synchronously at the end of the injected conversion.
Note:

The sequences must be converted with the same length, the N-th conversion in master and
slave mode must be configured with the same sampling time inside a given sequence, or
the interval between triggers has to be longer than the long conversion time of the two
sequences. If the above conditions are not respected, the ADC with the shortest sequence
may restart while the ADC with the longest sequence is completing the previous
conversions.

RM0456 Rev 6

<!-- pagebreak -->

1377

Analog-to-digital converter (ADC12)

RM0456

Figure 267. Alternate + regular simultaneous
1st trigger

ADC MASTER reg

CH1

CH2

CH3

CH4

CH4

CH5

CH7

CH8

CH8

CH9

CH1

ADC MASTER inj
ADC SLAVE reg

CH3

CH4

CH6

CH7

CH1

ADC SLAVE inj

synchronization not lost
2nd trigger
ai16062V2-m

If a trigger occurs during an injected conversion that has interrupted a regular conversion,
the alternate trigger is served. Figure 268 shows the behavior in this case (note that the 6th
trigger is ignored because the associated alternate conversion is not complete).
Figure 268. Case of trigger occurring during injected conversion
1st trigger

ADC MASTER reg

CH1

CH2

CH7

CH8

ADC MASTER inj
ADC SLAVE reg
ADC SLAVE inj

5th trigger

3rd trigger

CH3
CH14
CH9

CH3

CH4
CH14

CH4

CH9

CH10

CH10

CH5
CH14
CH11

CH15
2nd trigger

CH5

CH6

CH11

CH12

CH15
4th trigger

6th trigger
(ignored)
ai16063V2

Combined injected simultaneous plus interleaved
This mode is selected by programming bits DUAL[4:0] = 00011.
It is possible to interrupt an interleaved conversion with a simultaneous injected event.

Caution:

<!-- pagebreak -->

In this case the interleaved conversion is interrupted immediately and the simultaneous
injected conversion starts. At the end of the injected sequence the interleaved conversion is
resumed. When the interleaved regular conversion resumes, the first regular conversion
which is performed is always the master’s one. Figure 269, Figure 270 and Figure 271 show
the behavior using an example.
In this mode, it is mandatory to use the Common Data Register to read the regular data with
a single read access. On the contrary, master-slave data coherency is not guaranteed.

RM0456 Rev 6

RM0456

Analog-to-digital converter (ADC12)
Figure 269. Interleaved single channel CH0 with injected sequence CH11, CH12

ADC1 (master)

CH0

CH0

ADC2 (slave)

CH0

CH0

Conversions
aborted

CH0
read
CDR

CH0

CH0
read
CDR

CH0
CH0

CH11

CH11

CH12

CH12

CH0
CH0

read
CDR

CH0
read
CDR

Legend:
Injected trigger
Sampling

Resume
(always restart with the master)

Conversion

MS34461V1

Figure 270. Two interleaved channels (CH1, CH2) with injected sequence CH11, CH12
- case 1: Master interrupted first
ADC1 (master)

CH1

CH1

ADC2 (slave)

CH1

CH2

Conversions
aborted

CH2
read
CDR

CH1

CH2
read
CDR

CH1
CH2

CH11

CH11

CH12

CH12

CH1
CH2

read
CDR

CH2
read
CDR

Legend:
Injected trigger
Sampling

Resume
(always restart with the master)

Conversion

MS34462V1

Figure 271. Two Interleaved channels (CH1, CH2) with injected sequence CH11, CH12
- case 2: Slave interrupted first
ADC1 (master)

CH1

ADC2 (slave)

CH1
CH2

CH1
CH2

read
CDR

Conversions
aborted
CH2

read
CDR

CH1

CH1
CH2

CH11

CH11

CH12

CH12

CH1
CH2

read
CDR

CH2
read
CDR

Legend:
Injected trigger
Sampling

Conversion

Resume
(always restart with the master)
MS34463V2

RM0456 Rev 6

<!-- pagebreak -->

1377

Analog-to-digital converter (ADC12)

RM0456

DMA requests in dual ADC mode
In all dual ADC modes, it is possible to use two DMA channels (one for the master, one for
the slave) to transfer the data, like in single mode (refer to Figure 272: DMA Requests in
regular simultaneous mode when DAMDF[1:0] = 00).
Figure 272. DMA Requests in regular simultaneous mode when DAMDF[1:0] = 00
Trigger
ADC Master regular

Trigger

CH1

CH1

ADC Master EOC

ADC Slave regular

CH2

CH2

ADC Slave EOC
DMA request from ADC Master
DMA reads Master
ADC_DR

DMA reads Mater
ADC_DR

DMA request from ADC Slave
DMA reads Slave
ADC_DR

DMA reads Slave
ADC_DR

Configuration where each sequence contains only one conversion

MSv31032V2

In simultaneous regular and interleaved modes, it is also possible to save one DMA channel
and transfer both data using a single DMA channel. For this the DAMDF[1:0] bits must be
configured in the ADC12_CCR register:
•

DAMDF[1:0] = 10, 32-bit format: A single DMA request is generated alternatively
when either the master or slave EOC events have occurred. At that time, the data items
are alternatively available in the ADC12_CDR2 32-bit register. This mode is used in
interleaved mode and in regular simultaneous mode when resolution is above 16-bit.
Example:
Interleaved dual mode: a DMA request is generated each time a new 32-bit data is
available:
1st DMA request: ADC12_CDR2[31:0] = MST_ADC_DR[31:0]
2nd DMA request: ADC12_CDR2[31:0] = SLV_ADC_DR[31:0]

•

<!-- pagebreak -->

DAMDF[1:0] = 10, 16-bit format: A single DMA request is generated each time both
master and slave EOC events have occurred. At that time, two data items are available
and the 32-bit register ADC12_CDR contains the two half-words representing two

RM0456 Rev 6

RM0456

Analog-to-digital converter (ADC12)
ADC-converted data items. The slave ADC data take the upper half-word and the
master ADC data take the lower half-word.
This mode is used in interleaved mode and in regular simultaneous mode when
resolution is ranging from 10 to 16-bit. Any value above 16-bit in the master or the
slave converter is truncated to the least 16 significant bits.
Example:
Interleaved dual mode: a DMA request is generated each time 2 data items are
available:
1st DMA request: ADC12_CDR[31:0] = SLV_ADC_DR[15:0] |
MST_ADC_DR[15:0]
2nd DMA request: ADC12_CDR[31:0] = SLV_ADC_DR[15:0] |
MST_ADC_DR[15:0]
Figure 273. DMA requests in regular simultaneous mode when DAMDF[1:0] = 10
Trigger
ADC Master regular

Trigger

Trigger

Trigger

CH1

CH1

CH1

CH1

CH2

CH2

CH2

CH2

ADC Master EOC
ADC Slave regular
ADC Slave EOC
DMA request from
ADC Master
DMA request from
ADC Slave

Configuration where each sequence contains only one conversion
MSv31033V3

RM0456 Rev 6

<!-- pagebreak -->

1377

Analog-to-digital converter (ADC12)

RM0456

Figure 274. DMA requests in interleaved mode when DAMDF[1:0] = 10

Trigger

Trigger

ADC Master regular

CH1

CH1

CH1

ADC Master EOC

Delay

Delay

Delay

ADC Slave regular

CH2

CH2

Trigger

Trigger

Trigger

CH1
Delay
CH2

CH1
Delay

CH2

CH2

ADC Slave EOC
DMA request from
ADC Master
DMA request from
ADC Slave

Configuration where each sequence contains only one conversion
MSv31034V2

Note:

When using multiple-ADC mode, the user must take care to configure properly the duration
of the master and slave conversions so that a DMA request is generated and served for
reading both data (master + slave) before a new conversion is available.
•

DAMDF[1:0] = 11: This mode is similar to the DAMDF[1:0] = 10. The only differences
are that on each DMA request (two data items are available), two bytes representing
two ADC converted data items are transferred as a half-word.
This mode is used in interleaved and regular simultaneous mode when the result is 8bit. A new DMA request is issued when four new 8-bit values are available.
Example:
Interleaved dual mode: a DMA request is generated each time 2 data items are
available
DMA request:
ADC12_CDR[7:0] = MST_ADC_DR[7:0]
ADC12_CDR[15:8] = SLV_ADC_DR[7:0]
ADC12_CDR[31:16] = 0x0

Overrun detection
In dual ADC mode (when DUAL[4:0] is not equal to 0b00000), if an overrun is detected on
one of the ADCs, the DMA requests are no longer issued to ensure that all the data
transferred to the RAM are valid (this behavior occurs whatever the DAMDF configuration).
It may happen that the EOC bit corresponding to one ADC remains set because the data
register of this ADC contains valid data.

DMA one shot mode/ DMA circular mode when multiple-ADC mode is
selected
When DAMDF mode is selected (10 or 11), bit DMNGT[1:0] = 10 in the master ADC
ADC12_CCR register must also be configured to select between DMA one shot mode and
circular mode, as explained in section Section : Managing conversions using the DMA.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Analog-to-digital converter (ADC12)

Stopping the conversions in dual ADC modes
The user must set the control bits ADSTP/JADSTP of the master ADC to stop the
conversions of both ADC in dual ADC mode. The other ADSTP control bit of the slave ADC
has no effect in dual ADC mode.
Once both ADCs are effectively stopped, the bits ADSTART/JADSTART of the master and
slave ADCs are both cleared by hardware.

MDF mode in dual ADC mode interleaved mode
In dual ADC interleaved modes, the ADC conversion results can be transferred directly to
the multifunction digital filter (MDF).
This mode is enabled by setting the bits DMNGT[1:0] = 10 in the master ADC ADC_CFGR
register.
The ADC transfers alternatively the 16 least significant bits of the regular data register from
the master and the slave converter to a single channel of the MDF.
The data format must be 16-bit signed:
ADC_DR[31:16] = 0x0000
ADC_DR[15] = sign
ADC_DR[14:0] = data
Any value above 16-bit signed format in any converter is truncated.

MDF mode in dual ADC simultaneous mode
The dual mode is not required to use MDF in dual ADC simultaneous mode since
conversion data are treated by each individual channel. Single mode with same trigger
source results in simultaneous conversion with MDF interface.

33.4.33

Temperature sensor
The temperature sensor can measure the device junction temperature (TJ) in the –40 to
125 °C temperature range.
The temperature sensor is internally connected ADC input channels that are used to convert
the sensor output voltage to a digital value (see Section 33.4.4: ADC connectivity for more
details). The sampling time for the temperature sensor analog pin must be greater than the
stabilization time specified in the device datasheet.
When it is not in use, the sensor can be placed in power-down mode.
Figure 275 shows the block diagram of the temperature sensor.

RM0456 Rev 6

<!-- pagebreak -->

1377

Analog-to-digital converter (ADC12)

RM0456

Figure 275. Temperature sensor channel block diagram

Converted
data
ADCx

Temperature
sensor

VSENSE

Address/data bus

VSENSESEL
control bit

ADC input

MSv62477V2

Reading the temperature
To use the sensor:
1.

Select the input channels to which the temperature sensor is connected (with the
appropriate sampling time).

2.

Program with the appropriate sampling time (refer to electrical characteristics section of
the device datasheet).

3.

Set the VSENSESEL bit in the ADC12_CCR register to wake up the temperature
sensor from power-down mode.

4.

Start the ADC conversion.

5.

Read the resulting data in the ADC data register.

6.

Calculate the actual temperature using the following formula:

TS_CAL2_TEMP – TS_CAL1_TEMP
Temperature ( in °C ) = ------------------------------------------------------------------------------------------------- × ( TS_DATA – TS_CAL1 ) + TS_CAL1_TEMP
TS_CAL2 – TS_CAL1

Where:
–

TS_CAL2 is the temperature sensor calibration value acquired at
TS_CAL2_TEMP.

–

TS_CAL1 is the temperature sensor calibration value acquired at
TS_CAL1_TEMP.

–

TS_DATA is the actual temperature sensor output value converted by ADC

Refer to Section 33.3: ADC implementation for more information on TS_CAL1 and
TS_CAL2 calibration points.
Note:

<!-- pagebreak -->

The sensor has a startup time after waking from power-down mode and before it can output
at the correct level. The ADC also has a startup time after power-on. As a result, to minimize
the delay, the ADEN and VSENSESEL bits must be set simultaneously.

RM0456 Rev 6

RM0456

33.4.34

Analog-to-digital converter (ADC12)

VBAT supply monitoring
The VBATEN bit in the ADC12_CCR register is used to switch to the battery voltage. As the
VBAT voltage can be higher than VDDA, the VBAT pin is internally connected to a bridge
divider by 4 to ensure the correct operation of the ADC. This bridge is automatically enabled
when VBATEN is set, to connect VBAT/4 to the corresponding ADC input channels (see
Section 33.4.4: ADC connectivity for more details). As a consequence, the converted digital
value is one fourth of the VBAT voltage. To prevent any unwanted consumption on the
battery, it is recommended to enable the bridge divider only when needed, for ADC
conversion.
Refer to the electrical characteristics of the device datasheet for the sampling time value to
be applied when converting the VBAT/4 voltage.
Figure 276 shows the block diagram of the VBAT sensing feature.
Figure 276. VBAT channel block diagram

VBAT

ADCx

VBAT/4

Address/data bus

VBATEN control bit

ADC input

MSv41032V5

Note:

The VBATEN bit of the ADC12_CCR register must be set to enable the conversion of the
ADC internal channels to which VBAT is connected (see Section 33.4.4: ADC connectivity
for more details).

33.4.35

Monitoring the internal voltage reference
The internal voltage reference can be monitored to have a reference point for evaluating the
ADC VREF+ voltage level.
Refer to Section 33.4.4: ADC connectivity for details on the ADC input channels to which the
internal voltage reference is internally connected.
The sampling time for this channel must be greater than the stabilization time specified in
the device datasheet.
Figure 276 shows the block diagram of the VREFINT sensing feature.

RM0456 Rev 6

<!-- pagebreak -->

1377

Analog-to-digital converter (ADC12)

RM0456

Figure 277. VREFINT channel block diagram

VREFEN control bit
Internal power
block

VREFINT

ADCx

ADC input

MSv41033V7

Note:

The VREFEN bit of the ADC12_CCR register must be set to enable the conversion of the
ADC internal channels to which VREFINT is connected (see Section 33.4.4: ADC
connectivity for more details).

Calculating the actual VREF+ voltage using the internal reference voltage
The VDDA power supply voltage applied to the microcontroller may be subject to variation or
not precisely known. The embedded internal voltage reference (VREFINT) and its calibration
data acquired by the ADC during the manufacturing process at VREF+ = 3.0 V can be used
to evaluate the actual VREF+ voltage level, if VREF+ pin is connected to a variable VDDA
power supply.
The following formula gives the actual VREF+ voltage supplying the device:
V REF+ = 3.0 V × VREFINT_CAL ⁄ VREFINT_DATA

Where:

<!-- pagebreak -->

