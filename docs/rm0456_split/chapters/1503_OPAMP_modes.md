RM0456 Rev 6

RM0456

38.3.4

Operational amplifier (OPAMP)

OPAMP modes
The OPAMP inputs and outputs are all accessible on terminals. The amplifiers can be used
in the following configuration environments:
•

standalone mode (external gain setting mode)

•

follower configuration mode

•

PGA modes

The amplifier output pin is directly connected to the output pad to minimize the output
impedance. It cannot be used as a general purpose I/O, even if the amplifier is configured
as a PGA and only connected to the ADC channel.
The impedance of the signal must be maintained below a level that avoids the input leakage
to create significant artifacts (due to a resistive drop in the source). Refer to the “electrical
characteristics” section in the datasheet for further details.

Standalone mode (external gain setting mode)
The procedure to use the OPAMP in standalone mode is detailed below:
1.

Keep default value of OPAMPx_CSR and the default state of GPIOx_MODER.

2.

As soon as OPAEN is set in OPAMPx_CSR, the two input pins and the output pin are
connected to the operational amplifier.

This default configuration uses the factory trimming values and operates in normal mode
(standard performance). The OPAMP behavior can be changed with the following bits in
OPAMPx_CSR:
•

If OPALPM is set to 1, the OPAMP switches in low-power mode in order to save power.

•

If OPAHSM is set to 1, the OPAMP switches in high-speed mode in order to have high
slew rate.

•

If USERTRIM is set to 1, the input offset values can be trimmed.
Figure 323. Standalone mode: external gain setting mode
STM32
GPIO
DAC_OUT

+
ADC

GPIO
-

MS35324V1

RM0456 Rev 6

<!-- pagebreak -->

1514

Operational amplifier (OPAMP)

RM0456

Follower configuration mode
The procedure to use the OPAMP in follower mode is detailed below (all bits in
OPAMPx_CSR):

Note:

1.

Set OPAMODE[1:0] = 11 (internal follower).

2.

Clear VP_SEL to 0 (GPIO connected to OPAMPx_VINP, named VINP in this
document).

3.

As soon as OPAEN is set to 1, the signal on the VINP pin is copied to the
OPAMP_VOUT pin.

The pin corresponding to OPAMP_VINM is free for another usage.
The signal on the OPAMP output is also seen as an ADC input. As a consequence, the
OPAMP configured in follower mode can be used to perform impedance adaptation on input
signals before feeding them to the ADC input, assuming the input signal frequency is
compatible with the operational amplifier gain bandwidth specification.
Figure 324. Follower configuration
STM32
GPIO
DAC_OUT

+
ADC

GPIO
-

Always connected to
OPAMP output (can be
used during debug)
MS35325V1

Programmable gain amplifier mode
The procedure to use the OPAMP to amplify the amplitude of an input signal is presented
hereafter (all bits in OPAMPx_CSR):

Note:

<!-- pagebreak -->

1.

Set OPAMODE[1:0] to 10 (internal PGA enabled).

2.

Set PGA_GAIN[1:0] to the selected PGA gain (2, 4, 8 or 16) in OPAMPx_CSR.

3.

Set VM_SEL[1:0] to 1x in OPAMPx_CSR (inverting input not externally connected).

4.

Clear VP_SEL to 0 in OPAMPx_CSR (GPIO connected to VINP).

5.

As soon as OPAEN is set in OPAMPx_CSR, the signal on the OPAMP_VINP pin is
amplified by the selected gain and visible on the OPAMP_VOUT pin.

To avoid saturation, the input voltage must stay below VDDA divided by the selected gain.

RM0456 Rev 6

RM0456

Operational amplifier (OPAMP)
Figure 325. PGA mode, internal gain setting (x2/x4/x8/x16), inverting input not used
STM32
GPIO
DAC_OUT

+
ADC

GPIO
-

Always connected to
OPAMP output (can be
used during debug)
MS35326V1

Programmable gain amplifier mode with external filtering
The procedure to use the OPAMP to amplify the amplitude of an input signal, with an
external Set, is detailed below (all bits in OPAMPx_CSR):
1.

Configure OPAMODE[1:0] to 10 (internal PGA enabled).

2.

Set PGA_GAIN[1:0] to the selected PGA gain (2, 4, 8 or 16).

3.

Clear VM_SEL[1:0] to 00 or 01(GPIO connected to VINM).

4.

Clear VP_SEL to 0 (GPIO connected to VINP).

Any external connection on VINP can be used in parallel with the internal PGA. For
example, a capacitor can be connected between VOUT and VINM for filtering purpose (see
datasheet for the value of resistors used in the PGA resistor network).

RM0456 Rev 6

<!-- pagebreak -->

