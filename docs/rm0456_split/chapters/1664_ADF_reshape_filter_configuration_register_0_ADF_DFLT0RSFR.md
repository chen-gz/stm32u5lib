1674

Audio digital filter (ADF)

RM0456

Bits 6:4 CICMOD[2:0]: Select the CIC order
This bitfield is set and cleared by software. It is used to select the order of the MCIC.
100: MCIC configured in single Sinc4 filter
101: MCIC configured in single Sinc5 filter
Others: Reserved
Note: This bitfield can be write-protected (see Section 40.4.13: Register protection for details).
Bits 3:2 Reserved, must be kept at reset value.
Bits 1:0 DATSRC[1:0]: Source data for the digital filter
This bitfield is set and cleared by software.
0x: Stream coming from the BSMX selected
10: Stream coming from the ADCITF1 selected
11: Stream coming from the ADCITF2 selected
Note: This bitfield can be write-protected (see Section 40.4.13: Register protection for details).

40.8.7

ADF reshape filter configuration register 0 (ADF_DFLT0RSFR)
Address offset: 0x090
Reset value: 0x0000 0000
This register is used to control the reshape and HPF filter.

31

30

29

28

27

26

25

24

23

22

21

20

19

18

17

16

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

15

14

13

12

11

10

9

8

7

6

5

4

3

2

1

0

Res.

RSFLT
D

Res.

RSFLT
BYP

Res.

Res.

Res.

Res.

Res.

Res.

HPFC[1:0]
rw

HPFBY
P

rw

rw

Res.

rw

Res.

Res.

rw

Bits 31:10 Reserved, must be kept at reset value.
Bits 9:8 HPFC[1:0]: High-pass filter cut-off frequency
This bitfield is set and cleared by software. it is used to select the cut-off frequency of the
high-pass filter. FPCM represents the sampling frequency at HPF input.
00: Cut-off frequency = 0.000625 x FPCM
01: Cut-off frequency = 0.00125 x FPCM
10: Cut-off frequency = 0.00250 x FPCM
11: Cut-off frequency = 0.00950 x FPCM
Note: This bitfield can be write-protected (see Section 40.4.13: Register protection for details).
Bit 7 HPFBYP: High-pass filter bypass
This bit is set and cleared by software. It is used to bypass the high-pass filter.
0: HPF not bypassed (default value)
1: HPF bypassed
Note: This bit can be write-protected (see Section 40.4.13: Register protection for details).
Bits 6:5 Reserved, must be kept at reset value.

<!-- pagebreak -->

