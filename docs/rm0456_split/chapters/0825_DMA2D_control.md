RM0456 Rev 6

32

RM0456

19.3.3

Chrom-ART Accelerator controller (DMA2D)

DMA2D control
The DMA2D controller is configured through the DMA2D control register (DMA2D_CR).
The user application can perform the following operations:

19.3.4

•

Select the operating mode.

•

Enable/disable the DMA2D interrupt.

•

Start/suspend/abort ongoing data transfers.

DMA2D foreground and background FIFOs
The DMA2D foreground (FG) FG FIFO and background (BG) FIFO fetch the input data to
be copied and/or processed.
These FIFOs fetch the pixels according to the color format defined in their respective pixel
format converter (PFC). They are programmed through a set of control registers:
•

DMA2D foreground memory address register (DMA2D_FGMAR)

•

DMA2D foreground offset register (DMA2D_FGOR)

•

DMA2D background memory address register (DMA2D_BGMAR)

•

DMA2D background offset register (DMA2D_BGBOR)

•

DMA2D number of lines register (number of lines and pixel per lines) (DMA2D_NLR)

When the DMA2D operates in register-to-memory mode, none of the FIFOs is activated.
When the DMA2D operates in memory-to-memory mode (no pixel format conversion nor
blending operation), only the FG FIFO is activated and acts as a buffer.
When the DMA2D operates in memory-to-memory operation with pixel format conversion
(no blending operation), the BG FIFO is not activated.

19.3.5

DMA2D foreground and background pixel format converter (PFC)
DMA2D foreground pixel format converter (PFC) and background pixel format converter
perform the pixel format conversion to generate a 32-bit per pixel value. The PFC can also
modify the alpha channel.
The first stage of the converter converts the color format. The original color format of the
foreground pixel and background pixels are configured through the CM[3:0] bits of the
DMA2D_FGPFCCR and DMA2D_BGPFCCR, respectively.
The supported input formats are given in the table below.
Table 159. Supported color mode in input
CM[3:0]

Color mode

0000

ARGB8888

0001

RGB888

0010

RGB565

0011

ARGB1555

0100

ARGB4444

0101

L8

RM0456 Rev 6

<!-- pagebreak -->

