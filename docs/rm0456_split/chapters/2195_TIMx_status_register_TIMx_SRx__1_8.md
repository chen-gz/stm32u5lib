RM0456 Rev 6

RM0456

Advanced-control timers (TIM1/TIM8)

Bit 8 UDE: Update DMA request enable
0: Update DMA request disabled
1: Update DMA request enabled
Bit 7 BIE: Break interrupt enable
0: Break interrupt disabled
1: Break interrupt enabled
Bit 6 TIE: Trigger interrupt enable
0: Trigger interrupt disabled
1: Trigger interrupt enabled
Bit 5 COMIE: COM interrupt enable
0: COM interrupt disabled
1: COM interrupt enabled
Bit 4 CC4IE: Capture/compare 4 interrupt enable
0: CC4 interrupt disabled
1: CC4 interrupt enabled
Bit 3 CC3IE: Capture/compare 3 interrupt enable
0: CC3 interrupt disabled
1: CC3 interrupt enabled
Bit 2 CC2IE: Capture/compare 2 interrupt enable
0: CC2 interrupt disabled
1: CC2 interrupt enabled
Bit 1 CC1IE: Capture/compare 1 interrupt enable
0: CC1 interrupt disabled
1: CC1 interrupt enabled
Bit 0 UIE: Update interrupt enable
0: Update interrupt disabled
1: Update interrupt enabled

54.6.5

TIMx status register (TIMx_SR)(x = 1, 8)
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

12

11

21

20

19

18

17

16

Res.

Res.

CC6IF

CC5IF

rc_w0

rc_w0

TERRF IERRF

DIRF

IDXF

rc_w0

rc_w0

rc_w0

rc_w0

8

7

6

5

4

3

2

1

0

14

13

Res.

Res.

SBIF

CC4OF CC3OF CC2OF CC1OF

B2IF

BIF

TIF

COMIF

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

rc_w0

rc_w0

rc_w0

rc_w0

rc_w0

rc_w0

9

22

15

rc_w0

10

23

rc_w0

RM0456 Rev 6

<!-- pagebreak -->

2236

Advanced-control timers (TIM1/TIM8)

RM0456

Bits 31:24 Reserved, must be kept at reset value.
Bit 23 TERRF: Transition error interrupt flag
This flag is set by hardware when a transition error is detected in encoder mode. It is
cleared by software by writing it to 0.
0: No encoder transition error has been detected.
1: An encoder transition error has been detected
Bit 22 IERRF: Index error interrupt flag
This flag is set by hardware when an index error is detected. It is cleared by software by
writing it to 0.
0: No index error has been detected.
1: An index error has been detected
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
Bits 19:18 Reserved, must be kept at reset value.
Bit 17 CC6IF: Compare 6 interrupt flag
Refer to CC1IF description
Note: Channel 6 can only be configured as output.
Bit 16 CC5IF: Compare 5 interrupt flag
Refer to CC1IF description
Note: Channel 5 can only be configured as output.
Bits 15:14 Reserved, must be kept at reset value.
Bit 13 SBIF: System break interrupt flag
This flag is set by hardware as soon as the system break input goes active. It can be
cleared by software if the system break input is not active.
This flag must be reset to re-start PWM operation.
0: No break event occurred.
1: An active level has been detected on the system break input. An interrupt is generated if
BIE = 1 in the TIMx_DIER register.
Bit 12 CC4OF: Capture/compare 4 overcapture flag
Refer to CC1OF description
Bit 11 CC3OF: Capture/compare 3 overcapture flag
Refer to CC1OF description
Bit 10 CC2OF: Capture/compare 2 overcapture flag
Refer to CC1OF description

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Advanced-control timers (TIM1/TIM8)

Bit 9 CC1OF: Capture/compare 1 overcapture flag
This flag is set by hardware only when the corresponding channel is configured in input
capture mode. It is cleared by software by writing it to 0.
0: No overcapture has been detected.
1: The counter value has been captured in TIMx_CCR1 register while CC1IF flag was
already set
Bit 8 B2IF: Break 2 interrupt flag
This flag is set by hardware as soon as the break 2 input goes active. It can be cleared by
software if the break 2 input is not active.
0: No break event occurred.
1: An active level has been detected on the break 2 input. An interrupt is generated if
BIE = 1 in the TIMx_DIER register.
Bit 7 BIF: Break interrupt flag
This flag is set by hardware as soon as the break input goes active. It can be cleared by
software if the break input is not active.
0: No break event occurred.
1: An active level has been detected on the break input. An interrupt is generated if BIE = 1
in the TIMx_DIER register.
Bit 6 TIF: Trigger interrupt flag
This flag is set by hardware on the TRG trigger event (active edge detected on tim_trgi input
when the slave mode controller is enabled in all modes but gated mode. It is set when the
counter starts or stops when gated mode is selected. It is cleared by software.
0: No trigger event occurred.
1: Trigger interrupt pending.
Bit 5 COMIF: COM interrupt flag
This flag is set by hardware on COM event (when capture/compare Control bits - CCxE,
CCxNE, OCxM - have been updated). It is cleared by software.
0: No COM event occurred.
1: COM interrupt pending.
Bit 4 CC4IF: Capture/compare 4 interrupt flag
Refer to CC1IF description
Bit 3 CC3IF: Capture/compare 3 interrupt flag
Refer to CC1IF description

RM0456 Rev 6

<!-- pagebreak -->

