RM0456 Rev 6

RM0456

Graphic timer (GFXTIM)

59.5.3

GFXTIM timers configuration register (GFXTIM_TCR)
Address offset: 0x008
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

FRFC2 RFC2C RFC2E
R
M
N

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

w

rw

w

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

FALCR ALCEN
w

19
Res.

18

17

16

FRFC1 RFC1C RFC1E
R
M
N
w

rw

w

3

2

1

0

Res.

Res.

w

FAFCR AFCEN
w

w

Bits 31:23 Reserved, must be kept at reset value.
Bit 22 FRFC2R: Force relative frame counter 2 reload
This bit forces the reload of the relative frame counter 2.
0: No effect
1: Relative frame counter 2 reload forced
Bit 21 RFC2CM: Relative frame counter 2 continuous mode
This bit enables the continuous mode of the relative frame counter 2.
0: Relative frame counter 2 is one shot.
1: Relative frame counter 2 is in continuous mode.
Bit 20 RFC2EN: Relative frame counter 2 enable
This bit enables the relative frame counter 2.
0: No effect
1: Relative frame counter 2 enabled
Bit 19 Reserved, must be kept at reset value.
Bit 18 FRFC1R: Force relative frame counter 1 reload
This bit forces the reload of the relative frame counter 1.
0: No effect
1: Relative frame counter 1 reload forced
Bit 17 RFC1CM: Relative frame counter 1 continuous mode
This bit enables the continuous mode of the relative frame counter 1.
0: Relative frame counter 1 is one shot.
1: Relative frame counter 1 is in continuous mode.
Bit 16 RFC1EN: Relative frame counter 1 enable
This bit enables the relative frame counter 1.
0: No effect
1: Relative frame counter enabled
Bits 15:6 Reserved, must be kept at reset value.
Bit 5 FALCR: Force absolute line counter reset
This bit forces the reset of the absolute line counter.
0: No effect
1: Absolute line counter reset forced

RM0456 Rev 6

<!-- pagebreak -->

2562

Graphic timer (GFXTIM)

RM0456

Bit 4 ALCEN: Absolute line counter enable
This bit enables the absolute line counter.
0: No effect
1: Absolute line counter enabled
Bits 3:2 Reserved, must be kept at reset value.
Bit 1 FAFCR: Force absolute frame counter reset
This bit forces the reset of the absolute frame counter.
0: No effect
1: Absolute frame counter reset forced
Bit 0 AFCEN: Absolute frame counter enable
This bit enables the absolute frame counter.
0: No effect
1: Absolute frame counter enabled

59.5.4

GFXTIM timers disable register (GFXTIM_TDR)
Address offset: 0x00C
Reset value: 0x0000 0000

31
Res.

30

29

Res.

Res.

28
Res.

27
Res.

26
Res.

25
Res.

24
Res.

23
Res.

22
Res.

21

20

Res.

RFC2D
IS

19
Res.

18
Res.

17

16

Res.

RFC1D
IS

w
15
Res.

14

13

Res.

Res.

12
Res.

11
Res.

10
Res.

9
Res.

8
Res.

7
Res.

6
Res.

5

4

Res.

ALCDI
S
w

Bits 31:21 Reserved, must be kept at reset value.
Bit 20 RFC2DIS: Relative frame counter 2 disable
This bit disables the relative frame counter 2.
0: No effect
1: Relative frame counter 2 disabled
Bits 19:17 Reserved, must be kept at reset value.
Bit 16 RFC1DIS: Relative frame counter 1 disable
This bit disables the relative frame counter 1.
0: No effect
1: Relative frame counter 1 disabled
Bits 15:5 Reserved, must be kept at reset value.
Bit 4 ALCDIS: Absolute line counter disable
This bit disables the absolute line counter.
0: No effect
1: Absolute line counter disabled
Bits 3:1 Reserved, must be kept at reset value.

<!-- pagebreak -->

RM0456 Rev 6

w
3
Res.

2
Res.

1

0

Res.

AFCDI
S
w

RM0456

Graphic timer (GFXTIM)

Bit 0 AFCDIS: Absolute frame counter disable
This bit disables the absolute frame counter.
0: No effect
1: Absolute frame counter disabled

59.5.5

GFXTIM events control register (GFXTIM_EVCR)
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

EV4EN EV3EN EV2EN EV1EN
rw

rw

19

18

rw

rw

17

16

Bits 31:4 Reserved, must be kept at reset value.
Bit 3 EV4EN: Event 4 enable
This bit enables the complex event 4 generation.
0: Event 4 generation disabled
1: Event 4 generation enabled
Bit 2 EV3EN: Event 3 enable
This bit enables the complex event 3 generation.
0: Event 3 generation disabled
1: Event 3 generation enabled
Bit 1 EV2EN: Event 2 enable
This bit enables the complex event 2 generation.
0: Event 2 generation disabled
1: Event 2 generation enabled
Bit 0 EV1EN: Event 1 enable
This bit enables the complex event 1 generation.
0: Event 1 generation disabled
1: Event 1 generation enabled

59.5.6

GFXTIM events selection register (GFXTIM_EVSR)
Address offset: 0x014
Reset value: 0x0000 0000

31

30

Res.

15

29

28

FES4[2:0]
rw

rw

rw

14

13

12

Res.

FES2[2:0]
rw

rw

27

26

Res.

11

24

rw

rw

rw

9

8

LES2[2:0]
rw

rw

23

22

Res.

10

Res.
rw

25
LES4[2:0]

7

RM0456 Rev 6

20

Res.

rw

rw

rw

6

5

4

Res.
rw

21
FES3[2:0]

FES1[2:0]
rw

rw

3

LES3[2:0]
rw

rw

rw

2

1

0

Res.
rw

LES1[2:0]
rw

rw

rw

<!-- pagebreak -->

