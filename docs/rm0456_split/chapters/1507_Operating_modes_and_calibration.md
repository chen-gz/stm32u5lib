RM0456 Rev 6

RM0456

Operational amplifier (OPAMP)
Clearing CALSEL to 0 initializes the offset calibration for the N differential pair (high voltage
reference used).
When CALON = 1, the CALOUT bit in OPAMPx_CSR reflects the influence of the trimming
value selected by CALSEL and OPALPM. When the value of CALOUT switches between
two consecutive trimming values, this means that those two values are the best trimming
values. The CALOUT flag needs up to 1 ms after the trimming value is changed to become
steady (see tOFFTRIM max delay specification in the “electrical characteristics” section of the
datasheet).

Note:

The closer the trimming value is to the optimum trimming value, the longer it takes to
stabilize (with a maximum stabilization time remaining below 1 ms in any case).
When the calibration operation is done, OPAHSM must be cleared to 0 in OPAMPx_CSR.
Table 365. Operating modes and calibration
Control bits

Output

Mode
OPAEN

OPALPM

CALON

CALSEL

VOUT

CALOUT

Normal operating mode

1

0

0

X

Analog

0

Low-power mode

1

1

0

X

Analog

0

Power down

0

X

X

X

Z

0

Offset cal high for normal mode

1

0

1

0

Analog

X

Offset cal low for normal mode

1

0

1

1

Analog

X

Offset cal high for low-power mode

1

1

1

0

Analog

X

Offset cal low for low-power mode

1

1

1

1

Analog

X

Calibration procedure
Here are the steps to perform a full calibration of either one of the operational amplifiers:
1.

Set OPAEN to 1 in OPAMPx_CSR to enable the OPAMP and set OPA_RANGE = 1 in
OPAMP1_CSR.

2.

Clear OPAHSM to 0 in OPAMPx_CSR.

3.

Set CALON and USERTRIM to 1 in OPAMPx_CSR.

4.

Choose a calibration mode (refer to Table 365). Steps 4 to 5 must be repeated four
times. For the first iteration, select normal mode, offset cal high (N differential pair), with
OPALPM = 0 and CALSEL = 0 in OPAMPx_CSR.

5.

Increment TRIMOFFSETN[4:0] in OPAMPx_OTR starting from 0, until CALOUT
changes to 1 in OPAMPx_CSR.
Between the write to OPAMP_OTR and the read of the CALOUT value, make sure to
wait for the tOFFTRIM max delay specified in the “electrical characteristics” section of the
datasheet, to get the correct CALOUT value.
The commutation means that the offset is correctly compensated and that the
corresponding trim code must be saved in OPAMP_OTR.

RM0456 Rev 6

<!-- pagebreak -->

