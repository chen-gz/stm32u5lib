856

Chrom-ART Accelerator controller (DMA2D)

RM0456

The CLUT memory loading can be done in two different ways:
•

Automatic loading
The following sequence must be followed to load the CLUT:

•

a)

Program the CLUT address in DMA2D_FGCMAR (foreground CLUT) or
DMA2D_BGCMAR (background CLUT).

b)

Program the CLUT size with CS[7:0] in DMA2D_FGPFCCR (foreground CLUT) or
DMA2D_BGPFCCR (background CLUT).

c)

Set the START bit in DMA2D_FGPFCCR (foreground CLUT) or
DMA2D_BGPFCCR (background CLUT) to start the transfer. During this
automatic loading process, the CLUT is not accessible by the CPU. If a conflict
occurs, a CLUT access error interrupt is raised assuming CAEIE = 1
in DMA2D_CR.

Manual loading
The application has to program the CLUT manually through the DMA2D AHB slave
port to which the local CLUT memory is mapped.The foreground CLUT (FGCLUT) is
located at address offset 0x0400 and the background CLUT (BGCLUT) at address
offset 0x0800.

The CLUT format is 24 or 32 bits. It is configured through the CCM bit in
DMA2D_FGPFCCR (foreground CLUT) or DMA2D_BGPFCCR (background CLUT) as
shown in the table below.
Table 162. Supported CLUT color mode
CCM

CLUT color mode

0

32-bit ARGB8888

1

24-bit RGB888

The way the CLUT data are organized in the system memory is specified in the table below.
Table 163. CLUT data order in system memory
CLUT color mode

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

RGB888

<!-- pagebreak -->

