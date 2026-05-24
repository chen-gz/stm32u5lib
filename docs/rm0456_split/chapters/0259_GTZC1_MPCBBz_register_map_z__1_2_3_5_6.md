Some registers are only available on some devices in the STM32U5 series. Refer to the
device datasheet for availability of its associated memory region.

RM0456 Rev 6

RM0456

31

Global TrustZone controller (GTZC)

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

rw

Bits 31:0 PRIVy: Privileged configuration for block y, belonging to super-block x (y = 31 to 0).
0: Privileged and unprivileged access to block y, belonging to super-block x
1: Only privileged access to block y, belonging to super-block x
Nonsecure write to this bit is ignored if SECy bit is set in GTZC1_MPCBBz_SECCFGRx.
Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCKR1/2.

5.8.6

GTZC1 MPCBBz register map (z = 1, 2, 3, 5, 6)

Register name

0x000

GTZC1_
MPCBBz_CR

SRWILADIS

Reset value

0

0

Res.

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

SPLCK12

SPLCK11

SPLCK10

SPLCK9

SPLCK8

SPLCK7

SPLCK6

SPLCK5

SPLCK4

SPLCK3

SPLCK2

SPLCK1

SPLCK0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

GTZC1_MPCBBz
_CFGLOCKR2

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

SPLCK51

SPLCK50

SPLCK49

SPLCK48

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

SEC30

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
SEC31

Reserved

SPLCK32

SPLCK13

0

SPLCK33

SPLCK14

0

SPLCK34

SPLCK15

0

SPLCK35

SPLCK16

0

SPLCK36

SPLCK17

0

SPLCK37

SPLCK18

0

SPLCK3/8

SPLCK19

0

SPLCK39

SPLCK20

0

SPLCK40

SPLCK21

0

SPLCK41

SPLCK22

0

SPLCK42

SPLCK23

0

SPLCK43

SPLCK24

0

SPLCK44

SPLCK25

0

SPLCK45

SPLCK26

0

SPLCK46

SPLCK27

0

SPLCK47

SPLCK28

0

0x100 + GTZC1_MPCBBz
_SECCFGRx
0x04 *x
(x = 0 to 51)
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

PRIV28

PRIV27

PRIV26

PRIV25

PRIV24

PRIV23

PRIV22

PRIV21

PRIV20

PRIV19

PRIV18

PRIV17

PRIV16

PRIV15

PRIV14

PRIV13

PRIV12

PRIV11

PRIV10

PRIV9

PRIV8

PRIV7

PRIV6

PRIV5

PRIV4

PRIV3

PRIV2

PRIV1

PRIV0

0x200 + GTZC1_MPCBBz
_PRIVCFGRx
0x04 *x
(x = 0 to 51)
Reset value
1

PRIV29

Reserved
PRIV30

Reserved
PRIV31

0x1800x1FC

Res.

SPLCK29

Reset value

Reset value
0x0180x0FC

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

GTZC1_MPCBBz
_CFGLOCKR1

SPLCK30

Reserved
SPLCK31

0x014

Reserved

Res.

0x010

0

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

Table 41. GTZC1 MPCBBz register map and reset values (z = 1, 2, 3, 5, 6)

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

Refer to Table 29: GTZC1 subblocks address offset.

RM0456 Rev 6

<!-- pagebreak -->

