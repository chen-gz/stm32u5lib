Gain settings (dB) for configuration
SITF + CICx + RSFLT (+ HPF)

Gain settings (dB) for configuration
SITF + CICx (+ HPF)

CIC5

CIC4

CIC5

CIC4

8

33.6

51.7

45.7

63.7

12

18.1

39.6

30.1

51.7

16

3.5

27.6

15.6

39.6

20

- 6.0

21.6

6.0

33.6

24

- 12.0

15.6

0

27.6

28

- 20.6

9.5

- 8.5

21.6

RM0456 Rev 6

RM0456

Audio digital filter (ADF)
Table 397. Recommended maximum gain values
versus CIC decimation ratios (continued)
CIC
decimation
ratio

Gain settings (dB) for configuration
SITF + CICx + RSFLT (+ HPF)

Gain settings (dB) for configuration
SITF + CICx (+ HPF)

CIC5

CIC4

CIC5

CIC4

32

- 26.6

3.5

- 4.5

15.6

48

-

- 8.5

-

3.5

64

-

- 20.6

-

- 8.5

Reshaping filter (RSFLT)
In addition to the CIC, the ADF offers a reshaping IIR filter mainly dedicated to the audio
application, but also usable in other applications.
When the RSFLT is used, the sample size at its input must not exceed 22 bits.
The samples at the RSFLT output can be decimated by four or not according to the RSFLTD
bit in the ADF reshape filter configuration register 0 (ADF_DFLT0RSFR).
The RSFLT can be bypassed by setting RSFBYP to 1 in the ADF reshape filter configuration
register 0 (ADF_DFLT0RSFR).
Table 398 shows which sampling rate must be provided to the RSFLT in order to process
the most common audio streams.
The RSFLT cutoff frequency (FC) depends on the sample rates at its input (FRS), and is
given by the following formula:
F C = 0.111 × F RS

Table 398. Most common microphone settings
Sample rate (kHz) at RSFLT (FRS) Pass band (kHz)

D2

PCM sampling rate (kHz)

32

3.55

4

8

64

7.1

4

16

128

14.2

4

32

192

21.3

4

48

RM0456 Rev 6

<!-- pagebreak -->

1674

Audio digital filter (ADF)

RM0456

Table 370 shows the frequency response of the reshape filter.

Magnitude (DB)

Figure 370. Reshape filter frequency response normalized (FRS / 2 = 1)

Magnitude (DB)

1RUPDOL]HGIUHTXHQF\ [ʌUDGVDPSOH

1RUPDOL]HGIUHTXHQF\ [ʌUDGVDPSOH

MSv62664V1

The RSFLT gain is close to 9.3 dB, so the output data size is a little bit lower than 24 bits for
a 22-bit wide input signal.
The RSFLT takes 24 clock cycles of adf_proc_ck clock to process one sample at FRS. When
the RSFLT is enabled, the application must insure that the adf_proc_ck is at least 24 times
faster FRS.
The RSFLT generates an event (rfovr_evt) and sets the RFOVRF flag, if the RSFLT
receives a new samples while the previous one is still under processing.
When RFOVRF is set, the samples provided by the RSFLT are invalid. The application must
then stop the data acquisition and provides a faster adf_proc_ck clock to the RSFLT.

High-pass filter (HPF)
The high-pass filter suppresses the low-frequency content from the final output data stream
in case of continuous conversion mode. The high-pass filter can be enabled or disabled via
HPFBYP in the ADF reshape filter configuration register 0 (ADF_DFLT0RSFR).
The HPF is useful when there is parasitic low-frequency noise (or DC signal) in the input
data source that must be removed from the final data.

<!-- pagebreak -->

