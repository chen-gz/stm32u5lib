1377

Analog-to-digital converter (ADC12)

33.4.19

RM0456

Conversion on external trigger and trigger polarity (EXTSEL,
EXTEN[1:0], JEXTSEL, JEXTEN[1:0])
A conversion or a sequence of conversions can be triggered either by software or by an
external event (for example, timer capture, input pins). If the EXTEN[1:0] control bits (for a
regular conversion) or JEXTEN[1:0] bits (for an injected conversion) are different from 00,
then external events are able to trigger a conversion with the selected polarity.
The regular trigger selection is effective once software has set bit ADSTART = 1 and the
injected trigger selection is effective once software has set bit JADSTART = 1.
Any hardware trigger which occurs while a conversion is ongoing are ignored.
•

If ADSTART = 0, regular hardware triggers are ignored.

•

If JADSTART = 0, injected hardware triggers are ignored.

Table 311 provides the correspondence between the EXTEN[1:0] and JEXTEN[1:0] values
and the trigger polarity.
Table 311. Configuring the trigger polarity for regular external triggers
EXTEN[1:0]

Note:

Source

00

Hardware Trigger detection disabled, software trigger detection enabled

01

Hardware Trigger with detection on the rising edge

10

Hardware Trigger with detection on the falling edge

11

Hardware Trigger with detection on both the rising and falling edges

The polarity of the regular trigger cannot be changed on-the-fly.
Table 312. Configuring the trigger polarity for injected external triggers
JEXTEN[1:0]

Source

00

Hardware trigger detection disabled, software trigger detection enabled

01

Hardware trigger with detection on the rising edge

10

Hardware trigger with detection on the falling edge

11

Hardware trigger with detection on both the rising and falling edges

The EXTSEL[4:0] and JEXTSEL[4:0] control bits select which events can trigger regular and
injected groups conversion, out of 21 possibilities.
A regular group conversion can be interrupted by an injected trigger.
Note:

<!-- pagebreak -->

The trigger selection cannot be changed on-the-fly.

RM0456 Rev 6

RM0456

Analog-to-digital converter (ADC12)
Figure 231. Triggers are shared between ADC master and ADC slave
ADC MASTER
adc_ext0_trg
EXTi mapped at
adc_ext1_trg
product level ....... .......

External regular trigger

adc_ext31_trg
EXTSEL[4:0]

External injected trigger

JEXTSEL[4:0]
ADC SLAVE

External regular trigger

EXTSEL[4:0]
adc_jext0_trg
JEXTi mapped at adc_jext1_trg
....... .......
product level
adc_jext31_trg

External injected trigger

JEXTSEL[4:0]
MSv41035V2

Refer to Table ADC external triggers for regular channels and Table ADC external triggers
for injected channels in Section 33.4.2: ADC pins and internal signals for the connection of
the above internal analog inputs to external ADC pins or internal signals.

33.4.20

Injected channel management
Triggered injection mode
To use triggered injection, the JAUTO bit must be cleared in the ADC_CFGR1 register:

Note:

1.

Start the conversion of a group of regular channels either by an external trigger or by
setting the ADSTART bit in the ADC_CR register.

2.

If an external injected trigger occurs or if the JADSTART bit in the ADC_CR register is
set during the conversion of a regular group of channels, the current conversion is
reset and the injected channel sequence switches are launched (all the injected
channels are converted once).

3.

Then, the regular conversion of the regular group of channels is resumed from the last
interrupted regular conversion.

4.

If a regular event occurs during an injected conversion, the injected conversion is not
interrupted but the regular sequence is executed at the end of the injected sequence.
Figure 232 shows the corresponding timing diagram.

When using triggered injection, one must ensure that the interval between trigger events is
longer than the injection sequence. For instance, if the sequence length is 44 ADC clock
cycles (that is two conversions with a minimum sampling time), the minimum interval
between triggers must be 45 ADC clock cycles.
RM0456 Rev 6

<!-- pagebreak -->

1377

Analog-to-digital converter (ADC12)

RM0456

Auto-injection mode
If the JAUTO bit is set in the ADC_CFGR1 register, the channels in the injected group are
automatically converted after the regular group of channels. This can be used to convert a
sequence of up to 20 conversions programmed in the ADC_SQRy and ADC_JSQR
registers.
In this mode, the ADSTART bit in the ADC_CR register must be set to start regular
conversions, followed by injected conversions (JADSTART must be kept cleared). Setting
the ADSTP bit aborts both regular and injected conversions (JADSTP bit must not be used).
In this mode, the external trigger on injected channels must be disabled.
If the CONT bit is also set in addition to the JAUTO bit, regular channels followed by injected
channels are continuously converted.
Note:

It is not possible to use both the auto-injected and discontinuous modes simultaneously.
When the DMA is used for exporting the regular sequencer’s data in JAUTO mode, it is
necessary to program it in circular mode (CIRC bit set in the DMA_CCRx register). If the
CIRC bit is reset (single-shot mode), the JAUTO sequence is stopped upon a DMA Transfer
Complete event.
Figure 232. Injected conversion latency

adc_ker_ck

Injection event

Reset ADC

max. latency

(1)

SOC

MSv43771V1

1. The maximum latency value can be found in the electrical characteristics of the device datasheet.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

33.4.21

Analog-to-digital converter (ADC12)

Discontinuous mode (DISCEN, DISCNUM, JDISCEN)
Regular group mode
This mode is enabled by setting the DISCEN bit in the ADC_CFGR1 register.
It is used to convert a short sequence (subgroup) of n conversions (n ≤ 8) that is part of the
sequence of conversions selected in the ADC_SQRy registers. The value of n is specified
by writing to the DISCNUM[2:0] bits in the ADC_CFGR1 register.
When an external trigger occurs, it starts the next n conversions selected in the ADC_SQRx
registers until all the conversions in the sequence are done. The total sequence length is
defined by the L[3:0] bits in the ADC_SQR1 register.
Example
•

•

Note:

DISCEN = 1, n=3, channels to be converted = 1, 2, 3, 6, 7, 8, 9, 10, 11
–

1st trigger: channels converted are 1, 2, 3 (an EOC event is generated at each
conversion).

–

2nd trigger: channels converted are 6, 7, 8 (an EOC event is generated at each
conversion).

–

3rd trigger: channels converted are 9, 10, 11 (an EOC event is generated at each
conversion) and an EOS event is generated after the conversion of channel 11.

–

4th trigger: channels converted are 1, 2, 3 (an EOC event is generated at each
conversion).

–

...

DISCEN = 0, channels to be converted = 1, 2, 3, 6, 7, 8, 9, 10,11
–

First trigger: the complete sequence is converted: channel 1, then 2, 3, 6, 7, 8, 9,
10 and 11. Each conversion generates an EOC event and the last one also
generates an EOS event.

–

All the next trigger events relaunch the complete sequence.

When a regular group is converted in discontinuous mode, no rollover occurs (the last
subgroup of the sequence can have less than n conversions).
When all subgroups are converted, the next trigger starts the conversion of the first
subgroup. In the example above, the 4th trigger reconverts the channels 1, 2 and 3 in the
1st subgroup.
It is not possible to have both discontinuous mode and continuous mode enabled. In this
case (if DISCEN = 1, CONT = 1), the ADC behaves as if continuous mode was disabled.

Injected group mode
This mode is enabled by setting the JDISCEN bit in the ADC_CFGR1 register. It converts
the sequence selected in the ADC_JSQR register, channel by channel, after an external
injected trigger event. This is equivalent to discontinuous mode for regular channels where
‘n’ is fixed at 1.
When an external trigger occurs, it starts the next channel conversions selected in the
ADC_JSQR registers until all the conversions in the sequence are done. The total sequence
length is defined by the JL[1:0] bits in the ADC_JSQR register.

RM0456 Rev 6

<!-- pagebreak -->

1377

Analog-to-digital converter (ADC12)

RM0456

Example
•

Note:

JDISCEN = 1, channels to be converted = 1, 2, 3
–

1st trigger: channel 1 converted (a JEOC event is generated)

–

2nd trigger: channel 2 converted (a JEOC event is generated)

–

3rd trigger: channel 3 converted and a JEOC event + a JEOS event are generated

–

...

When all injected channels have been converted, the next trigger starts the conversion of
the first injected channel. In the example above, the 4th trigger reconverts the 1st injected
channel 1.
It is not possible to use both auto-injected mode and discontinuous mode simultaneously:
the bits DISCEN and JDISCEN must be kept cleared by software when JAUTO is set.

33.4.22

Programmable resolution (RES) - fast conversion mode
It is possible to perform faster conversion by reducing the ADC resolution.
The resolution can be configured to be either 14, 12, 10, 8 bits by programming the control
bits RES[1:0]. Figure 237, Figure 238, Figure 239 and Figure 240 show the conversion
result format with respect to the resolution as well as to the data alignment (in continuous
mode assuming no added extra sampling cycle for high input resistance).
Lower resolution enables faster conversion time for applications where high-data precision
is not required. It reduces the conversion time spent by the successive approximation steps
according to Table 313.
Table 313. TSAR timings depending on resolution

33.4.23

RES

TSAR
(ADC clock cycles)

TSAR (ns) at
Fadc_ker_ck =
55 MHz

Tadc_ker_ck
(ADC clock cycles) with
Sampling time=5 ADC
clock cycles

Tadc_ker_ck (ns)
at Fadc_ker_ck =
55 MHz

14

17 ADC clock cycles

309.1

22 ADC clock cycles

400.0

12

15 ADC clock cycles

272.7

20 ADC clock cycles

363.6

10

13 ADC clock cycles

236.4

18 ADC clock cycles

327.3

8

11 ADC clock cycles

200.0

16 ADC clock cycles

290.9

End of conversion and end of sampling phase
(EOC, JEOC, EOSMP)
The ADC notifies the application for each end of regular conversion (EOC) event and each
injected conversion (JEOC) event.
The ADC sets the EOC flag as soon as a new regular conversion data is available in the
ADC_DR register. An interrupt can be generated if bit EOCIE is set. EOC flag is cleared by
the software either by writing 1 to it or by reading ADC_DR.
The ADC sets the JEOC flag as soon as a new injected conversion data is available in one
of the ADC_JDRy registers. An interrupt can be generated if bit JEOCIE is set. JEOC flag is
cleared by the software either by writing 1 to it or by reading the corresponding ADC_JDRy
register.

<!-- pagebreak -->

