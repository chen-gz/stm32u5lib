RM0456 Rev 6

RM0456

General-purpose timers (TIM2/TIM3/TIM4/TIM5)

55.5.3

TIMx slave mode control register (TIMx_SMCR)(x = 2 to 5)
Address offset: 0x008
Reset value: 0x0000 0000

31

30

29

28

27

26

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

ETP

ECE

ETPS[1:0]

rw

rw

rw

rw

rw

rw

25

24

SMSPS SMSPE
rw

rw

9

8

ETF[3:0]
rw

23

22

Res.

Res.

7

6

MSM
rw

rw

21

20

TS[4:3]
rw

rw

5

4

TS[2:0]
rw

rw

19

18

17

16

Res.

Res.

Res.

SMS[3]

3

2

1

rw

OCCS
rw

rw

0

SMS[2:0]
rw

rw

rw

Bits 31:26 Reserved, must be kept at reset value.
Bit 25 SMSPS: SMS preload source
This bit selects whether the events that triggers the SMS[3:0] bitfield transfer from preload to
active
0: The transfer is triggered by the Timer’s Update event
1: The transfer is triggered by the Index event
Bit 24 SMSPE: SMS preload enable
This bit selects whether the SMS[3:0] bitfield is preloaded
0: SMS[3:0] bitfield is not preloaded
1: SMS[3:0] preload is enabled
Bits 23:22 Reserved, must be kept at reset value.
Bits 19:17 Reserved, must be kept at reset value.
Bit 15 ETP: External trigger polarity
This bit selects whether tim_etr_in or tim_etr_in is used for trigger operations
0: tim_etr_in is non-inverted, active at high level or rising edge
1: tim_etr_in is inverted, active at low level or falling edge
Bit 14 ECE: External clock enable
This bit enables External clock mode 2.
0: External clock mode 2 disabled
1: External clock mode 2 enabled. The counter is clocked by any active edge on the tim_etrf
signal.
Note: Setting the ECE bit has the same effect as selecting external clock mode 1 with tim_trgi
connected to tim_etrf (SMS = 111 and TS = 00111).
It is possible to simultaneously use external clock mode 2 with the following slave
modes: reset mode, gated mode and trigger mode. Nevertheless, tim_trgi must not be
connected to tim_etrf in this case (TS bits must not be 00111).
If external clock mode 1 and external clock mode 2 are enabled at the same time, the
external clock input is tim_etrf.
Bits 13:12 ETPS[1:0]: External trigger prescaler
External trigger signal tim_etrp frequency must be at most 1/4 of tim_ker_ck frequency. A
prescaler can be enabled to reduce tim_etrp frequency. It is useful when inputting fast
external clocks on tim_etr_in.
00: Prescaler OFF
01: tim_etrp frequency divided by 2
10: tim_etrp frequency divided by 4
11: tim_etrp frequency divided by 8

RM0456 Rev 6

<!-- pagebreak -->

2350

General-purpose timers (TIM2/TIM3/TIM4/TIM5)

RM0456

Bits 11:8 ETF[3:0]: External trigger filter
This bitfield then defines the frequency used to sample tim_etrp signal and the length of the
digital filter applied to tim_etrp. The digital filter is made of an event counter in which N
consecutive events are needed to validate a transition on the output:
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
Bit 7 MSM: Master/Slave mode
0: No action
1: The effect of an event on the trigger input (tim_trgi) is delayed to allow a perfect
synchronization between the current timer and its slaves (through tim_trgo). It is useful if we
want to synchronize several timers on a single external event.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

General-purpose timers (TIM2/TIM3/TIM4/TIM5)

Bits 21, 20, 6, 5, 4 TS[4:0]: Trigger selection
This bitfield selects the trigger input to be used to synchronize the counter.
00000: Internal trigger 0 (tim_itr0)
00001: Internal trigger 1 (tim_itr1)
00010: Internal trigger 2 (tim_itr2)
00011: Internal trigger 3 (tim_itr3)
00100: tim_ti1 edge detector (tim_ti1f_ed)
00101: Filtered timer input 1 (tim_ti1fp1)
00110: Filtered timer input 2 (tim_ti2fp2)
00111: External trigger input (tim_etrf)
01000: Internal trigger 4 (tim_itr4)
01001: Internal trigger 5 (tim_itr5)
01010: Internal trigger 6 (tim_itr6)
01011: Internal trigger 7 (tim_itr7)
01100: Internal trigger 8 (tim_itr8)
01101: Internal trigger 9 (tim_itr9)
01110: Internal trigger 10 (tim_itr10)
01111: Internal trigger 11 (tim_itr11)
10000: Internal trigger 12 (tim_itr12)
10001: Internal trigger 13 (tim_itr13)
10010: Internal trigger 14 (tim_itr14)
10011: Internal trigger 15 (tim_itr15)
Others: Reserved
See Section 55.4.2: TIM2/TIM3/TIM4/TIM5 pins and internal signals for product specific
implementation details.
Note: These bits must be changed only when they are not used (for example when
SMS = 000) to avoid wrong edge detections at the transition.
Bit 3 OCCS: OCREF clear selection
This bit is used to select the OCREF clear source
0: tim_ocref_clr_int is connected to the tim_ocref_clr input
1: tim_ocref_clr_int is connected to tim_etrf
Note: If the OCREF clear selection feature is not supported, this bit is reserved and forced by
hardware to 0. Section 55.3: TIM2/TIM3/TIM4/TIM5 implementation.

RM0456 Rev 6

<!-- pagebreak -->

2350

General-purpose timers (TIM2/TIM3/TIM4/TIM5)

RM0456

Bits 16, 2, 1, 0 SMS[3:0]: Slave mode selection
When external signals are selected the active edge of the trigger signal (tim_trgi) is linked to
the polarity selected on the external input (refer to ETP bit in TIMx_SMCR for tim_etr_in and
CCxP/CCxNP bits in TIMx_CCER register for tim_ti1fp1 and tim_ti2fp2).
0000:Slave mode disabled - if CEN = 1 then the prescaler is clocked directly by the internal
clock.
0001:Encoder mode 1 - Counter counts up/down on tim_ti1fp1 edge depending on
tim_ti2fp2 level.
0010:Encoder mode 2 - Counter counts up/down on tim_ti2fp2 edge depending on
tim_ti1fp1 level.
0011:Encoder mode 3 - Counter counts up/down on both tim_ti1fp1 and tim_ti2fp2 edges
depending on the level of the other input.
0100:Reset mode - Rising edge of the selected trigger input (tim_trgi) reinitializes the
counter and generates an update of the registers.
0101:Gated mode - The counter clock is enabled when the trigger input (tim_trgi) is high.
The counter stops (but is not reset) as soon as the trigger becomes low. Both start and
stop of the counter are controlled.
0110:Trigger mode - The counter starts at a rising edge of the trigger tim_trgi (but it is not
reset). Only the start of the counter is controlled.
0111:External clock mode 1 - Rising edges of the selected trigger (tim_trgi) clock the
counter.
1000:Combined reset + trigger mode - Rising edge of the selected trigger input (tim_trgi)
reinitializes the counter, generates an update of the registers and starts the counter.
1001:Combined gated + reset mode - The counter clock is enabled when the trigger input
(tim_trgi) is high. The counter stops and is reset) as soon as the trigger becomes low.
Both start and stop of the counter are controlled.
1010:Encoder mode: Clock plus direction, x2 mode.
1011:Encoder mode: Clock plus direction, x1 mode, tim_ti2fp2 edge sensitivity is set by
CC2P.
1100:Encoder mode: Directional clock, x2 mode.
1101:Encoder mode: Directional clock, x1 mode, tim_ti1fp1 and tim_ti2fp2 edge sensitivity is
set by CC1P and CC2P.
1110:Quadrature encoder mode: x1 mode, counting on tim_ti1fp1 edges only, edge
sensitivity is set by CC1P.
1111:Quadrature encoder mode: x1 mode, counting on tim_ti2fp2 edges only, edge
sensitivity is set by CC2P.
Note: The gated mode must not be used if tim_ti1f_ed is selected as the trigger input (TS =
00100). Indeed, tim_ti1f_ed outputs 1 pulse for each transition on tim_ti1f, whereas the
gated mode checks the level of the trigger signal.
Note: The clock of the slave peripherals (such as timer, ADC) receiving the tim_trgo signal
must be enabled prior to receive events from the master timer, and the clock frequency
(prescaler) must not be changed on-the-fly while triggers are received from the master
timer.

<!-- pagebreak -->

