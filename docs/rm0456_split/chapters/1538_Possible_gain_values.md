1599

Multi-function digital filter (MDF)

RM0456

The table below shows the possible gain values.
Table 378. Possible gain values
SCALE[5:0]

Gain
(dB)

SCALE[5:0]

Gain
(dB)

SCALE[5:0]

Gain
(dB)

SCALE[5:0]

Gain
(dB)

0x20

- 48.2

0x2B

- 14.5

0x06

+ 18.1

0x11

+ 51.7

0x21

- 44.6

0x2C

- 12.0

0x07

+ 21.6

0x12

+ 54.2

0x22

- 42.1

0x2D

- 8.5

0x08

+ 24.1

0x13

+ 57.7

0x23

- 38.6

0x2E

- 6.0

0x09

+ 27.6

0x14

+ 60.2

0x24

- 36.1

0x2F

- 2.5

0x0A

+ 30.1

0x15

+ 63.7

0x25

- 32.6

0x00

0.0

0x0B

+ 33.6

0x16

+ 66.2

0x26

- 30.1

0x01

+ 3.5

0x0C

+ 36.1

0x17

+ 69.7

0x27

- 26.6

0x02

+ 6.0

0x0D

+ 39.6

0x18

+ 72.2

0x28

- 24.1

0x03

+ 9.5

0x0E

+ 42.1

-

-

0x29

- 20.6

0x04

+ 12.0

0x0F

+ 45.7

-

-

0x2A

- 18.1

0x05

+ 15.6

0x10

+ 48.2

-

-

The SAT blocks avoid having a wrap-around of the binary code when the code exceeds its
maximal or minimal value.
The MDF performs saturation operations at the following levels:
•

after the SCALE block (performed by the SAT block). The signal is saturated at 24 bits.

•

inside the RSFLT, to insure a good filter behavior

•

at the output of the HPF, to insure that the output signal does not exceed 24 bits

The SATF bit informs the application that a saturation occurred either after the SCALE,
inside the RSFLT or after the HPF. In addition, an interrupt can be generated if SATIE is set
to 1. As soon as a saturation is detected, the SATF flag is set to 1. It is up to the application
to clear this flag in order to be able to detect a new saturation.
Those bits are in the MDF DFLTx interrupt enable register x (MDF_DFLTxIER) and MDF
DFLTx interrupt status register x (MDF_DFLTxISR).

Gain adjustment policy
To get the best MDF performances, it is important to properly adjust the gain value via
SCALE[5:0].
A usual way to adjust the gain is to select the SCALE[5:0] value that gives a final signal
amplitude as close as possible to the 24-bit full scale, for the maximum input signal.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Multi-function digital filter (MDF)
A way to select the optimal gain is detailed below:
1.

Check that, for the expected input signal, the data size into the CIC filter does not
exceed 26 bits. This can be checked using this
formula:
N
LN ( SIN pp ⋅ D )
-------------------------------------------- < 26
LN ( 2 )

where N represents the CIC order, D the decimation ratio and SINpp the maximum
peak-to-peak amplitude of the input signal.
SINppcan take:
–

a maximum peak-to-peak amplitude of 2 (± 1), for samples coming from SITF

–

a maximum peak-to-peak amplitude of 4095 (+ 2047, - 2048) for samples coming
from a 12-bit ADC

Example: a Sinc4 can be used with a decimation ratio of 96, if the maximum input
signal does not exceed ± 0.35. Indeed:
4

LN ( 0.7 ⋅ 96 )
------------------------------------∼25.82 bits < 26 bits
LN ( 2 )

2.

Adjust the SCALE value.
To select the most appropriate SCALE value, the user must check if the RSFLT is used
or not. If the RSFLT is used, the data size at SCALE output must not exceed 22 bits,
otherwise the data size can be up to 24 bits.
The SCALE value in dB is selected using this formula:
NB
⎛
⎞
2
SCALE dB < 20 ⋅ log 10 ⎜ ------------------------------N-⎟
⎝ SIN pp ⋅ D ⎠

where NB is equal to 22 if RSFLT is enabled, or 24 if RSFLT is bypassed. SCALEdB
represents the gain value selected by SCALE[5:0].
Example: for a Sinc4 with a decimation ratio of 96 and a SINpp of 0.7:
–

If the RSFLT is bypassed:
⎛ 2 24 ⎞
20 ⋅ log 10 ⎜ ------------------------4⎟ ∼– 11 dB
⎝ 0.7 ⋅ 96 ⎠

SCALEdB value must be lower than - 11 dB. The closest lower value is - 12dB
(SCALE[5:0] = 0x2C).
–

If the RSFLT is enabled:
⎛ 2 22 ⎞
20 ⋅ log 10 ⎜ ------------------------4⎟ ∼– 23 dB
⎝ 0.7 ⋅ 96 ⎠

SCALEdB value must be lower than - 23 dB, the closest lower value is - 24.1 dB
(SCALE[5:0] = 0x28).
If SCALE[5:0] is set to a higher value, then a saturation may occur. An event flag
informs the user if a saturation occurred.
The table below proposes gain values for different filter configurations, when the data
comes from the SITFx, according to the CIC order, and the CIC decimation ratio. This table
is not exhaustive, and considers a full-scale input signal (see Section 39.7.5 for details).

RM0456 Rev 6

<!-- pagebreak -->

