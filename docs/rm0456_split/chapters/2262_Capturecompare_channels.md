2350

General-purpose timers (TIM2/TIM3/TIM4/TIM5)

RM0456

Figure 623. Control circuit in external clock mode 2

tim_ker_ck

CEN
tim_etr_in

tim_etrp

tim_etrf
tim_cnt_ck
tim_psc_ck

Counter register

34

35

36
MSv62321V1

55.4.6

Capture/compare channels
Each Capture/Compare channel is built around a capture/compare register (including a
shadow register), an input stage for capture (with digital filter, multiplexing and prescaler)
and an output stage (with comparator and output control).
The following figure gives an overview of one Capture/Compare channel.
The input stage samples the corresponding tim_tix input to generate a filtered signal
tim_tixf. Then, an edge detector with polarity selection generates a signal (tim_tixfpy) which
can be used as trigger input by the slave mode controller or as the capture command. It is
prescaled before the capture register (ICxPS).
Figure 624. Capture/compare channel (example: channel 1 input stage)
TIMx_TISEL
TI1SEL[3:0]
tim_ti1f_ed

tim_ti1_in0

TIM_CH1

To the slave mode controller

tim_ti1_in[15:1]
Filter
downcounter
fDTS

tim_ti1f_rising
tim_ti1f

Edge
detector

0
tim_ti1f_falling

01

1 tim_ti1_fp1
tim_ti2fp1

ICF[3:0]
TIMx_CCMR1

CC1P/CC1NP
TIMx_CCER
tim_ti2f_rising
0
(from channel 2)
tim_ti2f_falling
1
(from channel 2)

10

tim_ic1

Divider
/1, /2, /4, /8

tim_ic1f

tim_trc
11
(from slave mode
controller)
CC1S[1:0]

ICPS[1:0]

TIMx_CCMR1

CC1E
TIMx_CCER
MSv62322V2

<!-- pagebreak -->

RM0456 Rev 6

RM0456

General-purpose timers (TIM2/TIM3/TIM4/TIM5)
The output stage generates an intermediate waveform which is then used for reference:
tim_ocxref (active high). The polarity acts at the end of the chain.
Figure 625. Capture/compare channel 1 main circuit
APB Bus

MCU-peripheral interface
Input mode

Output mode

16/32-bit

CC1S[1]
Capture/compare preload register

CC1S[0]
IC1PS

CC1S[1]
CC1S[0]

Compare
transfer

Capture

CC1E

OC1PE

OC1PE
UEV
TIMx_CCMR1
(from time
base unit)

compare shadow register
CC1G
Comparator

TIMx_EGR

CNT>CCR1
Counter

CNT=CCR1

MSv63030V1

Figure 626. Output stage of capture/compare channel (channel 1, idem ch.2, 3 and 4)
TIMx_SMCR
OCCS(1)
tim_ocref_clr 0

To the master
mode controller

tim_etrf 1

tim_oc1refc

tim_ocref_clr_int

tim_oc1ref

‘0’

CNT > CCR1

Output
mode
CNT = CCR1
controller

tim_oc2ref

Output
selector

0

0

1

1

CC1E

CC1P

TIMx_CCER

OC1CE

TIMx_CCER

Output
enable
circuit

tim_oc1

CC1E TIMx_CCER

OC1M[3:0]

TIMx_CCMR1
MSv62374V2

1. Available on some instances only. If not available, tim_etrf is directly connected to tim_ocref_clr_int.

The capture/compare block is made of one preload register and one shadow register. Write
and read always access the preload register.
In capture mode, captures are actually done in the shadow register, which is copied into the
preload register.

RM0456 Rev 6

<!-- pagebreak -->

