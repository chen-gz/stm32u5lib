RM0456 Rev 6

rw

rw

rw

RM0456

Debug support (DBG)

TPIU device configuration register (TPIU_DEVIDR)
Address offset: 0xFC8
Reset value: 0x0000 0CA1
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

Res.

Res.

Res.

Res.

SWOU
SWOM TCLKD
ARTNR
AN
ATA
Z
r

r

r

CLKRE
LAT

FIFOSIZE[2:0]
r

r

r

r

MAXNUM[4:0]
r

r

r

Bits 31:12 Reserved, must be kept at reset value.
Bit 11 SWOUARTNRZ: Serial-wire output, NRZ support
0x1: supported
Bit 10 SWOMAN: Serial-wire output, Manchester encoded format, support
0x1: supported
Bit 9 TCLKDATA: trace clock plus data support
0x0: supported
Bits 8:6 FIFOSIZE[2:0]: FIFO size in powers of 2
0x2: FIFO size = 4 bytes
Bit 5 CLKRELAT: ATB clock and TRACECLKIN relationship (synchronous or asynchronous)
0x1: asynchronous
Bits 4:0 MAXNUM[4:0]: number/type of ATB input port multiplexing
0x1: two input ports

TPIU device type identifier register (TPIU_DEVTYPER)
Address offset: 0xFCC
Reset value: 0x0000 0011
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

SUBTYPE[3:0]
r

r

r

MAJORTYPE[3:0]
r

r

r

Bits 31:8 Reserved, must be kept at reset value.
Bits 7:4 SUBTYPE[3:0]: sub-classification
0x1: trace port component
Bits 3:0 MAJORTYPE[3:0]: major classification
0x1: trace sink component

RM0456 Rev 6

<!-- pagebreak -->

