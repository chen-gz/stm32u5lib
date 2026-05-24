2474

Basic timers (TIM6/TIM7)

RM0456

Figure 728. Counter timing diagram, internal clock divided by 4
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

Figure 729. Counter timing diagram, internal clock divided by N
tim_psc_ck

tim_cnt_ck

Counter register

1F

20

00

Counter overflow

Update event (UEV)

Update interrupt flag
(UIF)

<!-- pagebreak -->

MSv62302V1

RM0456 Rev 6

RM0456

Basic timers (TIM6/TIM7)
Figure 730. Counter timing diagram, update event when ARPE = 0 (TIMx_ARR not
preloaded)

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

RM0456 Rev 6

MSv62303V1

<!-- pagebreak -->

