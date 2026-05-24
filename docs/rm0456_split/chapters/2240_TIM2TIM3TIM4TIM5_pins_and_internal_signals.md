2350

General-purpose timers (TIM2/TIM3/TIM4/TIM5)
55.4.2

RM0456

TIM2/TIM3/TIM4/TIM5 pins and internal signals
Table 557 and Table 558 in this section summarize the TIM inputs and outputs.
Table 557. TIM input/output pins
Pin name

TIM_CH1
TIM_CH2
TIM_CH3
TIM_CH4

TIM_ETR

Signal type

Description

Input/Output

Timer multi-purpose channels.
Each channel be used for capture, compare,
or PWM.
TIM_CH1 and TIM_CH2 can also be used
as external clock (below 1/4 of the
tim_ker_ck clock) , external trigger and
quadrature encoder inputs.
TIM_CH1, TIM_CH2 and TIM_CH3 can be
used to interface with digital hall effect
sensors.

Input

External trigger input. This input can be
used as external trigger or as external clock
source. This input can receive a clock with a
frequency higher than the tim_ker_ck if the
tim_etr_in prescaler is used.

Table 558. TIM internal input/output signals
Internal signal name

Signal type

Description

Input

Internal timer inputs bus. The
tim_ti1_in[15:0] and tim_ti2_in[15:0] inputs
can be used for capture or as external clock
(below 1/4 of the tim_ker_ck clock) and for
quadrature encoder signals.

Input

External trigger internal input bus. These
inputs can be used as trigger, external clock
or for hardware cycle-by-cycle pulse width
control. These inputs can receive clock with
a frequency higher than the tim_ker_ck if the
tim_etr_in prescaler is used.

tim_itr[15:0]

Input

Internal trigger input bus. These inputs can
be used for the slave mode controller or as a
input clock (below 1/4 of the tim_ker_ck
clock).

tim_trgo

Output

Internal trigger output. This trigger can
trigger other on-chip peripherals.

tim_ocref_clr[7:0]

Input

Timer tim_ocref_clr input bus. These inputs
can be used to clear the tim_ocxref signals,
typically for hardware cycle-by-cycle pulse
width control.

tim_pclk

Input

Timer APB clock.

tim_ker_ck

Input

Timer kernel clock

tim_ti1_in[15:0]
tim_ti2_in[15:0]
tim_ti3_in[15:0]
tim_ti4_in[15:0]

tim_etr[15:0]

<!-- pagebreak -->

RM0456 Rev 6

RM0456

General-purpose timers (TIM2/TIM3/TIM4/TIM5)
Table 558. TIM internal input/output signals (continued)
Internal signal name

Signal type

Description

tim_it

Output

Global Timer interrupt, gathering
capture/compare, update and break trigger
requests.

tim_cc1_dma
tim_cc2_dma
tim_cc3_dma
tim_cc4_dma

Output

Timer capture/compare [4:1] dma requests.

tim_upd_dma

Output

Timer update dma request.

tim_trgi_dma

Output

Timer trigger dma request.

Tables below list the sources connected to the tim_ti[4:1] input multiplexers.
Table 559. Interconnect to the tim_ti1 input multiplexer
Sources
tim_ti1 inputs
TIM2

TIM3

TIM4

TIM5

tim_ti1_in0

TIM2_CH1

TIM3_CH1

TIM4_CH1

TIM5_CH1

tim_ti1_in1

comp1_out

comp1_out

comp1_out

LSI

tim_ti1_in2

comp2_out(1)

comp2_out(1)

comp2_out(1)

LSE

tim_ti1_in3

rtc_wut_trg

tim_ti1_in4

Reserved

comp1_out
comp2_out(1)

tim_ti1_in5
tim_ti1_in[15:6]

Reserved

1. This connection is not present in STM32U535/545 since COMP2 is not available.

Table 560. Interconnect to the tim_ti2 input multiplexer
Sources
tim_ti2 inputs
TIM2

TIM3

TIM4

TIM5

tim_ti2_in0

TIM2_CH2

TIM3_CH2

TIM4_CH2

TIM5_CH2

tim_ti2_in1

comp1_out

comp1_out

comp1_out

comp1_out

tim_ti2_in2

comp2_out(1)

comp2_out(1)

comp2_out(1)

comp2_out(1)

tim_ti2_in[15:3]

Reserved

1. This connection is not present in STM32U535/545 since COMP2 is not available.

RM0456 Rev 6

<!-- pagebreak -->

2350

General-purpose timers (TIM2/TIM3/TIM4/TIM5)

RM0456

Table 561. Interconnect to the tim_ti3 input multiplexer
Sources
tim_ti3 inputs
tim_ti3_in0

TIM2

TIM3

TIM4

TIM5

TIM2_CH3

TIM3_CH3

TIM4_CH3

TIM5_CH3

tim_ti3_in[15:1]

Reserved

Table 562. Interconnect to the tim_ti4 input multiplexer
Sources
tim_ti4 inputs
TIM2

TIM3

TIM4

TIM5

tim_ti4_in0

TIM2_CH4

TIM3_CH4

TIM4_CH4

TIM5_CH4

tim_ti4_in1

comp1_out

tim_ti4_in2

comp2_out(1)

Reserved

tim_ti4_in[15:3]

Reserved

1. This connection is not present in STM32U535/545 since COMP2 is not available.

The table below lists the internal sources connected to the tim_itr input multiplexer.
Table 563. TIMx internal trigger connection
TIMx

TIM2

TIM3

TIM4

TIM5

tim_itr0

tim1_trgo

tim1_trgo

tim1_trgo

tim1_trgo

tim_itr1

Reserved

tim2_trgo

tim2_trgo

tim2_trgo

tim_itr2

tim3_trgo

Reserved

tim3_trgo

tim3_trgo

tim_itr3

tim4_trgo

tim4_trgo

Reserved

tim4_trgo

tim_itr4

tim5_trgo

tim5_trgo

tim5_trgo

Reserved

tim_itr5

tim8_trgo

tim8_trgo

tim8_trgo

tim8_trgo

tim_itr6

tim15_trgo

tim15_trgo

tim15_trgo

tim15_trgo

tim_itr7

tim16_oc1

tim16_oc1

tim16_oc1

tim16_oc1

tim_itr8

tim17_oc1

tim17_oc1

tim17_oc1

tim17_oc1

tim_itr9
tim_itr10
tim_itr11

Reserved
Reserved
OTG_FS/OTG_HS
SOF

tim_itr[15:12]

<!-- pagebreak -->

Reserved

RM0456 Rev 6

RM0456

General-purpose timers (TIM2/TIM3/TIM4/TIM5)
Tables below list the internal sources connected to the tim_etr input multiplexer.
Table 564. Interconnect to the tim_etr input multiplexer
for STM32U535/545/575/585
Timer external
trigger input
signal

Timer external trigger signal assignment
TIM2

TIM3

TIM4

TIM5

tim_etr0

TIM2_ETR

TIM3_ETR

TIM4_ETR

TIM5_ETR

tim_etr1

comp1_out

comp1_out

comp1_out

comp1_out

tim_etr2

(1)

comp2_out

comp2_out

(1)

(1)

comp2_out(1)

tim_etr3

MSIK

MSIK

MSIK

MSIK

tim_etr4

HSI

HSI

HSI

HSI

tim_etr5

MSIS

MSIS

MSIS

MSIS

tim_etr6

comp2_out

Reserved

tim_etr7
tim_etr8

TIM3_ETR

TIM2_ETR

TIM3_ETR

TIM2_ETR

tim_etr9

TIM4_ETR

TIM4_ETR

TIM5_ETR

TIM3_ETR

tim_etr10

TIM5_ETR

Reserved

tim_etr11

LSE

adc1_awd1

tim_etr12

Reserved

tim_etr13

Reserved

adc1_awd2
adc1_awd3

tim_etr[15:14]

Reserved

1. This connection is not present in STM32U535/545 since COMP2 is not available.

Table 565. Interconnect to the tim_etr input multiplexer
for the STM32U59x/5Ax/5Fx/5Gx
Timer external
trigger input
signal

Timer external trigger signal assignment
TIM2

TIM3

TIM4

TIM5

tim_etr0

TIM2_ETR

TIM3_ETR

TIM4_ETR

TIM5_ETR

tim_etr1

comp1_out

comp1_out

comp1_out

comp1_out

tim_etr2

comp2_out

comp2_out

comp2_out

comp2_out

tim_etr3

MSIK

MSIK

MSIK

MSIK

tim_etr4

HSI

HSI

HSI

HSI

tim_etr5

MSIS

MSIS

MSIS

MSIS

tim_etr6

DCMI_VSYNC

DCMI_VSYNC

DCMI_VSYNC

DCMI_VSYNC

tim_etr7

LTDC_VSYNC

LTDC_VSYNC

LTDC_VSYNC

LTDC_VSYNC

tim_etr8

TIM3_ETR

TIM2_ETR

TIM3_ETR

TIM2_ETR

tim_etr9

TIM4_ETR

TIM4_ETR

TIM5_ETR

TIM3_ETR

RM0456 Rev 6

<!-- pagebreak -->

