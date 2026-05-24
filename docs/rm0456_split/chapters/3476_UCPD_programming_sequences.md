RM0456 Rev 6

rw

RM0456

Debug support (DBG)

Bits 3:0 PC[3:0]: processor comparator inputs selection for single-shot control
0xXXX0: processor comparator input 0 not selected
0xXXX1: processor comparator input 0 selected
0xXX0X: Processor comparator input 1 not selected
0xXX1X: processor comparator input 1 selected
0xX0XX: processor comparator input 2 not selected
0xX1XX: processor comparator input 2 selected
0x0XXX: processor comparator input 3 not selected
0x1XXX: processor comparator input 3 selected

ETM power-down control register (ETM_PDCR)
Address offset: 0x310
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

PU

Res.

Res.

Res.

rw

Bits 31:4 Reserved, must be kept at reset value.
Bit 3 PU: power-up request
0: power-up not requested
1: power-up requested
Bits 2:0 Reserved, must be kept at reset value.

ETM power-down status register (ETM_PDSR)
Address offset: 0x314
Reset value: 0x0000 0003
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

STICK
YPD

POWE
R

r

r

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Bits 31:2 Reserved, must be kept at reset value.
Bit 1 STICKYPD: sticky power-down state
0: Trace register power has not been removed since the ETM_PDSR was last read.
1: Trace register power has been removed since the ETM_PDSR was last read.
Bit 0 POWER: ETM power-up status
1: ETM powered up

RM0456 Rev 6

<!-- pagebreak -->

3631

Debug support (DBG)

RM0456

ETM claim tag set register (ETM_CLAIMSETR)
Address offset: 0xFA0
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

Res.

Res.

Res.

Res.

Res.

Res.

Res.

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

CLAIMSET[3:0]
rw

rw

rw

rw

Bits 31:4 Reserved, must be kept at reset value.
Bits 3:0 CLAIMSET[3:0]: claim tag bits setting
Write:
0000: no effect
xxx1: Sets bit 0.
xx1x: Sets bit 1.
x1xx: Sets bit 2.
1xxx: Sets bit 3.
Read:
0xF: Indicates there are four bits in claim tag.

ETM claim tag clear register (ETM_CLAIMCLR)
Address offset: 0xFA4
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

CLAIMCLR[3:0]
rw

Bits 31:4 Reserved, must be kept at reset value.
Bits 3:0 CLAIMCLR[3:0]: claim tag bits reset
Write:
0000: no effect
xxx1: Clears bit 0.
xx1x: Clears bit 1.
x1xx: Clears bit 2.
1xxx: Clears bit 3.
Read: Returns current value of claim tag.

<!-- pagebreak -->

