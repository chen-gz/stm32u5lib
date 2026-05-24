1443

Analog-to-digital converter (ADC4)

RM0456

Bits 31:8 SMPSELx: Channel-x sampling time selection (x = 23 to 0)
These bits are written by software to define which sampling time is used.
0: Sampling time of CHANNELx use the setting of SMP1[2:0] register.
1: Sampling time of CHANNELx use the setting of SMP2[2:0] register.
Note: The software is allowed to write this bit only when ADSTART is cleared to 0 by writing ADSTP
to 1 (which ensures that no conversion is ongoing).
Bit 7 Reserved, must be kept at reset value.
Bits 6:4 SMP2[2:0]: Sampling time selection 2
These bits are written by software to select the sampling time that applies to all channels.
000: 1.5 ADC clock cycles
001: 3.5 ADC clock cycles
010: 7.5 ADC clock cycles
011: 12.5 ADC clock cycles
100: 19.5 ADC clock cycles
101: 39.5 ADC clock cycles
110: 79.5 ADC clock cycles
111: 814.5 ADC clock cycles
Note: The software is allowed to write this bit only when ADSTART is cleared to 0 by writing ADSTP
to 1 (which ensures that no conversion is ongoing).
Bit 3 Reserved, must be kept at reset value.
Bits 2:0 SMP1[2:0]: Sampling time selection 1
These bits are written by software to select the sampling time that applies to all channels.
000: 1.5 ADC clock cycles
001: 3.5 ADC clock cycles
010: 7.5 ADC clock cycles
011: 12.5 ADC clock cycles
100: 19.5 ADC clock cycles
101: 39.5 ADC clock cycles
110: 79.5 ADC clock cycles
111: 814.5 ADC clock cycles
Note: The software is allowed to write this bit only when ADSTART is cleared to 0 by writing ADSTP
to 1 (which ensures that no conversion is ongoing).

34.7.7

ADC watchdog threshold register (ADC_AWD1TR)
Address offset: 0x20
Reset value: 0x0FFF 0000

31

30

29

28

Res.

Res.

Res.

Res.

15

14

13

12

Res.

Res.

Res.

Res.

27

26

25

24

23

21

20

19

18

17

16

HT1[11:0]
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

rw

rw

rw

rw

rw

LT1[11:0]
rw

rw

rw

rw

rw

Bits 31:28 Reserved, must be kept at reset value.

<!-- pagebreak -->

