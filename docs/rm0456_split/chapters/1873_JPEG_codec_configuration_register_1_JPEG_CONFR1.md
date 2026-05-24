RM0456 Rev 6

RM0456

JPEG codec (JPEG)

46.5.2

JPEG codec configuration register 1 (JPEG_CONFR1)
Address offset: 0x004
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

YSIZE[15:0]
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

Res.

Res.

Res.

Res.

Res.

Res.

Res.

HDR

NS[1:0]

DE

Res.

rw

rw

rw

COLSPACE[1:0]
rw

rw

rw

NF[1:0]
rw

rw

Bits 31:16 YSIZE[15:0]: Y Size
This field defines the number of lines in source image.
Bits 15:9 Reserved, must be kept at reset value.
Bit 8 HDR: Header processing
This bit enables the header processing (generation/parsing).
0: Disable
1: Enable
Bits 7:6 NS[1:0]: Number of components for scan
This field defines the number of components minus 1 for scan header marker segment.
Bits 5:4 COLSPACE[1:0]: Color space
This filed defines the number of quantization tables minus 1 to insert in the output stream.
00: Grayscale (1 quantization table)
01: YUV (2 quantization tables)
10: RGB (3 quantization tables)
11: CMYK (4 quantization tables)
Bit 3 DE: Codec operation as coder or decoder
This bit selects the code or decode process
0: Code
1: Decode
Bit 2 Reserved, must be kept at reset value.
Bits 1:0 NF[1:0]: Number of color components
This field defines the number of color components minus 1.
00: Grayscale (1 color component)
01: - (2 color components)
10: YUV or RGB (3 color components)
11: CMYK (4 color components)

RM0456 Rev 6

<!-- pagebreak -->

