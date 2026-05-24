RM0456 Rev 6

RM0456

LCD-TFT display controller (LTDC)

Bits 15:11 Reserved, must be kept at reset value.
Bits 10:0 AAH[10:0]: Accumulated active height (in units of horizontal scan line)
These bits define the accumulated height which includes the vertical synchronization, vertical
back porch and the active height lines minus 1. The active height is the number of active
lines in the panel.
Refer to device datasheet for maximum active height supported following maximum pixel
clock.

43.7.4

LTDC total width configuration register (LTDC_TWCR)
Address offset: 0x014
Reset value: 0x0000 0000
This register defines the accumulated number of horizontal synchronization, back porch,
active and front porch pixels minus 1 (HSYNC width + HBP + active width + HFP - 1) and
the accumulated number of vertical synchronization, back porch lines, active and front lines
minus 1 (VSYNC height + VBP + active height + VFP - 1). Refer to Figure 408 and
Section 43.4 for an example of configuration.

31

30

29

28

Res.

Res.

Res.

Res.

27

26

25

24

23

22

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

TOTALW[11:0]

TOTALH[10:0]
rw

Bits 31:28 Reserved, must be kept at reset value.
Bits 27:16 TOTALW[11:0]: Total width (in units of pixel clock period)
These bits define the accumulated total width which includes the horizontal synchronization,
horizontal back porch, active width and horizontal front porch pixels minus 1.
Bits 15:11 Reserved, must be kept at reset value.
Bits 10:0 TOTALH[10:0]: Total height (in units of horizontal scan line)
These bits define the accumulated height which includes the vertical synchronization, vertical
back porch, the active height and vertical front porch height lines minus 1.

43.7.5

LTDC global control register (LTDC_GCR)
Address offset: 0x018
Reset value: 0x0000 2220
This register defines the global configuration of the LCD-TFT controller.

31

30

29

28

HSPOL VSPOL DEPOL PCPOL
rw

rw

rw

rw

15

14

13

12

Res.

DRW[2:0]
r

r

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

DEN
rw

11

10

Res.
r

9

8

DGW[2:0]
r

r

7

6

Res.
r

RM0456 Rev 6

5

4

DBW[2:0]
r

r

3
Res.

r

2
Res.

1

0

Res.

LTDCE
N
rw

<!-- pagebreak -->

1741

LCD-TFT display controller (LTDC)

RM0456

Bit 31 HSPOL: Horizontal synchronization polarity
This bit is set and cleared by software.
0: Horizontal synchronization polarity is active low.
1: Horizontal synchronization polarity is active high.
Bit 30 VSPOL: Vertical synchronization polarity
This bit is set and cleared by software.
0: Vertical synchronization is active low.
1: Vertical synchronization is active high.
Bit 29 DEPOL: Not data enable polarity
This bit is set and cleared by software.
0: Not data enable polarity is active low.
1: Not data enable polarity is active high.
Bit 28 PCPOL: Pixel clock polarity
This bit is set and cleared by software.
0: Pixel clock polarity is active low.
1: Pixel clock is active high.
Bits 27:17 Reserved, must be kept at reset value.
Bit 16 DEN: Dither enable
This bit is set and cleared by software.
0: Dither disabled
1: Dither enabled
Bit 15 Reserved, must be kept at reset value.
Bits 14:12 DRW[2:0]: Dither red width
These bits return the dither red bits.
Bit 11 Reserved, must be kept at reset value.
Bits 10:8 DGW[2:0]: Dither green width
These bits return the dither green bits.
Bit 7 Reserved, must be kept at reset value.
Bits 6:4 DBW[2:0]: Dither blue width
These bits return the dither blue bits.
Bits 3:1 Reserved, must be kept at reset value.
Bit 0 LTDCEN: LCD-TFT controller enable
This bit is set and cleared by software.
0: LTDC disabled
1: LTDC enabled

<!-- pagebreak -->

