3631

Debug support (DBG)

RM0456

Bit 5 TRACE_EN: trace port and clock enable.
This bit enables the trace port clock, TRACECK.
0: disabled
1: enabled
Bit 4 TRACE_IOEN: trace pin enable
0: disabled - trace pins not assigned
1: enabled - trace pins assigned according to the value of TRACE_MODE field
Bit 3 Reserved, must be kept at reset value.
Bit 2 DBG_STANDBY: Allows debug in Standby mode
0: normal operation
All clocks are disabled and the core powered down automatically in Standby mode.
1: automatic clock stop/power down disabled
All active clocks and oscillators continue to run during Standby mode, and the core supply is
maintained, allowing full debug capability. On exit from Standby mode, a system reset is
performed.
Bit 1 DBG_STOP: Allows debug in Stop mode
0: normal operation
All clocks are disabled automatically in Stop mode.
1: automatic clock stop disabled
All active clocks and oscillators continue to run during Stop mode, allowing full debug
capability. On exit from Stop mode, the clock settings are set to the Stop mode exit state.
Bit 0 Reserved, must be kept at reset value.

DBGMCU APB1L peripheral freeze register (DBGMCU_APB1LFZR)
Address offset: 0x08
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

DBG_I
2C2_S
TOP

DBG_I
2C1_S
TOP

Res.

Res.

Res.

Res.

Res.

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

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

DBG_I DBG_
WDG_ WWDG
STOP _STOP
rw

rw

DBG_T DBG_T DBG_T DBG_T DBG_T DBG_T
IM7_ST IM6_ST IM5_ST IM4_ST IM3_ST IM2_ST
OP
OP
OP
OP
OP
OP
rw

rw

rw

rw

rw

Bits 31:23 Reserved, must be kept at reset value.
Bit 22 DBG_I2C2_STOP: I2C2 SMBUS timeout stop in debug
0: normal operation. I2C2 SMBUS timeout continues to operate while CPU is in debug mode.
1: stop in debug. I2C2 SMBUS timeout is frozen while CPU is in debug mode.
Bit 21 DBG_I2C1_STOP: I2C1 SMBUS timeout stop in debug
0: normal operation. I2C1 SMBUS timeout continues to operate while CPU is in debug mode.
1: stop in debug. I2C1 SMBUS timeout is frozen while CPU is in debug mode.
Bits 20:13 Reserved, must be kept at reset value.

<!-- pagebreak -->

