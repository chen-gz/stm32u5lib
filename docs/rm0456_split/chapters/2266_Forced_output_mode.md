2350

General-purpose timers (TIM2/TIM3/TIM4/TIM5)

RM0456

Figure 627. PWM input mode timing
tim_ti1

TIMx_CNT

0004

0000

0001

0002

TIMx_CCR1

0004

TIMx_CCR2

0002

IC1 capture
IC2 capture
reset counter

0003

0004

IC2 capture
pulse width
measurement

0000

IC1 capture
pulse width
measurement
MSv62325V1

1. The PWM input mode can be used only with the TIMx_CH1/TIMx_CH2 signals due to the fact that only
tim_ti1fp1 and tim_ti2fp2 are connected to the slave mode controller.

55.4.9

Forced output mode
In output mode (CCxS bits = 00 in the TIMx_CCMRx register), each output compare signal
(tim_ocxref and then tim_ocx) can be forced to active or inactive level directly by software,
independently of any comparison between the output compare register and the counter.
To force an output compare signal (tim_ocxref/tim_ocx) to its active level, the user just
needs to write 101 in the OCxM bits in the corresponding TIMx_CCMRx register. Thus
tim_ocxref is forced high (tim_ocxref is always active high) and tim_ocx get opposite value
to CCxP polarity bit.
For example: CCxP = 0 (tim_ocx active high) => tim_ocx is forced to high level.
tim_ocxref signal can be forced low by writing the OCxM bits to 100 in the TIMx_CCMRx
register.
Anyway, the comparison between the TIMx_CCRx shadow register and the counter is still
performed and allows the flag to be set. Interrupt and DMA requests can be sent
accordingly. This is described in the Output Compare mode section.

55.4.10

Output compare mode
This function is used to control an output waveform or indicating when a period of time has
elapsed.
When a match is found between the capture/compare register and the counter, the output
compare function:
•

<!-- pagebreak -->

Assigns the corresponding output pin to a programmable value defined by the output
compare mode (OCxM bits in the TIMx_CCMRx register) and the output polarity (CCxP
bit in the TIMx_CCER register). The output pin can keep its level (OCXM = 000), be set

RM0456 Rev 6

RM0456

General-purpose timers (TIM2/TIM3/TIM4/TIM5)
active (OCxM = 001), be set inactive (OCxM = 010) or can toggle (OCxM = 011) on
match.
•

Sets a flag in the interrupt status register (CCxIF bit in the TIMx_SR register).

•

Generates an interrupt if the corresponding interrupt mask is set (CCXIE bit in the
TIMx_DIER register).

•

Sends a DMA request if the corresponding enable bit is set (CCxDE bit in the
TIMx_DIER register, CCDS bit in the TIMx_CR2 register for the DMA request
selection).

The TIMx_CCRx registers can be programmed with or without preload registers using the
OCxPE bit in the TIMx_CCMRx register.
In output compare mode, the update event UEV has no effect on tim_ocxref and tim_ocx
output. The timing resolution is one count of the counter. Output compare mode can also be
used to output a single pulse (in One-pulse mode).

Procedure
1.

Select the counter clock (internal, external, prescaler).

2.

Write the desired data in the TIMx_ARR and TIMx_CCRx registers.

3.

Set the CCxIE and/or CCxDE bits if an interrupt and/or a DMA request is to be
generated.

4.

Select the output mode. For example:

5.

a)

Write OCxM = 0011 to toggle tim_ocx output pin when CNT matches CCRx.

b)

Write OCxPE = 0 to disable preload register.

c)

Write CCxP = 0 to select active high polarity.

d)

Write CCxE = 1 to enable the output.

Enable the counter by setting the CEN bit in the TIMx_CR1 register.

The TIMx_CCRx register can be updated at any time by software to control the output
waveform, provided that the preload register is not enabled (OCxPE = 0, else TIMx_CCRx
shadow register is updated only at the next update event UEV). An example is given in
Figure 628.

RM0456 Rev 6

<!-- pagebreak -->

2350

General-purpose timers (TIM2/TIM3/TIM4/TIM5)

RM0456

Figure 628. Output compare mode, toggle on tim_oc1
Write B201h in the CC1R register

CNT

0039

CCR1

003A

003B

B200

B201

B201

003A

tim_oc1ref = tim_oc1

Match detected on CCR1
Interrupt generated if enabled
MSv62326V1

55.4.11

PWM mode
Pulse width modulation mode is used to generate a signal with a frequency determined by
the value of the TIMx_ARR register and a duty cycle determined by the value of the
TIMx_CCRx register.
The PWM mode can be selected independently on each channel (one PWM per tim_ocx
output) by writing 110 (PWM mode 1) or 111 (PWM mode 2) in the OCxM bits in the
TIMx_CCMRx register. The corresponding preload register must be enabled by setting the
OCxPE bit in the TIMx_CCMRx register, and eventually the autoreload preload register (in
up-counting or center-aligned modes) by setting the ARPE bit in the TIMx_CR1 register.
As the preload registers are transferred to the shadow registers only when an update event
occurs, before starting the counter, all registers must be initialized by setting the UG bit in
the TIMx_EGR register.
tim_ocx polarity is software programmable using the CCxP bit in the TIMx_CCER register. It
can be programmed as active high or active low. tim_ocx output is enabled by the CCxE bit
in the TIMx_CCER register. Refer to the TIMx_CCERx register description for more details.
In PWM mode (1 or 2), TIMx_CNT and TIMx_CCRx are always compared to determine
whether TIMx_CCRx ≤ TIMx_CNT or TIMx_CNT ≤ TIMx_CCRx (depending on the direction
of the counter). The tim_ocref_clr can be cleared by an external event through the
tim_etr_in or the tim_oceref_clr signals. In this case the tim_ocref_clr signal is asserted
only:
•

After a compare match event.

•

When the output compare mode (OCxM bits in TIMx_CCMRx register) switches from
the “frozen” configuration (no comparison, OCxM = 000) to one of the PWM modes
(OCxM = 110 or 111). This forces the PWM by software while the timer is running.

The timer is able to generate PWM in edge-aligned mode or center-aligned mode
depending on the CMS bits in the TIMx_CR1 register.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

General-purpose timers (TIM2/TIM3/TIM4/TIM5)

PWM edge-aligned mode
•

Up-counting configuration

•

Up-counting is active when the DIR bit in the TIMx_CR1 register is low. Refer to Upcounting mode.
In the following example, we consider PWM mode 1. The reference PWM signal
tim_ocxref is high as long as TIMx_CNT <TIMx_CCRx else it becomes low. If the
compare value in TIMx_CCRx is greater than the autoreload value (in TIMx_ARR) then
tim_ocxref is held at 1. If the compare value is 0 then tim_ocxref is held at 0. Figure 629
shows some edge-aligned PWM waveforms in an example where TIMx_ARR = 8.
Figure 629. Edge-aligned PWM waveforms (ARR = 8)

0

Counter register

1

2

3

4

5

6

7

8

0

1

tim_ocxref
CCRx=4
CCxIF

tim_ocxref
CCRx=8
CCxIF

tim_ocxref

‘1’

CCRx>8
CCxIF

tim_ocxref

‘0’

CCRx=0
CCxIF
MSv62327V1

Down-counting configuration
•

Down-counting is active when DIR bit in TIMx_CR1 register is high. Refer to Downcounting mode.
In PWM mode 1, the reference signal tim_ocxref is low as long as
TIMx_CNT>TIMx_CCRx else it becomes high. If the compare value in TIMx_CCRx is
greater than the autoreload value in TIMx_ARR, then tim_ocxref is held at 100%. PWM
is not possible in this mode.

PWM center-aligned mode
Center-aligned mode is active when the CMS bits in TIMx_CR1 register are different from
00 (all the remaining configurations having the same effect on the tim_ocxref/tim_ocx
signals). The compare flag is set when the counter counts up, when it counts down or both
when it counts up and down depending on the CMS bits configuration. The direction bit

RM0456 Rev 6

<!-- pagebreak -->

2350

General-purpose timers (TIM2/TIM3/TIM4/TIM5)

RM0456

(DIR) in the TIMx_CR1 register is updated by hardware and must not be changed by
software. Refer to Center-aligned mode (up/down-counting).
Figure 630 shows some center-aligned PWM waveforms in an example where:
•

TIMx_ARR = 8.

•

PWM mode is the PWM mode 1.

•

The flag is set when the counter counts down corresponding to the center-aligned
mode 1 selected for CMS = 01 in TIMx_CR1 register.
Figure 630. Center-aligned PWM waveforms (ARR = 8)
Counter register

0

1

2

3

4

5

6

7

8

7

6

5

4

3

2

1

0

1

tim_ocxref
CCRx = 4
CMS=01
CMS=10
CMS=11

CCxIF

tim_ocxref
CCRx=7

CMS=10 or 11

CCxIF
tim_ocxref
CCRx=8

‘1’

CMS=01
CMS=10
CMS=11

CCxIF

tim_ocxref
CCRx>8

‘1’

CMS=01
CMS=10
CMS=11

CCxIF

tim_ocxref
CCRx=0
CCxIF

‘0’

CMS=01
CMS=10
CMS=11
MSv62328V1

Hints on using center-aligned mode:
•

<!-- pagebreak -->

When starting in center-aligned mode, the current up-down configuration is used. It
means that the counter counts up or down depending on the value written in the DIR bit

RM0456 Rev 6

RM0456

General-purpose timers (TIM2/TIM3/TIM4/TIM5)
in the TIMx_CR1 register. Moreover, the DIR and CMS bits must not be changed at the
same time by the software.
•

•

Writing to the counter while running in center-aligned mode is not recommended as it
can lead to unexpected results. In particular:
–

The direction is not updated if a value greater than the autoreload value is written
in the counter (TIMx_CNT>TIMx_ARR). For example, if the counter was counting
up, it continues to count up.

–

The direction is updated if 0 or the TIMx_ARR value is written in the counter but no
update event UEV is generated.

The safest way to use center-aligned mode is to generate an update by software
(setting the UG bit in the TIMx_EGR register) just before starting the counter and not to
write the counter while it is running.

Dithering mode
The PWM mode effective resolution can be increased by enabling the dithering mode, using
the DITHEN bit in the TIMx_CR1 register. This applies to both the CCR (for duty cycle
resolution increase) and ARR (for PWM frequency resolution increase).
The operating principle is to have the actual CCR (or ARR) value slightly changed (adding
or not one timer clock period) over 16 consecutive PWM periods, with predefined patterns.
This allows a 16-fold resolution increase, considering the average duty cycle or PWM
period. Figure 631 presents the dithering principle applied to four consecutive PWM cycles.
Figure 631. Dithering principle
Average duty cycle

7

5

DC = 7/5

DC = (7+¼)/5

DC = (7+½)/5

DC = (7+¾)/5

DC = 8/5
1 clock cycle

MSv45752V1

When the dithering mode is enabled, the register coding is changed as following (see
Figure 632 for example):
•

The four LSBs are coding for the enhanced resolution part (fractional part).

•

The MSBs are left-shifted by four places and are coding for the base value. In 16-bit
mode, the 16-bit format is maintained.

RM0456 Rev 6

<!-- pagebreak -->

2350

General-purpose timers (TIM2/TIM3/TIM4/TIM5)
Note:

RM0456

The following sequence must be followed when resetting the DITHEN bit:
1.

CEN and ARPE bits must be reset.

2.

The DITHEN bit must be reset.

3.

The CCIF flags must be cleared.

4.

The CEN bit can be set (eventually with ARPE = 1).
Figure 632. Data format and register coding in dithering mode

b31

b0

Register format in
dithering mode
(32-bit)

b31
Register format in
dithering mode
(16-bit)

LSB: 4-bits
fractional part

MSB: 28-bits, integer part

b19
Reserved

b0
LSB: 4-bits
fractional part

MSB: 16-bits, integer part

b19

b0
326

Example
20

6

Base compare value is
20 during 16 periods

Additional 6 cycles are spread
over the 16 periods
MSv50911V1

The minimum frequency is given by the following formula:
F Tim
F Tim
Resolution = ------------- ⇒ F pwmMin = -----------------------------------F pwm
Max Resolution
F Tim
Dithering mode disabled: F pwmMin = --------------65536
F Tim
Dithering mode (16-bit timer): F pwmMin = ----------------------------65535 + 15
-----16
F Tim
Dithering mode (32-bit timer): F pwmMin = -----------------------------------------268435454 + 15
-----16

Note:

<!-- pagebreak -->

For 16-bit timers, the maximum TIMx_ARR and TIMxCCRy values are limited to 0xFFFEF
in dithering mode (corresponds to 65534 for the integer part and 15 for the dithered part).
For 32-bit timers, the maximum TIMx_ARR and TIMxCCRy values are limited to
RM0456 Rev 6

RM0456

General-purpose timers (TIM2/TIM3/TIM4/TIM5)
0xFFFFFFEF in dithering mode (corresponds to 264435454 for the integer part and 15 for
the dithered part).
As shown on Figure 633 and Figure 634, the dithering mode is used to increase the PWM
resolution.
Figure 633. PWM resolution vs frequency (16-bit mode)
PWM resolution

20-bit

16-bit
Dithering

No Dithering

PWM frequency

FPWM
min

MSv47464V2

Figure 634. PWM resolution vs frequency (32-bit mode)
PWM Resolution
32-bit
No Dithering

F(cnt) min
No dithering

Dithering

F(cnt) min
with dithering

PWM frequency
MSv50912V1

The duty cycle and/or period changes are spread over 16 consecutive periods, as described
in Figure 635.

RM0456 Rev 6

<!-- pagebreak -->

2350

General-purpose timers (TIM2/TIM3/TIM4/TIM5)

RM0456

Figure 635. PWM dithering pattern

Counter period

1

2

3

4

5

6

7

8

CCR1 value

Compare1 value

21

20

20

20

20

20

20

20

21

20

21

20

21

20

20

20

21

21

21

21

21

21

21

20

13

14

15

16

21

20

20

20

20

20

20

20

21

20

21

20

21

20

20

20

21

21

21

21

21

21

21

20

21

21

21

21

21

21

21

21

41

40

40

40

40

40

40

336

21

21

21

21

21

21

21

21

ARR value
Auto-Reload
value

12

334

CCR4 value

Compare4 value

11

326

CCR3 value

Compare3 value

10

322

CCR2 value

Compare2 value

9

643

41

40

40

40

41

40

40

40

40
MSv45755V1

The autoreload and compare values increments are spread following specific patterns
described in Table 567. The dithering sequence is done to have increments distributed as
evenly as possible and minimize the overall ripple.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

General-purpose timers (TIM2/TIM3/TIM4/TIM5)
Table 567. CCR and ARR register change dithering pattern
PWM period
LSB value
1

2

3

4

5

6

7

8

9

10

11

12

13

14

15

16

0000

-

-

-

-

-

-

-

-

-

-

-

-

-

-

-

-

0001

+1

-

-

-

-

-

-

-

-

-

-

-

-

-

-

-

0010

+1

-

-

-

-

-

-

-

+1

-

-

-

-

-

-

-

0011

+1

-

-

-

+1

-

-

-

+1

-

-

-

-

-

-

-

0100

+1

-

-

-

+1

-

-

-

+1

-

-

-

+1

-

-

-

0101

+1

-

+1

-

+1

-

-

-

+1

-

-

-

+1

-

-

-

0110

+1

-

+1

-

+1

-

-

-

+1

-

+1

-

+1

-

-

-

0111

+1

-

+1

-

+1

-

+1

-

+1

-

+1

-

+1

-

-

-

1000

+1

-

+1

-

+1

-

+1

-

+1

-

+1

-

+1

-

+1

-

1001

+1

+1

+1

-

+1

-

+1

-

+1

-

+1

-

+1

-

+1

-

1010

+1

+1

+1

-

+1

-

+1

-

+1

+1

+1

-

+1

-

+1

-

1011

+1

+1

+1

-

+1

+1

+1

-

+1

+1

+1

-

+1

-

+1

-

1100

+1

+1

+1

-

+1

+1

+1

-

+1

+1

+1

-

+1

+1

+1

-

1101

+1

+1

+1

+1

+1

+1

+1

-

+1

+1

+1

-

+1

+1

+1

-

1110

+1

+1

+1

+1

+1

+1

+1

-

+1

+1

+1

+1

+1

+1

+1

-

1111

+1

+1

+1

+1

+1

+1

+1

+1

+1

+1

+1

+1

+1

+1

+1

-

The dithering mode is also available in center-aligned PWM mode (CMS bits in TIMx_CR1
register are not equal to 00). In this case, the dithering pattern is applied over eight
consecutive PWM periods, considering the up and down-counting phases as shown in
Figure 636.
Figure 636. Dithering effect on duty cycle in center-aligned PWM mode

No dithering

Dithering up

Dithering down
MSv50904V1

RM0456 Rev 6

<!-- pagebreak -->

2350

General-purpose timers (TIM2/TIM3/TIM4/TIM5)

RM0456

Table 568 shows how the dithering pattern is added in center-aligned PWM mode.
Table 568. CCR register change dithering pattern in center-aligned PWM mode
PWM period
LSB
value

1

2

3

4

5

6

7

8

Up

Dn

Up

Dn

Up

Dn

Up

Dn

Up

Dn

Up

Dn

Up

Dn

Up

Dn

0000

-

-

-

-

-

-

-

-

-

-

-

-

-

-

-

-

0001

+1

-

-

-

-

-

-

-

-

-

-

-

-

-

-

-

0010

+1

-

-

-

-

-

-

-

+1

-

-

-

-

-

-

-

0011

+1

-

-

-

+1

-

-

-

+1

-

-

-

-

-

-

-

0100

+1

-

-

-

+1

-

-

-

+1

-

-

-

+1

-

-

-

0101

+1

-

+1

-

+1

-

-

-

+1

-

-

-

+1

-

-

-

0110

+1

-

+1

-

+1

-

-

-

+1

-

+1

-

+1

-

-

-

0111

+1

-

+1

-

+1

-

+1

-

+1

-

+1

-

+1

-

-

-

1000

+1

-

+1

-

+1

-

+1

-

+1

-

+1

-

+1

-

+1

-

1001

+1

+1

+1

-

+1

-

+1

-

+1

-

+1

-

+1

-

+1

-

1010

+1

+1

+1

-

+1

-

+1

-

+1

+1

+1

-

+1

-

+1

-

1011

+1

+1

+1

-

+1

+1

+1

-

+1

+1

+1

-

+1

-

+1

-

1100

+1

+1

+1

-

+1

+1

+1

-

+1

+1

+1

-

+1

+1

+1

-

1101

+1

+1

+1

+1

+1

+1

+1

-

+1

+1

+1

-

+1

+1

+1

-

1110

+1

+1

+1

+1

+1

+1

+1

-

+1

+1

+1

+1

+1

+1

+1

-

1111

+1

+1

+1

+1

+1

+1

+1

+1

+1

+1

+1

+1

+1

+1

+1

-

55.4.12

Asymmetric PWM mode
Asymmetric mode allows two center-aligned PWM signals to be generated with a
programmable phase shift. While the frequency is determined by the value of the
TIMx_ARR register, the duty cycle and the phase-shift are determined by a pair of
TIMx_CCRx registers. One register controls the PWM during up-counting, the second
during down-counting, so that PWM is adjusted every half PWM cycle:
•

tim_oc1refc (or tim_oc2refc) is controlled by TIMx_CCR1 and TIMx_CCR2.

•

tim_oc3refc (or tim_oc4refc) is controlled by TIMx_CCR3 and TIMx_CCR4.

Asymmetric PWM mode can be selected independently on two channels (one tim_ocx
output per pair of CCR registers) by writing 1110 (Asymmetric PWM mode 1) or 1111
(Asymmetric PWM mode 2) in the OCxM bits in the TIMx_CCMRx register.
Note:

The OCxM[3:0] bitfield is split into two parts for compatibility reasons, the most significant bit
is not contiguous with the three least significant ones.
When a given channel is used as asymmetric PWM channel, its secondary channel can also
be used. For instance, if an tim_oc1refc signal is generated on channel 1 (Asymmetric PWM
mode 1), it is possible to output either the tim_oc2ref signal on channel 2, or an tim_oc2refc
signal resulting from asymmetric PWM mode 2.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

General-purpose timers (TIM2/TIM3/TIM4/TIM5)
Figure 637 shows an example of signals that can be generated using asymmetric PWM
mode (channels 1 to 4 are configured in asymmetric PWM mode 2).
Figure 637. Generation of two phase-shifted PWM signals with 50% duty cycle

Counter register

0

1

2

3

4

5

6

7

8

7

6

5

4

3

2

1

0

1

tim_oc1refc
CCR1=0
CCR2=8
tim_oc3refc
CCR3=3
CCR4=5
MSv62329V1

55.4.13

Combined PWM mode
Combined PWM mode allows two edge or center-aligned PWM signals to be generated with
programmable delay and phase shift between respective pulses. While the frequency is
determined by the value of the TIMx_ARR register, the duty cycle and delay are determined
by the two TIMx_CCRx registers. The resulting signals, tim_ocxrefc, are made of an OR or
AND logical combination of two reference PWMs:
•

tim_oc1refc (or tim_oc2refc) is controlled by TIMx_CCR1 and TIMx_CCR2

•

tim_oc3refc (or tim_oc4refc) is controlled by TIMx_CCR3 and TIMx_CCR4

Combined PWM mode can be selected independently on two channels (one tim_ocx output
per pair of CCR registers) by writing 1100 (Combined PWM mode 1) or 1101 (Combined
PWM mode 2) in the OCxM bits in the TIMx_CCMRx register.
When a given channel is used as combined PWM channel, its secondary channel must be
configured in the opposite PWM mode (for instance, one in Combined PWM mode 1 and the
other in Combined PWM mode 2).
Note:

The OCxM[3:0] bitfield is split into two parts for compatibility reasons, the most significant bit
is not contiguous with the three least significant ones.
Figure 638 shows an example of signals that can be generated using combined PWM
mode, obtained with the following configuration:
•

Channel 1 is configured in Combined PWM mode 2.

•

Channel 2 is configured in PWM mode 1.

•

Channel 3 is configured in Combined PWM mode 2.

•

Channel 4 is configured in PWM mode 1.

RM0456 Rev 6

<!-- pagebreak -->

2350

General-purpose timers (TIM2/TIM3/TIM4/TIM5)

RM0456

Figure 638. Combined PWM mode on channels 1 and 3

CCR2
CCR1
tim_oc1ref
tim_oc2ref
tim_oc1refc
tim_oc1refc = tim_oc1ref AND tim_oc2ref

CCR2
CCR1

tim_oc1ref
tim_oc2ref
tim_oc1refc
tim1_oc1refc = tim1_oc1ref OR tim1_oc2ref

55.4.14

MSv62330V1

Clearing the tim_ocxref signal on an external event
The tim_ocxref signal of a given channel can be cleared when a high level is applied on the
tim_ocref_clr_int input (OCxCE enable bit in the corresponding TIMx_CCMRx register set to
1). tim_ocxref remains low until the next transition to the active state, on the following PWM
cycle. This function can only be used in Output compare and PWM modes. It does not work
in Forced mode.
The tim_ocref_clr_int source depends on the OCREF clear selection feature
implementation, refer to Section 55.3: TIM2/TIM3/TIM4/TIM5 implementation.
If the OCREF clear selection feature is implemented, the tim_ocref_clr_int can be selected
between the tim_ocref_clr input and the tim_etrf input (tim_etr_in after the filter) by
configuring the OCCS bit in the TIMx_SMCR register. The tim_ocref_clr input can be
selected among several tim_ocref_clr[7:0] inputs, using the OCRSEL[2:0] bitfield in the
TIMx_AF2 register, as shown in Figure 639.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

General-purpose timers (TIM2/TIM3/TIM4/TIM5)
Figure 639. OCREF_CLR input selection multiplexer
TIMx_SMCR
OCCS

TIMx_AF2
OCRSEL[2:0]
tim_ocref_clr0
tim_ocref_clr1
tim_ocref_clr2
tim_ocref_clr3
tim_ocref_clr4
tim_ocref_clr5
tim_ocref_clr6
tim_ocref_clr7

tim_ocref_clr
tim_ocref_clr_int
tim_etrf

MSv62341V2

If the OCREF clear selection feature is not implemented, the tim_ocref_clr_int input is
directly connected to the tim_etrf input.
For example, the tim_ocref_clr_int signal can be connected to the output of a comparator to
be used for current handling. In this case, tim_etr_in must be configured as follows:
1.

The external trigger prescaler must be kept off: bits ETPS[1:0] in the TIMx_SMCR
register are cleared to 00.

2.

The external clock mode 2 must be disabled: bit ECE in the TIM1_SMCR register is
cleared to 0.

3.

The external trigger polarity (ETP) and the external trigger filter (ETF) can be
configured according to the application’s needs.

Figure 640 shows the behavior of the tim_ocxref signal when the tim_etrf input becomes
high, for both values of the OCxCE enable bit. In this example, the timer TIMx is
programmed in PWM mode.
Figure 640. Clearing TIMx tim_ocxref

(CCRx)
Counter (CNT)

tim_etrf

tim_ocxref
(OCxCE = ‘0’)
tim_ocxref
(OCxCE = ‘1’)

tim_ocref_clr_int
becomes high

tim_ocref_clr_int
still high
MSv62342V1

RM0456 Rev 6

<!-- pagebreak -->

2350

General-purpose timers (TIM2/TIM3/TIM4/TIM5)

RM0456

Note:

In case of a PWM with a 100% duty cycle (if CCRx>ARR), tim_ocxref is enabled again at
the next counter overflow.

55.4.15

One-pulse mode
One-pulse mode (OPM) is a particular case of the previous modes. It allows the counter to
be started in response to a stimulus and to generate a pulse with a programmable length
after a programmable delay.
Starting the counter can be controlled through the slave mode controller. Generating the
waveform can be done in output compare mode or PWM mode. One-pulse mode is selected
by setting the OPM bit in the TIMx_CR1 register. This makes the counter stop automatically
at the next update event UEV.
A pulse can be correctly generated only if the compare value is different from the counter
initial value. Before starting (when the timer is waiting for the trigger), the configuration must
be:
CNT<CCRx ≤ ARR (in particular, 0<CCRx).
Figure 641. Example of One-pulse mode
tim_ti2
tim_oc1ref
tim_oc1

Counter

TIMx_ARR
TIMx_CCR1

0
t

DELAY

t

PULSE

t
MSv62344V1

For example if the user wants to generate a positive pulse on tim_oc1 with a length of
tPULSE and after a delay of tDELAY as soon as a positive edge is detected on the tim_ti2 input
pin.
Use tim_ti2fp2 as trigger 1:

<!-- pagebreak -->

RM0456 Rev 6

RM0456

General-purpose timers (TIM2/TIM3/TIM4/TIM5)
1.

Select the proper tim_ti2_in[15:0] source (internal or external) with the TI2SEL[3:0] bits
in the TIMx_TISEL register.

2.

Map tim_ti2fp2 on tim_ti2 by writing CC2S = 01 in the TIMx_CCMR1 register.

3.

tim_ti2fp2 must detect a rising edge, write CC2P = 0 and CC2NP = 0 in the
TIMx_CCER register.

4.

Configure tim_ti2fp2 as trigger for the slave mode controller (tim_trgi) by writing TS =
00110 in the TIMx_SMCR register.

5.

tim_ti2fp2 is used to start the counter by writing SMS to 110 in the TIMx_SMCR register
(trigger mode).

The OPM waveform is defined by writing the compare registers (taking into account the
clock frequency and the counter prescaler).
•

The tDELAY is defined by the value written in the TIMx_CCR1 register.

•

The tPULSE is defined by the difference between the autoreload value and the compare
value (TIMx_ARR - TIMx_CCR1).

•

Suppose the user wants to build a waveform with a transition from 0 to 1 when a
compare match occurs and a transition from 1 to 0 when the counter reaches the
autoreload value. To do this PWM mode 2 must be enabled by writing OC1M = 111 in
the TIMx_CCMR1 register. Optionally the preload registers can be enabled by writing
OC1PE = 1 in the TIMx_CCMR1 register and ARPE in the TIMx_CR1 register. In this
case one has to write the compare value in the TIMx_CCR1 register, the autoreload
value in the TIMx_ARR register, generate an update by setting the UG bit and wait for
external trigger event on tim_ti2. CC1P is written to 0 in this example.

In this example, the DIR and CMS bits in the TIMx_CR1 register must be low.
Since only one pulse (Single mode) is needed, a one must be written in the OPM bit in the
TIMx_CR1 register to stop the counter at the next update event (when the counter rolls over
from the autoreload value back to 0). When OPM bit in the TIMx_CR1 register is set to 0, so
the Repetitive mode is selected.

Particular case: tim_ocx fast enable:
In One-pulse mode, the edge detection on tim_tix input set the CEN bit which enables the
counter. Then the comparison between the counter and the compare value makes the
output toggle. But several clock cycles are needed for these operations and it limits the
minimum delay tDELAY min we can get.
If one wants to output a waveform with the minimum delay, the OCxFE bit can be set in the
TIMx_CCMRx register. Then tim_ocxref (and tim_ocx) is forced in response to the stimulus,
without taking in account the comparison. Its new level is the same as if a compare match
had occurred. OCxFE acts only if the channel is configured in PWM1 or PWM2 mode.

55.4.16 Retriggerable one-pulse mode
This mode allows the counter to be started in response to a stimulus and to generate a
pulse with a programmable length, but with the following differences with non-retriggerable
one-pulse mode described in Section 55.4.15:
•

The pulse starts as soon as the trigger occurs (no programmable delay).

•

The pulse is extended if a new trigger occurs before the previous one is completed.

RM0456 Rev 6

<!-- pagebreak -->

2350

General-purpose timers (TIM2/TIM3/TIM4/TIM5)

RM0456

The timer must be in Slave mode, with the bits SMS[3:0] = 1000 (Combined Reset + trigger
mode) in the TIMx_SMCR register, and the OCxM[3:0] bits set to 1000 or 1001 for
Retriggerable OPM mode 1 or 2.
If the timer is configured in Up-counting mode, the corresponding CCRx must be set to 0
(the ARR register sets the pulse length). If the timer is configured in down-counting mode
CCRx must be above or equal to ARR.
Note:

In Retriggerable one-pulse mode, the CCxIF flag is not significant.
The OCxM[3:0] and SMS[3:0] bitfields are split into two parts for compatibility reasons, the
most significant bit is not contiguous with the three least significant ones.
This mode must not be used with center-aligned PWM modes. It is mandatory to have
CMS[1:0] = 00 in TIMx_CR1.
Figure 642. Retriggerable one-pulse mode

tim_trgi

Counter

tim_ocx
MSv62345V2

55.4.17

Pulse on compare mode
A pulse can be generated upon compare match event. A signal with a programmable pulse
width generated when the counter value equals a given compare value, for debugging or
synchronization purposes.
This mode is available for any slave mode selection, including encoder modes, in edge and
center aligned counting modes. It is solely available for channel 3 and channel 4. The pulse
generator is unique and is shared by the two channels, as shown on Figure 643.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

General-purpose timers (TIM2/TIM3/TIM4/TIM5)
Figure 643. Pulse generator circuitry
Set
Reset

R/S
tim_oc3

OC3M = 1010

CCR3 match

Enable

CCR4 match

Enable

Pulse generator

OC4M = 1010

tim_oc4
PWPRSC PW[7:0]
[2:0]
Reset

R/S

Set

MSv62346V1

Figure 644 shows how the pulse is generated for edge-aligned and encoder operating
modes.
Figure 644. Pulse generation on compare event, for edge-aligned and encoder modes
Counter
CMP3

Triggers
tim_ocx
Extended pulsewidth
due to re-trigger

Counter
CMP3

Triggers
tim_ocx
MSv62347V1

This output compare mode is selected using the OC3M[3:0] and OC4M[3:0] bitfields in
TIMx_CCMR2 register.

RM0456 Rev 6

<!-- pagebreak -->

2350

General-purpose timers (TIM2/TIM3/TIM4/TIM5)

RM0456

The pulse width is programmed using the PW[7:0] bitfield in the register, using a specific
clock prescaled according to PWPRSC[2:0] bits, as follows:
tPW = PW[7:0] x tPWG
where tPWG = (2(PWPRSC[2:0])) x ttim_ker_ck.
gives the resolution and maximum values depending on the prescaler value.
The pulse is retriggerable: a new trigger while the pulse is ongoing, causes the pulse to be
extended.
Note:

If the two channels are enabled simultaneously, the pulses are issued independently as long
as the trigger on one channel is not overlapping the pulse generated on the concurrent
output. On the opposite, if the two triggers are overlapping, the pulse width related to the
first arriving trigger is extended (because of the retrigger), while the pulse width of the last
arriving trigger is correct (as shown on Figure 645).
Figure 645. Extended pulse width in case of concurrent triggers
Trigger CMP3

Trigger CMP4

tim_oc3
Extended pulsewidth
due to overlapping
CMP4 trigger

tim_oc4
MSv62348V1

55.4.18

Encoder interface mode
Quadrature encoder
To select Encoder interface mode write SMS = 0001 in the TIMx_SMCR register if the
counter is counting on tim_ti1 edges only, SMS = 0010 if it is counting on tim_ti2 edges only
and SMS = 0011 if it is counting on both tim_ti1 and tim_ti2 edges.
Select the tim_ti1 and tim_ti2 polarity by programming the CC1P and CC2P bits in the
TIMx_CCER register. CC1NP and CC2NP must be kept cleared. When needed, the input
filter can be programmed as well.
The two inputs tim_ti1 and tim_ti2 are used to interface to an incremental encoder. Refer to
Table 569. The counter is clocked by each valid transition on tim_ti1fp1 or tim_ti2fp2 (tim_ti1
and tim_ti2 after input filter and polarity selection, tim_ti1fp1 = tim_ti1 if not filtered and not
inverted, tim_ti2fp2 = tim_ti2 if not filtered and not inverted) assuming that it is enabled
(CEN bit in TIMx_CR1 register written to 1). The sequence of transitions of the two inputs is
evaluated and generates count pulses as well as the direction signal. Depending on the
sequence the counter counts up or down, the DIR bit in the TIMx_CR1 register is modified
by hardware accordingly. The DIR bit is calculated at each transition on any input (tim_ti1 or

<!-- pagebreak -->

RM0456 Rev 6

RM0456

General-purpose timers (TIM2/TIM3/TIM4/TIM5)
tim_ti2), whatever the counter is counting on tim_ti1 only, tim_ti2 only or both tim_ti1 and
tim_ti2.
Encoder interface mode acts simply as an external clock with direction selection. This
means that the counter just counts continuously between 0 and the autoreload value in the
TIMx_ARR register (0 to ARR or ARR down to 0 depending on the direction). So the
TIMx_ARR must be configured before starting. In the same way, the capture, compare,
prescaler, trigger output features continue to work as normal. Encoder mode and External
clock mode 2 are not compatible and must not be selected together.
In this mode, the counter is modified automatically following the speed and the direction of
the quadrature encoder and its content, therefore, always represents the encoder’s position.
The count direction corresponds to the rotation direction of the connected sensor. The table
summarizes the possible combinations, assuming tim_ti1 and tim_ti2 do not switch at the
same time.
Table 569. Counting direction versus encoder signals(CC1P = CC2P = 0)

Active edge

SMS[3:0]

Counting on
tim_ti1 only
x1 mode

1110

Counting on
tim_ti2 only
x1 mode

1111

Counting on
tim_ti1 only
x2 mode

0001

Counting on
tim_ti2 only
x2 mode

0010

Counting on
tim_ti1 and
tim_ti2
x4 mode

0011

Level on opposite
signal (tim_ti1fp1
for tim_ti2,
tim_ti2fp2 for
tim_ti1)

tim_ti1fp1 signal

tim_ti2fp2 signal

Rising

Falling

Rising

Falling

High

Down

Up

No count

No count

Low

No count

No count

No count

No count

High

No count

No count

Up

Down

Low

No count

No count

No count

No count

High

Down

Up

No count

No count

Low

Up

Down

No count

Down

High

No count

No count

Up

Down

Low

No count

No count

Down

Up

High

Down

Up

Up

Down

Low

Up

Down

Down

Up

A quadrature encoder can be connected directly to the MCU without external interface logic.
However, comparators are normally be used to convert the encoder’s differential outputs to
digital signals. This greatly increases noise immunity. The third encoder output which
indicates the mechanical zero position, can be connected to the external trigger input and
trigger a counter reset.
Figure 646 gives an example of counter operation, showing count signal generation and
direction control. It also shows how input jitter is compensated where both edges are

RM0456 Rev 6

<!-- pagebreak -->

2350

General-purpose timers (TIM2/TIM3/TIM4/TIM5)

RM0456

selected. This might occur if the sensor is positioned near to one of the switching points. For
this example we assume that the configuration is the following:
•

CC1S = 01 (TIMx_CCMR1 register, tim_ti1fp1 mapped on tim_ti1).

•

CC2S = 01 (TIMx_CCMR1 register, tim_ti2fp2 mapped on tim_ti2).

•

CC1P and CC1NP = 0 (TIMx_CCER register, tim_ti1fp1 noninverted,
tim_ti1fp1 = tim_ti1).

•

CC2P and CC2NP = 0 (TIMx_CCER register, tim_ti2fp2 noninverted,
tim_ti2fp2 = tim_ti2).

•

SMS = 0011 (TIMx_SMCR register, both inputs are active on both rising and falling
edges).

•

CEN = 1 (TIMx_CR1 register, counter is enabled).
Figure 646. Example of counter operation in encoder interface mode
forward

jitter

backward

jitter

forward

tim_ti1
tim_ti2

Counter

up

down

up

MSv62349V1

Figure 647 gives an example of counter behavior when tim_ti1fp1 polarity is inverted (same
configuration as above except CC1P = 1).
Figure 647. Example of encoder interface mode with tim_ti1fp1 polarity inverted
forward

jitter

backward

jitter

forward

tim_ti1
tim_ti2

Counter

down

up

down
MSv62350V1

<!-- pagebreak -->

RM0456 Rev 6

RM0456

General-purpose timers (TIM2/TIM3/TIM4/TIM5)
Figure 648 shows the timer counter value during a speed reversal, for various counting
modes.
Figure 648. Quadrature encoder counting modes
tim_ti1

tim_ti2

DIR bit

Counter x4

Counter x2

Counter x1

6 7 8 9 0 1 2 3 4

8

9

9

0

1

0

5

2

1

4 3 2 1 0 9 8 7 6 5 4 3 2

1

0

9

0

8

9

7

6

8
MSv62351V1

The timer, when configured in Encoder Interface mode provides information on the sensor’s
current position. Dynamic information can be obtained (speed, acceleration, deceleration)
by measuring the period between two encoder events using a second timer configured in
capture mode. The output of the encoder which indicates the mechanical zero can be used
for this purpose. Depending on the time between two events, the counter can also be read
at regular times. This can be done by latching the counter value into a third input capture
register if available (then the capture signal must be periodic and can be generated by
another timer). When available, it is also possible to read its value through a DMA request.
The IUFREMAP bit in the TIMx_CR1 register forces a continuous copy of the update
interrupt flag (UIF) into the timer counter register’s bit 31 (TIMxCNT[31]). This allows both
the counter value and a potential roll-over condition signaled by the UIFCPY flag to be read
in an atomic way. It eases the calculation of angular speed by avoiding race conditions
caused, for instance, by a processing shared between a background task (counter reading)
and an interrupt (update interrupt).
There is no latency between the UIF and UIFCPY flag assertions.
In 32-bit timer implementations, when the IUFREMAP bit is set, bit 31 of the counter is
overwritten by the UIFCPY flag upon read access (the counter’s most significant bit is only
accessible in write mode).

Clock plus direction encoder mode
In addition to the quadrature encoder mode, the timer offers support for other types of
encoders.
In the “clock plus direction” mode shown on Figure 649, the clock is provided on a single
line, on tim_ti2, while the direction is forced using the tim_ti1 input.

RM0456 Rev 6

<!-- pagebreak -->

2350

General-purpose timers (TIM2/TIM3/TIM4/TIM5)

RM0456

This mode is enabled with the SMS[3:0] bitfield in the TIMx_SMCR register, as following:
•

1010: x2 mode, the counter is updated on both rising and falling edges of the clock.

•

1011: x1 mode, the counter is updated on a single clock edge, as per CC2P bit value:
CC2P = 0 corresponds to rising edge sensitivity and CC2P = 1 corresponds to falling
edge sensitivity.

The polarity of the direction signal on tim_ti1 is set with the CC1P bit: 0 corresponds to
positive polarity (up-counting when tim_ti1 is high and down-counting when tim_ti1 is low)
and CC1P = 1 corresponds to negative polarity (up-counting when tim_ti1 is low).
Figure 649. Direction plus clock encoder mode
tim_ti1

tim_ti2

Counter x2 mode

6

Counter x1 mode

6

7

8

7

9

10

8

11

10

9

9

8

8

7

6

7
MSv62352V1

Directional clock encoder mode
In the “directional clock” mode on Figure 650, the clocks are provided on two lines, with a
single one at once, depending on the direction, so as to have one up-counting clock line and
one down-counting clock line.
This mode is enabled with the SMS[3:0] bitfield in the TIMx_SMCR register, as following:

<!-- pagebreak -->

•

1100: x2 mode, the counter is updated on both rising and falling edges of any of the two
clock lines. The CC1P and CC2P bits are coding for the clock idle state. CCxP = 0
corresponds to high-level idle state (refer to Figure 650) and CCxP = 1 corresponds to
low-level idle state (refer to Figure 651).

•

1101: x1 mode, the counter is updated on a single clock edge, as per CC1P and CC2P
bit value. CCxP = 0 corresponds to falling edge sensitivity and high-level idle state
(refer to Figure 650), CCxP = 1 corresponds to rising edge sensitivity and low-level idle
state (refer to Figure 651).

RM0456 Rev 6

RM0456

General-purpose timers (TIM2/TIM3/TIM4/TIM5)
Figure 650. Directional clock encoder mode (CC1P = CC2P = 0)
tim_ti1

tim_ti2

DIR bit

Counter x2 mode

6

7

8

6

Counter x1 mode

9

10

7

11

10

8

9

8

7

7

6

6

5

5
MSv62353V1

Figure 651. Directional clock encoder mode (CC1P = CC2P = 1)
tim_ti1

tim_ti2

DIR bit

Counter x2 mode

Counter x1 mode

6

7

7

8

9

10

8

11

9

10

9

8

8

7

7

6

5

6
MSv62354V1

RM0456 Rev 6

<!-- pagebreak -->

2350

General-purpose timers (TIM2/TIM3/TIM4/TIM5)

RM0456

Table 570 details how the directional clock mode operates, for any input transition.
Table 570. Counting direction versus encoder signals and polarity settings

Directional
clock mode

SMS[3:0]

x2 mode
CCxP = 0

1100

x2 mode
CCxP = 1

1100

x1 mode
CCxP = 0

1101

x1 mode
CCxP = 1

1101

Level on
opposite
signal
(tim_ti1fp1 for
tim_ti2,
tim_ti2fp2 for
tim_ti1)

tim_ti1fp1 signal

tim_ti2fp2 signal

Rising

Falling

Rising

Falling

High

Down

Down

Up

Up

Low

No count

No count

No count

No count

High

No count

No count

No count

No count

Low

Down

Down

Up

Up

High

No count

Down

No count

Up

Low

No count

No count

No count

No count

High

No count

No count

No count

No count

Low

Down

No count

Up

No count

Index input
The counter can be reset by an index signal coming from the encoder, indicating an
absolute reference position. The index signal must be connected to the tim_etr_in input. It
can be filtered using the digital input filter.
The index functionality is enabled with the IE bit in the TIMx_ECR register. The IE bit must
be set only in encoder mode, when the SMS[3:0] bitfield has the following values: 0001,
0010, 011, 1010, 1011, 1100, 1101, 1110, 1111.
Available encoders are proposed with several options for index pulse conditioning, as per
Figure 652:

<!-- pagebreak -->

•

Gated with A and B: the pulse width is 1/4 of one channel period, aligned with both A
and B edges.

•

Gated with A (or gated with B): the pulse width is 1/2 of one channel period, aligned
with the two edges on channel A (resp. channel B).

•

Ungated: the pulse width is up to one channel period, without any alignment to the
edges.

RM0456 Rev 6

RM0456

General-purpose timers (TIM2/TIM3/TIM4/TIM5)
Figure 652. Index gating options
Channel A

Channel B

Gated A & B

Gated A

Ungated
MSv45765V1

The circuitry tolerates jitter on index signal, whatever the gating mode, as shown on
Figure 653.
In ungated mode, the signal must be strictly below two encoder periods. If the pulse width is
greater or equal to two encoder period, the counter is reset multiple times.
Figure 653. Jittered Index signals
Channel A

Channel B

Gated A & B

Gated A

Ungated
Max pulsewidth ungated mode
MSv45766V1

The timer supports the three gating options identically, without any specific programming
needed. It is only necessary to define on which encoder state (for example channel A and
channel B state combination) the index must be synchronized, using the IPOS[1:0] bitfield
in the TIMx_ECR register.
The index detection event acts differently depending on counting direction to ensure
symmetrical operation during speed reversal:
•

The counter is reset during up-counting (DIR bit = 0).

•

The counter is set to TIMx_ARR when down-counting.

This allows the index to be generated on the very same mechanical angular position
whatever the counting direction. Figure 654 shows at which position is the index generated,
for a simplistic example (an encoder providing four edges par mechanical rotation).

RM0456 Rev 6

<!-- pagebreak -->

2350

General-purpose timers (TIM2/TIM3/TIM4/TIM5)

RM0456

Figure 654. Index generation for IPOS[1:0] = 11
AB = 00
State 1

Rotor angle = 270°

AB = 10
State 4

Up-counting
Down-counting

AB = 11
State 3

Rotor angle = 180°

Rotor angle = 0°

AB = 01
State 2

Rotor angle = 90°

The index event is always generated here
MSv45767V1

Figure 655 presents waveforms and corresponding values for IPOS[1:0] = 11. It shows that
the instant at which the counter value is forced is automatically adjusted depending on the
counting direction:
•

Counter set to 0 when encoder state is 11 (ChA = 1, ChB = 1), when up-counting
(DIR bit = 0).

•

Counter set to TIMx_ARR when exiting the 11 state, when down-counting (DIR bit = 1).

An interrupt can be issued upon index detection event.
The arrows are indicating on which transition is the index event interrupt generated.
Figure 655. Counter reading with index gated on channel A (IPOS[1:0] = 11)
Channel A

Channel B

Index

DIR bit

Counter

5 6 7 0 1 2 3 4 5

6

5 4 3 2 1 0 7 6 5 4 3 2 1
MSv45768V1

Figure 656 presents waveforms and corresponding values for the ungated mode. The
arrows are indicating on which transition is the index event generated.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

General-purpose timers (TIM2/TIM3/TIM4/TIM5)
Figure 656. Counter reading with index ungated (IPOS[1:0] = 00)
Channel A

Channel B

Index

DIR bit

Counter

3 4 5 6 7 0 1 2 3

4

3 2 1 0 7 6 5 4 3 2 1 0 7
MSv45769V1

Figure 657 shows how the gated on A & B mode is handled, for various pulse alignment
scenarios. The arrows are indicating on which transition is the index event generated.
Figure 657. Counter reading with index gated on channel A and B
Channel A

Channel B

Index

DIR bit

Counter

5 6 7 0 1 2 3 4 5

6

5 4 3 2 1 0 7 6 5 4 3 2 1
MSv45770V1

Figure 658 and Figure 659 detail the case where the subsequent index pulse may be
narrower than one quarter of the encoder clock period.

RM0456 Rev 6

<!-- pagebreak -->

2350

General-purpose timers (TIM2/TIM3/TIM4/TIM5)

RM0456

Figure 658. Encoder mode behavior in case of narrow index pulse (IPOS[1:0] = 11)
Channel A

Channel B

Index

DIR bit

Counter

5 6 7 0 1 2 3 4 5

6

5 4 3 2 1 0 7 6 5 4 3 2 1

Index leading state transition

Channel A

Channel B

Index

DIR bit

Counter

5 6 7 0 1 2 3 4 5

6

5 4 3 2 1 0 7 6 5 4 3 2 1

Index delayed versus state transition

<!-- pagebreak -->

RM0456 Rev 6

MSv45771V1

RM0456

General-purpose timers (TIM2/TIM3/TIM4/TIM5)
Figure 659. Counter reset Narrow index pulse (closer view, ARR = 0x07)
Channel A

Channel B

Index

DIR bit

Counter

5

6

7

4

5

6

0

1

2

3

0

1

2

3

Channel A

Channel B

Index

DIR bit

Counter

7

MSv45772V1

RM0456 Rev 6

<!-- pagebreak -->

2350

General-purpose timers (TIM2/TIM3/TIM4/TIM5)

RM0456

Figure 660 shows how the index is managed in x1 and x2 modes.
Figure 660. Index behavior in x1 and x2 mode (IPOS[1:0] = 01)
AB = IPOS[1:0] = 01

Channel A

Channel B

Index

DIR bit

Counter x2

Counter x1

10

11

0

5

1

6

2

7

1

0

11

0

10

1

9

8

3
MSv45773V1

Directional index sensitivity
The IDIR[1:0] bitfield in the TIMx_ECR register allows the index to be active only in a
selected counting direction.
Figure 661 shows the relationship between index and counter reset events, depending on
IDIR[1:0] value.
Figure 661. Directional index sensitivity
DIR bit

UP-counting

Down-counting

Counter

Index input

Counter reset

IDIR[1:0]=00

IDIR[1:0]=01

IDIR[1:0]=10
MSv45774V1

<!-- pagebreak -->

RM0456 Rev 6

RM0456

General-purpose timers (TIM2/TIM3/TIM4/TIM5)

Special first index event management
The FIDX bit in the TIMx_ECR register allows the index to be taken only once, as shown on
Figure 662. Once the first index has arrived, any subsequent index is ignored. If needed, the
circuitry can be rearmed by writing the FIDX bit to 0 and setting it again to 1.
Figure 662. Counter reset as function of FIDX bit setting

Counter

Counter reset

Index input

FIDX = 0

FIDX = 1
MSv45775V1

Index blanking
The index event can be blanked using the tim_ti3 or tim_ti4 inputs. During the blanking
window, the index events are no longer resetting the counter, as shown on Figure 663.
This mode is enabled using the IBLK[1:0] bitfield in the TIMx_ECR register, as following:
•

IBLK[1:0] = 00: Index signal always active.

•

IBLK[1:0] = 01: Index signal blanking on tim_ti3 input.

•

IBLK[1:0] = 10: Index signal blanking on tim_ti4 input.
Figure 663. Index blanking

Counter

Index input

Counter reset

Blanking signal
TI3 (CC3P=0)

IBLK[1:0] = 00

IBLK[1:0] = 01
MSv45776V1

RM0456 Rev 6

<!-- pagebreak -->

2350

General-purpose timers (TIM2/TIM3/TIM4/TIM5)

RM0456

Index management in nonquadrature mode
Figure 664 and Figure 665 detail how the index is managed in directional clock mode and
clock plus direction mode, when the SMS[3:0] bitfield is equal to 1010, 1011, 1100, 1101.
For both of these modes, the index sensitivity is set with the IPOS[0] bit as following:
•

IPOS[0] = 0: Index is detected on clock low level.

•

IPOS[0] = 1: Index is detected on clock high level.

The IPOS[1] bit is not-significant.
Figure 664. Index behavior in clock + direction mode, IPOS[0] = 1
Direction (TI1)

Clock (TI2)

Index

Counter x2 mode

7

Counter x1 mode

7

0

1

2

0

3

4

1

3

2

2

7

6

1

5

7
MSv45777V1

Figure 665. Index behavior in directional clock mode, IPOS[0] = 1
Clock Down (TI1)

Clock Up (TI2)

DIR bit

Counter x2 mode

9

Counter x1 mode

9

0

1

0

2

3

1

4

3

2

2

1

1

0

9

0

8

9
MSv45778V1

Encoder error management
For encoder configurations where two quadrature signals are available, it is possible to
detect transition errors. The reading on the two inputs corresponds to a 2-bit gray code
which can be represented as a state diagram, on Figure 666. A single bit is expected to
change at once. An erroneous transition sets the TERRF interrupt flag in the TIMx_SR

<!-- pagebreak -->

RM0456 Rev 6

RM0456

General-purpose timers (TIM2/TIM3/TIM4/TIM5)
status register. A transition error interrupt is generated if the TERRIE bit is set in the
TIMx_DIER register.
Figure 666. State diagram for quadrature encoded signals

00

01

10

11

Correct transitions
Erroneous transitions

MSv45779V1

For encoder having an index signal, it is possible to detect abnormal operation resulting in
an excess of pulses per revolution. An encoder with N pulses per revolution provides 4xN
counts per revolution. The index signal resets the counter every 4xN clock periods.
If the counter value is incremented from TIMx_ARR to 0 or decremented from 0 to TIMxARR
value without any index event, this is reported as an index position error.
The overflow threshold is programmed using the TIMx_ARR register. A 1000 lines encoder
results in a counter value being between 0 and 3999 (in 4x reading mode). The overflow
detection threshold must be programmed by setting TIMx_ARR = 3999 + 1 = 4000.

RM0456 Rev 6

<!-- pagebreak -->

2350

General-purpose timers (TIM2/TIM3/TIM4/TIM5)

RM0456

The error assertion is delayed to the transition 0 to 1 when in up-counting. This is to cope
with narrow index pulses in gated A and B mode, as shown on Figure 667.
Figure 667. Up-counting encoder error detection
Channel A

Channel B

Index

IERRF

Counter

5

6

7

0

1

Error detected

Abort (index detection)

6

0

2

3

2

3

Channel A

Channel B

Index

IERRF

Counter

5

7

Error detected

<!-- pagebreak -->

RM0456 Rev 6

1

Error asserted

MSv45780V1

RM0456

General-purpose timers (TIM2/TIM3/TIM4/TIM5)
In down-counting mode, the detection is conditioned by a preliminary transition from 1 to 0.
This is to cope with narrow index pulses in gated A and B mode, as shown on Figure 668, to
avoid any false error detection in case the encoder dithers between TIMx_ARR and 0
immediately after the index detection.
Figure 668. Down-counting encode error detection
Channel A

Channel B

Index

IERRF

Counter

2

1

0

7

No error: transition from 0 to
TIMx_ARR following an index

0

7

6

5

No error: transition from 0 to
TIMx_ARR without index, but not
following a transition from 1 to 0

Channel A

Channel B

Index

IERRF

Counter

2

1

0

Error detected

7

6

Error asserted

5

4

MSv47416V1

An index error sets the IERRF interrupt flag in the TIMx_SR status register. An index error
interrupt is generated if the IERRIE bit is set in the TIMx_DIER register.

Functional encoder interrupts
The following interrupts are also available in encoder mode
•

Direction change: any change of the counting direction in encoder mode causes the
DIR bit in the TIMx_CR1 register to toggle. The direction change sets the DIRF
interrupt flag in the TIMx_SR status register. A direction change interrupt is generated if
the DIRIE bit is set in the TIMx_DIER register.

•

Index event: the index event sets the IDXF interrupt flag in the TIMx_SR status register.
An index interrupt is generated if the IDXIE bit is set in the TIMx_DIER register.
RM0456 Rev 6

<!-- pagebreak -->

2350

General-purpose timers (TIM2/TIM3/TIM4/TIM5)

RM0456

Slave mode selection preload for run-time encoder mode update
It can be necessary to switch from one encoder mode to another during run-time. This is
typically done at high-speed to decrease the update interrupt rate, by switching from x4 to
x2 to x1 mode, as shown on Figure 669.
For this purpose, the SMS[3:0] bit can be preloaded. This is enabled by setting the SMSPE
enable bit in the TIMx_SMCR register. The trigger for the transfer from SMS[3:0] preload to
active value can be selected with the SMSPS bit in the TIMx_SMCR register.
•

SMSPS = 0: the transfer is triggered by the update event (UEV) occurring when the
counter overflows when up-counting, and underflows when down-counting.

•

SMSPS = 1: the transfer is triggered by the index event.
Figure 669. Encoder mode change with preload transferred on update (SMSPS = 0)
x4 mode

x2 mode

x1 mode

Update event

Preload value

Active value

SMS = 0011

SMS = 0011

SMS = 0001

SMS = 0001

SMS = 1110

SMS = 1110
MSv45781V1

Encoder clock output
The encoder mode operating principle is not perfectly suited for high-resolution velocity
measurements, at low speed, as it requires a relatively long integration time to have a
sufficient number of clock edges and a precise measurement.
At low speed, a better solution is to do an edge-to-edge clock period measurement. This
can be achieved using a slave timer. The timer can output the encoder clock information on
the tim_trgo output. The slave timer can then perform a period measurement and provide
velocity information for each and every encoder clock edge.
This mode is enabled by setting the MMS[3:0] bitfield to 1000, in the TIMx_CR2 register. It
is valid for the following SMS[3:0] values: 0001, 0010, 0011, 1010, 1011, 1100, 1101, 1110,
1111. Any other SMS[3:0] code is not allowed and may lead to unexpected behavior.

55.4.19

Direction bit output
It is possible to output a direction signal out of the timer, on the tim_oc3 and tim_oc4 output
signals (copy of the DIR bit in the TIMx_CR1 register). This is achieved by setting the
OC3M[3:0] or the OC4M[3:0] bitfield to 1011 in the TIMx_CCMR2 register.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

General-purpose timers (TIM2/TIM3/TIM4/TIM5)
This feature can be used for monitoring the counting direction (or rotation direction) in
encoder mode, or to have a signal indicating the up/down phases in center-aligned PWM
mode.

55.4.20

UIF bit remapping
The IUFREMAP bit in the TIMx_CR1 register forces a continuous copy of the update
interrupt flag (UIF) into bit 31 of the timer counter register’s bit 31 (TIMxCNT[31]). This is
used to atomically read both the counter value and a potential roll-over condition signaled by
the UIFCPY flag. It eases the calculation of angular speed by avoiding race conditions
caused, for instance, by a processing shared between a background task (counter reading)
and an interrupt (update interrupt).
There is no latency between the UIF and UIFCPY flag assertions.
In 32-bit timer implementations, when the IUFREMAP bit is set, bit 31 of the counter is
overwritten by the UIFCPY flag upon read access (the counter’s most significant bit is only
accessible in write mode).

55.4.21

Timer input XOR function
The TI1S bit in the TIM1xx_CR2 register, allows the input filter of channel 1 to be connected
to the output of an XOR gate, combining the three input pins tim_ti1, tim_ti2 and tim_ti3.
The XOR output can be used with all the timer input functions such as trigger or input
capture.
An example of this feature used to interface Hall sensors is given in Section 54.3.29:
Interfacing with Hall sensors.

55.4.22

Timers and external trigger synchronization
The TIMx timers can be synchronized with an external trigger in several modes: Reset
mode, Gated mode, Trigger mode, Reset + trigger and gated + reset modes.

Slave mode: Reset mode
The counter and its prescaler can be reinitialized in response to an event on a trigger input.
Moreover, if the URS bit from the TIMx_CR1 register is low, an update event UEV is
generated. Then all the preloaded registers (TIMx_ARR, TIMx_CCRx) are updated.
In the following example, the upcounter is cleared in response to a rising edge on tim_ti1
input:
1.

Configure the channel 1 to detect rising edges on tim_ti1. Configure the input filter
duration (in this example, we do not need any filter, so we keep IC1F = 0000). The
capture prescaler is not used for triggering, so it does not need to be configured. The
CC1S bits select the input capture source only, CC1S = 01 in the TIMx_CCMR1
register. Write CC1P = 0 and CC1NP = 0 in TIMx_CCER register to validate the polarity
(and detect rising edges only).

2.

Configure the timer in reset mode by writing SMS = 100 in TIMx_SMCR register. Select
tim_ti1 as the input source by writing TS = 00101 in TIMx_SMCR register.

3.

Start the counter by writing CEN = 1 in the TIMx_CR1 register.

The counter starts counting on the internal clock, then behaves normally until tim_ti1 rising
edge. When tim_ti1 rises, the counter is cleared and restarts from 0. In the meantime, the

RM0456 Rev 6

<!-- pagebreak -->

2350

General-purpose timers (TIM2/TIM3/TIM4/TIM5)

RM0456

trigger flag is set (TIF bit in the TIMx_SR register) and an interrupt request, or a DMA
request can be sent if enabled (depending on the TIE and TDE bits in TIMx_DIER register).
The following figure shows this behavior when the autoreload register TIMx_ARR = 0x36.
The delay between the rising edge on tim_ti1 and the actual reset of the counter is due to
the resynchronization circuit on tim_ti1 input.
Figure 670. Control circuit in reset mode

tim_ti1

UG
tim_cnt_ck, tim_psc_ck
Counter register

30 31

32 33 34 35 36 00 01 02 03 00 01 02 03

TIF
MSv62361V1

Slave mode: Gated mode
The counter can be enabled depending on the level of a selected input.
In the following example, the upcounter counts only when tim_ti1 input is low:
1.

Configure the channel 1 to detect low levels on tim_ti1. Configure the input filter
duration (in this example, we do not need any filter, so we keep IC1F = 0000). The
capture prescaler is not used for triggering, so it does not need to be configured. The
CC1S bits select the input capture source only, CC1S = 01 in TIMx_CCMR1 register.
Write CC1P = 1 and CC1NP = 0 in TIMx_CCER register to validate the polarity (and
detect low level only).

2.

Configure the timer in gated mode by writing SMS = 101 in TIMx_SMCR register.
Select tim_ti1 as the input source by writing TS = 00101 in TIMx_SMCR register.

3.

Enable the counter by writing CEN = 1 in the TIMx_CR1 register (in gated mode, the
counter does not start if CEN = 0, whatever is the trigger input level).

The counter starts counting on the internal clock as long as tim_ti1 is low and stops as soon
as tim_ti1 becomes high. The TIF flag in the TIMx_SR register is set both when the counter
starts or stops.
The delay between the rising edge on tim_ti1 and the actual stop of the counter is due to the
resynchronization circuit on tim_ti1 input.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

General-purpose timers (TIM2/TIM3/TIM4/TIM5)
Figure 671. Control circuit in gated mode
tim_ti1
CEN
tim_cnt_ck, tim_psc_ck

Counter register

30 31 32 33

34

35

36

37 38

TIF

Write TIF = 0
MSv62362V1

Note:

The configuration “CCxP = CCxNP = 1” (detection of both rising and falling edges) does not
have any effect in gated mode because gated mode acts on a level and not on an edge.

Slave mode: Trigger mode
The counter can start in response to an event on a selected input.
In the following example, the upcounter starts in response to a rising edge on tim_ti2 input:
1.

Configure the channel 2 to detect rising edges on tim_ti2. Configure the input filter
duration (in this example, we do not need any filter, so we keep IC2F = 0000). The
capture prescaler is not used for triggering, so it does not need to be configured. CC2S
bits are selecting the input capture source only, CC2S = 01 in TIMx_CCMR1 register.
Write CC2P = 1 and CC2NP = 0 in TIMx_CCER register to validate the polarity (and
detect low level only).

2.

Configure the timer in trigger mode by writing SMS = 110 in TIMx_SMCR register.
Select tim_ti2 as the input source by writing TS = 00110 in TIMx_SMCR register.

When a rising edge occurs on tim_ti2, the counter starts counting on the internal clock and
the TIF flag is set.
The delay between the rising edge on tim_ti2 and the actual start of the counter is due to the
resynchronization circuit on tim_ti2 input.
Figure 672. Control circuit in trigger mode
tim_ti2
CEN
tim_cnt_ck, tim_psc_ck
Counter register

34

35

36 37 38

TIF
MSv62363V1

RM0456 Rev 6

<!-- pagebreak -->

2350

General-purpose timers (TIM2/TIM3/TIM4/TIM5)

RM0456

Slave mode selection preload for run-time encoder mode update
The SMS[3:0] bit can be preloaded. This is enabled by setting the SMSPE enable bit in the
TIMx_SMCR register. The trigger for the transfer from SMS[3:0] preload to active value is
the update event (UEV) occurring when the counter overflows.

Slave mode – combined reset + trigger mode
In this case, a rising edge of the selected trigger input (tim_trgi) reinitializes the counter,
generates an update of the registers, and starts the counter.
This mode is used for one-pulse mode.

Slave mode – combined gated + reset mode
The counter clock is enabled when the trigger input (tim_trgi) is high. The counter stops and
is reset as soon as the trigger becomes low. Both start and stop of the counter are
controlled.
This mode is used to detect out-of-range PWM signal (duty cycle exceeding a maximum
expected value).

Slave mode – external clock mode 2 + trigger mode
The external clock mode 2 can be used in addition to another slave mode (except external
clock mode 1 and encoder mode). In this case, the tim_etr_in signal is used as external
clock input, and another input can be selected as trigger input when operating in reset
mode, gated mode, or trigger mode. It is recommended not to select tim_etr_in as tim_trgi
through the TS bits of TIMx_SMCR register.
In the following example, the upcounter is incremented at each rising edge of the tim_etr_in
signal as soon as a rising edge of tim_ti1 occurs:
1.

2.

3.

Configure the external trigger input circuit by programming the TIMx_SMCR register as
follows:
–

ETF = 0000: no filter.

–

ETPS = 00: prescaler disabled.

–

ETP = 0: detection of rising edges on tim_etr_in and ECE = 1 to enable the
external clock mode 2.

Configure the channel 1 as follows, to detect rising edges on TI:
–

IC1F = 0000: no filter.

–

The capture prescaler is not used for triggering and does not need to be
configured.

–

CC1S = 01in TIMx_CCMR1 register to select only the input capture source.

–

CC1P = 0 and CC1NP = 0 in TIMx_CCER register to validate the polarity (and
detect rising edge only).

Configure the timer in trigger mode by writing SMS = 110 in TIMx_SMCR register.
Select tim_ti1 as the input source by writing TS = 00101 in TIMx_SMCR register.

A rising edge on tim_ti1 enables the counter and sets the TIF flag. The counter then counts
on tim_etr_in rising edges.
The delay between the rising edge of the tim_etr_in signal and the actual reset of the
counter is due to the resynchronization circuit on tim_etrp input.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

General-purpose timers (TIM2/TIM3/TIM4/TIM5)
Figure 673. Control circuit in external clock mode 2 + trigger mode

tim_ti1

CEN

ETR

tim_cnt_ck, tim_psc_ck

Counter register

34

35

36

TIF
MSv62364V1

55.4.23

Timer synchronization
The TIMx timers are linked together internally for timer synchronization or chaining. When
one timer is configured in Master mode, it can reset, start, stop, or clock the counter of
another timer configured in Slave mode.
Figure 674 and Figure 675 show examples of master/slave timer connections.
Figure 674. Master/Slave timer example
TIM_mstr

TIM_slv

Clock
TS

MMS

SMS

UEV

Prescaler

Counter

Master
mode
control

tim_trgo

tim_itr

Slave
mode
control

CK_PSC
Prescaler

Counter

Input
trigger
selection
MSv62375V1

RM0456 Rev 6

<!-- pagebreak -->

2350

General-purpose timers (TIM2/TIM3/TIM4/TIM5)

RM0456

Figure 675. Master/slave connection example with 1 channel only timers
TIM_mstr

TIM_slv

Clock
Prescaler

Counter

TS

Output
Compare 1

tim_oc1

SMS

tim_itr

control

TIM_CH1

Slave
mode
control

CK_PSC
Prescaler

Counter

Input
trigger
selection
MSv65225V1

Note:

The timers with one channel only (see Figure 675) do not feature a master mode. However,
the tim_oc1 output signal can serve as trigger for slave timer (see TIMx internal trigger
connection table in Section 55.4.2: TIM2/TIM3/TIM4/TIM5 pins and internal signals).
The tim_oc1 signal pulse width must be programmed to be at least two clock cycles of the
destination timer, to make sure the slave timer detects the trigger.
For instance, if the destination timer tim_ker_ck clock is four times slower than the source
timer, the OC1 pulse width must be eight clock cycles.

Using one timer as prescaler for another timer
For example, TIM_mstr can be configured to act as a prescaler for TIM_slv. Refer to
Figure 674. To do this:

Note:

1.

Configure TIM_mstr in master mode so that it outputs a periodic trigger signal on each
update event UEV. If MMS = 010 is written in the TIM_mstr_CR2 register, a rising edge
is output on tim_trgo each time an update event is generated.

2.

To connect the tim_trgo output of TIM_mstr to TIM_slv, TIM_slv must be configured in
slave mode using ITR2 as internal trigger. This is selected through the TS bits in the
TIM_slv_SMCR register (writing TS = 00010).

3.

Then the slave mode controller must be put in external clock mode 1 (write SMS = 111
in the TIM_slv_SMCR register). This causes TIM_slv to be clocked by the rising edge
of the periodic TIM_mstr trigger signal (which correspond to the TIM_mstr counter
overflow).

4.

Finally both timers must be enabled by setting their respective CEN bits (TIMx_CR1
register).

If tim_ocx is selected on TIM_mstr as the trigger output (MMS = 1xx), its rising edge is used
to clock the counter of TIM_slv.

Using one timer to enable another timer
In this example, we control the enable of TIM_slv with the output compare 1 of TIM_mstr.
Refer to Figure 674 for connections. TIM_slv counts on the divided internal clock only when
tim_oc1ref of TIM_mstr is high. Both counter clock frequencies are divided by 3 by the
prescaler compared to tim_ker_ck (ftim_cnt_ck = ftim_ker_ck/3).

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Note:

General-purpose timers (TIM2/TIM3/TIM4/TIM5)
1.

Configure TIM_mstr master mode to send its output compare 1 reference (tim_oc1ref)
signal as trigger output (MMS = 100 in the TIM_mstr_CR2 register).

2.

Configure the TIM_mstr tim_oc1ref waveform (TIM_mstr_CCMR1 register).

3.

Configure TIM_slv to get the input trigger from TIM_mstr (TS = 00010 in the
TIM_slv_SMCR register).

4.

Configure TIM_slv in gated mode (SMS = 101 in TIM_slv_SMCR register).

5.

Enable TIM_slv by writing 1 in the CEN bit (TIM_slv_CR1 register).

6.

Start TIM_mstr by writing 1 in the CEN bit (TIM_mstr_CR1 register).

The slave timer counter clock is not synchronized with the master timer counter clock, this
mode only affects the TIM_slv counter enable signal.
Figure 676. Gating TIM_slv with tim_oc1ref of TIM_mstr

tim_ker_ck

TIM_mst_oc1ref
tim_mstr_CNT

FC

tim_slv_CNT

3045

FD

FE

3046

FF

3047

00

01

3048

tim_slv TIF bit

Write TIF = 0

MSv62376V1

In the example in Figure 676, the TIM_slv counter and prescaler are not initialized before
being started. So they start counting from their current value. It is possible to start from a
given value by resetting both timers before starting TIM_mstr. Then any value can be written
in the timer counters. The timers can easily be reset by software using the UG bit in the
TIMx_EGR registers.

RM0456 Rev 6

<!-- pagebreak -->

2350

General-purpose timers (TIM2/TIM3/TIM4/TIM5)

RM0456

In the next example (refer to Figure 677), we synchronize TIM_mstr and TIM_slv. TIM_mstr
is the master and starts from 0. TIM_slv is the slave and starts from 0xE7. The prescaler
ratio is the same for both timers. TIM_slv stops when TIM_mstr is disabled by writing 0 to
the CEN bit in the TIM_mstr_CR1 register:
1.

Configure TIM_mstr master mode to send its output compare 1 reference (tim_oc1ref)
signal as trigger output (MMS = 100 in the TIM_mstr_CR2 register).

2.

Configure the TIM_mstr tim_oc1ref waveform (TIM_mstr_CCMR1 register).

3.

Configure TIM_slv to get the input trigger from TIM_mstr (TS = 00010 in the
TIM_slv_SMCR register).

4.

Configure TIM_slv in gated mode (SMS = 101 in TIM_slv_SMCR register).

5.

Reset TIM_mstr by writing 1 in UG bit (TIM_mstr_EGR register).

6.

Reset TIM_slv by writing 1 in UG bit (TIM_slv_EGR register).

7.

Initialize TIM_slv to 0xE7 by writing 0xE7 in the TIM_slv counter (TIM_slv_CNT).

8.

Enable TIM_slv by writing 1 in the CEN bit (TIM_slv_CR1 register).

9.

Start TIM_mstr by writing 1 in the CEN bit (TIM_mstr_CR1 register).

10. Stop TIM_mstr by writing 0 in the CEN bit (TIM_mstr_CR1 register).
Figure 677. Gating TIM_slv with Enable of TIM_mstr
tim_ker_ck
TIM_mst counter enable (CEN bit)
tim_mstr_CNT reset
tim_mstr_CNT
tim_slv_CNT

75

00
AB

00

E7

01

02

E8

E9

tim_slv_CNT reset
tim_slv_CNT write
tim_slv TIF bit

Write TIF = 0

MSv62377V1

Using one timer to start another timer
In this example, we set the enable of TIM_slv with the update event of TIM_mstr. Refer to
Figure 674 for connections. TIM_slv starts counting from its current value (which can be
nonzero) on the divided internal clock as soon as the update event is generated by
TIM_mstr. When TIM_slv receives the trigger signal its CEN bit is automatically set and the
counter counts until we write 0 to the CEN bit in the TIM_slv_CR1 register. Both counter
clock frequencies are divided by 3 by the prescaler compared to tim_ker_ck (ftim_cnt_ck =
ftim_ker_ck/3).

<!-- pagebreak -->

RM0456 Rev 6

RM0456

General-purpose timers (TIM2/TIM3/TIM4/TIM5)
1.

Configure TIM_mstr master mode to send its update event (UEV) as trigger output
(MMS = 010 in the TIM_mstr_CR2 register).

2.

Configure the TIM_mstr period (TIM_mstr_ARR registers).

3.

Configure TIM_slv to get the input trigger from TIM_mstr (TS = 00010 in the
TIM_slv_SMCR register).

4.

Configure TIM_slv in trigger mode (SMS = 110 in TIM_slv_SMCR register).

5.

Start TIM_mstr by writing 1 in the CEN bit (TIM_mstr_CR1 register).
Figure 678. Triggering TIM_slv with update of TIM_mstr
tim_ker_ck
tim_mstr UEV
event
tim_mst_CNT

FD

FE

tim_slv_CNT

FF

00

45

01
46

02
47

48

TIM_slv counter enable (CEN bit)
tim_slv TIF bit

Write TIF = 0

MSv62378V1

As in the previous example, both counters can be initialized before starting counting.
Figure 679 shows the behavior with the same configuration as in Figure 678 but in trigger
mode (SMS = 110 in the TIM_slv_SMCR register) instead of gated mode.
Figure 679. Triggering TIM_slv with Enable of TIM_mstr
tim_ker_ck
TIM_mst counter enable (CEN bit)
tim_mstr_CNT reset
tim_mstr_CNT
tim_slv_CNT

75

00
CD

00

E7

01

02

E8

E9

EA

tim_slv_CNT reset
Tim_slv_CNT write
tim_slv TIF bit

Write TIF = 0

RM0456 Rev 6

MSv62379V1

<!-- pagebreak -->

2350

General-purpose timers (TIM2/TIM3/TIM4/TIM5)

RM0456

Starting two timers synchronously in response to an external trigger
In this example, we set the enable of TIM_mstr when its tim_ti1 input rises, and the enable
of TIM_slv with the enable of TIM_mstr. Refer to Figure 674 for connections. To ensure the
counters are aligned, TIM_mstr must be configured in Master/Slave mode (slave with
respect to tim_ti1, master with respect to TIM_slv):
1.

Configure TIM_mstr master mode to send its enable as trigger output (MMS = 001 in
the TIM_mstr_CR2 register).

2.

Configure TIM_mstr slave mode to get the input trigger from tim_ti1 (TS = 00100 in the
TIM_mstr_SMCR register).

3.

Configure TIM_mstr in trigger mode (SMS = 110 in the TIM_mstr_SMCR register).

4.

Configure the TIM_mstr in Master/Slave mode by writing MSM = 1 (TIM_mstr_SMCR
register).

5.

Configure TIM_slv to get the input trigger from TIM_mstr (TS = 00000 in the
TIM_slv_SMCR register).

6.

Configure TIM_slv in trigger mode (SMS = 110 in the TIM_slv_SMCR register).

When a rising edge occurs on tim_ti1 (TIM_mstr), both counters start counting
synchronously on the internal clock and both TIF flags are set.
Note:

In this example both timers are initialized before starting (by setting their respective UG
bits). Both counters starts from 0, but an offset can easily be inserted between them by
writing any of the counter registers (TIMx_CNT). One can see that the master/slave mode
inserts a delay between CNT_EN and CK_PSC on TIM_mstr.
Figure 680. Triggering TIM_mstr and TIM_slv with TIM_mstr tim_ti1 input
tim_ker_ck
tim_mstr_ti1
TIM_mst counter enable (CEN bit))
tim_mstr_psc_ck
tim_mstr_CNT

00

01 02 03 04 05 06 07 08 09

00

01 02 03 04 05 06 07 08 09

tim_mstr TIF bit
TIM_slv counter enable (CEN bit)
tim_slv_psc_ck
tim_slv_CNT
tim_slv TIF bit

MSv62380V1

Note:

The clock of the slave peripherals (such as timer, ADC) receiving the tim_trgo signal must
be enabled prior to receive events from the master timer, and the clock frequency
(prescaler) must not be changed on-the-fly while triggers are received from the master
timer.

55.4.24

ADC triggers
The timer can generate an ADC triggering event with various internal signals, such as reset,
enable or compare events.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

General-purpose timers (TIM2/TIM3/TIM4/TIM5)

Note:

The clock of the slave peripherals (such as timer, ADC) receiving the tim_trgo signal must
be enabled prior to receive events from the master timer, and the clock frequency
(prescaler) must not be changed on-the-fly while triggers are received from the master
timer.

55.4.25

DMA burst mode
The TIMx timers have the capability to generate multiple DMA requests upon a single event.
The main purpose is to be able to reprogram part of the timer multiple times without
software overhead, but it can also be used to read several registers in a row, at regular
intervals.
The DMA controller destination is unique and must point to the virtual register TIMx_DMAR.
On a given timer event, the timer launches a sequence of DMA requests (burst). Each write
into the TIMx_DMAR register is actually redirected to one of the timer registers.
The DBL[4:0] bits in the TIMx_DCR register set the DMA burst length. The timer recognizes
a burst transfer when a read or a write access is done to the TIMx_DMAR address), i.e. the
number of transfers (either in half-words or in bytes).
The DBA[4:0] bits in the TIMx_DCR registers define the DMA base address for DMA
transfers (when read/write accesses are done through the TIMx_DMAR address). DBA is
defined as an offset starting from the address of the TIMx_CR1 register:
Example:
00000: TIMx_CR1
00001: TIMx_CR2
00010: TIMx_SMCR
The DBSS[3:0] bits in the TIMx_DCR register defines the interrupt source that triggers the
DMA burst transfers (see Section 55.5.23: TIMx DMA control register (TIMx_DCR)(x = 2 to
5) for details).
As an example, the timer DMA burst feature is used to update the contents of the CCRx
registers (x = 2, 3, 4) upon an update event, with the DMA transferring half words into the
CCRx registers.
This is done in the following steps:
1.

Configure the corresponding DMA channel as follows:
–

DMA channel peripheral address is the DMAR register address.

–

DMA channel memory address is the address of the buffer in the RAM containing
the data to be transferred by DMA into CCRx registers.

–

Number of data to transfer = 3 (See note below).

–

Circular mode disabled.

2.

Configure the DCR register by configuring the DBA and DBL bitfields as follows:
DBL = 3 transfers, DBA = 0xE and DBSS = 1.

3.

Enable the TIMx update DMA request (set the UDE bit in the DIER register).

4.

Enable TIMx.

5.

Enable the DMA channel.

This example is for the case where every CCRx register has to be updated once. If every
CCRx register is to be updated twice for example, the number of data to transfer must be 6.
Let's take the example of a buffer in the RAM containing data1, data2, data3, data4, data5,
RM0456 Rev 6

<!-- pagebreak -->

2350

General-purpose timers (TIM2/TIM3/TIM4/TIM5)

RM0456

and data6. The data is transferred to the CCRx registers as follows: on the first update DMA
request, data1 is transferred to CCR2, data2 is transferred to CCR3, data3 is transferred to
CCR4 and on the second update DMA request, data4 is transferred to CCR2, data5 is
transferred to CCR3, and data6 is transferred to CCR4.
Note:

A null value can be written to the reserved registers.

55.4.26

TIM2/TIM3/TIM4/TIM5 DMA requests
The TIM2/TIM3/TIM4/TIM5 can generate a DMA requests, as shown in Table 571.
Table 571. DMA request
DMA request

Enable
control bit

tim_upd_dma

Update

UDE

tim_cc1_dma

Capture/compare 1

CC1DE

tim_cc2_dma

Capture/compare 2

CC2DE

tim_cc3_dma

Capture/compare 3

CC3DE

tim_cc4_dma

Capture/compare 4

CC4DE

tim_trgi_dma

Trigger

TDE

DMA request signal

Note:

Some timer's DMA requests may not be connected to the DMA controller. Refer to the DMA
section(s) for more details.

55.4.27

Debug mode
When the microcontroller enters debug mode (Cortex®-M33 core halted), the TIMx counter
can either continue to work normally or stops.
The behavior in debug mode can be programmed with a dedicated configuration bit per
timer in the Debug support (DBG) module.
For more details, refer to section Debug support (DBG).

55.4.28

TIM2/TIM3/TIM4/TIM5 low-power modes
Table 572. Effect of low-power modes on TIM2/TIM3/TIM4/TIM5
Mode

<!-- pagebreak -->

Description

Sleep

No effect, peripheral is active. The interrupts can cause the device to exit from Sleep
mode.

Stop

The timer operation is stopped and the register content is kept. No interrupt can be
generated.

Standby

The timer is powered-down and must be reinitialized after exiting the Standby mode.

RM0456 Rev 6

RM0456

55.4.29

General-purpose timers (TIM2/TIM3/TIM4/TIM5)

TIM2/TIM3/TIM4/TIM5 interrupts
The TIM2/TIM3/TIM4/TIM5 can generate multiple interrupts, as shown in Table 573.
Table 573. Interrupt requests
Event
flag

Enable
control bit

Interrupt clear
method

Exit
from
Sleep
mode

Exit from
Stop and
Standby
mode

Update

UIF

UIE

write 0 in UIF

Yes

No

Capture/compare 1

CC1IF

CC1IE

write 0 in CC1IF

Yes

No

Capture/compare 2

CC2IF

CC2IE

write 0 in CC2IF

Yes

No

Capture/compare 3

CC3IF

CC3IE

write 0 in CC3IF

Yes

No

Capture/compare 4

CC4IF

CC4IE

write 0 in CC4IF

Yes

No

TIM_TRG

Trigger

TIF

TIE

write 0 in TIF

Yes

No

TIM_DIR
_IDX

Index

IDXF

IDXIE

write 0 in IDXF

Yes

No

Direction

DIRF

DIRIE

write 0 in DIRF

Yes

No

TIM_IERR

Index Error

IERRF

IERRIE

write 0 in IERRF

Yes

No

TIM_TER

Transition Error

TERRF

TERRIE

write 0 in
TERRF

Yes

No

Interrupt
acronym
TIM_UP

TIM_CC

Interrupt event

RM0456 Rev 6

<!-- pagebreak -->

