3631

Debug support (DBG)

RM0456

CTI trigger output status register (CTI_TRGOSTSR)
Address offset: 0x134
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

TRIGOUTSTATUS[7:0]
r

r

r

r

r

Bits 31:8 Reserved, must be kept at reset value.
Bits 7:0 TRIGOUTSTATUS[7:0]: trigger output status
There is one bit of the register for each CTITRIGOUT output. When a bit is set to 1, it
indicates that the corresponding trigger output is active. When it is set to 0, the corresponding
trigger output is inactive.

CTI channel input status register (CTI_CHINSTSR)
Address offset: 0x138
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

CHINSTATUS[3:0]
r

r

r

r

Bits 31:4 Reserved, must be kept at reset value.
Bits 3:0 CHINSTATUS[3:0]: channel input status
There is one bit of the register for each channel input. When a bit is set to 1 it indicates that
the corresponding channel input is active. When it is set to 0, the corresponding channel
input is inactive.

CTI channel output status register (CTI_CHOUTSTSR)
Address offset: 0x13C
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

CHOUTSTATUS[3:0]
r

<!-- pagebreak -->

