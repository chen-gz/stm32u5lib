•

DMA2D output memory address register (DMA2D_OMAR)

•

DMA2D output offset register (DMA2D_OOR)

•

DMA2D number of lines register (number of lines and pixel per lines) (DMA2D_NLR)

RM0456 Rev 6

RM0456

Chrom-ART Accelerator controller (DMA2D)
If the DMA2D operates in register-to-memory mode, the configured output rectangle is filled
by the color specified in DMA2D_OCOLR which contains a fixed 32-, 24-, or 16-bit value.
The format is selected by CM[2:0] in DMA2D_OPFCCR.
The data are stored into the memory in the order defined in the table below.
Table 178. Data order in memory

Color mode

@+3

@+2

@+1

@+0

ARGB8888

A0[7:0]

R0[7:0]

G0[7:0]

B0[7:0]

B1[7:0]

R0[7:0]

G0[7:0]

B0[7:0]

G2[7:0]

B2[7:0]

R1[7:0]

G1[7:0]

R3[7:0]

G3[7:0]

B3[7:0]

R2[7:0]

RGB565

R1[4:0]G1[5:3]

G1[2:0]B1[4:0]

R0[4:0]G0[5:3]

G0[2:0]B0[4:0]

ARGB1555

A1[0]R1[4:0]G1[4:3]

G1[2:0]B1[4:0]

A0[0]R0[4:0]G0[4:3]

G0[2:0]B0[4:0]

ARGB4444

A1[3:0]R1[3:0]

G1[3:0]B1[3:0]

A0[3:0]R0[3:0]

G0[3:0]B0[3:0]

RGB888

The RGB888 aligned on 32 bits is supported through the ARGB8888 mode.

20.3.10

DMA2D output FIFO byte reordering
The output FIFO bytes can be reordered to support display frame buffer update through a
parallel interface (F(S)MC) directly from the DMA2D.
The reordering of bytes can be done using:
•

RBS bit to swap red and blue component

•

SB bit to swap byte two by two in the output FIFO

When the byte swapping is activated (SB = 1 in DMA2D_OPFCR), the number of pixel per
line (PL field in DMA2D_NLR) must be even, and the output memory address (MA field
in DMA2D_OMAR) must be even. The output line offset computed in bytes (resulting from
LOM field in DMA2D_CR and LO field in DMA2D_OOR values) must also be even. If not a
configuration error is detected.
Table 179. Standard data order in memory
Color Mode

RGB888

RGB565

@+3

@+2

@+1

@+0

B1[7:0]

R0[7:0]

G0[7:0]

B0[7:0]

G2[7:0]

B2[7:0]

R1[7:0]

G1[7:0]

R3[7:0]

G3[7:0]

B3[7:0]

R2[7:0]

R1[4:0]G1[5:3]

G1[2:0]B1[4:0]

R0[4:0]G0[5:3]

G0[2:0]B0[4:0]

16-bit mode (RGB565)
This mode is supported without byte reordering by the DMA2D.

RM0456 Rev 6

<!-- pagebreak -->

