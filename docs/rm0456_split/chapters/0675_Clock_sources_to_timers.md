•

Section 39.4.7: Short-circuit detectors (SCD)

•

Section 39.4.9: Out-of-limit detector (OLD)

•

Section 54.3.18: Using the break function

•

Section 54.3.19: Bidirectional break inputs

RM0456 Rev 6

RM0456

Peripherals interconnect matrix

Triggering signals
The mdf1_break[0:3] output signals are connected to break1 and break2 inputs signals of
TIM1/8. The tables below gives the assignment of break signals:
•

Table 372: MDF break connections

•

Table 541: Timer break interconnect

•

Table 542: Timer break2 interconnect

Active power mode
This interconnection is active under Run and Sleep modes. Refer to:

16.3.7

•

Section 39.4.14: Autonomous mode

•

Table 383: Effect of low-power modes on MDF

Clock sources to timers
From HSE, LSE, LSI, MSIK, HSI and MCO to timers (TIM1/2/3/4/5/8/15/16/17)
and LP timers (LPTIM1/2/3)

Purpose
A timer input or timer counter can receive different clock sources and can be used to
calibrate internal oscillator on a reference clock for example.
External clocks (HSE, LSE), internal clocks (LSI, MSI, HSI) and microcontroller output clock
(MCO) can be used as input to timers:
•

•

•

MSIK/HSI are assigned to advanced-control timers TIM1/8 as external trigger signals
inputs (tim_etr3/ tim_etr4). MSIK/HSI can be selected as counter clock provided by an
external clock source in mode2: external trigger input (tim_etr_in). Inputs assignment
and clock selection description are detailed in:
–

Section 54.3.7: Clock selection for TIM1/8

–

Table 539: Interconnect to the tim_etr input multiplexer for
STM32U535/545/575/585 for TIM1/8

MSIK, HSI and LSI are assigned to general purpose timers TIM2/3/4/5 as external
inputs signals. MSIK/HSI/LSI can be selected as counter clock provided by an external
clock source in mode1 (tim_ti1_in) and mode2 (external trigger input tim_etr_in). Inputs
assignment and clock selection description are detailed in:
–

Section 55.4.5: Clock selection for TIM2/3/4/5

–

External clock mode1: Table 559: Interconnect to the tim_ti1 input multiplexer for
TIM5, tim_ti1_in1 (LSI) and tim_ti1_in2 (LSE)

–

External clock mode2: Table 565, tim_etr3 (MSIK), tim_etr4 (HSI) and
tim_etr5 (MSIS) for TIM2/TIM3/TIM4/TIM5

LSE, LSI, MSI and HSI are assigned to general purpose timers TIM15/16/17 as
external inputs signals. LSE/LSI/MSI/HSI can be selected as counter clock provided by

RM0456 Rev 6

<!-- pagebreak -->

