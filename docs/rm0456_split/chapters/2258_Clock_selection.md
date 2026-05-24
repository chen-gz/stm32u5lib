2350

General-purpose timers (TIM2/TIM3/TIM4/TIM5)

RM0456

Figure 618. Counter timing diagram, Update event with ARPE = 1 (counter overflow)

tim_psc_ck

CEN
tim_cnt_ck
Counter register

F7

F8

F9

FA

FB FC

36

35

34

33

32

31

30

2F

Counter overflow
Update event (UEV)

Update interrupt flag
(UIF)
Auto-reload preload
register

FD

36

Write a new value in TIMx_ARR
Auto-reload active
FD
register

36
MSv62315V1

55.4.5

Clock selection
The counter clock can be provided by the following clock sources:
•

Internal clock (tim_ker_ck).

•

External clock mode1: external input pin (tim_ti1 or tim_ti2).

•

External clock mode2: external trigger input (tim_etr_in).

•

Internal trigger inputs (tim_itr): using one timer as prescaler for another timer, for
example, timer 1 can be configured to act as a prescaler for timer 2. Refer to Using one
timer as prescaler for another timer for more details.

Internal clock source (tim_ker_ck)
If the slave mode controller is disabled (SMS = 000 in the TIMx_SMCR register), then the
CEN, DIR (in the TIMx_CR1 register), and UG bits (in the TIMx_EGR register) are actual
control bits and can be changed only by software (except UG which remains cleared
automatically). As soon as the CEN bit is written to 1, the prescaler is clocked by the internal
clock tim_ker_ck.
Figure 619 shows the behavior of the control circuit and the upcounter in normal mode,
without prescaler.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

General-purpose timers (TIM2/TIM3/TIM4/TIM5)
Figure 619. Control circuit in normal mode, internal clock divided by 1
tim_ker_ck

CEN

UG
counter initialization
(internal)

tim_cnt_ck, tim_psc_ck

Counter register

32

31

33

34

35 36

00 01

02

03 04 05

06

07

MSv62317V2

External clock source mode 1
This mode is selected when SMS = 111 in the TIMx_SMCR register. The counter can count
at each rising or falling edge on a selected input.
Figure 620. tim_ti2 external clock connection example
TIMx_SMCR
TIMx_TISEL

TS[4:0]

TI2SEL[3:0]
or tim_ti2f
tim_ti1f

tim_itrx

TIM_CH2

tim_ti1f_ed

tim_ti2_in0
tim_ti2
tim_ti2_in[15:1]

Filter

Edge
detector

tim_ti2f_rising
tim_ti2f_failing

0
1

00100
00101

tim_ti2fp2

00110

etrf

00111
(1)

CC2P

TIMx_CCMR1

TIMx_CCER

or

000xx

tim_ti1fp2

ICF[3:0]

or

tim_trgi

Encoder
mode
External clock
mode 1

tim_etrf

External clock
mode 2

tim_ker_ck

Internal clock
mode

(internal clock)

ECE

tim_psc_ck

SMS[2:0]

TIMx_SMCR
MSv62318V3

1. Codes ranging from 01000 to 11111: tim_itr[15:0].

For example, to configure the upcounter to count in response to a rising edge on the tim_ti2
input, use the following procedure:
1.

Select the proper tim_ti2_in[15:0] source (internal or external) with the TI2SEL[3:0] bits
in the TIMx_TISEL register.

2.

Configure channel 2 to detect rising edges on the tim_ti2 input by writing CC2S= 01 in
the TIMx_CCMR1 register.

3.

Configure the input filter duration by writing the IC2F[3:0] bits in the TIMx_CCMR1
register (if no filter is needed, keep IC2F = 0000).

RM0456 Rev 6

<!-- pagebreak -->

2350

General-purpose timers (TIM2/TIM3/TIM4/TIM5)
Note:

RM0456

The capture prescaler is not used for triggering, so it does not need to be configured.
4.

Select rising edge polarity by writing CC2P = 0 and CC2NP = 0 in the TIMx_CCER
register.

5.

Configure the timer in external clock mode 1 by writing SMS = 111 in the TIMx_SMCR
register.

6.

Select tim_ti2 as the input source by writing TS = 00110 in the TIMx_SMCR register.

7.

Enable the counter by writing CEN = 1 in the TIMx_CR1 register.

When a rising edge occurs on tim_ti2, the counter counts once and the TIF flag is set.
The delay between the rising edge on tim_ti2 and the actual clock of the counter is due to
the resynchronization circuit on tim_ti2 input.
Figure 621. Control circuit in external clock mode 1

tim_ti2
CEN

tim_cnt_ck, tim_psc_ck

Counter register

34

35

36

TIF

Write TIF=0

MSv62319V1

External clock source mode 2
This mode is selected by writing ECE = 1 in the TIMx_SMCR register.
The counter can count at each rising or falling edge on the external trigger input tim_etr_in.
Figure 622 gives an overview of the external trigger input block.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

General-purpose timers (TIM2/TIM3/TIM4/TIM5)
Figure 622. External trigger input block
or tim_ti1f
tim_ti2f

or
or

TIMx_AF1[17:14]
tim_trgi

TIM_ETR

tim_etr_in

(tim_etr0)

0
1

tim_etr[15:1]

ETP

tim_etrp
Divider
/1, /2, /4, /8 f
DTS

ETPS[1:0]

Filter
downcounter
ETF[3:0]

TIMx_SMCR TIMx_SMCR

Encoder
mode
External clock
mode 1

tim_etrf

External clock
mode 2

tim_ker_ck

Internal clock
mode

(internal clock)

tim_psc_ck

TIMx_SMCR
ECE

SMS[2:0]

TIMx_SMCR
MSv62385V2

For example, to configure the upcounter to count each two rising edges on tim_etr_in, use
the following procedure:
1.

Select the proper tim_etr_in source (internal or external) with the ETRSEL[3:0] bits in
the TIMx_AF1 register.

2.

As no filter is needed in this example, write ETF[3:0] = 0000 in the TIMx_SMCR
register.

3.

Set the prescaler by writing ETPS[1:0] = 01 in the TIMx_SMCR register.

4.

Select rising edge detection on the tim_etr_in by writing ETP = 0 in the TIMx_SMCR
register.

5.

Enable external clock mode 2 by writing ECE = 1 in the TIMx_SMCR register.

6.

Enable the counter by writing CEN = 1 in the TIMx_CR1 register.

The counter counts once each two tim_etr_in rising edges.
The delay between the rising edge on tim_etr_in and the actual clock of the counter is due
to the resynchronization circuit on the tim_etrp signal. As a consequence, the maximum
frequency that can be correctly captured by the counter is at most ¼ of TIMxCLK frequency.
When the ETRP signal is faster, the user must apply a division of the external signal by a
proper ETPS prescaler setting.

RM0456 Rev 6

<!-- pagebreak -->

