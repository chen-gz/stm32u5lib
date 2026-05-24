RM0456 Rev 6

RM0456

Advanced-control timers (TIM1/TIM8)
Figure 511. Counter timing diagram with prescaler division change from 1 to 4

tim_psc_ck
CEN
tim_cnt_ck

Counter register

F7

F8

F9

FA FB FC

00

01

Update event (UEV)

Prescaler control register

0

3

Write a new value in TIMx_PSC

Prescaler buffer

Prescaler counter

0

3

0

0

1

2

3

0

1

2

3
MSv50999V1

54.3.4

Counter modes
Upcounting mode
In upcounting mode, the counter counts from 0 to the autoreload value (content of the
TIMx_ARR register), then restarts from 0 and generates a counter overflow event.
If the repetition counter is used, the update event (UEV) is generated after upcounting is
repeated for the number of times programmed in the repetition counter register
(TIMx_RCR) + 1. Else the update event is generated at each counter overflow.
Setting the UG bit in the TIMx_EGR register (by software or by using the slave mode
controller) also generates an update event.
The UEV event can be disabled by software by setting the UDIS bit in the TIMx_CR1
register. This is to avoid updating the shadow registers while writing new values in the
preload registers. Then no update event occurs until the UDIS bit has been written to 0.
However, the counter restarts from 0, as well as the counter of the prescaler (but the
prescale rate does not change). In addition, if the URS bit (update request selection) in
TIMx_CR1 register is set, setting the UG bit generates an update event UEV but without
setting the UIF flag (thus no interrupt or DMA request is sent). This is to avoid generating
both update and capture interrupts when clearing the counter on the capture event.

RM0456 Rev 6

<!-- pagebreak -->

2236

Advanced-control timers (TIM1/TIM8)

RM0456

When an update event occurs, all the registers are updated and the update flag (UIF bit in
TIMx_SR register) is set (depending on the URS bit):
•

The repetition counter is reloaded with the content of TIMx_RCR register,

•

The autoreload shadow register is updated with the preload value (TIMx_ARR),

•

The buffer of the prescaler is reloaded with the preload value (content of the TIMx_PSC
register).

The following figures show some examples of the counter behavior for different clock
frequencies when TIMx_ARR = 0x36.
Figure 512. Counter timing diagram, internal clock divided by 1
tim_psc_ck

CEN

tim_cnt_ck
Counter register

31

32

33

34 35 36

00

01

02

03

04

05

06

07

Counter overflow

Update event (UEV)

Update interrupt flag
(UIF)

MSv50997V1

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Advanced-control timers (TIM1/TIM8)
Figure 513. Counter timing diagram, internal clock divided by 2
tim_psc_ck

CEN

tim_cnt_ck

Counter register

0034

0035

0036

0000

0001

0002

0003

Counter overflow

Update event (UEV)

Update interrupt flag
(UIF)

MSv62300V1

Figure 514. Counter timing diagram, internal clock divided by 4
tim_psc_ck

CEN

tim_cnt_ck

Counter register

0035

0036

0000

0001

Counter overflow

Update event (UEV)

Update interrupt flag
(UIF)

MSv62301V1

RM0456 Rev 6

<!-- pagebreak -->

2236

Advanced-control timers (TIM1/TIM8)

RM0456

Figure 515. Counter timing diagram, internal clock divided by N
tim_psc_ck

tim_cnt_ck

Counter register

1F

00

20

Counter overflow

Update event (UEV)

Update interrupt flag
(UIF)

MSv62302V1

Figure 516. Counter timing diagram, update event when ARPE = 0
(TIMx_ARR not preloaded)

tim_psc_ck

CEN

tim_cnt_ck
Counter register

31

32

33

34

35

36

00

01

02

03

04

05

06

07

Counter overflow

Update event (UEV)

Update interrupt flag (UIF)

Auto-reload preload register

FF

36

Write a new value in TIMx_ARR

<!-- pagebreak -->

RM0456 Rev 6

MSv62303V1

RM0456

Advanced-control timers (TIM1/TIM8)
Figure 517. Counter timing diagram, update event when ARPE = 1
(TIMx_ARR preloaded)
tim_psc_ck

CEN
tim_cnt_ck
Counter register

F0

F1

F2

F3

F4

F5

00

01

02

03

04

05

06

07

Counter overflow
Update event (UEV)

Update interrupt flag
(UIF)
Auto-reload preload
register

F5

36

Auto-reload shadow
register

F5

Write a new value in TIMx_ARR

36

MSv62304V1

Downcounting mode
In downcounting mode, the counter counts from the autoreload value (content of the
TIMx_ARR register) down to 0, then restarts from the autoreload value and generates a
counter underflow event.
If the repetition counter is used, the update event (UEV) is generated after downcounting is
repeated for the number of times programmed in the repetition counter register
(TIMx_RCR) + 1. Else the update event is generated at each counter underflow.
Setting the UG bit in the TIMx_EGR register (by software or by using the slave mode
controller) also generates an update event.
The UEV update event can be disabled by software by setting the UDIS bit in TIMx_CR1
register. This is to avoid updating the shadow registers while writing new values in the
preload registers. Then no update event occurs until UDIS bit has been written to 0.
However, the counter restarts from the current autoreload value, whereas the counter of the
prescaler restarts from 0 (but the prescale rate doesn’t change).
In addition, if the URS bit (update request selection) in TIMx_CR1 register is set, setting the
UG bit generates an update event UEV but without setting the UIF flag (thus no interrupt or
DMA request is sent). This is to avoid generating both update and capture interrupts when
clearing the counter on the capture event.

RM0456 Rev 6

<!-- pagebreak -->

2236

Advanced-control timers (TIM1/TIM8)

RM0456

When an update event occurs, all the registers are updated and the update flag (UIF bit in
TIMx_SR register) is set (depending on the URS bit):
•

The repetition counter is reloaded with the content of TIMx_RCR register.

•

The buffer of the prescaler is reloaded with the preload value (content of the TIMx_PSC
register).

•

The autoreload active register is updated with the preload value (content of the
TIMx_ARR register). Note that the autoreload is updated before the counter is
reloaded, so that the next period is the expected one.

The following figures show some examples of the counter behavior for different clock
frequencies when TIMx_ARR = 0x36.
Figure 518. Counter timing diagram, internal clock divided by 1
tim_psc_ck

CEN

tim_cnt_ck
Counter register

05

04

03

02

01 00

36

35

34 33 32

31

30

2F

Counter underflow
(cnt_udf)

Update event (UEV)

Update interrupt flag
(UIF)

<!-- pagebreak -->

MSv62305V1

RM0456 Rev 6

RM0456

Advanced-control timers (TIM1/TIM8)
Figure 519. Counter timing diagram, internal clock divided by 2

tim_psc_ck

CEN

tim_cnt_ck
Counter register

0001

0002

0000

0036

0035

0034

0033

Counter underflow

Update event (UEV)

Update interrupt flag
(UIF)

MSv62306V1

Figure 520. Counter timing diagram, internal clock divided by 4

tim_psc_ck

CEN

tim_cnt_ck
Counter register

0001

0000

0000

0001

Counter underflow

Update event (UEV)

Update interrupt flag
(UIF)

MSv62307V1

RM0456 Rev 6

<!-- pagebreak -->

2236

Advanced-control timers (TIM1/TIM8)

RM0456

Figure 521. Counter timing diagram, internal clock divided by N

tim_psc_ck

tim_cnt_ck

Counter register

20

1F

00

36

Counter underflow

Update event (UEV)

Update interrupt flag
(UIF)

MSv62308V1

Figure 522. Counter timing diagram, update event when repetition counter is not used
tim_pasc_ck

CEN

tim_cnt_ck
Counter register

05

04

03

02

01

00

36

35

34

33

32

31

30

2F

Counter underflow

Update event (UEV)

Update interrupt flag
(UIF)
Auto-reload preload
register

FF

36

Write a new value in TIMx_ARR

MSv62309V1

Center-aligned mode (up/down counting)
In center-aligned mode, the counter counts from 0 to the autoreload value (content of the
TIMx_ARR register) – 1, generates a counter overflow event, then counts from the

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Advanced-control timers (TIM1/TIM8)
autoreload value down to 1 and generates a counter underflow event. Then it restarts
counting from 0.
Center-aligned mode is active when the CMS bits in TIMx_CR1 register are not equal to 00.
The Output compare interrupt flag of channels configured in output is set when: the counter
counts down (Center aligned mode 1, CMS = 01), the counter counts up (Center aligned
mode 2, CMS = 10) the counter counts up and down (Center aligned mode 3, CMS = 11).
In this mode, the DIR direction bit in the TIMx_CR1 register cannot be written. It is updated
by hardware and gives the current direction of the counter.
The update event can be generated at each counter overflow and at each counter underflow
or by setting the UG bit in the TIMx_EGR register (by software or by using the slave mode
controller) also generates an update event. In this case, the counter restarts counting from
0, as well as the counter of the prescaler.
The UEV update event can be disabled by software by setting the UDIS bit in the TIMx_CR1
register. This is to avoid updating the shadow registers while writing new values in the
preload registers. Then no update event occurs until UDIS bit has been written to 0.
However, the counter continues counting up and down, based on the current autoreload
value.
In addition, if the URS bit (update request selection) in TIMx_CR1 register is set, setting the
UG bit generates an UEV update event but without setting the UIF flag (thus no interrupt or
DMA request is sent). This is to avoid generating both update and capture interrupts when
clearing the counter on the capture event.
When an update event occurs, all the registers are updated and the update flag (UIF bit in
TIMx_SR register) is set (depending on the URS bit):
•

The repetition counter is reloaded with the content of TIMx_RCR register

•

The buffer of the prescaler is reloaded with the preload value (content of the TIMx_PSC
register)

•

The autoreload active register is updated with the preload value (content of the
TIMx_ARR register). Note that if the update source is a counter overflow, the
autoreload is updated before the counter is reloaded, so that the next period is the
expected one (the counter is loaded with the new value).

The following figures show some examples of the counter behavior for different clock
frequencies.

RM0456 Rev 6

<!-- pagebreak -->

2236

Advanced-control timers (TIM1/TIM8)

RM0456

Figure 523. Counter timing diagram, internal clock divided by 1, TIMx_ARR = 0x6

tim_psc_ck

CEN

tim_cnt_ck
Counter register

04

03

02

01

00

01

02

03

04

05

06

05

04

03

Counter underflow

Counter overflow

Update event (UEV)
Update interrupt flag
(UIF)

MSv62310V1

1. Here, center-aligned mode 1 is used (for more details refer to Section 54.6: TIM1/TIM8 registers).

Figure 524. Counter timing diagram, internal clock divided by 2

tim_psc_ck

CEN

tim_cnt_ck
Counter register

0003

0002

0001

0000

0001

0002

0003

Counter underflow

Update event (UEV)

Update interrupt flag
(UIF)

<!-- pagebreak -->

MSv62311V1

RM0456 Rev 6

RM0456

Advanced-control timers (TIM1/TIM8)
Figure 525. Counter timing diagram, internal clock divided by 4, TIMx_ARR = 0x36

tim_psc_ck

CEN

tim_cnt_ck
Counter register

0034

0035

0036

0035

Counter overflow

Update event (UEV)

Update interrupt flag
(UIF)
Note: Here, center_aligned mode 2 or 3 is updated with an UIF on overflow

MSv62312V1

Figure 526. Counter timing diagram, internal clock divided by N
tim_psc_ck

tim_cnt_ck

Counter register

20

1F

01

00

Counter underflow

Update event (UEV)

Update interrupt flag
(UIF)

MSv62313V1

RM0456 Rev 6

<!-- pagebreak -->

2236

Advanced-control timers (TIM1/TIM8)

RM0456

Figure 527. Counter timing diagram, update event with ARPE = 1 (counter underflow)

tim_psc_ck

CEN

tim_cnt_ck
Counter register

06

05

04

03

02

01

00

01

02

03

04

05

06

07

Counter underflow

Update event (UEV)

Update interrupt flag
(UIF)
Auto-reload preload
register

FD

36

Write a new value in TIMx_ARR
Auto-reload active
register

FD

36
MSv62314V1

<!-- pagebreak -->

