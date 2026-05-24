•

CIC order 4 or 5, with a decimation ratio of 16

•

RSFLT enabled, with a decimation ratio of 4

•

HPF enable with a cut-off frequency of 40 Hz

RM0456 Rev 6

RM0456

Multi-function digital filter (MDF)
The figure below shows the theoretical frequency response using a CIC4 and a CIC5.
Figure 358. Global frequency response

Figure 359 shows the in-band ripple for a 16 kHz audio signal with a digital microphone
working at 1.024 MHz. The filter configuration is the following:
•

CIC order 4 or 5, with a decimation ratio of 16

•

RSFLT enabled, with a decimation ratio of 4

•

HPF enable with a cut-off frequency of 20 Hz

The resulting in-band ripple is ± 0.41 dB for CIC5, and ± 0.45 for CIC4.
The -3 dB cut-off frequency is 7061 Hz.
Figure 359. Detailed frequency response

39.7.5

Total MDF gain
This section details how to compute the signal level provided by the MDF according to the
filter settings. The formula does not take into account the filters transfer function.
A signal level may be expressed in dBFS (decibel full scale). A 0 dBFS level is assigned to
the maximum possible digital level. For example, a signal that reaches 50 % of the
maximum level, has a - 6 dBFS level (6 dB below full scale).
For example, for the MDF offering a final data width of 24 bits, a signal having an amplitude
of 2 * 106 LSB has a level of:
⎛ 2 × 10 6⎞
20 × log 10 ⎜ ------------------⎟ = – 12.45 dBFS
⎝ 2 ( 24 – 1 )⎠

RM0456 Rev 6

<!-- pagebreak -->

1599

Multi-function digital filter (MDF)

RM0456

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
DS CIC = ( N × log 2 ( D1 ) ) + DS IN

And the bit growth is:
BG CIC = ( N × log 2 ( D1 ) )

where N represents the CIC order (selected by CICMOD[2:0]), and D1 is the decimation
ratio (given by MCICD).
DSIN represents the data size (in bits) of the input signal.

Warning:

G CIC = 2

DSCIC is very important for CIC filters. In order to work fine,
DSCIC must not exceed 26 bits.

( BG CIC )

= ( D1 )

N

which gives, in decibels:
N

GdB CIC = 20 × log 10 ( ( D1 ) )

Note:

The same formulas are valid for the ACIC.

Data size at SCALE output
The data size at SCALE output (including the CIC gain) is a key information as the RSFLT
starts to have some saturations if the peak-to-peak signal amplitude at SCALE output is
higher than 22 bits.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Multi-function digital filter (MDF)
If the RSFLT is bypassed, then a peak-to-peak signal amplitude of 24 bits is accepted. The
resulting data size is given by:
GDB SCALE

⎛ ---------------------------⎞
20
DS SCALE = N × log 2 ( D1 ) + log 2 ⎜ 10
⎟ + DS IN
⎝
⎠

The data size at SCALE output (DSSCALE) is expressed in bits and GdBSCALE represents
the gain selected by SCALE[5:0], in dB.

RSFLT gain
The RSFLT gain in the useful bandwidth is typically 9.5 dB, but due to ripple a margin of
about ± 0.5 dB must be considered. Typically, the RSFLT increases the bit size by BGRSFLT:
BG RSFLT = 10

9.5
dB
----------------20

= 2.98 = 1.6 bits

INT gain
The INT block can also introduce a gain if the rescaling value is different from the integration
value.
G INT = IVAL
------------IDIV

and:
IVAL
GdB INT = 20 × log 10 ⎛ -------------⎞
⎝ IDIV ⎠

The bit growth of the INT is then given by the following formula:
IVAL
BG INT = log 2 ⎛ -------------⎞
⎝ IDIV ⎠

IVAL represents the integration value selected by INTVAL[6:0], and IDIV represents the
integrator output division selected by INTDIV[1:0].
Note:

The HPF filter has a gain of 0 dB.

RM0456 Rev 6

<!-- pagebreak -->

