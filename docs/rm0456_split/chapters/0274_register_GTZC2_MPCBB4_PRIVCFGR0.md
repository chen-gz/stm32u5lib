275

Global TrustZone controller (GTZC)

RM0456

Bits 31:0 SECy: Security configuration for block y, belonging to super-block 0 (y = 31 to 0)
0: Nonsecure access only to block y, belonging to super-block 0. Secure access is also
allowed if the SRWILADIS bit is set in GTZC2_MPCBB4_CR.
1: Secure access only to block y, belonging to super-block 0.
Unprivileged write to this bit is ignored if PRIVy bit is set in GTZC2_MPCBB4_PRIVCFGR0.
Write are ignored if SPLCK0 bit is set in GTZC2_MPCBB4_CFGLOCKR1.

5.11.4

GTZC2 SRAM4 MPCBB privileged configuration for super-block 0
register (GTZC2_MPCBB4_PRIVCFGR0)
Address offset: 0x200
Reset value: 0xFFFF FFFF
The given reset value is valid when TZEN = 1. The reset value is 0x0000 0000 when
TZEN = 0.
Write access to this register is privileged only. Any read is allowed.

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

PRIV31 PRIV30 PRIV29 PRIV28 PRIV27 PRIV26 PRIV25 PRIV24 PRIV23 PRIV22 PRIV21 PRIV20 PRIV19 PRIV18 PRIV17 PRIV16
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

PRIV8

PRIV7

PRIV6

PRIV5

PRIV4

PRIV3

PRIV2

PRIV1

PRIV0

rw

rw

rw

rw

rw

rw

rw

rw

rw

PRIV15 PRIV14 PRIV13 PRIV12 PRIV11 PRIV10 PRIV9
rw

rw

rw

rw

rw

rw

rw

Bits 31:0 PRIVy: Privileged configuration for block y, belonging to super-block 0 (y = 31 to 0).
0: Privileged and unprivileged access to block y, belonging to super block 0
1: privileged access only to block y, belonging to super-block 0
Nonsecure write to this bit is ignored if SECy bit is set in GTZC2_MPBCC4_SECCFGR0.
Write are ignored if SPLCK0 bit is set in GTZC2_MPCBB4_CFGLOCKR1.

5.11.5

GTZC2 MPCBB4 register map

Register name

0x000

GTZC2_
MPCBB4_CR

SRWILADIS

Reset value

0

0

Res.

0

GTZC2_MPCBB4_
SECCFGR0

SEC29

SEC28

SEC27

SEC26

SEC25

SEC24

SEC23

SEC22

SEC21

SEC20

SEC19

SEC18

SEC17

SEC16

SEC15

SEC14

SEC13

SEC12

SEC11

SEC10

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

Reserved
SEC30

Reserved
SEC31

0x100

SPLCK0

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

Reset value
0x0140x0FC

GLOCK

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

GTZC2_MPCBB4_
CFGLOCKR1

Reserved
Res.

0x010

0

Reserved

Res.

0x0040x00C

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

Offset

INVSECSTATE

Table 44. GTZC2 MPCBB4 register map and reset values

Reset value

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

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Global TrustZone controller (GTZC)

PRIV14

PRIV13

PRIV12

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

PRIV0

PRIV15

1

PRIV1

PRIV16

1

PRIV2

PRIV17

1

PRIV3

PRIV18

1

PRIV4

PRIV19

1

PRIV5

PRIV20

1

PRIV6

PRIV21

1

PRIV7

PRIV22

1

PRIV8

PRIV23

1

PRIV9

PRIV24

1

PRIV11

PRIV25

Reset value

PRIV10

GTZC2_MPCBB4_
PRIVCFGR0

PRIV26

0x200

PRIV27

Reserved
PRIV28

Reserved
PRIV29

0x1040x1FC

PRIV30

Register name

PRIV31

Offset

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

Table 44. GTZC2 MPCBB4 register map and reset values (continued)

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

Refer to Table 30: GTZC2 subblocks address offset.

RM0456 Rev 6

<!-- pagebreak -->

