rw

RM0456 Rev 6

rw

EXT
ARRM
TRIGIE
IE
rw

rw

0
CC1IE
rw

RM0456

Low-power timer (LPTIM)

Bits 31:28 Reserved, must be kept at reset value.
Bit 27 Reserved, must be kept at reset value.
Bit 26 Reserved, must be kept at reset value.
Bit 25 CC2DE: Capture/compare 2 DMA request enable
0: CC2 DMA request disabled. Writing '0' to the CC2DE bit resets the associated ic2_dma_req
signal.
1: CC2 DMA request enabled
Note: If LPTIM does not implement at least 2 channels this bit is reserved. Refer to Section 58.3.
Bit 24 Reserved, must be kept at reset value.
Bit 23 UEDE: Update event DMA request enable
0: UE DMA request disabled. Writing '0' to the UEDE bit resets the associated ue_dma_req signal.
1: UE DMA request enabled
Note: If LPTIM does not implement at least 1 channel this bit is reserved. Refer to Section 58.3.
Bits 22:17 Reserved, must be kept at reset value.
Bit 16 CC1DE: Capture/compare 1 DMA request enable
0: CC1 DMA request disabled. Writing '0' to the CC1DE bit resets the associated ic1_dma_req
signal.
1: CC1 DMA request enabled
Note: If LPTIM does not implement at least 1 channel this bit is reserved. Refer to Section 58.3.
Bit 15 Reserved, must be kept at reset value.
Bit 14 Reserved, must be kept at reset value.
Bit 13 CC2OIE: Capture/compare 2 over-capture interrupt enable
0: CC2 over-capture interrupt disabled
1: CC2 over-capture interrupt enabled
Note: If LPTIM does not implement at least 2 channels this bit is reserved. Refer to Section 58.3.
Bit 12 CC1OIE: Capture/compare 1 over-capture interrupt enable
0: CC1 over-capture interrupt disabled
1: CC1 over-capture interrupt enabled
Note: If LPTIM does not implement at least 1 channel this bit is reserved. Refer to Section 58.3.
Bit 11 Reserved, must be kept at reset value.
Bit 10 Reserved, must be kept at reset value.
Bit 9 CC2IE: Capture/compare 2 interrupt enable
0: Capture/compare 2 interrupt disabled
1: Capture/compare 2 interrupt enabled
Note: If LPTIM does not implement at least 2 channels this bit is reserved. Refer to Section 58.3.
Bit 8 REPOKIE: Repetition register update OK interrupt Enable
0: Repetition register update OK interrupt disabled
1: Repetition register update OK interrupt enabled
Bit 7 UEIE: Update event interrupt enable
0: Update event interrupt disabled
1: Update event interrupt enabled

RM0456 Rev 6

<!-- pagebreak -->

2526

Low-power timer (LPTIM)

RM0456

Bit 6 DOWNIE: Direction change to down Interrupt Enable
0: DOWN interrupt disabled
1: DOWN interrupt enabled
Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Refer to Section 58.3.
Bit 5 UPIE: Direction change to UP Interrupt Enable
0: UP interrupt disabled
1: UP interrupt enabled
Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Refer to Section 58.3.
Bit 4 ARROKIE: Autoreload register update OK Interrupt Enable
0: ARROK interrupt disabled
1: ARROK interrupt enabled
Bit 3 Reserved, must be kept at reset value.
Bit 2 EXTTRIGIE: External trigger valid edge Interrupt Enable
0: EXTTRIG interrupt disabled
1: EXTTRIG interrupt enabled
Bit 1 ARRMIE: Autoreload match Interrupt Enable
0: ARRM interrupt disabled
1: ARRM interrupt enabled
Bit 0 CC1IE: Capture/compare 1 interrupt enable
0: Capture/compare 1 interrupt disabled
1: Capture/compare 1 interrupt enabled

Caution:

The LPTIMx_DIER register must only be modified when the LPTIM is enabled (ENABLE bit
set to 1). After a write to the LPTIMx_DIER register, a new write operation to the same
register can only be performed when the previous write operation is completed. Any
successive write before the DIEROK flag is set, leads to unpredictable results.

58.7.10

LPTIM configuration register (LPTIM_CFGR)
Address offset: 0x00C
Reset value: 0x0000 0000

31

30

29

28

27

26

25

24

Res.

Res.

Res.

Res.

Res.

Res.

Res.

ENC

15

14

13

12

11

10

9

TRIGSEL[2:0]
rw

rw

Res.
rw

PRESC[2:0]
rw

rw

COUNT PRE
MODE LOAD

21
WAV
POL

20

19

WAVE TIMOUT

18

17

16

TRIGEN[1:0]

Res.

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

TRGFLT[1:0]
rw

Bits 31:30 Reserved, must be kept at reset value.
Bit 29 Reserved, must be kept at reset value.
Bits 28:25 Reserved, must be kept at reset value.

<!-- pagebreak -->

22

rw

Res.
rw

23

RM0456 Rev 6

rw

Res.

CKFLT[1:0]

CKPOL[1:0]

rw

rw

rw

rw

0
CKSEL
rw

RM0456

Low-power timer (LPTIM)

Bit 24 ENC: Encoder mode enable
The ENC bit controls the Encoder mode
0: Encoder mode disabled
1: Encoder mode enabled
Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Refer to Section 58.3.
Bit 23 COUNTMODE: counter mode enabled
The COUNTMODE bit selects which clock source is used by the LPTIM to clock the counter:
0: the counter is incremented following each internal clock pulse
1: the counter is incremented following each valid clock pulse on the LPTIM external Input1
Bit 22 PRELOAD: Registers update mode
The PRELOAD bit controls the LPTIM_ARR, LPTIM_RCR and the LPTIM_CCRx registers update
modality
0: Registers are updated after each APB bus write access
1: Registers are updated at the end of the current LPTIM period
Bit 21 WAVPOL: Waveform shape polarity
The WAVPOL bit controls the output polarity
0: The LPTIM output reflects the compare results between LPTIM_CNT and LPTIM_CCRx
registers
1: The LPTIM output reflects the inverse of the compare results between LPTIM_CNT and
LPTIM_CCRx registers
Note: If the LPTIM implements at least one capture/compare channel, this bit is reserved. Refer to
Section 58.3.
Bit 20 WAVE: Waveform shape
The WAVE bit controls the output shape
0: Deactivate Set-once mode
1: Activate the Set-once mode
Bit 19 TIMOUT: Timeout enable
The TIMOUT bit controls the Timeout feature
0: A trigger event arriving when the timer is already started is ignored
1: A trigger event arriving when the timer is already started resets and restarts the LPTIM counter
and the repetition counter
Bits 18:17 TRIGEN[1:0]: Trigger enable and polarity
The TRIGEN bits controls whether the LPTIM counter is started by an external trigger or not. If the
external trigger option is selected, three configurations are possible for the trigger active edge:
00: software trigger (counting start is initiated by software)
01: rising edge is the active edge
10: falling edge is the active edge
11: both edges are active edges
Bit 16 Reserved, must be kept at reset value.

RM0456 Rev 6

<!-- pagebreak -->

2526

Low-power timer (LPTIM)

RM0456

Bits 15:13 TRIGSEL[2:0]: Trigger selector
The TRIGSEL bits select the trigger source that serves as a trigger event for the LPTIM among the
below 8 available sources:
000: lptim_ext_trig0
001: lptim_ext_trig1
010: lptim_ext_trig2
011: lptim_ext_trig3
100: lptim_ext_trig4
101: lptim_ext_trig5
110: lptim_ext_trig6
111: lptim_ext_trig7
See Section 58.4.3: LPTIM input and trigger mapping for details.
Bit 12 Reserved, must be kept at reset value.
Bits 11:9 PRESC[2:0]: Clock prescaler
The PRESC bits configure the prescaler division factor. It can be one among the following division
factors:
000: /1
001: /2
010: /4
011: /8
100: /16
101: /32
110: /64
111: /128
Bit 8 Reserved, must be kept at reset value.
Bits 7:6 TRGFLT[1:0]: Configurable digital filter for trigger
The TRGFLT value sets the number of consecutive equal samples that are detected when a level
change occurs on an internal trigger before it is considered as a valid level transition. An internal
clock source must be present to use this feature
00: any trigger active level change is considered as a valid trigger
01: trigger active level change must be stable for at least 2 clock periods before it is considered as
valid trigger.
10: trigger active level change must be stable for at least 4 clock periods before it is considered as
valid trigger.
11: trigger active level change must be stable for at least 8 clock periods before it is considered as
valid trigger.
Bit 5 Reserved, must be kept at reset value.
Bits 4:3 CKFLT[1:0]: Configurable digital filter for external clock
The CKFLT value sets the number of consecutive equal samples that are detected when a level
change occurs on an external clock signal before it is considered as a valid level transition. An
internal clock source must be present to use this feature
00: any external clock signal level change is considered as a valid transition
01: external clock signal level change must be stable for at least 2 clock periods before it is
considered as valid transition.
10: external clock signal level change must be stable for at least 4 clock periods before it is
considered as valid transition.
11: external clock signal level change must be stable for at least 8 clock periods before it is
considered as valid transition.

<!-- pagebreak -->

