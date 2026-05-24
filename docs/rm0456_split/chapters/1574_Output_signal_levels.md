1599

Multi-function digital filter (MDF)

RM0456

The figure below shows a simplified view of the filter path, and gives for each significant
component the expression of the bit growth and the gain.
Figure 360. Simplified DFLT view with gain information
(1)

Gain (linear)
GSAD = 0.00391

GHPF = 1

GRSFLT = 2.98

GMCIC = D1N

GSSCALE =

DFLTx

HPF

PCM[23:0]

Ļ RSFLT

SAD

GDBSAD = - 48.1 dB

GDBHPF = 0 dB

SAT

GDBRSFLT = 9.5 dB

D1
љ

SCALE

CIC

Bin

GdBMCIC = 20 x log10(D1N)

GDBSCALE

Gain (dB)
(1) The SAD is not always implemented (see the ADF implementation section for details).

MSv62698V1

The table below summarizes the final data size for different filter configurations.
Table 387. Output signal levels
Filter configurations

Final signal level (dBFS)

Final signal size (bits)
DS OUT = DS SCALE + 1.6 bits

CIC + RSFLT (+ HPF)

DSSCALE must be lower than 22 bits.
DS

CIC + RSFLT (+ HPF) + INT

⎛ 2 OUT⎞
SdB OUT = 20 × log 10 ⎜ ----------------⎟
⎝ 2 24 ⎠

DS OUT = DS SCALE + BG INT + 1.6 bits

DSSCALE must be lower than 22 bits.
DS OUT = DS SCALE + BG INT

CIC (+ HPF) + INT

DSSCALE must be lower than 24 bits.

Example using the main filter chain
If the MDF filter is programmed as follows:
•

The input signal is coming from a serial interface (DSIN = 1 bit).

•

CIC order = 5 (N), with a decimation value of 24 (D1).

•

SCALE[5:0] is set to -12 dB.

•

RSFLT is enabled and the decimation by four is enabled.

•

HPF is enabled.

•

INT is disabled.

Check first the data size at CIC output:
DS CIC = ( 5 × log 2 ( 24 ) ) + 1 bit = 23.92 bits

The size is lower than 26 bits, so the CIC works in good conditions.

<!-- pagebreak -->

