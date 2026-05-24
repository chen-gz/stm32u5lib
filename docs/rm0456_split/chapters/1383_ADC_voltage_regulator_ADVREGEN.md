Source/destination

dac1_out1
dac1_out2

adc_trg0

tim1_trgo2

adc_trg1

tim1_oc4

adc_trg2

tim2_trgo

adc_trg3

tim15_trgo

adc_trg4

tim6_trgo

adc_trg5

lptim1_ch1

adc_trg6

lptim3_ch2

adc_trg7

exti15

RM0456 Rev 6

RM0456

34.4.3

Analog-to-digital converter (ADC4)

ADC voltage regulator (ADVREGEN)
The ADC has a specific internal voltage regulator which must be enabled and stable before
using the ADC.
The ADC internal voltage regulator can be enabled by setting ADVREGEN bit to 1 in the
ADC_CR register. The software must wait for the ADC voltage regulator startup time
(tADCVREG_SETUP) before launching a calibration or enabling the ADC. The LDO status can
be verified by checking the LDORDY bit in ADC_ISR register.
After ADC operations are complete, the ADC can be disabled (ADEN = 0). It is then
possible to save additional power by disabling the ADC voltage regulator (refer to Section :
ADC voltage regulator disable sequence).

Note:

When the internal voltage regulator is disabled, the internal analog calibration factor is reset,
and a new calibration must be performed.

ADC voltage regulator enable sequence
To enable the ADC voltage regulator, follow the sequence below:
1.

Clear the LDORDY bit in ADC_ISR register by programming this bit to 1.

2.

Set the ADVREGEN bit to 1 in ADC_CR register.

3.

Wait until LDORDY = 1 in the ADC_ISR register (LDORDY is set after the ADC voltage
regulator startup time). This can be handled by interrupt if the interrupt is enabled by
setting the LDORDYIE bit in the ADC_IER register.

ADC voltage regulator disable sequence
To disable the ADC voltage regulator, follow the sequence below:

34.4.4

1.

Make sure that the ADC is disabled (ADEN = 0).

2.

Clear ADVREGEN bit in ADC_CR register.

3.

Clear the LDORDY bit in ADC_ISR register by programming this bit to 1(optional),

Calibration (ADCAL)
The ADC has a calibration feature. During the procedure, the ADC calculates a calibration
factor which is internally applied to the ADC until the next ADC power-off. The application
must not use the ADC during calibration and must wait until it is complete.
The calibration must be performed before starting analog-to-digital conversion. It removes
the offset error which may vary from chip to chip due to process variation, supply voltage
and temperature.
The calibration is initiated by software by setting bit ADCAL to 1. It can be initiated only
when all the following conditions are met:
•

the ADC voltage regulator is enabled (ADVREGEN = 1 and LDORDY = 1),

•

the ADC is disabled (ADEN = 0), and

•

the auto-off mode is disabled (AUTOFF = 0).

ADCAL bit stays at 1 during all the calibration sequence. It is then cleared by hardware as
soon the calibration completes. After this, the calibration factor can be read from the
ADC_DR register (from bits 6 to 0).

RM0456 Rev 6

<!-- pagebreak -->

1443

Analog-to-digital converter (ADC4)

RM0456

The internal analog calibration is kept if the ADC is disabled (ADEN = 0). When the ADC
operating conditions change (VDDA changes are the main contributor to ADC offset
variations and temperature change to a lesser extend), it is recommended to re-run a
calibration cycle. It is recommended to recalibrate when VREF+ voltage changed more than
10%.
The calibration factor is lost in the following cases:
•

The power supply is removed from the ADC (for example when the product enters
Standby or VBAT mode).

•

The ADC peripheral is reset.

The calibration factor is lost each time power is removed from the ADC (for example when
the product enters Standby or VBAT mode). Still, it is possible to save and restore the
calibration factor by software to save time when re-starting the ADC (as long as temperature
and voltage are stable during the ADC power-down).
The calibration factor can be written if the ADC is enabled but not converting (ADEN = 1 and
ADSTART = 0). Then, at the next start of conversion, the calibration factor is automatically
injected into the analog ADC. This loading is transparent and does not add any cycle
latency to the start of the conversion.

Software calibration procedure
1.

Ensure that ADEN = 0, ADVREGEN = 1, AUTOFF = 0, DPD = 0, and DMAEN = 0.

2.

Set ADCAL = 1.

3.

Wait until ADCAL = 0 (or until EOCAL = 1). This can be handled by interrupt if the
interrupt is enabled by setting the EOCALIE bit in the ADC_IER register

4.

The calibration factor can be read from bits 6:0 of ADC_DR or ADC_CALFACT
registers.
Figure 279. ADC calibration
t CAB
ADCAL

ADC State

OFF

Startup

ADC_DR[6:0]
ADC_CALFACT[6:0]
by SW

CALIBRATE
0x00

OFF
CALIBRATION
FACTOR

by HW
MSv33703V2

1. Refer to the device datasheet for the value of tCAB.

<!-- pagebreak -->

