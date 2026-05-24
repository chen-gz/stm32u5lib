RM0456 Rev 6

RM0456

Multi-function digital filter (MDF)
Table 376. Data size according to CIC order and CIC decimation values
Data size (bits) when DSIN = 1 bit
(data from SITFx)

Decimation

Data size (bits) when DSIN = 12 bits
(data from ADCITF)

Sinc1 Sinc2 FastSinc Sinc3 Sinc4 Sinc5 Sinc1 Sinc2 FastSinc Sinc3 Sinc4 Sinc5
64

7

13

14

19

25

-

18

24

25

-

-

-

128

8

15

16

22

-

-

19

26

-

-

-

-

256

9

17

18

25

-

-

20

-

-

-

-

-

512

10

19

20

-

-

-

21

-

-

-

-

-

The LSB part of the data provided by the CIC is not necessarily significant: it depends on the
sensor performances and the ability of the CIC to reject the out-off-band noise.
The sample size at CIC output can be adjusted thanks to the SCALE block.
The table below shows the maximum allowed decimation ratio for the CIC filter, depending
on the input data size. Bigger decimation ratio causes a wrap-around of the signal at CIC
output, for strong input signals.
Note:

The MDF cannot detect or prevent a CIC wrap-around.
Table 377. Maximum decimation ratio versus order and input data size

Filter order

Max. decimation ratio when
DSIN = 1 bit (SITFx)

Max. decimation ratio when
DSIN = 12 bits (ADCITF)

Max. decimation ratio when
DSIN = 16 bits (ADCITF)

Sinc1

512

512

512

Sinc2

512

128

32

FastSinc

512

90

22

3

322

25

10

Sinc4

76

11

5

5

32

6

4

Sinc

Sinc

Scaling (SCALE) and saturation (SAT)
The SCALE block allows the application to adjust the amplitude of the signal provided by the
CIC, by steps of 3 dB (± 0.5 dB).
The signal amplitude can be decreased by up to 8 bits (- 48.2 dB) and can be increased by
up to 12 bits (+ 72.2 dB).
The gain is adjusted by the SCALE[5:0] bitfield in the MDF digital filter configuration register
x (MDF_DFLTxCICR).
SCALE[5:0] can be changed even if the corresponding DLFTx is enabled. During the gain
transition, the signal provided by the filter is disturbed.
Due to internal resynchronization, there is a delay of some cycles of mdf_proc_ck clock
between the moment where the application writes the new gain, and the moment where the
gain is effectively applied to the samples. If the application attempts to write a new gain
value while the previous one is not yet applied, this new gain value is ignored. Reading back
SCALE[5:0] informs the application on the current gain value.
RM0456 Rev 6

<!-- pagebreak -->

