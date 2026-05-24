RM0456 Rev 6

RM0456

Analog-to-digital converter (ADC4)

Bits 8:5 OVSS[3:0]: Oversampling shift
This bit is set and cleared by software.
0000: No shift
0001: Shift 1-bit
0010: Shift 2-bits
0011: Shift 3-bits
0100: Shift 4-bits
0101: Shift 5-bits
0110: Shift 6-bits
0111: Shift 7-bits
1000: Shift 8-bits
Others: Reserved
Note: The software is allowed to write this bit only when ADSTART is cleared to 0 by writing ADSTP
to 1(which ensures that no conversion is ongoing).
Bits 4:2 OVSR[2:0]: Oversampling ratio
This bit filed defines the number of oversampling ratio.
000: 2x
001: 4x
010: 8x
011: 16x
100: 32x
101: 64x
110: 128x
111: 256x
Note: The software is allowed to write this bit only when ADSTART is cleared to 0 by writing ADSTP
to 1 (which ensures that no conversion is ongoing).
Bit 1

Reserved, must be kept at reset value.

Bit 0 OVSE: Oversampler Enable
This bit is set and cleared by software.
0: Oversampler disabled
1: Oversampler enabled
Note: Software is allowed to write this bit only when ADSTART is cleared to 0 by writing ADSTP to 1
(which ensures that no conversion is ongoing).

34.7.6

ADC sampling time register (ADC_SMPR)
Address offset: 0x14
Reset value: 0x0000 0000

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

SMPSE SMPSE SMPSE SMPSE SMPSE SMPSE SMPSE SMPSE SMPSE SMPSE SMPSE SMPSE SMPSE SMPSE SMPSE SMPSE
L23
L22
L21
L20
L19
L18
L17
L16
L15
L14
L13
L12
L11
L10
L9
L8
rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

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

SMPSE SMPSE SMPSE SMPSE SMPSE SMPSE SMPSE SMPSE
L7
L6
L5
L4
L3
L2
L1
L0
rw

rw

rw

rw

rw

rw

rw

Res.

rw

RM0456 Rev 6

SMP2[2:0]
rw

rw

Res.
rw

SMP1[2:0]
rw

rw

rw

<!-- pagebreak -->

