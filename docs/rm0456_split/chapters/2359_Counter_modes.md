2474

Basic timers (TIM6/TIM7)

RM0456

Figure 731. Counter timing diagram, update event when ARPE = 1 (TIMx_ARR
preloaded)
tim_psc_ck

CEN
tim_cnt_ck
Counter register

F0

F1

F2

F3

F4

F5

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

Update interrupt flag
(UIF)
Auto-reload preload
register

F5

36

Auto-reload shadow
register

F5

Write a new value in TIMx_ARR

36

MSv62304V1

Dithering mode
The time base effective resolution can be increased by enabling the dithering mode, using
the DITHEN bit in the TIMx_CR1 register. This affects the way the TIMx_ARR is behaving,
and is useful for adjusting the average counter period when the timer is used as a trigger
(typically for a DAC).
The operating principle is to have the actual ARR value slightly changed (adding or not one
timer clock period) over 16 consecutive counting periods, with predefined patterns. This
allows a 16-fold resolution increase, considering the average counting period.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Basic timers (TIM6/TIM7)
Figure 732 presents the dithering principle applied to four consecutive counting periods.
Figure 732. Dithering principle

Average period

12

12

12

12

13

12

12

12

13

12

13

12

13

13

13

12

13

13

13

13

T = 12

T = 12+¼

T = 12+½

T = 12+¾

T = 13
MSv47466V1

When the dithering mode is enabled, the register coding is changed as follows (see
Figure 733 for example):

Note:

•

The four LSBs are coding for the enhanced resolution part (fractional part).

•

The MSBs are left-shifted to the bits 19:4 and are coding for the base value.

The following sequence must be followed when resetting the DITHEN bit:
1. CEN and ARPE bits must be reset
2. The ARR[3:0] bits must be reset
3. The DITHEN bit must be reset
4. The CEN bit can be set (eventually with ARPE = 1).
Figure 733. Data format and register coding in dithering mode
b19

Register format in
dithering mode

b0
MSB: 16-bits, integer part

LSB: 4-bits
fractional part

b19

b0
326

Example
20

Base compare value is 20 during 16 periods

6

Additional 6 cycles are spread over the
16 periods
MSv45753V2

RM0456 Rev 6

<!-- pagebreak -->

2474

Basic timers (TIM6/TIM7)

RM0456

The minimum frequency is given by the following formula:
F Tim
F Tim
Resolution = ------------- ⇒ F pwmMin = -----------------------------------F pwm
Max Resolution
F Tim
Dithering mode disabled: F pwmMin = --------------65536
F Tim
Dithering mode enabled: F pwmMin = ----------------------------65535 + 15
-----16

Note:

The maximum TIMx_ARR value is limited to 0xFFFEF in dithering mode (corresponds to
65534 for the integer part and 15 for the dithered part).
As shown on Figure 734, the dithering mode is used to increase the PWM resolution
whatever the PWM frequency.
Figure 734. FCnt resolution vs frequency
Resolution

20-bit

16-bit
Dithering

No Dithering

FCnt

FCnt
min

MSv50910V1

The period changes are spread over 16 consecutive periods, as described in Figure 735.
Figure 735. PWM dithering pattern

Counter period

1

2

3

4

5

6

7

8

ARR value
Auto-Reload
value

9

10

11

12

13

14

15

41

40

40

40

40

40

40

16

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
MSv47465V1

The autoreload and compare values increments are spread following the specific patterns
described in Table 594. The dithering sequence is done to have increments distributed as
evenly as possible and minimize the overall ripple.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Basic timers (TIM6/TIM7)
Table 594. TIMx_ARR register change dithering pattern
-

57.3.6

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

UIF bit remapping
The IUFREMAP bit in the TIMx_CR1 register forces a continuous copy of the update
interrupt flag UIF into the timer counter register’s bit 31 (TIMxCNT[31]). This is used to
atomically read both the counter value and a potential roll-over condition signaled by the
UIFCPY flag. In particular cases, it can ease the calculations by avoiding race conditions
caused for instance by a processing shared between a background task (counter reading)
and an interrupt (update interrupt).
There is no latency between the assertions of the UIF and UIFCPY flags.

RM0456 Rev 6

<!-- pagebreak -->

