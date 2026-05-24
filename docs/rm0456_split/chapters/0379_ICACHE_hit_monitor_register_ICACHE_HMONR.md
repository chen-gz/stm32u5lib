RM0456 Rev 6

RM0456

Instruction cache (ICACHE)

Bit 0 Reserved, must be kept at reset value.

8.7.5

ICACHE hit monitor register (ICACHE_HMONR)
Address offset: 0x010
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

r

r

r

r

r

r

r

HITMON[31:16]

HITMON[15:0]
r

r

r

r

r

r

r

r

r

Bits 31:0 HITMON[31:0]: cache hit monitor counter

8.7.6

ICACHE miss monitor register (ICACHE_MMONR)
Address offset: 0x014
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

r

r

r

r

r

r

r

MISSMON[15:0]
r

r

r

r

r

r

r

r

r

Bits 31:16 Reserved, must be kept at reset value.
Bits 15:0 MISSMON[15:0]: cache miss monitor counter

8.7.7

ICACHE region x configuration register (ICACHE_CRRx)
Address offset: 0x020 + 0x4 * x, (x = 0 to 3)
Reset value: 0x0000 0200
Define an alias address in the code region for other regions, making them cacheable.
BASEDADDR and REMAPADDR fields are write locked (read only) when EN = 1 in
ICACHE_CR.

31

30

29

28

27

HBURST

Res.

Res.

MSTSEL

Res.

15

14

13

12

REN

Res.

Res.

Res.

rw

rw

rw
11

26

25

23

22

21

20

19

18

17

16

REMAPADDR[31:21]
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

rw

rw

rw

rw

rw

rw

RSIZE[2:0]
rw

24

rw

Res.
rw

BASEADDR[28:21]

RM0456 Rev 6

rw

rw

<!-- pagebreak -->

381

Instruction cache (ICACHE)

RM0456

Bit 31 HBURST: output burst type for region x
0: WRAP
1: INCR
Bits 30:29 Reserved, must be kept at reset value.
Bit 28 MSTSEL: AHB cache master selection for region x
0: no action (master1 selected by default)
1: master2 selected
Bit 27 Reserved, must be kept at reset value.
Bits 26:16 REMAPADDR[31:21]: remapped address for region x
This field replaces the alias address defined by BASEADDR field.
The only useful bits are [31:RI], where 21 ≤ RI ≤ 27 is the number of bits of RSIZE
(see Section 8.4.7). If the programmed value has more LSBs, the useless bits are ignored.
Bit 15 REN: enable for region x
0: disabled
1: enabled
Bits 14:12 Reserved, must be kept at reset value.
Bits 11:9 RSIZE[2:0]: size for region x
000: reserved
001: 2 Mbytes
010: 4 Mbytes
011: 8 Mbytes
100: 16 Mbytes
101: 32 Mbytes
110: 64 Mbytes
111: 128 Mbytes
Bit 8 Reserved, must be kept at reset value.
Bits 7:0 BASEADDR[28:21]: base address for region x
This alias address is replaced by REMAPADDR field.
The only useful bits are [28:RI], where 21 ≤ RI ≤ 27 is the number of bits of RSIZE
(see Section 8.4.7). If the programmed value has more LSBs, the useless bits are ignored.

<!-- pagebreak -->

