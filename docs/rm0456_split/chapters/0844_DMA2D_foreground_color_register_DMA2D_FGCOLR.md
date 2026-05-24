856

Chrom-ART Accelerator controller (DMA2D)

RM0456

Bit 5 START: Start
This bit can be set to start the automatic loading of the CLUT. It is automatically reset:
- at the end of the transfer
- when the transfer is aborted by the user application by setting ABORT in DMA2D_CR
- when a transfer error occurs
- when the transfer has not started due to a configuration error or another transfer operation
already ongoing (data transfer or automatic background CLUT transfer)
Bit 4 CCM: CLUT color mode
This bit defines the color format of the CLUT. It can only be written when the transfer is
disabled. Once the CLUT transfer has started, this bit is read-only.
0: ARGB8888
1: RGB888
Other: Reserved
Bits 3:0 CM[3:0]: Color mode
These bits defines the color format of the foreground image. They can only be written when
data transfers are disabled. Once the transfer has started, they are read-only.
0000: ARGB8888
0001: RGB888
0010: RGB565
0011: ARGB1555
0100: ARGB4444
0101: L8
0110: AL44
0111: AL88
1000: L4
1001: A8
1010: A4
Other: Reserved

19.5.9

DMA2D foreground color register (DMA2D_FGCOLR)
Address offset: 0x020
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

Res.

15

14

13

12

11

10

9

8

23

22

21

rw

rw

rw

rw

19

18

17

16

RED[7:0]
rw

rw

rw

rw

rw

rw

rw

rw

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

GREEN[7:0]
rw

20

BLUE[7:0]
rw

rw

rw

rw

rw

rw

rw

rw

Bits 31:24 Reserved, must be kept at reset value.
Bits 23:16 RED[7:0]: Red value
These bits define the Red value for the A4 or A8 mode of the foreground image. They can
only be written when data transfers are disabled. Once the transfer has started, they are
read-only.
These bits can also be used for fixed color FG in memory-to-memory with blending and fixed
color FG (BG fetch only with FG and BG PFC active) mode.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Chrom-ART Accelerator controller (DMA2D)

Bits 15:8 GREEN[7:0]: Green value
These bits defines the Green value for the A4 or A8 mode of the foreground image. They can
only be written when data transfers are disabled. Once the transfer has started, They are
read-only.
These bits can also be used for fixed color FG in memory-to-memory with blending and fixed
color FG (BG fetch only with FG and BG PFC active) mode.
Bits 7:0 BLUE[7:0]: Blue value
These bits defines the Blue value for the A4 or A8 mode of the foreground image. They can
only be written when data transfers are disabled. Once the transfer has started, They are
read-only.
These bits can also be used for fixed color FG in memory-to-memory with blending and fixed
color FG (BG fetch only with FG and BG PFC active) mode.

19.5.10

DMA2D background PFC control register (DMA2D_BGPFCCR)
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

ALPHA[7:0]
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

rw

rw

rw

rw

rw

rw

rw

CS[7:0]
rw

23

22

21

20

19

18

Res.

Res.

RBS

AI

Res.

Res.

rw

rw

7

6

5

4

3

2

Res.

Res.

START

CCM

rs

rw

rw

rw

17

16

AM[1:0]
rw

rw

1

0

rw

rw

CM[3:0]

Bits 31:24 ALPHA[7:0]: Alpha value
These bits define a fixed alpha channel value which can replace the original alpha value or
be multiplied with the original alpha value according to the alpha mode selected with AM[1:0].
These bits can only be written when data transfers are disabled. Once the transfer has
started, they are read-only.
Bits 23:22 Reserved, must be kept at reset value.
Bit 21 RBS: Red Blue swap
This bit is used to swap the R and B to support BGR or ABGR color formats. Once the
transfer has started, this bit is read-only.
0: Regular mode (RGB or ARGB)
1: Swap mode (BGR or ABGR)
Bit 20 AI: Alpha inverted
This bit inverts the alpha value. Once the transfer has started, this bit is read-only.
0: Regular alpha
1: Inverted alpha
Bits 19:18 Reserved, must be kept at reset value.

RM0456 Rev 6

<!-- pagebreak -->

856

Chrom-ART Accelerator controller (DMA2D)

RM0456

Bits 17:16 AM[1:0]: Alpha mode
These bits define which alpha channel value to be used for the background image. They can
only be written when data transfers are disabled. Once the transfer has started, they are
read-only.
00: No modification of the foreground image alpha channel value
01: Replace original background image alpha channel value by ALPHA[7:0]
10: Replace original background image alpha channel value by ALPHA[7:0] multiplied with
original alpha channel value
Other: Reserved
Bits 15:8 CS[7:0]: CLUT size
These bits define the size of the CLUT used for the BG. Once the CLUT transfer has started,
this field is read-only.
The number of CLUT entries is equal to CS[7:0] + 1.
Bits 7:6 Reserved, must be kept at reset value.
Bit 5 START: Start
This bit is set to start the automatic loading of the CLUT. This bit is automatically reset:
- at the end of the transfer
- when the transfer is aborted by the user application by setting ABORT in DMA2D_CR
- when a transfer error occurs
- when the transfer has not started due to a configuration error or another transfer operation
already ongoing (data transfer or automatic foreground CLUT transfer).
Bit 4 CCM: CLUT color mode
These bits define the color format of the CLUT. This register can only be written when the
transfer is disabled. Once the CLUT transfer has started, this bit is read-only.
0: ARGB8888
1: RGB888
Other: Reserved
Bits 3:0 CM[3:0]: Color mode
These bits define the color format of the foreground image. These bits can only be written
when data transfers are disabled. Once the transfer has started, they are read-only.
0000: ARGB8888
0001: RGB888
0010: RGB565
0011: ARGB1555
0100: ARGB4444
0101: L8
0110: AL44
0111: AL88
1000: L4
1001: A8
1010: A4
Other: Reserved

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Chrom-ART Accelerator controller (DMA2D)

19.5.11

DMA2D background color register (DMA2D_BGCOLR)
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

rw

rw

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

RED[7:0]

GREEN[7:0]
rw

20

BLUE[7:0]
rw

Bits 31:24 Reserved, must be kept at reset value.
Bits 23:16 RED[7:0]: Red value
These bits define the Red value for the A4 or A8 mode of the background. These bits can
only be written when data transfers are disabled. Once the transfer has started, they are
read-only.
These bits are also used for fixed color FG in memory-to-memory with blending and fixed
color FG (BG fetch only with FG and BG PFC active) mode.
Bits 15:8 GREEN[7:0]: Green value
These bits define the green value for the A4 or A8 mode of the background. These bits can
only be written when data transfers are disabled. Once the transfer has started, they are
read-only.
These bits are also used for fixed color FG in memory-to-memory with blending and fixed
color FG (BG fetch only with FG and BG PFC active) mode.
Bits 7:0 BLUE[7:0]: Blue value
These bits define the blue value for the A4 or A8 mode of the background. These bits can
only be written when data transfers are disabled. Once the transfer has started, they are
read-only.
These bits are also used for fixed color FG in memory-to-memory with blending and fixed
color FG (BG fetch only with FG and BG PFC active) mode.

19.5.12

DMA2D foreground CLUT memory address register
(DMA2D_FGCMAR)
Address offset: 0x02C
Reset value: 0x0000 0000

31

30

29

28

27

26

25

24

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

rw

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

MA[31:16]

MA[15:0]
rw

rw

rw

rw

rw

rw

rw

rw

rw

RM0456 Rev 6

<!-- pagebreak -->

856

Chrom-ART Accelerator controller (DMA2D)

RM0456

Bits 31:0 MA[31:0]: Memory address
This field contains the address of the data used for the CLUT address dedicated to the
foreground image. This register can only be written when no transfer is ongoing. Once the
CLUT transfer has started, this register is read-only.
If the foreground CLUT format is 32-bit, the address must be 32-bit aligned.

19.5.13

DMA2D background CLUT memory address register
(DMA2D_BGCMAR)
Address offset: 0x030
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

MA[31:16]
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

MA[15:0]
rw

rw

rw

rw

rw

rw

rw

rw

rw

Bits 31:0 MA[31:0]: Memory address
This field contains the address of the data used for the CLUT address dedicated to the
background image. This register can only be written when no transfer is ongoing. Once the
CLUT transfer has started, this register is read-only.
If the background CLUT format is 32-bit, the address must be 32-bit aligned.

19.5.14

DMA2D output PFC control register (DMA2D_OPFCCR)
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

RBS

AI

Res.

Res.

Res.

Res.

rw

rw
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

SB

Res.

Res.

Res.

Res.

Res.

rw

CM[2:0]
rw

rw

Bits 31:22 Reserved, must be kept at reset value.
Bit 21 RBS: Red Blue swap
This bit is used to swap the R and B to support BGR or ABGR color formats. Once the
transfer has started, this bit is read-only.
0: Regular mode (RGB or ARGB)
1: Swap mode (BGR or ABGR)
Bit 20 AI: Alpha Inverted
This bit inverts the alpha value. Once the transfer has started, this bit is read-only.
0: Regular alpha
1: Inverted alpha

<!-- pagebreak -->

RM0456 Rev 6

rw

RM0456

Chrom-ART Accelerator controller (DMA2D)

Bits 19:9 Reserved, must be kept at reset value.
Bit 8 SB: Swap bytes
When this bit is set, the bytes in the output FIFO are swapped two by two, the number of
pixel per line (PL) must be even, and the output memory address (OMAR) must be even.
This register can only be written when the transfer is disabled. Once the transfer has started,
this register is read-only.
0: Bytes in regular order in the output FIFO
1: Bytes are swapped two by two in the output FIFO
Bits 7:3 Reserved, must be kept at reset value.
Bits 2:0 CM[2:0]: Color mode
These bits define the color format of the output image. These bits can only be written when
data transfers are disabled. Once the transfer has started, they are read-only.
000: ARGB8888
001: RGB888
010: RGB565
011: ARGB1555
100: ARGB4444
Other: Reserved

19.5.15

DMA2D output color register (DMA2D_OCOLR)
Address offset: 0x038
Reset value: 0x0000 0000
The same register is used to show the color values, with different formats depending on the
color mode.
This register can only be written when transfers are disabled. Once the CLUT transfer
started, this register is read-only.
ARGB8888 or RGB888 color mode

31

30

29

28

rw

rw

rw

rw

15

14

13

12

27

26

25

24

23

22

21

20

rw

rw

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

5

4

ALPHA[7:0]

rw

rw

rw

rw

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

RED[7:0]

GREEN[7:0]
rw

19

BLUE[7:0]
rw

rw

rw

rw

rw

rw

rw

rw

Bits 31:24 ALPHA[7:0]: Alpha channel value of the output color in ARGB8888 mode (otherwise reserved)
Bits 23:16 RED[7:0]: Red value of the output image in ARGB8888 or RGB888 mode
Bits 15:8 GREEN[7:0]: Green value of the output image in ARGB8888 or RGB888
Bits 7:0 BLUE[7:0]: Blue value of the output image in ARGB8888 or RGB888

RM0456 Rev 6

<!-- pagebreak -->

856

Chrom-ART Accelerator controller (DMA2D)

19.5.16

RM0456

DMA2D output color register [alternate] (DMA2D_OCOLR)
Address offset: 0x038
Reset value: 0x0000 0000
The same register is used to show the color values, with different formats depending on the
color mode.
This register can only be written when transfers are disabled. Once the CLUT transfer
started, this register is read-only.
RGB565 color mode

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

RED[4:0]
rw

rw

rw

GREEN[5:0]
rw

rw

rw

rw

rw

rw

BLUE[4:0]
rw

rw

rw

rw

rw

Bits 31:16 Reserved, must be kept at reset value.
Bits 15:11 RED[4:0]: Red value of the output image in RGB565 mode
Bits 10:5 GREEN[5:0]: Green value of the output image in RGB565 mode
Bits 4:0 BLUE[4:0]: Blue value of the output image in RGB565 mode

19.5.17

DMA2D output color register [alternate] (DMA2D_OCOLR)
Address offset: 0x038
Reset value: 0x0000 0000
The same register is used to show the color values, with different formats depending on the
color mode.
This register can only be written when transfers are disabled. Once the CLUT transfer
started, this register is read-only.
ARGB1555 color mode

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

A
rw

RED[4:0]
rw

rw

rw

GREEN[4:0]
rw

rw

rw

rw

rw

BLUE[4:0]
rw

rw

rw

Bits 31:16 Reserved, must be kept at reset value.
Bit 15 A: Alpha channel value of the output color in ARGB1555 mode
Bits 14:10 RED[4:0]: Red value of the output image in ARGB1555 mode
Bits 9:5 GREEN[4:0]: Green value of the output image in ARGB1555 mode
Bits 4:0 BLUE[4:0]: Blue value of the output image in ARGB1555 mode

<!-- pagebreak -->

RM0456 Rev 6

rw

rw

RM0456

Chrom-ART Accelerator controller (DMA2D)

19.5.18

DMA2D output color register [alternate] (DMA2D_OCOLR)
Address offset: 0x038
Reset value: 0x0000 0000
The same register is used to show the color values, with different formats depending on the
color mode.
This register can only be written when transfers are disabled. Once the CLUT transfer
started, this register is read-only.
ARGB4444 color mode

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

ALPHA[3:0]
rw

rw

rw

RED[3:0]
rw

rw

rw

rw

GREEN[3:0]
rw

rw

rw

rw

BLUE[3:0]
rw

rw

rw

rw

rw

Bits 31:16 Reserved, must be kept at reset value.
Bits 15:12 ALPHA[3:0]: Alpha channel of the output color value in ARGB4444
Bits 11:8 RED[3:0]: Red value of the output image in ARGB4444 mode
Bits 7:4 GREEN[3:0]: Green value of the output image in ARGB4444 mode
Bits 3:0 BLUE[3:0]: Blue value of the output image in ARGB4444 mode

19.5.19

DMA2D output memory address register (DMA2D_OMAR)
Address offset: 0x003C
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

MA[31:16]
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

MA[15:0]
rw

rw

rw

rw

rw

rw

rw

rw

rw

Bits 31:0 MA[31:0]: Memory Address
This field contains the address of the data used for the output FIFO. These bits can only be
written when data transfers are disabled. Once the transfer has started, they are read-only.
The address alignment must match the image format selected e.g. a 32-bit per pixel format
must be 32-bit aligned and a 16-bit per pixel format must be 16-bit aligned.

RM0456 Rev 6

<!-- pagebreak -->

856

Chrom-ART Accelerator controller (DMA2D)

19.5.20

RM0456

DMA2D output offset register (DMA2D_OOR)
Address offset: 0x040
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

rw

LO[15:0]
rw

Bits 31:16 Reserved, must be kept at reset value.
Bits 15:0 LO[15:0]: Line offset
This field contains the line offset used for the output expressed in pixel when the LOM bit is
reset and in byte when the LOM bit is set.
When expressed in pixels, only LO[13:0] is considered, LO[15:14] are ignored.
This value is used for the address generation. It is added at the end of each line to determine
the starting address of the next line.
These bits can only be written when data transfers are disabled. Once data transfer has
started, they become read-only.

19.5.21

DMA2D number of line register (DMA2D_NLR)
Address offset: 0x044
Reset value: 0x0000 0000

31

30

Res.

Res.

15

14

29

28

27

26

25

24

23

rw

rw

rw

rw

rw

rw

rw

13

12

11

10

9

8

7

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

rw

PL[13:0]

NL[15:0]
rw

rw

rw

rw

rw

rw

rw

rw

rw

Bits 31:30 Reserved, must be kept at reset value.
Bits 29:16 PL[13:0]: Pixel per lines
Number of pixels per lines of the area to be transferred. These bits can only be written when
data transfers are disabled. Once the transfer has started, they are read-only.
If any of the input image format is 4-bit per pixel, pixel per lines must be even.
Bits 15:0 NL[15:0]: Number of lines
Number of lines of the area to be transferred. These bits can only be written when data
transfers are disabled. Once the transfer has started, they are read-only.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Chrom-ART Accelerator controller (DMA2D)

19.5.22

DMA2D line watermark register (DMA2D_LWR)
Address offset: 0x048
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

rw

LW[15:0]
rw

Bits 31:16 Reserved, must be kept at reset value.
Bits 15:0 LW[15:0]: Line watermark
These bits allow the configuration of the line watermark for interrupt generation.
An interrupt is raised when the last pixel of the watermarked line has been transferred.
These bits can only be written when data transfers are disabled. Once the transfer has
started, they are read-only.

19.5.23

DMA2D AHB master timer configuration register (DMA2D_AMTCR)
Address offset: 0x04C
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

EN

rw

rw

rw

rw

rw

rw

rw

DT[7:0]
rw

rw

Bits 31:16 Reserved, must be kept at reset value.
Bits 15:8 DT[7:0]: Dead time
Dead time value in the AHB clock cycle inserted between two consecutive accesses on the
AHB master port. These bits represent the minimum guaranteed number of cycles between
two consecutive AHB accesses.
Bits 7:1 Reserved, must be kept at reset value.
Bit 0 EN: Enable
Enables the dead time functionality.

RM0456 Rev 6

<!-- pagebreak -->

856

Chrom-ART Accelerator controller (DMA2D)

19.5.24

RM0456

DMA2D foreground CLUT (DMA2D_FGCLUTx)
Address offset: 0x400 + 0x4 * x, (x = 0 to 255)
Reset value: 0xXXXX XXXX

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

ALPHA[7:0]

20

19

18

17

16

RED[7:0]

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

19

18

17

16

GREEN[7:0]
rw

rw

BLUE[7:0]

Bits 31:24 ALPHA[7:0]: Alpha
Alpha value for index {x} for the foreground
Bits 23:16 RED[7:0]: Red
Red value for index {x} for the foreground
Bits 15:8 GREEN[7:0]: Green
Green value for index {x} for the foreground
Bits 7:0 BLUE[7:0]: Blue
Blue value for index {x} for the foreground

19.5.25

DMA2D background CLUT (DMA2D_BGCLUTx)
Address offset: 0x800 + 0x4 * x, (x = 0 to 255)
Reset value: 0xXXXX XXXX

31

30

29

28

rw

rw

rw

rw

15

14

13

12

27

26

25

24

23

22

21

20

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

ALPHA[7:0]

RED[7:0]

GREEN[7:0]
rw

rw

rw

rw

rw

BLUE[7:0]
rw

rw

rw

rw

rw

Bits 31:24 ALPHA[7:0]:Alpha
Alpha value for index {x} for the background
Bits 23:16 RED[7:0]: Red
Red value for index {x} for the background
Bits 15:8 GREEN[7:0]: Green
Green value for index {x} for the background
Bits 7:0 BLUE[7:0]: Blue
Blue value for index {x} for the background

<!-- pagebreak -->

