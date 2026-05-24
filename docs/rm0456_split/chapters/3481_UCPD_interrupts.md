3631

Debug support (DBG)

RM0456

Bits 7:4 REVISION[3:0]: component revision number
0x1: r0p1
Bit 3 JEDEC: JEDEC assigned value
0x1: designer identification specified by JEDEC
Bits 2:0 JEP106ID[6:4]: JEP106 identity code bits [6:4]
0x3: Arm JEDEC code

ETM CoreSight peripheral identity register 3 (ETM_PIDR3)
Address offset: 0xFEC
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

7

6

5

4

3

2

1

0

15

14

13

12

11

10

9

8

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

REVAND[3:0]
r

r

r

CMOD[3:0]
r

r

r

r

r

Bits 31:8 Reserved, must be kept at reset value.
Bits 7:4 REVAND[3:0]: metal fix version
0x0: no metal fix
Bits 3:0 CMOD[3:0]: customer modified
0x0: no customer modifications

ETM CoreSight component identity register 0 (ETM_CIDR0)
Address offset: 0xFF0
Reset value: 0x0000 000D
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

15

14

13

12

11

10

9

8

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

PREAMBLE[7:0]
r

r

Bits 31:8 Reserved, must be kept at reset value.
Bits 7:0 PREAMBLE[7:0]: component identification bits [7:0]
0x0D: common identification value

<!-- pagebreak -->

