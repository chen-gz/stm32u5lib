3631

Debug support (DBG)

RM0456

ETM trace identification register (ETM_TRACEIDR)
Address offset: 0x040
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

15

14

13

12

11

10

9

8

7

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

TRACEID[6:0]
rw

rw

rw

rw

Bits 31:7 Reserved, must be kept at reset value.
Bits 6:0 TRACEID[6:0]: Trace identification to output onto the trace bus
This field must be programmed with a unique value to differentiate it from other trace sources
in the system.
Values 0x00 and 0x70-0x7F are reserved.

ETM ViewInst main control register (ETM_VICTLR)
Address offset: 0x080
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

Res.

Res.

Res.

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
Res.

14
Res.

13
Res.

12
Res.

11

10

9

TRCER TRCRE SSSTA
R
SET
TUS
rw

rw

8

7

6

5

Res.

rw

4

19

18

17

16

EXLEVEL_S[3:0]
rw

rw

rw

rw

3

2

1

0

rw

rw

rw

EVENT[7:0]
rw

rw

rw

rw

rw

Bits 31:20 Reserved, must be kept at reset value.
Bits 19:16 EXLEVEL_S[3:0]: exception level in secure state
Controls whether instruction tracing is enabled for the corresponding exception level, in
secure state.
0bXXX0: instruction trace not generated in secure state, for exception level 0
0bXXX1: instruction trace generated in secure state, for exception level 0
0b0XXX: instruction trace not generated in secure state, for exception level 3
0b1XXX: instruction trace generated in secure state, for exception level 3
Bits 15:12 Reserved, must be kept at reset value.
Bit 11 TRCERR: trace system error exception
0: The system error exception is traced only if the instruction or exception immediately before
the system error exception is traced.
1: The system error exception is always traced.
Bit 10 TRCRESET: trace reset exception
0: The reset exception is traced only if the instruction or exception immediately before the
reset exception is traced.
1: The reset exception is always traced.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Debug support (DBG)

Bit 9 SSSTATUS: start/stop logic status
0: stopped
1: started
Bit 8 Reserved, must be kept at reset value.
Bits 7:0 EVENT[7:0]: event selector

ETM counter reload value register 0 (ETM_CNTRLDVR0)
Address offset: 0x140
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

rw

rw

rw

rw

rw

rw

rw

18

17

16

VALUE[15:0]
rw

rw

rw

rw

rw

rw

rw

rw

rw

Bits 31:16 Reserved, must be kept at reset value.
Bits 15:0 VALUE[15:0]: counter reload value
This value is loaded in to the counter each time the reload event occurs.

ETM identification register 8 (ETM_IDR8)
Address offset: 0x180
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

MAXSPEC[31:16]
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

r

r

r

r

r

r

r

MAXSPEC[15:0]
r

r

Bits 31:0 MAXSPEC[31:0]: maximum speculation depth
Indicates the maximum speculation depth of the instruction trace stream. This is the
maximum number of P0 elements that have not been committed in the trace stream at any
one time.
0x0: The maximum trace speculation depth is zero.

RM0456 Rev 6

<!-- pagebreak -->

3631

Debug support (DBG)

RM0456

ETM identification register 9 (ETM_IDR9)
Address offset: 0x184
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

NUMP0KEY[31:16]

NUMP0KEY[15:0]
r

r

r

r

r

r

r

r

r

Bits 31:0 NUMP0KEY[31:0]: number of P0 right-hand keys used
0x0: no P0 right-hand keys used in instruction trace

ETM identification register 10 (ETM_IDR10)
Address offset: 0x188
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

NUMP1KEY[31:16]
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

r

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

NUMP1KEY[15:0]
r

r

r

r

r

r

r

r

r

Bits 31:0 NUMP1KEY[31:0]: number of P1 right-hand keys used (including normal and special keys)
0x0: no P1 right-hand keys used in instruction trace

ETM identification register 11 (ETM_IDR11)
Address offset: 0x18C
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

NUMP1SPC[31:16]

NUMP1SPC[15:0]
r

r

r

r

r

r

r

r

r

Bits 31:0 NUMP1SPC[31:0]: number of special P1 right-hand keys used
0x0: no special P1 right-hand keys used in any configuration

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Debug support (DBG)

ETM identification register 12 (ETM_IDR12)
Address offset: 0x190
Reset value: 0x0000 0001
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

NUMCONDKEY[31:16]

NUMCONDKEY[15:0]
r

r

r

r

r

r

r

r

r

Bits 31:0 NUMCONDKEY[31:0]: number of conditional instruction right-hand keys used (including
normal and special keys)
0x1: one conditional instruction right-hand key implemented

ETM identification register 13 (ETM_IDR13)
Address offset: 0x194
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

NUMCONDSPC[31:16]
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

r

r

r

r

r

r

r

NUMCONDSPC[15:0]
r

r

Bits 31:0 NUMCONDSPC[31:0]: number of special conditional instruction right-hand keys used
0x0: no special conditional instruction right-hand keys implemented

ETM implementation specific register 0 (ETM_IMSPECR0)
Address offset: 0x1C0
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

7

6

5

4

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

SUPPORT[3:0]
r

r

r

r

Bits 31:4 Reserved, must be kept at reset value.
Bits 3:0 SUPPORT[3:0]: implementation specific extension support
0x0: no implementation specific extensions are supported

RM0456 Rev 6

<!-- pagebreak -->

3631

Debug support (DBG)

RM0456

ETM identification register 0 (ETM_IDR0)
Address offset: 0x1E0
Reset value: 0x2800 06E1
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

Res.

Res.

COMM
OPT

Res.

Res.

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

QSUPP
[0]

Res.

RETST
ACK

Res.

r

r

CONDTYPE[1:0] NUMEVENT[1:0]
r

r

r

r

TRCCC TRCC
I
OND

r

r

r

TRCBB

TRCDATA[1:0]

r

r

r

17

TRCEX QSUPP
DATA
[1]
r

r

1

0

INSTP0[1:0]
r

16

Res.

r

Bits 31:30 Reserved, must be kept at reset value.
Bit 29 COMMOPT: commit field meaning
Indicates the meaning of the commit field in some packets.
1: commit mode 1
Bits 28:18 Reserved, must be kept at reset value.
Bit 17 TRCEXDATA: trace data transfers for exceptions
Indicates support for the tracing of data transfers for exceptions and exception returns.
0: not implemented
Bits 16:15 QSUPP[1:0]: Q element support
0: not supported
Bit 14 Reserved, must be kept at reset value.
Bits 13:12 CONDTYPE[1:0]: conditional results tracing
Indicates how conditional results are traced.
0: The trace unit indicates only if a conditional instruction passes or fails its condition code
check
Bits 11:10 NUMEVENT[1:0]: Number of events supported
0x1: two events
Bit 9 RETSTACK: return stack support
1: two entry return stacks
Bit 8 Reserved, must be kept at reset value.
Bit 7 TRCCCI: cycle counting support
1: cycle counting implemented
Bit 6 TRCCOND: conditional instruction support
1: conditional instruction tracing implemented
Bit 5 TRCBB: branch broadcast support
1: branch broadcast tracing implemented
Bits 4:3 TRCDATA[1:0]: data tracing support
0x0: data tracing not supported
Bits 2:1 INSTP0[1:0]: support for tracing of load and store instructions as P0 elements
0x0: not supported
Bit 0 Reserved, must be kept at reset value.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Debug support (DBG)

ETM identification register 1 (ETM_IDR1)
Address offset: 0x1E4
Reset value: 0x4100 F421
31

30

29

r

r

r

28

27

26

25

24

r

r

r

10

9

8

DESIGNER[7:0]
r

r
11

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

6

5

4

3

2

1

0

7

TRCARCHMAJ[3:0]
r

r

TRCARCHMIN[3:0]

r

r

r

r

r

REVISION[3:0]
r

r

r

r

r

18

17

16

Bits 31:24 DESIGNER[7:0]: trace unit designer
0x41: Arm
Bits 23:12 Reserved, must be kept at reset value.
Bits 11:8 TRCARCHMAJ[3:0]: major trace unit architecture version number
0x4: ETMv4
Bits 7:4 TRCARCHMIN[3:0]: minor trace unit architecture version number
0x2: minor revision 2
Bits 3:0 REVISION[3:0]: implementation revision number
0x1: implementation revision 1

ETM identification register 2 (ETM_IDR2)
Address offset: 0x1E8
Reset value: 0x0000 0004
31

30

29

Res.

Res.

Res.

15

14

13

DASIZ
E[0]
r

28

27

26

25

24

r

22

21

20

19

DVSIZE[4:0]

DASIZE[4:1]

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

VMIDSIZE[4:0]
r

23

CCSIZE[3:0]

r

CIDSIZE[4:0]
r

r

r

r

r

IASIZE[4:0]
r

r

r

r

r

Bits 31:29 Reserved, must be kept at reset value.
Bits 28:25 CCSIZE[3:0]: cycle counter size
0x0: 12 bits
Bits 24:20 DVSIZE[4:0]: data value size
0x0: data value size not supported
Bits 19:15 DASIZE[4:0]: data address size.
0x0: data address size not supported
Bits 14:10 VMIDSIZE[4:0]: virtual machine ID size
0x0: virtual machine ID tracing not implemented
Bits 9:5 CIDSIZE[4:0]: context ID size
0x0: context ID tracing not implemented

RM0456 Rev 6

<!-- pagebreak -->

