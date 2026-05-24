RM0456 Rev 6

RM0456

General-purpose timers (TIM2/TIM3/TIM4/TIM5)

Bit 3 OC1PE: Output compare 1 preload enable
0: Preload register on TIMx_CCR1 disabled. TIMx_CCR1 can be written at anytime, the
new value is taken in account immediately.
1: Preload register on TIMx_CCR1 enabled. Read/Write operations access the preload
register. TIMx_CCR1 preload value is loaded in the active register at each update event.
Bit 2 OC1FE: Output compare 1 fast enable
This bit decreases the latency between a trigger event and a transition on the timer output.
It must be used in one-pulse mode (OPM bit set in TIMx_CR1 register), to have the output
pulse starting as soon as possible after the starting trigger.
0: CC1 behaves normally depending on counter and CCR1 values even when the trigger is
ON. The minimum delay to activate CC1 output when an edge occurs on the trigger input is
5 clock cycles.
1: An active edge on the trigger input acts like a compare match on CC1 output. Then, OC
is set to the compare level independently from the result of the comparison. Delay to sample
the trigger input and to activate CC1 output is reduced to three clock cycles. OCFE acts
only if the channel is configured in PWM1 or PWM2 mode.
Bits 1:0 CC1S[1:0]: Capture/Compare 1 selection
This bitfield defines the direction of the channel (input/output) as well as the used input.
00: CC1 channel is configured as output.
01: CC1 channel is configured as input, tim_ic1 is mapped on tim_ti1.
10: CC1 channel is configured as input, tim_ic1 is mapped on tim_ti2.
11: CC1 channel is configured as input, tim_ic1 is mapped on tim_trc. This mode is working
only if an internal trigger input is selected through TS bit (TIMx_SMCR register)
Note: CC1S bits are writable only when the channel is OFF (CC1E = 0 in TIMx_CCER).

55.5.9

TIMx capture/compare mode register 2 (TIMx_CCMR2)(x = 2 to 5)
Address offset: 0x01C
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
rw

rw

IC3PSC[1:0]

CC3S[1:0]

rw

rw

rw

rw

Input capture mode
Bits 31:16 Reserved, must be kept at reset value.
Bits 15:12 IC4F[3:0]: Input capture 4 filter
Bits 11:10 IC4PSC[1:0]: Input capture 4 prescaler

RM0456 Rev 6

<!-- pagebreak -->

2350

General-purpose timers (TIM2/TIM3/TIM4/TIM5)

RM0456

Bits 9:8 CC4S[1:0]: Capture/Compare 4 selection
This bitfield defines the direction of the channel (input/output) as well as the used input.
00: CC4 channel is configured as output
01: CC4 channel is configured as input, tim_ic4 is mapped on tim_ti4
10: CC4 channel is configured as input, tim_ic4 is mapped on tim_ti3
11: CC4 channel is configured as input, tim_ic4 is mapped on tim_trc. This mode is working
only if an internal trigger input is selected through TS bit (TIMx_SMCR register)
Note: CC4S bits are writable only when the channel is OFF (CC4E = 0 in TIMx_CCER).
Bits 7:4 IC3F[3:0]: Input capture 3 filter
Bits 3:2 IC3PSC[1:0]: Input capture 3 prescaler
Bits 1:0 CC3S[1:0]: Capture/Compare 3 selection
This bitfield defines the direction of the channel (input/output) as well as the used input.
00: CC3 channel is configured as output
01: CC3 channel is configured as input, tim_ic3 is mapped on tim_ti3
10: CC3 channel is configured as input, tim_ic3 is mapped on tim_ti4
11: CC3 channel is configured as input, tim_ic3 is mapped on tim_trc. This mode is working
only if an internal trigger input is selected through TS bit (TIMx_SMCR register)
Note: CC3S bits are writable only when the channel is OFF (CC3E = 0 in TIMx_CCER).

55.5.10

TIMx capture/compare mode register 2 [alternate]
(TIMx_CCMR2)(x = 2 to 5)
Address offset: 0x01C
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
OC4M
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
OC3M
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

OC4CE
rw

OC4M[2:0]
rw

rw

OC4PE OC4FE
rw

rw

rw

CC4S[1:0]
rw

rw

8

OC3CE

rw

rw

Output compare mode
Bits 31:25 Reserved, must be kept at reset value.
Bits 23:17 Reserved, must be kept at reset value.
Bit 15 OC4CE: Output compare 4 clear enable
Bits 24, 14:12 OC4M[3:0]: Output compare 4 mode
Refer to OC3M[3:0]
Bit 11 OC4PE: Output compare 4 preload enable
Bit 10 OC4FE: Output compare 4 fast enable

<!-- pagebreak -->

RM0456 Rev 6

OC3M[2:0]
rw

rw

OC3PE OC3FE
rw

rw

rw

0

CC3S[1:0]
rw

rw

RM0456

General-purpose timers (TIM2/TIM3/TIM4/TIM5)

Bits 9:8 CC4S[1:0]: Capture/Compare 4 selection
This bitfield defines the direction of the channel (input/output) as well as the used input.
00: CC4 channel is configured as output
01: CC4 channel is configured as input, tim_ic4 is mapped on tim_ti4
10: CC4 channel is configured as input, tim_ic4 is mapped on tim_ti3
11: CC4 channel is configured as input, tim_ic4 is mapped on tim_trc. This mode is working
only if an internal trigger input is selected through TS bit (TIMx_SMCR register)
Note: CC4S bits are writable only when the channel is OFF (CC4E = 0 in TIMx_CCER).
Bit 7 OC3CE: Output compare 3 clear enable

RM0456 Rev 6

<!-- pagebreak -->

2350

General-purpose timers (TIM2/TIM3/TIM4/TIM5)

RM0456

Bits 16, 6:4 OC3M[3:0]: Output compare 3 mode
These bits define the behavior of the output reference signal tim_oc3ref from which tim_oc3
and tim_oc3n are derived. tim_oc3ref is active high whereas tim_oc3 and tim_oc3n active
level depends on CC3P and CC3NP bits.
0000:Frozen - The comparison between the output compare register TIMx_CCR3 and the
counter TIMx_CNT has no effect on the outputs.(this mode is used to generate a
timing base).
0001:Set channel 3 to active level on match. tim_oc3ref signal is forced high when the
counter TIMx_CNT matches the capture/compare register 3 (TIMx_CCR3).
0010:Set channel 3 to inactive level on match. tim_oc3ref signal is forced low when the
counter TIMx_CNT matches the capture/compare register 3 (TIMx_CCR3).
0011:Toggle - tim_oc3ref toggles when TIMx_CNT = TIMx_CCR3.
0100:Force inactive level - tim_oc3ref is forced low.
0101:Force active level - tim_oc3ref is forced high.
0110:PWM mode 1 - In up-counting, channel 3 is active as long as TIMx_CNT<TIMx_CCR3
else inactive. In down-counting, channel 3 is inactive (tim_oc3ref = 0) as long as
TIMx_CNT>TIMx_CCR3 else active (tim_oc3ref = 1).
0111:PWM mode 2 - In up-counting, channel 3 is inactive as long as
TIMx_CNT<TIMx_CCR3 else active. In down-counting, channel 3 is active as long as
TIMx_CNT>TIMx_CCR3 else inactive.
1000:Retrigerrable OPM mode 1 - In up-counting mode, the channel is active until a trigger
event is detected (on tim_trgi signal). Then, a comparison is performed as in PWM
mode 1 and the channels becomes active again at the next update. In down-counting
mode, the channel is inactive until a trigger event is detected (on tim_trgi signal).
Then, a comparison is performed as in PWM mode 1 and the channels becomes
inactive again at the next update.
1001:Retrigerrable OPM mode 2 - In up-counting mode, the channel is inactive until a trigger
event is detected (on tim_trgi signal). Then, a comparison is performed as in PWM
mode 2 and the channels becomes inactive again at the next update. In downcounting mode, the channel is active until a trigger event is detected (on tim_trgi
signal). Then, a comparison is performed as in PWM mode 1 and the channels
becomes active again at the next update.
1010:Pulse on compare: a pulse is generated on tim_oc3ref upon CCR3 match event, as per
PWPRSC[2:0] and PW[7:0] bitfields programming in TIMxECR.
1011:Direction output. The tim_oc3ref signal is overridden by a copy of the DIR bit.
1100:Combined PWM mode 1 - tim_oc3ref has the same behavior as in PWM mode 1.
tim_oc3refc is the logical OR between tim_oc3ref and tim_oc4ref.
1101: Combined PWM mode 2 - tim_oc3ref has the same behavior as in PWM mode 2.
tim_oc3refc is the logical AND between tim_oc3ref and tim_oc4ref.
1110:Asymmetric PWM mode 1 - tim_oc3ref has the same behavior as in PWM mode 1.
tim_oc3refc outputs tim_oc3ref when the counter is counting up, tim_oc4ref when it is
counting down.
1111:Asymmetric PWM mode 2 - tim_oc3ref has the same behavior as in PWM mode 2.
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

<!-- pagebreak -->

RM0456 Rev 6

RM0456

General-purpose timers (TIM2/TIM3/TIM4/TIM5)

Bit 3 OC3PE: Output compare 3 preload enable
Bit 2 OC3FE: Output compare 3 fast enable
Bits 1:0 CC3S[1:0]: Capture/Compare 3 selection
This bitfield defines the direction of the channel (input/output) as well as the used input.
00: CC3 channel is configured as output
01: CC3 channel is configured as input, tim_ic3 is mapped on tim_ti3
10: CC3 channel is configured as input, tim_ic3 is mapped on tim_ti4
11: CC3 channel is configured as input, tim_ic3 is mapped on tim_trc. This mode is working
only if an internal trigger input is selected through TS bit (TIMx_SMCR register)
Note: CC3S bits are writable only when the channel is OFF (CC3E = 0 in TIMx_CCER).

55.5.11

TIMx capture/compare enable register (TIMx_CCER)(x = 2 to 5)
Address offset: 0x020
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

CC4NP

Res.

CC4P

CC4E

CC3NP

Res.

CC3P

CC3E

CC2NP

Res.

CC2P

CC2E

CC1NP

Res.

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

rw

rw

rw

rw

Bit 15 CC4NP: Capture/Compare 4 output Polarity.
Refer to CC1NP description
Bit 14 Reserved, must be kept at reset value.
Bit 13 CC4P: Capture/Compare 4 output Polarity.
Refer to CC1P description
Bit 12 CC4E: Capture/Compare 4 output enable.
refer to CC1E description
Bit 11 CC3NP: Capture/Compare 3 output Polarity.
Refer to CC1NP description
Bit 10 Reserved, must be kept at reset value.
Bit 9 CC3P: Capture/Compare 3 output Polarity.
Refer to CC1P description
Bit 8 CC3E: Capture/Compare 3 output enable.
Refer to CC1E description
Bit 7 CC2NP: Capture/Compare 2 output Polarity.
Refer to CC1NP description
Bit 6 Reserved, must be kept at reset value.
Bit 5 CC2P: Capture/Compare 2 output Polarity.
refer to CC1P description
Bit 4 CC2E: Capture/Compare 2 output enable.
Refer to CC1E description

RM0456 Rev 6

<!-- pagebreak -->

2350

General-purpose timers (TIM2/TIM3/TIM4/TIM5)

RM0456

Bit 3 CC1NP: Capture/Compare 1 output Polarity.
CC1 channel configured as output: CC1NP must be kept cleared in this case.
CC1 channel configured as input: This bit is used in conjunction with CC1P to define
tim_ti1fp1/tim_ti2fp1 polarity. refer to CC1P description.
Bit 2 Reserved, must be kept at reset value.
Bit 1 CC1P: Capture/Compare 1 output Polarity.
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
CC1NP = 1, CC1P = 1:non-inverted/both edges. The circuit is sensitive to both TIxFP1
rising and falling edges (capture or trigger operations in reset,
external clock or trigger mode), TIxFP1is not inverted (trigger
operation in gated mode). This configuration must not be used in
encoder mode.
CC1NP = 1, CC1P = 0:this configuration is reserved, it must not be used.
Bit 0 CC1E: Capture/Compare 1 output enable.
0: Capture mode disabled / OC1 is not active
1: Capture mode enabled / OC1 signal is output on the corresponding output pin

Table 574. Output control bit for standard tim_ocx channels
CCxE bit

Note:

<!-- pagebreak -->

tim_ocx output state

0

Output disabled (not driven by the timer: Hi-Z)

1

Output enabled (tim_ocx = tim_ocxref + Polarity)

The state of the external IO pins connected to the standard tim_ocx channels depends only
on the GPIO registers when CCxE = 0.

RM0456 Rev 6

RM0456

General-purpose timers (TIM2/TIM3/TIM4/TIM5)

55.5.12

TIMx counter (TIMx_CNT)(x = 2 to 5)
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

UIF
CPY_
CNT
[31]

23

22

21

20

19

18

17

16

CNT[30:16]

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

rw

CNT[15:0]
rw

Bit 31 UIFCPY_CNT[31]: Value depends on IUFREMAP in TIMx_CR1.
If UIFREMAP = 0
CNT[31]: Most significant bit of counter value
If UIFREMAP = 1
UIFCPY: UIF Copy
This bit is a read-only copy of the UIF bit of the TIMx_ISR register
Bits 30:0 CNT[30:0]: Least significant part of counter value
Non-dithering mode (DITHEN = 0)
The register holds the counter value.
Dithering mode (DITHEN = 1)
The register holds the non-dithered part in CNT[30:0]. The fractional part is not available.

55.5.13

TIMx prescaler (TIMx_PSC)(x = 2 to 5)
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

rw

rw

rw

rw

rw

rw

rw

rw

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

Bits 15:0 PSC[15:0]: Prescaler value
The counter clock frequency tim_cnt_ck is equal to ftim_psc_ck / (PSC[15:0] + 1).
PSC contains the value to be loaded in the active prescaler register at each update event
(including when the counter is cleared through UG bit of TIMx_EGR register or through
trigger controller when configured in “reset mode”).

RM0456 Rev 6

<!-- pagebreak -->

2350

General-purpose timers (TIM2/TIM3/TIM4/TIM5)

55.5.14

RM0456

TIMx autoreload register (TIMx_ARR)(x = 2 to 5)
Address offset: 0x02C
Reset value: 0xFFFF FFFF

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

ARR[31:16]
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

rw

ARR[15:0]
rw

Bits 31:0 ARR[31:0]: Autoreload value
ARR is the value to be loaded in the actual autoreload register.
Refer to the Section 55.4.3: Time-base unit for more details about ARR update and
behavior.
The counter is blocked while the autoreload value is null.
Non-dithering mode (DITHEN = 0)
The register holds the autoreload value.
Dithering mode (DITHEN = 1)
The register holds the integer part in ARR[31:4]. The ARR[3:0] bitfield contains the dithered
part.

55.5.15

TIMx capture/compare register 1 (TIMx_CCR1)(x = 2 to 5)
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

23

22

21

20

19

18

17

16

rw

rw

rw

rw

rw

rw

rw

rw

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

CCR1[31:16]

CCR1[15:0]
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

General-purpose timers (TIM2/TIM3/TIM4/TIM5)

Bits 31:0 CCR1[31:0]: Capture/compare 1 value
If channel CC1 is configured as output:
CCR1 is the value to be loaded in the actual capture/compare 1 register (preload value). It is
loaded permanently if the preload feature is not selected in the TIMx_CCMR1 register (bit
OC1PE). Else the preload value is copied in the active capture/compare 1 register when an
update event occurs.
The active capture/compare register contains the value to be compared to the counter
TIMx_CNT and signaled on tim_oc1 output.
Non-dithering mode (DITHEN = 0)
The register holds the compare value.
Dithering mode (DITHEN = 1)
The register holds the integer part in CCR1[31:4]. The CCR1[3:0] bitfield contains the
dithered part.
If channel CC1 is configured as input:
CCR1 is the counter value transferred by the last input capture 1 event (tim_ic1). The
TIMx_CCR1 register is read-only and cannot be programmed.
Non-dithering mode (DITHEN = 0)
The register holds the capture value.
Dithering mode (DITHEN = 1)
The register holds the capture in CCR1[31:0]. The CCR1[3:0] bits are reset.

55.5.16

TIMx capture/compare register 2 (TIMx_CCR2)(x = 2 to 5)
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

19

18

17

16

CCR2[31:16]
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

CCR2[15:0]
rw

rw

RM0456 Rev 6

<!-- pagebreak -->

2350

General-purpose timers (TIM2/TIM3/TIM4/TIM5)

RM0456

Bits 31:0 CCR2[31:0]: Capture/compare 2 value
If channel CC2 is configured as output:
CCR2 is the value to be loaded in the actual capture/compare 2 register (preload value). It is
loaded permanently if the preload feature is not selected in the TIMx_CCMR2 register (bit
OC2PE). Else the preload value is copied in the active capture/compare 2 register when an
update event occurs.
The active capture/compare register contains the value to be compared to the counter
TIMx_CNT and signaled on tim_oc2 output.
Non-dithering mode (DITHEN = 0)
The register holds the compare value.
Dithering mode (DITHEN = 1)
The register holds the integer part in CCR2[31:4]. The CCR2[3:0] bitfield contains the
dithered part.
If channel CC2 is configured as input:
CCR2 is the counter value transferred by the last input capture 2 event (tim_ic2). The
TIMx_CCR2 register is read-only and cannot be programmed.
Non-dithering mode (DITHEN = 0)
The register holds the capture value.
Dithering mode (DITHEN = 1)
The register holds the capture in CCR2[31:0]. The CCR2[3:0] bits are reset.

55.5.17

TIMx capture/compare register 3 (TIMx_CCR3)(x = 2 to 5)
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

19

18

17

16

CCR3[31:16]
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

CCR3[15:0]

<!-- pagebreak -->

rw

rw

RM0456 Rev 6

RM0456

General-purpose timers (TIM2/TIM3/TIM4/TIM5)

Bits 31:0 CCR3[31:0]: Capture/compare 3 value
If channel CC3 is configured as output:
CCR3 is the value to be loaded in the actual capture/compare 3 register (preload value). It is
loaded permanently if the preload feature is not selected in the TIMx_CCMR3 register (bit
OC3PE). Else the preload value is copied in the active capture/compare 3 register when an
update event occurs.
The active capture/compare register contains the value to be compared to the counter
TIMx_CNT and signaled on tim_oc3 output.
Non-dithering mode (DITHEN = 0)
The register holds the compare value.
Dithering mode (DITHEN = 1)
The register holds the integer part in CCR3[31:4]. The CCR3[3:0] bitfield contains the
dithered part.
If channel CC3 is configured as input:
CCR3 is the counter value transferred by the last input capture 3 event (tim_ic3). The
TIMx_CCR3 register is read-only and cannot be programmed.
Non-dithering mode (DITHEN = 0)
The register holds the capture value.
Dithering mode (DITHEN = 1)
The register holds the capture in CCR3[31:0]. The CCR3[3:0] bits are reset.

55.5.18

TIMx capture/compare register 4 (TIMx_CCR4)(x = 2 to 5)
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

19

18

17

16

CCR4[31:16]
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

CCR4[15:0]
rw

rw

RM0456 Rev 6

<!-- pagebreak -->

2350

General-purpose timers (TIM2/TIM3/TIM4/TIM5)

RM0456

Bits 31:0 CCR4[31:0]: Capture/compare 4 value
If channel CC4 is configured as output:
CCR4 is the value to be loaded in the actual capture/compare 4 register (preload value). It is
loaded permanently if the preload feature is not selected in the TIMx_CCMR4 register (bit
OC4PE). Else the preload value is copied in the active capture/compare 4 register when an
update event occurs.
The active capture/compare register contains the value to be compared to the counter
TIMx_CNT and signaled on tim_oc4 output.
Non-dithering mode (DITHEN = 0)
The register holds the compare value.
Dithering mode (DITHEN = 1)
The register holds the integer part in CCR4[31:4]. The CCR4[3:0] bitfield contains the
dithered part.
If channel CC4 is configured as input:
CCR4 is the counter value transferred by the last input capture 4 event (tim_ic4). The
TIMx_CCR4 register is read-only and cannot be programmed.
Non-dithering mode (DITHEN = 0)
The register holds the capture value.
Dithering mode (DITHEN = 1)
The register holds the capture in CCR4[31:0]. The CCR4[3:0] bits are reset.

55.5.19

TIMx timer encoder control register (TIMx_ECR)(x = 2 to 5)
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

Bits 31:27 Reserved, must be kept at reset value.
Bits 26:24 PWPRSC[2:0]: Pulse width prescaler

This bitfield sets the clock prescaler for the pulse generator, as following:
tPWG = (2(PWPRSC[2:0])) x ttim_ker_ck
Bits 23:16 PW[7:0]: Pulse width
This bitfield defines the pulse duration, as following:

tPW = PW[7:0] x tPWG
Bits 15:8 Reserved, must be kept at reset value.

<!-- pagebreak -->

RM0456 Rev 6

rw

RM0456

General-purpose timers (TIM2/TIM3/TIM4/TIM5)

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
Bits 4:3 IBLK[1:0]: Index blanking
This bit indicates if the Index event is conditioned by the tim_ti3 input
00: Index always active
01: Index disabled hen tim_ti3 input is active, as per CC3P bitfield
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

55.5.20

TIMx timer input selection register (TIMx_TISEL)(x = 2 to 5)
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

27

rw
15

14

13

12

Res.

Res.

Res.

Res.

26

25

24

TI4SEL[3:0]

11

rw

rw

rw

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

19

rw
7

6

5

4

Res.

Res.

Res.

Res.

rw

18

17

16

TI3SEL[3:0]

3

rw

rw

rw

2

1

0

TI1SEL[3:0]
rw

rw

rw

rw

Bits 31:28 Reserved, must be kept at reset value.

RM0456 Rev 6

<!-- pagebreak -->

2350

General-purpose timers (TIM2/TIM3/TIM4/TIM5)

RM0456

Bits 27:24 TI4SEL[3:0]: Selects tim_ti4[15:0] input
0000: tim_ti4_in0: TIMx_CH4
0001: tim_ti4_in1
...
1111: tim_ti4_in15
Refer to Section 55.4.2: TIM2/TIM3/TIM4/TIM5 pins and internal signals for product specific
implementation.
Bits 23:20 Reserved, must be kept at reset value.
Bits 19:16 TI3SEL[3:0]: Selects tim_ti3[15:0] input
0000: tim_ti3_in0: TIMx_CH3
0001: tim_ti3_in1
...
1111: tim_ti3_in15
Refer to Section 55.4.2: TIM2/TIM3/TIM4/TIM5 pins and internal signals for product specific
implementation.
Bits 15:12 Reserved, must be kept at reset value.
Bits 11:8 TI2SEL[3:0]: Selects tim_ti2[15:0] input
0000: tim_ti2_in0: TIMx_CH2
0001: tim_ti2_in1
...
1111: tim_ti2_in15
Refer to Section 55.4.2: TIM2/TIM3/TIM4/TIM5 pins and internal signals for product specific
implementation.
Bits 7:4 Reserved, must be kept at reset value.
Bits 3:0 TI1SEL[3:0]: Selects tim_ti1[15:0] input
0000: tim_ti1_in0: TIMx_CH1
0001: tim_ti1_in1
...
1111: tim_ti1_in15
Refer to Section 55.4.2: TIM2/TIM3/TIM4/TIM5 pins and internal signals for product specific
implementation.

55.5.21

TIMx alternate function register 1 (TIMx_AF1)(x = 2 to 5)
Address offset: 0x060
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
rw

15

16

rw

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

ETRSEL[1:0]

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

rw

rw

<!-- pagebreak -->

RM0456 Rev 6

RM0456

General-purpose timers (TIM2/TIM3/TIM4/TIM5)

Bits 31:18 Reserved, must be kept at reset value.
Bits 17:14 ETRSEL[3:0]: etr_in source selection
These bits select the etr_in input source.
0000: tim_etr0: TIMx_ETR input
0001: tim_etr1
...
1111: tim_etr15
Refer to Section 55.4.2: TIM2/TIM3/TIM4/TIM5 pins and internal signals for product specific
implementation.
Bits 13:0 Reserved, must be kept at reset value.

55.5.22

TIMx alternate function register 2 (TIMx_AF2)(x = 2 to 5)
Address offset: 0x064
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

18

17

16

OCRSEL[2:0]
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

Bits 31:19 Reserved, must be kept at reset value.
Bits 18:16 OCRSEL[2:0]: ocref_clr source selection
These bits select the ocref_clr input source.
000: tim_ocref_clr0
001: tim_ocref_clr1
...
111: tim_ocref_clr7
Refer to Section 55.4.2: TIM2/TIM3/TIM4/TIM5 pins and internal signals for product specific
implementation.
Bits 15:0 Reserved, must be kept at reset value.

RM0456 Rev 6

<!-- pagebreak -->

2350

General-purpose timers (TIM2/TIM3/TIM4/TIM5)

55.5.23

RM0456

TIMx DMA control register (TIMx_DCR)(x = 2 to 5)
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

DBL[4:0]
rw

19

18

17

DBA[4:0]
rw

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

<!-- pagebreak -->

RM0456 Rev 6

16

DBSS[3:0]

RM0456

General-purpose timers (TIM2/TIM3/TIM4/TIM5)

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
00000: TIMx_CR1,
00001: TIMx_CR2,
00010: TIMx_SMCR,
...

55.5.24

TIMx DMA address for full transfer (TIMx_DMAR)(x = 2 to 5)
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

DMAB[15:0]
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

2350

General-purpose timers (TIM2/TIM3/TIM4/TIM5)

RM0456

Bits 31:0 DMAB[31:0]: DMA register for burst accesses
A read or write operation to the DMAR register accesses the register located at the address
(TIMx_CR1 address) + (DBA + DMA index) x 4
where TIMx_CR1 address is the address of the control register 1, DBA is the DMA base
address configured in TIMx_DCR register, DMA index is automatically controlled by the
DMA transfer, and ranges from 0 to DBL (DBL configured in TIMx_DCR).

55.5.25

TIMx register map
TIMx registers are mapped as described in the table below.

7

6

5

4

3

2

1

0

ARPE

DIR

OPM

URS

UDIS

CEN
0

MMS[2:0]

CCDS

Res.

Res.

Res.

0

0

0

0
Res.

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

<!-- pagebreak -->

TIE

Res.

0

0

CC2IF

CC1IF

UIF

0

0

CC3G

CC2G

CC1G

UG

0

0

0

0

0

0

0

0

OC2M
[2:0]

OC2FE

CC2S
[1:0]

OC1CE

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

IC4
PSC
[1:0]

CC4S
[1:0]

0

0

0

0

0

0

OC4M
[2:0]

CC4S
[1:0]

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

OC1M
[2:0]
0

OC3CE

OC2CE

0

0

0

0

IC3F[3:0]
0

0

0

OC3M
[2:0]
0

0

0

CC1S
[1:0]

0

0

0

CC1S
[1:0]

0

0

0

0

0

IC3
PSC
[1:0]

CC3S
[1:0]

0

0

0

OC3FE

Res.

TG

Res.

0

IC4F[3:0]

O24CE

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

RM0456 Rev 6

0

OC4FE

OC1M[3]

0

0

0

0

0

IC1
PSC
[1:0]

OC1FE

CC3IF

0

OC1PE

CC4IF

0

CC4G

Res.

0

IC1F[3:0]

OC4PE

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

0

OC3M[3]

Reset value

0

0

Reset value
TIMx_CCMR2
Output Compare
mode

CC2S
[1:0]

OC2PE

Res.

Res.

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

TIMx_CCMR2
Input Capture
mode

0
Res.

Reset value

0x01C

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

Reset value
TIMx_CCMR1
Output Compare
mode

0

0

IC2
PSC
[1:0]

IC2F[3:0]

0

0

0

Res.

0x018

TIF

UDE
Res.

0
Res.

0

0

0

0
Res.

CC1DE
CC1OF

0

Res.

CC2DE
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

Res.
Res.

Res.
Res.

Res.
Res.

Res.

0

0

UIE

0

CC1IE

0

CC2IE

0

CC3IE

0

SMS[2:0]

CC4IE

MSM

0

ETF[3:0]

Reset value
TIMx_CCMR1
Input Capture
mode

0

OC3PE

8
Res.

0

TI1S

9

Res.

Res.

0

ECE

Res.

11

0

TDE

Res.

0

0

ETP

Res.

0

Res.

Res.

0

SMS[3]

Res.

0

TS[2:0]

Res.

Res.
Res.

Res.
Res.

0

CMS
[1:0]

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

Res.

0

Res.

0

Res.

IDXIE
IDXF

0

CKD
[1:0]

ETPS
[1:0]

0

Res.

Res.

Res.

0

DIRF

0

Res.

DIRIE

Res.

Res.
Res.
0
IERRF

0

Res.

IERRIE

0

TERRF

Res.

Res.

Res.

Res.

Res.

TIMx_EGR

Res.

0x014

Res.

Reset value

0

Res.

Res.

Res.

Res.

Res.

Res.

Res.

TIMx_SR

Res.

Reset value

0x010

Res.
SMSPE
0

TS
[4:3]

TERRIE

SMSPS

Res.

Res.

Res.

Res.

Res.

Res.

TIMx_DIER

0
Res.

Res.

Res.

Res.

Res.

TIMx_SMCR
Reset value

0x00C

MMS[3]

Res.

Res.

Res.

Res.

Res.

0
Res.

Reset value
Res.

0x008

Res.

TIMx_CR2

0x004

10

UIFREMA
0
Res.

12
DITHEN
0

Reset value

Res.

13
Res.

15

16

17

18

19

20

14
Res.

Res.

Res.

Res.

Res.

Res.

Res.

21
Res.

22
Res.

23
Res.

25

26

27

24
Res.

Res.

Res.

Res.

28
Res.

30

29
Res.

TIMx_CR1

0x000

Res.

Register name

Res.

Offset

31

Table 575. TIM2/TIM3/TIM4/TIM5 register map and reset values

CC3S
[1:0]

0

0

0

0

0

RM0456

General-purpose timers (TIM2/TIM3/TIM4/TIM5)

TIMx_PSC

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

1

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

IPOS
[1:0]

IBLK
[1:0]

IDIR
[1:0]

IE

0

0

0

0

0

0

0

TI1SEL[3:0]

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

TI4SEL[3:0]

Res.

Res.

Res.

TI3SEL[3:0]
0

Res.

Res.

Res.

Res.

Res.

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

0

0

0

Res.

ETRSEL
[3:0]

0

TI2SEL[3:0]
0

Res.
0

Res.

0

Res.

Res.

0

0

Res.

Res.

Res.
Res.

Res.
Res.

Res.

0

Res.

Res.

0

Res.

Res.

Res.

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

0

0

Res.

0

Res.

0

Res.

0

TIMx_AF2

Res.

Res.

Res.

0

0

0

Res.

0

Res.

Res.

Res.

Res.

Res.

0

PW[7:0]

Res.

PWPRSC
[2:0]

Res.

Res.

Res.

Res.

Res.

Res.

Res.

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

Reserved

Reset value

0

CCR4[31:20]
(32-bit timers only)

OCRSEL[
2:0]

TIMx_DCR

0

CCR3[19:0]

Reset value

TIMx_AF1

0

CCR3[31:20]
(32-bit timers only)

Reserved

TIMx_TISEL

0

CCR2[19:0]

Reset value

0x3DC

1

0

CCR2[31:20]
(32-bit timers only)

Reset value

0x068..
0x3D8

1

0

Res.

1

0

Res.

1

Reset value

0x064

0

FIDX

0

TIMx_CCR4

TIMx_ECR

0x060

0

Res.

1

0

CCR1[31:20]
(32-bit timers only)

TIMx_CCR3

0x058

0x05C

0

Res.

1

TIMx_CCR2

Reset value
0x044..
0x054

0

Res.

1

TIMx_CCR1

Reset value

0x040

0

Reserved

Reset value

0x03C

0

PSC[15:0]

ARR[31:0]

Reset value

0x038

0

0

0x030

0x034

0

TIMx_ARR
(x = 2 to 5)
1

0

CNT[15:0]

Reset value

Reset value

CC1E

11
CC3NP

0

1

12
CC4E

0

CC1P

13
CC4P

0

2

14
Res.

0

Res.

15
CC4NP

0

3

16
Res.

0

CC1NP

17
Res.

0

4

18
Res.

0

CC2E

19
Res.

0

5

20
Res.

0

CC2P

21
Res.

0

6

22
Res.

0

Res.

23
Res.

0

7

24
Res.

0

CC2NP

25
Res.

0

8

26
Res.

UIFCPY_CNT[31]
0

0

CNT[30:16]
(CNT[31:16] on 32-bit timers only)

DBSS[3:0]
0

0

0

RM0456 Rev 6

0

Res.

0x02C

Reset value

Res.

0x028

TIMx_CNT

Res.

0x024

CC3E

27
Res.

0

9

28
Res.

0

0

Res.

29
Res.

0

Reset value

CC3P

30

TIMx_CCER

10

31

0x020

Register name

Res.

Offset

Res.

Table 575. TIM2/TIM3/TIM4/TIM5 register map and reset values (continued)

DBL[4:0]
0

0

0

0

0

DBA[4:0]
0

0

0

0

0

<!-- pagebreak -->

2350

General-purpose timers (TIM2/TIM3/TIM4/TIM5)

RM0456

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

0

11

0

10

14

15

12

Reset value

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

TIMx_DMAR

13

0x3E0

Register name

30

Offset

31

Table 575. TIM2/TIM3/TIM4/TIM5 register map and reset values (continued)

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

DMAB[31:0]
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

Refer to Section 2.3: Memory organization for the register boundary addresses.

<!-- pagebreak -->

