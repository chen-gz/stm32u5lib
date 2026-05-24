RM0456 Rev 6

RM0456

Analog-to-digital converter (ADC12)
The output data in differential mode is an unsigned data:
•

When VINP[i] = VREF-: VINN[i] = VREF+and output data = 0x0000 (14-bit resolution
mode),

•

When VINP[i] = VREF+: VINN[i] = VREF- and output data = 0x3FFF.
V INP – V INN
ADC_Full_Scale
Converted value = ------------------------------------------- × 1 + ----------------------------------2
V
REF+

When ADC is configured as differential mode, both inputs must be biased at VREF+ / 2
voltage.
For a complete description of how the input channels are connected, refer to Section 33.4.4:
ADC connectivity
Caution:

When channel “i” is configured in differential input mode, its negative input voltage is
connected to VINN[i-1].

33.4.8

Calibration (ADCAL, ADCALLIN, ADC_CALFACT)
The ADC provides an automatic calibration procedure that controls the whole calibration
sequence including the ADC power-on/off. During the procedure, the ADC calculates an
offset calibration factor for single-ended and differential mode. This factor includes the
internal offset and the linearity that are applied internally to the ADC until the next ADC
power-off. During the calibration procedure, the application must not use the ADC and must
wait until the calibration is complete.
The calibration is a prerequisite to any ADC operation. It removes the systematic errors that
may vary from chip to chip and enables to compensate offset and linearity deviation.
The offset calibration is the same for single-ended or differential channels.
The linearity correction must be done only once, regardless of single / differential
configuration:
•

Set ADCALLIN in ADC_CR before launching a calibration that runs the linearity
calibration simultaneously with the offset calibration or

•

Clear ADCALLIN in ADC_CR before launching a calibration that does not run the
linearity calibration but only the offset calibration.

The calibration is then initiated by software by setting the ADCAL bit. The calibration can
only be initiated when the ADC is disabled (ADEN = 0). ADCAL bit remains at 1 during all
the calibration sequence. It is cleared by hardware as soon the calibration completes. At this
time, the associated calibration factor is stored internally in the analog ADC.
The internal analog calibration is kept if the ADC is disabled (ADEN = 0). However, if the
ADC is disabled for extended periods of time, the temperature changes, or the reference
voltage is modified of more than 10%, it is recommended that a new offset calibration cycle
is run before enabling the ADC again.
The internal analog calibration is lost each time the ADC power is switched off (for example,
when the device enters Standby or VBAT mode). In this case, to avoid spending time
recalibrating the ADC, the calibration factor can be written again to the ADC analog block
without recalibrating, assuming that the software has previously saved the calibration factor
generated during the previous calibration.
RM0456 Rev 6

<!-- pagebreak -->

1377

Analog-to-digital converter (ADC12)

RM0456

The calibration factor can be written if the ADC is enabled and no calibration is ongoing
(ADEN = 1 and ADSTART = 0 and JADSTART = 0). Then, at the next start of conversion,
the calibration factor is injected into the analog ADC.
Refer to the device datasheet for the offset and linearity calibration time requirements.

Software procedure to calibrate the ADC
1.

Make sure DEEPPWD = 0, ADVREGEN = 1 and check that the ADC voltage regulator
startup time has elapsed (LDORDY = 1).

2.

Make sure ADEN = 0.

3.

Either enable the linearity calibration (ADCALLIN = 1) or disable it (ADCALLIN = 0).

4.

Make sure CAPTURE_COEF and LATCH_COEF in ADC_CALFACT are cleared.

5.

Set ADCAL in the ADC_CR register and wait until ADCAL = 0.

Figure 225 shows the ADC calibration timing diagram.
Note:

The software can launch the calibration only when ADEN = 0, LDORDY = 1 (ADC disabled
and LDO ready).
Single-ended and differential channels cannot be calibrated separately. The offset
calibration is performed within the same sequence.
Figure 225. ADC calibration

ADCALLIN

0: Linear calibrarion disable

1: Linear calibration enable

tCAB
ADCAL
ADC State

OFF

Startup

Internal calibration
factor

by S/W

OFF

Default factor

by H/W

Calibration factor

Indicative timings
MSv62470V3

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Analog-to-digital converter (ADC12)

Reading calibration factor procedure
Once the calibration is complete (ADCAL bit cleared by hardware), the calibration factor can
be read from the ADC_CALFACT2 register. The CALINDEX[0:3] bitfield of the ADC_CR
register can be incremented to access other calibration factors:

Note:

1.

Make sure DEEPPWD = 0, ADVREGEN = 1, and check that the ADC voltage regulator
startup time has elapsed (LDORDY = 1).

2.

Make sure that ADEN = 1.

3.

Set CAPTURE_COEF and clear LATCH_COEF in the ADC_CALFACT register.

4.

Select the calibration factor by setting CALINDEX[0:3] in ADC control register
(ADC_CR).

5.

Read the calibration factor from the ADC_CALFACT2 register.

6.

Repeat steps 4 and 5 for each required calibration factor.

7.

Clear the CAPTURE_COEF bit in the ADC_CALFACT register.

The software can access the calibration factor only when ADEN = 1, ADSTART = 0 and
JADSTART = 0 (ADC enabled and no conversion is ongoing).

Software procedure to reinject the calibration factor into the ADC

Note:

1.

Make sure ADEN = 1, ADSTART = 0 and JADSTART = 0 (ADC enabled and no
conversion is ongoing).

2.

Clear the CAPTURE_COEF and LATCH_COEF bits in the ADC_CALFACT register

3.

Set CALINDEX[3:0] to the targeted calibration index to be updated, then write the
calibration factor to CALFACT[31:0] in the ADC_CALFACT2 register.

4.

Repeat step 3 for all the calibration index bytes.

5.

Set the LATCH_COEF bit in the ADC_CALFACT register.

6.

Clear the LATCH_COEF bit in the ADC_CALFACT register.

The software is allowed to update the calibration factor only when ADEN = 1 and
ADSTART = 0 and JADSTART = 0 (ADC enabled and no conversion is ongoing).
If some of the calibration factor bytes are not written when the LATCH_COEF bit is set, the
calibration factor becomes the default value.

Calibration factor index
The calibration factors are stored in the analog block in an indexed way. Index 0b0000 and
0b1000 contain the offset calibration factor while index 0b0001 to 0b0110 contain the
linearity factor. The lower two bytes of index 0b0111 contain the linearity calibration factor.
The internal offset calibration factor must be programmed into byte 2 of index (0b0111).
However, it is read at byte 3 of index 0b1000. When programming or reinjecting the
calibration factor, make sure to use the correct indexes for read and write operations.
Refer to Table 310 for a summary.

RM0456 Rev 6

<!-- pagebreak -->

1377

Analog-to-digital converter (ADC12)

RM0456

Table 310. Calibration factor index
Calibration factor
CALINDEX[3:0]
values

Byte location
Reset value
Byte 3

0b0000

Byte 2

Byte 1

Differential offset

Byte 0

Single-end offset

0x036D 0648

0b0001

Linearity factor 1

0x0004 0000

0b0010

Linearity factor 2

0x0000 0200

0b0011

Linearity factor 3

0x0080 0001

0b0100

Linearity factor 4

0x2000 4000

0b0101

Linearity factor 5

0x09EB 13D6

0b0110

Linearity factor 6

0x027A 04F5

0b0111 (read)

Reserved

Reserved

0b0111 (write)

Reserved

Internal offset

0b1000 (read)

Internal offset

Reserved

Reserved

Reserved

0x8C02 7A00

0b1001

Reserved

Calibration mode

Reserved

Reserved

0x3000 1100

0b1000 (write)

Reserved

Reserved

Reserved

Reserved

-

Other values

Linearity factor 7

0x8000 016A

Reserved

-

When the calibration factor is latched, all the index bytes must be uploaded, otherwise the
non-uploaded index bytes are reset to their default value.

Extended calibration mode
To enhance the ADC performance, the extended calibration mode is implemented on some
product versions (see Section 33.3: ADC implementation).
Below the procedure to enable the extended calibration mode:

Note:

1.

Make sure that DEEPPWD = 0 and ADVREGEN = 1, and check that the ADC voltage
regulator startup time has elapsed (LDORDY = 1).

2.

Set ADCCALLIN in the ADC_CR register.

3.

Make sure CAPTURE_COEF and LATCH_COEF of ADC_CALFACT are cleared.

4.

Set ADEN and wait until ADRDY is set.

5.

Set CALINDEX[3:0] = 0b1001, then write CALFACT[31:0] = 0x0302 1100 in the
ADC_CALFACT2 register.

6.

Set the LATCH_COEF bit in the ADC_CALFACT register.

7.

Set the ADDIS and wait until the ADEN bit is cleared.

8.

Set the ADCAL in the ADC_CR register and wait until ADCAL bit is cleared.

Once the calibration is complete, the value of CALFACT[31:0] at CALINDEX[3:0] = 0b1001
is reset.
The values written to the ADC_CR (CALINDEX[3:0] and ADCALLIN), ADC_CALFACT, and
ADC_CALFACT2 registers are internally applied asynchronously from Fadc_ker_ck. As a
result, consecutive fast accesses to these registers might generate errors. To prevent errors

<!-- pagebreak -->

