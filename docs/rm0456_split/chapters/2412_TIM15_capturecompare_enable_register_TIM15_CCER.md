RM0456 Rev 6

RM0456

Low-power timer (LPTIM)

Bits 2:1 CKPOL[1:0]: Clock Polarity
When the LPTIM is clocked by an external clock source, CKPOL bits is used to configure the active
edge or edges used by the counter:
00:the rising edge is the active edge used for counting.
If the LPTIM is configured in Encoder mode (ENC bit is set), the encoder sub-mode 1 is active.
01:the falling edge is the active edge used for counting.
If the LPTIM is configured in Encoder mode (ENC bit is set), the encoder sub-mode 2 is active.
10:both edges are active edges. When both external clock signal edges are considered active ones,
the LPTIM must also be clocked by an internal clock source with a frequency equal to at least
four times the external clock frequency.
If the LPTIM is configured in Encoder mode (ENC bit is set), the encoder sub-mode 3 is active.
11:not allowed
Refer to Section 58.4.15: Encoder mode for more details about Encoder mode sub-modes.
Bit 0 CKSEL: Clock selector
The CKSEL bit selects which clock source the LPTIM uses:
0: LPTIM is clocked by internal clock source (APB clock or any of the embedded oscillators)
1: LPTIM is clocked by an external clock source through the LPTIM external Input1

Caution:

The LPTIM_CFGR register must only be modified when the LPTIM is disabled (ENABLE bit
reset to 0).

58.7.11

LPTIM control register (LPTIM_CR)
Address offset: 0x010
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

Res.

RST
ARE

COUN
TRST

CNT
STRT

SNG
STRT

ENA
BLE

rw

rs

rw

rw

rw

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

Bits 31:5 Reserved, must be kept at reset value.
Bit 4 RSTARE: Reset after read enable
This bit is set and cleared by software. When RSTARE is set to '1', any read access to LPTIM_CNT
register asynchronously resets LPTIM_CNT register content.
This bit can be set only when the LPTIM is enabled.
Bit 3 COUNTRST: Counter reset
This bit is set by software and cleared by hardware. When set to '1' this bit triggers a synchronous
reset of the LPTIM_CNT counter register. Due to the synchronous nature of this reset, it only takes
place after a synchronization delay of 3 LPTimer core clock cycles (LPTimer core clock can be
different from APB clock).
This bit can be set only when the LPTIM is enabled. It is automatically reset by hardware.
Caution: COUNTRST must never be set to '1' by software before it is already cleared to '0' by
hardware. Software must consequently check that COUNTRST bit is already cleared to '0'
before attempting to set it to '1'.

RM0456 Rev 6

<!-- pagebreak -->

2526

Low-power timer (LPTIM)

RM0456

Bit 2 CNTSTRT: Timer start in Continuous mode
This bit is set by software and cleared by hardware.
In case of software start (TRIGEN[1:0] = 00), setting this bit starts the LPTIM in Continuous mode.
If the software start is disabled (TRIGEN[1:0] different than 00), setting this bit starts the timer in
Continuous mode as soon as an external trigger is detected.
If this bit is set when a single pulse mode counting is ongoing, then the timer does not stop at the
next match between the LPTIM_ARR and LPTIM_CNT registers and the LPTIM counter keeps
counting in Continuous mode.
This bit can be set only when the LPTIM is enabled. It is automatically reset by hardware.
Bit 1 SNGSTRT: LPTIM start in Single mode
This bit is set by software and cleared by hardware.
In case of software start (TRIGEN[1:0] = 00), setting this bit starts the LPTIM in single pulse mode.
If the software start is disabled (TRIGEN[1:0] different than 00), setting this bit starts the LPTIM in
single pulse mode as soon as an external trigger is detected.
If this bit is set when the LPTIM is in continuous counting mode, then the LPTIM stops at the
following match between LPTIM_ARR and LPTIM_CNT registers.
This bit can only be set when the LPTIM is enabled. It is automatically reset by hardware.
Bit 0 ENABLE: LPTIM enable
The ENABLE bit is set and cleared by software.
0: LPTIM is disabled. Writing '0' to the ENABLE bit resets all the DMA request signals (input capture
and update event DMA requests).
1: LPTIM is enabled

58.7.12

LPTIM compare register 1 (LPTIM_CCR1)
Address offset: 0x014
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

CCR1[15:0]
rw

rw

rw

rw

rw

rw

rw

rw

rw

Bits 31:16 Reserved, must be kept at reset value.
Bits 15:0 CCR1[15:0]: Capture/compare 1 value
If channel CC1 is configured as output:
CCR1 is the value to be loaded in the capture/compare 1 register.
Depending on the PRELOAD option, the CCR1 register is immediately updated if the PRELOAD bit
is reset and updated at next LPTIM update event if PREOAD bit is reset.
The capture/compare register 1 contains the value to be compared to the counter LPTIM_CNT and
signaled on OC1 output.
If channel CC1 is configured as input:
CCR1 becomes read-only, it contains the counter value transferred by the last input capture 1 event.
The LPTIM_CCR1 register is read-only and cannot be programmed.
If LPTIM does not implement any channel:
The compare register 1 contains the value to be compared to the counter LPTIM_CNT and signaled
on LPTIM output.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Low-power timer (LPTIM)

Caution:

The LPTIM_CCR1 register must only be modified when the LPTIM is enabled (ENABLE bit
set to 1).

58.7.13

LPTIM autoreload register (LPTIM_ARR)
Address offset: 0x018
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

rw

rw

rw

ARR[15:0]
rw

rw

rw

rw

rw

rw

rw

rw

rw

Bits 31:16 Reserved, must be kept at reset value.
Bits 15:0 ARR[15:0]: Auto reload value
ARR is the autoreload value for the LPTIM.
This value must be strictly greater than the CCRx[15:0] value.

Caution:

The LPTIM_ARR register must only be modified when the LPTIM is enabled (ENABLE bit
set to 1).

58.7.14

LPTIM counter register (LPTIM_CNT)
Address offset: 0x01C
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

r

r

r

r

r

r

r

CNT[15:0]
r

r

r

r

r

r

r

r

r

Bits 31:16 Reserved, must be kept at reset value.
Bits 15:0 CNT[15:0]: Counter value
When the LPTIM is running, reading the LPTIM_CNT register may return unreliable values. In this
case it is necessary to perform consecutive reads until two returned values are identical.

RM0456 Rev 6

<!-- pagebreak -->

2526

Low-power timer (LPTIM)

58.7.15

RM0456

LPTIM configuration register 2 (LPTIM_CFGR2)
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

IC2SEL[1:0]

Res.

Res.

IC1SEL[1:0]

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

3

2

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

5

4

IN2SEL[1:0]
rw

rw

16

rw

rw

1

0

IN1SEL[1:0]
rw

rw

Bits 31:22 Reserved, must be kept at reset value.
Bits 21:20 IC2SEL[1:0]: LPTIM input capture 2 selection
The IC2SEL bits control the LPTIM Input capture 2 multiplexer, which connects LPTIM Input capture
2 to one of the available inputs.
00: lptim_ic2_mux0
01: lptim_ic2_mux1
10: lptim_ic2_mux2
11: lptim_ic2_mux3
For connection details refer to Section 58.4.3: LPTIM input and trigger mapping.
Bits 19:18 Reserved, must be kept at reset value.
Bits 17:16 IC1SEL[1:0]: LPTIM input capture 1 selection
The IC1SEL bits control the LPTIM Input capture 1 multiplexer, which connects LPTIM Input capture
1 to one of the available inputs.
00: lptim_ic1_mux0
01: lptim_ic1_mux1
10: lptim_ic1_mux2
11: lptim_ic1_mux3
For connection details refer to Section 58.4.3: LPTIM input and trigger mapping.
Bits 15:6 Reserved, must be kept at reset value.
Bits 5:4 IN2SEL[1:0]: LPTIM input 2 selection
The IN2SEL bits control the LPTIM input 2 multiplexer, which connects LPTIM input 2 to one of the
available inputs.
00: lptim_in2_mux0
01: lptim_in2_mux1
10: lptim_in2_mux2
11: lptim_in2_mux3
For connection details refer to Section 58.4.3: LPTIM input and trigger mapping.
Bits 3:2 Reserved, must be kept at reset value.
Bits 1:0 IN1SEL[1:0]: LPTIM input 1 selection
The IN1SEL bits control the LPTIM input 1 multiplexer, which connects LPTIM input 1 to one of the
available inputs.
00: lptim_in1_mux0
01: lptim_in1_mux1
10: lptim_in1_mux2
11: lptim_in1_mux3
For connection details refer to Section 58.4.3: LPTIM input and trigger mapping.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Low-power timer (LPTIM)

58.7.16

LPTIM repetition register (LPTIM_RCR)
Address offset: 0x028
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

REP[7:0]
rw

Bits 31:8 Reserved, must be kept at reset value.
Bits 7:0 REP[7:0]: Repetition register value
REP is the repetition value for the LPTIM.

Caution:

The LPTIM_RCR register must only be modified when the LPTIM is enabled (ENABLE bit
set to 1). When using repetition counter with PRELOAD = 0, LPTIM_RCR register must be
changed at least five counter cycles before the auto reload match event, otherwise an
unpredictable behavior may occur.

58.7.17

LPTIM capture/compare mode register 1 (LPTIM_CCMR1)
Address offset: 0x02C
Reset value: 0x0000 0000
The channels can be used in input (capture mode) or in output (PWM mode). The direction
of a channel is defined by configuring the corresponding CCxSEL bits.

31

30

29

28

Res.

Res.

IC2F[1:0]
rw

rw

15

14

13

12

Res.

Res.

IC1F[1:0]
rw

rw

27

26

Res.

Res.

11

10

Res.

Res.

25

24

IC2PSC[1:0]
rw

rw

9

8

IC1PSC[1:0]
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

rw

RM0456 Rev 6

Res.

Res.

Res.

19

18

CC2P[1:0]

17

16

CC2E

CC2
SEL
rw

rw

rw

rw

3

2

1

0

CC1E

CC1
SEL

rw

rw

CC1P[1:0]
rw

rw

<!-- pagebreak -->

2526

Low-power timer (LPTIM)

RM0456

Bits 31:30 Reserved, must be kept at reset value.
Bits 29:28 IC2F[1:0]: Input capture 2 filter
This bitfield defines the number of consecutive equal samples that are detected when a level change
occurs on an external input capture signal before it is considered as a valid level transition. An
internal clock source must be present to use this feature.
00: any external input capture signal level change is considered as a valid transition
01: external input capture signal level change must be stable for at least 2 clock periods before it is
considered as valid transition.
10: external input capture signal level change must be stable for at least 4 clock periods before it is
considered as valid transition.
11: external input capture signal level change must be stable for at least 8 clock periods before it is
considered as valid transition.
Bits 27:26 Reserved, must be kept at reset value.
Bits 25:24 IC2PSC[1:0]: Input capture 2 prescaler
This bitfield defines the ratio of the prescaler acting on the CC2 input (IC2).
00: no prescaler, capture is done each time an edge is detected on the capture input
01: capture is done once every 2 events
10: capture is done once every 4 events
11: capture is done once every 8 events
Bits 23:20 Reserved, must be kept at reset value.
Bits 19:18 CC2P[1:0]: Capture/compare 2 output polarity.
Condition: CC2 as output
Only bit2 is used to set polarity when output mode is enabled, bit3 is don't care.
0: OC2 active high
1: OC2 active low
Condition: CC2 as input
This field is used to select the IC2 polarity for capture operations.
00: rising edge, circuit is sensitive to IC2 rising edge
01: falling edge, circuit is sensitive to IC2 falling edge
10: reserved, do not use this configuration.
11: both edges, circuit is sensitive to both IC2 rising and falling edges.
Bit 17 CC2E: Capture/compare 2 output enable.
Condition: CC2 as output
0: Off - OC2 is not active. Writing '0' to the CC2E bit resets the ue_dma_req signal only if all the
other LPTIM channels are disabled.
1: On - OC2 signal is output on the corresponding output pin
Condition: CC2 as input
This bit determines if a capture of the counter value can actually be done into the input
capture/compare register 2 (LPTIM_CCR2) or not.
0: Capture disabled. Writing '0' to the CC2E bit resets the associated ic2_dma_req signal.
1: Capture enabled.
Bit 16 CC2SEL: Capture/compare 2 selection
This bitfield defines the direction of the channel, input (capture) or output mode.
0: CC2 channel is configured in output PWM mode
1: CC2 channel is configured in input capture mode
Bits 15:14 Reserved, must be kept at reset value.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Low-power timer (LPTIM)

Bits 13:12 IC1F[1:0]: Input capture 1 filter
This bitfield defines the number of consecutive equal samples that are detected when a level change
occurs on an external input capture signal before it is considered as a valid level transition. An
internal clock source must be present to use this feature.
00: any external input capture signal level change is considered as a valid transition
01: external input capture signal level change must be stable for at least 2 clock periods before it is
considered as valid transition.
10: external input capture signal level change must be stable for at least 4 clock periods before it is
considered as valid transition.
11: external input capture signal level change must be stable for at least 8 clock periods before it is
considered as valid transition.
Bits 11:10 Reserved, must be kept at reset value.
Bits 9:8 IC1PSC[1:0]: Input capture 1 prescaler
This bitfield defines the ratio of the prescaler acting on the CC1 input (IC1).
00: no prescaler, capture is done each time an edge is detected on the capture input
01: capture is done once every 2 events
10: capture is done once every 4 events
11: capture is done once every 8 events
Bits 7:4 Reserved, must be kept at reset value.
Bits 3:2 CC1P[1:0]: Capture/compare 1 output polarity.
Condition: CC1 as output
Only bit2 is used to set polarity when output mode is enabled, bit3 is don't care.
0: OC1 active high, the LPTIM output reflects the compare results between LPTIM_ARR and
LPTIM_CCRx registers
1: OC1 active low, the LPTIM output reflects the inverse of the compare results between LPTIM_ARR
and LPTIM_CCRx registers
Condition: CC1 as input
This field is used to select the IC1 polarity for capture operations.
00: rising edge, circuit is sensitive to IC1 rising edge
01: falling edge, circuit is sensitive to IC1 falling edge
10: reserved, do not use this configuration.
11: both edges, circuit is sensitive to both IC1 rising and falling edges.
Bit 1 CC1E: Capture/compare 1 output enable.
Condition: CC1 as output
0: Off - OC1 is not active. Writing '0' to the CC1E bit resets the ue_dma_req signal only if all the
other LPTIM channels are disabled.
1: On - OC1 signal is output on the corresponding output pin
Condition: CC1 as input
This bit determines if a capture of the counter value can actually be done into the input
capture/compare register 1 (LPTIM_CCR1) or not.
0: Capture disabled. Writing '0' to the CC1E bit resets the associated ic1_dma_req signal.
1: Capture enabled.
Bit 0 CC1SEL: Capture/compare 1 selection
This bitfield defines the direction of the channel input (capture) or output mode.
0: CC1 channel is configured in output PWM mode
1: CC1 channel is configured in input capture mode

Caution:

After a write to the LPTIM_CCMRx register, a new write operation to the same register can
only be performed after a delay that must be equal or greater than the value of (PRESC × 3)

RM0456 Rev 6

<!-- pagebreak -->

2526

Low-power timer (LPTIM)

RM0456

kernel clock cycles, PRESC[2:0] being the clock decimal division factor (1, 2, 4,..128). Any
successive write violating this delay, leads to unpredictable results.
Caution:

The CCxSEL, ICxF[1:0], CCxP[1:0] and ICxPSC[1:0] fields must only be modified when the
channel x is disabled (CCxE bit reset to 0).
If LPTIM does not implement any channel this register is reserved. Refer to Section 58.3.

58.7.18

LPTIM compare register 2 (LPTIM_CCR2)
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

Bits 31:16 Reserved, must be kept at reset value.
Bits 15:0 CCR2[15:0]: Capture/compare 2 value
If channel CC2 is configured as output:
CCR2 is the value to be loaded in the capture/compare 2 register.
Depending on the PRELOAD option, the CCR2 register is immediately updated if the PRELOAD bit
is reset and updated at next LPTIM update event if PREOAD bit is reset.
The capture/compare register 2 contains the value to be compared to the counter LPTIM_CNT and
signaled on OC2 output.
If channel CC2 is configured as input:
CCR2 becomes read-only, it contains the counter value transferred by the last input capture 2 event.
The LPTIM_CCR2 register is read-only and cannot be programmed.

Caution:

The LPTIM_CCR2 register must only be modified when the LPTIM is enabled (ENABLE bit
set to 1).

Note:

If the LPTIM implements less than 2 channels this register is reserved. Refer to
Section 58.3: LPTIM implementation.

58.7.19

LPTIM register map
The following table summarizes the LPTIM registers.

<!-- pagebreak -->

11

10

Res.

Res.

0

12
Res.

1

13
Res.

CC1IF

14
Res.

2

15
Res.

ARRM

16
Res.

3

17
Res.

EXTTRIG

18
Res.

4

19
Res.

CMP1OK

20
Res.

5

21
Res.

ARROK

22
Res.

6

23
Res.

UP(2)

24
DIEROK

7

25
Res.

DOWN(2)

26
Res.

8

27
Res.

RM0456 Rev 6

UE

28
Res.

0

9

29
Res.

Reset value

Res.

30

LPTIM4_ISR

REPOK

31

0x000

Register name

Res.

Offset

Res.

Table 616. LPTIM register map and reset values

0

0

0

0

0

0

0

0

0

0x014

LPTIM_CCR1

0

Reset value

RM0456 Rev 6

Res.

0

0

Res.

Res.

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

SNGSTRT

ENABLE

0

CNTSTRT

CCR1[15:0]

COUNTRST

Reset value

RSTARE

Res.

UPIE(2)
ARROKIE
CMP1OKIE
EXTTRIGIE
ARRMIE
CC1IE

0
0
0
0
0
0
0

UPIE(2)
ARROKIE
Res.
EXTTRIGIE
ARRMIE
CC1IE

CMP1OKIE
EXTTRIGIE
ARRMIE
CC1IE

ARRMCF
CC1CF

0
0
0
0
0
0
0
0

CC1CF

EXTTRIGCF

0
ARRMCF

CMP1OKCF

0

Res.

CMP1OKCF
EXTTRIGCF
ARRMCF
CC1CF

EXTTRIG
ARRM
CC1IF

Res.

ARROK

ARROKCF

UP(2)

UPCF(2)

DOWN(2)

DOWNCF(2)

0

EXTTRIGCF

ARROKCF

ARROKCF
0

ARROKIE

UPCF(2)

UPCF(2)
0

UPIE(2)

DOWNCF(2)

DOWNCF(2)
0

DOWNIE(2)

0

CKSEL

DOWNIE(2)

0

DOWNIE(2)

UE
0

UECF

REPOK
0

REPOKCF

CC2IF(1)

Res.

Res.

0

Res.

Res.

Res.

0

CKPOL

CKFLT

Res.

TRGFLT

UECF

UECF
0

UEIE

Reset value

Res.

UEIE

0

UEIE

CC2CF(1)

Res.
REPOKCF

CC2CF(1)
0

REPOKIE

REPOKCF

Res.
0

Res.

Res.

0

Res.

CC2IE(1)
REPOKIE

0

CC2IE(1)

0

REPOKIE

Res.

0

Res.

Res.

Res.

CC1OF

Res.

2
1
0
CC1IF

Res.

ARRM

12

Res.

3

13

Res.

EXTTRIG

14

Res.

4

15

Res.

CMP1OK

16

Res.

5

17

Res.

ARROK

18

CMP2OK(1)

6

19

Res.

UP(2)

20

Res.

7

21

Res.

DOWN(2)

22

Res.

8

23

DIEROK

UE

24

Res.

9

25

Res.

REPOK

26

Res.

CC2IF(1)

27

Res.

11

28

Res.

10

29

Res.

Res.

30

Res.

Res.

0

Res.

0

Res.

PRESC

0

Res.

0

Res.

Res.

CC2OF(1)

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

CC1OCF

0

Res.

Res.

Res.

Res.

Res.

Res.

Res.

CMP2OKCF(1)

Res.

Res.

Res.

Res.

0

Res.

CC2OCF(1)

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

DIEROK

Res.

Res.

Res.

Res.

31

LPTIMx_ISR
(x = 1 to 3)
Output compare
mode

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

DIEROKCF

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Register name

Res.

CC1OIE

0
CC2OIE(1)

Res.

Res.

Res.

Res.

Res.

CMP2OKIE(1)

Res.

Res.

0

Res.

TRIGSEL[2:0]

Res.

CC1DE

Res.

Res.

Res.

Res.

Res.

Res.

Res.

DIEROKCF

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

Res.

0

Res.

TRIGEN

0

Res.

0

Res.

TIMOUT

0

Res.

Res.

0

Res.

WAVE

0

Res.

Res.

Res.

UEDE

0

Res.

WAVPOL(3)

0

Res.

Res.

UEDE

DIEROKCF

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

Res.

PRELOAD

0

Res.

0

Res.

COUNTMODE

0

Res.

Reset value

Res.

Reset value
Res.

Reset value

Res.

Res.

Res.

Res.

Res.

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

Reset value

ENC(2)

CC2DE(1)

Res.

Res.

Res.

Res.

Res.

Res.

Reset value

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Reset value

Res.

Res.

Res.

Res.

Res.

Reset value

Res.

Res.

LPTIM_CR

Res.

0x010
LPTIM_CFGR

Res.

0x00C
Res.

LPTIMx_DIER
(x = 1 to 3)
Input capture mode

Res.

LPTIMx_DIER
(x = 1 to 3)
Output compare
mode

Res.

0x008
LPTIM4_DIER

Res.

0x008

Res.

LPTIMx_ICR
(x = 1 to 3)
Input capture mode

Res.

LPTIMx_ICR
(x = 1 to 3)
Output compare
mode

Res.

0x004
LPTIM4_ICR

Res.

0x004

Res.

LPTIMx_ISR
(x = 1 to 3)
Input capture mode

Res.

0x000

Res.

Offset

Res.

RM0456
Low-power timer (LPTIM)

Table 616. LPTIM register map and reset values (continued)

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

<!-- pagebreak -->

2526

Low-power timer (LPTIM)

RM0456

3

2

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

Res.

0

0

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

CCR2[15:0]
0

0

0

0

0

0

0

If LPTIM does not implement at least 2 channels this bit is reserved. Refer to Section 58.3: LPTIM implementation.

2.

If LPTIM does not support encoder mode feature, this bit is reserved. Refer to Section 58.3: LPTIM implementation.

0

0

0

0

3.

If the LPTIM implements at least one capture/compare channel, this bit is reserved. Refer to Section 58.3: LPTIM implementation.

4.

If LPTIM does not implement any channel this register is reserved. Refer to Section 58.3: LPTIM implementation.

5.

If the LPTIM implements less than 2 channels this register is reserved. Refer to Section 58.3: LPTIM implementation.

Refer to Section 2.3: Memory organization for the register boundary addresses.

RM0456 Rev 6

0

REP[7:0]

0

1.

<!-- pagebreak -->

0

CC1SEL

IC1PSC[1:0]

Res.

0

0

CC1E

0

0

CC1P[1:0]

Res.

Res.

Res.

Res.

Res.

Res.

0

IN1SEL[1:0]

0

Res.

Res.

4

0

IC1F[1:0]

Res.

0

Res.

0

0

Reset value

0

IN2SEL[1:0]

0

Res.

0

0

CNT[15:0]

Res.

CC2SEL

0

5

0

Res.

16
Res.
Res.

IC1SEL[1:0]
Res.

CC2E
0

Res.

CC2P[1:0]
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
Res.

Res.

IC2PSC[1:0]
0
Res.

Res.
Res.

0
Res.

Res.

IC2F[1:0]
0

Res.

LPTIM_CCR2

Res.

0x034

(5)

Res.

Reset value

Res.

Res.

LPTIM_CCMR1(4)

6

17
Res.
Res.
Res.

0

Res.

18
Res.
Res.
Res.
Res.

0

7

19
Res.
Res.
Res.
Res.

0

0

ARR[15:0]

Res.

20
Res.
Res.

IC2SEL[1:0]
Res.

0

8

21
Res.
Res.
Res.

0

Res.

22
Res.
Res.
Res.
Res.

9

23
Res.
Res.
Res.
Res.

0

Res.

24
Res.
Res.
Res.
Res.

0

Res.

25
Res.
Res.
Res.
Res.

0

11

26
Res.
Res.
Res.
Res.

0

0

10

27
Res.
Res.
Res.
Res.

Res.

Res.

0

0

Reset value
Res.

0x02C

Res.

LPTIM_RCR

0x028

Res.

Reset value

12

28
Res.
Res.
Res.

Res.

Res.

LPTIM_
CFGR2

Res.

Reset value

0x024

13

29
Res.
Res.

Res.

LPTIM_CNT

0x01C

Res.

Reset value

14

30

LPTIM_ARR

15

31

0x018

Register name

Res.

Offset

Res.

Table 616. LPTIM register map and reset values (continued)

RM0456

59

Graphic timer (GFXTIM)

Graphic timer (GFXTIM)
This section only applies to STM32U5Fx/5Gx devices.

59.1

Introduction
The graphic timer (GFXTIM) is a graphic oriented timer allowing smart management of
graphical events for frame or line counting.

59.2

GFXTIM main features
•

Integrated frame and line clock generation

•

One absolute frame counter with one compare channel

•

Two auto-reload relative frame counters

•

One line timer with two compare channels

•

External tearing-effect line management and synchronization

•

Four programmable event generators with external trigger generation

•

One watchdog counter

59.3

GFXTIM functional description

59.3.1

Block diagram
The graphic timer is split into six functional blocks
•

Clock generator

•

Absolute timers

•

Relative timers

•

Tearing-effect detection

•

Event generators

•

Watchdog timer

RM0456 Rev 6

<!-- pagebreak -->

2562

Graphic timer (GFXTIM)

RM0456
Figure 750. GFXTIM block diagram
TE

GFXTIM_TE

gfxtim_ite
gfxtim_hsync[3:0]
gfxtim_vsync[3:0]

Source
Selection

Clock generator

HSYNC

Auto-reload

Auto-reload

22-bit
line clock counter

12-bit
frame clock counter

Frame Ck

Control and status
gfxtim_it

TE

TE Detect

VSYNC

Line Ck

Int. registers

gfxtim_wit
Absolute timer

gfxtim_hclk

AFCO

Event Generators

AFCC

32-bit APB bus

20-bit frame counter

AFCO

12-bit line counter

ALCO
RFC1R
RFC2R

AFCC

Compare 1 register

ALCC1

Compare 1 register

Event generator 1

EV1

Event generator 2

EV2

Event generator 3

EV3

Event generator 4

EV4

ALCO
TE

ALCC2

Compare 2 register

ALCC1
ALCC2

gfxtim_ev1
gfxtim_ev2
Relative timers

Watchdog timer

wrld

gfxtim_ev3
gfxtim_ev4
gfxtim_wrld
GFXTIM_FCKCAL
GFXTIM_LCKCAL

Frame Ck

Auto-reload 1
12-bit
frame counter 1

Auto-reload 2
RFC1R

12-bit
frame counter 2

Line Ck

Frame clock domain

59.3.2

RFC2R

Line Ck
Frame Ck
HSYNC
VSYNC
TE
EV1
EV2
EV3
EV4

Auto-reload
16-bit counter

PALRM

Pre-alarm

Line clock domain

MSv66908V2

GFXTIM pins and internal signals
Table 617. GFXTIM input/output pins
Pin name

Signal type

Description

GFXTIM_TE

Input

Tearing effect

GFXTIM_FCKCAL

Output

Frame clock calibration output

GFXTIM_LCKCAL

Output

Line clock calibration output

Table 618. GFXTIM internal signals

<!-- pagebreak -->

Signal name

Signal type

Description

gfxtim_hclk

Digital input

Kernel and register interface clock

gfxtim_it

Digital output

Global interrupt

gfxtim_wit

Digital output

Watchdog global interrupt

gfxtim_ev1

Digital output

Graphic timer event 1

gfxtim_ev2

Digital output

Graphic timer event 2

gfxtim_ev3

Digital output

Graphic timer event 3

gfxtim_ev4

Digital output

Graphic timer event 4

gfxtim_wrld

Digital input

Watchdog reload

gfxtim_ite

Digital input

Internal tearing effect

RM0456 Rev 6

ALRM

RM0456

Graphic timer (GFXTIM)
Table 618. GFXTIM internal signals (continued)
Signal name

Signal type

Description

gfxtim_hsync[3:0]

Digital input

Horizontal synchronization

gfxtim_vsync[3:0]

Digital input

Vertical synchronization

The table below shows how GFXTIM triggers are connected.
Table 619. GFXTIM trigger interconnections

59.3.3

Trigger name

Direction

Trigger source/destination

gfxtim_ev1

Output

gpdma_trigsel[62]

gfxtim_ev2

Output

gpdma_trigsel[61]

gfxtim_ev3

Output

gpdma_trigsel[60]

gfxtim_ev4

Output

gpdma_trigsel[59]

gfxtim_wrld

Input

reserved

gfxtim_ite

Input

reserved

gfxtim_hsync[0]

Input

LCD_HSYNC

gfxtim_hsync[1]

Input

reserved

gfxtim_hsync[2]

Input

reserved

gfxtim_hsync[3]

Input

reserved

gfxtim_vsync[0]

Input

LCD_VSYNC

gfxtim_vsync[1]

Input

reserved

gfxtim_vsync[2]

Input

reserved

gfxtim_vsync[3]

Input

reserved

Clock generator
Two clocks are generated internally to fed the absolute and relative timers:
•

the frame clock: clocking frame counters

•

the line clock: clocking line counters

Internal counter for time base generation
The GFXTIM embeds a two-clock generation counter:
•

a 22-bit auto-reload down-counter on the system clock

•

a 12-bit auto-reload down-counter on selectable internal or external event

RM0456 Rev 6

<!-- pagebreak -->

2562

Graphic timer (GFXTIM)

RM0456
Figure 751. Clock generator
FCCUF
VSYNC
HSYNC

Line clock counter
LCCReload

Auto-reload

TE
SYSCK

22-bit Counter

SWReload
LCCUF
FCCUF
VSYNC
LCCUF HSYNC

Line clock

TE

FCCUF
VSYNC
HSYNC
TE

Frame clock counter
FCCReload
Auto-reload
LCCUF
VSYNC
HSYNC

12-bit Counter

SWReload
LCCUF
FCCUF
VSYNC
FCCUF HSYNC

Frame clock

TE

TE
MSv66909V1

Line clock counter
The line clock counter is a 22-bit auto-reload down-counter on the system clock.
The line clock counter is enabled selecting its clock source with LCCCS (line clock counter
clock source) in GFXTIM_CGCR (clock generator control register).
The line clock counter can be reloaded automatically when one of the following event
occurs:
•

frame clock counter underflows

•

VSYNC edge (with control of polarity)

•

HSYNC edge (with control of polarity)

•

TE edge (with control of polarity)

The hardware reload function and source can be selected with LCCHRS (line clock counter
hardware reload source) in GFXTIM_CGCR (clock generator control register).
The line clock counter can be reloaded by software, setting LCCFR (line clock counter force
reload) in GFXTIM_CGCR (clock generator control register).
The reload value is programmed in GFXTIM_LCCRR (line clock counter reload register).

Frame clock counter
The frame clock counter if an 12-bit auto-reload down-counter clocked by either, TE,
HYSNC, VSYNC or a line clock counter underflow.
The frame clock counter is enabled selecting its clock source with FCCCS in
GFXTIM_CGCR.
The frame clock counter can be reloaded automatically when one of the following event
occurs:
•

line clock counter underflow

•

VSYNC edge (with control of polarity)

•

HSYNC edge (with control of polarity)

•

TE edge (with control of polarity)

The hardware reload function and source can be selected with FCCHRS in
GFXTIM_CGCR.
The frame clock counter can be reloaded by software, setting FCCFR in GFXTIM_CGCR.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Graphic timer (GFXTIM)
The reload value is programmed in GFXTIM_FCCRR.

Clock generation
The line clock source can be one of the following:
•

underflow flag of the internal 22-bit down-counter

•

underflow flag of the internal 12-bit down-counter

•

HSYNC, VSYNC or TE pin (with control of polarity)

The frame clock source can be one of the following:
•

underflow flag of the internal 22-bit down-counter

•

underflow flag of the internal 12-bit down-counter

•

HSYNC, VSYNC or TE pin (with control of polarity)

Clock calibration output
For calibration/debug purpose, the frame clock and line clock can be output
on a specific I/O.
The frame clock calibration output is enabled by setting FCCOE (frame clock calibration
output enable) in GFXTIM_CR (configuration register).
The line clock calibration output is enabled by setting LCCOE in GFXTIM_CR.

Synchronization and tearing-effect sources
The GFXTIM can be connected to peripherals providing HSYNC and/or VSYNC
synchronization signals, like the LCD-TFT controller or the camera interface.
The source of HSYNC and/or VSYNC is selected through SYNCS (synchronization source)
in GFXTIM_CR.
The tearing-effect source can be an external pin or can be provided by the MIPI® DSI Host
on system embedding this interface. To extend the orthogonality versus the synchronization
signals, the tearing-effect source can be also the selected HSYNC or VSYNC input.
The tearing-effect source can be selected with TES (TE source) in GFXTIM_CR.

59.3.4

Example of clock generator configuration
The clock generator can have several configuration to work:
•

in standalone (without any external synchronization)

•

with external HSYNC and VSYNC

•

with external HSYNC only

•

with external VSYNC only

•

with external CSYNC (TE) only

The synchronization signals, HSYNC, VSYNC and TE, are symmetrical in the
implementation and can be exchanged if needed.
The set of examples detailed below are given for reference, but other combinations can be
programmed int the clock generator.

RM0456 Rev 6

<!-- pagebreak -->

2562

Graphic timer (GFXTIM)

RM0456

Standalone
In standalone configuration, the clock generator provides to the GFXTIM, the frame clock
and the line clock without any external signals.
Figure 752. Waveforms in standalone
Display refresh duration
Line duration

LCCounter
LCCUF
FCCounter

1

0

n n-1 n-2 n-3 n-4

2

1

0

n n-1 n-2

FCCUF
MSv66910V1

The LCCUF (line-clock counter underflow) event acts as line clock and the FCCUF
(frame-clock counter underflow) acts as frame clock.
The clock generator can be synchronized by one of the following ways:
•

by software reload, setting the FCCFR in GFXTIM_CGCR.

•

by hardware reload on an external VSYNC (or TE)
Figure 753. Active counters and signals in standalone
FCCUF
VSYNC
HSYNC

Line clock counter
LCCReload

Auto-reload

TE
SYSCK

22-bit counter

SWReload
LCCUF
FCCUF
VSYNC
LCCUF HSYNC

Line clock

TE
Selected function
FCCUF
VSYNC
HSYNC
TE

Frame clock counter
FCCReload

Auto-reload
LCCUF
VSYNC
HSYNC

12-bit counter

SWReload
LCCUF
FCCUF
VSYNC
FCCUF HSYNC

Frame clock

TE

TE
MSv66911V1

External HSYNC and VSYNC
When using external HSYNC and VSYNC, the counters are not used for the line and frame
clock generation. The clock generator copy directly HSYNC to the line clock and VSYNC to
the frame clock.
Figure 754. Waveforms with external HSYNC and VSYNC
Display refresh duration
VSYNC
Line
duration

HSYNC
MSv66912V1

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Graphic timer (GFXTIM)
HSYNC acts as line clock and VSYNC acts as frame clock.

External HSYNC only
With external HSYNC only, the clock generator provides to the GFXTIM, the frame clock
and the line clock based only on HSYNC.
Figure 755. Waveforms with external HSYNC only
Display refresh duration
Line
duration

TE/HSYNC
FCCounter

1

0

n n-1 n-2 n-3 n-4

2

1

0

n n-1 n-2

FCCUF

MSv66913V1

HSYNC acts as line clock and FCCUF acts as frame clock.
The clock generator can be synchronized by one of the following ways:
•

by software reload, setting FCCFR in GFXTIM_CGCR

•

by hardware reload on an external VSYNC (or TE)
Figure 756. Active counters and signals with external HSYNC only
FCCUF
VSYNC
HSYNC

Line clock counter
LCCReload

Auto-reload

TE
SYSCK

22-bit counter

SWReload
LCCUF
FCCUF
VSYNC
LCCUF HSYNC

Line clock

TE
Selected function
FCCUF
VSYNC
HSYNC

Frame clock counter
FCCReload

Auto-reload

TE

LCCUF
VSYNC
HSYNC

12-bit counter

SWReload
LCCUF
FCCUF
VSYNC
FCCUF HSYNC

Frame clock

TE

TE
MSv66914V1

External VSYNC only
With external VSYNC only, the clock generator provides to the GFXTIM, the frame clock
and the line clock based only on VSYNC.
Figure 757. Waveforms with external VSYNC only
Display refresh duration
Line
duration

TE/VSYNC
xCCReload

xCCReload

xCCounter
xCCUF
MSv66915V1

LCCUF (line clock counter underflow) acts as line clock and VSYNC acts as frame clock.

RM0456 Rev 6

<!-- pagebreak -->

2562

Graphic timer (GFXTIM)

RM0456

The line clock counter generating the line clock is reloaded on VSYNC event.
Figure 758. Active counters with external VSYNC only
Line clock counter

FCCUF
VSYNC
HSYNC

SWReload

LCCReload

LCCUF

Auto-reload

TE
SYSCK

22-bit counter

FCCUF
VSYNC
LCCUF HSYNC

Line clock

TE
Selected function

Frame clock counter

FCCUF
VSYNC
HSYNC

FCCReload

LCCUF

Auto-reload

TE

LCCUF
VSYNC
HSYNC

SWReload

12-bit counter

FCCUF
VSYNC
FCCUF HSYNC

Frame clock

TE

TE
MSv66916V1

The line clock counter can also act as a prescaler for the frame clock counter to have a
wider range of counting. FCCUF acts as line clock.
Figure 759. Prescaling when external VSYNC only
Line clock counter

FCCUF
VSYNC
HSYNC

SWReload

LCCReload

LCCUF

Auto-reload

TE
SYSCK

22-bit counter

FCCUF
VSYNC
LCCUF HSYNC

Line clock

TE

Frame clock counter

FCCUF
VSYNC
HSYNC

FCCReload

LCCUF
VSYNC
HSYNC

SWReload
LCCUF

Auto-reload

TE

Selected function

12-bit counter

FCCUF
VSYNC
FCCUF HSYNC

Frame clock

TE

TE
MSv66917V1

External CSYNC only
With external CSYNC only, the clock generator provides to the GFXTIM, the frame clock
and the line clock based only on CSYNC (in the following figures, CSYNC is input on TE).
Figure 760. Waveforms with external CSYNC only
TE (CSYNC)

(VSYNC + HSYNC)
xCCReload

xCCounter

n n-1

xCCUF
0

n n-1

xCCReload
m

n n-1

xCCReload
q

n n-1

xCCReload
q

n n-1

xCCUF
MSv66918V1

The CSYNC (on TE pin in this example) acts as line clock and LCCUF acts as frame clock.
The line clock counter generating the frame clock is reloaded on CSYNC event.

<!-- pagebreak -->

