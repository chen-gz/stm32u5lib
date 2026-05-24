3631

Debug support (DBG)

RM0456

Bits 4:0 EXTMUXNUM[4:0]: number of trigger input/output multiplexers
0x0: none

CTI device type identifier register (CTI_DEVTYPER)
Address offset: 0xFCC
Reset value: 0x0000 0014
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
Bits 7:4 SUBTYPE[3:0]: sub-classification
0x1: cross-triggering component.
Bits 3:0 MAJORTYPE[3:0]: major classification
0x4: Indicates that this component allows a debugger to control other components in a
CoreSight SoC-400 system.

CTI CoreSight peripheral identity register 4 (CTI_PIDR4)
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

RM0456 Rev 6

JEP106CON[3:0]
r

r

r

r

r

RM0456

Debug support (DBG)

CTI CoreSight peripheral identity register 0 (CTI_PIDR0)
Address offset: 0xFE0
Reset value: 0x0000 0021
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

PARTNUM[7:0]
r

r

r

r

r

Bits 31:8 Reserved, must be kept at reset value.
Bits 7:0 PARTNUM[7:0]: part number bits [7:0]
0x21: CTI part number

CTI CoreSight peripheral identity register 1 (CTI_PIDR1)
Address offset: 0xFE4
Reset value: 0x0000 00BD
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

JEP106ID[3:0]
r

r

r

PARTNUM[11:8]
r

r

r

r

r

Bits 31:8 Reserved, must be kept at reset value.
Bits 7:4 JEP106ID[3:0]: JEP106 identity code bits [3:0]
0xB: Arm JEDEC code
Bits 3:0 PARTNUM[11:8]: part number bits [11:8]
0xD: CTI part number

CTI CoreSight peripheral identity register 2 (CTI_PIDR2)
Address offset: 0xFE8
Reset value: 0x0000 000B
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

REVISION[3:0]
r

r

r

JEDEC
r

r

JEP106ID[6:4]
r

r

r

Bits 31:8 Reserved, must be kept at reset value.

RM0456 Rev 6

<!-- pagebreak -->

