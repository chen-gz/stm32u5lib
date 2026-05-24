2236

Advanced-control timers (TIM1/TIM8)

54.3.2

RM0456

TIM1/TIM8 pins and internal signals
The tables in this section summarize the TIM inputs and outputs
Table 532. TIM input/output pins
Pin name

Signal type

Description

TIM_CH1
TIM_CH2
TIM_CH3
TIM_CH4

Input/output

Timer multi-purpose channels.
Each channel can be used for capture,
compare or PWM.
TIM_CH1 and TIM_CH2 can also be used
as external clock (below 1/4 of the
tim_ker_ck clock), external trigger and
quadrature encoder inputs.
TIM_CH1, TIM_CH2 and TIM_CH3 can be
used to interface with digital hall effect
sensors.

TIM_CH1N
TIM_CH2N
TIM_CH3N
TIM_CH4N

Output

Timer complementary outputs, derived from
TIM_CHx outputs with the possibility to have
deadtime insertion.

TIM_ETR

Input

External trigger input. This input can be
used as external trigger or as external clock
source. This input can receive a clock with a
frequency higher than the tim_ker_ck if the
tim_etr_in prescaler is used.

TIM_BKIN
TIM_BKIN2

Input/output

Break and Break2 inputs. These inputs can
also be configured in bidirectional mode.

Table 533. TIM internal input/output signals
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
or for hardware cycle-by-cycle pulsewidth
control. These inputs can receive clock with
a frequency higher than the tim_ker_ck if the
tim_etr_in prescaler is used.

tim_itr[15:0]

Input

Internal trigger input bus. These inputs can
be used for the slave mode controller or as a
input clock (below 1/4 of the tim_ker_ck
clock).

tim_trgo/tim_trgo2

Output

Internal trigger outputs. These triggers are
used by other timers and /or other
peripherals.

tim_ti1_in[15:0]
tim_ti2_in[15:0]
tim_ti3_in[15:0]
tim_ti4_in[15:0]

tim_etr[15:0]

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Advanced-control timers (TIM1/TIM8)
Table 533. TIM internal input/output signals (continued)
Internal signal name

Signal type

Description

tim_ocref_clr[7:0]

Input

Timer tim_ocref_clr input bus. These inputs
can be used to clear the tim ocxref signals,
typically for hardware cycle-by-cycle
pulsewidth control.

tim_brk_cmp[8:1]

Input

Break input for internal signals

tim_brk2_cmp[8:1]

Input

Break2 input for internal signals

tim_sys_brk[n:0]

Input

System break input. This input gathers the
MCU’s system level errors.

tim_pclk

Input

Timer APB clock

tim_ker_ck

Input

Timer kernel clock

tim_cc_it

Output

Timer capture/compare interrupt

tim_upd_it

Output

Timer update event interrupt

tim_brk_terr_ierr_it

Output

Timer break, break2, transition error and
index error interrupt

tim_trgi_com_dir_idx_it

Output

Timer trigger, commutation, direction and
index interrupt

tim_cc1_dma
tim_cc2_dma
tim_cc3_dma
tim_cc4_dma

Output

Timer capture / compare 1..4 dma requests

tim_upd_dma

Output

Timer update dma request

tim_trgi_dma

Output

Timer trigger dma request

tim_com_dma

Output

Timer commutation dma request

Tables below list the sources connected to the tim_ti[4:1] input multiplexers.
Table 534. Interconnect to the tim_ti1 input multiplexer
Sources
tim_ti1 inputs
TIM1

TIM8

tim_ti1_in0

TIM1_CH1

TIM8_CH1

tim_ti1_in1

comp1_out

comp1_out

tim_ti1_in2

comp2_out(1)

comp2_out(1)

tim_ti1_in[15:3]

Reserved

1. This connection is not present in STM32U535/545 as COMP2 is not available.

RM0456 Rev 6

<!-- pagebreak -->

2236

Advanced-control timers (TIM1/TIM8)

RM0456

Table 535. Interconnect to the tim_ti2 input multiplexer
Sources
tim_ti2 inputs
tim_ti2_in0

TIM1

TIM8

TIM1_CH2

TIM8_CH2

tim_ti2_in[15:1]

Reserved

Table 536. Interconnect to the tim_ti3 input multiplexer
Sources
tim_ti3 inputs
tim_ti3_in0

TIM1

TIM8

TIM1_CH3

TIM8_CH3

tim_ti3_in[15:1]

Reserved

Table 537. Interconnect to the tim_ti4 input multiplexer
Sources
tim_ti4 inputs
tim_ti4_in0

TIM1

TIM8

TIM1_CH4

TIM8_CH4

tim_ti4_in[15:1]

Reserved

The table below lists the internal sources connected to the tim_itr input multiplexer.
Table 538. Internal trigger connection
Timer internal trigger input signal

TIM1

TIM8

tim_itr0

Reserved

tim1_trgo

tim_itr1

tim2_trgo

tim2_trgo

tim_itr2

tim3_trgo

tim3_trgo

tim_itr3

tim4_trgo

tim4_trgo

tim_itr4

tim5_trgo

tim5_trgo

tim_itr5

tim8_trgo

Reserved

tim_itr6

tim15_trgo

tim15_trgo

tim_itr7

tim16_oc1

tim16_oc1

tim_itr8

tim17_oc1

tim17_oc1

tim_itr[15:9]

<!-- pagebreak -->

Reserved

RM0456 Rev 6

RM0456

Advanced-control timers (TIM1/TIM8)
Tables below list the internal sources connected to the tim_etr input multiplexer.
Table 539. Interconnect to the tim_etr input multiplexer for STM32U535/545/575/585
Timer external trigger input
signal

Timer external trigger signal assignment
TIM1

TIM8

tim_etr0

TIM1_ETR

TIM8_ETR

tim_etr1

comp1_out

comp1_out

tim_etr2

comp2_out

(1)

comp2_out(1)

tim_etr3

MSIK

MSIK

tim_etr4

HSI

HSI

tim_etr5

MSIS

MSIS

tim_etr6

Reserved

tim_etr7
tim_etr8

adc1_awd1

adc1_awd1

tim_etr9

adc1_awd2

adc1_awd2

tim_etr10

adc1_awd3

adc1_awd3

tim_etr11

adc4_awd1

adc4_awd1

tim_etr12

adc4_awd2

adc4_awd2

tim_etr13

adc4_awd3

adc4_awd3

tim_etr[15:14]

Reserved

1. This connection is not present in STM32U535/545 as COMP2 is not available.

Table 540. Interconnect to the tim_etr input multiplex for STM2U59x/5Ax/5Fx/5Gx
Timer external trigger input
signal

Timer external trigger signal assignment
TIM1

TIM8

tim_etr0

TIM1_ETR

TIM8_ETR

tim_etr1

comp1_out

comp1_out

tim_etr2

comp2_out

comp2_out

tim_etr3

MSIK

MSIK

tim_etr4

HSI

HSI

tim_etr5

MSIS

MSIS

tim_etr6

adc2_awd2

adc2_awd2

tim_etr7

adc2_awd3

adc2_awd3

tim_etr8

adc1_awd1

adc1_awd1

tim_etr9

adc1_awd2

adc1_awd2

tim_etr10

adc1_awd3

adc1_awd3

tim_etr11

adc4_awd1

adc4_awd1

RM0456 Rev 6

<!-- pagebreak -->

2236

Advanced-control timers (TIM1/TIM8)

RM0456

Table 540. Interconnect to the tim_etr input multiplex for STM2U59x/5Ax/5Fx/5Gx
Timer external trigger input
signal

Timer external trigger signal assignment
TIM1

TIM8

tim_etr12

adc4_awd2

adc4_awd2

tim_etr13

adc4_awd3

adc4_awd3

tim_etr14

adc2_awd1

adc2_awd1

tim_etr15

Reserved

Tables below list the sources connected to the tim_brk and tim_brk2inputs.
Table 541. Timer break interconnect
tim_brk inputs

TIM1

TIM8

TIM1_BKIN pin

TIM8_BKIN pin

tim_brk_cmp1

comp1_out

comp1_out

tim_brk_cmp2

(1)

comp2_out(1)

TIM_BKIN

comp2_out

tim_brk_cmp3
tim_brk_cmp4

Reserved

tim_brk_cmp5
tim_brk_cmp6
tim_brk_cmp7

mdf1_break0

tim_brk_cmp8

mdf1_break2
Reserved

1. This connection is not present in STM32U535/545 as COMP2 is not available.

Table 542. Timer break2 interconnect
tim_brk2 inputs

TIM1

TIM8

TIM1_BKIN2 pin

TIM8_BKIN2 pin

tim_brk2_cmp1

comp1_out

comp1_out

tim_brk2_cmp2

comp2_out(1)

comp2_out(1)

TIM_BKIN2

tim_brk2_cmp3
tim_brk2_cmp4

Reserved

tim_brk2_cmp5
tim_brk2_cmp6
tim_brk2_cmp7

mdf1_break1

tim_brk2_cmp8

mdf1_break3
Reserved

1. This connection is not present in STM32U535/545 as COMP2 is not available.

<!-- pagebreak -->

