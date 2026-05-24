RM0456 Rev 6

RM0456

Basic timers (TIM6/TIM7)

57.4.2

TIMx control register 2 (TIMx_CR2)(x = 6 to 7)
Address offset: 0x04
Reset value: 0x0000

15

14

13

12

11

10

9

8

7

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

6

5

4

MMS[2:0]
rw

rw

3

2

1

0

Res.

Res.

Res.

Res.

rw

Bits 15:7 Reserved, must be kept at reset value.
Bits 6:4 MMS[2:0]: Master mode selection
These bits are used to select the information to be sent in master mode to slave timers for
synchronization (TRGO). The combination is as follows:
000:Reset - the UG bit from the TIMx_EGR register is used as a trigger output (tim_trgo).
001:Enable - the Counter enable signal, tim_cnt_en, is used as a trigger output (tim_trgo). It
is useful to start several timers at the same time or to control a window in which a slave
timer is enabled. The Counter Enable signal is generated when the CEN control bit is
written.
010:Update - The update event is selected as a trigger output (tim_trgo). For instance a
master timer can then be used as a prescaler for a slave timer.
Note: The clock of the slave timer or he peripheral receiving the tim_trgo must be enabled
prior to receive events from the master timer, and must not be changed on-the-fly while
triggers are received from the master timer.
Bits 3:0 Reserved, must be kept at reset value.

57.4.3

TIMx DMA/Interrupt enable register (TIMx_DIER)(x = 6 to 7)
Address offset: 0x0C
Reset value: 0x0000

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

UDE

Res.

Res.

Res.

Res.

Res.

Res.

Res.

UIE

rw

rw

Bits 15:9 Reserved, must be kept at reset value.
Bit 8 UDE: Update DMA request enable
0: Update DMA request disabled.
1: Update DMA request enabled.
Bits 7:1 Reserved, must be kept at reset value.
Bit 0 UIE: Update interrupt enable
0: Update interrupt disabled.
1: Update interrupt enabled.

RM0456 Rev 6

<!-- pagebreak -->

2474

Basic timers (TIM6/TIM7)

57.4.4

RM0456

TIMx status register (TIMx_SR)(x = 6 to 7)
Address offset: 0x10
Reset value: 0x0000

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

UIF
rc_w0

Bits 15:1 Reserved, must be kept at reset value.
Bit 0 UIF: Update interrupt flag
This bit is set by hardware on an update event. It is cleared by software.
0: No update occurred.
1: Update interrupt pending. This bit is set by hardware when the registers are updated:
–On counter overflow if UDIS = 0 in the TIMx_CR1 register.
–When CNT is reinitialized by software using the UG bit in the TIMx_EGR register, if
URS = 0 and UDIS = 0 in the TIMx_CR1 register.

57.4.5

TIMx event generation register (TIMx_EGR)(x = 6 to 7)
Address offset: 0x14
Reset value: 0x0000

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

UG
w

Bits 15:1 Reserved, must be kept at reset value.
Bit 0 UG: Update generation
This bit can be set by software, it is automatically cleared by hardware.
0: No action.
1: Re-initializes the timer counter and generates an update of the registers. Note that the
prescaler counter is cleared too (but the prescaler ratio is not affected).

57.4.6

TIMx counter (TIMx_CNT)(x = 6 to 7)
Address offset: 0x24
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

UIF
CPY

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

rw

r

CNT[15:0]

<!-- pagebreak -->

