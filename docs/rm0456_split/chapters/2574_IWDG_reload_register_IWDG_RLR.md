RM0456 Rev 6

RM0456

Tamper and backup registers (TAMP)

Bit 2 TAMP3F: TAMP3 detection flag
This flag is set by hardware when a tamper detection event is detected on the TAMP3 input.
Bit 1 TAMP2F: TAMP2 detection flag
This flag is set by hardware when a tamper detection event is detected on the TAMP2 input.
Bit 0 TAMP1F: TAMP1 detection flag
This flag is set by hardware when a tamper detection event is detected on the TAMP1 input.

64.7.13

TAMP nonsecure masked interrupt status register (TAMP_MISR)
This register can be protected against unprivileged access. Refer to Section 64.4.7: TAMP
privilege protection modes.
Address offset: 0x34
Backup domain reset value: 0x0000 0000
System reset: not affected

31

30

29

28

27

26

ITAMP1 ITAMP1 ITAMP1
3MF
2MF
1MF

25
Res.

24

23

22

21

20

ITAMP9 ITAMP8 ITAMP ITAMP6 ITAMP5
MF
MF
7MF
MF
MF

19
Res.

18

17

16

ITAMP3 ITAMP2 ITAMP1
MF
MF
MF

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
8MF

TAMP
7MF

TAMP
6MF

TAMP
5MF

TAMP
4MF

TAMP
3MF

TAMP
2MF

TAMP
1MF

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
Bit 28 ITAMP13MF: internal tamper 13 nonsecure interrupt masked flag
This flag is set by hardware when the internal tamper 13 nonsecure interrupt is raised.
Bit 27 ITAMP12MF: internal tamper 12 nonsecure interrupt masked flag
This flag is set by hardware when the internal tamper 12 nonsecure interrupt is raised.
Bit 26 ITAMP11MF: internal tamper 11 nonsecure interrupt masked flag
This flag is set by hardware when the internal tamper 11 nonsecure interrupt is raised.
Bit 25 Reserved, must be kept at reset value.
Bit 24 ITAMP9MF: internal tamper 9 nonsecure interrupt masked flag
This flag is set by hardware when the internal tamper 9 nonsecure interrupt is raised.
Bit 23 ITAMP8MF: Internal tamper 8 nonsecure interrupt masked flag
This flag is set by hardware when the internal tamper 8 nonsecure interrupt is raised.
Bit 22 ITAMP7MF: Internal tamper 7 tamper nonsecure interrupt masked flag
This flag is set by hardware when the internal tamper 7 nonsecure interrupt is raised.
Bit 21 ITAMP6MF: Internal tamper 6 nonsecure interrupt masked flag
This flag is set by hardware when the internal tamper 6 nonsecure interrupt is raised.
Bit 20 ITAMP5MF: Internal tamper 5 nonsecure interrupt masked flag
This flag is set by hardware when the internal tamper 5 nonsecure interrupt is raised.

RM0456 Rev 6

<!-- pagebreak -->

