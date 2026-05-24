1921

True random number generator (RNG)

RM0456

Bit 6 Reserved, must be kept at reset value.
Bit 5 CED: Clock error detection
0: Clock error detection enabled
1: Clock error detection is disabled
The clock error detection cannot be enabled nor disabled on-the-fly when the RNG is
enabled, that is to enable or disable CED, the RNG must be disabled.
Writing this bit is taken into account only if the CONDRST bit is set to 1 in the same access,
while CONFIGLOCK remains at 0. Writing to this bit is ignored if CONFIGLOCK = 1.
Bit 4 Reserved, must be kept at reset value.
Bit 3 IE: Interrupt enable
0: RNG interrupt is disabled
1: RNG interrupt is enabled. An interrupt is pending as soon as the DRDY, SEIS, or CEIS is
set in the RNG_SR register.
Bit 2 RNGEN: True random number generator enable
0: True random number generator is disabled. Analog noise sources are powered off and
logic clocked by the RNG clock is gated.
1: True random number generator is enabled.
Bits 1:0 Reserved, must be kept at reset value.

48.7.2

RNG status register (RNG_SR)
Address offset: 0x004
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

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

SEIS

CEIS

Res.

Res.

SECS

CECS

DRDY

rc_w0

rc_w0

r

r

r

Bits 31:7 Reserved, must be kept at reset value.
Bit 6 SEIS: Seed error interrupt status
This bit is set at the same time as SECS. It is cleared by writing 0 (unless CONDRST is
used). Writing 1 has no effect.
0: No faulty sequence detected
1: At least one faulty sequence is detected. See SECS bit description for details.
An interrupt is pending if IE = 1 in the RNG_CR register.
Bit 5 CEIS: Clock error interrupt status
This bit is set at the same time as CECS. It is cleared by writing 0. Writing 1 has no effect.
0: The RNG clock is correct (fRNGCLK> fHCLK/32)
1: The RNG clock before the internal divider is detected too slow (fRNGCLK< fHCLK/32)
An interrupt is pending if IE = 1 in the RNG_CR register.
Bits 4:3 Reserved, must be kept at reset value.

<!-- pagebreak -->

