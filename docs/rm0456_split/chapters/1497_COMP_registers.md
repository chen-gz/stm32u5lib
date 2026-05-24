1.

Configure and enable the EXTI line corresponding to the COMPx output event in
interrupt mode and select the rising, falling or both edges sensitivity.

2.

Configure and enable the NVIC IRQ channel mapped to the corresponding EXTI lines.

3.

Enable the COMPx.

RM0456 Rev 6

RM0456

Comparator (COMP)
Table 362. Interrupt control bits
Interrupt event

Event flag

Enable
control bit

Exit Sleep
mode

COMP1 output

VALUE in COMP1_CSR

through EXTI

Yes

Yes

No

COMP2 output

VALUE in COMP2_CSR

through EXTI

Yes

Yes

No

37.7

COMP registers

37.7.1

COMP1 control and status register (COMP1_CSR)

Exit Stop Exit Standby
modes
mode

Address offset: 0x00
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

rw

r

15

14

POLARITY

WINOUT

rw

rw

13
Res.

12

11

Res.

WIN
MODE
rw

10

9

24

23

rw

21

20

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

INPSEL[2:0]
rw

22

BLANKSEL[4:0]

INMSEL[3:0]
rw

rw

rw

rw

rw

rw

Bit 31 LOCK: COMP1_CSR register lock
This bit is set by the software and cleared by reset. It locks the whole content of
COMP1_CSR.
0: COMP1_CSR read/write bits can be written by the software.
1: COMP1_CSR bits can be read but not written by the software.
Bit 30 VALUE: COMP1 output status
This bit is read-only. It reflects the level of the COMP1 output after the polarity selector and
blanking (see Figure 321).
Bits 29:25 Reserved, must be kept at reset value.
Bits 24:20 BLANKSEL[4:0]: COMP1 blanking source selector
This field is controlled by the software (if not locked) and selects the PWM signal for
comparator output-blanking (see Table 359 for the assignment).
Bits 19:18 PWRMODE[1:0]: COMP1 power mode selector
This bitfield is controlled by the software (if not locked). It selects the power consumption
and, as a consequence, the speed of the COMP1.
00: High speed
01-10: Medium speed and power
11: Ultra-low-power
Bits 17:16 HYST[1:0]: COMP1 hysteresis selector
This bitfield is controlled by the software (if not locked). It selects the COMP1 hysteresis.
00: None
01: Low hysteresis
10: Medium hysteresis
11: High hysteresis

RM0456 Rev 6

<!-- pagebreak -->

