1674

Audio digital filter (ADF)

RM0456

Example using the main filter chain
If the ADF filter is programmed as follows:
•

The input signal is coming from a serial interface (DsinRSFLT = 1 bit).

•

CIC order = 5 (N), with a decimation value of 24 (D1).

•

SCALE[5:0] is set to - 12 dB.

•

RSFLT enabled, and the decimation by four is enabled.

•

HPF is enabled.

Check first the data size at CIC output:
DS CIC = ( 5 × log 2 ( 24 ) ) + 1 bit = 23.92 bits

The size is lower than 26 bits, so the CIC works in good conditions.
The data size at CIC output is very close to 24 bits, so the SCALE must be adjusted in order
to provide a 22-bit max signal to the RSFLT. An attenuation of 12 dB is needed.
Then the signal level provided to the RSFLT is:
5

Asout SCALE = 24 × 10

–
12
--------20

× 1 = 2.10

6

6

ln ( 2.10 ) + 1 = 21.93 bits
DS SCALE = -----------------------ln ( 2 )

If a higher gain is used, the RSFLT may saturate the output signal for strong input signals.
At the end, the final signal amplitude is:
5

Asout HPF = 24 × 10

–--------12
20

× 10

9.5
------20

× 1 = 5.9711 × 10

6

6

ln ( 5.9711 × 10 )- + 1⎞ = 23.51 bits
Dsout HPF = ⎛ -------------------------------------------⎝
⎠
ln ( 2 )

or:

40.7.6

⎛ 2 23.51⎞
SDB OUT = 20 × log 10 ⎜ -------------⎟ = – 2.84 dBFS
⎝ 2 24 ⎠

How to compute SAD thresholds
The SAD does not compute the RMS value of the converted signal, but the average of the
absolute values. As a consequence, the estimated level differs from the RMS value of the
signal:

<!-- pagebreak -->

•

For a sine signal having an RMS value of 1, the SAD computes a level of 0.9.

•

For a white or pink noise signal having an RMS value of 1, the SAD computes a level of
about 0.8.

RM0456 Rev 6

RM0456
Note:

Audio digital filter (ADF)
FRSIZE[2:0] has a big influence on the accuracy of the level estimation: big FRSIZE[2:0]
values give better results.

Threshold programming with SADMOD = 01
Consider the case of a sound capture where the application wants to wake up the system
when the captured sound is bigger than 63 dBSPL.
The sound capture can be performed with a digital microphone such as the MP45DT02.
The sensitivity of this microphone is typically -26 dBFS for an input signal of 94 dBSPL.
An acoustic signal at 63 dBSPL produces a digital signal of about:
- 26 dBFS - (94 - 63) = - 57 dBFS.
•

SCALE value adjustment
For this example, the filter configuration is the following:
–

CIC5 with a decimation by 16

–

RSFLT enabled with a decimation by 4

–

HPF enabled

A SCALE value of 3.5 dB is recommended for this configuration. As DFTL0 provides
samples only used for a sound detection (samples not provided to the application), a
bigger gain value can be applied: it increases the SAD accuracy and a saturation does
not affect the SAD behavior (for example, a SCALE value of 15.6 dB).
•

Input signal amplitude
The input signal is - 57 dBFS, corresponding to an amplitude
Asin = 10(-57/20) = 0.00141 LSB.

•

Signal level at SAD input
The total filter gain for the SAD is:

5

G SAD = 16 × 10

15.6
----------20

× 10

9.5
------20

1
3
× --------- = 73.68 × 10 or 97.3 dB
256

The signal amplitude received by the SAD is:
Ain SAD = 0.00141 × G SAD ∼104 LSB

The gain can be increased if the expected amplitude is too small. For the targeted
application, 104 LSB is fine.
If the input signal is expected to be a sine, the sound level for a signal amplitude of
104 LSB is:
Ain SAD × 2
SDLVL = ---------------------------------- × 0.9 ∼66 LSB
2

where 0.9 is the correction factor to apply with respect to the RMS value.
•

Program the trigger value

RM0456 Rev 6

<!-- pagebreak -->

1674

Audio digital filter (ADF)

RM0456

ANMIN and SNTHR must be programmed to trigger the SAD when the input signal
level reaches 66 LSB.
For SADMOD[1:0] = 01, the threshold value is given by:
THRH = ANMIN × 10

GdB SNTHR
---------------------------20

where GdBSNTHR represents the decibel value selected by SNTHR[3:0].
When SNTHR[3:0] = 6 dB for example, this formula becomes:
THRH = 2 × A NMIN

So ANMIN = THRH/2 = 66 /2 = 33 LSB.
In Figure 389, the trigger value (THRH in red) is fixed to 66 LSB. The input signal is at
- 65dBFS during 256 samples, then its value goes to - 55 dB for 256 samples, and
finally it is reduced to - 60 dBFS.
The blue curve is showing the sound level estimation (SDLVL) versus time. Fluctuation
on the estimated value can be observed due to windowing effect of FRSIZE samples.
The SAD DETECT state (when green signal is high) is maintained during four
additional frames due to hangover function value.
In this example ANSLP = FRSIZE = 3 (64 samples), LFRNB = 0 (2 frames),
HGOVR = 0 (4 frames), SNTHR = 1 (6 dB) and ANMIN = 33.
Figure 390. SAD example working with SADMOD = 01

Threshold programming with SADMOD = 1x
Consider the case of a sound capture where the application wants to wake up the system
when the captured sound is bigger than 57 dBSPL.
The sound capture can be performed with a digital microphone such as the MP45DT02. The
sensitivity of this microphone is typically - 26 dBFS for an input signal of 94 dBSPL.
An acoustic signal at 57 dBSPL produces a digital signal of about:
- 26 dBFS - (94-63) = - 63 dBFS.
•

Adjust SCALE value
For this example, the filter configuration is the following:
–

<!-- pagebreak -->

CIC4 with a decimation by 48

RM0456 Rev 6

RM0456

Audio digital filter (ADF)
–

RSFLT bypassed

–

HPF enabled

A SCALE value of 3.5 dB is recommended for this configuration. The samples provided
by DFTL0 are only used for a sound detection, without providing the samples to the
application, a bigger gain value can be provided: it increases the SAD accuracy and a
saturation does not affect the SAD behavior (for example, a SCALE value of 24 dB).
•

Input signal amplitude
The input signal is - 63 dBFS, corresponding to an amplitude:
Asin = 10(-63/20) = 0.000708 LSB.

•

Signal level at SAD input
The total filter gain for the SAD is:

4

G SAD = 48 × 10

24
-----20

3

× 0.003906 = 328.6 × 10 or 110.3 dB

The signal amplitude received by the SAD is:
Ain SAD = 0.000708 × G SAD ∼232 LSB

The gain can be increased if the expected amplitude is too small.
If the input signal is expected to be a sine, the sound level for a signal amplitude of
232 LSB is:
Ain SAD × 2
SDLVL = ---------------------------------- × 0.9 ∼148 LSB
2

where 0.9 is the correction factor to apply with respect to the RMS value.
Note:

ANLVL converges to average of SDLVL values, with a long constant time.
So SDLVL ~ ANLVL = 148 LSB for a constant input signal at 57 dBSPL.
•

Programming trigger value
For SADMOD = 1', the SAD compares the estimated ambient noise multiplied by the
gain selected by SNTHR[3:0] to ANMIN[12:0] * 4.
For simplification, SNTHR[3:0] is set to 1 (6 dB), meaning that ANLVL is multiplied by
two.
The SAD triggers if 2 * ANLVL > THRH.
In this mode,
THRH = 4 × A NMIN

So the SAD triggers if:
ANLVL > 2 × ANMIN

RM0456 Rev 6

<!-- pagebreak -->

