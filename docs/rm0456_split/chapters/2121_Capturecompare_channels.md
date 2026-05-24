RM0456 Rev 6

RM0456

Advanced-control timers (TIM1/TIM8)
Figure 535. Control circuit in external clock mode 2

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

54.3.8

Capture/compare channels
Each capture/compare channel is built around a capture/compare register (including a
shadow register), an input stage for capture (with digital filter, multiplexing, and prescaler,
except for channels 5 and 6) and an output stage (with comparator and output control).
Figure 536 to Figure 539 give an overview of one capture/compare channel.
The input stage samples the corresponding tim_tix input to generate a filtered signal
tim_tixf. Then, an edge detector with polarity selection generates a signal (tim_tixfpy) which
can be used as trigger input by the slave mode controller or as the capture command. It is
prescaled before the capture register (ICxPS).
Figure 536. Capture/compare channel (example: channel 1 input stage)
TIMx_TISEL
TI1SEL[3:0]

TIM_CH1

tim_ti1f_ed

tim_ti1_in0

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

RM0456 Rev 6

<!-- pagebreak -->

2236

Advanced-control timers (TIM1/TIM8)

RM0456

The output stage generates an intermediate waveform which is then used for reference:
tim_ocxref (active high). The polarity acts at the end of the chain.
Figure 537. Capture/compare channel 1 main circuit
APB Bus

MCU-peripheral interface
Input mode

Output mode

16/32-bit

CC1S[1]
CC1S[0]
IC1PS

Capture/compare preload register

CC1S[1]
CC1S[0]

Compare
transfer

Capture

CC1E

OC1PE

compare shadow register
CC1G
Comparator

TIMx_EGR

OC1PE
UEV
(from time TIMx_CCMR1
base unit)

CNT>CCR1
Counter

CNT=CCR1

MSv63030V1

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Advanced-control timers (TIM1/TIM8)

Figure 538. Output stage of capture/compare channel (channel 1, idem ch. 2, 3 and 4)
TIMx_SMCR
OCCS
tim_ocref_clr

To the master mode
controller

0

0

tim_etrf

1
‘0’

ocref_clr_int

tim_oc1refc

tim_oc1ref

tim_oc1_dt

CNT>CCR1

Output
mode
CNT=CCR1
controller

Ouput
selector

Dead-time
generator

x0
01

1

11

CC1P

Output
enable
circuit

tim_oc1

Output
enable
circuit

tim_oc1n

TIMx_CCER
tim_oc1n_dt

tim_ocxref(1)
tim_oc5ref

‘0’

11
0

10
0x

1

CC1NE CC1E
TIMx_CCER
OC1CE OC1M[3:0]

DTG[7:0]

CC1NE CC1E

CC1NP

MOE OSSI OSSR

TIMx_CCMR1

TIMx_BDTR

TIMx_CCER

TIMx_CCER

TIMx_BDTR
OIS1 OIS1N
TIMx_CR2
MSv62323V3

1. tim_ocxref, where x is the rank of the complementary channel

Figure 539. Output stage of capture/compare channel (channel 5, idem ch. 6)
TIMx_SMCR
OCCS
tim_ocref_clr

0

tim_etrf

1

To the master
mode controller

ocref_clr_int
‘0’

0

0

1

1

CC5E

CC5P

TIMx_CCER

TIMx_CCER

CNT > CCR5

Output
mode
CNT = CCR5
controller

OC5CE

tim_oc5ref

OC5M[3:0]

Output
enable
circuit

tim_oc5

(1)

CC5E TIMx_CCER
MOE

OSSI TIMx_BDTR

TIMx_CCMR3
OIS5

TIMx_CR2
MSv62324V3

1. Not available externally.

The capture/compare block is made of one preload register and one shadow register. Write
and read always access the preload register.

RM0456 Rev 6

<!-- pagebreak -->

