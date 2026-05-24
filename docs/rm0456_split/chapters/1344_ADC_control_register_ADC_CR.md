1377

Analog-to-digital converter (ADC12)

RM0456

Bit 1 EOSMPIE: End of sampling flag interrupt enable for regular conversions
This bit is set and cleared by software to enable/disable the end of the sampling phase interrupt for
regular conversions.
0: EOSMP interrupt disabled.
1: EOSMP interrupt enabled. An interrupt is generated when the EOSMP bit is set.
Note: Software is allowed to write this bit only when ADSTART = 0 (which ensures that no regular
conversion is ongoing).
Bit 0 ADRDYIE: ADC ready interrupt enable
This bit is set and cleared by software to enable/disable the ADC Ready interrupt.
0: ADRDY interrupt disabled
1: ADRDY interrupt enabled. An interrupt is generated when the ADRDY bit is set.
Note: Software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0 (which
ensures that no conversion is ongoing).

33.6.3

ADC control register (ADC_CR)
Address offset: 0x08
Reset value: 0x2000 0000

31
ADCA
L

30
Res.

rs
15
Res.

14
Res.

29

28

27

DEEP ADVREG
PWD
EN

26

25

24

CALINDEX[3:0]

Res.

rw

rw

rw

rw

rw

rw

13

12

11

10

9

8

Res.

Res.

Res.

Res.

Res.

23

Res.

22
Res.

21
Res.

20
Res.

19
Res.

18
Res.

17

16

Res.

ADCA
LLIN
rw

7
Res.

6
Res.

5

4

3

2

1

JADST ADSTA
ADDIS
JADSTP ADSTP
ART
RT
rs

rs

rs

rs

rs

Bit 31 ADCAL: ADC calibration
This bit is set by software to start the ADC calibration.
It is cleared by hardware after calibration is complete.
0: Calibration complete
1: Write 1 to calibrate the ADC. Read at 1 means that a calibration in progress.
Note: The software is allowed to launch a calibration by setting ADCAL only when ADEN = 0.
Bit 30 Reserved, must be kept at reset value.
Bit 29 DEEPPWD: Deep-power-down enable
This bit is set and cleared by software to put the ADC in Deep-power-down mode.
0: ADC not in deep-power down
1: ADC in Deep-power-down (default reset state)
Note: The software is allowed to write this bit only when the ADC is disabled (ADCAL = 0,
JADSTART = 0, JADSTP = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0).

<!-- pagebreak -->

RM0456 Rev 6

0
ADEN
rs

RM0456

Analog-to-digital converter (ADC12)

Bit 28 ADVREGEN: ADC voltage regulator enable
This bits is set by software to enable the ADC voltage regulator.
Before performing any operation such as launching a calibration or enabling the ADC, the ADC
voltage regulator must first be enabled and the software must wait for the regulator start-up time.
0: ADC Voltage regulator disabled
1: ADC Voltage regulator enabled.
For more details about the ADC voltage regulator enable and disable sequences, refer to
Section 33.4.6: ADC Deep-power-down mode (DEEPPWD) and ADC voltage regulator
(ADVREGEN).
The software can program this bitfield only when the ADC is disabled (ADCAL = 0, JADSTART = 0,
ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0).
Bits 27:24 CALINDEX[3:0]: Calibration factor
This bitfield controls the calibration factor to be read or written.
Calibration index 0 is dedicated to single-ended and differential offsets, calibration index 1 to 7 to the
linearity calibration factors, and index 8 to the internal offset:
0000: Offset calibration factor
0001: linearity calibration factor 1
0010: linearity calibration factor 2
0011: linearity calibration factor 3
0100: linearity calibration factor 4
0101: linearity calibration factor 5
0110: linearity calibration factor 6
0111: linearity calibration factor 7 and internal offset (write access only)
1000: internal offset (read access only)
1001: Calibration mode selection
Others: Reserved, must not be used
Note: ADC_CALFACT2[31:0] correspond to the location of CALINDEX[3:0] calibration factor data
(see Section 33.4.8: Calibration (ADCAL, ADCALLIN, ADC_CALFACT) for details).
Bits 23:17 Reserved, must be kept at reset value.
Bit 16 ADCALLIN: Linearity calibration
This bit is set and cleared by software to enable the linearity calibration.
0: Writing ADCAL launches a calibration without the linearity calibration.
1: Writing ADCAL launches a calibration with he linearity calibration.
Note: The software is allowed to write this bit only when the ADC is disabled and is not calibrating
(ADCAL = 0, JADSTART = 0, JADSTP = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and
ADEN = 0).
Bits 15:6 Reserved, must be kept at reset value.
Bit 5 JADSTP: ADC stop of injected conversion command
This bit is set by software to stop and discard an ongoing injected conversion (JADSTP Command).
It is cleared by hardware when the conversion is effectively discarded and the ADC injected
sequence and triggers can be re-configured. The ADC is then ready to accept a new start of injected
conversions (JADSTART command).
0: No ADC stop injected conversion command ongoing
1: Write 1 to stop injected conversions ongoing. Read 1 means that an ADSTP command is in
progress.
Note: The software is allowed to set JADSTP only when JADSTART = 1 and ADDIS = 0 (ADC is
enabled and eventually converting an injected conversion and there is no pending request to
disable the ADC).
In auto-injection mode (JAUTO = 1), setting ADSTP bit aborts both regular and injected
conversions (do not use JADSTP)

RM0456 Rev 6

<!-- pagebreak -->

1377

Analog-to-digital converter (ADC12)

RM0456

Bit 4 ADSTP: ADC stop of regular conversion command
This bit is set by software to stop and discard an ongoing regular conversion (ADSTP Command).
It is cleared by hardware when the conversion is effectively discarded and the ADC regular
sequence and triggers can be re-configured. The ADC is then ready to accept a new start of regular
conversions (ADSTART command).
0: No ADC stop regular conversion command ongoing
1: Write 1 to stop regular conversions ongoing. Read 1 means that an ADSTP command is in
progress.
Note: The software is allowed to set ADSTP only when ADSTART = 1 and ADDIS = 0 (ADC is
enabled and eventually converting a regular conversion and there is no pending request to
disable the ADC).
In auto-injection mode (JAUTO = 1), setting ADSTP bit aborts both regular and injected
conversions (do not use JADSTP).
Bit 3 JADSTART: ADC start of injected conversion
This bit is set by software to start ADC conversion of injected channels. Depending on the
configuration bits JEXTEN[1:0], a conversion starts immediately (software trigger configuration) or
once an injected hardware trigger event occurs (hardware trigger configuration).
It is cleared by hardware:
–
in single conversion mode when software trigger is selected (JEXTSEL = 0x0): at the
assertion of the end of injected conversion sequence (JEOS) flag.
–
in all cases: after the execution of the JADSTP command, at the same time as JADSTP is
cleared by hardware.
0: No ADC injected conversion is ongoing.
1: Write 1 to start injected conversions. Read 1 means that the ADC is operating and eventually
converting an injected channel.
Note: The software is allowed to set JADSTART only when ADEN = 1 and ADDIS = 0 (ADC is
enabled and there is no pending request to disable the ADC).
In auto-injection mode (JAUTO = 1), regular and auto-injected conversions are started by
setting bit ADSTART (JADSTART must be kept cleared)
Bit 2 ADSTART: ADC start of regular conversion
This bit is set by software to start ADC conversion of regular channels. Depending on the
configuration bits EXTEN[1:0], a conversion starts immediately (software trigger configuration) or
once a regular hardware trigger event occurs (hardware trigger configuration).
It is cleared by hardware:
–
In single conversion mode (CONT = 0, DISCEN = 0) when software trigger is selected
(EXTEN[1:0] = 0x0): at the assertion of the end of regular conversion sequence (EOS) flag.
–
In discontinuous conversion mode (CONT = 0, DISCEN = 1), when the software trigger is
selected (EXTEN[1:0] = 0x0): at the end of conversion (EOC) flag.
–
in all other cases: after the execution of the ADSTP command, at the same time that
ADSTP is cleared by hardware.
0: No ADC regular conversion is ongoing.
1: Write 1 to start regular conversions. Read 1 means that the ADC is operating and eventually
converting a regular channel.
Note: The software is allowed to set ADSTART only when ADEN = 1 and ADDIS = 0 (ADC is enabled
and there is no pending request to disable the ADC)
In auto-injection mode (JAUTO = 1), regular and auto-injected conversions are started by
setting bit ADSTART (JADSTART must be kept cleared)

<!-- pagebreak -->

