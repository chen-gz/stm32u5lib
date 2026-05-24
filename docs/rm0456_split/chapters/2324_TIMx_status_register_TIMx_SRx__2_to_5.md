2350

General-purpose timers (TIM2/TIM3/TIM4/TIM5)

RM0456

Bit 7 Reserved, must be kept at reset value.
Bit 6 TIE: Trigger interrupt enable
0: Trigger interrupt disabled.
1: Trigger interrupt enabled.
Bit 5 Reserved, must be kept at reset value.
Bit 4 CC4IE: Capture/Compare 4 interrupt enable
0: CC4 interrupt disabled.
1: CC4 interrupt enabled.
Bit 3 CC3IE: Capture/Compare 3 interrupt enable
0: CC3 interrupt disabled.
1: CC3 interrupt enabled.
Bit 2 CC2IE: Capture/Compare 2 interrupt enable
0: CC2 interrupt disabled.
1: CC2 interrupt enabled.
Bit 1 CC1IE: Capture/Compare 1 interrupt enable
0: CC1 interrupt disabled.
1: CC1 interrupt enabled.
Bit 0 UIE: Update interrupt enable
0: Update interrupt disabled.
1: Update interrupt enabled.

55.5.5

TIMx status register (TIMx_SR)(x = 2 to 5)
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

Res.

Res.

Res.

Res.

CC4OF CC3OF CC2OF CC1OF
rc_w0

rc_w0

rc_w0

21

20

19

18

17

16

TERRF IERRF

23

DIRF

IDXF

Res.

Res.

Res.

Res.

rc_w0

rc_w0

rc_w0

rc_w0

7

6

5

4

3

2

1

0

Res.

TIF

Res.

CC4IF

CC3IF

CC2IF

CC1IF

UIF

rc_w0

rc_w0

rc_w0

rc_w0

rc_w0

rc_w0

22

rc_w0

Bits 31:24 Reserved, must be kept at reset value.
Bit 23 TERRF: Transition error interrupt flag
This flag is set by hardware when a transition error is detected in encoder mode. It is cleared
by software by writing it to 0.
0: No encoder transition error has been detected.
1: An encoder transition error has been detected
Bit 22 IERRF: Index error interrupt flag
This flag is set by hardware when an index error is detected. It is cleared by software by
writing it to 0.
0: No index error has been detected.
1: An index error has been detected

<!-- pagebreak -->

RM0456 Rev 6

RM0456

General-purpose timers (TIM2/TIM3/TIM4/TIM5)

Bit 21 DIRF: Direction change interrupt flag
This flag is set by hardware when the direction changes in encoder mode (DIR bit value in
TIMx_CR is changing). It is cleared by software by writing it to 0.
0: No direction change
1: Direction change
Bit 20 IDXF: Index interrupt flag
This flag is set by hardware when an index event is detected. It is cleared by software by
writing it to 0.
0: No index event occurred.
1: An index event has occurred
Bits 19:13 Reserved, must be kept at reset value.
Bit 12 CC4OF: Capture/Compare 4 overcapture flag
refer to CC1OF description
Bit 11 CC3OF: Capture/Compare 3 overcapture flag
refer to CC1OF description
Bit 10 CC2OF: Capture/compare 2 overcapture flag
refer to CC1OF description
Bit 9 CC1OF: Capture/Compare 1 overcapture flag
This flag is set by hardware only when the corresponding channel is configured in input
capture mode. It is cleared by software by writing it to 0.
0: No overcapture has been detected.
1: The counter value has been captured in TIMx_CCR1 register while CC1IF flag was
already set
Bits 8:7 Reserved, must be kept at reset value.
Bit 6 TIF: Trigger interrupt flag
This flag is set by hardware on the TRG trigger event (active edge detected on tim_trgi input)
when the slave mode controller is enabled in all modes but gated mode. It is set when the
counter starts or stops when gated mode is selected. It is cleared by software.
0: No trigger event occurred.
1: Trigger interrupt pending.
Bit 5 Reserved, must be kept at reset value.
Bit 4 CC4IF: Capture/Compare 4 interrupt flag
Refer to CC1IF description
Bit 3 CC3IF: Capture/Compare 3 interrupt flag
Refer to CC1IF description

RM0456 Rev 6

<!-- pagebreak -->

