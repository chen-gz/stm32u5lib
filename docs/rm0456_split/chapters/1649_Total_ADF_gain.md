RM0456 Rev 6

RM0456

Audio digital filter (ADF)
Figure 388. Detailed frequency response

40.7.5

Total ADF gain
This section details how to compute the signal level provided by the ADF according to the
filter settings.
A signal level may be expressed in dBFS (decibel full scale). A 0 dBFS level is assigned to
the maximum possible digital level. For example, a signal that reaches 50 % of the
maximum level, has a − 6 dBFS level (6 dB below full scale).
For example, for the ADF offering a final data width of 24 bits, a signal having an amplitude
of 2 * 106 LSB has a level of:
⎛ 2 × 10 6⎞
20 × log 10 ⎜ ------------------⎟ = – 12.45 dBFS
⎝ 2 ( 24 – 1 )⎠

In addition, the data size of a signal having an amplitude (Amp) expressed in LSB is given
by:
ln ( Amp ) + 1⎞ bits
DS = ⎛ ---------------------⎝ ln ( 2 )
⎠

One bit need to be added for negative values.
So a signal having an amplitude of 2 * 106 LSB, has a data size of 21.9 bits.

CIC gain
The CIC gain (GCIC and GdBCIC) can be deduced from the following formula giving data
size in bits (DSCIC).
DS CIC = ( N × log 2 ( D1 ) ) + DSin

where N represents the CIC order (selected by CICMOD[2:0]), and D1 is the decimation
ratio (given by MCICD[8:0]).
DSin represents the data size (in bits) of the signal at CIC input.

RM0456 Rev 6

<!-- pagebreak -->

1674

Audio digital filter (ADF)

RM0456

Warning:

DSCIC is very important for CIC filter. In order to work fine,
DSCIC must not exceed 26 bits.

The CIC gain GCIC is given by:
G CIC = ( D1 )

N

which gives in decibels:
N

GdB CIC = 20 × log 10 ( ( D1 ) )

Data size at SCALE output
The data size at SCALE output (including the CIC gain), is a key information as the RSFLT
starts to have some saturations, if the peak-to-peak signal amplitude at SCALE output is
higher than 22 bits.
If the RSFLT is bypassed, then a peak-to-peak signal amplitude of 24 bits is accepted.
The signal amplitude at SCALE output is:
N

Asout SCALE = D1 × 10

GdB SCALE
--------------------------20

× Asin DFLT

GdBSCALE represents the gain selected by SCALE[5:0], in dB.
AsoutSCALE is the signal amplitude at SCALE output (in LSB), and AsinDFLT is the signal
amplitude at CIC input (LSB).
ln ( Asout SCALE )
+1
DS SCALE = ------------------------------------------ln ( 2 )

The data size at SCALE output (DSSCALE) is expressed in bits.

RSFLT gain
The RSFLT gain in the useful bandwidth is typically 9.5 dB, but due to ripple a margin of
about ± 0.41 dB must be considered.
G RSFLT = 10

Note:

9.5 dB
----------------20

= 2.98 typical

The HPF filter has a gain of 0 dB.

SAD gain
The SAD is using only the 16 MSB on the signal, as a consequence, from the SAD point of
view, the truncation from 24 to 16 bits can be seen as an attenuation.

<!-- pagebreak -->

