0
0
0
0
0
0
0
0
0

BYTE3[7:0]

0
0
0
0
0

Reserved

0

RM0456 Rev 6
0
0

BYTE2[7:0]

0
0

Refer to Section 2.3 for the register boundary addresses.
0
0
0
0
0
0

BYTE1[7:0]

0
0
0
0

Reset value

Reset value

0
0

Res.

Reset value

Res.

Res.

Reset value

OVR_IE

Res.

Res.
Res.

OVR_RIS

Res.

13
12
11

Res.

Res.

Res.

Res.

Res.

Res.

5

RTT4B

6
CKPOL

0

1

2

3

4

7
DEPOL

Res.

8
Res.

RTT1B

9

Res.
RDYPOL

0

OVR_MIS

Res.

Res.

14
Res.

Res.

Res.

15
ENABLE[2:0]

Res.

Res.

Res.

16
Res.

10

17
Res.

18

19
Res.

0

OVR_ISC

Res.

Res.

Res.

Res.

Reset value

Res.

Res.

Res.

Res.

Res.

Res.

Res.

0

Res.

Res.

Res.

Res.

Res.

Res.

Res.

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

0

Res.

Res.

Res.

Res.

Res.
Res.

Res.

0

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

21

Res.

0

Res.

Res.

Res.

Res.

Res.

Res.

Res.am

Res.

22

Res.

Res.

EDM

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

23

Res.

Res.

20

24

Res.

Res.

DERDYCFG

25

Res.

Res.

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

26

Res.

Res.

Res.

Res.

Res.

0

Res.

Res.

Res.

27

Res.

Res.

0

Res.

Res.

Res.

28

Res.

Res.

0

Res.

Res.

Res.

29

Res.

PSSI_SR

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

30

Res.

1

Res.

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

31

OUTEN
DMAEN

Reset value

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

PSSI_CR

Res.

Res.

Res.

Res.

Res.

Res.

Res.

PSSI_MIS

Res.

0x10

Res.

PSSI_IER

Res.

0x0C
Res.

PSSI_RIS

Res.

0x08

Res.

0x04

Res.

0x00

Res.

Register name
reset value

Res.

Offset

Res.

42.5.8

Res.

Parallel synchronous slave interface (PSSI)
RM0456

Bits 31:24 BYTE3[7:0]: Data byte 3

Bits 23:16 BYTE2[7:0]: Data byte 2
Bits 15:8 BYTE1[7:0]: Data byte 1
Bits 7:0 BYTE0[7:0]: Data byte 0

PSSI register map
Table 427. PSSI register map and reset values

0

0

0

0

Res.

BYTE0[7:0]

0

RM0456

43

LCD-TFT display controller (LTDC)

LCD-TFT display controller (LTDC)
This section only applies to the STM32U599/5A9/5Fx/5Gx devices.

43.1

Introduction
The LCD-TFT (liquid crystal display - thin film transistor) display controller provides a
parallel digital RGB (red, green, blue) and signals for horizontal, vertical synchronization,
pixel clock and data enable as output to interface directly to a variety of LCD and TFT
panels.

43.2

LTDC main features
•

24-bit RGB parallel pixel output; 8 bits-per-pixel (RGB888)

•

2 display layers with dedicated FIFO (64x32-bit)

•

Color look-up table (CLUT) up to 256 color (256x24-bit) per layer

•

Programmable timings for different display panels

•

Programmable background color

•

Programmable polarity for HSYNC, VSYNC and data enable

•

Up to 8 input color formats selectable per layer:

•

–

ARGB8888

–

RGB888

–

RGB565

–

ARGB1555

–

ARGB4444

–

L8 (8-bit luminance or CLUT)

–

AL44 (4-bit alpha + 4-bit luminance)

–

AL88 (8-bit alpha + 8-bit luminance)

Pseudo-random dithering output for low bits per channel
–

Dither width 2 bits for red, green, blue

•

Flexible blending between two layers using alpha value (per pixel or constant)

•

Color keying (transparency color)

•

Programmable window position and size

•

Supports thin film transistor (TFT) color displays

•

AHB master interface with burst of 16 words

•

Up to 4 programmable interrupt events

RM0456 Rev 6

<!-- pagebreak -->

