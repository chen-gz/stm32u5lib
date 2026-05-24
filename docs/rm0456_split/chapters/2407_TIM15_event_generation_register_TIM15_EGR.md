2526

Low-power timer (LPTIM)

RM0456

Bit 6 DOWNIE: Direction change to down Interrupt Enable
0: DOWN interrupt disabled
1: DOWN interrupt enabled
Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Refer to Section 58.3.
Bit 5 UPIE: Direction change to UP Interrupt Enable
0: UP interrupt disabled
1: UP interrupt enabled
Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Refer to Section 58.3.
Bit 4 ARROKIE: Autoreload register update OK Interrupt Enable
0: ARROK interrupt disabled
1: ARROK interrupt enabled
Bit 3 CMP1OKIE: Compare register 1 update OK interrupt enable
0: CMPOK register 1 interrupt disabled
1: CMPOK register 1 interrupt enabled
Bit 2 EXTTRIGIE: External trigger valid edge Interrupt Enable
0: EXTTRIG interrupt disabled
1: EXTTRIG interrupt enabled
Bit 1 ARRMIE: Autoreload match Interrupt Enable
0: ARRM interrupt disabled
1: ARRM interrupt enabled
Bit 0 CC1IE: Capture/compare 1 interrupt enable
0: Capture/compare 1 interrupt disabled
1: Capture/compare 1 interrupt enabled

Caution:

The LPTIMx_DIER register must only be modified when the LPTIM is enabled (ENABLE bit
set to 1). After a write to the LPTIMx_DIER register, a new write operation to the same
register can only be performed when the previous write operation is completed. Any
successive write before the DIEROK flag is set, leads to unpredictable results.

58.7.9

LPTIMx interrupt enable register [alternate] (LPTIMx_DIER)
(x = 1 to 3)
This description of the register can only be used for input capture mode. See previous
section for output compare mode.
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

CC2DE

Res.

UEDE

Res.

Res.

Res.

Res.

Res.

Res.

CC1DE

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

CC2OI
E

CC1OI
E

Res.

Res.

CC2IE

REPOK
IE

UEIE

DOWNI
E

UPIE

ARRO
KIE

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

<!-- pagebreak -->

