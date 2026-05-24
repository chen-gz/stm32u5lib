•

Single AHB master bus architecture

•

AHB slave programming interface supporting 8/16/32-bit accesses (except for CLUT
accesses which are 32-bit)

•

User programmable working area size

•

User programmable offset for sources and destination areas expressed in pixels or
bytes

•

User programmable sources and destination addresses on the whole memory space

•

Up to 2 sources with blending operation

•

Alpha value can be modified (source value, fixed value or modulated value)

•

User programmable source and destination color format

•

Up to 11 color formats supported from 4-bit up to 32-bit per pixel with indirect or direct
color coding

•

2 internal memories for CLUT storage in indirect color mode

•

Automatic CLUT loading or CLUT programming via the CPU

•

User programmable CLUT size

•

Internal timer to control AHB bandwidth

•

6 operating modes: register-to-memory, memory-to-memory, memory-to-memory with
pixel format conversion, memory-to-memory with pixel format conversion and blending,
memory-to memory with pixel format conversion, blending and fixed color foreground,
and memory-to memory with pixel format conversion, blending and fixed color
background.

•

Area filling with a fixed color

RM0456 Rev 6

RM0456

Chrom-ART Accelerator controller (DMA2D)
•

Copy from an area to another

•

Copy with pixel format conversion between source and destination images

•

Copy from two sources with independent color format and blending

•

Output buffer byte swapping to support refresh of displays through parallel interface

•

Abort and suspend of DMA2D operations

•

Watermark interrupt on a user programmable destination line

•

Interrupt generation on bus error or access conflict

•

Interrupt generation on process completion

19.3

DMA2D functional description

19.3.1

DMA2D block diagram
The DMA2D controller performs direct memory transfer. As an AHB master, it can take the
control of the AHB bus matrix to initiate AHB transactions.
The DMA2D can operate in the following modes:
•

Register-to-memory

•

Memory-to-memory

•

Memory-to-memory with pixel format conversion

•

Memory-to-memory with pixel format conversion and blending

•

Memory-to memory with pixel format conversion, blending and fixed color foreground

•

Memory-to memory with pixel format conversion, blending and fixed color background

The AHB slave port is used to program the DMA2D controller.

RM0456 Rev 6

<!-- pagebreak -->

