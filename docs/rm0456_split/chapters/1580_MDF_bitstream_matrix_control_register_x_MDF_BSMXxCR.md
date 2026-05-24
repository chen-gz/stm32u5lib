1599

Multi-function digital filter (MDF)

RM0456

Bit 0 SITFEN: Serial interface enable
This bit is et and cleared by software. It is used to enable/disable the serial interface.
0: Serial interface disabled
1: Serial interface enabled

39.8.4

MDF bitstream matrix control register x (MDF_BSMXxCR)
Address offset: 0x084 + 0x80 * x, (x = 0 to 5)
Reset value: 0x0000 0000
This register is used to select the bitstream to be provided to the corresponding digital filter
and to the SCD. The number of registers is equal to the amount of filters. Refer to
Section 39.3 for details.

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

BSMXA
CTIVE

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

4

3

2

1

0

rw

rw

r
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

BSSEL[4:0]
rw

rw

rw

Bit 31 BSMXACTIVE: BSMX active flag
This bit is set and cleared by hardware. It is used by the application to check if the BSMX is
effectively enabled (active) or not. BSSEL[4:0] can only be updated when the BSMXACTIVE
is set to 0. Th is BSMXACTIVE flag is a logical OR between OLDACTIVE, DFLTACTIVE, and
SCDACTIVE flags. Both of them must be set to 0 in order update BSSEL[4:0] bitfield.
0: BSMX is not active and can be configured if needed.
1: BSMX is active and protected fields cannot be configured.
Bits 30:5 Reserved, must be kept at reset value.
Bits 4:0 BSSEL[4:0]: Bitstream Selection
This bitfield is set and cleared by software. It is used to select the bitstream to be processed
for DFLTx and SCDx. The size of this bitfield depends on the number of DFLTx instantiated. If
this bitfield selects a not instantiated input, the MDF selects the valid stream bsx_f having the
higher index number.
00000: bs0_r provided to DFLTx and SCDx
00001: bs0_f provided to DFLTx and SCDx
00010: bs1_r provided to DFLTx and SCDx (if instantiated)
00011: bs1_f provided to DFLTx and SCDx (if instantiated)
...
11110: bs15_r provided to DFLTx and SCDx (if instantiated)
11111: bs15_f provided to DFLTx and SCDx (if instantiated)
Note: This bitfield can be write-protected (refer to Section 39.4.15 for details).

<!-- pagebreak -->

