RM0456 Rev 6

T[6:0]
rw

rw

rw

rw

RM0456

System window watchdog (WWDG)

Bit 7 WDGA: Activation bit
This bit is set by software and only cleared by hardware after a reset. When WDGA = 1, the
watchdog can generate a reset.
0: Watchdog disabled
1: Watchdog enabled
Bits 6:0 T[6:0]: 7-bit counter (MSB to LSB)
These bits contain the value of the watchdog counter, decremented every
(4096 x 2WDGTB[2:0]) PCLK cycles. A reset is produced when it is decremented from 0x40 to
0x3F (T6 becomes cleared).

62.6.2

WWDG configuration register (WWDG_CFR)
Address offset: 0x004
Reset value: 0x0000 007F

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

6

5

4

3

2

1

0

Res.

Res.

rw

rw

rw

WDGTB[2:0]
rw

rw

rw

10

9

8

7

Res.

EWI

Res.

Res.

rs

W[6:0]
rw

rw

rw

rw

Bits 31:14 Reserved, must be kept at reset value.
Bits 13:11 WDGTB[2:0]: Timer base
The timebase of the prescaler can be modified as follows:
000: CK counter clock (PCLK div 4096) div 1
001: CK counter clock (PCLK div 4096) div 2
010: CK counter clock (PCLK div 4096) div 4
011: CK counter clock (PCLK div 4096) div 8
100: CK counter clock (PCLK div 4096) div 16
101: CK counter clock (PCLK div 4096) div 32
110: CK counter clock (PCLK div 4096) div 64
111: CK counter clock (PCLK div 4096) div 128
Bit 10 Reserved, must be kept at reset value.
Bit 9 EWI: Early wake-up interrupt enable
Set by software and cleared by hardware after a reset. When set, an interrupt occurs
whenever the counter reaches the value 0x40.
Bits 8:7 Reserved, must be kept at reset value.
Bits 6:0 W[6:0]: 7-bit window value
These bits contain the window value to be compared with the down-counter.

RM0456 Rev 6

<!-- pagebreak -->

2584

System window watchdog (WWDG)

62.6.3

RM0456

WWDG status register (WWDG_SR)
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

Res.

Res.

Res.

EWIF
rc_w0

Bits 31:1 Reserved, must be kept at reset value.
Bit 0 EWIF: Early wake-up interrupt flag
This bit is set by hardware when the counter has reached the value 0x40. It must be cleared
by software by writing 0. Writing 1 has no effect. This bit is also set if the interrupt is not
enabled.

62.6.4

WWDG register map
The following table gives the WWDG register map and reset values.

Res.
Res.

Res.

Res.

1

0

2

1

1

1

1

1

1

1

1
EWIF

EWI

Res.

Res.
Res.

Res.

1

Res.

12
Res.
Res.

1

W[6:0]

Reset value

0

Refer to Section 2.3: Memory organization for the register boundary addresses.

<!-- pagebreak -->

