3631

Debug support (DBG)

RM0456

TPIU CoreSight peripheral identity register 4 (TPIU_PIDR4)
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

JEP106CON[3:0]
r

r

r

r

r

Bits 31:8 Reserved, must be kept at reset value.
Bits 7:4 SIZE[3:0]: register file size
0x0: The register file occupies a single 4-Kbyte region.
Bits 3:0 JEP106CON[3:0]: JEP106 continuation code
0x4: Arm JEDEC code

TPIU CoreSight peripheral identity register 0 (TPIU_PIDR0)
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

PARTNUM[7:0]
r

r

Bits 31:8 Reserved, must be kept at reset value.
Bits 7:0 PARTNUM[7:0]: part number bits [7:0]
0x21: TPIU part number

TPIU CoreSight peripheral identity register 1 (TPIU_PIDR1)
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

Bits 31:8 Reserved, must be kept at reset value.

<!-- pagebreak -->

RM0456 Rev 6

r

r

PARTNUM[11:8]
r

r

r

r

r

RM0456

Debug support (DBG)

Bits 7:4 JEP106ID[3:0]: JEP106 identity code bits [3:0]
0xB: Arm JEDEC code
Bits 3:0 PARTNUM[11:8]: part number bits [11:8]
0xD: TPIU part number

TPIU CoreSight peripheral identity register 2 (TPIU_PIDR2)
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
Bits 7:4 REVISION[3:0]: component revision number
0x0: r0p0
Bit 3 JEDEC: JEDEC assigned value
0x1: designer identification specified by JEDEC
Bits 2:0 JEP106ID[6:4]: JEP106 identity code bits [6:4]
0x3: Arm JEDEC code

TPIU CoreSight peripheral identity register 3 (TPIU_PIDR3)
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

RM0456 Rev 6

<!-- pagebreak -->

3631

Debug support (DBG)

RM0456

TPIU CoreSight component identity register 0 (TPIU_CIDR0)
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

r

r

r

Bits 31:8 Reserved, must be kept at reset value.
Bits 7:0 PREAMBLE[7:0]: component identification bits [7:0]
0x0D: common identification value

TPIU CoreSight peripheral identity register 1 (TPIU_CIDR1)
Address offset: 0xFF4
Reset value: 0x0000 0090
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

PREAMBLE[11:8]

r

r

r

r

r

r

Bits 31:8 Reserved, must be kept at reset value.
Bits 7:4 CLASS[3:0]: component ID bits [15:12] - component class
0x9: CoreSight component
Bits 3:0 PREAMBLE[11:8]: component identification bits [11:8]
0x0: common identification value

TPIU CoreSight component identity register 2 (TPIU_CIDR2)
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

Bits 31:8 Reserved, must be kept at reset value.

<!-- pagebreak -->

RM0456 Rev 6

r

r

r

r

RM0456

Debug support (DBG)

Bits 7:0 PREAMBLE[19:12]: component identification bits [23:16]
0x05: common identification value

TPIU CoreSight component identity register 3 (TPIU_CIDR3)
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

75.10.2

TPIU register map

0

0

0

0

0

0

0

0

0

TPIU_CSPSR
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

1

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

TPIU_SPPR

0

Reserved
Res.

0x0F0

Reserved

0

Res.

Reset value
0x014 to
0x0EC

PRESCALER[12:0]

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

Reserved
Res.

Reserved
TPIU_ACPR

0

PORTSIZE[31:0]

Res.

Reset value

0

Res.

0

TXMODE
[1:0]

0

Res.

0

Res.

0

Res.

0x010

PORTSIZE[31:0]
0

Res.

0x008

Reset value

Res.

0x004

TPIU_SSPSR

Res.

0x000

Res.

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

Table 805. TPIU register map and reset values

Reset value

Reset value

1

RM0456 Rev 6

0

FTNONSTOP

TCPRESENT

FTSTOPPED

FLINPROG

1

0

0

0

Res.

ENFCONT

Res.

Res.

Res.

FONMAN

Res.

TRIGIN

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

TPIU_FFCR

Res.

Reset value

0x304

1

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

TPIU_FFSR

0
Reserved

Res.

0x300

Reserved

Res.

0x0F4 to
0x2FC

1

<!-- pagebreak -->

3631

0xFFC

<!-- pagebreak -->

TPIU_CIDR3

RM0456 Rev 6
Res.

1
1
1

TPIU_PIDR2
REVISION
[3:0]
JEDEC
JEP106ID
[6:4]

0
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

Reset value
TCLKDATA

Reset value

Reset value

Reset value

Reset value

Reset value

Refer to Table 797: MCU ROM table for register boundary addresses.

1

1
SUBTYPE
[3:0]
MAJORTYPE
[3:0]

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

1

1

0

0

0

CLASS[3:0]
PREAMBLE
[11:8]

1

0

0

0

0

0

0

1

Res.

0

Reserved
0
0

Res.

Res.

Res.

0

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

0

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

0

CLKRELAT

FIFOSIZE[2:0]

SWOMAN

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

SWOUARTNRZ:

0

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

0

Res.

Res.
1

Res.

1

Res.

Res.

Res.

Res.

Reset value
Res.

1

Res.

Res.

Res.

Res.

PARTNUM
[11:8]

Res.

Res.

Res.

0

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

JEP106ID
[3:0]

Res.

Res.

Res.

0

Res.

Res.

Res.

Res.

Res.

Res.

Res.

TPIU_PIDR1

Res.

Res.

Res.

Reset value
0

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

0

Res.

0

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

0

Res.

Reset value

Res.

Reset value

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

Reset value
0

Res.

Reserved

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
0

Res.

Reserved

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

TPIU_CIDR2

Res.

0xFF8
TPIU_CIDR1

Res.

0xFF4
TPIU_CIDR0

Res.

0xFF0
TPIU_PIDR3

Res.

0xFEC

Res.

0xFE8

Res.

0xFE4
TPIU_PIDR0

Res.

0xFE0
TPIU_PIDR4

Res.

0xFD0

Res.

TPIU_DEVTYPER

Res.

0xFCC
TPIU_DEVIDR

Res.

0FA8 to
0xFC4

Res.

0xFC8
TPIU_CLAIMCLR

Res.

0xFA4
TPIU_CLAIMSETR

Res.

030C to
0xF9C

Res.

0xFA0
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

TPIU_PSCR

Res.

0x308

Res.

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

Offset Register name

Res.

Debug support (DBG)
RM0456

Table 805. TPIU register map and reset values (continued)

PSCOUNT[12:0]

CLAIMSET
[3:0]

Reset value
1

Reset value
0

MaXNUM[4:0]

1
0

0
1

SIZE[3:0]
0

0

0

0

0

1

0

1

0

0

0

0

1

0

0

0

1

0

0

0

1

0

0

1

0

1

0

0

1

0

0

0

0

1

0

0

0

0

1

CLAIMCLR
[3:0]
0

Reserved

1

1

JEP106CON
[3:0]

PARTNUM[7:0]

0
0

0
1

1

1

REVAND[3:0] CMOD[3:0]

PREAMBLE[7:0]
0
0

0
1

0

PREAMBLE[19:12]

1

PREAMBLE[27:20]

1

RM0456

75.11

Debug support (DBG)

Cross-trigger interface (CTI)
The CTI allows cross triggering between the processor and the ETM (see the figure below).
Figure 960. Embedded cross trigger
PPB
ETMTRIGGER0
ETMTRIGGER1

DWT

ETMTRIGGER2
HALTED
Cortex-M33
CPU

EDBGRQ
DBGRESTART
ETMTRIGOUT0
ETMTRIGOUT1
ETMEXTIN0

ETM

ETMEXTIN1
ETMEXTIN2
ETMEXTIN3

Cortex-M33 CTI
TRIGIN1
TRIGIN2
TRIGIN3
TRIGIN0
TRIGOUT0
TRIGOUT1
TRIGIN4
TRIGIN5
TRIGOUT4
TRIGOUT5
TRIGOUT6
TRIGOUT7
MSv49705V1

The CTI enables events from various sources to trigger debug and/or trace activity. For
example, a watchpoint reached in the processor can start or stop code trace, or a trace
comparator can halt the processor.
The trigger input and output signals for the CTI are listed in the tables below.
Table 806. CTI inputs
Number

Source signal

Source component

Comments

0

HALTED

CPU

Processor halted - CPU is in debug mode

1

ETMTRIGGER0

DWT

DWT comparator output 0

2

ETMTRIGGER1

DWT

DWT comparator output 1

3

ETMTRIGGER2

DWT

DWT comparator output 2

4

ETMTRIGOUT0

ETM

ETM event output 0

5

ETMTRIGOUT1

ETM

ETM event output 1

6

-

-

Not used

7

-

-

Not used

Table 807. CTI outputs
Number

Source signal

Destination component

Comments

0

EDBGRQ

CPU

CPU halt request - Puts CPU in debug mode

1

DBGRESTART

CPU

CPU restart request - CPU exits debug mode

RM0456 Rev 6

<!-- pagebreak -->

3631

Debug support (DBG)

RM0456
Table 807. CTI outputs (continued)

Number

Source signal

Destination component

Comments

2

-

-

Not used

3

-

-

Not used

4

ETMEXTIN0

ETM

ETM event input 0

5

ETMEXTIN1

ETM

ETM event input 1

6

ETMEXTIN2

ETM

ETM event input 2

7

ETMEXTIN3

ETM

ETM event input 3

For more information on the cross-trigger interface CoreSight component, refer to the Arm
CoreSight SoC-400 Technical Reference Manual [2].

75.11.1

CTI registers
The register file base address for the CTI is 0xE004 2000.

CTI control register (CTI_CONTROLR)
Address offset: 0x000
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

Res.

Res.

Res.

Res.

Res.

Res.

Res.

GLBEN
rw

Bits 31:1 Reserved, must be kept at reset value.
Bit 0 GLBEN: global CTI enable
0: disabled
1: enabled

CTI trigger acknowledge register (CTI_INTACKR)
Address offset: 0x010
Reset value: 0xXXXX XXXX
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

w

w

w

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

INTACK[7:0]
w

Bits 31:8 Reserved, must be kept at reset value.

<!-- pagebreak -->

