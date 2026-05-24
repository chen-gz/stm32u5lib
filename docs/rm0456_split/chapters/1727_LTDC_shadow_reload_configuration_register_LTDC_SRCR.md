RM0456 Rev 6

RM0456

LCD-TFT display controller (LTDC)

43.7.6

LTDC shadow reload configuration register (LTDC_SRCR)
Address offset: 0x024
Reset value: 0x0000 0000
This register allows to reload either immediately or during the vertical blanking period, the
shadow registers values to the active registers. The shadow registers are all Layer1 and
Layer2 registers except the LTDC_L1CLUTWR and the LTDC_L2CLUTWR.
The shadow registers read back the active values. Until the reload has been done, the 'old'
value is read.

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

Res.

Res.

Res.

Res.

Res.

Res.

VBR

IMR

rw

rw

Bits 31:2 Reserved, must be kept at reset value.
Bit 1 VBR: Vertical blanking reload
This bit is set by software and cleared only by hardware after reload (it cannot be cleared
through register write once it is set).
0: No effect
1: The shadow registers are reloaded during the vertical blanking period (at the beginning of
the first line after the active display area).
Bit 0 IMR: Immediate reload
This bit is set by software and cleared only by hardware after reload.
0: No effect
1: The shadow registers are reloaded immediately.

43.7.7

LTDC background color configuration register (LTDC_BCCR)
Address offset: 0x02C
Reset value: 0x0000 0000
This register defines the background color (RGB888).

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

Res.

23

22

21

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

rw

rw

rw

rw

rw

rw

rw

rw

rw

BCGREEN[7:0]
rw

rw

20

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

rw

rw

rw

BCRED[7:0]

BCBLUE[7:0]
rw

rw

Bits 31:24 Reserved, must be kept at reset value.
Bits 23:16 BCRED[7:0]: Background color red value
These bits configure the background red value.

RM0456 Rev 6

<!-- pagebreak -->

