RM0456 Rev 6

RM0456

Tamper and backup registers (TAMP)

Bit 4 TAMP5IE: Tamper 5 interrupt enable
0: Tamper 5 interrupt disabled.
1: Tamper 5 interrupt enabled.
Bit 3 TAMP4IE: Tamper 4 interrupt enable
0: Tamper 4 interrupt disabled.
1: Tamper 4 interrupt enabled.
Bit 2 TAMP3IE: Tamper 3 interrupt enable
0: Tamper 3 interrupt disabled.
1: Tamper 3 interrupt enabled..
Bit 1 TAMP2IE: Tamper 2 interrupt enable
0: Tamper 2 interrupt disabled.
1: Tamper 2 interrupt enabled.
Bit 0 TAMP1IE: Tamper 1 interrupt enable
0: Tamper 1 interrupt disabled.
1: Tamper 1 interrupt enabled.

64.7.12

TAMP status register (TAMP_SR)
This register can be protected against nonsecure access. Refer to Section 64.4.5: TAMP
secure protection modes.
This register can be protected against unprivileged access. Refer to Section 64.4.7: TAMP
privilege protection modes.
Address offset: 0x30
Backup domain reset value: 0x0000 0000
System reset: not affected

31

30

29

28

27

26

ITAMP1 ITAMP1 ITAMP1
3F
2F
1F

25
Res.

24

23

22

21

20

ITAMP9 ITAMP8 ITAMP7 ITAMP6 ITAMP5
F
F
F
F
F

19
Res.

18

17

16

ITAMP3 ITAMP2 ITAMP1
F
F
F

Res.

Res.

Res.

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

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

TAMP
8F

TAMP
7F

TAMP
6F

TAMP
5F

TAMP
4F

TAMP
3F

TAMP
2F

TAMP
1F

r

r

r

r

r

r

r

r

Bit 31 Reserved, must be kept at reset value.
Bit 30 Reserved, must be kept at reset value.
Bit 29 Reserved, must be kept at reset value.
Bit 28 ITAMP13F: Internal tamper 13 flag
This flag is set by hardware when a tamper detection event is detected on the internal
tamper 13.
Bit 27 ITAMP12F: Internal tamper 12 flag
This flag is set by hardware when a tamper detection event is detected on the internal
tamper 12.

RM0456 Rev 6

<!-- pagebreak -->

