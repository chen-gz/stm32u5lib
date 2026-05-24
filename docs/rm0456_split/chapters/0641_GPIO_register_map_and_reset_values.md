RM0456 Rev 6

RM0456

General-purpose I/Os (GPIO)

13.4.13

GPIO secure configuration register (GPIOx_SECCFGR) (x = A to J)
Address offset: 0x30
Reset value: 0x0000 FFFF (for ports A to E)
Reset value: 0x0000 FFFF (for port F on STM32U575/585/59x/5Ax/5Fx/5Gx)
Reset value: 0x0000 FFFC (for port G on STM32U535/545)
Reset value: 0x0000 FFFF (for port G on STM32U575/585/59x/5Ax/5Fx/5Gx)
Reset value: 0x0000 000B (for port H on STM32U535/545)
Reset value: 0x0000 FFFF (for port H on STM32U575/585/59x/5Ax/5Fx/5Gx)
Reset value: 0x0000 00FF (for port I on STM32U575/585)
Reset value: 0x0000 FFFF (for port I on STM32U59x/5Ax/5Fx/5Gx)
Reset value: 0x0000 0FFF (for port J on STM32U59x/5Ax/5Fx/5Gx)
When the system is secure (TZEN = 1), this register provides write access security and can
be written only by a secure access. It is used to configure a selected I/O as secure. A
non-secure write access to this register is discarded.
When the system is not secure (TZEN = 0), this register is RAZ/WI.

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

SEC9

SEC8

SEC7

SEC6

SEC5

SEC4

SEC3

SEC2

SEC1

SEC0

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

SEC15 SEC14 SEC13 SEC12 SEC11 SEC10
rw

rw

rw

rw

rw

rw

Bits 31:16 Reserved, must be kept at reset value.
Bits 15:0 SECy: I/O pin of Port x secure bit enable y (y = 15 to 0)
These bits are written by software to enable or disable the I/O port pin security.
0: The I/O pin is nonsecure
1: The I/O pin is secure. Refer to Table 128 for all corresponding secured bits.
Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not
available on the selected package.

13.4.14

GPIO register map

0x00

MODE0
[1:0]

MODE1
[1:0]

MODE2
[1:0]

MODE3
[1:0]

MODE4
[1:0]

MODE5
[1:0]

MODE6
[1:0]

MODE7
[1:0]

MODE8
[1:0]

MODE9
[1:0]

MODE10
[1:0]

MODE11
[1:0]

MODE12
[1:0]

MODE13
[1:0]

MODE14
[1:0]

GPIOx_MODER
(x = A to J)

MODE15
[1:0]

Offset Register name

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

Table 129. GPIO register map and reset values

Reset value for port A 1

0

1

0

1

0

1

1

1

1

1

1

1

1

1

1

1

1

1

1

1

1

1

1

1

1

1

1

1

1

1

1

Reset value for port B 1

1

1

1

1

1

1

1

1

1

1

1

1

1

1

1

1

1

1

1

1

1

1

0

1

0

1

1

1

1

1

1

Reset value for
ports C...I

1

1

1

1

1

1

1

1

1

1

1

1

1

1

1

1

1

1

1

1

1

1

1

1

1

1

1

1

1

1

1

1

Reset value for port J 0

0

0

0

0

0

0

0

1

1

1

1

1

1

1

1

1

1

1

1

1

1

1

1

1

1

1

1

1

1

1

1

RM0456 Rev 6

<!-- pagebreak -->

642

0x30

GPIOx_
SECCFGR
(x = A to J)

<!-- pagebreak -->

