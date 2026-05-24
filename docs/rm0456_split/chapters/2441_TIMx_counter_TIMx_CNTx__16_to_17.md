2562

Graphic timer (GFXTIM)

RM0456

Bit 31 Reserved, must be kept at reset value.
Bits 30:28 FES4[2:0]: Frame-event selection 4
This bitfield defines the frame-event selection for complex event 4 generation.
000: No frame event
001: Absolute frame counter overflow
010: Absolute frame counter compare
100: Relative frame counter 1 reload
101: Relative frame counter 2 reload
Others: Reserved
Bit 27 Reserved, must be kept at reset value.
Bits 26:24 LES4[2:0]: Line-event selection 4
This bitfield defines the line-event selection for complex event 4 generation.
000: No line event
001: Absolute line counter overflow
010: Tearing effect
100: Absolute line counter 1 compare
101: Absolute line counter 2 compare
Others: Reserved
Bit 23 Reserved, must be kept at reset value.
Bits 22:20 FES3[2:0]: Frame-event selection 3
This bitfield defines the frame-event selection for complex event 3 generation.
000: No frame event
001: Absolute frame counter overflow
010: Absolute frame counter compare
100: Relative frame counter 1 reload
101: Relative frame counter 2 reload
Others: Reserved
Bit 19 Reserved, must be kept at reset value.
Bits 18:16 LES3[2:0]: Line-event selection 3
This bitfield defines the line-event selection for complex event 3 generation.
000: No line event
001: Absolute line counter overflow
010: Tearing effect
100: Absolute line counter 1 compare
101: Absolute line counter 2 compare
Others: Reserved
Bit 15 Reserved, must be kept at reset value.
Bits 14:12 FES2[2:0]: Frame-event selection 2
This bitfield defines the frame-event selection for complex event 2 generation.
000: No frame event
001: Absolute frame counter overflow
010: Absolute frame counter compare
100: Relative frame counter 1 reload
101: Relative frame counter 2 reload
Others: Reserved
Bit 11 Reserved, must be kept at reset value.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Graphic timer (GFXTIM)

Bits 10:8 LES2[2:0]: Line-event selection 2
This bitfield defines the line-event selection for complex event 2 generation.
000: No line event
001: Absolute line counter overflow
010: Tearing effect
100: Absolute line counter 1 compare
101: Absolute line counter 2 compare
Others: Reserved
Bit 7 Reserved, must be kept at reset value.
Bits 6:4 FES1[2:0]: Frame-event selection 1
This bitfield defines the frame-event selection for complex event 1 generation.
000: No frame event
001: Absolute frame counter overflow
010: Absolute frame counter compare
100: Relative frame counter 1 reload
101: Relative frame counter 2 reload
Others: Reserved
Bit 3 Reserved, must be kept at reset value.
Bits 2:0 LES1[2:0]: Line-event selection 1
This bitfield defines the line-event selection for complex event 1 generation.
000: No line event
001: Absolute line counter overflow
010: Tearing effect
100: Absolute line counter 1 compare
101: Absolute line counter 2 compare
Others: Reserved

59.5.7

GFXTIM watchdog timer configuration register
(GFXTIM_WDGTCR)
Address offset: 0x020
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
FWDG
R

Res.

Res.

Res.

Res.

Res.

Res.

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

Res.

Res.

Res.

Res.

Res.

Res.

Res.

WDGS

w

WDGCS[3:0]
rw

rw

rw

rw

WDGHRC[1:0]
rw

rw

r

0

WDGDI WDGE
S
N
w

w

Bits 31:17 Reserved, must be kept at reset value.
Bit 16 FWDGR: Force watchdog reload
This bit forces the reload of the graphic watchdog.
0: nNo effect
1: Graphic watchdog reload forced
Bits 15:12 Reserved, must be kept at reset value.

RM0456 Rev 6

<!-- pagebreak -->

2562

Graphic timer (GFXTIM)

RM0456

Bits 11:8 WDGCS[3:0]: Watchdog clock source
This bitfield selects the watchdog clock source.
0000: Line clock
0001: Frame clock
0010: HSYNC rising edge
0011: HSYNC falling edge
0100: VSYNC rising edge
0101: VSYNC falling edge
0110: TE rising edge
0111: TE falling edge
1000: Event 1
1001: Event 2
1010: Event 3
1011: Event 4
Others: Reserved
Bits 7:6 Reserved, must be kept at reset value.
Bits 5:4 WDGHRC[1:0]: Watchdog hardware reload configuration
This bitfield configures the watchdog hardware reload.
00: Watchdog hardware reload disabled
01: Watchdog reloaded a rising edge of gfxtim_wrld
10: Watchdog reloaded a falling edge of gfxtim_wrld
11: Reserved
Bit 3 Reserved, must be kept at reset value.
Bit 2 WDGS: Watchdog status
This bit returns the status of the graphic watchdog.
0: Graphic watchdog disabled
1: Graphic watchdog enabled
Bit 1 WDGDIS: Watchdog disable
This bit disables the graphic watchdog.
0: No effect
1: Graphic watchdog disabled
Bit 0 WDGEN: Watchdog enable
This bit enables the graphic watchdog.
0: No effect
1: Graphic watchdog enabled

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Graphic timer (GFXTIM)

59.5.8

GFXTIM interrupt status register (GFXTIM_ISR)
Address offset: 0x030
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

WDGA
F

Res.

Res.

Res.

Res.

EV4F

EV3F

EV2F

EV1F

r

r

r

r

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

AFCC1
F

Res.

TEF

Res.

Res.

Res.

Res.

Res.

Res.

WDGP
F
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

RFC2R RFC1R
F
F
r

r

ALCC2 ALCC1
F
F
r

r

r

r

ALCOF AFCOF
r

r

Bits 31:26 Reserved, must be kept at reset value.
Bit 25 WDGPF: Watchdog pre-alarm flag
This bit indicates that a graphic watchdog pre-alarm occurred.
0: No graphic watchdog pre-alarm occurred.
1: A graphic watchdog pre-alarm occurred.
Bit 24 WDGAF: Watchdog alarm flag
This bit indicates that a graphic watchdog alarm occurred.
0: No graphic watchdog alarm occurred.
1: A graphic watchdog alarm occurred.
Bits 23:20 Reserved, must be kept at reset value.
Bit 19 EV4F: Event 4 flag
This bit indicates a complex event 4 occurred.
0: No complex event 4 occurred.
1: A complex event 4 occurred.
Bit 18 EV3F: Event 3 flag
This bit indicates a complex event 3 occurred.
0: No complex event 3 occurred.
1: A complex event 3 occurred.
Bit 17 EV2F: Event 2 flag
This bit indicates a complex event 2 occurred.
0: No complex event 2 occurred.
1: A complex event 2 occurred.
Bit 16 EV1F: Event 1 flag
This bit indicates a complex event 1 occurred.
0: No complex event 1 occurred.
1: Complex event 1 occurred.
Bits 15:14 Reserved, must be kept at reset value.
Bit 13 RFC2RF: Relative frame counter 2 reload flag
This bit indicates relative frame counter 2 has been reloaded.
0: No reload occurred on relative frame counter 2.
1: A reload on relative frame counter 2 occurred.

RM0456 Rev 6

<!-- pagebreak -->

2562

Graphic timer (GFXTIM)

RM0456

Bit 12 RFC1RF: Relative frame counter 1 reload flag
This bit indicates relative frame counter 1 has been reloaded.
0: No reload occurred on relative frame counter 1.
1: A reload on relative frame counter 1 occurred.
Bits 11:10 Reserved, must be kept at reset value.
Bit 9 ALCC2F: Absolute line counter compare 2 flag
This bit indicates match on compare 2 of the absolute line counter.
0: No match occurred on compare 2 of the absolute line counter.
1: A match on compare 2 of the absolute line counter occurred.
Bit 8 ALCC1F: Absolute line counter compare 1 flag
This bit indicates match on compare 1 of the absolute line counter.
0: No match occurred on compare 1 of the absolute line counter.
1: A match on compare 1 of the absolute line counter occurred.
Bits 7:5 Reserved, must be kept at reset value.
Bit 4 AFCC1F: Absolute frame counter compare 1 flag
This bit indicates match on compare 1 of the absolute frame counter.
0: No match occurred on compare 1 of the absolute frame counter.
1: A match on compare 1 of the absolute frame counter occurred.
Bit 3 Reserved, must be kept at reset value.
Bit 2 TEF: Tearing-effect flag
This bit indicates a tearing effect event occurred.
0: No tearing effect occurred.
1: A tearing effect occurred.
Bit 1 ALCOF: Absolute line counter overflow flag
This bit indicates an overflow occurred on the absolute line counter.
0: No overflow occurred on the absolute line counter.
1: A overflow on the absolute line counter occurred.
Bit 0 AFCOF: absolute frame counter overflow flag
This bit indicates an overflow occurred on the absolute frame counter.
0: No overflow occurred on the absolute frame counter.
1: An overflow on the absolute frame counter occurred.

59.5.9

GFXTIM interrupt clear register (GFXTIM_ICR)
Address offset: 0x034
Reset value: 0x0000 0000

31

30

29

28

27

26

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

Res.

Res.

CRFC2 CRFC1
RF
RF
w

w

Res.

Res.

25

24

CWDG CWDG
PF
AF
w

w

9

8

CALCC CALCC
2F
1F
w

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

4

Res.

CAFCC
1F

Res.

w

Bits 31:26 Reserved, must be kept at reset value.

<!-- pagebreak -->

RM0456 Rev 6

Res.

w

19

18

17

16

CEV4F CEV3F CEV2F CEV1F
w

w

w

w

3

2

1

0

Res.

CTEF
w

CALCO CAFCO
F
F
w

w

RM0456

Graphic timer (GFXTIM)

Bit 25 CWDGPF: Clear watchdog pre-alarm flag
This bit clears WDGPF in GXFXTIM_ISR.
0: No effect
1: WDGPF cleared
Bit 24 CWDGAF: Clear watchdog alarm flag
This bit clears WDGAF in GXFXTIM_ISR.
0: No effect
1: WDGAF cleared
Bits 23:20 Reserved, must be kept at reset value.
Bit 19 CEV4F: Clear event 4 flag
This bit clears EV4F in GXFXTIM_ISR.
0: No effect
1: EV4F cleared
Bit 18 CEV3F: Clear event 3 flag
This bit clears EV3F in GXFXTIM_ISR.
0: No effect
1: EV3F cleared
Bit 17 CEV2F: Clear event 2 flag
This bit clears EV2F in GXFXTIM_ISR.
0: No effect
1: EV2F cleared
Bit 16 CEV1F: Clear event 1 flag
This bit EV1F in GXFXTIM_ISR.
0: No effect
1: EV1F cleared
Bits 15:14 Reserved, must be kept at reset value.
Bit 13 CRFC2RF: Clear relative frame counter 2 reload flag
This bit clears RFC2RF in GXFXTIM_ISR.
0: No effect
1: RFC2RF cleared
Bit 12 CRFC1RF: Clear relative frame counter 1 reload flag
This bit clears RFC1RF in GXTIM_ISR.
0: No effect
1: RFC1RF cleared
Bits 11:10 Reserved, must be kept at reset value.
Bit 9 CALCC2F: Clear absolute line counter compare 2 flag
This bit clears ALCC2F in GXTIM_ISR.
0: No effect
1: ALCC2F cleared
Bit 8 CALCC1F: Clear absolute line counter compare 1 flag
This bit clears ALCC1F in GXTIM_ISR.
0: No effect
1: ALCC1F cleared
Bits 7:5 Reserved, must be kept at reset value.

RM0456 Rev 6

<!-- pagebreak -->

2562

Graphic timer (GFXTIM)

RM0456

Bit 4 CAFCC1F: Clear absolute frame counter compare 1 flag
This bit clears AFCC1F in GXTIM_ISR.
0: No effect
1: AFCC1F cleared
Bit 3 Reserved, must be kept at reset value.
Bit 2 CTEF: Clear tearing-effect flag
This bit clears TEF in GXTIM_ISR.
0: No effect
1: TEF cleared
Bit 1 CALCOF: Clear absolute line counter overflow flag
This bit clears ALCOF in GXTIM_ISR.
0: No effect
1: ALCOF cleared
Bit 0 CAFCOF: Clear absolute frame counter overflow flag
This bit clears AFCOF in GXTIM_ISR.
0: No effect
1: AFCOF cleared

59.5.10

GFXTIM interrupt enable register (GFXTIM_IER)
Address offset: 0x038
Reset value: 0x0000 0000

31

30

29

28

27

26

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

Res.

Res.

RFC2R RFC1R
IE
IE
rw

rw

Res.

Res.

25

24

WDGPI WDGAI
E
E
rw

rw

9

8

ALCC2I ALCC1I
E
E
rw

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

EV4IE

EV3IE

EV2IE

EV1IE

rw

rw

rw

rw

7

6

5

4

3

2

1

0

Res.

AFCC1
IE

TEIE

ALCOI
E

AFCOI
E

rw

rw

rw

Res.

Res.

rw

rw

Bits 31:26 Reserved, must be kept at reset value.
Bit 25 WDGPIE: Watchdog pre-alarm interrupt enable
This bit enables the watchdog pre-alarm interrupt generation.
0: Watchdog pre-alarm interrupt disabled
1: Watchdog pre-alarm interrupt enabled
Bit 24 WDGAIE: Watchdog alarm interrupt enable
This bit enables the watchdog alarm interrupt generation.
0: Watchdog alarm interrupt disabled
1: Watchdog alarm interrupt enabled
Bits 23:20 Reserved, must be kept at reset value.
Bit 19 EV4IE: Event 4 interrupt enable
This bit enables the complex event 4 interrupt generation.
0: Event 4 interrupt disabled
1: Event 4 interrupt enabled

<!-- pagebreak -->

RM0456 Rev 6

Res.

RM0456

Graphic timer (GFXTIM)

Bit 18 EV3IE: Event 3 interrupt enable
This bit enables the complex event 3 interrupt generation.
0: Event 3 interrupt disabled
1: Event 3 interrupt enabled
Bit 17 EV2IE: Event 2 interrupt enable
This bit enables the complex event 2 interrupt generation.
0: Event 2 interrupt disabled
1: Event 2 interrupt enabled
Bit 16 EV1IE: Event 1 interrupt enable
This bit enables the complex event 1 interrupt generation.
0: Event 1 interrupt disabled
1: Event 1 interrupt enabled
Bits 15:14 Reserved, must be kept at reset value.
Bit 13 RFC2RIE: Relative frame counter 2 reload interrupt enable
This bit enables the relative frame counter 2 reload interrupt generation.
0: Relative frame counter 2 reload interrupt disabled
1: Relative frame counter 2 reload interrupt enabled
Bit 12 RFC1RIE: Relative frame counter 1 reload interrupt enable
This bit enables the relative frame counter 1 reload interrupt generation.
0: Relative frame counter 1 reload interrupt disabled
1: Relative frame counter 1 reload interrupt enabled
Bits 11:10 Reserved, must be kept at reset value.
Bit 9 ALCC2IE: Absolute line counter compare 2 interrupt enable
This bit enables the absolute line counter compare 2 interrupt generation.
0: Absolute line counter compare 2 interrupt disabled
1: Absolute line counter compare 2 interrupt enabled
Bit 8 ALCC1IE: Absolute line counter compare 1 interrupt enable
This bit enables the absolute line counter compare 1 interrupt generation.
0: Absolute line counter compare 1 interrupt disabled
1: Absolute line counter compare 1 interrupt enabled
Bits 7:5 Reserved, must be kept at reset value.
Bit 4 AFCC1IE: Absolute frame counter compare 1 interrupt enable
This bit enables the absolute frame counter compare interrupt generation.
0: Absolute frame counter compare 1 interrupt disabled
1: Absolute frame counter compare 1 interrupt enabled
Bit 3 Reserved, must be kept at reset value.
Bit 2 TEIE: Tearing-effect interrupt enable
This bit enables the Tearing Effect interrupt generation.
0: Tearing-effect interrupt disabled
1: Tearing-effect interrupt enabled
Bit 1 ALCOIE: Absolute line counter overflow interrupt enable
This bit enables the absolute line counter overflow interrupt generation.
0: Absolute line counter overflow interrupt disabled
1: Absolute line counter overflow interrupt enabled

RM0456 Rev 6

<!-- pagebreak -->

2562

Graphic timer (GFXTIM)

RM0456

Bit 0 AFCOIE: Absolute frame counter overflow interrupt enable
This bit enables the absolute frame counter overflow interrupt generation.
0: Absolute frame counter overflow interrupt disabled
1: Absolute frame counter overflow interrupt enabled

59.5.11

GFXTIM timers status register (GFXTIM_TSR)
Address offset: 0x03C
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

RFC2S

Res.

Res.

Res.

RFC1S

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

ALCS

Res.

Res.

Res.

AFCS

r

r

Bits 31:21 Reserved, must be kept at reset value.
Bit 20 RFC2S: Relative frame counter 2 status
This bit returns the status of the relative frame counter 2.
0: Relative frame counter 2 disabled
1: Relative frame counter 2 enabled
Bits 19:17 Reserved, must be kept at reset value.
Bit 16 RFC1S: Relative frame counter 1 status
This bit returns the status of the relative frame counter 1.
0: Relative frame counter 1 disabled
1: Relative frame counter 1 enabled
Bits 15:5 Reserved, must be kept at reset value.
Bit 4 ALCS: Absolute line counter status
This bit returns the status of the absolute line counter.
0: Absolute line counter disabled
1: Absolute line counter enabled
Bits 3:1 Reserved, must be kept at reset value.
Bit 0 AFCS: Absolute frame counter status
This bit returns the status of the absolute frame counter.
0: Absolute frame counter disabled
1: Absolute frame counter enabled

<!-- pagebreak -->

RM0456 Rev 6

r

r

RM0456

Graphic timer (GFXTIM)

59.5.12

GFXTIM line clock counter reload register (GFXTIM_LCCRR)
Address offset: 0x040
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

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

21

20

19

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

6

5

4

3

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

18

17

16

rw

rw

rw

2

1

0

rw

rw

rw

RELOAD[21:16]

RELOAD[15:0]
rw

rw

Bits 31:22 Reserved, must be kept at reset value.
Bits 21:0 RELOAD[21:0]: Reload value
Reload value of the line clock counter.

59.5.13

GFXTIM frame clock counter reload register (GFXTIM_FCCRR)
Address offset: 0x044
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

20

19

18

17

16

15

14

13

12

Res.

Res.

Res.

Res.

RELOAD[11:0]
rw

rw

rw

rw

rw

rw

rw

Bits 31:12 Reserved, must be kept at reset value.
Bits 11:0 RELOAD[11:0]: Reload value
Reload value of the frame clock counter.

59.5.14

GFXTIM absolute time register (GFXTIM_ATR)
Address offset: 0x050
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

FRAME[19:4]
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

FRAME[3:0]
r

r

r

LINE[11:0]
r

r

Bits 31:12 FRAME[19:0]: Fame number
Current value of the absolute frame counter.

RM0456 Rev 6

<!-- pagebreak -->

2562

Graphic timer (GFXTIM)

RM0456

Bits 11:0 LINE[11:0]: Line number
Current value of the absolute line counter.

59.5.15

GFXTIM absolute frame counter register (GFXTIM_AFCR)
Address offset: 0x054
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

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

19

18

17

16

rw

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

rw

rw

rw

rw

rw

rw

rw

FRAME[19:16]

FRAME[15:0]
rw

rw

Bits 31:20 Reserved, must be kept at reset value.
Bits 19:0 FRAME[19:0]: Frame number
Current value of the absolute frame counter.
Note: This bitfield can only be written when the absolute frame counter is disabled.

59.5.16

GFXTIM absolute line counter register (GFXTIM_ALCR)
Address offset: 0x058
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

12

Res.

Res.

Res.

Res.

LINE[11:0]
rw

rw

rw

rw

rw

rw

rw

Bits 31:12 Reserved, must be kept at reset value.
Bits 11:0 LINE[11:0]: Line number
Current value of the absolute line counter.
Note: This bitfield can only be written when the absolute line counter is disabled.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Graphic timer (GFXTIM)

59.5.17

GFXTIM absolute frame counter compare 1 register
(GFXTIM_AFCC1R)
Address offset: 0x060
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

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

19

18

17

16

rw

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

FRAME[19:16]

FRAME[15:0]
rw

rw

rw

rw

rw

rw

rw

rw

rw

Bits 31:20 Reserved, must be kept at reset value.
Bits 19:0 FRAME[19:0]: Frame number
Compare 1 value for the absolute frame counter.

59.5.18

GFXTIM absolute line counter compare 1 register
(GFXTIM_ALCC1R)
Address offset: 0x070
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

12

Res.

Res.

Res.

Res.

LINE[11:0]
rw

rw

rw

rw

rw

rw

rw

Bits 31:12 Reserved, must be kept at reset value.
Bits 11:0 LINE[11:0]: Line number
Compare value 1 for the absolute line counter.

59.5.19

GFXTIM absolute line counter compare 2 register
(GFXTIM_ALCC2R)
Address offset: 0x074
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

12

Res.

Res.

Res.

Res.

LINE[11:0]
rw

rw

rw

rw

rw

RM0456 Rev 6

rw

rw

<!-- pagebreak -->

2562

Graphic timer (GFXTIM)

RM0456

Bits 31:12 Reserved, must be kept at reset value.
Bits 11:0 LINE[11:0]: Line number
Compare value 2 for the absolute line counter.

59.5.20

GFXTIM relative frame counter 1 register (GFXTIM_RFC1R)
Address offset: 0x080
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

15

14

13

12

Res.

Res.

Res.

Res.

FRAME[11:0]
r

r

r

r

r

r

r

Bits 31:12 Reserved, must be kept at reset value.
Bits 11:0 FRAME[11:0]: Frame number
Current value of the relative frame counter 1.

59.5.21

GFXTIM relative frame counter 1 reload register
(GFXTIM_RFC1RR)
Address offset: 0x084
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

12

Res.

Res.

Res.

Res.

FRAME[11:0]
rw

rw

rw

rw

rw

rw

Bits 31:12 Reserved, must be kept at reset value.
Bits 11:0 FRAME[11:0]: Frame reload value
Reload value for the relative frame counter 1.

<!-- pagebreak -->

RM0456 Rev 6

rw

RM0456

Graphic timer (GFXTIM)

59.5.22

GFXTIM relative frame counter 2 register (GFXTIM_RFC2R)
Address offset: 0x088
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

FRAME[11:0]
r

r

Bits 31:12 Reserved, must be kept at reset value.
Bits 11:0 FRAME[11:0]: Frame number
Current value of the relative frame counter 2.

59.5.23

GFXTIM relative frame counter 2 reload register
(GFXTIM_RFC2RR)
Address offset: 0x08C
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

FRAME[11:0]
rw

rw

Bits 31:12 Reserved, must be kept at reset value.
Bits 11:0 FRAME[11:0]: Frame reload value
Reload value for the relative frame counter 2.

59.5.24

GFXTIM watchdog counter register (GFXTIM_WDGCR)
Address offset: 0x0A0
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

VALUE[15:0]
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

RM0456 Rev 6

<!-- pagebreak -->

2562

Graphic timer (GFXTIM)

RM0456

Bits 15:0 VALUE[15:0]: Value
Current value of the watchdog counter.

59.5.25

GFXTIM watchdog reload register (GFXTIM_WDGRR)
Address offset: 0x0A4
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

rw

rw

rw

RELOAD[15:0]
rw

rw

Bits 31:16 Reserved, must be kept at reset value.
Bits 15:0 RELOAD[15:0]: Reload value
Reload value of the watchdog counter.

59.5.26

GFXTIM watchdog pre-alarm register (GFXTIM_WDGPAR)
Address offset: 0x0A8
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

rw

rw

rw

rw

rw

rw

rw

PREALARM[15:0]
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
Bits 15:0 PREALARM[15:0]: Pre-alarm value
Pre-alarm value of the watchdog counter.

59.5.27

GFXTIM register map

Reset value

<!-- pagebreak -->

