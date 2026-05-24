2236

Advanced-control timers (TIM1/TIM8)

RM0456

Bit 2 CC2IF: Capture/compare 2 interrupt flag
Refer to CC1IF description
Bit 1 CC1IF: Capture/compare 1 interrupt flag
This flag is set by hardware. It is cleared by software (input capture or output compare
mode) or by reading the TIMx_CCR1 register (input capture mode only).
0: No compare match / No input capture occurred
1: A compare match or an input capture occurred
If channel CC1 is configured as output: this flag is set when the content of the counter
TIMx_CNT matches the content of the TIMx_CCR1 register. When the content of
TIMx_CCR1 is greater than the content of TIMx_ARR, the CC1IF bit goes high on the
counter overflow (in up-counting and up/down-counting modes) or underflow (in
downcounting mode). There are 3 possible options for flag setting in center-aligned mode,
refer to the CMS bits in the TIMx_CR1 register for the full description.
If channel CC1 is configured as input: this bit is set when counter value has been
captured in TIMx_CCR1 register (an edge has been detected on IC1, as per the edge
sensitivity defined with the CC1P and CC1NP bits setting, in TIMx_CCER).
Bit 0 UIF: Update interrupt flag
This bit is set by hardware on an update event. It is cleared by software.
0: No update occurred.
1: Update interrupt pending. This bit is set by hardware when the registers are updated:
–At overflow or underflow regarding the repetition counter value (update if repetition counter
= 0) and if the UDIS = 0 in the TIMx_CR1 register.
–When CNT is reinitialized by software using the UG bit in TIMx_EGR register, if URS = 0
and UDIS = 0 in the TIMx_CR1 register.
–When CNT is reinitialized by a trigger event (refer to Section 54.6.3: TIMx slave mode
control register (TIMx_SMCR)(x = 1, 8)), if URS = 0 and UDIS = 0 in the TIMx_CR1
register.

54.6.6

TIMx event generation register (TIMx_EGR)(x = 1, 8)
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

B2G

BG

TG

COMG

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

w

w

w

Bits 15:9 Reserved, must be kept at reset value.
Bit 8 B2G: Break 2 generation
This bit is set by software in order to generate an event, it is automatically cleared by
hardware.
0: No action
1: A break 2 event is generated. MOE bit is cleared and B2IF flag is set. Related interrupt
can occur if enabled.
Bit 7 BG: Break generation
This bit is set by software in order to generate an event, it is automatically cleared by
hardware.
0: No action
1: A break event is generated. MOE bit is cleared and BIF flag is set. Related interrupt or
DMA transfer can occur if enabled.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Advanced-control timers (TIM1/TIM8)

Bit 6 TG: Trigger generation
This bit is set by software in order to generate an event, it is automatically cleared by
hardware.
0: No action
1: The TIF flag is set in TIMx_SR register. Related interrupt or DMA transfer can occur if
enabled.
Bit 5 COMG: Capture/compare control update generation
This bit can be set by software, it is automatically cleared by hardware
0: No action
1: CCxE, CCxNE and OCxM bits update (providing CCPC bit is set)
Note: This bit acts only on channels having a complementary output.
Bit 4 CC4G: Capture/compare 4 generation
Refer to CC1G description
Bit 3 CC3G: Capture/compare 3 generation
Refer to CC1G description
Bit 2 CC2G: Capture/compare 2 generation
Refer to CC1G description
Bit 1 CC1G: Capture/compare 1 generation
This bit is set by software in order to generate an event, it is automatically cleared by
hardware.
0: No action
1: A capture/compare event is generated on channel 1:
If channel CC1 is configured as output:
CC1IF flag is set, Corresponding interrupt or DMA request is sent if enabled.
If channel CC1 is configured as input:
The current value of the counter is captured in TIMx_CCR1 register. The CC1IF flag is set,
the corresponding interrupt or DMA request is sent if enabled. The CC1OF flag is set if the
CC1IF flag was already high.
Bit 0 UG: Update generation
This bit can be set by software, it is automatically cleared by hardware.
0: No action
1: Reinitialize the counter and generates an update of the registers. Note that the prescaler
counter is cleared too (anyway the prescaler ratio is not affected). The counter is cleared if
the center-aligned mode is selected or if DIR = 0 (upcounting), else it takes the autoreload
value (TIMx_ARR) if DIR = 1 (downcounting).

54.6.7

TIMx capture/compare mode register 1 (TIMx_CCMR1)
(x = 1, 8)
Address offset: 0x018
Reset value: 0x0000 0000
The same register can be used for input capture mode (this section) or for output compare
mode (next section). The direction of a channel is defined by configuring the corresponding
CCxS bits. All the other bits of this register have a different function for input capture and for
output compare modes. It is possible to combine both modes independently (for example
channel 1 in input capture mode and channel 2 in output compare mode).

RM0456 Rev 6

<!-- pagebreak -->

2236

Advanced-control timers (TIM1/TIM8)

RM0456

31

30

29

28

27

26

25

24

23

22

21

20

19

18

17

16

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

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

IC2F[3:0]
rw

rw

rw

rw

IC2PSC[1:0]

CC2S[1:0]

rw

rw

rw

rw

IC1F[3:0]
rw

rw

rw

rw

IC1PSC[1:0]

CC1S[1:0]

rw

rw

rw

rw

Input capture mode:
Bits 31:16 Reserved, must be kept at reset value.
Bits 15:12 IC2F[3:0]: Input capture 2 filter
Bits 11:10 IC2PSC[1:0]: Input capture 2 prescaler
Bits 9:8 CC2S[1:0]: Capture/compare 2 selection
This bitfield defines the direction of the channel (input/output) as well as the used input.
00: CC2 channel is configured as output
01: CC2 channel is configured as input, tim_ic2 is mapped on tim_ti2
10: CC2 channel is configured as input, tim_ic2 is mapped on tim_ti1
11: CC2 channel is configured as input, tim_ic2 is mapped on tim_trc. This mode is working only if an
internal trigger input is selected through TS bit (TIMx_SMCR register)
Note: CC2S bits are writable only when the channel is OFF (CC2E = 0 in TIMx_CCER).

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Advanced-control timers (TIM1/TIM8)

Bits 7:4 IC1F[3:0]: Input capture 1 filter
This bitfield defines the frequency used to sample tim_ti1 input and the length of the digital filter
applied to tim_ti1. The digital filter is made of an event counter in which N consecutive events are
needed to validate a transition on the output:
0000: No filter, sampling is done at fDTS
0001: fSAMPLING = ftim_ker_ck, N = 2
0010: fSAMPLING = ftim_ker_ck, N = 4
0011: fSAMPLING = ftim_ker_ck, N = 8
0100: fSAMPLING = fDTS/2, N = 6
0101: fSAMPLING = fDTS/2, N = 8
0110: fSAMPLING = fDTS/4, N = 6
0111: fSAMPLING = fDTS/4, N = 8
1000: fSAMPLING = fDTS/8, N = 6
1001: fSAMPLING = fDTS/8, N = 8
1010: fSAMPLING = fDTS/16, N = 5
1011: fSAMPLING = fDTS/16, N = 6
1100: fSAMPLING = fDTS/16, N = 8
1101: fSAMPLING = fDTS/32, N = 5
1110: fSAMPLING = fDTS/32, N = 6
1111: fSAMPLING = fDTS/32, N = 8
Bits 3:2 IC1PSC[1:0]: Input capture 1 prescaler
This bitfield defines the ratio of the prescaler acting on CC1 input (tim_ic1). The prescaler is reset as
soon as CC1E = 0 (TIMx_CCER register).
00: no prescaler, capture is done each time an edge is detected on the capture input
01: capture is done once every 2 events
10: capture is done once every 4 events
11: capture is done once every 8 events
Bits 1:0 CC1S[1:0]: Capture/compare 1 Selection
This bitfield defines the direction of the channel (input/output) as well as the used input.
00: CC1 channel is configured as output
01: CC1 channel is configured as input, tim_ic1 is mapped on tim_ti1
10: CC1 channel is configured as input, tim_ic1 is mapped on tim_ti2
11: CC1 channel is configured as input, tim_ic1 is mapped on tim_trc. This mode is working only if an
internal trigger input is selected through TS bit (TIMx_SMCR register)
Note: CC1S bits are writable only when the channel is OFF (CC1E = 0 in TIMx_CCER).

54.6.8

TIMx capture/compare mode register 1 [alternate]
(TIMx_CCMR1)(x = 1, 8)
Address offset: 0x018
Reset value: 0x0000 0000
The same register can be used for output compare mode (this section) or for input capture
mode (previous section). The direction of a channel is defined by configuring the
corresponding CCxS bits. All the other bits of this register have a different function for input
capture and for output compare modes. It is possible to combine both modes independently
(for example channel 1 in input capture mode and channel 2 in output compare mode).

RM0456 Rev 6

<!-- pagebreak -->

2236

Advanced-control timers (TIM1/TIM8)

RM0456

31

30

29

28

27

26

25

24

23

22

21

20

19

18

17

16

Res.

Res.

Res.

Res.

Res.

Res.

Res.

OC2M[3]

Res.

Res.

Res.

Res.

Res.

Res.

Res.

OC1M[3]

7

6

5

4

3

2

1

OC1
PE

OC1
FE

CC1S[1:0]

rw

rw

rw

rw
15

14

OC2
CE
rw

13

12

OC2M[2:0]
rw

rw

rw

9

rw

11

10

8

OC2
PE

OC2
FE

CC2S[1:0]

OC1
CE

rw

rw

rw

rw

rw

OC1M[2:0]
rw

rw

rw

0

rw

Output compare mode:
Bits 31:25 Reserved, must be kept at reset value.
Bits 23:17 Reserved, must be kept at reset value.
Bit 15 OC2CE: Output compare 2 clear enable
Bits 24, 14:12 OC2M[3:0]: Output compare 2 mode
Bit 11 OC2PE: Output compare 2 preload enable
Bit 10 OC2FE: Output compare 2 fast enable
Bits 9:8 CC2S[1:0]: Capture/compare 2 selection
This bitfield defines the direction of the channel (input/output) as well as the used input.
00: CC2 channel is configured as output
01: CC2 channel is configured as input, tim_ic2 is mapped on tim_ti2
10: CC2 channel is configured as input, tim_ic2 is mapped on tim_ti1
11: CC2 channel is configured as input, tim_ic2 is mapped on tim_trc. This mode is working
only if an internal trigger input is selected through the TS bit (TIMx_SMCR register)
Note: CC2S bits are writable only when the channel is OFF (CC2E = 0 in TIMx_CCER).
Bit 7 OC1CE: Output compare 1 clear enable
0: tim_oc1ref is not affected by the tim_ocref_clr_int signal
1: tim_oc1ref is cleared as soon as a High level is detected on tim_ocref_clr_int signal
(tim_ocref_clr input or tim_etrf input)

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Advanced-control timers (TIM1/TIM8)

Bits 16, 6:4 OC1M[3:0]: Output compare 1 mode
These bits define the behavior of the output reference signal tim_oc1ref from which tim_oc1
and tim_oc1n are derived. tim_oc1ref is active high whereas tim_oc1 and tim_oc1n active
level depends on CC1P and CC1NP bits.
0000: Frozen - The comparison between the output compare register TIMx_CCR1 and the
counter TIMx_CNT has no effect on the outputs.This mode can be used when the
timer serves as a software timebase. When the frozen mode is enabled during timer
operation, the ouput keeps the state (active or inactive) it had before entering the
frozen state.
0001: Set channel 1 to active level on match. tim_oc1ref signal is forced high when the
counter TIMx_CNT matches the capture/compare register 1 (TIMx_CCR1).
0010: Set channel 1 to inactive level on match. tim_oc1ref signal is forced low when the
counter TIMx_CNT matches the capture/compare register 1 (TIMx_CCR1).
0011: Toggle - tim_oc1ref toggles when TIMx_CNT = TIMx_CCR1.
0100: Force inactive level - tim_oc1ref is forced low.
0101: Force active level - tim_oc1ref is forced high.
0110: PWM mode 1 - In upcounting, channel 1 is active as long as TIMx_CNT<TIMx_CCR1
else inactive. In downcounting, channel 1 is inactive (tim_oc1ref = 0) as long as
TIMx_CNT>TIMx_CCR1 else active (tim_oc1ref = 1).
0111: PWM mode 2 - In upcounting, channel 1 is inactive as long as
TIMx_CNT<TIMx_CCR1 else active. In downcounting, channel 1 is active as long as
TIMx_CNT>TIMx_CCR1 else inactive.
1000: Retrigerrable OPM mode 1 - In up-counting mode, the channel is active until a trigger
event is detected (on tim_trgi signal). Then, a comparison is performed as in PWM
mode 1 and the channels becomes active again at the next update. In down-counting
mode, the channel is inactive until a trigger event is detected (on tim_trgi signal).
Then, a comparison is performed as in PWM mode 1 and the channels becomes
inactive again at the next update.
1001: Retrigerrable OPM mode 2 - In up-counting mode, the channel is inactive until a
trigger event is detected (on tim_trgi signal). Then, a comparison is performed as in
PWM mode 2 and the channels becomes inactive again at the next update. In downcounting mode, the channel is active until a trigger event is detected (on tim_trgi
signal). Then, a comparison is performed as in PWM mode 1 and the channels
becomes active again at the next update.
1010: Reserved,
1011: Reserved,
1100: Combined PWM mode 1 - tim_oc1ref has the same behavior as in PWM mode 1.
tim_oc1refc is the logical OR between tim_oc1ref and tim_oc2ref.
1101: Combined PWM mode 2 - tim_oc1ref has the same behavior as in PWM mode 2.
tim_oc1refc is the logical AND between tim_oc1ref and tim_oc2ref.
1110: Asymmetric PWM mode 1 - tim_oc1ref has the same behavior as in PWM mode 1.
tim_oc1refc outputs tim_oc1ref when the counter is counting up, tim_oc2ref when it is
counting down.
1111: Asymmetric PWM mode 2 - tim_oc1ref has the same behavior as in PWM mode 2.
tim_oc1refc outputs tim_oc1ref when the counter is counting up, tim_oc2ref when it is
counting down.
Note: These bits can not be modified as long as LOCK level 3 has been programmed (LOCK
bits in TIMx_BDTR register) and CC1S = 00 (the channel is configured in output).
Note: In PWM mode, the OCREF level changes when the result of the comparison changes,
when the output compare mode switches from “frozen” mode to “PWM” mode and
when the output compare mode switches from “force active/inactive” mode to “PWM”
mode.
Note: On channels having a complementary output, this bitfield is preloaded. If the CCPC bit
is set in the TIMx_CR2 register then the OC1M active bits take the new value from the
preloaded bits only when a COM event is generated.
RM0456 Rev 6

<!-- pagebreak -->

2236

Advanced-control timers (TIM1/TIM8)

RM0456

Bit 3 OC1PE: Output compare 1 preload enable
0: Preload register on TIMx_CCR1 disabled. TIMx_CCR1 can be written at anytime, the new
value is taken in account immediately.
1: Preload register on TIMx_CCR1 enabled. Read/Write operations access the preload
register. TIMx_CCR1 preload value is loaded in the active register at each update event.
Note: These bits can not be modified as long as LOCK level 3 has been programmed (LOCK
bits in TIMx_BDTR register) and CC1S = 00 (the channel is configured in output).
Bit 2 OC1FE: Output compare 1 fast enable
This bit decreases the latency between a trigger event and a transition on the timer output.
It must be used in one-pulse mode (OPM bit set in TIMx_CR1 register), to have the output
pulse starting as soon as possible after the starting trigger.
0: CC1 behaves normally depending on counter and CCR1 values even when the trigger is
ON. The minimum delay to activate CC1 output when an edge occurs on the trigger input
is 5 clock cycles.
1: An active edge on the trigger input acts like a compare match on CC1 output. Then, OC is
set to the compare level independently from the result of the comparison. Delay to sample
the trigger input and to activate CC1 output is reduced to 3 clock cycles. OCFE acts only
if the channel is configured in PWM1 or PWM2 mode.
Bits 1:0 CC1S[1:0]: Capture/compare 1 selection
This bitfield defines the direction of the channel (input/output) as well as the used input.
00: CC1 channel is configured as output
01: CC1 channel is configured as input, tim_ic1 is mapped on tim_ti1
10: CC1 channel is configured as input, tim_ic1 is mapped on tim_ti2
11: CC1 channel is configured as input, tim_ic1 is mapped on tim_trc. This mode is working
only if an internal trigger input is selected through TS bit (TIMx_SMCR register)
Note: CC1S bits are writable only when the channel is OFF (CC1E = 0 in TIMx_CCER).

54.6.9

TIMx capture/compare mode register 2 (TIMx_CCMR2)
(x = 1, 8)
Address offset: 0x01C
Reset value: 0x0000 0000
The same register can be used for input capture mode (this section) or for output compare
mode (next section). The direction of a channel is defined by configuring the corresponding
CCxS bits. All the other bits of this register have a different function for input capture and for
output compare modes. It is possible to combine both modes independently (for example
channel 3 in input capture mode and channel 4 in output compare mode).

31

30

29

28

27

26

25

24

23

22

21

20

19

18

17

16

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

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

rw

rw

rw

rw

IC4F[3:0]
rw

rw

IC4PSC[1:0]

CC4S[1:0]

rw

rw

rw

rw

IC3F[3:0]

Input capture mode

<!-- pagebreak -->

RM0456 Rev 6

rw

rw

IC3PSC[1:0]

CC3S[1:0]

rw

rw

rw

rw

RM0456

Advanced-control timers (TIM1/TIM8)

Bits 31:16 Reserved, must be kept at reset value.
Bits 15:12 IC4F[3:0]: Input capture 4 filter
Bits 11:10 IC4PSC[1:0]: Input capture 4 prescaler
Bits 9:8 CC4S[1:0]: Capture/compare 4 selection
This bitfield defines the direction of the channel (input/output) as well as the used input.
00: CC4 channel is configured as output
01: CC4 channel is configured as input, tim_ic4 is mapped on tim_ti4
10: CC4 channel is configured as input, tim_ic4 is mapped on tim_ti3
11: CC4 channel is configured as input, tim_ic4 is mapped on tim_trc. This mode is working
only if an internal trigger input is selected through TS bit (TIMx_SMCR register)
Note: CC4S bits are writable only when the channel is OFF (CC4E = 0 in TIMx_CCER).
Bits 7:4 IC3F[3:0]: Input capture 3 filter
Bits 3:2 IC3PSC[1:0]: Input capture 3 prescaler
Bits 1:0 CC3S[1:0]: Capture/compare 3 selection
This bitfield defines the direction of the channel (input/output) as well as the used input.
00: CC3 channel is configured as output
01: CC3 channel is configured as input, tim_ic3 is mapped on tim_ti3
10: CC3 channel is configured as input, tim_ic3 is mapped on tim_ti4
11: CC3 channel is configured as input, tim_ic3 is mapped on tim_trc. This mode is working
only if an internal trigger input is selected through TS bit (TIMx_SMCR register)
Note: CC3S bits are writable only when the channel is OFF (CC3E = 0 in TIMx_CCER).

54.6.10

TIMx capture/compare mode register 2 [alternate]
(TIMx_CCMR2)(x = 1, 8)
Address offset: 0x01C
Reset value: 0x0000 0000
The same register can be used for output compare mode (this section) or for input capture
mode (previous section). The direction of a channel is defined by configuring the
corresponding CCxS bits. All the other bits of this register have a different function for input
capture and for output compare modes. It is possible to combine both modes independently
(for example channel 3 in input capture mode and channel 4 in output compare mode).

31

30

29

28

27

26

25

24

23

22

21

20

19

18

17

16

Res.

Res.

Res.

Res.

Res.

Res.

Res.

OC4M[3]

Res.

Res.

Res.

Res.

Res.

Res.

Res.

OC3M[3]

7

6

5

4

3

2

1

OC3
PE

OC3
FE

CC3S[1:0]

rw

rw

rw

rw
15

14

OC4
CE
rw

13

12

OC4M[2:0]
rw

rw

rw

9

rw

11

10

8

OC4
PE

OC4
FE

CC4S[1:0]

OC3
CE

rw

rw

rw

rw

rw

OC3M[2:0]
rw

rw

rw

0

rw

Output compare mode
Bits 31:25 Reserved, must be kept at reset value.
Bits 23:17 Reserved, must be kept at reset value.
Bit 15 OC4CE: Output compare 4 clear enable

RM0456 Rev 6

<!-- pagebreak -->

2236

Advanced-control timers (TIM1/TIM8)

RM0456

Bits 24, 14:12 OC4M[3:0]: Output compare 4 mode
Refer to OC3M[3:0] bit description
Bit 11 OC4PE: Output compare 4 preload enable
Bit 10 OC4FE: Output compare 4 fast enable
Bits 9:8 CC4S[1:0]: Capture/compare 4 selection
This bitfield defines the direction of the channel (input/output) as well as the used input.
00: CC4 channel is configured as output
01: CC4 channel is configured as input, tim_ic4 is mapped on tim_ti4
10: CC4 channel is configured as input, tim_ic4 is mapped on tim_ti3
11: CC4 channel is configured as input, tim_ic4 is mapped on tim_trc. This mode is working
only if an internal trigger input is selected through TS bit (TIMx_SMCR register)
Note: CC4S bits are writable only when the channel is OFF (CC4E = 0 in TIMx_CCER).
Bit 7 OC3CE: Output compare 3 clear enable

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Advanced-control timers (TIM1/TIM8)

Bits 16, 6:4 OC3M[3:0]: Output compare 3 mode
These bits define the behavior of the output reference signal tim_oc3ref from which tim_oc3
and tim_oc3n are derived. tim_oc3ref is active high whereas tim_oc3 and tim_oc3n active
level depends on CC3P and CC3NP bits.
0000: Frozen - The comparison between the output compare register TIMx_CCR3 and the
counter TIMx_CNT has no effect on the outputs.(this mode is used to generate a
timing base).
0001: Set channel 3 to active level on match. tim_oc3ref signal is forced high when the
counter TIMx_CNT matches the capture/compare register 3 (TIMx_CCR3).
0010: Set channel 3 to inactive level on match. tim_oc3ref signal is forced low when the
counter TIMx_CNT matches the capture/compare register 3 (TIMx_CCR3).
0011: Toggle - tim_oc3ref toggles when TIMx_CNT = TIMx_CCR3.
0100: Force inactive level - tim_oc3ref is forced low.
0101: Force active level - tim_oc3ref is forced high.
0110: PWM mode 1 - In upcounting, channel 3 is active as long as TIMx_CNT<TIMx_CCR3
else inactive. In downcounting, channel 3 is inactive (tim_oc3ref = 0) as long as
TIMx_CNT>TIMx_CCR3 else active (tim_oc3ref = 1).
0111: PWM mode 2 - In upcounting, channel 3 is inactive as long as
TIMx_CNT<TIMx_CCR3 else active. In downcounting, channel 3 is active as long as
TIMx_CNT>TIMx_CCR3 else inactive.
1000: Retrigerrable OPM mode 1 - In up-counting mode, the channel is active until a trigger
event is detected (on tim_trgi signal). Then, a comparison is performed as in PWM
mode 1 and the channels becomes active again at the next update. In down-counting
mode, the channel is inactive until a trigger event is detected (on tim_trgi signal).
Then, a comparison is performed as in PWM mode 1 and the channels becomes
inactive again at the next update.
1001: Retrigerrable OPM mode 2 - In up-counting mode, the channel is inactive until a
trigger event is detected (on tim_trgi signal). Then, a comparison is performed as in
PWM mode 2 and the channels becomes inactive again at the next update. In downcounting mode, the channel is active until a trigger event is detected (on tim_trgi
signal). Then, a comparison is performed as in PWM mode 1 and the channels
becomes active again at the next update.
1010: Pulse on compare: a pulse is generated on tim_oc3ref upon CCR3 match event, as
per PWPRSC[2:0] and PW[7:0] bitfields programming in TIMxECR.
1011: Direction output. The tim_oc3ref signal is overridden by a copy of the DIR bit.
1100: Combined PWM mode 1 - tim_oc3ref has the same behavior as in PWM mode 1.
tim_oc3refc is the logical OR between tim_oc3ref and tim_oc4ref.
1101: Combined PWM mode 2 - tim_oc3ref has the same behavior as in PWM mode 2.
tim_oc3refc is the logical AND between tim_oc3ref and tim_oc4ref.
1110: Asymmetric PWM mode 1 - tim_oc3ref has the same behavior as in PWM mode 1.
tim_oc3refc outputs tim_oc3ref when the counter is counting up, tim_oc4ref when it is
counting down.
1111: Asymmetric PWM mode 2 - tim_oc3ref has the same behavior as in PWM mode 2.
tim_oc3refc outputs tim_oc3ref when the counter is counting up, tim_oc4ref when it is
counting down.
Note: These bits can not be modified as long as LOCK level 3 has been programmed (LOCK
bits in TIMx_BDTR register) and CC1S = 00 (the channel is configured in output).
Note: In PWM mode, the OCREF level changes only when the result of the comparison
changes or when the output compare mode switches from “frozen” mode to “PWM”
mode.
On channels having a complementary output, this bitfield is preloaded. If the CCPC bit is set in
the TIMx_CR2 register then the OC3M active bits take the new value from the preloaded bits
only when a COM event is generated.

RM0456 Rev 6

<!-- pagebreak -->

2236

Advanced-control timers (TIM1/TIM8)

RM0456

Bit 3 OC3PE: Output compare 3 preload enable
Bit 2 OC3FE: Output compare 3 fast enable
Bits 1:0 CC3S[1:0]: Capture/compare 3 selection
This bitfield defines the direction of the channel (input/output) as well as the used input.
00: CC3 channel is configured as output
01: CC3 channel is configured as input, tim_ic3 is mapped on tim_ti3
10: CC3 channel is configured as input, tim_ic3 is mapped on tim_ti4
11: CC3 channel is configured as input, tim_ic3 is mapped on tim_trc. This mode is working
only if an internal trigger input is selected through TS bit (TIMx_SMCR register)
Note: CC3S bits are writable only when the channel is OFF (CC3E = 0 in TIMx_CCER).

54.6.11

TIMx capture/compare enable register (TIMx_CCER)(x = 1, 8)
Address offset: 0x020
Reset value: 0x0000 0000

31

30

29

28

27

26

25

24

23

22

21

20

19

18

17

16

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

CC6P

CC6E

Res.

Res.

CC5P

CC5E

rw

rw

rw

rw

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

CC4P

CC4E

CC3P

CC3E

CC2P

CC2E

CC1P

CC1E

rw

rw

rw

rw

rw

rw

rw

rw

CC4NP CC4NE
rw

rw

CC3NP CC3NE
rw

rw

CC2NP CC2NE
rw

rw

Bits 31:22 Reserved, must be kept at reset value.
Bit 21 CC6P: Capture/compare 6 output polarity
Refer to CC1P description
Bit 20 CC6E: Capture/compare 6 output enable
Refer to CC1E description
Bits 19:18 Reserved, must be kept at reset value.
Bit 17 CC5P: Capture/compare 5 output polarity
Refer to CC1P description
Bit 16 CC5E: Capture/compare 5 output enable
Refer to CC1E description
Bit 15 CC4NP: Capture/compare 4 complementary output polarity
Refer to CC1NP description
Bit 14 CC4NE: Capture/compare 4 complementary output enable
Refer to CC1NE description
Bit 13 CC4P: Capture/compare 4 output polarity
Refer to CC1P description
Bit 12 CC4E: Capture/compare 4 output enable
Refer to CC1E description
Bit 11 CC3NP: Capture/compare 3 complementary output polarity
Refer to CC1NP description

<!-- pagebreak -->

RM0456 Rev 6

CC1NP CC1NE
rw

rw

RM0456

Advanced-control timers (TIM1/TIM8)

Bit 10 CC3NE: Capture/compare 3 complementary output enable
Refer to CC1NE description
Bit 9 CC3P: Capture/compare 3 output polarity
Refer to CC1P description
Bit 8 CC3E: Capture/compare 3 output enable
Refer to CC1E description
Bit 7 CC2NP: Capture/compare 2 complementary output polarity
Refer to CC1NP description
Bit 6 CC2NE: Capture/compare 2 complementary output enable
Refer to CC1NE description
Bit 5 CC2P: Capture/compare 2 output polarity
Refer to CC1P description
Bit 4 CC2E: Capture/compare 2 output enable
Refer to CC1E description
Bit 3 CC1NP: Capture/compare 1 complementary output polarity
CC1 channel configured as output:
0: tim_oc1n active high.
1: tim_oc1n active low.
CC1 channel configured as input:
This bit is used in conjunction with CC1P to define the polarity of tim_ti1fp1 and tim_ti2fp1.
Refer to CC1P description.
Note: This bit is not writable as soon as LOCK level 2 or 3 has been programmed (LOCK bits
in TIMx_BDTR register) and CC1S = 00 (channel configured as output).
Note: On channels having a complementary output, this bit is preloaded. If the CCPC bit is
set in the TIMx_CR2 register then the CC1NP active bit takes the new value from the
preloaded bit only when a Commutation event is generated.
Bit 2 CC1NE: Capture/compare 1 complementary output enable
0: Off - tim_oc1n is not active. tim_oc1n level is then function of MOE, OSSI, OSSR, OIS1,
OIS1N and CC1E bits.
1: On - tim_oc1n signal is output on the corresponding output pin depending on MOE, OSSI,
OSSR, OIS1, OIS1N and CC1E bits.
Note: On channels having a complementary output, this bit is preloaded. If the CCPC bit is
set in the TIMx_CR2 register then the CC1NE active bit takes the new value from the
preloaded bit only when a Commutation event is generated.

RM0456 Rev 6

<!-- pagebreak -->

2236

Advanced-control timers (TIM1/TIM8)

RM0456

Bit 1 CC1P: Capture/compare 1 output polarity
0: OC1 active high (output mode) / Edge sensitivity selection (input mode, see below)
1: OC1 active low (output mode) / Edge sensitivity selection (input mode, see below)
When CC1 channel is configured as input, both CC1NP/CC1P bits select the active polarity
of TI1FP1 and TI2FP1 for trigger or capture operations.
CC1NP = 0, CC1P = 0:non-inverted/rising edge. The circuit is sensitive to TIxFP1 rising
edge (capture or trigger operations in reset, external clock or trigger
mode), TIxFP1 is not inverted (trigger operation in gated mode or
encoder mode).
CC1NP = 0, CC1P = 1:inverted/falling edge. The circuit is sensitive to TIxFP1 falling edge
(capture or trigger operations in reset, external clock or trigger
mode), TIxFP1 is inverted (trigger operation in gated mode or
encoder mode).
CC1NP = 1, CC1P = 1: non-inverted/both edges/ The circuit is sensitive to both TIxFP1
rising and falling edges (capture or trigger operations in reset,
external clock or trigger mode), TIxFP1is not inverted (trigger
operation in gated mode). This configuration must not be used in
encoder mode.
CC1NP = 1, CC1P = 0:the configuration is reserved, it must not be used.
Note: This bit is not writable as soon as LOCK level 2 or 3 has been programmed (LOCK bits
in TIMx_BDTR register).
Note: On channels having a complementary output, this bit is preloaded. If the CCPC bit is
set in the TIMx_CR2 register then the CC1P active bit takes the new value from the
preloaded bit only when a Commutation event is generated.
Bit 0 CC1E: Capture/compare 1 output enable
0: Capture mode disabled / OC1 is not active (see below)
1: Capture mode enabled / OC1 signal is output on the corresponding output pin
When CC1 channel is configured as output, the OC1 level depends on MOE, OSSI,
OSSR, OIS1, OIS1N and CC1NE bits, regardless of the CC1E bits state. Refer to Table 554
for details.
Note: On channels having a complementary output, this bit is preloaded. If the CCPC bit is
set in the TIMx_CR2 register then the CC1E active bit takes the new value from the
preloaded bit only when a Commutation event is generated.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Advanced-control timers (TIM1/TIM8)

Table 554. Output control bits for complementary tim_ocx and tim_ocxn channels
with break feature
Output states(1)

Control bits
MOE bit OSSI bit OSSR bit CCxE bit CCxNE bit

1

0

0

Output disabled (not driven by the timer: Hi-Z)
tim_ocx = 0, tim_ocxn = 0

0

0

1

Output disabled (not driven
tim_ocxref + Polarity tim_ocxn
by the timer: Hi-Z)
= tim_ocxref xor CCxNP
tim_ocx = 0

0

1

0

tim_ocxref + Polarity
tim_ocx = tim_ocxref xor
CCxP

Output Disabled (not driven by
the timer: Hi-Z)
tim_ocxn = 0

X

1

1

OCREF + Polarity + deadtime

Complementary to OCREF (not
OCREF) + Polarity + dead-time

1

0

1

Off-State (output enabled
with inactive state)
tim_ocx = CCxP

tim_ocxref + Polarity
tim_ocxn = tim_ocxref x or
CCxNP

1

1

0

tim_ocxref + Polarity
tim_ocx = tim_ocxref xor
CCxP

Off-State (output enabled with
inactive state)
tim_ocxn = CCxNP

X

X

0

0

0

1

1

0

1

1

X

1

tim_ocxn output state

X

0

0

tim_ocx output state

X

Output disabled (not driven by the timer: Hi-Z).
Off-State (output enabled with inactive state)
Asynchronously: tim_ocx = CCxP, tim_ocxn = CCxNP (if
tim_brk or tim_brk2 is triggered).
Then (this is valid only if tim_brk is triggered), if the clock is
present: tim_ocx = OISx and tim_ocxn = OISxN after a deadtime, assuming that OISx and OISxN do not correspond to
OCX and tim_ocxn both in active state (may cause a short
circuit when driving switches in half-bridge configuration).
Note: tim_brk2 can only be used if OSSI = OSSR = 1.

1. When both outputs of a channel are not used (control taken over by GPIO), the OISx, OISxN, CCxP and CCxNP bits must
be kept cleared.

Note:

The state of the external I/O pins connected to the complementary tim_ocx and tim_ocxn
channels depends on the tim_ocx and tim_ocxn channel state and the GPIO registers.

RM0456 Rev 6

<!-- pagebreak -->

2236

Advanced-control timers (TIM1/TIM8)

54.6.12

RM0456

TIMx counter (TIMx_CNT)(x = 1, 8)
Address offset: 0x024
Reset value: 0x0000 0000

31

30

29

28

27

26

25

24

23

22

21

20

19

18

17

16

UIF
CPY

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

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

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

r

CNT[15:0]
rw

Bit 31 UIFCPY: UIF copy
This bit is a read-only copy of the UIF bit of the TIMx_ISR register. If the UIFREMAP bit in
the TIMxCR1 is reset, bit 31 is reserved and read at 0.
Bits 30:16 Reserved, must be kept at reset value.
Bits 15:0 CNT[15:0]: Counter value
Non-dithering mode (DITHEN = 0)
The register holds the counter value.
Dithering mode (DITHEN = 1)
The register only holds the non-dithered part in CNT[15:0]. The fractional part is not
available.

54.6.13

TIMx prescaler (TIMx_PSC)(x = 1, 8)
Address offset: 0x028
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

rw

rw

rw

rw

rw

rw

rw

PSC[15:0]
rw

rw

rw

rw

rw

rw

rw

rw

rw

Bits 15:0 PSC[15:0]: Prescaler value
The counter clock frequency (ftim_cnt_ck) is equal to ftim_psc_ck / (PSC[15:0] + 1).
PSC contains the value to be loaded in the active prescaler register at each update event
(including when the counter is cleared through UG bit of TIMx_EGR register or through
trigger controller when configured in “reset mode”).

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Advanced-control timers (TIM1/TIM8)

54.6.14

TIMx autoreload register (TIMx_ARR)(x = 1, 8)
Address offset: 0x02C
Reset value: 0x0000 FFFF

31

30

29

28

27

26

25

24

23

22

21

20

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

19

18

17

16

rw

rw

rw

rw

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

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

ARR[19:16]

ARR[15:0]
rw

Bits 31:20 Reserved, must be kept at reset value.
Bits 19:0 ARR[19:0]: Autoreload value
ARR is the value to be loaded in the actual autoreload register.
Refer to the Section 54.3.3: Time-base unit for more details about ARR update and
behavior.
The counter is blocked while the autoreload value is null.
Non-dithering mode (DITHEN = 0)
The register holds the autoreload value.
Dithering mode (DITHEN = 1)
The register holds the integer part in ARR[19:4]. The ARR[3:0] bitfield contains the dithered
part.

54.6.15

TIMx repetition counter register (TIMx_RCR)(x = 1, 8)
Address offset: 0x030
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

rw

rw

rw

rw

rw

rw

rw

REP[15:0]
rw

rw

rw

rw

rw

rw

rw

rw

rw

Bits 15:0 REP[15:0]: Repetition counter reload value
This bitfield defines the update rate of the compare registers (i.e. periodic transfers from
preload to active registers) when preload registers are enable. It also defines the update
interrupt generation rate, if this interrupt is enable.
When the repetition down-counter reaches zero, an update event is generated and it
restarts counting from REP value. As the repetition counter is reloaded with REP value only
at the repetition update event UEV, any write to the TIMx_RCR register is not taken in
account until the next repetition update event.
It means in PWM mode (REP+1) corresponds to:
– the number of PWM periods in edge-aligned mode
– the number of half PWM period in center-aligned mode.

RM0456 Rev 6

<!-- pagebreak -->

2236

Advanced-control timers (TIM1/TIM8)

54.6.16

RM0456

TIMx capture/compare register 1 (TIMx_CCR1)(x = 1, 8)
Address offset: 0x034
Reset value: 0x0000 0000

31

30

29

28

27

26

25

24

23

22

21

20

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

19

18

17

16

rw

rw

rw

rw

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

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

CCR1[19:16]

CCR1[15:0]
rw

rw

Bits 31:20 Reserved, must be kept at reset value.
Bits 19:0 CCR1[19:0]: Capture/compare 1 value
If channel CC1 is configured as output: CCR1 is the value to be loaded in the actual
capture/compare 1 register (preload value).
It is loaded permanently if the preload feature is not selected in the TIMx_CCMR1 register
(bit OC1PE). Else the preload value is copied in the active capture/compare 1 register when
an update event occurs.
The active capture/compare register contains the value to be compared to the counter
TIMx_CNT and signaled on tim_oc1 output.
Non-dithering mode (DITHEN = 0)
The register holds the compare value in CCR1[15:0]. The CCR1[19:16] bits are reset.
Dithering mode (DITHEN = 1)
The register holds the integer part in CCR1[19:4]. The CCR1[3:0] bitfield contains the
dithered part.
If channel CC1 is configured as input: CR1 is the counter value transferred by the last
input capture 1 event (tim_ic1). The TIMx_CCR1 register is read-only and cannot be
programmed.
Non-dithering mode (DITHEN = 0)
The register holds the capture value in CCR1[15:0]. The CCR1[19:16] bits are reset.
Dithering mode (DITHEN = 1)
The register holds the capture in CCR1[19:4]. The CCR1[3:0] bits are reset.

54.6.17

TIMx capture/compare register 2 (TIMx_CCR2)(x = 1, 8)
Address offset: 0x038
Reset value: 0x0000 0000

31

30

29

28

27

26

25

24

23

22

21

20

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

19

18

17

16

rw

rw

rw

rw

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

rw

rw

rw

rw

rw

rw

rw

CCR2[19:16]

CCR2[15:0]
rw

rw

<!-- pagebreak -->

rw

rw

rw

rw

rw

rw

rw

RM0456 Rev 6

RM0456

Advanced-control timers (TIM1/TIM8)

Bits 31:20 Reserved, must be kept at reset value.
Bits 19:0 CCR2[19:0]: Capture/compare 2 value
If channel CC2 is configured as output: CCR2 is the value to be loaded in the actual
capture/compare 2 register (preload value).
It is loaded permanently if the preload feature is not selected in the TIMx_CCMR1 register
(bit OC2PE). Else the preload value is copied in the active capture/compare 2 register when
an update event occurs.
The active capture/compare register contains the value to be compared to the counter
TIMx_CNT and signaled on tim_oc2 output.
Non-dithering mode (DITHEN = 0)
The register holds the compare value in CCR2[15:0]. The CCR2[19:16] bits are reset.
Dithering mode (DITHEN = 1)
The register holds the integer part in CCR2[19:4]. The CCR2[3:0] bitfield contains the
dithered part.
If channel CC2 is configured as input: CCR2 is the counter value transferred by the last
input capture 2 event (tim_ic2). The TIMx_CCR2 register is read-only and cannot be
programmed.
Non-dithering mode (DITHEN = 0)
The register holds the capture value in CCR2[15:0]. The CCR2[19:16] bits are reset.
Dithering mode (DITHEN = 1)
The register holds the capture in CCR2[19:4]. The CCR2[3:0] bits are reset.

54.6.18

TIMx capture/compare register 3 (TIMx_CCR3)(x = 1, 8)
Address offset: 0x03C
Reset value: 0x0000 0000

31

30

29

28

27

26

25

24

23

22

21

20

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

15

14

13

12

11

10

9

8

7

19

18

17

16

CCR3[19:16]
rw

rw

rw

rw

6

5

4

3

2

1

0

rw

rw

rw

rw

rw

rw

rw

CCR3[15:0]
rw

rw

rw

rw

rw

rw

rw

rw

rw

RM0456 Rev 6

<!-- pagebreak -->

2236

Advanced-control timers (TIM1/TIM8)

RM0456

Bits 31:20 Reserved, must be kept at reset value.
Bits 19:0 CCR3[19:0]: Capture/compare value
If channel CC3 is configured as output: CCR3 is the value to be loaded in the actual
capture/compare 3 register (preload value).
It is loaded permanently if the preload feature is not selected in the TIMx_CCMR2 register
(bit OC3PE). Else the preload value is copied in the active capture/compare 3 register when
an update event occurs.
The active capture/compare register contains the value to be compared to the counter
TIMx_CNT and signaled on tim_oc3 output.
Non-dithering mode (DITHEN = 0)
The register holds the compare value in CCR3[15:0]. The CCR3[19:16] bits are reset.
Dithering mode (DITHEN = 1)
The register holds the integer part in CCR3[19:4]. The CCR3[3:0] bitfield contains the
dithered part.
If channel CC3 is configured as input: CCR3 is the counter value transferred by the last
input capture 3 event (tim_ic3). The TIMx_CCR3 register is read-only and cannot be
programmed.
Non-dithering mode (DITHEN = 0)
The register holds the capture value in CCR3[15:0]. The CCR3[19:16] bits are reset.
Dithering mode (DITHEN = 1)
The register holds the capture in CCR3[19:4]. The CCR3[3:0] bits are reset.

54.6.19

TIMx capture/compare register 4 (TIMx_CCR4)(x = 1, 8)
Address offset: 0x040
Reset value: 0x0000 0000

31

30

29

28

27

26

25

24

23

22

21

20

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

15

14

13

12

11

10

9

8

7

19

18

17

16

CCR4[19:16]
rw

rw

rw

rw

6

5

4

3

2

1

0

rw

rw

rw

rw

rw

rw

rw

CCR4[15:0]
rw

rw

<!-- pagebreak -->

rw

rw

rw

rw

rw

rw

rw

RM0456 Rev 6

RM0456

Advanced-control timers (TIM1/TIM8)

Bits 31:20 Reserved, must be kept at reset value.
Bits 19:0 CCR4[19:0]: Capture/compare value
If channel CC4 is configured as output: CCR4 is the value to be loaded in the actual
capture/compare 4 register (preload value).
It is loaded permanently if the preload feature is not selected in the TIMx_CCMR2 register
(bit OC4PE). Else the preload value is copied in the active capture/compare 4 register when
an update event occurs.
The active capture/compare register contains the value to be compared to the counter
TIMx_CNT and signalled on tim_oc4 output.
Non-dithering mode (DITHEN = 0)
The register holds the compare value in CCR4[15:0]. The CCR4[19:16] bits are reset.
Dithering mode (DITHEN = 1)
The register holds the integer part in CCR4[19:4]. The CCR4[3:0] bitfield contains the
dithered part.
If channel CC4 is configured as input: CCR4 is the counter value transferred by the last
input capture 4 event (tim_ic4). The TIMx_CCR4 register is read-only and cannot be
programmed.
Non-dithering mode (DITHEN = 0)
The register holds the capture value in CCR4[15:0]. The CCR4[19:16] bits are reset.
Dithering mode (DITHEN = 1)
The register holds the capture in CCR4[19:4]. The CCR4[3:0] bits are reset.

54.6.20

TIMx break and dead-time register (TIMx_BDTR)(x = 1, 8)
Address offset: 0x044
Reset value: 0x0000 0000

31

30

29

28

26

25

24

BK
DSRM

BK2P

BK2E

23

22

21

20

19

17

16

Res.

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

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

MOE

AOE

BKP

BKE

OSSR

OSSI

LOCK[1:0]

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

BK2F[3:0]

18

Res.

Note:

BK2BID BKBID

27
BK2
DSRM

BKF[3:0]

DTG[7:0]
rw

As the bits BKBID/BK2BID/BK2P, BK2E, BK2F[3:0], BKF[3:0], AOE, BKP, BKE, OSSI,
OSSR, and DTG[7:0] can be write-locked depending on the LOCK configuration, it can be
necessary to configure all of them during the first write access to the TIMx_BDTR register.

RM0456 Rev 6

<!-- pagebreak -->

2236

Advanced-control timers (TIM1/TIM8)

RM0456

Bits 31:30 Reserved, must be kept at reset value.
Bit 29 BK2BID: Break2 bidirectional
Refer to BKBID description
Bit 28 BKBID: Break bidirectional
0: Break input tim_brk in input mode
1: Break input tim_brk in bidirectional mode
In the bidirectional mode (BKBID bit set to 1), the break input is configured both in input
mode and in open drain output mode. Any active break event asserts a low logic level on the
Break input to indicate an internal break event to external devices.
Note: This bit cannot be modified as long as LOCK level 1 has been programmed (LOCK bits
in TIMx_BDTR register).
Note: Any write operation to this bit takes a delay of 1 APB clock cycle to become effective.
Bit 27 BK2DSRM: Break2 disarm
Refer to BKDSRM description
Bit 26 BKDSRM: Break disarm
0: Break input tim_brk is armed
1: Break input tim_brk is disarmed
This bit is cleared by hardware when no break source is active.
The BKDSRM bit must be set by software to release the bidirectional output control (opendrain output in Hi-Z state) and then be polled it until it is reset by hardware, indicating that the
fault condition has disappeared.
Note: Any write operation to this bit takes a delay of 1 APB clock cycle to become effective.
Bit 25 BK2P: Break 2 polarity
0: Break input tim_brk2 is active low
1: Break input tim_brk2 is active high
Note: This bit cannot be modified as long as LOCK level 1 has been programmed (LOCK bits
in TIMx_BDTR register).
Note: Any write operation to this bit takes a delay of 1 APB clock cycle to become effective.
Bit 24 BK2E: Break 2 enable
This bit enables the complete break 2 protection, see Figure 556: Break and Break2 circuitry
overview).
0: Break2 function disabled
1: Break2 function enabled
Note: The BRKIN2 must only be used with OSSR = OSSI = 1.
Note: This bit cannot be modified when LOCK level 1 has been programmed (LOCK bits in
TIMx_BDTR register).
Note: Any write operation to this bit takes a delay of 1 APB clock cycle to become effective.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Advanced-control timers (TIM1/TIM8)

Bits 23:20 BK2F[3:0]: Break 2 filter
This bitfield defines the frequency used to sample tim_brk2 input and the length of the digital
filter applied to tim_brk2. The digital filter is made of an event counter in which N consecutive
events are needed to validate a transition on the output:
0000: No filter, tim_brk2 acts asynchronously
0001: fSAMPLING = ftim_ker_ck, N = 2
0010: fSAMPLING = ftim_ker_ck, N = 4
0011: fSAMPLING = ftim_ker_ck, N = 8
0100: fSAMPLING = fDTS/2, N = 6
0101: fSAMPLING = fDTS/2, N = 8
0110: fSAMPLING = fDTS/4, N = 6
0111: fSAMPLING = fDTS/4, N = 8
1000: fSAMPLING = fDTS/8, N = 6
1001: fSAMPLING = fDTS/8, N = 8
1010: fSAMPLING = fDTS/16, N = 5
1011: fSAMPLING = fDTS/16, N = 6
1100: fSAMPLING = fDTS/16, N = 8
1101: fSAMPLING = fDTS/32, N = 5
1110: fSAMPLING = fDTS/32, N = 6
1111: fSAMPLING = fDTS/32, N = 8
Note: This bit cannot be modified when LOCK level 1 has been programmed (LOCK bits in
TIMx_BDTR register).
Bits 19:16 BKF[3:0]: Break filter
This bitfield defines the frequency used to sample tim_brk input and the length of the digital
filter applied to tim_brk. The digital filter is made of an event counter in which N consecutive
events are needed to validate a transition on the output:
0000: No filter, tim_brk acts asynchronously
0001: fSAMPLING = ftim_ker_ck, N = 2
0010: fSAMPLING = ftim_ker_ck, N = 4
0011: fSAMPLING = ftim_ker_ck, N = 8
0100: fSAMPLING = fDTS/2, N = 6
0101: fSAMPLING = fDTS/2, N = 8
0110: fSAMPLING = fDTS/4, N = 6
0111: fSAMPLING = fDTS/4, N = 8
1000: fSAMPLING = fDTS/8, N = 6
1001: fSAMPLING = fDTS/8, N = 8
1010: fSAMPLING = fDTS/16, N = 5
1011: fSAMPLING = fDTS/16, N = 6
1100: fSAMPLING = fDTS/16, N = 8
1101: fSAMPLING = fDTS/32, N = 5
1110: fSAMPLING = fDTS/32, N = 6
1111: fSAMPLING = fDTS/32, N = 8
Note: This bit cannot be modified when LOCK level 1 has been programmed (LOCK bits in
TIMx_BDTR register).

RM0456 Rev 6

<!-- pagebreak -->

2236

Advanced-control timers (TIM1/TIM8)

RM0456

Bit 15 MOE: Main output enable
This bit is cleared asynchronously by hardware as soon as one of the break inputs is active
(tim_brk or tim_brk2). It is set by software or automatically depending on the AOE bit. It is
acting only on the channels which are configured in output.
0: In response to a break 2 event. OC and OCN outputs are disabled
In response to a break event or if MOE is written to 0: OC and OCN outputs are disabled or
forced to idle state depending on the OSSI bit.
1: OC and OCN outputs are enabled if their respective enable bits are set (CCxE, CCxNE in
TIMx_CCER register).
See OC/OCN enable description for more details (Section 54.6.11: TIMx capture/compare
enable register (TIMx_CCER)(x = 1, 8)).
Bit 14 AOE: Automatic output enable
0: MOE can be set only by software
1: MOE can be set by software or automatically at the next update event (if none of the break
inputs tim_brk and tim_brk2 is active)
Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits
in TIMx_BDTR register).
Bit 13 BKP: Break polarity
0: Break input tim_brk is active low
1: Break input tim_brk is active high
Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits
in TIMx_BDTR register).
Note: Any write operation to this bit takes a delay of 1 APB clock cycle to become effective.
Bit 12 BKE: Break enable
This bit enables the complete break protection (including all sources connected to
tim_sys_brk and BKIN sources, as per Figure 556: Break and Break2 circuitry overview).
0: Break function disabled
1: Break function enabled
Note: This bit cannot be modified when LOCK level 1 has been programmed (LOCK bits in
TIMx_BDTR register).
Note: Any write operation to this bit takes a delay of 1 APB clock cycle to become effective.
Bit 11 OSSR: Off-state selection for Run mode
This bit is used when MOE = 1 on channels having a complementary output which are
configured as outputs. OSSR is not implemented if no complementary output is implemented
in the timer.
See OC/OCN enable description for more details (Section 54.6.11: TIMx capture/compare
enable register (TIMx_CCER)(x = 1, 8)).
0: When inactive, OC/OCN outputs are disabled (the timer releases the output control which
is taken over by the GPIO logic, which forces a Hi-Z state).
1: When inactive, OC/OCN outputs are enabled with their inactive level as soon as CCxE = 1
or CCxNE = 1 (the output is still controlled by the timer).
Note: This bit can not be modified as soon as the LOCK level 2 has been programmed (LOCK
bits in TIMx_BDTR register).

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Advanced-control timers (TIM1/TIM8)

Bit 10 OSSI: Off-state selection for idle mode
This bit is used when MOE = 0 due to a break event or by a software write, on channels
configured as outputs.
See OC/OCN enable description for more details (Section 54.6.11: TIMx capture/compare
enable register (TIMx_CCER)(x = 1, 8)).
0: When inactive, OC/OCN outputs are disabled (the timer releases the output control which
is taken over by the GPIO logic and which imposes a Hi-Z state).
1: When inactive, OC/OCN outputs are first forced with their inactive level then forced to their
idle level after the deadtime. The timer maintains its control over the output.
Note: This bit can not be modified as soon as the LOCK level 2 has been programmed (LOCK
bits in TIMx_BDTR register).
Bits 9:8 LOCK[1:0]: Lock configuration
These bits offer a write protection against software errors.
00: LOCK OFF - No bit is write protected.
01: LOCK Level 1 = DTG bits in TIMx_BDTR register, OISx and OISxN bits in TIMx_CR2
register and BKBID/BK2BID/BKE/BKP/AOE bits in TIMx_BDTR register can no longer be
written.
10: LOCK Level 2 = LOCK Level 1 + CC Polarity bits (CCxP/CCxNP bits in TIMx_CCER
register, as long as the related channel is configured in output through the CCxS bits) as
well as OSSR and OSSI bits can no longer be written.
11: LOCK Level 3 = LOCK Level 2 + CC Control bits (OCxM and OCxPE bits in
TIMx_CCMRx registers, as long as the related channel is configured in output through
the CCxS bits) can no longer be written.
Note: The LOCK bits can be written only once after the reset. Once the TIMx_BDTR register
has been written, their content is frozen until the next reset.
Bits 7:0 DTG[7:0]: Dead-time generator setup
This bitfield defines the duration of the dead-time inserted between the complementary
outputs. DT correspond to this duration.
DTG[7:5] = 0xx => DT = DTG[7:0]x tdtg with tdtg = tDTS.
DTG[7:5] = 10x => DT = (64+DTG[5:0])xtdtg with Tdtg = 2xtDTS.
DTG[7:5] = 110 => DT = (32+DTG[4:0])xtdtg with Tdtg = 8xtDTS.
DTG[7:5] = 111 => DT = (32+DTG[4:0])xtdtg with Tdtg = 16xtDTS.
Example if TDTS = 125 ns (8 MHz), dead-time possible values are:
0 to 15875 ns by 125 ns steps,
16 μs to 31750 ns by 250 ns steps,
32 μs to 63 μs by 1 μs steps,
64 μs to 126 μs by 2 μs steps
Note: This bitfield can not be modified as long as LOCK level 1, 2 or 3 has been programmed
(LOCK bits in TIMx_BDTR register).

54.6.21

TIMx capture/compare register 5 (TIMx_CCR5)(x = 1, 8)
Address offset: 0x048
Reset value: 0x0000 0000

31

30

29

GC5C3 GC5C2 GC5C1

28

27

26

25

24

23

22

21

20

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.
rw

rw

rw

rw

8

7

6

5

4

3

2

1

0

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

15

14

13

12

11

10

9

rw

rw

rw

rw

rw

rw

rw

19

18

17

16

CCR5[19:16]

CCR5[15:0]
rw

rw

RM0456 Rev 6

<!-- pagebreak -->

2236

Advanced-control timers (TIM1/TIM8)

RM0456

Bit 31 GC5C3: Group channel 5 and channel 3
Distortion on channel 3 output:
0: No effect of tim_oc5ref on tim_oc3refc
1: tim_oc3refc is the logical AND of tim_oc3ref and tim_oc5ref
This bit can either have immediate effect or be preloaded and taken into account after an
update event (if preload feature is selected in TIMxCCMR2).
Note: it is also possible to apply this distortion on combined PWM signals.
Bit 30 GC5C2: Group channel 5 and channel 2
Distortion on channel 2 output:
0: No effect of tim_oc5ref on tim_oc2refc
1: tim_oc2refc is the logical AND of tim_oc2ref and tim_oc5ref
This bit can either have immediate effect or be preloaded and taken into account after an
update event (if preload feature is selected in TIMxCCMR1).
Note: it is also possible to apply this distortion on combined PWM signals.
Bit 29 GC5C1: Group channel 5 and channel 1
Distortion on channel 1 output:
0: No effect of oc5ref on oc1refc
1: oc1refc is the logical AND of oc1ref and oc5ref
This bit can either have immediate effect or be preloaded and taken into account after an
update event (if preload feature is selected in TIMxCCMR1).
Note: it is also possible to apply this distortion on combined PWM signals.
Bits 28:20 Reserved, must be kept at reset value.
Bits 19:0 CCR5[19:0]: Capture/compare 5 value
CCR5 is the value to be loaded in the actual capture/compare 5 register (preload value).
It is loaded permanently if the preload feature is not selected in the TIMx_CCMR3 register
(bit OC5PE). Else the preload value is copied in the active capture/compare 5 register when
an update event occurs.
The active capture/compare register contains the value to be compared to the counter
TIMx_CNT and signaled on tim_oc5 output.
Non-dithering mode (DITHEN = 0)
The register holds the compare value in CCR5[15:0]. The CCR5[19:16] bits are reset.
Dithering mode (DITHEN = 1)
The register holds the integer part in CCR5[19:4]. The CCR5[3:0] bitfield contains the
dithered part.

54.6.22

TIMx capture/compare register 6 (TIMx_CCR6)(x = 1, 8)
Address offset: 0x04C
Reset value: 0x0000 0000

31

30

29

28

27

26

25

24

23

22

21

20

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

15

14

13

12

11

10

9

8

7

19

18

17

16

CCR6[19:16]
rw

rw

rw

rw

6

5

4

3

2

1

0

rw

rw

rw

rw

rw

rw

rw

CCR6[15:0]
rw

rw

<!-- pagebreak -->

rw

rw

rw

rw

rw

rw

rw

RM0456 Rev 6

RM0456

Advanced-control timers (TIM1/TIM8)

Bits 31:20 Reserved, must be kept at reset value.
Bits 19:0 CCR6[19:0]: Capture/compare 6 value
CCR6 is the value to be loaded in the actual capture/compare 6 register (preload value).
It is loaded permanently if the preload feature is not selected in the TIMx_CCMR3 register
(bit OC6PE). Else the preload value is copied in the active capture/compare 6 register when
an update event occurs.
The active capture/compare register contains the value to be compared to the counter
TIMx_CNT and signaled on tim_oc6 output.
Non-dithering mode (DITHEN = 0)
The register holds the compare value in CCR6[15:0]. The CCR6[19:16] bits are reset.
Dithering mode (DITHEN = 1)
The register holds the integer part in CCR6[19:4]. The CCR6[3:0] bitfield contains the
dithered part.

54.6.23

TIMx capture/compare mode register 3 (TIMx_CCMR3)
(x = 1, 8)
Address offset: 0x050
Reset value: 0x0000 0000
Refer to the above CCMR1 register description. Channels 5 and 6 can only be configured in
output.

31

30

29

28

27

26

25

24

23

22

21

20

19

18

17

16

Res.

Res.

Res.

Res.

Res.

Res.

Res.

OC6M[3]

Res.

Res.

Res.

Res.

Res.

Res.

Res.

OC5M[3]

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

OC5
CE

Res.

Res.

rw
15

14

OC6
CE
rw

13

12

OC6M[2:0]
rw

rw

rw

11

10

OC6
PE

OC6FE

rw

rw

9
Res.

rw

rw

OC5M[2:0]
rw

rw

OC5PE OC5FE
rw

rw

rw

Bits 31:25 Reserved, must be kept at reset value.
Bits 23:17 Reserved, must be kept at reset value.
Bit 15 OC6CE: Output compare 6 clear enable
Bits 24, 14:12 OC6M[3:0]: Output compare 6 mode
Bit 11 OC6PE: Output compare 6 preload enable
Bit 10 OC6FE: Output compare 6 fast enable
Bits 9:8 Reserved, must be kept at reset value.
Bit 7 OC5CE: Output compare 5 clear enable
Bits 16, 6:4 OC5M[3:0]: Output compare 5 mode
Bit 3 OC5PE: Output compare 5 preload enable
Bit 2 OC5FE: Output compare 5 fast enable
Bits 1:0 Reserved, must be kept at reset value.

RM0456 Rev 6

<!-- pagebreak -->

2236

Advanced-control timers (TIM1/TIM8)

54.6.24

RM0456

TIMx timer deadtime register 2 (TIMx_DTR2)(x = 1, 8)
Address offset: 0x054
Reset value: 0x0000 0000

31

30

29

28

27

26

25

24

23

22

21

20

19

18

17

16

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

DTPE

DTAE

rw

rw

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
rw

rw

rw

rw

rw

rw

rw

DTGF[7:0]
rw

Bits 31:18 Reserved, must be kept at reset value.
Bit 17 DTPE: Deadtime preload enable
0: Deadtime value is not preloaded
1: Deadtime value preload is enabled
Note: This bit can not be modified as long as LOCK level 1, 2 or 3 has been programmed
(LOCK bits in TIMx_BDTR register).
Bit 16 DTAE: Deadtime asymmetric enable
0: Deadtime on rising and falling edges are identical, and defined with DTG[7:0] register
1: Deadtime on rising edge is defined with DTG[7:0] register and deadtime on falling edge is
defined with DTGF[7:0] bits.
Note: This bit can not be modified as long as LOCK level 1, 2 or 3 has been programmed
(LOCK bits in TIMx_BDTR register).
Bits 15:8 Reserved, must be kept at reset value.
Bits 7:0 DTGF[7:0]: Dead-time falling edge generator setup
This bitfield defines the duration of the dead-time inserted between the complementary
outputs, on the falling edge.
DTGF[7:5] = 0xx => DTF = DTGF[7:0]x tdtg with tdtg = tDTS.
DTGF[7:5] = 10x => DTF = (64+DTGF[5:0])xtdtg with Tdtg = 2xtDTS.
DTGF[7:5] = 110 => DTF = (32+DTGF[4:0])xtdtg with Tdtg = 8xtDTS.
DTGF[7:5] = 111 => DTF = (32+DTGF[4:0])xtdtg with Tdtg = 16xtDTS.
Example if TDTS = 125 ns (8 MHz), dead-time possible values are:
0 to 15875 ns by 125 ns steps,
16 μs to 31750 ns by 250 ns steps,
32 μs to 63 μs by 1 μs steps,
64 μs to 126 μs by 2 μs steps
Note: This bitfield can not be modified as long as LOCK level 1, 2 or 3 has been programmed
(LOCK bits in TIMx_BDTR register).

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Advanced-control timers (TIM1/TIM8)

54.6.25

TIMx timer encoder control register (TIMx_ECR)(x = 1, 8)
Address offset: 0x058
Reset value: 0x0000 0000

31

30

29

28

27

Res.

Res.

Res.

Res.

Res.

26

25

24

23

22

21

20

rw

rw

rw

rw

rw

rw

rw

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

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

PWPRSC[2:0]

19

18

17

16

rw

rw

rw

rw

3

2

1

0

PW[7:0]

IPOS[1:0]

FIDX

IBLK[1:0]

IDIR[1:0]

IE

rw

rw

rw

rw

rw

rw

rw

rw

Bits 31:27 Reserved, must be kept at reset value.
Bits 26:24 PWPRSC[2:0]: Pulse width prescaler

This bitfield sets the clock prescaler for the pulse generator, as following:
tPWG = (2(PWPRSC[2:0])) x ttim_ker_ck
Bits 23:16 PW[7:0]: Pulse width
This bitfield defines the pulse duration, as following:

tPW = PW[7:0] x tPWG
Bits 15:8 Reserved, must be kept at reset value.
Bits 7:6 IPOS[1:0]: Index positioning
In quadrature encoder mode (SMS[3:0] = 0001, 0010, 0011, 1110, 1111), this bit indicates in
which AB input configuration the Index event resets the counter.
00: Index resets the counter when AB = 00
01: Index resets the counter when AB = 01
10: Index resets the counter when AB = 10
11: Index resets the counter when AB = 11
In directional clock mode or clock plus direction mode (SMS[3:0] = 1010, 1011, 1100, 1101),
these bits indicates on which level the Index event resets the counter. In bidirectional clock
mode, this applies for both clock inputs.
x0: Index resets the counter when clock is 0
x1: Index resets the counter when clock is 1
Note: IPOS[1] bit is not significant
Bit 5 FIDX: First index
This bit indicates if the first index only is taken into account
0: Index is always active
1: the first Index only resets the counter

RM0456 Rev 6

<!-- pagebreak -->

2236

Advanced-control timers (TIM1/TIM8)

RM0456

Bits 4:3 IBLK[1:0]: Index blanking
This bit indicates if the Index event is conditioned by the tim_ti3 or tim_ti4 input
00: Index always active
01: Index disabled when tim_ti3 input is active, as per CC3P bitfield
10: Index disabled when tim_ti4 input is active, as per CC4P bitfield
11: Reserved
Bits 2:1 IDIR[1:0]: Index direction
This bit indicates in which direction the Index event resets the counter.
00: Index resets the counter whatever the direction
01: Index resets the counter when up-counting only
10: Index resets the counter when down-counting only
11: Reserved
Bit 0 IE: Index enable
This bit indicates if the Index event resets the counter.
0: Index disabled
1: Index enabled

54.6.26

TIMx timer input selection register (TIMx_TISEL)(x = 1, 8)
Address offset: 0x05C
Reset value: 0x0000 0000

31

30

29

28

Res.

Res.

Res.

Res.

15

14

13

12

Res.

Res.

Res.

Res.

27

26

25

24

TI4SEL[3:0]
rw

rw

rw

rw

11

10

9

8

TI2SEL[3:0]
rw

rw

rw

23

22

21

20

Res.

Res.

Res.

Res.

7

6

5

4

Res.

Res.

Res.

Res.

rw

19

18

17

TI3SEL[3:0]
rw

rw

rw

rw

3

2

1

0

TI1SEL[3:0]
rw

rw

rw

Bits 31:28 Reserved, must be kept at reset value.
Bits 27:24 TI4SEL[3:0]: Selects tim_ti4[15:0] input
0000: tim_ti4_in0: TIMx_CH4
0001: tim_ti4_in1
...
1111: tim_ti4_in15
Refer to Section 54.3.2: TIM1/TIM8 pins and internal signals for interconnects list.
Bits 23:20 Reserved, must be kept at reset value.
Bits 19:16 TI3SEL[3:0]: Selects tim_ti3[15:0] input
0000: tim_ti3_in0: TIMx_CH2
0001: tim_ti3_in1
...
1111: tim_ti3_in15
Refer to Section 54.3.2: TIM1/TIM8 pins and internal signals for interconnects list.
Bits 15:12 Reserved, must be kept at reset value.

<!-- pagebreak -->

RM0456 Rev 6

16

rw

RM0456

Advanced-control timers (TIM1/TIM8)

Bits 11:8 TI2SEL[3:0]: Selects tim_ti2[15:0] input
0000: tim_ti2_in0: TIMx_CH2
0001: tim_ti2_in1
...
1111: tim_ti2_in15
Refer to Section 54.3.2: TIM1/TIM8 pins and internal signals for interconnects list.
Bits 7:4 Reserved, must be kept at reset value.
Bits 3:0 TI1SEL[3:0]: Selects tim_ti1[15:0] input
0000: tim_ti1_in0: TIMx_CH1
0001: tim_ti1_in1
...
1111: tim_ti1_in15
Refer to Section 54.3.2: TIM1/TIM8 pins and internal signals for interconnects list.

54.6.27

TIMx alternate function option register 1 (TIMx_AF1)(x = 1, 8)
Address offset: 0x060
Reset value: 0x0000 0001

31

30

29

28

27

26

25

24

23

22

21

20

19

18

17

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

ETRSEL[3:2]

15

14

ETRSEL[1:0]
rw

rw

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

16

rw

rw

1

0

BK
BK
BK
BK
BK
BK
BK
BK
BK
BK
BK
BK
BKINP
BKINE
CMP4P CMP3P CMP2P CMP1P
CMP8E CMP7E CMP6E CMP5E CMP4E CMP3E CMP2E CMP1E
rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

Bits 31:18 Reserved, must be kept at reset value.
Bits 17:14 ETRSEL[3:0]: etr_in source selection
These bits select the etr_in input source.
0000: tim_etr0: TIMx_ETR input
0001: tim_etr1
...
1111: tim_etr15
Refer to Section 54.3.2: TIM1/TIM8 pins and internal signals for product specific
implementation.
Note: These bits can not be modified as long as LOCK level 1 has been programmed (LOCK
bits in TIMx_BDTR register).
Bit 13 BKCMP4P: tim_brk_cmp4 input polarity
This bit selects the tim_brk_cmp4 input sensitivity. It must be programmed together with the
BKP polarity bit.
0: tim_brk_cmp4 input polarity is not inverted (active low if BKP = 0, active high if BKP = 1)
1: tim_brk_cmp4 input polarity is inverted (active high if BKP = 0, active low if BKP = 1)
Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits
in TIMx_BDTR register).

RM0456 Rev 6

<!-- pagebreak -->

2236

Advanced-control timers (TIM1/TIM8)

RM0456

Bit 12 BKCMP3P: tim_brk_cmp3 input polarity
This bit selects the tim_brk_cmp3 input sensitivity. It must be programmed together with the
BKP polarity bit.
0: tim_brk_cmp3 input polarity is not inverted (active low if BKP = 0, active high if BKP = 1)
1: tim_brk_cmp3 input polarity is inverted (active high if BKP = 0, active low if BKP = 1)
Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits
in TIMx_BDTR register).
Bit 11 BKCMP2P: tim_brk_cmp2 input polarity
This bit selects the tim_brk_cmp2 input sensitivity. It must be programmed together with the
BKP polarity bit.
0: tim_brk_cmp2 input polarity is not inverted (active low if BKP = 0, active high if BKP = 1)
1: tim_brk_cmp2 input polarity is inverted (active high if BKP = 0, active low if BKP = 1)
Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits
in TIMx_BDTR register).
Bit 10 BKCMP1P: tim_brk_cmp1 input polarity
This bit selects the tim_brk_cmp1 input sensitivity. It must be programmed together with the
BKP polarity bit.
0: tim_brk_cmp1 input polarity is not inverted (active low if BKP = 0, active high if BKP = 1)
1: tim_brk_cmp1 input polarity is inverted (active high if BKP = 0, active low if BKP = 1)
Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits
in TIMx_BDTR register).
Bit 9 BKINP: TIMx_BKIN input polarity
This bit selects the TIMx_BKIN alternate function input sensitivity. It must be programmed
together with the BKP polarity bit.
0: TIMx_BKIN input polarity is not inverted (active low if BKP = 0, active high if BKP = 1)
1: TIMx_BKIN input polarity is inverted (active high if BKP = 0, active low if BKP = 1)
Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits
in TIMx_BDTR register).
Bit 8 BKCMP8E: tim_brk_cmp8 enable
This bit enables the tim_brk_cmp8 for the timer’s tim_brk input. tim_brk_cmp8 output is
‘ORed’ with the other tim_brk sources.
0: tim_brk_cmp8 input disabled
1: tim_brk_cmp8 input enabled
Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits
in TIMx_BDTR register).
Bit 7 BKCMP7E: tim_brk_cmp7 enable
This bit enables the tim_brk_cmp7 for the timer’s tim_brk input. tim_brk_cmp7 output is
‘ORed’ with the other tim_brk sources.
0: tim_brk_cmp7 input disabled
1: tim_brk_cmp7 input enabled
Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits
in TIMx_BDTR register).
Bit 6 BKCMP6E: tim_brk_cmp6 enable
This bit enables the tim_brk_cmp6 for the timer’s tim_brk input. tim_brk_cmp6 output is
‘ORed’ with the other tim_brk sources.
0: tim_brk_cmp6 input disabled
1: tim_brk_cmp6 input enabled
Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits
in TIMx_BDTR register).

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Advanced-control timers (TIM1/TIM8)

Bit 5 BKCMP5E: tim_brk_cmp5 enable
This bit enables the tim_brk_cmp5 for the timer’s tim_brk input. tim_brk_cmp5 output is
‘ORed’ with the other tim_brk sources.
0: tim_brk_cmp5 input disabled
1: tim_brk_cmp5 input enabled
Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits
in TIMx_BDTR register).
Bit 4 BKCMP4E: tim_brk_cmp4 enable
This bit enables the tim_brk_cmp4 for the timer’s tim_brk input. tim_brk_cmp4 output is
‘ORed’ with the other tim_brk sources.
0: tim_brk_cmp4 input disabled
1: tim_brk_cmp4 input enabled
Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits
in TIMx_BDTR register).
Bit 3 BKCMP3E: tim_brk_cmp3 enable
This bit enables the tim_brk_cmp3 for the timer’s tim_brk input. tim_brk_cmp3 output is
‘ORed’ with the other tim_brk sources.
0: tim_brk_cmp3 input disabled
1: tim_brk_cmp3 input enabled
Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits
in TIMx_BDTR register).
Bit 2 BKCMP2E: tim_brk_cmp2 enable
This bit enables the tim_brk_cmp2 for the timer’s tim_brk input. tim_brk_cmp2 output is
‘ORed’ with the other tim_brk sources.
0: tim_brk_cmp2 input disabled
1: tim_brk_cmp2 input enabled
Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits
in TIMx_BDTR register).
Bit 1 BKCMP1E: tim_brk_cmp1 enable
This bit enables the tim_brk_cmp1 for the timer’s tim_brk input. tim_brk_cmp1 output is
‘ORed’ with the other tim_brk sources.
0: tim_brk_cmp1 input disabled
1: tim_brk_cmp1 input enabled
Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits
in TIMx_BDTR register).
Bit 0 BKINE: TIMx_BKIN input enable
This bit enables the TIMx_BKIN alternate function input for the timer’s tim_brk input.
TIMx_BKIN input is ‘ORed’ with the other tim_brk sources.
0: TIMx_BKIN input disabled
1: TIMx_BKIN input enabled
Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits
in TIMx_BDTR register).

Note:

Refer to Section 54.3.2: TIM1/TIM8 pins and internal signals for product specific
implementation.

RM0456 Rev 6

<!-- pagebreak -->

2236

Advanced-control timers (TIM1/TIM8)

54.6.28

RM0456

TIMx alternate function register 2 (TIMx_AF2)(x = 1, 8)
Address offset: 0x064
Reset value: 0x0000 0001

31

30

29

28

27

26

25

24

23

22

21

20

19

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

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

Res.

Res.

BK2C
MP4P

BK2C
MP3P

BK2C
MP2P

BK2C
MP1P

BK2IN
P

BK2CM
P8E

BK2C
MP7E

BK2C
MP6E

BK2C
MP5E

rw

rw

rw

rw

rw

rw

rw

rw

rw

18

17

rw

rw

rw

2

1

0

BK2C BK2CMP BK2CMP BK2CM
MP4E
3E
2E
P1E
rw

rw

16

OCRSEL[2:0]

rw

rw

BK2INE
rw

Bits 31:19 Reserved, must be kept at reset value.
Bits 18:16 OCRSEL[2:0]: ocref_clr source selection
These bits select the ocref_clr input source.
000: tim_ocref_clr0
001: tim_ocref_clr1
...
111: tim_ocref_clr7
Refer to Section 54.3.2: TIM1/TIM8 pins and internal signals for product specific information.
Note: These bits can not be modified as long as LOCK level 1 has been programmed (LOCK
bits in TIMx_BDTR register).
Bits 15:14 Reserved, must be kept at reset value.
Bit 13 BK2CMP4P: tim_brk2_cmp4 input polarity
This bit selects the tim_brk2_cmp4 input sensitivity. It must be programmed together with the
BK2P polarity bit.
0: tim_brk2_cmp4 input polarity is not inverted (active low if BK2P = 0, active high if
BK2P = 1)
1: tim_brk2_cmp4 input polarity is inverted (active high if BK2P = 0, active low if BK2P = 1)
Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits
in TIMx_BDTR register).
Bit 12 BK2CMP3P: tim_brk2_cmp3 input polarity
This bit selects the tim_brk2_cmp3 input sensitivity. It must be programmed together with the
BK2P polarity bit.
0: tim_brk2_cmp3 input polarity is not inverted (active low if BK2P = 0, active high if
BK2P = 1)
1: tim_brk2_cmp3 input polarity is inverted (active high if BK2P = 0, active low if BK2P = 1)
Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits
in TIMx_BDTR register).
Bit 11 BK2CMP2P: tim_brk2_cmp2 input polarity
This bit selects the tim_brk2_cmp2 input sensitivity. It must be programmed together with the
BK2P polarity bit.
0: tim_brk2_cmp2 input polarity is not inverted (active low if BK2P = 0, active high if
BK2P = 1)
1: tim_brk2_cmp2 input polarity is inverted (active high if BK2P = 0, active low if BK2P = 1)
Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits
in TIMx_BDTR register).

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Advanced-control timers (TIM1/TIM8)

Bit 10 BK2CMP1P: tim_brk2_cmp1 input polarity
This bit selects the tim_brk2_cmp1 input sensitivity. It must be programmed together with the
BK2P polarity bit.
0: tim_brk2_cmp1 input polarity is not inverted (active low if BK2P = 0, active high if
BK2P = 1)
1: tim_brk2_cmp1 input polarity is inverted (active high if BK2P = 0, active low if BK2P = 1)
Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits
in TIMx_BDTR register).
Bit 9 BK2INP: TIMx_BKIN2 input polarity
This bit selects the TIMx_BKIN2 alternate function input sensitivity. It must be programmed
together with the BK2P polarity bit.
0: TIMx_BKIN2 input polarity is not inverted (active low if BK2P = 0, active high if BK2P = 1)
1: TIMx_BKIN2 input polarity is inverted (active high if BK2P = 0, active low if BK2P = 1)
Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits
in TIMx_BDTR register).
Bit 8 BK2CMP8E: tim_brk2_cmp8 enable
This bit enables the tim_brk2_cmp8 for the timer’s tim_brk2 input. tim_brk2_cmp8 output is
‘ORed’ with the other tim_brk2 sources.
0: tim_brk2_cmp8 input disabled
1: tim_brk2_cmp8 input enabled
Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits
in TIMx_BDTR register).
Bit 7 BK2CMP7E: tim_brk2_cmp7 enable
This bit enables the tim_brk2_cmp7 for the timer’s tim_brk2 input. tim_brk2_cmp7 output is
‘ORed’ with the other tim_brk2 sources.
0: tim_brk2_cmp7 input disabled
1: tim_brk2_cmp7 input enabled
Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits
in TIMx_BDTR register).
Bit 6 BK2CMP6E: tim_brk2_cmp6 enable
This bit enables the tim_brk2_cmp6 for the timer’s tim_brk2 input. tim_brk2_cmp6 output is
‘ORed’ with the other tim_brk2 sources.
0: tim_brk2_cmp6 input disabled
1: tim_brk2_cmp6 input enabled
Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits
in TIMx_BDTR register).
Bit 5 BK2CMP5E: tim_brk2_cmp5 enable
This bit enables the tim_brk2_cmp5 for the timer’s tim_brk2 input. tim_brk2_cmp5 output is
‘ORed’ with the other tim_brk2 sources.
0: tim_brk2_cmp5 input disabled
1: tim_brk2_cmp5 input enabled
Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits
in TIMx_BDTR register).
Bit 4 BK2CMP4E: tim_brk2_cmp4 enable
This bit enables the tim_brk2_cmp4 for the timer’s tim_brk2 input. tim_brk2_cmp4 output is
‘ORed’ with the other tim_brk2 sources.
0: tim_brk2_cmp4 input disabled
1: tim_brk2_cmp4 input enabled
Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits
in TIMx_BDTR register).

RM0456 Rev 6

<!-- pagebreak -->

2236

Advanced-control timers (TIM1/TIM8)

RM0456

Bit 3 BK2CMP3E: tim_brk2_cmp3 enable
This bit enables the tim_brk2_cmp3 for the timer’s tim_brk2 input. tim_brk2_cmp3 output is
‘ORed’ with the other tim_brk2 sources.
0: tim_brk2_cmp3 input disabled
1: tim_brk2_cmp3 input enabled
Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits
in TIMx_BDTR register).
Bit 2 BK2CMP2E: tim_brk2_cmp2 enable
This bit enables the tim_brk2_cmp2 for the timer’s tim_brk2 input. tim_brk2_cmp2 output is
‘ORed’ with the other tim_brk2 sources.
0: tim_brk2_cmp2 input disabled
1: tim_brk2_cmp2 input enabled
Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits
in TIMx_BDTR register).
Bit 1 BK2CMP1E: tim_brk2_cmp1 enable
This bit enables the tim_brk2_cmp1 for the timer’s tim_brk2 input. tim_brk2_cmp1 output is
‘ORed’ with the other tim_brk2 sources.
0: tim_brk2_cmp1 input disabled
1: tim_brk2_cmp1 input enabled
Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits
in TIMx_BDTR register).
Bit 0 BK2INE: TIMx_BKIN2 input enable
This bit enables the TIMx_BKIN2 alternate function input for the timer’s tim_brk2 input.
TIMx_BKIN2 input is ‘ORed’ with the other tim_brk2 sources.
0: TIMx_BKIN2 input disabled
1: TIMx_BKIN2 input enabled
Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits
in TIMx_BDTR register).

Note:

Refer to Section 54.3.2: TIM1/TIM8 pins and internal signals for product specific
implementation.

54.6.29

TIMx DMA control register (TIMx_DCR)(x = 1, 8)
Address offset: 0x3DC
Reset value: 0x0000 0000

31

30

29

28

27

26

25

24

23

22

21

20

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.
rw

rw

rw

rw

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

rw

rw

rw

rw

rw

rw

rw

rw

<!-- pagebreak -->

DBL[4:0]
rw

RM0456 Rev 6

19

18

17

16

DBSS[3:0]

DBA[4:0]
rw

RM0456

Advanced-control timers (TIM1/TIM8)

Bits 31:20 Reserved, must be kept at reset value.
Bits 19:16 DBSS[3:0]: DMA burst source selection
This bitfield defines the interrupt source that triggers the DMA burst transfers (the timer
recognizes a burst transfer when a read or a write access is done to the TIMx_DMAR
address).
0000: Reserved
0001: Update
0010: CC1
0011: CC2
0100: CC3
0101: CC4
0110: COM
0111: Trigger
Others: reserved
Bits 15:13 Reserved, must be kept at reset value.
Bits 12:8 DBL[4:0]: DMA burst length
This 5-bit vector defines the length of DMA transfers (the timer recognizes a burst transfer
when a read or a write access is done to the TIMx_DMAR address), i.e. the number of
transfers. Transfers can be in half-words or in bytes (see example below).
00000: 1 transfer
00001: 2 transfers
00010: 3 transfers
...
11010: 26 transfers
Example: Let us consider the following transfer: DBL = 7 bytes & DBA = TIM2_CR1.
–If DBL = 7 bytes and DBA = TIM2_CR1 represents the address of the byte to be
transferred, the address of the transfer is given by the following equation:
(TIMx_CR1 address) + DBA + (DMA index), where DMA index = DBL
In this example, 7 bytes are added to (TIMx_CR1 address) + DBA, which gives us the
address from/to which the data are copied. In this case, the transfer is done to 7 registers
starting from the following address: (TIMx_CR1 address) + DBA
According to the configuration of the DMA Data Size, several cases may occur:
–If the DMA Data Size is configured in half-words, 16-bit data are transferred to each of the 7
registers.
–If the DMA Data Size is configured in bytes, the data are also transferred to 7 registers: the
first register contains the first MSB byte, the second register, the first LSB byte and so on.
So with the transfer Timer, one also has to specify the size of data transferred by DMA.
Bits 7:5 Reserved, must be kept at reset value.
Bits 4:0 DBA[4:0]: DMA base address
This 5-bits vector defines the base-address for DMA transfers (when read/write access are
done through the TIMx_DMAR address). DBA is defined as an offset starting from the
address of the TIMx_CR1 register.
Example:
00000: TIMx_CR1
00001: TIMx_CR2
00010: TIMx_SMCR
...

RM0456 Rev 6

<!-- pagebreak -->

2236

Advanced-control timers (TIM1/TIM8)

54.6.30

RM0456

TIMx DMA address for full transfer (TIMx_DMAR)(x = 1, 8)
Address offset: 0x3E0
Reset value: 0x0000 0000

31

30

29

28

27

26

25

24

23

22

21

20

19

18

17

16

DMAB[31:16]
rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

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

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

DMAB[15:0]
rw

rw

Bits 31:0 DMAB[31:0]: DMA register for burst accesses
A read or write operation to the DMAR register accesses the register located at the address
(TIMx_CR1 address) + (DBA + DMA index) x 4
where TIMx_CR1 address is the address of the control register 1, DBA is the DMA base
address configured in TIMx_DCR register, DMA index is automatically controlled by the DMA
transfer, and ranges from 0 to DBL (DBL configured in TIMx_DCR).

54.6.31

TIMx register map
TIMx registers are mapped as 16-bit addressable registers as described in the table below:

<!-- pagebreak -->

3

2

1

0

URS

UDIS

CEN

Res.

CCPC

CCUS

TI1S

4

OIS1

CCDS

OIS1N

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0
UIE
UIF

0

0

0

0

0

0

0

0
UG

CC1IE
CC1IF

0

CC1G

CC2IE
CC2IF

0

CC2G

CC3IE
CC3IF

0

CC3G

CC4IE
CC4IF

0

CC4G

COMIE
COMIF

0

COMG

TIE
TIF

0

TG

BIE
BIF

UDE

0

BG

CC1DE

0

B2IF

0

CC1OF

0

B2G

0

Res.

CC2DE

0

CC2OF

0

Res.

CC3DE

0

CC3OF

0

Res.

CC4DE

0

CC4OF

0

Res.

COMDE

0

SBIF

0

Res.

TDE

0

Res.

ETF[3:0]

0

Res.

Res.

Res.

0

CC5IF

SMS[2:0]

Res.

TS[2:0]

OCCS

OIS2

OPM

6

OIS2N

5

7

OIS3

DIR

8

OIS3N

0

MMS
[2:0]

MSM

OIS4

0

0

ECE

OIS4N

0

0

ETP

0

0

Res.

OIS5

0

0

SMS[3]

Res.
Res.

0

0

0

ETP
S
[1:0]

CC6IF

Res.
Res.

0

0

CMS
[1:0]

Res.

Res.

OIS6

Res.
Res.
Res.

0

0

0

0

RM0456 Rev 6

ARPE

0

0

0

Reset value

9

11
0

CKD
[1:0]

0

Res.

0

Res.

0

Res.

0

Res.

0

0

Res.

IDXF

0

Res.

0
DIRF

0

Res.

0

IERRF

IDXIE

0

DIRIE

0
IERRIE

Res.

TS
[4:3]

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

TIMx_EGR

0x014

Res.

Reset value

Res.

Res.
Res.

Res.

Res.

Res.

Res.

Res.

Res.

TIMx_SR

0x010

Res.

Reset value

0

TERRIE

0

0

TERRF

SMSPE

0

0

Res.

SMSPS

Res.

Res.

Res.

Res.

Res.

TIMx_DIER

0x00C

Res.

Reset value

MMS2[3:0]
0

Res.

Res.

Res.

Res.

Res.

TIMx_SMCR
0x008

Res.

0
Res.

Reset value

Res.

MMS[3]

Res.

Res.

Res.

Res.

Res.

Res.

TIMx_CR2

0x004

10

12

UIFREMA

0

Reset value

Res.

13

DITHEN

14

15

16

17

18

19

20

21

22

23

24

25

26

27

28

29

30

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

TIMx_CR1

0x000

Res.

Register name

Res.

Offset

31

Table 555. TIMx register map and reset values

0

0

0

0

0

0

0

0

0

RM0456

Advanced-control timers (TIM1/TIM8)

CC3P

CC3E

CC2NP
0

0

0

0

0

0

0

1

2

3

OC1FE
OC3FE

4

5
0

0

0

0

0

0

0

0

0

0

0

0

0

0

PSC[15:0]
0

0

0

0

0

0

0

0

Res.

Res.

Res.

1

1

1

1

1

1

1

1

0

0

0

0

0

0

0

0

1

1

1

1

1

1

1

1

REP[15:0]
0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

CCR1[19:0]
0

0

0

0

0

0

0

0

0

0

0

CCR2[19:0]
0

0

0

0

0

0

0

0

0

0

0

CCR3[19:0]
0

0

0

0

0

0

0

0

0

0

0

CCR4[19:0]
0

0

BKF[3:0]

0

0

0

RM0456 Rev 6

0

0

0

0

0

0

0

0

0

OSSI

0

OSSR

0

0

Res.

0

BKE

0

0

CNT[15:0]

BKP

0

CC1E

CC3NE

0

CC1P

CC3NP

0

CC1NE

CC4E

0

CC1NP

CC4P

0

AOE

0

OC1PE

OC4FE

CC4NE

0

MOE

BK2E
0

OC3PE

OC4PE

CC4NP

0

CC5E

0

Res.

0

CC5P

0

Res.

Res.

BK2P
0

0

CC3
S
[1:0]

0

Res.

Res.

Res.
Res.

Res.

Res.

Res.

BKDSRM
0

0

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

Res.
BK2DSRM
0

0

IC3
PSC
[1:0]

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
Res.

Res.
Res.
.BKBID
0

0

0

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

Res.

Res.
Res.

Res.
Res.
Res.
0

CC3
S
[1:0]

0

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

Res.

Res.
Res.

Res.
Res.
Res.
Res.
Res.

.BK2BID

Reset value

BK2F[3:0]

0

0

0

0

0
Res.

TIMx_BDTR

0

0

0

0

0
Res.

TIMx_CCR4

0

0

0

0

0
Res.

TIMx_CCR3

CC1
S
[1:0]

ARR[19:0]

0
Res.

TIMx_CCR2

0

0

0
Res.

TIMx_CCR1

0

0

0

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

Res.

Res.
Res.

Res.

Res.

Res.
Res.

Res.

TIMx_RCR

0

0

0

0
Res.

TIMx_ARR

0

IC3F[3:0]

Res.

Res.
Res.

Res.

CC6E
Res.

Res.

CC6P

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.
Res.
Res.

Res.

Res.

TIMx_PSC

0

CC4
S
[1:0]

0

0

0

0
Res.

0

Res.

UIFCPY

Reset value

Res.

TIMx_CNT

0

IC4F[3:0]

0

IC4
PSC
[1:0]

0

CC1
S
[1:0]

CC2E

16

OC2CE
OC4CE

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.
Res.

Res.

TIMx_CCER

0

6

17

Res.
OC1M[3]
OC3M[3]

Res.
Res.

Res.
Res.

Res.
Res.

Res.
Res.

Res.
Res.

Res.
Res.

Res.

Res.

OC4M[3]

Res.
Res.

Res.
Res.

Res.
Res.

Res.
Res.

Res.
Res.

Res.

Res.

0

Res.

0

0

0

IC1
PSC
[1:0]

CC2P

18

Res.

0

0

0

7

19

Res.

0

0

0

IC1F[3:0]

CC2NE

20

Res.

8

21

Res.

OC1CE

22

Res.

OC3CE

23

Res.

9

24

Res.

11

25

Res.

OC3M
[2:0]

0

0

Res.

OC2M[3]

10

26

Res.

0

OC2FE

27

Res.

12

28

Res.

0

OC2PE

29

Res.

0

0

0

0

Reset value

0x044

OC1M
[2:0]

CC2
S
[1:0]

0

Reset value

Reset value

0x040

0

CC4
S
[1:0]

Reset value

0x03C

0

OC4M
[2:0]

Reset value

0x038

0

TIMx_CCMR2

Reset value

0x034

0

0

Reset value

0x030

0

0

0

Reset value

0x02C

0

0

0

Reset value

0x028

0

CC2
S
[1:0]

0

Reset value

0x024

0

IC2
PSC
[1:0]

Reset value

TIMx_CCMR2

0x020

IC2F[3:0]

OC2M
[2:0]

TIMx_CCMR1
Output
Compare mode

Res.

0x01C

13

30

Res.

Reset value
0x018

14

31

TIMx_CCMR1
Input Capture
mode

15

Register name

Res.

Offset

Res.

Table 555. TIMx register map and reset values (continued)

LOC
K
[1:0]

0

0

0

0

0

0

0

0

DT[7:0]

0

0

0

0

0

<!-- pagebreak -->

2236

0x3E0

0x3DC
TIMx_DCR

Reset value

<!-- pagebreak -->

