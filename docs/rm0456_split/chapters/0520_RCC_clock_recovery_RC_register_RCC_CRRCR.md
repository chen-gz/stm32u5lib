609

Reset and clock control (RCC)

11.8.5

RM0456

RCC clock recovery RC register (RCC_CRRCR)
Address offset: 0x014
Reset value: 0x0000 0XXX
X is factory-programmed.
Access: no wait state; word, half-word, and byte access

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
r

r

r

r

r

r

r

r

HSI48CAL[8:0]
r

Bits 31:9 Reserved, must be kept at reset value.
Bits 8:0 HSI48CAL[8:0]: HSI48 clock calibration
These bits are initialized at startup with the factory-programmed HSI48 calibration trim value.

11.8.6

RCC clock configuration register 1 (RCC_CFGR1)
Address offset: 0x01C
Reset value: 0x0000 0000
Access: 0 ≤ wait state ≤ 2; word, half-word, and byte access
1 or 2 wait states are inserted only if the access occurs during clock source switch.

31

30

Res.

15
Res.

29

28

27

MCOPRE[2:0]

26

25

24

MCOSEL[3:0]

rw

rw

rw

rw

rw

rw

rw

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

STOPK
STOP
ERWU
WUCK
CK
rw

rw

SWS[1:0]
r

r

SW[1:0]
rw

Bit 31 Reserved, must be kept at reset value.
Bits 30:28 MCOPRE[2:0]: microcontroller clock output prescaler
This bitfield is set and cleared by software. It is highly recommended to change this
prescaler before MCO output is enabled.
000: MCO divided by 1
001: MCO divided by 2
010: MCO divided by 4
011: MCO divided by 8
100: MCO divided by 16
Others: not allowed

<!-- pagebreak -->

RM0456 Rev 6

rw

RM0456

Reset and clock control (RCC)

Bits 27:24 MCOSEL[3:0]: microcontroller clock output
This bitfield is set and cleared by software.
0000: MCO output disabled, no clock on MCO
0001: SYSCLK system clock selected
0010: MSIS clock selected
0011: HSI16 clock selected
0100: HSE clock selected
0101: Main PLL clock pll1_r_ck selected
0110: LSI clock selected
0111: LSE clock selected
1000: Internal HSI48 clock selected
1001: MSIK clock selected
Others: reserved
Note: This clock output may have some truncated cycles at startup or during MCO clock
source switching.
Bits 23:6 Reserved, must be kept at reset value.
Bit 5 STOPKERWUCK: wake-up from Stop kernel clock automatic enable selection
This bit is set and cleared by software to enable automatically another oscillator when exiting
Stop mode. This oscillator can be used as independent kernel clock by peripherals.
0: MSIK oscillator automatically enabled when exiting Stop mode or when a CSS on HSE
event occurs.
1: HSI16 oscillator automatically enabled when exiting Stop mode or when a CSS on HSE
event occurs.
Bit 4 STOPWUCK: wake-up from Stop and CSS backup clock selection
This bit is set and cleared by software to select the system clock used when exiting Stop
mode. The selected clock is also used as emergency clock for the clock security system
on HSE.
Note: If this bit is used for CSS backup clock selection, the STOPKERWUCK bit value must
be programmed with the same value than STOPWUCK to avoid the other oscillator
power-on after CSS event.
Caution: STOPWUCK must not be modified when the CSS is enabled by CSSON
in RCC_CR, and the system clock is HSE (SWS = 10) or a switch on HSE
is requested (SW = 10).
0: MSIS oscillator selected as wake-up from stop clock and CSS backup clock
1: HSI16 oscillator selected as wake-up from stop clock and CSS backup clock
Bits 3:2 SWS[1:0]: system clock switch status
This bitfield is set and cleared by hardware to indicate which clock source is used as system
clock.
00: MSIS oscillator used as system clock
01: HSI16 oscillator used as system clock
10: HSE used as system clock
11: PLL pll1_r_ck used as system clock

RM0456 Rev 6

<!-- pagebreak -->

