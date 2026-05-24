891

Chrom-ART Accelerator controller (DMA2D)

20.5.9

RM0456

DMA2D foreground color register (DMA2D_FGCOLR)
Address offset: 0x020
Reset value: 0x0000 0000
This register can only be written when data transfers are disabled. Once the data transfer
started, this register is read-only.

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
Bits 23:16 RED[7:0]: Red value for the A4 or A8 mode of the foreground image
Used also for fixed color FG in memory-to-memory mode with blending and fixed color FG
(BG fetch only with FG and BG PFC active).
Bits 15:8 GREEN[7:0]: Green value for the A4 or A8 mode of the foreground image
Used also for fixed color FG in memory-to-memory mode with blending and fixed color FG
(BG fetch only with FG and BG PFC active).
Bits 7:0 BLUE[7:0]: Blue value for the A4 or A8 mode of the foreground image
Used also for fixed color FG in memory-to-memory mode with blending and fixed color FG
(BG fetch only with FG and BG PFC active).

20.5.10

DMA2D background PFC control register (DMA2D_BGPFCCR)
Address offset: 0x024
Reset value: 0x0000 0000
This register can only be written when data transfers are disabled. Once the data transfer
started, this register is read-only.

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

rw

rw

rw

rw

11

10

9

8

ALPHA[7:0]

CS[7:0]
rw

rw

rw

rw

rw

rw

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

Res.

Res.

START

CCM

rc_w1

rw

rw

3

17

16

AM[1:0]
rw

rw

1

0

rw

rw

2
CM[3:0]

rw

rw

Bits 31:24 ALPHA[7:0]: Alpha value
These bits define a fixed alpha channel value which can replace the original alpha value, or
be multiplied with the original alpha value according to the alpha mode selected with AM[1:0].
Bits 23:22 Reserved, must be kept at reset value.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Chrom-ART Accelerator controller (DMA2D)

Bit 21 RBS: Red/Blue swap
This bit allows to swap Red and Blue to support BGR or ABGR color formats.
0: Regular mode (RGB or ARGB)
1: Swap mode (BGR or ABGR)
Bit 20 AI: Alpha Inverted
This bit inverts the alpha value.
0: Regular alpha
1: Inverted alpha
Bits 19:18 Reserved, must be kept at reset value.
Bits 17:16 AM[1:0]: Alpha mode
These bits define which alpha channel value to be used for the background image.
00: No modification of the foreground image alpha channel value
01: Replace original background image alpha channel value by ALPHA[7: 0]
10: Replace original background image alpha channel value by ALPHA[7:0] multiplied with
original alpha channel value
Others: Reserved
Bits 15:8 CS[7:0]: CLUT size
These bits define the size of the CLUT used for the BG.
The number of CLUT entries is equal to CS[7:0] + 1.
Bits 7:6 Reserved, must be kept at reset value.
Bit 5 START: Start
This bit is set to start the automatic loading of the CLUT. This bit is automatically reset:
–
at the end of the transfer
–
when the transfer is aborted by the user by setting ABORT bit in DMA2D_CR
–
when a transfer error occurs
–
when the transfer has not started due to a configuration error or another transfer
operation already on going (data transfer or automatic background CLUT transfer)
Bit 4 CCM: CLUT color mode
These bits define the color format of the CLUT.
0: ARGB8888
1: RGB888
Bits 3:0 CM[3:0]: Color mode
These bits define the color format of the foreground image.
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
Others: Reserved

RM0456 Rev 6

<!-- pagebreak -->

891

Chrom-ART Accelerator controller (DMA2D)

20.5.11

RM0456

DMA2D background color register (DMA2D_BGCOLR)
Address offset: 0x028
Reset value: 0x0000 0000
This register can only be written when data transfers are disabled. Once the data transfer
started, this register is read-only.

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
Bits 23:16 RED[7:0]: Red value for the A4 or A8 mode of the background
Used also for fixed color BG in memory-to-memory mode with blending and fixed color BG
(FG fetch only with FG and BG PFC active).
Bits 15:8 GREEN[7:0]: Green value for the A4 or A8 mode of the background
Used also for fixed color BG in memory-to-memory mode with blending and fixed color BG
(FG fetch only with FG and BG PFC active).
Bits 7:0 BLUE[7:0]: Blue value for the A4 or A8 mode of the background
Used also for fixed color BG in memory-to-memory mode with blending and fixed color BG
(FG fetch only with FG and BG PFC active).

20.5.12

DMA2D foreground CLUT memory address register
(DMA2D_FGCMAR)
Address offset: 0x02C
Reset value: 0x0000 0000
This register can only be written when data transfers are disabled. Once the data transfer
started, this register is read-only.

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
Address of the data used for the CLUT address dedicated to the foreground image.
If the foreground CLUT format is 32-bit, the address must be 32-bit aligned.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Chrom-ART Accelerator controller (DMA2D)

20.5.13

DMA2D background CLUT memory address register
(DMA2D_BGCMAR)
Address offset: 0x030
Reset value: 0x0000 0000
This register can only be written when transfers are disabled. Once the CLUT transfer
started, this register is read-only.

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

rw

rw

rw

rw

rw

rw

rw

rw

MA[15:0]
rw

Bits 31:0 MA[31:0]: Memory address
Address of the data used for the CLUT address dedicated to the background image.
If the background CLUT format is 32-bit, the address must be 32-bit aligned.

20.5.14

DMA2D output PFC control register (DMA2D_OPFCCR)
Address offset: 0x034
Reset value: 0x0000 0000
This register can only be written when transfers are disabled. Once the CLUT transfer
started, this register is read-only.

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

rw

Bits 31:22 Reserved, must be kept at reset value.
Bit 21 RBS: Red/Blue swap
This bit allows to swap Red and Blue to support BGR or ABGR color formats.
0: Regular mode (RGB or ARGB)
1: Swap mode (BGR or ABGR)
Bit 20 AI: Alpha Inverted
This bit inverts the alpha value.
0: Regular alpha
1: Inverted alpha
Bits 19:9 Reserved, must be kept at reset value.

RM0456 Rev 6

<!-- pagebreak -->

891

Chrom-ART Accelerator controller (DMA2D)

RM0456

Bit 8 SB: Swap bytes
When this bit is set, the bytes in the output FIFO are swapped two by two. The number of
pixels per line (PL) must be even, and the output memory address (OMAR) must be even.
0: Bytes in regular order in the output FIFO
1: Bytes swapped two by two in the output FIFO
Bits 7:3 Reserved, must be kept at reset value.
Bits 2:0 CM[2:0]: Color mode
These bits define the color format of the output image.
000: ARGB8888
001: RGB888
010: RGB565
011: ARGB1555
100: ARGB4444
Others: Reserved

20.5.15

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

GREEN[7:0]
rw

rw

BLUE[7:0]
rw

Bits 31:24 ALPHA[7:0]: Alpha channel value of the output color in ARGB8888 mode (otherwise reserved)
Bits 23:16 RED[7:0]: Red value of the output image in ARGB8888 or RGB888 mode
Bits 15:8 GREEN[7:0]: Green value of the output image in ARGB8888 or RGB888
Bits 7:0 BLUE[7:0]: Blue value of the output image in ARGB8888 or RGB888

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Chrom-ART Accelerator controller (DMA2D)

20.5.16

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

20.5.17

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

rw

rw

Bits 31:16 Reserved, must be kept at reset value.
Bit 15 A: Alpha channel value of the output color in ARGB1555 mode
Bits 14:10 RED[4:0]: Red value of the output image in ARGB1555 mode
Bits 9:5 GREEN[4:0]: Green value of the output image in ARGB1555 mode
Bits 4:0 BLUE[4:0]: Blue value of the output image in ARGB1555 mode

RM0456 Rev 6

<!-- pagebreak -->

891

Chrom-ART Accelerator controller (DMA2D)

20.5.18

RM0456

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

20.5.19

DMA2D output memory address register (DMA2D_OMAR)
Address offset: 0x03C
Reset value: 0x0000 0000
This register can only be written when transfers are disabled. Once the CLUT transfer
started, this register is read-only.

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

rw

rw

rw

rw

rw

rw

rw

rw

MA[15:0]
rw

Bits 31:0 MA[31:0]: Memory address
Address of the data used for the output FIFO.
The address alignment must match the image format selected: a 32-bit per pixel format must
be 32-bit aligned and a 16-bit per pixel format must be 16-bit aligned.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Chrom-ART Accelerator controller (DMA2D)

20.5.20

DMA2D output offset register (DMA2D_OOR)
Address offset: 0x040
Reset value: 0x0000 0000
This register can only be written when transfers are disabled. Once the CLUT transfer
started, this register is read-only.

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

LO[15:0]
rw

rw

rw

rw

rw

rw

rw

rw

rw

Bits 31:16 Reserved, must be kept at reset value.
Bits 15:0 LO[15:0]: Line offset
This field gives the line offset used for the output, expressed:
–
in pixels when LOM = 0 in DMA2D_CR. Only LO[13:0] bits are considered,
LO[15:14] bits are ignored.
–
in bytes when LOM = 1
This value is used for the address generation. It is added at the end of each line to determine
the starting address of the next line.

20.5.21

DMA2D number of line register (DMA2D_NLR)
Address offset: 0x044
Reset value: 0x0000 0000
This register can only be written when transfers are disabled. Once the CLUT transfer
started, this register is read-only.

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

22

21

20

19

18

17

16

PL[13:0]
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
Bits 29:16 PL[13:0]: Pixel per lines per lines of the area to be transferred
If any of the input image format is 4-bit per pixel, pixel per lines must be even.
Bits 15:0 NL[15:0]: Number of lines of the area to be transferred.

RM0456 Rev 6

<!-- pagebreak -->

891

Chrom-ART Accelerator controller (DMA2D)

20.5.22

RM0456

DMA2D line watermark register (DMA2D_LWR)
Address offset: 0x048
Reset value: 0x0000 0000
This register can only be written when transfers are disabled. Once the CLUT transfer
started, this register is read-only.

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

LW[15:0]
rw

rw

rw

rw

rw

rw

rw

rw

rw

Bits 31:16 Reserved, must be kept at reset value.
Bits 15:0 LW[15:0]: Line watermark for interrupt generation
An interrupt is raised when the last pixel of the watermarked line has been transferred.

20.5.23

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
Bit 0 EN: Dead-time functionality enable

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Chrom-ART Accelerator controller (DMA2D)

20.5.24

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

20.5.25

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

RM0456 Rev 6

<!-- pagebreak -->

