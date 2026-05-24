RM0456 Rev 6

RM0456

Low-power timer (LPTIM)

Bit 4 ARROKCF: Autoreload register update OK clear flag
Writing 1 to this bit clears the ARROK flag in the LPTIM_ISR register
Bit 3 CMP1OKCF: Compare register 1 update OK clear flag
Writing 1 to this bit clears the CMP1OK flag in the LPTIM_ISR register.
Bit 2 EXTTRIGCF: External trigger valid edge clear flag
Writing 1 to this bit clears the EXTTRIG flag in the LPTIM_ISR register
Bit 1 ARRMCF: Autoreload match clear flag
Writing 1 to this bit clears the ARRM flag in the LPTIM_ISR register
Bit 0 CC1CF: Capture/compare 1 clear flag
Writing 1 to this bit clears the CC1IF flag in the LPTIM_ISR register.

58.7.5

LPTIMx interrupt clear register [alternate] (LPTIMx_ICR)
(x = 1 to 3)
This description of the register can only be used for output compare mode. See next section
for input capture compare mode.
Address offset: 0x004
Reset value: 0x0000 0000

31

30

29

28

27

26

25

24
DIER
OKCF

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

Res.

Res.

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

CMP2
OKCF

Res.

Res.

Res.

1

0

Res.

Res.

Res.

Res.

8

7

6

5

4

3

2

CC2CF

REPOK
CF

UECF

DOWN
CF

UPCF

ARR
OKCF

CMP1
OKCF

EXT
TRIG
CF

w

w

w

w

w

w

w

w

w

w

ARRM
CC1CF
CF
w

w

Bits 31:25 Reserved, must be kept at reset value.
Bit 24 DIEROKCF: Interrupt enable register update OK clear flag
Writing 1 to this bit clears the DIEROK flag in the LPTIM_ISR register.
Bits 23:22 Reserved, must be kept at reset value.
Bit 21 Reserved, must be kept at reset value.
Bit 20 Reserved, must be kept at reset value.
Bit 19 CMP2OKCF: Compare register 2 update OK clear flag
Writing 1 to this bit clears the CMP2OK flag in the LPTIM_ISR register.
Note: If LPTIM does not implement at least 2 channels this bit is reserved. Refer to Section 58.3.
Bits 18:12 Reserved, must be kept at reset value.
Bit 11 Reserved, must be kept at reset value.
Bit 10 Reserved, must be kept at reset value.
Bit 9 CC2CF: Capture/compare 2 clear flag
Writing 1 to this bit clears the CC2IF flag in the LPTIM_ISR register.
Note: If LPTIM does not implement at least 2 channels this bit is reserved. Refer to Section 58.3.

RM0456 Rev 6

<!-- pagebreak -->

2526

Low-power timer (LPTIM)

RM0456

Bit 8 REPOKCF: Repetition register update OK clear flag
Writing 1 to this bit clears the REPOK flag in the LPTIM_ISR register.
Bit 7 UECF: Update event clear flag
Writing 1 to this bit clear the UE flag in the LPTIM_ISR register.
Bit 6 DOWNCF: Direction change to down clear flag
Writing 1 to this bit clear the DOWN flag in the LPTIM_ISR register.
Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Refer to Section 58.3.
Bit 5 UPCF: Direction change to UP clear flag
Writing 1 to this bit clear the UP flag in the LPTIM_ISR register.
Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Refer to Section 58.3.
Bit 4 ARROKCF: Autoreload register update OK clear flag
Writing 1 to this bit clears the ARROK flag in the LPTIM_ISR register
Bit 3 CMP1OKCF: Compare register 1 update OK clear flag
Writing 1 to this bit clears the CMP1OK flag in the LPTIM_ISR register.
Bit 2 EXTTRIGCF: External trigger valid edge clear flag
Writing 1 to this bit clears the EXTTRIG flag in the LPTIM_ISR register
Bit 1 ARRMCF: Autoreload match clear flag
Writing 1 to this bit clears the ARRM flag in the LPTIM_ISR register
Bit 0 CC1CF: Capture/compare 1 clear flag
Writing 1 to this bit clears the CC1IF flag in the LPTIM_ISR register.

58.7.6

LPTIMx interrupt clear register [alternate] (LPTIMx_ICR)
(x = 1 to 3)
This description of the register can only be used for input capture mode. See previous
section for output compare mode.
Address offset: 0x004
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

DIER
OKCF

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

UECF

DOWN
CF

UPCF

ARRO
KCF

Res.

EXTTR
IGCF

w

w

w

w

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

Res.

CC2
OCF

CC1
OCF

w

w

w

Res.

Res.

Res.

8

REPOK
CC2CF
CF
w

w

Bits 31:25 Reserved, must be kept at reset value.
Bit 24 DIEROKCF: Interrupt enable register update OK clear flag
Writing 1 to this bit clears the DIEROK flag in the LPTIM_ISR register.
Bits 23:16 Reserved, must be kept at reset value.
Bit 15 Reserved, must be kept at reset value.
Bit 14 Reserved, must be kept at reset value.

<!-- pagebreak -->

