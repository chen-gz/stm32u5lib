RM0456 Rev 6

RM0456

LCD-TFT display controller (LTDC)

43.7.9

LTDC interrupt status register (LTDC_ISR)
Address offset: 0x038
Reset value: 0x0000 0000
This register returns the interrupt status flag.

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

RRIF

TERRI
F

FUIF

LIF

r

r

r

r

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

Bits 31:4 Reserved, must be kept at reset value.
Bit 3 RRIF: Register reload interrupt flag
0: No register reload interrupt generated
1: Register reload interrupt generated when a vertical blanking reload occurs (and the first
line after the active area is reached)
Bit 2 TERRIF: Transfer error interrupt flag
0: No transfer error interrupt generated
1: Transfer error interrupt generated when a bus error occurs
Bit 1 FUIF: FIFO underrun interrupt flag
0: No FIFO underrun interrupt generated
1: FIFO underrun interrupt generated, if one of the layer FIFOs is empty and pixel data is
read from the FIFO
Bit 0 LIF: Line interrupt flag
0: No line interrupt generated
1: Line interrupt generated when a programmed line is reached

43.7.10

LTDC interrupt clear register (LTDC_ICR)
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

CRRIF
w

CTERR
CFUIF
IF
w

w

CLIF
w

Bits 31:4 Reserved, must be kept at reset value.
Bit 3 CRRIF: Clear register reload interrupt flag
0: No effect
1: Clear the RRIF flag in the LTDC_ISR register

RM0456 Rev 6

<!-- pagebreak -->

1741

LCD-TFT display controller (LTDC)

RM0456

Bit 2 CTERRIF: Clear the transfer error interrupt flag
0: No effect
1: Clear the TERRIF flag in the LTDC_ISR register.
Bit 1 CFUIF: Clear the FIFO underrun interrupt flag
0: No effect
1: Clear the FUDERRIF flag in the LTDC_ISR register.
Bit 0 CLIF: Clear the line interrupt flag
0: No effect
1: Clear the LIF flag in the LTDC_ISR register.

43.7.11

LTDC line interrupt position configuration register (LTDC_LIPCR)
Address offset: 0x040
Reset value: 0x0000 0000
This register defines the position of the line interrupt. The line value to be programmed
depends on the timings parameters. Refer to Figure 408.

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

19

18

17

16

15

14

13

12

11

Res.

Res.

Res.

Res.

Res.

LIPOS[10:0]
rw

rw

rw

rw

rw

rw

Bits 31:11 Reserved, must be kept at reset value.
Bits 10:0 LIPOS[10:0]: Line interrupt position
These bits configure the line interrupt position.

43.7.12

LTDC current position status register (LTDC_CPSR)
Address offset: 0x044
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

CXPOS[15:0]
r

r

r

r

r

r

r

r

r

r

r

r

r

r

r

r

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

r

r

r

r

r

r

r

CYPOS[15:0]
r

r

Bits 31:16 CXPOS[15:0]: Current X position
These bits return the current X position.
Bits 15:0 CYPOS[15:0]: Current Y position
These bits return the current Y position.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

LCD-TFT display controller (LTDC)

43.7.13

LTDC current display status register (LTDC_CDSR)
Address offset: 0x048
Reset value: 0x0000 000F
This register returns the status of the current display phase which is controlled by the
HSYNC, VSYNC, and horizontal/vertical DE signals.
Example: if the current display phase is the vertical synchronization, the VSYNCS bit is set
(active high). If the current display phase is the horizontal synchronization, the HSYNCS bit
is active high.
The returned status does not depend on the configured polarity in the LTDC_GCR register,
instead it returns the current active display phase.

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

HDES

VDES

r

r

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

HSYNC VSYNC
S
S
r

r

Bits 31:4 Reserved, must be kept at reset value.
Bit 3 HSYNCS: Horizontal synchronization display status
0: Active low
1: Active high
Bit 2 VSYNCS: Vertical synchronization display status
0: Active low
1: Active high
Bit 1 HDES: Horizontal data enable display status
0: Active low
1: Active high
Bit 0 VDES: Vertical data enable display status
0: Active low
1: Active high

43.7.14

LTDC layer x control register (LTDC_LxCR)
Address offset: 0x084 + 0x80 * (x - 1), (x = 1 to 2)
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

CLUTE
N

Res.

COLKE
N

LEN

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

rw

RM0456 Rev 6

Res.

<!-- pagebreak -->

1741

LCD-TFT display controller (LTDC)

RM0456

Bits 31:5 Reserved, must be kept at reset value.
Bit 4 CLUTEN: Color look-up table enable
This bit is set and cleared by software.
0: Color look-up table disable
1: Color look-up table enable
The CLUT is only meaningful for L8, AL44 and AL88 pixel format. Refer to Color look-up
table (CLUT)
Bits 3:2 Reserved, must be kept at reset value.
Bit 1 COLKEN: Color keying enable
This bit is set and cleared by software.
0: Color keying disable
1: Color keying enable
Bit 0 LEN: Layer enable
This bit is set and cleared by software.
0: Layer disabled
1: Layer enabled

43.7.15

LTDC layer x window horizontal position configuration register
(LTDC_LxWHPCR)
Address offset: 0x088 + 0x80 * (x - 1), (x = 1 to 2)
Reset value: 0x0000 0000
This register defines the horizontal position (first and last pixel) of the layer 1 or 2 window.
The first visible pixel of a line is the programmed value of AHBP[11:0] bits + 1 in the
LTDC_BPCR register.
The last visible pixel of a line is the programmed value of AAW[11:0] bits in the
LTDC_AWCR register.
Example: The LTDC_BPCR register is configured to 0x000E0005 (AHBP[11:0] is 0xE) and
the LTDC_AWCR register is configured to 0x028E01E5 (AAW[11:0] is 0x28E). To configure
the horizontal position of a window size of 630x460, with horizontal start offset of 5 pixels in
the active data area:
•

layer window first pixel, WHSTPOS[11:0], must be programmed to 0x14 (0xE+1+0x5).

•

layer window last pixel, WHSPPOS[11:0], must be programmed to 0x28A.

31

30

29

28

Res.

Res.

Res.

Res.

15

14

13

12

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

11

10

9

8

7

6

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

WHSPPOS[11:0]

WHSTPOS[11:0]
rw

rw

rw

rw

rw

rw

rw

Bits 31:28 Reserved, must be kept at reset value.
Bits 27:16 WHSPPOS[11:0]: Window horizontal stop position
These bits configure the last visible pixel of a line of the layer window.
WHSPPOS[11:0] must be ≥ AHBP[11:0] bits + 1 (programmed in LTDC_BPCR register).

<!-- pagebreak -->

RM0456 Rev 6

RM0456

LCD-TFT display controller (LTDC)

Bits 15:12 Reserved, must be kept at reset value.
Bits 11:0 WHSTPOS[11:0]: Window horizontal start position
These bits configure the first visible pixel of a line of the layer window.
WHSTPOS[11:0] must be ≤ AAW[11:0] bits (programmed in LTDC_AWCR register).

43.7.16

LTDC layer x window vertical position configuration register
(LTDC_LxWVPCR)
Address offset: 0x08C + 0x80 * (x - 1), (x = 1 to 2)
Reset value: 0x0000 0000
This register defines the vertical position (first and last line) of the layer1 or 2 window.
The first visible line of a frame is the programmed value of AVBP[10:0] bits + 1 in the register
LTDC_BPCR register.
The last visible line of a frame is the programmed value of AAH[10:0] bits in the
LTDC_AWCR register.
Example:
The LTDC_BPCR register is configured to 0x000E0005 (AVBP[10:0] is 0x5) and the
LTDC_AWCR register is configured to 0x028E01E5 (AAH[10:0] is 0x1E5).
To configure the vertical position of a window size of 630x460, with vertical start offset of
eight lines in the active data area:
•

layer window first line, WVSTPOS[10:0], must be programmed to 0xE (0x5 + 1 + 0x8).

•

layer window last line, WVSPPOS[10:0] must be programmed to 0x1DA.

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

15

14

13

12

11

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

19

18

17

16

WVSPPOS[10:0]
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

WVSTPOS[10:0]
rw

rw

rw

rw

rw

rw

Bits 31:27 Reserved, must be kept at reset value.
Bits 26:16 WVSPPOS[10:0]: Window vertical stop position
These bits configure the last visible line of the layer window.
WVSPPOS[10:0] must be ≥ AVBP[10:0] bits + 1 (programmed in LTDC_BPCR register).
Bits 15:11 Reserved, must be kept at reset value.
Bits 10:0 WVSTPOS[10:0]: Window vertical start position
These bits configure the first visible line of the layer window.
WVSTPOS[10:0] must be ≤ AAH[10:0] bits (programmed in LTDC_AWCR register).

RM0456 Rev 6

<!-- pagebreak -->

1741

LCD-TFT display controller (LTDC)

43.7.17

RM0456

LTDC layer x color keying configuration register
(LTDC_LxCKCR)
Address offset: 0x090 + 0x80 * (x - 1), (x = 1 to 2)
Reset value: 0x0000 0000
This register defines the color key value (RGB), that is used by the color keying.

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

CKGREEN[7:0]
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

CKRED[7:0]

CKBLUE[7:0]
rw

rw

Bits 31:24 Reserved, must be kept at reset value.
Bits 23:16 CKRED[7:0]: Color key red value
Bits 15:8 CKGREEN[7:0]: Color key green value
Bits 7:0 CKBLUE[7:0]: Color key blue value

43.7.18

LTDC layer x pixel format configuration register
(LTDC_LxPFCR)
Address offset: 0x094 + 0x80 * (x - 1), (x = 1 to 2)
Reset value: 0x0000 0000
This register defines the pixel format that is used for the stored data in the frame buffer of a
layer. The pixel data is read from the frame buffer and then transformed to the internal
format 8888 (ARGB).

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

2

1

0

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

PF[2:0]
rw

Bits 31:3 Reserved, must be kept at reset value.
Bits 2:0 PF[2:0]: Pixel format
These bits configure the pixel format
000: ARGB8888
001: RGB888
010: RGB565
011: ARGB1555
100: ARGB4444
101: L8 (8-bit luminance)
110: AL44 (4-bit alpha, 4-bit luminance)
111: AL88 (8-bit alpha, 8-bit luminance)

<!-- pagebreak -->

RM0456 Rev 6

rw

rw

RM0456

LCD-TFT display controller (LTDC)

43.7.19

LTDC layer x constant alpha configuration register
(LTDC_LxCACR)
Address offset: 0x098 + 0x80 * (x - 1), (x = 1 to 2)
Reset value: 0x0000 00FF
This register defines the constant alpha value (divided by 255 by hardware), that is used in
the alpha blending. Refer to LTDC_LxBFCR register.

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

CONSTA[7:0]
rw

rw

Bits 31:8 Reserved, must be kept at reset value.
Bits 7:0 CONSTA[7:0]: Constant alpha
These bits configure the constant alpha used for blending. The constant alpha is divided by
255 by hardware.
Example: if the programmed constant alpha is 0xFF, the constant alpha value is
255 / 255 = 1.

43.7.20

LTDC layer x default color configuration register
(LTDC_LxDCCR)
Address offset: 0x09C + 0x80 * (x - 1), (x = 1 to 2)
Reset value: 0x0000 0000
This register defines the default color of a layer in the format ARGB. The default color is
used outside the defined layer window or when a layer is disabled. The reset value of
0x00000000 defines a transparent black color.

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

DCALPHA[7:0]

20

19

18

17

16

DCRED[7:0]

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

DCGREEN[7:0]
rw

rw

DCBLUE[7:0]
rw

rw

Bits 31:24 DCALPHA[7:0]: Default color alpha
These bits configure the default alpha value.
Bits 23:16 DCRED[7:0]: Default color red
These bits configure the default red value.
Bits 15:8 DCGREEN[7:0]: Default color green
These bits configure the default green value.
Bits 7:0 DCBLUE[7:0]: Default color blue
These bits configure the default blue value.

RM0456 Rev 6

<!-- pagebreak -->

1741

LCD-TFT display controller (LTDC)

43.7.21

RM0456

LTDC layer x blending factors configuration register
(LTDC_LxBFCR)
Address offset: 0x0A0 + 0x80 * (x - 1), (x = 1 to 2)
Reset value: 0x0000 0607
This register defines the blending factors F1 and F2.
The general blending formula is: BC = BF1 x C + BF2 x Cs
•

BC = blended color

•

BF1 = blend factor 1

•

C = current layer color

•

BF2 = blend factor 2

•

Cs = subjacent layers blended color

The constant alpha value, is the programmed value in LTDC_LxCACR divided by 255 by
hardware.
Example: Only layer1 is enabled, BF1 configured to constant alpha. BF2 configured to
1 - constant alpha. The constant alpha programmed in LTDC_LxCACR is 240 (0xF0). Thus,
the constant alpha value is 240 / 255 = 0.94. C: current layer color is 128.
Cs: background color is 48. Layer1 is blended with the background color.
BC = constant alpha x C + (1 - Constant Alpha) x Cs = 0.94 x 128 + (1- 0.94) x 48 = 123.
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

10

9

8

2

1

0

15

14

13

12

11

Res.

Res.

Res.

Res.

Res.

BF1[2:0]
rw

rw

7

6

5

4

3

Res.

Res.

Res.

Res.

Res.

rw

Bits 31:11 Reserved, must be kept at reset value.
Bits 10:8 BF1[2:0]: Blending factor 1
These bits select the blending factor F1.
100: constant alpha
110: pixel alpha x constant alpha
Others: Reserved
Bits 7:3 Reserved, must be kept at reset value.
Bits 2:0 BF2[2:0]: blending factor 2
These bits select the blending factor F2
101: 1 - constant alpha
111: 1 - (pixel alpha x constant alpha)
Others: Reserved

<!-- pagebreak -->

RM0456 Rev 6

BF2[2:0]
rw

rw

rw

RM0456

LCD-TFT display controller (LTDC)

43.7.22

LTDC layer x color frame buffer address register
(LTDC_LxCFBAR)
Address offset: 0x0AC + 0x80 * (x - 1), (x = 1 to 2)
Reset value: 0x0000 0000
This register defines the color frame buffer start address which has to point to the address
where the pixel data of the top left pixel of a layer is stored in the frame buffer.

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

CFBADD[31:16]
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

CFBADD[15:0]
rw

rw

Bits 31:0 CFBADD[31:0]: Color frame buffer start address
These bits define the color frame buffer start address.

43.7.23

LTDC layer x color frame buffer length register
(LTDC_LxCFBLR)
Address offset: 0x0B0 + 0x80 * (x - 1), (x = 1 to 2)
Reset value: 0x0000 0000
This register defines the color frame buffer line length and pitch.
Example:
•

A frame buffer having the format RGB565 (2 bytes per pixel) and a width of 256 pixels
(total number of bytes per line is 256 * 2 = 512), where pitch = line length requires a
value of 0x02000203 to be written into this register.

•

A frame buffer having the format RGB888 (3 bytes per pixel) and a width of 320 pixels
(total number of bytes per line is 320 * 3 = 960), where pitch = line length requires a
value of 0x03C003C3 to be written into this register.

31

30

29

Res.

Res.

Res.

15

14

13

Res.

Res.

Res.

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

rw

rw

rw

rw

rw

rw

rw

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

CFBP[12:0]
rw

rw

rw

rw

rw

rw

12

11

10

9

8

7

CFBLL[12:0]
rw

rw

rw

rw

rw

rw

rw

Bits 31:29 Reserved, must be kept at reset value.
Bits 28:16 CFBP[12:0]: Color frame buffer pitch in bytes
These bits define the pitch that is the increment from the start of one line of pixels to the start
of the next line in bytes.
Bits 15:13 Reserved, must be kept at reset value.

RM0456 Rev 6

<!-- pagebreak -->

1741

LCD-TFT display controller (LTDC)

RM0456

Bits 12:0 CFBLL[12:0]: Color frame buffer line length
These bits define the length of one line of pixels in bytes + 3.
The line length is computed as follows:
active high width * number of bytes per pixel + 3.

43.7.24

LTDC layer x color frame buffer line number register
(LTDC_LxCFBLNR)
Address offset: 0x0B4 + 0x80 * (x - 1), (x = 1 to 2)
Reset value: 0x0000 0000
This register defines the number of lines in the color frame buffer.
The number of lines and line length settings define how much data is fetched per frame for
every layer. If it is configured to less bytes than required, a FIFO underrun interrupt is
generated if enabled.
The start address and pitch settings on the other hand define the correct start of every line in
memory.

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

15

14

13

12

11

Res.

Res.

Res.

Res.

Res.

CFBLNBR[10:0]
rw

rw

rw

rw

rw

rw

Bits 31:11 Reserved, must be kept at reset value.
Bits 10:0 CFBLNBR[10:0]: Frame buffer line number
These bits define the number of lines in the frame buffer that corresponds to the active high
width.

43.7.25

LTDC layer x CLUT write register (LTDC_LxCLUTWR)
Address offset: 0x0C4 + 0x80 * (x - 1), (x = 1 to 2)
Reset value: 0x0000 0000
This register defines the CLUT address and the RGB value.
The CLUT write register must be configured only during blanking period or if the layer is
disabled. The CLUT can be enabled or disabled in the LTDC_LxCR register.
The CLUT is only meaningful for L8, AL44 and AL88 pixel format.

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

CLUTADD[7:0]

20

19

18

17

16

RED[7:0]

w

w

w

w

w

w

w

w

w

w

w

w

w

w

w

w

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

w

w

w

GREEN[7:0]
w

w

<!-- pagebreak -->

