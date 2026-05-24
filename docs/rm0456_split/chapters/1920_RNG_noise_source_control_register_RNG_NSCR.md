1921

True random number generator (RNG)

48.7.4

RM0456

RNG noise source control register (RNG_NSCR)
Address offset: 0x00C
Reset value: 0x0003 FFFF
Writing in RNG_NSCR is taken into account only if the CONDRST bit is set, and the
CONFIGLOCK bit is cleared in RNG_CR. Writing to this register is ignored if
CONFIGLOCK= 1.

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

EN_
OSC6[0]
rw

EN_OSC5[2:0]
rw

rw

EN_OSC4[2:0]
rw

rw

rw

EN_OSC3[2:0]
rw

rw

rw

EN_OSC2[2:0]
rw

rw

rw

17

16

EN_OSC6[2:1]
rw

rw

1

0

EN_OSC1[2:0]
rw

rw

rw

rw

Bits 31:18 Reserved, must be kept at reset value.
Bits 17:15 EN_OSC6[2:0]:
When the RNG is enabled (RNGEN bit set), each bit of this bitfield enables one of the three
inputs from the oscillator instance number 6. The bitfield has no effect otherwise.
Bits 14:12 EN_OSC5[2:0]:
When the RNG is enabled (RNGEN bit set), each bit of this bitfield enables one of the three
inputs from the oscillator instance number 5. The bitfield has no effect otherwise.
Bits 11:9 EN_OSC4[2:0]:
When the RNG is enabled (RNGEN bit set), each bit of this bitfield enables one of the three
inputs from the oscillator instance number 4. The bitfield has no effect otherwise.
Bits 8:6 EN_OSC3[2:0]:
When the RNG is enabled (RNGEN bit set), each bit of this bitfield enables one of the three
inputs from the oscillator instance number 3. The bitfield has no effect otherwise.
Bits 5:3 EN_OSC2[2:0]:
When the RNG is enabled (RNGEN bit set), each bit of this bitfield enables one of the three
inputs from the oscillator instance number 2. The bitfield has no effect otherwise.
Bits 2:0 EN_OSC1[2:0]:
When the RNG is enabled (RNGEN bit set), each bit of this bitfield enables one of the three
inputs from the oscillator instance number 1. The bitfield has no effect otherwise.

<!-- pagebreak -->

