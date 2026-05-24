2526

Low-power timer (LPTIM)

RM0456

Bit 2 EXTTRIG: External trigger edge event
EXTTRIG is set by hardware to inform application that a valid edge on the selected external trigger
input has occurred. If the trigger is ignored because the timer has already started, then this flag is
not set. EXTTRIG flag can be cleared by writing 1 to the EXTTRIGCF bit in the LPTIM_ICR register.
Bit 1 ARRM: Autoreload match
ARRM is set by hardware to inform application that LPTIM_CNT register’s value reached the
LPTIM_ARR register’s value. ARRM flag can be cleared by writing 1 to the ARRMCF bit in the
LPTIM_ICR register.
Bit 0 CC1IF: Compare 1 interrupt flag
If channel CC1 is configured as output:
The CC1IF flag is set by hardware to inform application that LPTIM_CNT register value matches the
compare register's value. CC1IF flag can be cleared by writing 1 to the CC1CF bit in the LPTIM_ICR
register.
0: No match
1: The content of the counter LPTIM_CNT register value has matched the LPTIM_CCR1 register's
value

58.7.3

LPTIMx interrupt and status register [alternate] (LPTIMx_ISR)
(x = 1 to 3)
This description of the register can only be used for input capture mode. See previous
section for output compare mode.
Address offset: 0x000
Reset value: 0x0000 0000

31
Res.

30
Res.

29
Res.

28
Res.

27
Res.

26
Res.

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

DIER
OK

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

EXT
TRIG

ARRM

CC1IF

r

r

r

r
15
Res.

14

13

12

Res.

CC2
OF

CC1
OF

r

r

11
Res.

10
Res.

9

8

CC2IF

REP
OK

UE

DOWN

UP

ARR
OK

r

r

r

r

r

r

Bits 31:25 Reserved, must be kept at reset value.
Bit 24 DIEROK: Interrupt enable register update OK
DIEROK is set by hardware to inform application that the APB bus write operation to the
LPTIM_DIER register has been successfully completed. DIEROK flag can be cleared by writing 1 to
the DIEROKCF bit in the LPTIM_ICR register.
Bits 23:16 Reserved, must be kept at reset value.
Bit 15 Reserved, must be kept at reset value.
Bit 14 Reserved, must be kept at reset value.
Bit 13 CC2OF: Capture 2 over-capture flag
This flag is set by hardware only when the corresponding channel is configured in input capture
mode. It is cleared by software by writing 1 to the CC2OCF bit in the LPTIM_ICR register.
0: No over-capture has been detected.
1: The counter value has been captured in LPTIM_CCR2 register while CC2IF flag was already set.
Note: If LPTIM does not implement at least 2 channels this bit is reserved. Refer to Section 58.3.

<!-- pagebreak -->

