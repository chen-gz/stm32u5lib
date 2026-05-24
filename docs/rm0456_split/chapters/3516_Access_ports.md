RM0456 Rev 6

rw

RM0456

Debug support (DBG)

Bit 12 DBG_IWDG_STOP: IWDG stop in debug
0: normal operation. IWDG continues to operate while CPU is in debug mode.
1: stop in debug. IWDG is frozen while CPU is in debug mode.
Bit 11 DBG_WWDG_STOP: WWDG stop in debug
0: normal operation. WWDG continues to operate while CPU is in debug mode.
1: stop in debug. WWDG is frozen while CPU is in debug mode.
Bits 10:6 Reserved, must be kept at reset value.
Bits 5:0 DBG_TIMy_STOP: TIMy stop in debug (y = 7 to 2)
0: normal operation. TIMy continues to operate while CPU is in debug mode.
1: stop in debug. TIMy is frozen while CPU is in debug mode.

DBGMCU APB1H peripheral freeze register (DBGMCU_APB1HFZR)
Address offset: 0x0C
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

DBG_I
2C6_S
TOP

Res.

Res.

Res.

DBG_I
2C4_S
TOP

Res.

rw

DBG_I DBG_L
2C5_S PTIM2_
TOP
STOP
rw

rw

rw

Bits 31:8 Reserved, must be kept at reset value.
Bit 7 DBG_I2C6_STOP: I2C6 stop in debug
0: normal operation. I2C6 continues to operate while CPU is in debug mode.
1: stop in debug. I2C6 is frozen while CPU is in debug mode.
Note: This bit is reserved on STM32U535/545/575/585 devices.
Bit 6 DBG_I2C5_STOP: I2C5 stop in debug
0: normal operation. I2C5 continues to operate while CPU is in debug mode.
1: stop in debug. I2C5 is frozen while CPU is in debug mode.
Note: This bit is reserved on STM32U535/545/575/585 devices.
Bit 5 DBG_LPTIM2_STOP: LPTIM2 stop in debug
0: normal operation. LPTIM2 continues to operate while CPU is in debug mode.
1: stop in debug. LPTIM2 is frozen while CPU is in debug mode.
Bits 4:2 Reserved, must be kept at reset value.
Bit 1 DBG_I2C4_STOP: I2C4 stop in debug
0: normal operation. I2C4 continues to operate while CPU is in debug mode.
1: stop in debug. I2C4 is frozen while CPU is in debug mode.
Bit 0 Reserved, must be kept at reset value.

RM0456 Rev 6

<!-- pagebreak -->

3631

Debug support (DBG)

RM0456

DBGMCU APB2 peripheral freeze register (DBGMCU_APB2FZR)
Address offset: 0x10
Reset value: 0x0000 0000
31
Res.

15
Res.

30
Res.

29
Res.

14

13

Res.

DBG_T
IM8_ST
OP

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
Res.

20
Res.

19
Res.

18

17

16

DBG_T DBG_T DBG_T
IM17_S IM16_S IM15_S
TOP
TOP
TOP
rw

rw

rw

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

DBG_T
IM1_ST
OP

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

17

16

rw

rw

Bits 31:19 Reserved, must be kept at reset value.
Bits 18:16 DBG_TIMy_STOP: TIMy stop in debug (y = 17 to 15)
0: normal operation. TIMy continues to operate while CPU is in debug mode.
1: stop in debug. TIMy is frozen while CPU is in debug mode.
Bits 15:14 Reserved, must be kept at reset value.
Bit 13 DBG_TIM8_STOP: TIM8 stop in debug
0: normal operation. TIM8 continues to operate while CPU is in debug mode.
1: stop in debug. TIM8 is frozen while CPU is in debug mode.
Bit 12 Reserved, must be kept at reset value.
Bit 11 DBG_TIM1_STOP: TIM1 stop in debug
0: normal operation. TIM1 continues to operate while CPU is in debug mode.
1: stop in debug. TIM1 is frozen while CPU is in debug mode.
Bits 10:0 Reserved, must be kept at reset value.

DBGMCU APB3 peripheral freeze register (DBGMCU_APB3FZR)
Address offset: 0x14
Reset value: 0x0000 0000
31

30

Res.

DBG_R
TC_ST
OP

29
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
Res.

20
Res.

rw
15
Res.

14
Res.

13
Res.

12
Res.

19

rw

rw

rw

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

Res.

DBG_I
2C3_S
TOP

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

rw

Bit 31 Reserved, must be kept at reset value.
Bit 30 DBG_RTC_STOP: RTC stop in debug
0: normal operation. RTC continues to operate while CPU is in debug mode.
1: stop in debug. RTC is frozen while CPU is in debug mode.

<!-- pagebreak -->

18

DBG_L DBG_L DBG_L
PTIM4_ PTIM3_ PTIM1_
STOP STOP STOP

RM0456 Rev 6

RM0456

Debug support (DBG)

Bits 29:20 Reserved, must be kept at reset value.
Bit 19 DBG_LPTIM4_STOP: LPTIM4 stop in debug
0: normal operation. LPTIM4 continues to operate while CPU is in debug mode.
1: stop in debug. LPTIM4 is frozen while CPU is in debug mode.
Bit 18 DBG_LPTIM3_STOP: LPTIM3 stop in debug
0: normal operation. LPTIM3 continues to operate while CPU is in debug mode.
1: stop in debug. LPTIM3 is frozen while CPU is in debug mode.
Bit 17 DBG_LPTIM1_STOP: LPTIM1 stop in debug
0: normal operation. LPTIM1 continues to operate while CPU is in debug mode.
1: stop in debug. LPTIM1 is frozen while CPU is in debug mode.
Bits 16:11 Reserved, must be kept at reset value.
Bit 10 DBG_I2C3_STOP: I2C3 stop in debug
0: normal operation. I2C3 continues to operate while CPU is in debug mode.
1: stop in debug. I2C3 is frozen while CPU is in debug mode.
Bits 9:0 Reserved, must be kept at reset value.

DBGMCU AHB1 peripheral freeze register (DBGMCU_AHB1FZR)
Address offset: 0x20
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

DBG_G DBG_G DBG_G DBG_G DBG_G DBG_G
DBG_G DBG_G DBG_G DBG_G DBG_G DBG_G DBG_G DBG_G DBG_G DBG_G
PDMA1 PDMA1 PDMA1 PDMA1 PDMA1 PDMA1
PDMA9 PDMA8 PDMA7 PDMA6 PDMA5 PDMA4 PDMA3 PDMA2 PDMA1 PDMA0
5_STO 4_STO 3_STO 2_STO 1_STO 0_STO
_STOP _STOP _STOP _STOP _STOP _STOP _STOP _STOP _STOP _STOP
P
P
P
P
P
P
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

rw

rw

Bits 31:16 Reserved, must be kept at reset value.
Bits 15:0 DBG_GPDMAy_STOP: GPDMA channel y stop in debug (y = 15 to 0)
0: normal operation. GPDMA channel y continues to operate while CPU is in debug mode.
1: stop in debug. GPDMA channel y is frozen while CPU is in debug mode.

DBGMCU AHB3 peripheral freeze register (DBGMCU_AHB3FZR)
Address offset: 0x28
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

DBG_L DBG_L DBG_L DBG_L
PDMA3 PDMA2 PDMA1 PDMA0
_STOP _STOP _STOP _STOP
rw

RM0456 Rev 6

rw

rw

rw

<!-- pagebreak -->

3631

Debug support (DBG)

RM0456

Bits 31:4 Reserved, must be kept at reset value.
Bits 3:0 DBG_LPDMAy_STOP: LPDMA channel y stop in debug (y = 3 to 0)
0: normal operation. LPDMA channel 3 continues to operate while CPU is in debug mode.
1: stop in debug. LPDMA channel 3 is frozen while CPU is in debug mode.

DBGMCU status register (DBGMCU_SR)
Address offset: 0xFC
Reset value: 0xXXXX 0001
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

AP_ENABLED[15:0]
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

AP_PRESENT[15:0]
r

r

Bits 31:16 AP_ENABLED[15:0]: Bit n identifies whether access port AP n is open (can be accessed via
the debug port) or locked (debug access to the AP is blocked)
Bit n = 0: APn locked
Bit n = 1: APn enabled
Bits 15:0 AP_PRESENT[15:0]: Bit n identifies whether access port AP n is present in device
Bit n = 0: APn absent
Bit n = 1: APn present

DBGMCU debug host authentication register
(DBGMCU_DBG_AUTH_HOST)
Address offset: 0x100
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

AUTH_KEY[31:16]
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

w

w

w

w

w

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

3

2

1

0

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

w

w

w

w

AUTH_KEY[15:0]
w

w

Bits 31:0 AUTH_KEY[31:0]: Device authentication key
The device specific 64-bit authentication key (OEM key) must be written to this register (in
two successive 32-bit writes, least significant word first) to permit RDP regression. Writing a
wrong key locks access to the device and prevent code execution from the flash memory.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Debug support (DBG)

DBGMCU debug device authentication register
(DBGMCU_DBG_AUTH_DEVICE)
Address offset: 0x104
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

AUTH_ID[31:16]
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

AUTH_ID[15:0]
r

r

Bits 31:0 AUTH_ID[31:0]: Device specific ID
Device specific ID used for RDP regression.

DBGMCU CoreSight peripheral identity register 4 (DBGMCU_PIDR4)
Address offset: 0xFD0
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

SIZE[3:0]
r

r

r

JEP106CON[3:0]
r

r

r

r

r

Bits 31:8 Reserved, must be kept at reset value.
Bits 7:4 SIZE[3:0]: register file size
0x0: The register file occupies a single 4-Kbyte region.
Bits 3:0 JEP106CON[3:0]: JEP106 continuation code
0x0: STMicroelectronics JEDEC code

DBGMCU CoreSight peripheral identity register 0 (DBGMCU_PIDR0)
Address offset: 0xFE0
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

PARTNUM[7:0]
r

r

r

r

r

Bits 31:8 Reserved, must be kept at reset value.

RM0456 Rev 6

<!-- pagebreak -->

