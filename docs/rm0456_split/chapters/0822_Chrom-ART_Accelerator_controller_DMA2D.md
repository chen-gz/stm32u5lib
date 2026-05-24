821

Chrom-ART Accelerator controller (DMA2D)

19

RM0456

Chrom-ART Accelerator controller (DMA2D)
This section only applies to STM32U575/585/59x/5Ax devices.

19.1

DMA2D introduction
The Chrom-ART Accelerator (DMA2D) is a specialized DMA dedicated to image
manipulation. It can perform the following operations:
•

fill a part or the whole of a destination image with a specific color

•

copy a part or the whole of a source image into a part or the whole of a destination
image

•

copy a part or the whole of a source image into a part or the whole of a destination
image with a pixel format conversion

•

blend a part and/or two complete source images with different pixel format and copy
the result into a part or the whole of a destination image with a different color format

All the classical color coding schemes are supported from 4-bit up to 32-bit per pixel with
indexed or direct color mode. The DMA2D has its own dedicated memories for CLUTs
(color look-up tables).

19.2

DMA2D main features
The main DMA2D features are:

<!-- pagebreak -->

