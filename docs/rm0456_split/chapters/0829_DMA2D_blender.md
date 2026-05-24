RM0456 Rev 6

RM0456

19.3.7

Chrom-ART Accelerator controller (DMA2D)

DMA2D blender
The DMA2D blender blends the source pixels by pair to compute the resulting pixel.
The blending is performed according to the following equation:

with αMult =

αFG . αBG
255

αOUT = αFG + αBG - αMult

COUT =

CFG.αFG + CBG.αBG - CBG.αMult

with C = R or G or B

αOUT

Division is rounded to the nearest lower integer

No configuration register is required by the blender. The blender use depends on the
DMA2D operating mode defined by MODE[2:0] in DMA2D_CR.

19.3.8

DMA2D output PFC
The output PFC performs the pixel format conversion from 32 bits to the output format
defined by CM[2:0]in DMA2D_OPFCCR.
The supported output formats are given in the table below.
Table 164. Supported color mode in output

Note:

CM[2:0]

Color mode

000

ARGB8888

001

RGB888

010

RGB565

011

ARGB1555

100

ARGB4444

To support the alternate format, the calculated alpha value is inverted setting AI bit
in DMA2D_OPFCCR. This applies also to the alpha value used in DMA2D_OCOLR.
The R and B fields can also be swapped setting RBS in DMA2D_OPFCCR. This applies
also to the RGB order used in DMA2D_OCOLR.

19.3.9

DMA2D output FIFO
The output FIFO programs the pixels according to the color format defined in the output
PFC.
The destination area is defined through a set of control registers:
•

DMA2D output memory address register (DMA2D_OMAR)

RM0456 Rev 6

<!-- pagebreak -->

