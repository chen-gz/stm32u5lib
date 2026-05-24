1514

Operational amplifier (OPAMP)

RM0456

When the OPAMP output is no more needed, the OPAMP can be disabled to save power.
All the configurations previously set (including the calibration) are maintained while the
OPAMP is disabled.

38.3.2

Initial configuration
The OPAMP default configuration is a functional mode where the three IOs are connected to
external pins. In the default mode, the OPAMP uses the factory trimming values (see
“electrical characteristics” section of the datasheet for factory trimming conditions.Usually
the temperature is 30 °C and the voltage is 3 V). The trimming values can be adjusted (see
Section 38.3.5). The default configuration uses the normal mode, that provides the standard
performance. The OPALPM bit in OPAMPx_CSR can be set in order to switch the OPAMP
to low-power mode and reduced performance. Normal, low-power, and high-speed modes
characteristics are defined in the “electrical characteristics” section of the datasheet. Before
utilization, the OPA_RANGE bit in OPAMP1_CSR must be set to 1.
As soon as the OPAEN bit in OPAMPx_CSR is set, the OPAMP is functional. The two input
pins and the output pin are connected as defined in Section 38.3.3 and the default
connection settings can be changed.

Note:

The inputs and output pins must be configured in analog mode (default state) in the
corresponding GPIOx_MODER register.

38.3.3

Signal routing
The routing for the OPAMP pins is determined by the OPAMPx_CSR register.
The connections of the operational amplifiers (OPAMP1 and OPAMP2) are described in the
table below.
Table 364. Operational amplifier possible connections(1)

Signal

Pin

Internal

OPAMP1_VINM

PA1 or dedicated
pin(2)

OPAMP1_OUT or
PGA

OPAMP1_VINP

PA0

dac1_out1

Controlled by bit VP_SEL

OPAMP1_VOUT

PA3

ADC1_IN8
ADC2_IN8

The pin is connected when the OPAMP is
enabled. The ADC input is controlled by the ADC.

OPAMP2_VINM

PA7 or dedicated
pin(2)

OPAMP2_OUT or
PGA

OPAMP2_VINP

PA6

dac1_out2

Controlled by bit VP_SEL

PB0

ADC1_IN15
ADC2_IN15
ADC4_IN18

The pin is connected when the OPAMP is
enabled. The ADC input is controlled by the ADC.

OPAMP2_VOUT

comment
Controlled by OPAMODE and VM_SEL

Controlled by bits OPAMODE and VM_SEL

1. STM32U535/545 devices only embed OPAMP1.
2. The dedicated pin is only available on BGA132/169/216 packages. This configuration provides the lowest input bias current
(see datasheet).

<!-- pagebreak -->

