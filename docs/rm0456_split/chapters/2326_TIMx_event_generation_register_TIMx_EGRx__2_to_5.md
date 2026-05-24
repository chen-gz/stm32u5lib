2350

General-purpose timers (TIM2/TIM3/TIM4/TIM5)

RM0456

Bit 2 CC2IF: Capture/Compare 2 interrupt flag
Refer to CC1IF description
Bit 1 CC1IF: Capture/compare 1 interrupt flag
This flag is set by hardware. It is cleared by software (input capture or output compare mode)
or by reading the TIMx_CCR1 register (input capture mode only).
0: No compare match / No input capture occurred
1: A compare match or an input capture occurred
If channel CC1 is configured as output: this flag is set when the content of the counter
TIMx_CNT matches the content of the TIMx_CCR1 register. When the content of
TIMx_CCR1 is greater than the content of TIMx_ARR, the CC1IF bit goes high on the
counter overflow (in up-counting and up/down-counting modes) or underflow (in downcounting mode). There are three possible options for flag setting in center-aligned mode,
refer to the CMS bits in the TIMx_CR1 register for the full description.
If channel CC1 is configured as input: this bit is set when counter value has been captured
in TIMx_CCR1 register (an edge has been detected on IC1, as per the edge sensitivity
defined with the CC1P and CC1NP bits setting, in TIMx_CCER).
Bit 0 UIF: Update interrupt flag
This bit is set by hardware on an update event. It is cleared by software.
0: No update occurred
1: Update interrupt pending. This bit is set by hardware when the registers are updated:
At overflow or underflow and if UDIS = 0 in the TIMx_CR1 register.
When CNT is reinitialized by software using the UG bit in TIMx_EGR register, if URS = 0 and
UDIS = 0 in the TIMx_CR1 register.
When CNT is reinitialized by a trigger event (refer to the synchro control register description),
if URS = 0 and UDIS = 0 in the TIMx_CR1 register.

55.5.6

TIMx event generation register (TIMx_EGR)(x = 2 to 5)
Address offset: 0x014
Reset value: 0x0000

15

14

13

12

11

10

9

8

7

6

5

4

3

2

1

0

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

TG

Res.

CC4G

CC3G

CC2G

CC1G

UG

w

w

w

w

w

w

Bits 15:7 Reserved, must be kept at reset value.
Bit 6 TG: Trigger generation
This bit is set by software in order to generate an event, it is automatically cleared by
hardware.
0: No action
1: The TIF flag is set in TIMx_SR register. Related interrupt or DMA transfer can occur if
enabled.
Bit 5 Reserved, must be kept at reset value.
Bit 4 CC4G: Capture/compare 4 generation
Refer to CC1G description
Bit 3 CC3G: Capture/compare 3 generation
Refer to CC1G description

<!-- pagebreak -->

