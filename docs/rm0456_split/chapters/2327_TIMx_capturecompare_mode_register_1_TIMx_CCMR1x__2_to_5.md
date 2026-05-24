RM0456 Rev 6

RM0456

General-purpose timers (TIM2/TIM3/TIM4/TIM5)

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
1: Re-initialize the counter and generates an update of the registers. Note that the prescaler
counter is cleared too (anyway the prescaler ratio is not affected). The counter is cleared if
the center-aligned mode is selected or if DIR = 0 (up-counting), else it takes the autoreload
value (TIMx_ARR) if DIR = 1 (down-counting).

55.5.7

TIMx capture/compare mode register 1 (TIMx_CCMR1)(x = 2 to 5)
Address offset: 0x018
Reset value: 0x0000 0000
The same register can be used for input capture mode (this section) or for output compare
mode (next section). The direction of a channel is defined by configuring the corresponding
CCxS bits. All the other bits of this register have a different function for input capture and for
output compare modes. It is possible to combine both modes independently (for example
channel 1 in input capture mode and channel 2 in output compare mode).

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

Input capture mode
Bits 31:16 Reserved, must be kept at reset value.
Bits 15:12 IC2F[3:0]: Input capture 2 filter
Bits 11:10 IC2PSC[1:0]: Input capture 2 prescaler

RM0456 Rev 6

<!-- pagebreak -->

2350

General-purpose timers (TIM2/TIM3/TIM4/TIM5)

RM0456

Bits 9:8 CC2S[1:0]: Capture/compare 2 selection
This bitfield defines the direction of the channel (input/output) as well as the used input.
00: CC2 channel is configured as output.
01: CC2 channel is configured as input, tim_ic2 is mapped on tim_ti2.
10: CC2 channel is configured as input, tim_ic2 is mapped on tim_ti1.
11: CC2 channel is configured as input, tim_ic2 is mapped on tim_trc. This mode is working
only if an internal trigger input is selected through TS bit (TIMx_SMCR register)
Note: CC2S bits are writable only when the channel is OFF (CC2E = 0 in TIMx_CCER).
Bits 7:4 IC1F[3:0]: Input capture 1 filter
This bitfield defines the frequency used to sample tim_ti1 input and the length of the digital
filter applied to tim_ti1. The digital filter is made of an event counter in which N consecutive
events are needed to validate a transition on the output:
0000:No filter, sampling is done at fDTS
0001:fSAMPLING = ftim_ker_ck, N = 2
0010:fSAMPLING = ftim_ker_ck, N = 4
0011:fSAMPLING = ftim_ker_ck, N = 8
0100:fSAMPLING = fDTS/2, N = 6
0101:fSAMPLING = fDTS/2, N = 8
0110:fSAMPLING = fDTS/4, N = 6
0111:fSAMPLING = fDTS/4, N = 8
1000:fSAMPLING = fDTS/8, N = 6
1001:fSAMPLING = fDTS/8, N = 8
1010:fSAMPLING = fDTS/16, N = 5
1011:fSAMPLING = fDTS/16, N = 6
1100:fSAMPLING = fDTS/16, N = 8
1101:fSAMPLING = fDTS/32, N = 5
1110:fSAMPLING = fDTS/32, N = 6
1111:fSAMPLING = fDTS/32, N = 8
Bits 3:2 IC1PSC[1:0]: Input capture 1 prescaler
This bitfield defines the ratio of the prescaler acting on CC1 input (tim_ic1). The prescaler is
reset as soon as CC1E = 0 (TIMx_CCER register).
00: no prescaler, capture is done each time an edge is detected on the capture input
01: capture is done once every 2 events
10: capture is done once every 4 events
11: capture is done once every 8 events
Bits 1:0 CC1S[1:0]: Capture/Compare 1 selection
This bitfield defines the direction of the channel (input/output) as well as the used input.
00: CC1 channel is configured as output
01: CC1 channel is configured as input, tim_ic1 is mapped on tim_ti1
10: CC1 channel is configured as input, tim_ic1 is mapped on tim_ti2
11: CC1 channel is configured as input, tim_ic1 is mapped on tim_trc. This mode is working
only if an internal trigger input is selected through TS bit (TIMx_SMCR register)
Note: CC1S bits are writable only when the channel is OFF (CC1E = 0 in TIMx_CCER).

<!-- pagebreak -->

RM0456 Rev 6

RM0456

General-purpose timers (TIM2/TIM3/TIM4/TIM5)

55.5.8

TIMx capture/compare mode register 1 [alternate]
(TIMx_CCMR1)(x = 2 to 5)
Address offset: 0x018
Reset value: 0x0000 0000
The same register can be used for output compare mode (this section) or for input capture
mode (previous section). The direction of a channel is defined by configuring the
corresponding CCxS bits. All the other bits of this register have a different function for input
capture and for output compare modes. It is possible to combine both modes independently
(for example channel 1 in input capture mode and channel 2 in output compare mode).

31

30

29

28

27

26

25

24
OC2M
[3]

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

23

22

21

20

19

18

17

16
OC1M
[3]

Res.

Res.

Res.

Res.

Res.

Res.

Res.

7

6

5

4

3

2

1

rw

OC2CE
rw

OC2M[2:0]
rw

rw

OC2PE OC2FE
rw

rw

rw

CC2S[1:0]
rw

rw

8

OC1CE

rw

rw

OC1M[2:0]
rw

rw

OC1PE OC1FE
rw

rw

rw

0

CC1S[1:0]
rw

rw

Output compare mode
Bits 31:25 Reserved, must be kept at reset value.
Bits 23:17 Reserved, must be kept at reset value.
Bit 15 OC2CE: Output compare 2 clear enable
Bits 24, 14:12 OC2M[3:0]: Output compare 2 mode
refer to OC1M description on bits 6:4
Bit 11 OC2PE: Output compare 2 preload enable
Bit 10 OC2FE: Output compare 2 fast enable
Bits 9:8 CC2S[1:0]: Capture/Compare 2 selection
This bitfield defines the direction of the channel (input/output) as well as the used input.
00: CC2 channel is configured as output
01: CC2 channel is configured as input, tim_ic2 is mapped on tim_ti2
10: CC2 channel is configured as input, tim_ic2 is mapped on tim_ti1
11: CC2 channel is configured as input, tim_ic2 is mapped on tim_trc. This mode is working
only if an internal trigger input is selected through the TS bit (TIMx_SMCR register)
Note: CC2S bits are writable only when the channel is OFF (CC2E = 0 in TIMx_CCER).
Bit 7 OC1CE: Output compare 1 clear enable
0: tim_oc1ref is not affected by the tim_ocref_clr_int input
1: tim_oc1ref is cleared as soon as a High level is detected on tim_ocref_clr_int input

RM0456 Rev 6

<!-- pagebreak -->

2350

General-purpose timers (TIM2/TIM3/TIM4/TIM5)

RM0456

Bits 16, 6:4 OC1M[3:0]: Output compare 1 mode
These bits define the behavior of the output reference signal tim_oc1ref from which tim_oc1
is derived. tim_oc1ref is active high whereas tim_oc1 active level depends on CC1P bit.
0000: Frozen - The comparison between the output compare register TIMx_CCR1 and the
counter TIMx_CNT has no effect on the outputs. This mode can be used when the
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
0110: PWM mode 1 - In up-counting, channel 1 is active as long as
TIMx_CNT<TIMx_CCR1 else inactive. In down-counting, channel 1 is inactive
(tim_oc1ref = 0) as long as TIMx_CNT>TIMx_CCR1 else active (tim_oc1ref = 1).
0111: PWM mode 2 - In up-counting, channel 1 is inactive as long as
TIMx_CNT<TIMx_CCR1 else active. In down-counting, channel 1 is active as long as
TIMx_CNT>TIMx_CCR1 else inactive.
1000: Retriggerable OPM mode 1 - In up-counting mode, the channel is active until a trigger
event is detected (on tim_trgi signal). Then, a comparison is performed as in PWM
mode 1 and the channels becomes inactive again at the next update. In downcounting mode, the channel is inactive until a trigger event is detected (on tim_trgi
signal). Then, a comparison is performed as in PWM mode 1 and the channels
becomes inactive again at the next update.
1001: Retriggerable OPM mode 2 - In up-counting mode, the channel is inactive until a
trigger event is detected (on tim_trgi signal). Then, a comparison is performed as in
PWM mode 2 and the channels becomes inactive again at the next update. In downcounting mode, the channel is active until a trigger event is detected (on tim_trgi
signal). Then, a comparison is performed as in PWM mode 1 and the channels
becomes active again at the next update.
1010: Reserved.
1011: Reserved.
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
Note: In PWM mode, the OCREF level changes when the result of the comparison changes,
when the output compare mode switches from “frozen” mode to “PWM” mode and
when the output compare mode switches from “force active/inactive” mode to “PWM”
mode.

<!-- pagebreak -->

