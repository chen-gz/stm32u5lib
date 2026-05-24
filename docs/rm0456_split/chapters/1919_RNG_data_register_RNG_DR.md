RM0456 Rev 6

RM0456

True random number generator (RNG)

Bit 2 SECS: Seed error current status
0: No faulty sequence has currently been detected. If the SEIS bit is set, this means that a
faulty sequence was detected and the situation has been recovered.
1: At least one of the following faulty sequences has been detected:
–
Runtime repetition count test failed (noise source has provided more than 24
consecutive bits at a constant value 0 or 1, or more than 32 consecutive occurrence
of two bits patterns 01 or 10)
–
Startup or continuous adaptive proportion test on noise source failed.
–
Startup post-processing/conditioning sanity check failed.
Bit 1 CECS: Clock error current status
0: The RNG clock is correct (fRNGCLK> fHCLK/32). If the CEIS bit is set, this means that a
slow clock was detected and the situation has been recovered.
1: The RNG clock is too slow (fRNGCLK< fHCLK/32).
Note: CECS bit is valid only if the CED bit in the RNG_CR register is set to 0.
Bit 0 DRDY: Data ready
0: The RNG_DR register is not yet valid, no random data is available.
1: The RNG_DR register contains valid random data.
Once the output buffer becomes empty (after reading the RNG_DR register), this bit returns
to 0 until a new random value is generated.
Note: The DRDY bit can rise when the peripheral is disabled (RNGEN = 0 in the RNG_CR
register).
If IE=1 in the RNG_CR register, an interrupt is generated when DRDY = 1.

48.7.3

RNG data register (RNG_DR)
Address offset: 0x008
Reset value: 0x0000 0000
The RNG_DR register is a read-only register that delivers a 32-bit random value when read.
The content of this register is valid when the DRDY = 1 and the value is not 0x0, even if
RNGEN = 0.

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

RNDATA[31:16]
r

r

r

r

r

r

r

r

r

r

r

r

r

r

r

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

4

3

2

1

0

r

r

r

r

r

r

r

r

r

r

r

r

r

r

RNDATA[15:0]
r

r

Bits 31:0 RNDATA[31:0]: Random data
32-bit random data, which are valid when DRDY = 1. When DRDY = 0, the RNDATA value
is zero.
When DRDY is set, it is recommended to always verify that RNG_DR is different from zero.
The zero value means that a seed error occurred between RNG_SR polling and RND_DR
output reading (a rare event).

RM0456 Rev 6

<!-- pagebreak -->

