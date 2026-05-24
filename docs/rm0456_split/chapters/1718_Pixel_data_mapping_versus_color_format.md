1741

LCD-TFT display controller (LTDC)

RM0456

Figure 409. Layer window programmable parameters
WVSTPOS bits in
LTDC_LxWVPCR

Active data area

WHSTPOS bits in
LTDC_LxWHPCR

WVSPPOS bits in
LTDC_LxWVPCR

Window

WHSPPOS bits in LTDC_LxWHPCR
MSv19676V4

Pixel input format
The programmable pixel format is used for the data stored in the frame buffer of a layer.
Up to eight input pixel formats can be configured for every layer through the LTDC_LxPFCR
register
The pixel data is read from the frame buffer and then transformed to the internal 8888
(ARGB) format as follows: components having a width of less than 8 bits get expanded to
8 bits by bit replication. The selected bit range is concatenated multiple times until it is
longer than 8 bits. Of the resulting vector, the 8 MSB bits are chosen. Example: 5 bits of an
RGB565 red channel become (bit positions) 43210432 (the three LSBs are filled with the
three MSBs of the five bits)
The table below describes the pixel data mapping depending on the selected format.
Table 432. Pixel data mapping versus color format
ARGB8888
@+3
Ax[7:0]

@+2
Rx[7:0]

@+1
Gx[7:0]

@
Bx[7:0]

@+7
Ax+1[7:0]

@+6
Rx+1[7:0]

@+5
Gx+1[7:0]

@+4
Bx+1[7:0]

RGB888
@+3
Bx+1[7:0]

@+2
Rx[7:0]

@+1
Gx[7:0]

@
Bx[7:0]

@+7
Gx+2[7:0]

@+6
Bx+2[7:0]

@+5
Rx+1[7:0]

@+4
Gx+1[7:0]

RGB565
@+3
Rx+1[4:0] Gx+1[5:3]

@+2
Gx+1[2:0] Bx+1[4:0]

@+1
Rx[4:0] Gx[5:3]

@
Gx[2:0] Bx[4:0]

@+7
Rx+3[4:0] Gx+3[5:3]

@+6
Gx+3[2:0] Bx+3[4:0]

@+5
Rx+2[4:0] Gx+2[5:3]

@+4
Gx+2[2:0] Bx+2[4:0]

ARGB1555

<!-- pagebreak -->

RM0456 Rev 6

RM0456

LCD-TFT display controller (LTDC)
Table 432. Pixel data mapping versus color format (continued)
@+3
Ax+1[0]Rx+1[4:0]
Gx+1[4:3]

@+2
Gx+1[2:0] Bx+1[4:0]

@+1
Ax[0] Rx[4:0] Gx[4:3]

@
Gx[2:0] Bx[4:0]

@+7
Ax+3[0]Rx+3[4:0]
Gx+3[4:3]

@+6
Gx+3[2:0] Bx+3[4:0]

@+5
Ax+2[0]Rx+2[4:0]Gx+2[4:
3]

@+4
Gx+2[2:0] Bx+2[4:0]

ARGB4444
@+3
Ax+1[3:0]Rx+1[3:0]

@+2
Gx+1[3:0] Bx+1[3:0]

@+1
Ax[3:0] Rx[3:0]

@
Gx[3:0] Bx[3:0]

@+7
Ax+3[3:0]Rx+3[3:0]

@+6
Gx+3[3:0] Bx+3[3:0]

@+5
Ax+2[3:0]Rx+2[3:0]

@+4
Gx+2[3:0] Bx+2[3:0]

L8
@+3
Lx+3[7:0]

@+2
Lx+2[7:0]

@+1
Lx+1[7:0]

@
Lx[7:0]

@+7
Lx+7[7:0]

@+6
Lx+6[7:0]

@+5
Lx+5[7:0]

@+4
Lx+4[7:0]

AL44
@+3
Ax+3[3:0] Lx+3[3:0]

@+2
Ax+2[3:0] Lx+2[3:0]

@+1
Ax+1[3:0] Lx+1[3:0]

@
Ax[3:0] Lx[3:0]

@+7
Ax+7[3:0] Lx+7[3:0]

@+6
Ax+6[3:0] Lx+6[3:0]

@+5
Ax+5[3:0] Lx+5[3:0]

@+4
Ax+4[3:0] Lx+4[3:0]

AL88
@+3
Ax+1[7:0]

@+2
Lx+1[7:0]

@+1
Ax[7:0]

@
Lx[7:0]

@+7
Ax+3[7:0]

@+6
Lx+3[7:0]

@+5
Ax+2[7:0]

@+4
Lx+2[7:0]

Color look-up table (CLUT)
The CLUT can be enabled at run-time for every layer through the LTDC_LxCR register and
it is only useful in case of indexed color when using the L8, AL44 and AL88 input pixel
format.
First, the CLUT must be loaded with the R, G and B values that replace the original R, G, B
values of that pixel (indexed color). Each color (RGB value) has its own address that is the
position within the CLUT.
The R, G and B values and their own respective address are programmed through the
LTDC_LxCLUTWR register:
• In case of L8 and AL88 input pixel format, the CLUT must be loaded by 256 colors. The
address of each color is configured in the CLUTADD bits in the LTDC_LxCLUTWR
register.

RM0456 Rev 6

<!-- pagebreak -->

1741

LCD-TFT display controller (LTDC)

RM0456

• In case of AL44 input pixel format, the CLUT must be loaded by only16 colors. The
address of each color must be filled by replicating the 4-bit L channel to 8-bit as follows:
– L0 (indexed color 0), at address 0x00
– L1, at address 0x11
– L2, at address 0x22
– .....
– L15, at address 0xFF

Color frame buffer address
Every layer has a start address for the color frame buffer configured through the
LTDC_LxCFBAR register.
When a layer is enabled, the data is fetched from the color frame buffer.

Color frame buffer length
Every layer has a total line length setting for the color frame buffer in bytes and a number of
lines in the frame buffer configurable in the LTDC_LxCFBLR and LTDC_LxCFBLNR register
respectively.
The line length and the number of lines settings are used to stop the prefetching of data to
the layer FIFO at the end of the frame buffer:
•

If it is set to less bytes than required, a FIFO underrun interrupt is generated if it has
been previously enabled.

•

If it is set to more bytes than actually required, the useless data read from the FIFO is
discarded. The useless data is not displayed.

Color frame buffer pitch
Every layer has a configurable pitch for the color frame buffer, that is the distance between
the start of one line and the beginning of the next line in bytes. It is configured through the
LTDC_LxCFBLR register.

Layer blending
The blending is always active and the two layers can be blended following the blending
factors configured through the LTDC_LxBFCR register.
The blending order is fixed and it is bottom up. If two layers are enabled, first the Layer1 is
blended with the Background color, then the layer2 is blended with the result of blended
color of layer1 and the background. Refer to the figure below.
Figure 410. Blending two layers with background

Layer2
Layer1

Layer2

BG

Layer1 + BG

Layer2 +
Layer1 + BG
MSv48123V1

<!-- pagebreak -->

