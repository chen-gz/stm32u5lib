3631

Debug support (DBG)

RM0456

clock output, TRACECLK, and four data outputs, TRACEDATA(3:0). The trace port width is
programmable in the range 1 to 4. Using a smaller port width reduces the number of test
points/connector pins needed, and frees up IOs for other purposes, at the expenses of
bandwidth restriction of the trace port, and hence of the quantity of trace information that
can be output in real time.
Figure 959. Trace port interface unit (TPIU)

ATB
interface

ETM ATB

TRACECLK

Trace
output
(serialiser)

Formatter

TRACEDATA(3:0)

ATB
interface

ITM ATB

Cortex-M33 private
peripheral bus
(PPB)

TRACESWO

APB
interface
MSv49704V1

Trace data can also be output on the serial-wire output, TRACESWO.
For more information on the trace port interface in the Cortex-M33, refer to the Arm
Cortex-M33 Technical Reference Manual [4].

75.10.1

TPIU registers
TPIU supported port size register (TPIU_SSPSR)
Address offset: 0x000
Reset value: 0x0000 000F

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

PORTSIZE[31:16]
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

PORTSIZE[15:0]
r

r

r

r

r

r

r

r

r

Bits 31:0 PORTSIZE[31:0]: trace port sizes, from 1 to 32 pins
Bit n-1 when set, indicates that port size n is supported.
0x0000 000F: port sizes 1 to 4 supported

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Debug support (DBG)

TPIU current port size register (TPIU_CSPSR)
Address offset: 0x004
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

PORTSIZE[31:16]

PORTSIZE[15:0]
r

r

r

r

r

r

r

r

r

Bits 31:0 PORTSIZE[31:0]: current trace port size
Bit n-1 when set, indicates that the current port size is n pins. The value of n must be within
the range of supported port sizes (1-4). Only one bit can be set, or unpredictable behavior
may result.
This register must only be modified when the formatter is stopped.

TPIU asynchronous clock prescaler register (TPIU_ACPR)
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

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

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

15

14

13

Res.

Res.

Res.

PRESCALER[12:0]
rw

rw

rw

rw

rw

rw

rw

rw

Bits 31:13 Reserved, must be kept at reset value.
Bits 12:0 PRESCALER[12:0]: baud rate for the asynchronous output, TRACESWO
The baud rate is given by the TRACELKIN frequency divided by (PRESCALER +1).

TPIU selected pin protocol register (TPIU_SPPR)
Address offset: 0x0F0
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

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

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

3

2

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

TXMODE[1:0]
rw

rw

Bits 31:2 Reserved, must be kept at reset value.

RM0456 Rev 6

<!-- pagebreak -->

