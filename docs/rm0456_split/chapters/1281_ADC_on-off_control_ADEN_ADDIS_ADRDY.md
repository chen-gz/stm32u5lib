RM0456 Rev 6

RM0456

Analog-to-digital converter (ADC12)
from occurring, the software can insert the __DMB(); command after each access to these
registers.

33.4.9

ADC on-off control (ADEN, ADDIS, ADRDY)
First of all, follow the procedure described in Section 33.4.6: ADC Deep-power-down mode
(DEEPPWD) and ADC voltage regulator (ADVREGEN)).
Once the DEEPPWD bit is cleared and the ADVREGEN bit is set, the ADC can be enabled.
It requires a stabilization time of tSTAB before staring converting accurately (see Figure 226).
Two control bits enable or disable the ADC:
•

When ADEN = 1: the ADC is enabled. The ADRDY is set as soon as the ADC is ready
for operation.

•

When ADDIS = 1: the ADC is disabled.

ADEN and ADDIS bits are automatically cleared by hardware as soon as the analog ADC is
effectively disabled.
Regular conversions can then start either by setting ADSTART (refer to Section 33.4.19:
Conversion on external trigger and trigger polarity (EXTSEL, EXTEN[1:0], JEXTSEL,
JEXTEN[1:0])) or when an external trigger event occurs if triggers are enabled.
Injected conversions start by setting JADSTART or when an external injected trigger event
occurs if injected triggers are enabled.

Software procedure to enable the ADC
1.

Clear the ADRDY bit in the ADC_ISR register by writing 1.

2.

Set ADEN = 1.

3.

Wait until ADRDY = 1 (ADRDY is set after the ADC startup time). This can be done by
using the associated interrupt (ADRDYIE = 1).

4.

Clear the ADRDY bit in the ADC_ISR register by writing 1 (optional).

Software procedure to disable the ADC
1.

Check that both ADSTART = 0 and JADSTART = 0 to make sure that no conversion is
ongoing. If required, stop any ongoing regular and injected conversion by setting
ADSTP = 1 and JADSTP = 1 and then wait until ADSTP = 0 and JADSTP = 0.

2.

Set ADDIS.

3.

If required by the application, wait until ADEN = 0, until the analog ADC is effectively
disabled (ADDIS is automatically reset once ADEN = 0).

RM0456 Rev 6

<!-- pagebreak -->

1377

Analog-to-digital converter (ADC12)

RM0456

Figure 226. Enabling/disabling the ADC

ADEN
tSTAB
ADRDY

ADDIS

ADC
state

by S/W

OFF

Startup

RDY

Converting CH

RDY

REQ
-OF

OFF

by H/W
MSv62472V1

33.4.10

Constraints when writing the ADC control bits
The software can program the RCC control bits to configure and enable the ADC clock
(refer to the Reset and clock control section), the control DIFSEL bits in the ADC_DIFSEL
register, the ADC12_CCR register and the ADCAL and ADEN control bits in the ADC_CR
register, only if the ADC is disabled.
The software can program the ADCAL bit to launch the calibration when ADEN is cleared. It
can read or update the calibration factor if ADEN is set and no conversion is ongoing
(ADSTART and JADSTART both cleared).
The software is then allowed to write the ADSTART, JADSTART, and ADDIS control bits in
the ADC_CR register only if the ADC is enabled and there is no pending request to disable
it (ADEN must be equal to 1 and ADDIS to 0).
The following constraints apply to all the other control bits of the ADC_CFGRx,
ADC_SMPRy, ADC_LTRy, ADC_HTRy, ADC_SQRy, ADC_OFRy and ADC_IER registers:
•

Control bits related to configuration of regular conversions: the software is allowed to
write them only if the ADC is enabled (ADEN = 1) and no regular conversion is ongoing
(ADSTART must be equal to 0).

•

Control bits related to configuration of injected conversions: the software is allowed to
write them only if the ADC is enabled (ADEN = 1) and no injected conversion is
ongoing (JADSTART must be equal to 0).

•

ADC_LTRy, ADC_HTRy registers can be modified when an analog-to-digital
conversion is ongoing (refer to Section 33.4.30: Analog window watchdog (AWD1EN,
JAWD1EN, AWD1SGL, AWD1CH, AWD2CH, AWD3CH, AWD_HTRy, AWD_LTRy,
AWDy)for details).

The software can write ADSTP or JADSTP control bits in the ADC_CR register only if the
ADC is enabled, a conversion is ongoing and there is no pending request to disable it
(ADSTART or JADSTART must be equal to 1 and ADDIS to 0).
Note:

<!-- pagebreak -->

There is no hardware protection to prevent these forbidden write accesses that may cause
the ADC to enter an unknown state. To recover from this situation, the ADC must be
disabled (clear ADEN as well as all the bits of the ADC_CR register).

RM0456 Rev 6

RM0456

33.4.11

Analog-to-digital converter (ADC12)

Channel selection (SQRx, JSQRx)
The ADC features up to 20 multiplexed channels per ADC:
•

Up to 17 analog inputs coming from GPIO pads

•

Each ADC is connected to 3 internal analog inputs:
–

the internal temperature sensor (VSENSE)

–

the internal reference voltage (VREFINT)

–

the VBAT monitoring channel (VBAT/4)

Refer to Table ADC interconnection in Section 33.4.2: ADC pins and internal signals for the
connection of the above internal analog inputs to external ADC pins or internal signals.
The conversions can be organized in two groups: regular and injected. A group consists of a
sequence of conversions that can be done on any channel and in any order. For instance, it
is possible to implement the conversion sequence in the following order: ADC_IN3,
ADC_IN8, ADC_IN2, ADC_IN2, ADC_IN0, ADC_IN2, ADC_IN2, ADC_IN15.
•

A regular group is composed of up to 16 conversions. The regular channels and their
order in the conversion sequence must be selected in the ADC_SQRy registers. The
total number of conversions in the regular group must be written in the L[3:0] bits in the
ADC_SQR1 register.

•

An injected group is composed of up to four conversions. The injected channels and
their order in the conversion sequence must be selected in the ADC_JSQR register.
The total number of conversions in the injected group must be written in the L[1:0] bits
in the ADC_JSQR register.

ADC_SQRy registers must not be modified while regular conversions are ongoing. To
modify ADC_SQRy registers, the ADC regular conversions must first be stopped by setting
ADSTP (refer to Section 33.4.18: Stopping an ongoing conversion (ADSTP, JADSTP)).
Note:

To convert one of the internal analog channels, the corresponding analog sources must first
be enabled by programming VBATEN, VSENSESEL, or VREFEN bits in the ADC12_CCR
registers.

33.4.12

Channel preselection register (ADC_PCSEL)
For each channel selected through SQRx or JSQRx bits, the corresponding ADC_PCSEL
bit must be configured in advance.
This ADC_PCSEL bit controls the analog switch integrated in the I/O level. The ADC input
multiplexer selects the ADC input according to SQRx and JSQRx configuration with very
high speed and the analog switch integrated in the I/O cannot react as fast as the ADC
multiplexer does. To avoid the delay due to on analog switch control on the I/O, it is
necessary to preselect the input channels that are selected through the SQRx and JSQRx.
The selection is based on the VINP[i] of each ADC input. For example, if the ADC converts
ADC_IN1, the PCSEL1 bit must also be set in ADC_PCSEL.

33.4.13

Channel-wise programmable sampling time (SMPR1, SMPR2)
Before starting a conversion, the ADC must establish a direct connection between the
voltage source under measurement and the embedded sampling capacitor of the ADC. This
sampling time must be enough for the input voltage source to charge the embedded
capacitor to the input voltage level.

RM0456 Rev 6

<!-- pagebreak -->

1377

Analog-to-digital converter (ADC12)

RM0456

Each channel can be sampled with a different sampling time that is programmable using the
SMP[2:0] bits in the ADC_SMPR1 and ADC_SMPR2 registers. It is therefore possible to
select among the following sampling time values:
•

SMP = 000: 5 ADC clock cycles

•

SMP = 001: 6 ADC clock cycles

•

SMP = 010: 12 ADC clock cycles

•

SMP = 011: 20 ADC clock cycles

•

SMP = 100: 36 ADC clock cycles

•

SMP = 101: 68 ADC clock cycles

•

SMP = 110: 391 ADC clock cycles

•

SMP = 111: 814 ADC clock cycles

The total conversion time is calculated as follows:
T CONV = Sampling time + Conversion time

Example
When converting a single data, the sampling time is five cycles and the conversion time
is 17 cycles for 14-bit mode. With an Fadc_ker_ck of 55 MHz:
TCONV = (5 + 17) ADC clock cycles = 22 ADC clock cycles = 0.40 µs
The above result assumes that RAIN << 1 KΩ (refer to the datasheet for additional
sampling time to be added depending on the external resistance).
The ADC notifies the end of the sampling phase by setting the status bit EOSMP (only for
regular conversion).

I/O analog switch voltage booster
The resistance of the I/O analog switches increases when the VDDA voltage is too low. The
sampling time must consequently be adapted accordingly (refer to the device datasheet for
the corresponding electrical characteristics). This resistance can be minimized at low VDDA
voltage by enabling an internal voltage booster (refer to the SYSCFG section for more
details).

Bulb sampling mode
When the BULB bit is set in the ADC_CFGR2 register, the sampling period starts
immediately after the last ADC conversion. A hardware or software trigger starts the
conversion after the sampling time has been programmed in the ADC_SMPR1 register. The
very first ADC conversion, after the ADC is enabled, is performed with the sampling time
programmed in SMP bits. The bulb mode is effective starting from the second conversion.
The maximum sampling time is limited (refer to the ADC characteristics section of the
datasheet).
The bulb mode may not be used in continuous conversion mode or with injected channel
conversion.
When the BULB bit is set, it is not allowed to set the SMPTRIG bit in ADC_CFGR2.
When conversions in bulb mode are stopped by setting ADSTP bit or when the DMA
transfers are complete, the ADC must be disabled by setting ADDIS bit.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Analog-to-digital converter (ADC12)
Figure 227. Bulb mode timing diagram
Normal mode (discontinuous sampling)
ADC state

idle

sample

conversion

idle

sample

conversion

idle

Trigger

BULB mode (continuous sampling)
ADC state

idle

sample

conversion

sample

conversion

sample

Trigger
Sampling time programmed in
SMP bits
MSv62473V1

Sampling time control trigger mode
When the SMPTRIG bit is set, the sampling time programmed though SMPx bits is not
applicable. The sampling time is controlled by the trigger signal edge.
When a hardware trigger is selected, each rising edge of the trigger signal starts the
sampling period. A falling edge ends the sampling period and starts the conversion.
When a software trigger is selected, the software trigger is not the ADSTART bit in ADC_CR
but the SWTRIG bit. SWTRIG bit has to be set to start the sampling period, and the
SWTRIG bit has to be cleared to end the sampling period and start the conversion.
The maximum sampling time is limited (refer to the ADC characteristics section of the
datasheet).
This mode is not compatible with the continuous conversion mode and injected channel
conversion.
When the SMPTRIG bit is set, it is not allowed to set the BULB bit.

33.4.14

Single conversion mode (CONT = 0)
In single conversion mode, the ADC performs once the conversions of all channels. This
mode is started with the CONT bit at 0 by either:
•

Setting the ADSTART bit in the ADC_CR register (for a regular channel, with software
trigger selected)

•

Setting the JADSTART bit in the ADC_CR register (for an injected channel, with
software trigger selected)

•

External hardware trigger event (for a regular or injected channel)
ADSTART bit or JADSTART bit must be set before triggering an external event.

Inside the regular sequence, after each conversion is complete:
•

The converted data are stored into the 32-bit ADC_DR register

•

The EOC (end of regular conversion) flag is set

•

An interrupt is generated if the EOCIE bit is set

RM0456 Rev 6

<!-- pagebreak -->

1377

Analog-to-digital converter (ADC12)

RM0456

Inside the injected sequence, after each conversion is complete:
•

The converted data are stored into one of the four 32-bit ADC_JDRy registers

•

The JEOC (end of injected conversion) flag is set

•

An interrupt is generated if the JEOCIE bit is set

After the regular sequence is complete:
•

The EOS (end of regular sequence) flag is set

•

An interrupt is generated if the EOSIE bit is set

After the injected sequence is complete:
•

The JEOS (end of injected sequence) flag is set

•

An interrupt is generated if the JEOSIE bit is set

Then the ADC stops until a new external regular or injected trigger occurs or until bit
ADSTART or JADSTART is set again.
Note:

To convert a single channel, program a sequence with a length of 1.

33.4.15

Continuous conversion mode (CONT = 1)
This mode applies to regular channels only.
In continuous conversion mode, when a software or hardware regular trigger event occurs,
the ADC performs once all the regular conversions of the channels and then automatically
restarts and continuously converts each conversion of the sequence. This mode is started
with the CONT bit at 1 either by an external trigger or by setting the ADSTART bit in the
ADC_CR register.
Inside the regular sequence, after each conversion is complete:
•

The converted data are stored into the 32-bit ADC_DR register

•

The EOC (end of conversion) flag is set

•

An interrupt is generated if the EOCIE bit is set

After the sequence of conversions is complete:
•

The EOS (end of sequence) flag is set

•

An interrupt is generated if the EOSIE bit is set

Then, a new sequence restarts immediately and the ADC continuously repeats the
conversion sequence.
Note:

To convert a single channel, program a sequence with a length of 1.
It is not possible to have both discontinuous mode and continuous mode enabled: it is
forbidden to set both DISCEN = 1 and CONT = 1.
Injected channels cannot be converted continuously. The only exception is when an injected
channel is configured to be converted automatically after regular channels in continuous
mode (using the JAUTO bit), refer to Section : Auto-injection mode).

<!-- pagebreak -->

RM0456 Rev 6

RM0456

33.4.16

Analog-to-digital converter (ADC12)

Starting conversions (ADSTART, JADSTART)
The software starts ADC regular conversions by setting ADSTART. When ADSTART is set,
the conversion starts:
•

immediately if EXTEN[1:0] = 0x0 (software trigger) or

•

at the next active edge of the selected regular hardware trigger, if EXTEN[1:0] is not
equal to 0x0

The software starts ADC injected conversions by setting JADSTART. When JADSTART is
set, the conversion starts:

Note:

•

immediately, if JEXTEN[1:0] = 0x0 (software trigger) or

•

at the next active edge of the selected injected hardware trigger if JEXTEN[1:0] is not
equal to 0x0

In auto-injection mode (JAUTO = 1), use the ADSTART bit to start regular conversions
followed by auto-injected conversions (JADSTART must be kept cleared).
ADSTART and JADSTART also provide information on whether any ADC operation is
ongoing. The ADC can be reconfigured while ADSTART and JADSTART are both cleared
(the ADC is idle).
ADSTART is cleared by hardware:

Note:

•

In single mode with software trigger (CONT = 0, EXTEN[1:0] = 0x0): at any end of the
conversion sequence (EOS = 1)

•

In discontinuous mode with software trigger (CONT = 0, DISCEN = 1,
EXTEN[1:0] = 0x0): at end of conversion (EOC = 1)

•

In all other cases (CONT = x, EXTEN[1:0] = x): after executing the ADSTP assertion
procedure by software.

In continuous mode (CONT = 1), ADSTART is not cleared by hardware with the assertion of
EOS because the sequence is automatically relaunched.
When a hardware trigger is selected in single mode (CONT = 0 and EXTEN[1:0] ≠ 0x0),
ADSTART is not cleared by hardware with the assertion of EOS to help the software that
does not need to reset ADSTART again for the next hardware trigger event. This ensures
that no further hardware triggers are missed.
JADSTART is cleared by hardware:

Note:

•

In single mode with software injected trigger (JEXTEN[1:0] = 0x0): at any end of the
injected conversion sequence (JEOS assertion) or at any end of subgroup processing
if JDISCEN = 1

•

In all other cases (JEXTEN[1:0]=x): after executing the JADSTP assertion procedure
by software.

When the software trigger is selected, the ADSTART bit must not be set if the EOC flag is
still high.

RM0456 Rev 6

<!-- pagebreak -->

1377

Analog-to-digital converter (ADC12)

33.4.17

RM0456

Timing
The elapsed time between the start of a conversion and the end of conversion is the sum of
the configured sampling time plus the successive approximation time depending on data
resolution for single conversion and minus the overlap time between the sampling and the
previous SAR for continuous conversion.
In single conversion mode:
T

T

CONV

= T

SMPL

CONV

+T

SAR

= T

SMPL

= 22 × T

+T

SAR

= [5

min

+ 17

14bits

adc_ker_ck min for 14bits

]× T

adc_ker_ck

= 400.0 ns (for F

adc_ker_ck

= 55 MHz)

Figure 228. Analog-to-digital conversion time in single conversion
ADC state

RDY

Analog channel

Sampling Ch(N)

Sampling Ch(N+1)
Ch(N+1)

Ch(N)

Internal S/H

Sample AIN(N)

Hold AIN(N)

(1)

tSAR(2)

tSMPL
ADSTART

Converting Ch(N)

Sample AIN(N+1)

Set by
SW

EOSMP

Set by
HW

Cleared
by SW
Set by
HW

EOC
ADC_DR

Data N-1

Cleared by
HW/SW
Data N

Indicative timings
MSv30532V2

1. tSMPL depends on SMP[2:0].
2. tSAR depends on RES[1:0].

33.4.18

Stopping an ongoing conversion (ADSTP, JADSTP)
The software can decide to stop regular conversions ongoing by setting ADSTP, and
injected conversions ongoing by setting JADSTP.
Stopping conversions resets the ongoing ADC operation. The ADC can then be
reconfigured (for example by changing the channel selection or the trigger). It is then ready
for a new operation.
Injected conversions can be stopped while regular conversions are still ongoing and vice
versa. This enables, for instance, to reconfigure the injected conversion sequence and
triggers while regular conversions are still ongoing (and vice versa).
When the ADSTP bit is set by software, any ongoing regular conversion is aborted with
partial result discarded (ADC_DR register is not updated with the current conversion).
When the JADSTP bit is set by software, any ongoing injected conversion is aborted with
partial result discarded (ADC_JDRy register is not updated with the current conversion).
The scan sequence is also aborted and reset (meaning that relaunching the ADC would
restart a new sequence).

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Analog-to-digital converter (ADC12)
Once this procedure is complete, ADSTP/ADSTART bits (in case of regular conversion), or
JADSTP/JADSTART bits (in case of injected conversion) are cleared by hardware. The
software must poll ADSTART (or JADSTART) until the bit is reset before assuming the ADC
is completely stopped.

Note:

In auto-injection mode (JAUTO = 1), setting the ADSTP bit aborts both regular and injected
conversions (JADSTP must not be used).
Figure 229. Stopping ongoing regular conversions
Trigger

Trigger
ADC state

RDY

Convert
Ch(N-1)

Sample
Ch(N-1)

RDY

Sample
Ch(N)

C

RDY

JADSTART
ADSTART

Set
by SW

Cleared
by HW

REGULAR CONVERSIONS ongoing
(The software is not allowed to configure regular
conversions selection and triggers)
Set
by SW

ADSTP
ADC_DR

Cleared
by HW

Data N-1

Data N-2

MSv62490V1

Figure 230. Stopping ongoing regular and injected conversions
Regular trigger
ADC state

RDY

Set by
SW

Sample
Ch(N-1)

Injected trigger
Convert
Ch(N-1)

RDY

Sample
Ch(M)

Regular trigger
C

RDY Sampl RDY

Cleared
by HW
(software is not allowed to configure injected conversions selection and triggers)
Cleared
Set by
by HW
SW
JADSTP

JADSTART

ADC_JDR

INJECTED CONVERSIONS ongoing

DATA M-1

Set by
Cleared
REGULAR CONVERSIONS ongoing
SW
by HW
(software is not allowed to configure regular conversions selection and triggers)
Set by Cleared
SW
by HW
ADSTP

ADSTART

ADC_DR

DATA N-2

DATA N-1
MS30534V1

RM0456 Rev 6

<!-- pagebreak -->

