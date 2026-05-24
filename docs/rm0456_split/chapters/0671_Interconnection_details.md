Table 134. Peripherals interconnect matrix(1) (2) (continued)

RM0456

Peripherals interconnect matrix

16.3

Interconnection details

16.3.1

Master to slave interconnection for timers
From timer (TIM1/2/3/4/5/8/15/16/17) to timer (TIM1/2/3/4/5/8/15).

Purpose
Some of the TIMx timers are linked together internally for timer synchronization or chaining.
When one timer is configured in master mode, it can reset, start, stop, or clock the counter
of another timer configured in slave mode.
A description of the feature is provided in Section 55.4.23: Timer synchronization.
The synchronization modes are detailed in:
•

Section 54.3.30 for advanced-control timers TIM1/TIM8

•

Section 55.4.22 for general-purpose timers TIM2/TIM3/TIM4/TIM5

•

Section 56.4.23 for the general-purpose timer TIM15

Triggering signals
The output (from master) is on signal TIMx_TRGO (and TIMx_TRGO2 for TIM1/8) following
a configurable timer event. It can be also from signals tim16_oc1 and tim17_oc1 in case of
TIM16/17. The input (to slave) is on signals TIMx_ITR0/1/2/3.
The possible master/slave connections are given in:
•

Table 538 for advanced-control timers TIM1/8

•

Table 563 for general-purpose timers TIM2/3/4/5

•

Table 580 for the general-purpose timers TIM15

Active power mode
Timers are optionally active in Run and Sleep modes. The effects of low-power modes on
TIMx are given in:

16.3.2

•

Table 552: Effect of low-power modes on TIM1/TIM8

•

Table 572: Effect of low-power modes on TIM2/TIM3/TIM4/TIM5

•

Table 587: Effect of low-power modes on TIM15/TIM16/TIM17

Triggers to ADCs
From EXTI, timers (TIM1/2/3/4/5/6/8/15/16/17) and LP timers (LPTIM1/ 2/3/4) to
ADC1/ADC2
From EXTI, timers (TIM1/2/6/15) and LP timers (LPTIM1/3) to ADC4

Purpose
A conversion, or a sequence of conversions, can be triggered either by software or by
an external event (such as timer capture or input pins). For ADC12, if the EXTEN[1:0]
control bits (for a regular conversion) or JEXTEN[1:0] bits (for an injected conversion) are
different from 0b00, then external events can trigger a conversion with the selected polarity.

RM0456 Rev 6

<!-- pagebreak -->

684

Peripherals interconnect matrix

RM0456

More details in:
•

Section 33.4.19: Conversion on external trigger and trigger polarity (EXTSEL,
EXTEN[1:0], JEXTSEL, JEXTEN[1:0])

•

EXTEN[1:0] defined in ADC configuration register (ADC_CFGR1)

•

JEXTEN[1:0] defined in ADC injected sequence register (ADC_JSQR)

General-purpose timers (TIM2/3/4/5), basic timer (TIM6), advanced-control timers (TIM1/8)
and general-purpose timer (TIM15/16/17) can be used to generate the ADC triggering event
through the timer outputs tim_oc and tim_trgo.
Low-power timers (LPTIM1/2/3/4) can be used to generate the ADC triggering event
through the LPTIM channels (TIMx synchronization described in Section 54.3.31: ADC
triggers for TIM1/8) in addition to the EXTI on channels 11 and 15.
The ADC4 do not have injected channels. The general-purpose timers (TIM2/15), basic
timers (TIM6), and advanced-control timers (TIM1) can be used to generate the ADC
triggering event through the timer outputs tim_oc and tim_trgo. Low-power timers
(LPTIM1/3) can be used to generate the ADC triggering event through the LPTIM channels
in addition to EXTI on channel 11 and 15.

Triggering signals
For ADC1/ADC2, the input triggering signals and the description of the interconnection
between ADC1/ADC2, and timers, are given in:
•

adc_ext_trgy: Table 308: ADC1/ADC12 external triggers for regular channels

•

adc_jext_trgy: Table 309: ADC1/ADC12 external triggers for injected channels

•

Section 33.4.19: Conversion on external trigger and trigger polarity (EXTSEL,
EXTEN[1:0], JEXTSEL, JEXTEN[1:0])

•

Section 33.4.25: Timing diagrams example (single/continuous modes,
hardware/software triggers)

For ADC4, the input triggering signals list and the description of the interconnection
between ADC4 and timers, are given in:
•

Table 330: ADC interconnection

•

Section 34.4.16: Conversion on external trigger and trigger polarity (EXTSEL, EXTEN)

•

Section 34.4.21: Example timing diagrams (single/continuous modes
hardware/software triggers)

Active power mode
This interconnection is active in Run and Sleep modes for all ADCs, and under Stop 0,
Stop 1, and Stop 2 modes for ADC4 assuming that its trigger event line is active as well
(such as LPTIM). The timers are active in Run and Sleep mode only. The effects of
low-power modes are given in:

<!-- pagebreak -->

