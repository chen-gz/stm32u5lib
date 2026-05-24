RM0456 Rev 6

RM0456

Basic timers (TIM6/TIM7)
The counter is clocked by the prescaler output tim_cnt_ck, which is enabled only when the
counter enable bit (CEN) in the TIMx_CR1 register is set.
Note that the actual counter enable signal tim_cnt_en is set one clock cycle after CEN bit
set.

Prescaler description
The prescaler can divide the counter clock frequency by any factor between 1 and 65536. It
is based on a 16-bit counter controlled through a 16-bit register (in the TIMx_PSC register).
It can be changed on the fly as the TIMx_PSC control register is buffered. The new
prescaler ratio is taken into account at the next update event.
Figure 724 and Figure 725 give some examples of the counter behavior when the prescaler
ratio is changed on the fly.
Figure 724. Counter timing diagram with prescaler division change from 1 to 2
tim_psc_ck

CEN
tim_cnt_ck

Counter register

F7

F8

F9

FA FB FC

01

00

02

03

Update event (UEV)

Prescaler control register

0

1

Write a new value in TIMx_PSC
Prescaler buffer

0

Prescaler counter

0

1

0

1

0

1

0

1

0

1
MSv50998V1

RM0456 Rev 6

<!-- pagebreak -->

2474

Basic timers (TIM6/TIM7)

RM0456

Figure 725. Counter timing diagram with prescaler division change from 1 to 4

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

57.3.5

Counting mode
The counter counts from 0 to the autoreload value (contents of the TIMx_ARR register),
then restarts from 0 and generates a counter overflow event.
An update event can be generate at each counter overflow or by setting the UG bit in the
TIMx_EGR register (by software or by using the slave mode controller).
The UEV event can be disabled by software by setting the UDIS bit in the TIMx_CR1
register. This avoids updating the shadow registers while writing new values into the preload
registers. In this way, no update event occurs until the UDIS bit has been written to 0,
however, the counter and the prescaler counter both restart from 0 (but the prescale rate
does not change). In addition, if the URS (update request selection) bit in the TIMx_CR1
register is set, setting the UG bit generates an update event UEV, but the UIF flag is not set
(so no interrupt or DMA request is sent).
When an update event occurs, all the registers are updated and the update flag (UIF bit in
the TIMx_SR register) is set (depending on the URS bit):
•

The buffer of the prescaler is reloaded with the preload value (contents of the
TIMx_PSC register).

•

The autoreload shadow register is updated with the preload value (TIMx_ARR).

The following figures show some examples of the counter behavior for different clock
frequencies when TIMx_ARR = 0x36.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Basic timers (TIM6/TIM7)
Figure 726. Counter timing diagram, internal clock divided by 1
tim_psc_ck

CEN

tim_cnt_ck
Counter register

32

31

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

Figure 727. Counter timing diagram, internal clock divided by 2
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

RM0456 Rev 6

<!-- pagebreak -->

