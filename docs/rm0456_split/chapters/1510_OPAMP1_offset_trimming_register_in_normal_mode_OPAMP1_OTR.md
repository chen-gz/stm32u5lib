1514

Operational amplifier (OPAMP)

38.5.2

RM0456

OPAMP1 offset trimming register in normal mode (OPAMP1_OTR)
Address offset: 0x04
Reset value: 0x0000 XXXX
XXXX are factory trimmed values.

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

4

3

2

1

0

15

14

13

Res.

Res.

Res.

TRIMOFFSETP[4:0]
rw

rw

rw

rw

7

6

5

Res.

Res.

Res.

rw

TRIMOFFSETN[4:0]
rw

rw

rw

rw

rw

Bits 31:13 Reserved, must be kept at reset value.
Bits 12:8 TRIMOFFSETP[4:0]: Trim for PMOS differential pairs
Bits 7:5 Reserved, must be kept at reset value.
Bits 4:0 TRIMOFFSETN[4:0]: Trim for NMOS differential pairs

38.5.3

OPAMP1 offset trimming register in low-power mode
(OPAMP1_LPOTR)
Address offset: 0x08
Reset value: 0x0000 XXXX
XXXX are factory trimmed values.

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

4

3

2

1

0

15

14

13

Res.

Res.

Res.

TRIMLPOFFSETP[4:0]
rw

rw

rw

rw

7

6

5

Res.

Res.

Res.

rw

TRIMLPOFFSETN[4:0]
rw

rw

Bits 31:13 Reserved, must be kept at reset value.
Bits 12:8 TRIMLPOFFSETP[4:0]: Low-power mode trim for PMOS differential pairs
Bits 7:5 Reserved, must be kept at reset value.
Bits 4:0 TRIMLPOFFSETN[4:0]: Low-power mode trim for NMOS differential pairs

<!-- pagebreak -->

