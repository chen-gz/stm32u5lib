RM0456 Rev 6

RM0456

Analog-to-digital converter (ADC12)

Bits 4:2 Reserved, must be kept at reset value.
Bit 1 JOVSE: Injected oversampling enable
This bit is set and cleared by software to enable injected oversampling.
0: Injected oversampling disabled
1: Injected oversampling enabled
Note: The software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0 (which
ensures that no conversion is ongoing)
Bit 0 ROVSE: Regular oversampling enable
This bit is set and cleared by software to enable regular oversampling.
0: Regular oversampling disabled
1: Regular oversampling enabled
Note: The software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0 (which
ensures that no conversion is ongoing)

33.6.6

ADC sample time register 1 (ADC_SMPR1)
Address offset: 0x14
Reset value: 0x0000 0000

31

30

Res.

Res.

15

14

SMP5[0]
rw

29

28

27

26

SMP9[2:0]

24

23

SMP8[2:0]

22

21

20

SMP7[2:0]

19

18

17

SMP6[2:0]

16

SMP5[2:1]

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

SMP4[2:0]
rw

25

rw

SMP3[2:0]
rw

rw

rw

SMP2[2:0]
rw

rw

rw

SMP1[2:0]
rw

rw

rw

SMP0[2:0]
rw

rw

rw

rw

Bits 31:30 Reserved, must be kept at reset value.
Bits 29:0 SMPx[2:0]: Channel x sampling time selection (x = 9 to 0)
These bits are written by software to select the sampling time individually for each channel.
During sample cycles, the channel selection bits must remain unchanged.
000: 5 ADC clock cycles
001: 6 ADC clock cycles
010: 12 ADC clock cycles
011: 20 ADC clock cycles
100: 36 ADC clock cycles
101: 68 ADC clock cycles
110: 391 ADC clock cycles
111: 814 ADC clock cycles
Note: The software is allowed to write these bits only when ADSTART = 0 and
JADSTART = 0 (which ensures that no conversion is ongoing).

RM0456 Rev 6

<!-- pagebreak -->

