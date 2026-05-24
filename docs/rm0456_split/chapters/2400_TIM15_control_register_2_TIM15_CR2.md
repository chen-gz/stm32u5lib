RM0456 Rev 6

RM0456

Low-power timer (LPTIM)

Bit 12 CC1OF: Capture 1 over-capture flag
This flag is set by hardware only when the corresponding channel is configured in input capture
mode. It is cleared by software by writing 1 to the CC1OCF bit in the LPTIM_ICR register.
0: No over-capture has been detected.
1: The counter value has been captured in LPTIM_CCR1 register while CC1IF flag was already set.
Note: If LPTIM does not implement at least 1 channel this bit is reserved. Refer to Section 58.3.
Bit 11 Reserved, must be kept at reset value.
Bit 10 Reserved, must be kept at reset value.
Bit 9 CC2IF: Capture 2 interrupt flag
If channel CC2 is configured as input:
CC2IF is set by hardware to inform application that the current value of the counter is captured in
LPTIM_CCR2 register. The corresponding interrupt or DMA request is generated if enabled. The
CC2OF flag is set if the CC2IF flag was already high.
0: No input capture occurred
1: The counter value has been captured in the LPTIM_CCR2 register. (An edge has been detected
on IC2 which matches the selected polarity). The CC2IF flag is automatically cleared by hardware
once the captured value is read (CPU or DMA). The CC2IF flag can be cleared by writing 1 to the
CC2CF bit in the LPTIM_ICR register.
Note: If LPTIM does not implement at least 2 channels this bit is reserved. Refer to Section 58.3.
Bit 8 REPOK: Repetition register update OK
REPOK is set by hardware to inform application that the APB bus write operation to the LPTIM_RCR
register has been successfully completed. REPOK flag can be cleared by writing 1 to the REPOKCF
bit in the LPTIM_ICR register.
Bit 7 UE: LPTIM update event occurred
UE is set by hardware to inform application that an update event was generated. The corresponding
interrupt or DMA request is generated if enabled. The UE flag can be cleared by writing 1 to the
UECF bit in the LPTIM_ICR register. The UE flag is automatically cleared by hardware once the
LPTIM_ARR register is written by any bus master like CPU or DMA.
Bit 6 DOWN: Counter direction change up to down
In Encoder mode, DOWN bit is set by hardware to inform application that the counter direction has
changed from up to down. DOWN flag can be cleared by writing 1 to the DOWNCF bit in the
LPTIM_ICR register.
Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Refer to Section 58.3.
Bit 5 UP: Counter direction change down to up
In Encoder mode, UP bit is set by hardware to inform application that the counter direction has
changed from down to up. UP flag can be cleared by writing 1 to the UPCF bit in the LPTIM_ICR
register.
Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Refer to Section 58.3.
Bit 4 ARROK: Autoreload register update OK
ARROK is set by hardware to inform application that the APB bus write operation to the LPTIM_ARR
register has been successfully completed. ARROK flag can be cleared by writing 1 to the ARROKCF
bit in the LPTIM_ICR register.
Bit 3 Reserved, must be kept at reset value.

RM0456 Rev 6

<!-- pagebreak -->

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
Bit 0 CC1IF: capture 1 interrupt flag
If channel CC1 is configured as input:
CC1IF is set by hardware to inform application that the current value of the counter is captured in
LPTIM_CCR1 register. The corresponding interrupt or DMA request is generated if enabled. The
CC1OF flag is set if the CC1IF flag was already high.
0:No input capture occurred
1:The counter value has been captured in the LPTIM_CCR1 register. (An edge has been detected
on IC1 which matches the selected polarity). The CC1IF flag is automatically cleared by hardware
once the captured value is read (CPU or DMA).CC1IF flag can be cleared by writing 1 to the
CC1CF bit in the LPTIM_ICR register.

58.7.4

LPTIM4 interrupt clear register (LPTIM4_ICR)
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

Res.

Res.

Res.

Res.

Res.

Res.

Res.

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

REPOK
CF

UECF

DOWN
CF

UPCF

ARRO
KCF

CMP1
OKCF

EXTTR
IGCF

w

w

w

w

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

ARRM
CC1CF
CF
w

w

Bits 31:25 Reserved, must be kept at reset value.
Bit 24 DIEROKCF: Interrupt enable register update OK clear flag
Writing 1 to this bit clears the DIEROK flag in the LPTIM_ISR register.
Bits 23:9 Reserved, must be kept at reset value.
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

<!-- pagebreak -->

