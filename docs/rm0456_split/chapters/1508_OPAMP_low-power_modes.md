1514

Operational amplifier (OPAMP)
6.

RM0456

Repeat steps 4 to 5 for:
–

normal mode and offset cal low

–

low-power mode and offset cal high

–

low-power mode and offset cal low

If a mode is not used, the corresponding calibration can be skipped.
All OPAMPs can be calibrated at the same time.
Note:

During the whole calibration phase:
- the external connection of the OPAMP output must not pull up or down currents higher
than 500 µA.
- OPAMODE[1:0] must be set up to 00 or 01 (PGA disable) or 11 (internal follower).

38.4

OPAMP low-power modes
Table 366. Effect of low-power modes on the OPAMP

Mode
Sleep
Stop0/1/2

Description
No effect.
No effect, OPAMP registers content is kept.

Stop3

The OPAMP is disabled; the register content is kept.

Standby

The OPAMP registers are powered down and must be re-initialized after exiting Standby mode.

38.5

OPAMP registers
These registers are only accessible by word.(byte and half-word not supported)

38.5.1

OPAMP1 control/status register (OPAMP1_CSR)
Address offset: 0x00
Reset value: 0x0000 0000

31

30

OPA_R OPAHS
ANGE
M
rw

rw

15

14

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

VP_SE
L

CALOU USERT CALSE
CALON
T
RIM
L
r

rw

rw

rw

rw

VM_SEL[1:0]
rw

Res.

rw

Res.

PGA_GAIN[1:0]
rw

rw

OPAMODE[1:0]
rw

rw

OPALP
OPAEN
M
rw

rw

Bit 31 OPA_RANGE: OPAMP range setting
This bit must be set before enabling the OPAMP and this bit affects all OPAMP instances.
0: reserved
1: OPAMP range set

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Operational amplifier (OPAMP)

Bit 30 OPAHSM: OPAMP high-speed mode
This bit is effective for both normal and low-power modes.
0: normal mode (standard slew rate)
1: increased consumption to improve the slew rate
Bits 29:16 Reserved, must be kept at reset value.
Bit 15 CALOUT: OPAMP calibration output
During the calibration mode, the offset is trimmed when this signal toggles.
Bit 14 USERTRIM: ‘factory’ or ‘user’ offset trimmed values selection
This bit is active for normal and low-power modes.
0: ‘factory’ trim code used
1: ‘user’ trim code used
Bit 13 CALSEL: Calibration selection
0: NMOS calibration (200 mV applied on OPAMP inputs)
1: PMOS calibration (VDDA - 200 mV applied on OPAMP inputs)
Bit 12 CALON: Calibration mode enable
0: normal mode
1: calibration mode (all switches opened by hardware)
Bit 11 Reserved, must be kept at reset value.
Bit 10 VP_SEL: Non-inverted input selection
0: GPIO connected to VINP
1: DAC connected to VINP
Bits 9:8 VM_SEL[1:0]: Inverting input selection
These bits are used only when OPAMODE = 00, 01 or 10.
00: GPIO connected to VINM (valid also in PGA mode for filtering)
01: dedicated low-leakage input connected to VINM (valid also in PGA mode for filtering)
1x: inverting input not externally connected
Bits 7:6 Reserved, must be kept at reset value.
Bits 5:4 PGA_GAIN[1:0]: OPAMP programmable amplifier gain value
00: internal PGA gain 2
01: internal PGA gain 4
10: internal PGA gain 8
11: internal PGA gain 16
Bits 3:2 OPAMODE[1:0]: OPAMP PGA mode
00 and 01: internal PGA disabled
10: internal PGA enabled, gain programmed in PGA_GAIN
11: internal follower
Bit 1 OPALPM: OPAMP low-power mode
The OPAMP must be disabled to change this configuration.
0: normal mode
1: low-power mode
Bit 0 OPAEN: OPAMP enable
0: OPAMP disabled
1: OPAMP enabled

RM0456 Rev 6

<!-- pagebreak -->

