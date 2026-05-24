3631

Debug support (DBG)

RM0456

Bits 4:0 IASIZE[4:0]: instruction address size
0x4: maximum 32-bit address size

ETM identification register 3 (ETM_IDR3)
Address offset: 0x1EC
Reset value: 0x0F09 0004
31

30

NOOV
ERFLO
W
r

29

28

r

26

25

24

SYSST STALL SYNCP TRCER
ALL
CTL
R
R

NUMPROC[2:0]
r

27

r

r

r

r

r

11

10

9

8

15

14

13

12

Res.

Res.

Res.

Res.

23

22

21

20

Res.

Res.

Res.

Res.

7

6

5

19

18

17

EXLEVEL_S[3:0]
r

r

r

r

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

CCITMIN[11:0]
r

r

r

r

r

r

r

Bit 31 NOOVERFLOW: ETM_STALLCTLR.NOOVERFLOW implementation
0: not implemented
Bits 30:28 NUMPROC[2:0]: number of processors available for tracing
0x0: one processor
Bit 27 SYSSTALL: system support for stall control of the processor
1: system supports stall control
Bit 26 STALLCTL: stall control support
1: ETM_STALLCTLR implemented
Bit 25 SYNCPR: trace synchronization period support
1: ETM_SYNCPR is read-only for instruction trace only configuration. The trace
synchronization period is fixed.
Bit 24 TRCERR: ETM_VICTLR.TRCERR implementation
0x1: implemented
Bits 23:20 Reserved, must be kept at reset value.
Bits 19:16 EXLEVEL_S[3:0]: privilege levels implementation
0x9: privilege levels thread and handler implemented
Bits 15:12 Reserved, must be kept at reset value.
Bits 11:0 CCITMIN[11:0]: minimum value that can be programmed to ETM_CCCTLR.THRESHOLD
Defines the minimum cycle counting threshold.
0x4: minimum of four-instruction trace cycles

<!-- pagebreak -->

16

RM0456 Rev 6

RM0456

Debug support (DBG)

ETM identification register 4 (ETM_IDR4)
Address offset: 0x1F0
Reset value: 0x0011 4000
31

30

29

28

27

NUMVMIDC[3:0]

26

25

24

23

NUMCIDC[3:0]

22

21

20

19

NUMSSCC[3:0]

18

17

16

NUMRSPAIR[3:0]

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

Res.

Res.

Res.

SUPPD
AC

NUMPC[3:0]
r

r

r

r

NUMDVC[3:0]

r

r

r

r

NUMACPAIRS[3:0]
r

r

r

r

r

19

18

17

16

Bits 31:28 NUMVMIDC[3:0]: number of virtual machine ID (VMID) comparators
0x0: VMID comparators not implemented
Bits 27:24 NUMCIDC[3:0]: number of context ID comparators
0x0: context ID comparators not supported
Bits 23:20 NUMSSCC[3:0]: number of single-shot comparator controls
0x1: one single-shot comparator control implemented
Bits 19:16 NUMRSPAIR[3:0]: number of resource selection pairs
0x1: two resource selection pairs implemented
Bits 15:12 NUMPC[3:0]: number of processor comparator inputs for the DWT
0x4: four processor comparator inputs implemented
Bits 11:9 Reserved, must be kept at reset value.
Bit 8 SUPPDAC: data address comparisons
0: data address comparisons not supported
Bits 7:4 NUMDVC[3:0]: number of data value comparators
0x0: no data value comparators implemented
Bits 3:0 NUMACPAIRS[3:0]: number of address comparator pairs
0x0: no address comparator pairs implemented

ETM identification register 5 (ETM_IDR5)
Address offset: 0x1F4
Reset value: 0x90C7 0004
31

30

REDFU
NCNTR

29

28

NUMCNTR[2:0]

27

26

25

NUMSEQSTATE[2:0]

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

Res.

Res.

Res.

Res.

NUMEXTINSEL[2:0]
r

r

r

24
Res.

23

22

21

20

LPOVE ATBTRI
RRIDE
G

TRACEIDSIZE[5:0]

r

r

r

r

r

r

r

r

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

r

NUMEXTIN[8:0]
r

Bit 31 REDFUNCNTR: reduced function counter
1: counter 0 implemented as a reduced function counter

RM0456 Rev 6

<!-- pagebreak -->

