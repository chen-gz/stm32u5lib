891

Chrom-ART Accelerator controller (DMA2D)

RM0456

Table 173. Data order in memory (continued)
AL88

A1[7:0]

L1[7:0]

A0[7:0]

L0[7:0]

L4

L7[3:0]L6[3:0]

L5[3:0]L4[3:0]

L3[3:0]L2[3:0]

L1[3:0]L0[3:0]

A8

A3[7:0]

A2[7:0]

A1[7:0]

A0[7:0]

A4

A7[3:0]A6[3:0]

A5[3:0]A4[3:0]

A3[3:0]A2[3:0]

A1[3:0]A0[3:0]

The 24-bit RGB888 aligned on 32-bit is supported through the ARGB8888 mode.
Once the 32-bit value is generated, the alpha channel can be modified according to AM[1:0]
in DMA2D_FGPFCCR or DMA2D_BGPFCCR, as shown in Table 174.
One of the following happens for the alpha channel:
•

It is kept as it is (no modification).

•

It is replaced by ALPHA[7:0] value i nDMA2D_FGPFCCR/DMA2D_BGPFCCR.

•

It is replaced by the original alpha value multiplied by ALPHA[7:0] in
DMA2D_FGPFCCR/DMA2D_BGPFCCR divided by 255.
Table 174. Alpha mode configuration
AM[1:0]

Note:

Alpha mode

00

No modification

01

Replaced by value in DMA2D_xxPFCCR

10

Replaced by original value multiplied by the value in DMA2D_xxPFCCR / 255

11

Reserved

To support the alternate format, the incoming alpha value can be inverted setting AI in
DMA2D_FGPFCCR or DMA2D_BGPFCCR. This applies also to the Alpha value stored in
the DMA2D_FGPFCCR or DMA2D_BGPFCCR, and in the CLUT.
The R and B fields can also be swapped setting RBS in DMA2D_FGPFCCR or
DMA2D_BGPFCCR. This applies also to the RGB order used in the CLUT, and in
DMA2D_FGCOLR or DMA2D_BGCOLR.

20.3.6

DMA2D foreground and background CLUT interface
The CLUT interface manages the CLUT memory access and the automatic loading of
the CLUT.
Three kinds of accesses are possible:
•

CLUT read by the PFC during pixel format conversion operation

•

CLUT accessed through the AHB slave port when the CPU is reading or writing data
into the CLUT

•

CLUT written through the AHB master port when an automatic loading of the CLUT
is performed

The CLUT memory loading can be done in two different ways:
•

Automatic loading
The following sequence must be followed to load the CLUT:

<!-- pagebreak -->

