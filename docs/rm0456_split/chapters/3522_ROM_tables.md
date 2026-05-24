RM0456 Rev 6

r

JEDEC
r

r

JEP106ID[6:4]
r

r

r

RM0456

Debug support (DBG)

DBGMCU CoreSight peripheral identity register 3 (DBGMCU_PIDR3)
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

DBGMCU CoreSight component identity register 0 (DBGMCU_CIDR0)
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
r

r

r

r

r

r

PREAMBLE[7:0]
r

r

Bits 31:8 Reserved, must be kept at reset value.
Bits 7:0 PREAMBLE[7:0]: component identification bits [7:0]
0x0D: common identification value

DBGMCU CoreSight component identity register 1 (DBGMCU_CIDR1)
Address offset: 0xFF4
Reset value: 0x0000 00F0
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

CLASS[3:0]
r

r

r

PREAMBLE[11:8]
r

r

r

r

r

Bits 31:8 Reserved, must be kept at reset value.

RM0456 Rev 6

<!-- pagebreak -->

3631

Debug support (DBG)

RM0456

Bits 7:4 CLASS[3:0]: component identification bits [15:12] - component class
0xF: Non-CoreSight component
Bits 3:0 PREAMBLE[11:8]: component identification bits [11:8]
0x0: common identification value

DBGMCU CoreSight component identity register 2 (DBGMCU_CIDR2)
Address offset: 0xFF8
Reset value: 0x0000 0005
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

PREAMBLE[19:12]
r

r

r

r

r

Bits 31:8 Reserved, must be kept at reset value.
Bits 7:0 PREAMBLE[19:12]: component identification bits [23:16]
0x05: common identification value

DBGMCU CoreSight component identity register 3 (DBGMCU_CIDR3)
Address offset: 0xFFC
Reset value: 0x0000 00B1
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

PREAMBLE[27:20]
r

r

r

r

r

Bits 31:8 Reserved, must be kept at reset value.
Bits 7:0 PREAMBLE[27:20]: component identification bits [31:24]
0xB1: common identification value

75.12.5

DBGMCU register map

Reset value

X X X X X X X X X X X X X X X X

<!-- pagebreak -->

