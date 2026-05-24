1599

Multi-function digital filter (MDF)

RM0456

Bits 16:8 MCICD[8:0]: CIC decimation ratio selection
This bitfield is set and cleared by software. It is used to select the CIC decimation ratio. A
decimation ratio smaller than two is not allowed. The decimation ratio is given by
(CICDEC+1).
0: Decimation ratio is 2.
1: Decimation ratio is 2.
2: Decimation ratio is 3..
3: Decimation ratio is 4
...
511: Decimation ratio is 512.
Note: This bitfield can be write-protected (refer to Section 39.4.15 for details).
Bit 7 Reserved, must be kept at reset value.
Bits 6:4 CICMOD[2:0]: Select the CIC mode
This bitfield is set and cleared by software.It is used to select the configuration and the order
of the MCIC. When CICMOD[2:0] = 0xx, the CIC is split into two filters: the main CIC (MCIC)
and the auxiliary CIC (ACIC, used for the out-off limit detector).
000: CIC split in two filters and MCIC configured in FastSinc filter
001: CIC split in two filters and MCIC configured in Sinc1 filter
010: CIC split in two filters and MCIC configured in Sinc2 filter
011: CIC split in two filters and MCIC configured in Sinc3 filter
100: CIC configured in single Sinc4 filter
others: CIC configured in single Sinc5 filter
Note: This bitfield can be write-protected (refer to Section 39.4.15 for details).
Bits 3:2 Reserved, must be kept at reset value.
Bits 1:0 DATSRC[1:0]: Source data for the digital filter
This bitfield is set and cleared by software.
0x: Stream coming from the BSMX selected
10: Stream coming from the ADCITF1 selected
11: Stream coming from the ADCITF2 selected
Note: This bitfield can be write-protected (refer to Section 39.4.15 for details).

39.8.7

MDF reshape filter configuration register x (MDF_DFLTxRSFR)
Address offset: 0x090 + 0x80 * x, (x = 0 to 5)
Reset value: 0x0000 0000
This register is used to control the reshape and HPF filters. The number of registers is equal
to the amount of filters. Refer to Section 39.3 for details.

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

<!-- pagebreak -->

