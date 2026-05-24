3631

Debug support (DBG)

RM0456

Bits 11:0 ARCHPART[11:0]: architecture part
0xA13: ETM architecture

ETM CoreSight device type register (ETM_DEVTYPER)
Address offset: 0xFCC
Reset value: 0x0000 0013
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

SUBTYPE[3:0]
r

r

r

MAJORTYPE[3:0]
r

r

r

r

r

Bits 31:8 Reserved, must be kept at reset value.
Bits 7:4 SUBTYPE[3:0]: device sub-type identifier
0x1: processor trace
Bits 3:0 MAJORTYPE[3:0]: device main type identifier
0x3: trace source

ETM CoreSight peripheral identity register 4 (ETM_PIDR4)
Address offset: 0xFD0
Reset value: 0x0000 0004
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

SIZE[3:0]
r

r

r

Bits 31:8 Reserved, must be kept at reset value.
Bits 7:4 SIZE[3:0]: register file size
0x0: The register file occupies a single 4-Kbyte region.
Bits 3:0 JEP106CON[3:0]: JEP106 continuation code
0x4: Arm JEDEC code

<!-- pagebreak -->

