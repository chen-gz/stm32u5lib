RM0456 Rev 6

0

0

0

0

INPSEL[3:0].

0

0

0

0

0

EN

Res.

Res.

Res.

0

0

EN

INPSEL[1:0]

0

0

Res.

0

Res.

0

INPSEL[3:0].

Res.

INPSEL[2:0]

0

Res.

0

Res.

0

WINMODE

0

Res.

0

WINMODE

0

0

0

Res.

0

0

WINOUT

BLANKSEL[4:0]

0

Res.

0

POLARITY

0

WINOUT

0

POLARITY

0

0

HYST[1:0]

PWRMODE[1:0]

0

HYST[1:0]

BLANKSEL[4:0]

PWRMODE[1:0]

Res.

Res.
Res.

0

Res.

0

Res.

Reset value

Res.

COMP2_CSR

Res.

0

Res.

0

Res.

Reset value

Res.

LOCK

VALUE

0x04

COMP1_CSR

LOCK

0x00

Register
name

VALUE

Offset

31
30
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

Table 363. COMP register map and reset values

0

RM0456

Operational amplifier (OPAMP)

38

Operational amplifier (OPAMP)

38.1

OPAMP introduction
STM32U535/545 devices embed one operational amplifier and
STM32U575/585/59x/5Ax/5Fx/5Gx devices embed two operational amplifiers with two
inputs and one output each. The three I/Os can be connected to the external pins, this
enables any type of external interconnections. The operational amplifier can be configured
internally as a follower or as an amplifier with a non-inverting gain ranging from 2 to 16.
The positive input can be connected to the internal DAC.
The output can be connected to the internal ADC.

38.2

38.3

OPAMP main features
•

Rail-to-rail input voltage range

•

Low input bias current

•

Low input offset voltage

•

Low-power mode

•

High-speed mode to achieve a better slew rate

•

Fast wake-up time

•

Gain bandwidth of 1.6 MHz

OPAMP functional description
The OPAMP has several modes.
Each OPAMP can be individually enabled, when disabled the output is high-impedance.
When enabled, it can be in calibration mode, all input and output of the OPAMP are then
disconnected, or in functional mode.
There are three functional modes: the low-power mode, the high-speed mode and the
normal mode. In functional mode the inputs and output of the OPAMP are connected as
described in the Section 38.3.3.

38.3.1

OPAMP reset and clocks
The operational amplifier clock is necessary for accessing the registers. When the
application does not need read or write access to those registers, the clock can be switched
off using the peripheral clock enable register (see OPAMPEN bit in RCC_APB3ENR).
The OPAEN bit in OPAMPx_CSR enables and disables the OPAMP operation. The OPAMP
registers configurations can be changed when the OPAEN bit is set in OPAMPx_CSR.
However it can create spurious effects (noise, glitch, overshoot or saturation). If the
configuration is changed, the application firmware must take care of these spurious effects
(such as ignore the ADC result on the OPAMP output).

RM0456 Rev 6

<!-- pagebreak -->

