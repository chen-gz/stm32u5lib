1500

Comparator (COMP)

RM0456

Bit 15 POLARITY: COMP1 polarity selector
This bit is controlled by the software (if not locked). It selects the COMP1 output polarity.
0: Non-inverted
1: Inverted
Bit 14 WINOUT: COMP1 output selector
This bit is controlled by the software (if not locked). It selects the COMP1 output.
This bit must be kept at zero when window comparator feature is not supported.
0: COMP1_VALUE
1: COMP1_VALUE XOR COMP2_VALUE (required for window mode, see Figure 319)
Bits 13:12 Reserved, must be kept at reset value.
Bit 11 WINMODE: COMP1 non-inverting input selector for window mode
This bit is controlled by the software (if not locked). It selects the signal for the COMP1_INP
input of the COMP1.
This bit must be kept at zero when window comparator feature is not supported.
0: Signal selected with INPSEL[1:0]
1: COMP2_INP signal of COMP2 (required for window mode, see Figure 319)
Bits 10:8 INPSEL[2:0]: COMP1 signal selector for non-inverting input
This field is controlled by the software (if not locked). It selects the signal for the noninverting input COMP1_INP (see Table 355 for the assignment).
Bits 7:4 INMSEL[3:0]: COMP1 signal selector for inverting input INM
This field is controlled by the software (if not locked). It selects the signal for the inverting
input COMP1_INM (see Table 356 for the assignment).
Bits 3:1 Reserved, must be kept at reset value.
Bit 0 EN: COMP1 enable
This bit is controlled by the software (if not locked). It enables COMP1.
0: COMP1 disabled
1: COMP1 enabled

37.7.2

COMP2 control and status register (COMP2_CSR)
Address offset: 0x04
Reset value: 0x0000 0000

31

30

29

28

27

26

25

LOCK

VALUE

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

Res.

WIN
MODE

Res.

INPSEL[1:0]

rw

r

15

14

POLARITY

WIN
OUT

rw

rw

<!-- pagebreak -->

Res.

rw

rw

24

23

22

21

20

BLANKSEL[4:0]

19

18

PWRMODE[1:0]

17

16

HYST[1:0]

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

Res.

Res.

Res.

EN

rw

INMSEL[3:0]
rw

RM0456 Rev 6

rw

rw

rw

rw

RM0456

Comparator (COMP)

Bit 31 LOCK: COMP2_CSR register lock
This bit is set by the software and cleared by reset. It locks the whole content of
COMP2_CSR.
0: COMP2_CSR read/write bits can be written by the software.
1: COMP2_CSR bits can be read but not written by the software.
Bit 30 VALUE: COMP2 output status
This bit is read-only. It reflects the level of the COMP2 output after the polarity selector and
blanking (see Figure 321).
Bits 29:25 Reserved, must be kept at reset value.
Bits 24:20 BLANKSEL[4:0]: COMP2 blanking source selector
This field is controlled by the software (if not locked) and selects the PWM signal for
comparator output-blanking (see Table 360 for the assignment).
Bits 19:18 PWRMODE[1:0]: COMP2 power mode selector
This bitfield is controlled by the software (if not locked). It selects the power consumption
and, as a consequence, the speed of the COMP2.
00: High speed
01-10: Medium speed and power
11: Ultra-low-power
Bits 17:16 HYST[1:0]: COMP2 hysteresis selector
This bitfield is controlled by the software (if not locked). It selects the COMP2 hysteresis.
00: None
01: Low hysteresis
10: Medium hysteresis
11: High hysteresis
Bit 15 POLARITY: COMP2 polarity selector
This bit is controlled by the software (if not locked). It selects the COMP2 output polarity.
0: Non-inverted
1: Inverted
Bit 14 WINOUT: COMP2 output selector
This bit is controlled by the software (if not locked). It selects the COMP2 output.
0: COMP2_VALUE
1: COMP1_VALUE XOR COMP2_VALUE (required for window mode, see Figure 319)
Bits 13:12 Reserved, must be kept at reset value.
Bit 11 WINMODE: COMP2 non-inverting input selector for window mode
This bit is controlled by the software (if not locked). It selects the signal for the COMP2_INP
input of the COMP2.
0: Signal selected with INPSEL[1:0]
1: COMP1_INP signal of COMP1 (required for window mode, see Figure 319)
Bit 10 Reserved, must be kept at reset value.
Bits 9:8 INPSEL[1:0]: COMP2 signal selector for non-inverting input
This field is controlled by the software (if not locked). It selects the signal for the noninverting input COMP2_INP (see Table 357 for the assignment).

RM0456 Rev 6

<!-- pagebreak -->

