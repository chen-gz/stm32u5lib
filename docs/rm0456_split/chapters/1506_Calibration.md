1514

Operational amplifier (OPAMP)

RM0456

Figure 326. PGA mode, internal gain setting (x2/x4/x8/x16), inverting input used for
filtering
STM32
GPIO
DAC_OUT

+
ADC

GPIO
-

Allows optional
low-pass
filtering (1)
Equivalent to

MS35327V1

1. The gain depends on the cut-off frequency.

38.3.5

Calibration
At startup, the trimming values are initialized with the preset ‘factory’ trimming value.
Each OPAMP offset can be trimmed by the user. Specific registers allow different trimming
values for normal and low-power modes.
The calibration purpose is to cancel as much as possible the OPAMP inputs offset voltage.
The calibration circuitry allows the inputs offset voltage to be reduced to less than
+/- 1.5 mV within stable voltage and temperature conditions.
For each OPAMP and each mode, two trimming values can be trimmed: one for N
differential pair and one for P differential pair.
The registers used to trim the offsets for each operational amplifiers are:
•

OPAMPx_OTR for normal mode

•

OPAMPx_LPOTR for low-power mode

Each register is composed of five bits for P differential pair trimming and five bits for N
differential pair trimming. These are the ‘user’ values.
The user is able to switch from ‘factory’ values to ‘user’ trimmed values, with the USERTRIM
bit in OPAMPx_CSR. This bit is reset at startup and the ‘factory’ value are applied by default
to the OPAMP trimming registers.
The trimming values can be changed in calibration or in functional mode.
The offset trimming registers are typically configured after the initialization of the calibration
operation (CALON set to 1 in OPAMPx_CSR). When CALON = 1, the OPAMP inputs are
disconnected from the functional environment.
Setting CALSEL to 1 in OPAMPx_CSR initializes the offset calibration for the P differential
pair (low-voltage reference used).

<!-- pagebreak -->

