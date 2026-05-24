1443

Analog-to-digital converter (ADC4)

34.4.8

RM0456

Configuring the ADC
The software can write to the ADCAL and ADEN bits in the ADC_CR and ADC_PWRR
register if the ADC is disabled (ADEN must be 0).
The software must only write to the ADSTART and ADDIS bits in the ADC_CR register only
if the ADC is enabled and there is no pending request to disable the ADC (ADEN = 1 and
ADDIS = 0).
For all the other control bits in the ADC_IER, ADC_CFGRi, ADC_SMPR, ADC_CHSELR
and ADC_CCR registers, refer to the description of the corresponding control bit in
Section 34.7: ADC registers. If the ADC operates in software trigger mode, set the ADSTP
bit in ADC_CR register, then wait until ADSTP bit become 0 before reconfiguring the above
registers.
ADC_AWDTRi registers can be modified when a conversion is ongoing.
The software must only write to the ADSTP bit in the ADC_CR register if the ADC is enabled
(and possibly converting) and there is no pending request to disable the ADC
(ADSTART = 1 and ADDIS = 0).

Note:

There is no hardware protection preventing software from making write operations forbidden
by the above rules. If such a forbidden write access occurs, the ADC may enter an
undefined state. To recover correct operation in this case, the ADC must be disabled (clear
ADEN = 0 and all the bits in the ADC_CR register).

34.4.9

Channel selection (CHSEL, SCANDIR, CHSELRMOD)
There are up to 25 multiplexed channels:
•

up to 19 analog inputs from GPIO pins (ADC_INx)

•

6 internal analog inputs: temperature Sensor, internal reference voltage, VCORE, VBAT
channel, DAC internal channels

It is possible to convert a single channel or a sequence of channels.
The sequence of the channels to be converted can be programmed in the ADC_CHSELR
channel selection register: each analog input channel has a dedicated selection bit
(CHSELx).
The ADC scan sequencer can be used in two different modes:
•

Sequencer not fully configurable:
The order in which the channels are scanned is defined by the channel number
(CHSELRMOD bit must be cleared in ADC_CFGR1 register):

<!-- pagebreak -->

–

Sequence length configured through CHSELx bits in ADC_CHSELR register

–

Sequence direction: the channels are scanned in a forward direction (from the
lowest to the highest channel number) or backward direction (from the highest to
the lowest channel number) depending on the value of SCANDIR bit
(SCANDIR = 0: forward scan, SCANDIR = 1: backward scan)

RM0456 Rev 6

RM0456

Analog-to-digital converter (ADC4)
–
•

Any channel can belong to in these sequences

Fully-configurable sequencer
The CHSELRMOD bit is set in ADC_CFGR1 register.
–

Sequencer length is up to eight channels

–

The order in which the channels are scanned is independent from the channel
number. Any order can be configured through SQ1[3:0] to SQ8[3:0] bits in
ADC_CHSELR register.

–

Only 15 channels can be selected in this sequence (refer to Section 34.7.10: ADC
channel selection register [alternate] (ADC_CHSELR).

–

If the sequencer detects SQx[3:0] = 0b1111, the following SQx[3:0] registers are
ignored.

–

If no 0b1111 is programmed in SQx[3:0], the sequencer scans full eight channels.

The software is allowed to program the CHSEL, SCANDIR and CHSELRMOD bit only when
ADSTART bit is cleared in ADC_CR register. This ensures that no conversion is ongoing. If
the ADC operated in software trigger mode, set ADSTP bit then wait until ADSTP bit
become 0 before reconfiguring these registers. This sequence must be respected even if
ADSTART bit is cleared to 0 after the conversion,

Temperature sensor, DAC output, VREFINT, VBAT, and VCORE internal channels
The temperature sensor, the internal DAC channels, the internal reference voltage
(VREFINT), VBAT, and VCORE are connected to ADC internal channels. Refer to Table ADC
interconnection in Section 34.4.2: ADC pins and internal signals for details.

34.4.10

Programmable sampling time (SMPx[2:0])
Before starting a conversion, the ADC needs to establish a direct connection between the
voltage source to be measured and the embedded sampling capacitor of the ADC. This
sampling time must be enough for the input voltage source to charge the sample and hold
capacitor to the input voltage level.
Having a programmable sampling time allows the conversion speed to be trimmed
according to the input resistance of the input voltage source.
The ADC samples the input voltage for a number of ADC clock cycles that can be modified
using the SMP1[2:0] and SMP2[2:0] bits in the ADC_SMPR register.
Each channel can choose one out of two sampling times configured in SMP1[2:0] and
SMP2[2:0] bitfields, through SMPSELx bits in ADC_SMPR register.
The total conversion time is calculated as follows:
tCONV = Sampling time + 12.5 x ADC clock cycles
Example:
With ADC_CLK = 16 MHz and a sampling time of 1.5 ADC clock cycles:
tCONV = 1.5 + 12.5 = 14 ADC clock cycles = 0.875 µs
The ADC indicates the end of the sampling phase by setting the EOSMP flag.

I/O analog switch voltage booster
The resistance of the I/O analog switch increases when the VDDA voltage is too low. The
sampling time must consequently be adapted accordingly (refer to the device datasheet for

RM0456 Rev 6

<!-- pagebreak -->

1443

Analog-to-digital converter (ADC4)

RM0456

the corresponding electrical characteristics). This resistance can be minimized at low VDDA
voltage by enabling an internal voltage booster through the BOOSTEN bit of the
SYSCFG_CFGR1 register or by selecting a VDD booster voltage through the ANASWVDD
bit of the SYSCFG_CFGR1 register.

34.4.11

Single conversion mode (CONT = 0)
In single conversion mode, the ADC performs a single sequence of conversions, converting
all the channels once. This mode is selected when CONT is cleared in the ADC_CFGR1
register. Conversion is started by either:
•

Setting the ADSTART bit in the ADC_CR register

•

Hardware trigger event

Inside the sequence, after each conversion is complete:
•

The converted data are stored in the 16-bit ADC_DR register

•

The EOC (end of conversion) flag is set

•

An interrupt is generated if the EOCIE bit is set

After the sequence of conversions is complete:
•

The EOS (end of sequence) flag is set

•

An interrupt is generated if the EOSIE bit is set

Then the ADC stops until a new external trigger event occurs or the ADSTART bit is set
again.
Note:

To convert a single channel, program a sequence with a length of 1.

34.4.12

Continuous conversion mode (CONT = 1)
In continuous conversion mode, when a software or hardware trigger event occurs, the ADC
performs a sequence of conversions, converting all the channels once and then
automatically re-starts and continuously performs the same sequence of conversions. This
mode is selected when CONT is set to 1 in the ADC_CFGR1 register. Conversion is started
by either:
•

Setting the ADSTART bit in the ADC_CR register

•

Hardware trigger event

Inside the sequence, after each conversion is complete:
•

The converted data are stored in the 16-bit ADC_DR register

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
forbidden to set both bits DISCEN = 1 and CONT = 1.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

34.4.13

Analog-to-digital converter (ADC4)

Starting conversions (ADSTART)
Software starts ADC conversions by setting ADSTART to 1.
When ADSTART is set, the conversion:
•

Starts immediately if EXTEN = 00 (software trigger)

•

At the next active edge of the selected hardware trigger if EXTEN ≠ 00

The ADSTART bit is also used to indicate whether an ADC operation is currently ongoing. It
is possible to re-configure the ADC while ADSTART remains at 0, indicating that the ADC is
idle.
The ADSTART bit is cleared by hardware:
•

In single mode with software trigger (CONT = 0, EXTEN = 00)
–

•

In discontinuous mode with software trigger (CONT = 0, DISCEN = 1, EXTEN = 00)
–

•

At any end of conversion sequence (EOS = 1)
At end of conversion (EOC = 1)

In all cases (CONT = x, EXTEN = XX)
–

After execution of the ADSTP procedure invoked by software (see
Section 34.4.15: Stopping an ongoing conversion (ADSTP)).

When the ADC operates in autonomous mode (DPD bit transition from 1 to 0, see
Autonomous mode (AUTOFF, DPD)), the ADSTART bit can be set only when the ADC is
powered on. (both LDORDY = 1 and ADRDY = 1). In continuous mode (CONT = 1), the
ADSTART bit is not cleared by hardware when the EOS flag is set because the sequence is
automatically relaunched.
Note:

When hardware trigger is selected in single mode (CONT = 0 and EXTEN = 01), ADSTART
is not cleared by hardware when the EOS flag is set. This avoids the need for software
having to set the ADSTART bit again and ensures the next trigger event is not missed.
It is necessary to set ADSTP to 1 and wait until ADSTP is cleared before reconfiguring or
disabling the ADC, even if ADSTART bit is cleared to after the software triggered ADC
conversion mode.

RM0456 Rev 6

<!-- pagebreak -->

1443

Analog-to-digital converter (ADC4)

34.4.14

RM0456

Timings
The elapsed time between the start of a conversion and the end of conversion is the sum of
the configured sampling time plus the successive approximation time depending on data
resolution:
tCONV = tSMPL + tSAR = [1.5 |min + 12.5 |12bit] x tADC_CLK
tCONV = tSMPL + tSAR = 42.9 ns |min + 357.1 ns |12bit = 0.400 µs |min (for fADC_CLK = 35 MHz)

Figure 284. Analog-to-digital conversion time
ADC state

Sampling Ch(N)

RDY

Analog channel

Sampling Ch(N+1)
Ch(N+1)

Ch(N)

Internal S/H

Hold AIN(N)

Sample AIN(N)
tSMPL

ADSTART

Converting Ch(N)

Sample AIN(N+1)

(2)

(1)

tSAR

Set by
SW
Cleared
by SW

Set by
HW

EOSMP

Cleared by
HW/SW

Set by
HW

EOC
ADC_DR

Data N-1

Data N

Indicative timings
MSv30532V2

1. tSMPL depends on SMP[2:0].
2. tSAR depends on RES[2:0].

Figure 285. ADC conversion timings

ADSTART(1)

ADC state

tLATENCY (2)

Ready

S0

Conversion 0

S1

WLATENCY

Conversion 1
(3)

S2

Conversion 2

WLATENCY

(3)

S3

Conversion 3

WLATENCY (3)

ADC_DR
Data 0

Data 1

Data 2

MSv33174V1

1. EXTEN = 00 or EXTEN ≠ 00.
2. Trigger latency (refer to datasheet for more details).
3. ADC_DR register write latency (refer to datasheet for more details).

<!-- pagebreak -->

RM0456 Rev 6

RM0456

34.4.15

Analog-to-digital converter (ADC4)

Stopping an ongoing conversion (ADSTP)
The software can decide to stop any ongoing conversions by setting ADSTP to 1 in the
ADC_CR register.
This resets the ADC operation and the ADC is idle, ready for a new operation.
When the ADSTP bit is set by software, any ongoing conversion is aborted and the result is
discarded (ADC_DR register is not updated with the current conversion).
The scan sequence is also aborted and reset (meaning that restarting the ADC would restart a new sequence).
Once this procedure is complete, the ADSTP and ADSTART bits are both cleared by
hardware and the software must wait until ADSTART is cleared to 0 before starting new
conversions.
Figure 286. Stopping an ongoing conversion

ADC state

RDY

ADSTART

set by
SW

SAMPLING CH(N)

RDY

CONVERTING CH(N)

cleared by HW

set by SW

ADSTOP
ADC_DR

cleared by HW

DATA N-1
MSv30337V2

34.4.16

Conversion on external trigger and trigger polarity
(EXTSEL, EXTEN)
A conversion or a sequence of conversion can be triggered either by software or by an
external event (for example timer capture). If the EXTEN[1:0] control bits are not equal to
“0b00”, then external events are able to trigger a conversion with the selected polarity. The
trigger selection is effective once software has set bit ADSTART to 1.
Any hardware triggers which occur while a conversion is ongoing are ignored.
If bit ADSTART is cleared, any hardware triggers which occur are ignored.
Table 332 provides the correspondence between the EXTEN[1:0] values and the trigger
polarity.
Table 332. Configuring the trigger polarity
Source

EXTEN[1:0]

Trigger detection disabled

00

Detection on rising edge

01

Detection on falling edge

10

Detection on both rising and falling edges

11

RM0456 Rev 6

<!-- pagebreak -->

1443

Analog-to-digital converter (ADC4)
Note:

RM0456

The polarity of the external trigger can be changed only when the ADC is not converting
(ADSTART = 0).
The EXTSEL[2:0] control bits are used to select which of 8 possible events can trigger
conversions.
Refer to Table ADC interconnection in Section 34.4.2: ADC pins and internal signals for the
list of all the external triggers that can be used for regular conversion.
The software source trigger events can be generated by setting the ADSTART bit in the
ADC_CR register.

Note:

The trigger selection can be changed only when the ADC is not converting (ADSTART = 0).

34.4.17

Discontinuous mode (DISCEN)
This mode is enabled by setting the DISCEN bit in the ADC_CFGR1 register.
In this mode (DISCEN = 1), a hardware or software trigger event is required to start each
conversion defined in the sequence. On the contrary, if DISCEN is cleared, a single
hardware or software trigger event successively starts all the conversions defined in the
sequence.
Example:
•

•

DISCEN = 1, channels to be converted are channels 0, 3, 7 and 10
–

1st trigger: channel 0 is converted and an EOC event is generated

–

2nd trigger: channel 3 is converted and an EOC event is generated

–

3rd trigger: channel 7 is converted and an EOC event is generated

–

4th trigger: channel 10 is converted and both EOC and EOS events are
generated.

–

5th trigger: channel 0 is converted an EOC event is generated

–

6th trigger: channel 3 is converted and an EOC event is generated

–

...

DISCEN = 0, channels to be converted are channels 0, 3, 7 and 10
–

1st trigger: the complete sequence is converted: channel 0, then 3, 7 and 10. Each
conversion generates an EOC event and the last one also generates an EOS
event.

–

Any subsequent trigger events restarts the complete sequence.

Note:

It is not possible to have both discontinuous mode and continuous mode enabled: it is
forbidden to set both bits DISCEN = 1 and CONT = 1.

34.4.18

Programmable resolution (RES) - fast conversion mode
It is possible to obtain faster conversion times (tSAR) by reducing the ADC resolution.
The resolution can be configured to be either 12, 10, 8, or 6 bits by programming the
RES[1:0] bits in the ADC_CFGR1 register. Lower resolution allows faster conversion times
for applications where high data precision is not required.

Note:

The RES[1:0] bit must only be changed when the ADEN bit is reset.
The result of the conversion is always 12 bits wide and any unused LSB bits are read as
zeros.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Analog-to-digital converter (ADC4)
Lower resolution reduces the conversion time needed for the successive approximation
steps as shown in Table 333.
Table 333. tSAR timings depending on resolution
RES[1:0]
bits

34.4.19

tSAR
(ADC clock
cycles)

tSAR (ns) at
fADC = 35 MHz

tSMPL (min)

tCONV

(ADC clock
cycles)

(ADC clock cycles)
(with min. tSMPL)

tCONV (ns) at
fADC = 35 MHz

12

12.5

357

1.5

14

400

10

10.5

300

1.5

12

343

8

8.5

243

1.5

10

286

6

6.5

186

1.5

8

229

End of conversion, end of sampling phase (EOC, EOSMP flags)
The ADC indicates each end of conversion (EOC) event.
The ADC sets the EOC flag in the ADC_ISR register as soon as a new conversion data
result is available in the ADC_DR register. An interrupt can be generated if the EOCIE bit is
set in the ADC_IER register. The EOC flag is cleared by software either by writing 1 to it, or
by reading the ADC_DR register.
The ADC also indicates the end of sampling phase by setting the EOSMP flag in the
ADC_ISR register. The EOSMP flag is cleared by software by writing1 to it. An interrupt can
be generated if the EOSMPIE bit is set in the ADC_IER register.
The aim of this interrupt is to allow the processing to be synchronized with the conversions.
Typically, an analog multiplexer can be accessed in hidden time during the conversion
phase, so that the multiplexer is positioned when the next sampling starts.

Note:

As there is only a very short time left between the end of the sampling and the end of the
conversion, it is recommenced to use polling or a WFE instruction rather than an interrupt
and a WFI instruction.

RM0456 Rev 6

<!-- pagebreak -->

