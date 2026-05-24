RM0456 Rev 6

RM0456

Chrom-ART Accelerator controller (DMA2D)
Figure 91. Intel 8080 16-bit mode (RGB565)

16-bit Data
Data Bus

Colors

R4

R3

R2

R1

R0

D0

D4

D3

D2

D1

G5 B4
G5 G4 G3 G2 G1 G0

B3

B2

B1 B0

D15 D14 D13 D12 D11 D10 D9

D8 D7

D6

D5

64K colors

MSv42078V2

18/24-bit mode (RGB888)
This mode needs data reordering.
1.

Red and the Blue have to be swapped (setting the RBS bit).

2.

MSB and the LSB bytes of a half-word have to be swapped (setting the SB bit).
Figure 92. Intel 8080 18/24-bit mode (RGB888)

Transfer
Order

1

2

16-bit Data

16-bit Data

Data Bus

D15 D14 D13 D12 D11 D10 D9 D8 D7 D6 D5 D4 D3 D2 D1 D0 D15 D14 D13 D12 D11 D10 D9 D8 D7 D6 D5 D4 D3 D2 D1 D0

Colors

R7 R6 R5 R4 R3 R2 R1 R0 G7 G6 G5 G4 G3 G2 G1 G0 B7 B6 B5 B4 B3 B2 B1 B0 R7 R6 R5 R4 R3 R2 R1 R0

16.7M colors
1st pixel

2nd pixel
MSv42079V2

Table 167. Output FIFO byte reordering steps
Steps

Original data ordering

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

Setting the RBS bit
Data ordering after red and blue
swap (RBS set)

R1[7:0]

B0[7:0]

G0[7:0]

R0[7:0]

G2[7:0]

R2[7:0]

B1[7:0]

G1[7:0]

B3[7:0]

G3[7:0]

R3[7:0]

B2[7:0]

Setting the SB bit

RM0456 Rev 6

<!-- pagebreak -->

856

Chrom-ART Accelerator controller (DMA2D)

RM0456

Table 167. Output FIFO byte reordering steps (continued)
Steps
Data ordering after byte swapping
(SB set)

19.3.11

@+3

@+2

@+1

@+0

B0[7:0]

R1[7:0]

R0[7:0]

G0[7:0]

R2[7:0]

G2[7:0]

G1[7:0]

B1[7:0]

G3[7:0]

B3[7:0]

B2[7:0]

R3[7:0]

DMA2D AHB master port timer
An 8-bit timer is embedded into the AHB master port to provide an optional limitation of the
bandwidth on the crossbar.
This timer is clocked by the AHB clock and counts a dead time between two consecutive
accesses. This limits the bandwidth availability.
The timer enabling and the dead time value are configured through the AHB master port
timer configuration register (DMA2D_AMPTCR).

19.3.12

DMA2D transactions
DMA2D transactions consist of a sequence of a given number of data transfers.
The number of data and the width can be programmed by software.
Each DMA2D data transfer is composed of up to four steps:

19.3.13

1.

Data loading from the memory location pointed by DMA2D_FGMAR and pixel format
conversion as defined in DMA2D_FGCR

2.

Data loading from a memory location pointed by DMA2D_BGMAR and pixel format
conversion as defined in DMA2D_BGCR

3.

Blending of all retrieved pixels according to the alpha channels resulting of the PFC
operation on alpha values

4.

Pixel format conversion of the resulting pixels according to DMA2D_OCR and
programming of the data to the memory location addressed through DMA2D_OMAR

DMA2D configuration
Both source and destination data transfers can target peripherals and memories in the
whole 4 Gbyte memory area, at addresses ranging between 0x0000 0000 and
0xFFFF FFFF.
The DMA2D can operate in any of the following modes selected through MODE[2:0] in
DMA2D_CR:

<!-- pagebreak -->

•

Register-to-memory

•

Memory-to-memory

•

Memory-to-memory with PFC

•

Memory-to-memory with PFC and blending

•

Memory-to-memory with PFC, blending and fixed FG color

•

Memory-to-memory with PFC, blending and fixed BG color

RM0456 Rev 6

RM0456

Chrom-ART Accelerator controller (DMA2D)

Register-to-memory
The register-to-memory mode is used to fill a user defined area with a predefined color.
The color format is set in DMA2D_OPFCCR.
The DMA2D does not perform any data fetching from any source. It just writes the color
defined in DMA2D_OCOLR to the area located at the address pointed by DMA2D_OMAR,
and defined in DMA2D_NLR and DMA2D_OOR.

Memory-to-memory
In memory-to-memory mode, the DMA2D does not perform any graphical data
transformation. The foreground input FIFO acts as a buffer and the data are transferred
from the source memory location defined in DMA2D_FGMAR to the destination memory
location pointed by DMA2D_OMAR.
The color mode programmed by CM[3:0] in DMA2D_FGPFCCR defines the number of bits
per pixel for both input and output.
The size of the area to be transferred is defined by DMA2D_NLR and DMA2D_FGOR for
the source, and by DMA2D_NLR and DMA2D_OOR for the destination.

Memory-to-memory with PFC
In this mode, the DMA2D performs a pixel format conversion of the source data and stores
them in the destination memory location.
The size of the areas to be transferred are defined by DMA2D_NLR and DMA2D_FGOR for
the source, and by DMA2D_NLR and DMA2D_OOR for the destination.
Data are fetched from the location defined in DMA2D_FGMAR and processed by the
foreground PFC. The original pixel format is configured through DMA2D_FGPFCCR.
If the original pixel format is direct color mode, then the color channels are all expanded
to 8 bits.
If the pixel format is indirect color mode, the associated CLUT has to be loaded into the
CLUT memory.
The CLUT loading can be done automatically by following the sequence below:
1.

Set the CLUT address into DMA2D_FGCMAR.

2.

Set the CLUT size with CS[7:0] bits in DMA2D_FGPFCCR.

3.

Set the CLUT format (24 or 32 bits) with CCM in DMA2D_FGPFCCR.

4.

Start the CLUT loading by setting START i nDMA2D_FGPFCCR.

Once the CLUT loading is complete, the CTCIF flag in DMA2D_IFR is raised, and an
interrupt is generated if CTCIE = 1 in DMA2D_CR. The automatic CLUT loading process
can not work in parallel with classical DMA2D transfers.
The CLUT can also be filled by the CPU or by any other master through the APB port. The
access to the CLUT is not possible when a DMA2D transfer is ongoing and uses
the CLUT (indirect color format).
In parallel to the color conversion process, the alpha value is added or changed depending
on the value programmed in DMA2D_FGPFCCR. If the original image does not have an

RM0456 Rev 6

<!-- pagebreak -->

856

Chrom-ART Accelerator controller (DMA2D)

RM0456

alpha channel, a default alpha value of 0xFF is automatically added to obtain a fully opaque
pixel. The alpha value is modified according to AM[1:0] in DMA2D_FGPFCCR:
•

It can be unchanged.

•

It can be replaced by the value defined by ALPHA[7:0] in DMA2D_FGPFCCR.

•

It can be replaced by the original value multiplied by ALPHA[7:0] divided by 255.

The resulting 32-bit data are encoded by the OUT PFC into the format specified by CM[2:0]
in DMA2D_OPFCCR. The output pixel format cannot be the indirect mode since no CLUT
generation process is supported.
The processed data are written into the destination memory location pointed
by DMA2D_OMAR.

Memory-to-memory with PFC and blending
In this mode, two sources are fetched in the foreground and background FIFOs from the
memory locations defined by DMA2D_FGMAR and DMA2D_BGMAR.
The two pixel format converters have to be configured as described
in the memory-to-memory mode. Their configurations can be different as each pixel format
converter is independent and has its own CLUT memory.
Once each pixel has been converted into 32 bits by its respective PFC, all pixels are
blended according to the equation below:
with αMult =

αFG . αBG
255

αOUT = αFG + αBG - αMult

COUT =

CFG.αFG + CBG.αBG - CBG.αMult
αOUT

with C = R or G or B

Division are rounded to the nearest lower integer

The resulting 32-bit pixel value is encoded by the output PFC according to the specified
output format, and the data are written into the destination memory location pointed by
DMA2D_OMAR.

Memory-to-memory with PFC, blending and fixed color FG
In this mode, only one source is fetched in the background FIFO from the memory location
defined by DMA2D_BGMAR.
The value of the foreground color is given by DMA2D_FGCOLR and the alpha value is set
to 0xFF (opaque).
The alpha value can be replaced or modified according to AM[1:0] and ALPHA[7:0]
in DMA2D_FGPFCR.
The two pixel format converters have to be configured as described in
the memory-to-memory mode. Their configurations can be different as each pixel format
converter is independent and has its own CLUT memory

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Chrom-ART Accelerator controller (DMA2D)
Once each pixel has been converted into 32 bits by its respective PFC, all pixels are
blended together, and the resulting 32-bit pixel value is encoded by the output PFC
according to the specified output format. Data are written into the destination memory
location pointed by DMA2D_OMAR.

Memory-to-memory with PFC, blending and fixed color BG
In this mode, only one source is fetched in the foreground FIFO from the memory location
defined by DMA2D_FGMAR.
The value of the background color is given by DMA2D_BGCOLR and the alpha value is set
to 0xFF (opaque).
The alpha value can be replaced or modified according to AM[1:0] and ALPHA[7:0]
in DMA2D_BGPFCR.
The two pixel format converters have to be configured as described in
the memory-to-memory mode. Their configurations can be different as each pixel format
converter is independent and has its own CLUT memory
Once each pixel has been converted into 32 bits by its respective PFC, all pixels are
blended together, and the resulting 32-bit pixel value is encoded by the output PFC
according to the specified output format, and the data are written into the destination
memory location pointed by DMA2D_OMAR.

Configuration error detection
The DMA2D checks that the configuration is correct before any transfer. The configuration
error interrupt flag is set by hardware when a wrong configuration is detected when a new
transfer/automatic loading starts. An interrupt is then generated if CEIE = 1 in DMA2D_CR.
The wrong configurations that can be detected are listed below:
•

Foreground CLUT automatic loading: MA bits in DMA2D_FGCMAR are not aligned
with CCM in DMA2D_FGPFCCR.

•

Background CLUT automatic loading: MA bits in DMA2D_BGCMAR are not aligned
with CCM in DMA2D_BGPFCCR.

•

Memory transfer (except in register-to-memory mode and except in
memory-to-memory mode with blending and fixed color FG): MA bits in
DMA2D_FGMAR are not aligned with CM in DMA2D_FGPFCCR.

•

Memory transfer (except in register-to-memory mode and except in
memory-to-memory mode with blending and fixed color FG): CM bits in
DMA2D_FGPFCCR are invalid

•

Memory transfer (except in register-to-memory mode and except in
memory-to-memory mode with blending and fixed color FG): PL bits in DMA2D_NLR
are odd while CM in DMA2D_FGPFCCR is A4 or L4.

•

Memory transfer (except in register-to-memory mode and except in
memory-to-memory mode with blending and fixed color FG): LO bits in DMA2D_FGOR
are odd while CM in DMA2D_FGPFCCR is A4 or L4, and LOM bit in DMA2D_CR is
pixel mode.

•

Memory transfer (only in blending mode and except in memory-to-memory mode with
blending and fixed color FG): MA bits in DMA2D_BGMAR are not aligned with CM in
DMA2D_BGPFCCR.

•

Memory transfer: (only in blending mode and in blending with fixed color FG mode):
CM bits in DMA2D_BGPFCCR are invalid
RM0456 Rev 6

<!-- pagebreak -->

856

Chrom-ART Accelerator controller (DMA2D)

19.3.14

RM0456

•

Memory transfer (only in blending mode and in blending with fixed color FG mode):
PL bits in DMA2D_NLR odd while CM in DMA2D_BGPFCCR is A4 or L4.

•

Memory transfer (only in blending mode and in blending with fixed color FG mode):
LO bits in DMA2D_BGOR are odd while CM in DMA2D_BGPFCCR is A4 or L4, and
LOM bit in DMA2D_CR is pixel mode.

•

Memory transfer (except in memory-to-memory mode and except
in memory-to-memory mode with blending and fixed color FG): MA bits in
DMA2D_OMAR are not aligned with CM bits in DMA2D_OPFCCR.

•

Memory transfer (except in memory to memory mode): CM bits in DMA2D_OPFCCR
are invalid.

•

Memory transfer with byte swapping: PL bits in DMA2D_NLR are odd or MA bits in
DMA2D_OMAR are odd, or LO in bytes (resulting from LOM in DMA2D_CR and LO
bits in DMA2D_OOR) are odd while SB = 1 in DMA2D_OPFCCR.

•

Memory transfer: NL = 0x0 in DMA2D_NLR

•

Memory transfer: PL = 0x0 i nDMA2D_NLR

•

Memory transfer: MODE bits in DMA2D_CR are invalid.

DMA2D transfer control (start, suspend, abort and completion)
Once the DMA2D is configured, the transfer can be launched by setting START
in DMA2D_CR. Once the transfer is completed, START is automatically reset and TCIF flag
in DMA2D_ISR is raised. An interrupt can be generated if TCIE is set in DMA2D_CR.
The user application can suspend the DMA2D at any time by setting SUSP in DMA2D_CR.
The transaction can then be aborted by setting ABORT in DMA2D_CR, or can be restarted
by resetting SUSP in DMA2D_CR.
The user application can abort at any time an ongoing transaction by setting ABORT
in DMA2D_CR. In this case, the TCIF flag is not raised.
Automatic CLUT transfers can also be aborted or suspended by using ABORT or SUSP
in DMA2D_CR.

19.3.15

Watermark
A watermark can be programmed to generate an interrupt when the last pixel of a given line
has been written to the destination memory area.
The line number is defined by LW[15:0] in DMA2D_LWR.
When the last pixel of this line has been transferred, the TWIF flag in DMA2D_ISR is raised,
and an interrupt is generated if TWIE is set in DMA2D_CR.

19.3.16

Error management
Two kind of errors can be triggered:
•

AHB master port errors signaled by TEIF in DMA2D_ISR

•

conflicts caused by a CLUT access (CPU trying to access the CLUT while a CLUT
loading or a DMA2D transfer is ongoing) signaled by CAEIF in DMA2D_ISR.

Both flags are associated to their own interrupt enable flag in DMA2D_CR to generate an
interrupt if need be (TEIE and CAEIE).

<!-- pagebreak -->

