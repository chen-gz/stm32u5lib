RM0456 Rev 6

RM0456

Analog-to-digital converter (ADC4)

34.7.3

ADC control register (ADC_CR)
Address offset: 0x08
Reset value: 0x0000 0000

31

29

28

27

26

25

24

23

22

21

20

19

18

17

16

Res.

Res.

ADVR
EGEN

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

15

14

13

12

11

10

9

8

7

6

5

4

3

2

1

0

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

ADSTP

Res.

AD
START

ADDIS

ADEN

rs

rs

rs

ADCAL

30

rs

rw

rs

Bit 31 ADCAL: ADC calibration
This bit is set by software to start the calibration of the ADC.
It is cleared by hardware after calibration is complete.
0: Calibration complete
1: Write 1 to calibrate the ADC. Read at 1 means that a calibration is in progress.
Note: The software is allowed to set ADCAL only when the ADC is disabled (ADCAL = 0,
ADSTART = 0, ADSTP = 0, ADDIS = 0, AUTOFF = 0, and ADEN = 0).
The software is allowed to update the calibration factor by writing ADC_CALFACT only when
ADEN is set to 1 and ADSTART is cleared to 0 by writing ADSTP to 1 (ADC enabled and no
conversion is ongoing).
Bits 30:29 Reserved, must be kept at reset value.
Bit 28 ADVREGEN: ADC voltage regulator enable
This bit is set by software, to enable the ADC internal voltage regulator. The voltage regulator output
is available after tADCVREG_SETUP.
It is cleared by software to disable the voltage regulator. It can be cleared only if ADEN is et to 0.
0: ADC voltage regulator disabled
1: ADC voltage regulator enabled
Note: The software is allowed to program this bit field only when the ADC is disabled (ADCAL = 0,
ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0).
Bits 27:5 Reserved, must be kept at reset value.
Bit 4 ADSTP: ADC stop conversion command
This bit is set by software to stop and discard an ongoing conversion (ADSTP Command).
It is cleared by hardware when the conversion is effectively discarded and the ADC is ready to
accept a new start conversion command.
0: No ADC stop conversion command ongoing
1: Write 1 to stop the ADC. Read 1 means that an ADSTP command is in progress.
Note: To clear the A/D converter state, ADSTP must be set to 1 even if ADSTART is cleared to 0 after
the software trigger A/D conversion. It is recommended to set ADSTP to 1 whenever the
configuration needs to be modified.
Bit 3

Reserved, must be kept at reset value.

RM0456 Rev 6

<!-- pagebreak -->

1443

Analog-to-digital converter (ADC4)

RM0456

Bit 2 ADSTART: ADC start conversion command
This bit is set by software to start ADC conversion. Depending on the EXTEN [1:0] configuration bits,
a conversion either starts immediately (software trigger configuration) or once a hardware trigger
event occurs (hardware trigger configuration).
It is cleared by hardware:
– In single conversion mode (CONT = 0, DISCEN = 0), when software trigger is selected
(EXTEN = 00): at the assertion of the end of Conversion Sequence (EOS) flag.
– In discontinuous conversion mode(CONT=0, DISCEN = 1), when the software trigger is selected
(EXTEN = 00): at the assertion of the end of Conversion (EOC) flag.
– In all other cases: after the execution of the ADSTP command, at the same time as the ADSTP bit is
cleared by hardware.
0: No ADC conversion is ongoing.
1: Write 1 to start the ADC. Read 1 means that the ADC is operating and may be converting.
Note: The software is allowed to set ADSTART only when ADEN = 1 and ADDIS = 0 (ADC is enabled
and there is no pending request to disable the ADC).
Bit 1 ADDIS: ADC disable command
This bit is set by software to disable the ADC (ADDIS command) and put it into power-down state
(OFF state).
It is cleared by hardware once the ADC is effectively disabled (ADEN is also cleared by hardware at
this time).
0: No ADDIS command ongoing
1: Write 1 to disable the ADC. Read 1 means that an ADDIS command is in progress.
Note: Setting ADDIS to 1 is only effective when ADEN = 1 and ADSTART = 0 (which ensures that no
conversion is ongoing)
Bit 0 ADEN: ADC enable command
This bit is set by software to enable the ADC. The ADC is effectively ready to operate once the
ADRDY flag has been set.
It is cleared by hardware when the ADC is disabled, after the execution of the ADDIS command.
0: ADC is disabled (OFF state)
1: Write 1 to enable the ADC.
Note: The software is allowed to set ADEN only when all bits of ADC_CR registers are 0 (ADCAL = 0,
ADSTP = 0, ADSTART = 0, ADDIS = 0 and ADEN = 0)

<!-- pagebreak -->

