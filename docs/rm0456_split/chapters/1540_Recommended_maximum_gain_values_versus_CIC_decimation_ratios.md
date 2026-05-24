1599

Multi-function digital filter (MDF)

RM0456

Table 379. Recommended maximum gain values versus CIC decimation ratios
Gain settings (dB) for configuration
SITF + CICx + RSFLT (+ HPF)

Gain settings (dB) for configuration
SITF + CICx (+ HPF)

CIC5

CIC4

CIC3

CIC5

CIC4

8

33.6

51.7

69.7

45.7

63.7

12

18.1

39.6

60.2

30.1

51.7

16

3.5

27.6

51.7

15.6

39.6

63.7

20

- 6.0

21.6

48.2

6.0

33.6

60.2

24

- 12.0

15.6

42.1

69.7

0

27.6

54.2

28

- 20.6

9.5

36.1

66.2

- 8.5

21.6

48.2

32

-26.6

3.5

33.6

63.7

- 14.5

15.6

45.7

48

-

- 8.5

24.1

57.7

-

3.5

31.6

69.7

64

-

- 20.6

15.6

51.7

-

-8.5

27.6

63.7

128

-

-

- 2.5

39.6

-

-

9.5

51.7

256

-

-

- 20.6

27.6

-

-

-8.5

39.6

CIC
decimation
ratio

CIC2

CIC1

72.2

72.2

CIC3

CIC2

CIC1

72.2

72.2

72.2

Reshaping filter (RSFLT)
In addition to the CIC, the MDF offers a reshaping IIR filter mainly dedicated to the audio
application but also usable in other applications.
When the RSFLT is used, the sample size at its input must not exceed 22 bits.
The samples at the RSFLT output can be decimated by four or not according to the RSFLTD
bit in the MDF reshape filter configuration register x (MDF_DFLTxRSFR).
The RSFLT can be bypassed by setting RSFBYP to 1 in MDF_DFLTxRSFR.
The table below shows which sampling rate must be provided to the RSFLT in order to
process the most common audio streams.
The RSFLT cutoff frequency (FC) depends on the sample rates at its input (FRS), and is
given by the following formula:
F C = 0.111 × F RS

Table 380. Most common microphone settings
Sample rate (kHz) at RSFLT (FRS) Pass band (kHz)

<!-- pagebreak -->

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

RM0456

Multi-function digital filter (MDF)
The figure below shows the frequency response of the reshape filter.

Magnitude (DB)

Figure 338. Reshape filter frequency response normalized (FRS / 2 = 1)

Magnitude (DB)

1RUPDOL]HGIUHTXHQF\ [ʌUDGVDPSOH

1RUPDOL]HGIUHTXHQF\ [ʌUDGVDPSOH

MSv62664V1

The RSFLT gain is about 9.3 dB, so the output data size is a little bit lower than 24 bits for a
22-bit wide input signal.
The RSFLT takes 24 clock cycles of mdf_proc_ck clock to process one sample at FRS.
When the RSFLT is enabled, the application must insure that the mdf_proc_ck is at least 24
times faster FRS.
The RSFLT generates an event (rfovr_evt) and sets the RFOVRF flag, if the RSFLT
receives a new sample while the previous one is still under processing.
When RFOVRF is set, the samples provided by the RSFLT are invalid. The application must
then stop the data acquisition and provide x a faster mdf_proc_ck clock to the RSFLT.

High-pass filter (HPF)
The high-pass filter suppresses the low-frequency content from the final output data stream
in case of continuous conversion mode. The high-pass filter can be enabled or disabled via
HPFBYP in the MDF reshape filter configuration register x (MDF_DFLTxRSFR).
The HPF is useful when there is parasitic low-frequency noise (or DC signal) in the input
data source and it must be removed from the final data.

RM0456 Rev 6

<!-- pagebreak -->

