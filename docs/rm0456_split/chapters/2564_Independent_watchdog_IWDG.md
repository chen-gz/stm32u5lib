RM0456 Rev 6

RM0456

Tamper and backup registers (TAMP)

Bit 14 SEEDF: Seed running flag
This flag is set by hardware when a new seed is written in the TAMP_ATSEEDR. It is cleared
by hardware when the PRNG has absorbed this new seed, and by system reset. The TAMP
APB clock must not be switched off as long as SEEDF is set.
Bits 13:8 Reserved, must be kept at reset value.
Bits 7:0 PRNG[7:0]: Pseudo-random generator value
This field provides the values of the PRNG output. Because of potential inconsistencies due
to synchronization delays, PRNG must be read at least twice. The read value is correct if it is
equal to previous read value.
This field can only be read when the APB is in secure mode.

64.7.8

TAMP active tamper control register 2 (TAMP_ATCR2)
This register can be protected against nonsecure access. Refer to Section 64.4.5: TAMP
secure protection modes.
This register can be protected against unprivileged access. Refer to Section 64.4.7: TAMP
privilege protection modes.
Address offset: 0x1C
Backup domain reset value: 0x0000 0000
System reset: not affected

31

30

29

28

ATOSEL8[2:0]

27

26

25

ATOSEL7[2:0]

24

23

22

ATOSEL6[2:0]

21

20

19

ATOSEL5[2:0]

18

17

16
ATO
SEL3
[2]

ATOSEL4[2:0]

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

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

ATOSEL3[1:0]
rw

rw

ATOSEL2[2:0]
rw

rw

ATOSEL1[2:0]
rw

rw

rw

rw

rw

RM0456 Rev 6

<!-- pagebreak -->

