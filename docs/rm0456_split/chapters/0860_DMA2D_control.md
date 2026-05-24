891

Chrom-ART Accelerator controller (DMA2D)

20.3.3

RM0456

DMA2D control
The DMA2D controller is configured through DMA2D_CR. The user application can perform
the following operations:

20.3.4

•

Select the operating mode.

•

Enable/disable the DMA2D interrupt.

•

Start/suspend/abort ongoing data transfers.

DMA2D foreground and background FIFOs
The DMA2D foreground (FG) FG FIFO and background (BG) FIFO fetch the input data to
be copied and/or processed.
The FIFOs fetch the pixels according to the color format defined in their respective pixel
format converter (PFC).
They are programmed through the following control registers:
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
blending operation), only the FG FIFO is activated, and acts as a buffer.
When the DMA2D operates in memory-to-memory operation with pixel format conversion
(no blending operation), the BG FIFO is not activated.

20.3.5

DMA2D foreground and background PFC
DMA2D foreground pixel format converter (PFC) and background pixel format converter
perform the pixel format conversion to generate a 32-bit per pixel value. The PFC can also
modify the alpha channel.
The first PFC stage converts the color format. The original color format of the foreground
and background pixels are configured through CM[3:0] in DMA2D_FGPFCCR and
DMA2D_BGPFCCR, respectively.
The supported input formats are given in the table below.
Table 172. Supported color mode in input

<!-- pagebreak -->

