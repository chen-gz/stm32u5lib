•

The physical buffer base address (PBBA) field of the GFXMMU buffer x configuration
register (GFXMMU_BxCR). It configures the physical location of the 8-Mbyte area
where the buffer is mapped.

•

The physical buffer location respective to the physical buffer base address is defined by
the physical buffer offset (PBO) field of the GFXMMU buffer x configuration register
(GFXMMU_BxCR).

RM0456 Rev 6

RM0456

Chrom-GRC (GFXMMU)
Figure 98. Virtual buffer and physical buffer memory map
Physical buffer

0xFF:FFFF

Virtual buffer
pBuffer3

0xDF:FFFF
Virtual buffer 3
(3/4 Mbyte)

0xYY:Y000

Physical buffer 3 offset

0xXX00:0000

Physical buffer 3 base address

0xYY:Y000

Physical buffer 2 offset

0xXX00:0000

Physical buffer 2 base address

0xYY:Y000

Physical buffer 1 offset

0xXX00:0000

Physical buffer 1 base address

0xYY:Y000

Physical buffer 0 offset

0xXX00:0000

Physical buffer 0 base address

0xC0:0000
0xAF:FFFF

pBuffer2

Virtual buffer 2
(3/4 Mbyte)

0x80:0000
0x6F:FFFF
Virtual buffer 1
(3/4 Mbyte)

pBuffer1

0x40:0000
0x2F:FFFF
Virtual buffer 0
(3/4 Mbyte)
pBuffer0
0x00:0000

MSv43801V2

The buffer can not overflow the 8-Mbyte boundary of the zone defined by its base address.
In case of overflow, the buffer x overflow flag (BxOF) of the GFXMMU status register
(GFXMMU_SR) is set and an interrupt is generated if the buffer x overflow interrupt enable
(BxOIE) bit of the GFXMMU configuration register (GFXMMU_CR) is set.

Virtual buffer application use case
As the physical locations are independently configurable, the four virtual buffers can be
physically mapped to non continuous locations. This would allow for example to have the
four buffers mapped on to four different SDRAM banks and avoid extra precharge cycles
accessing the SDRAM.
As a consequence, one buffer must be used by the CPU/Chrom ART for frame buffer
calculation while an other one must be used by the LTDC.
The two remaining buffers can be used as a graphical library requiring extra drawing
buffers.

21.4.2

MMU architecture
The MMU block is responsible of the address resolution. It receives the 24-bit address and
returns the physical 23-bit address and a valid signals to indicate the address is physically
mapped or not. The MMU also checks overflow of a area boundary.

RM0456 Rev 6

<!-- pagebreak -->

910

Chrom-GRC (GFXMMU)

RM0456

The MMU LUT is implemented as a 1024 x 35-bit RAM
Figure 99. MMU block diagram

Add[21:4]

Line/block
decoder

Block[7:0]
+

[21:4]

Block0Offset[21:4]
Line[9:0]

LookUp
RAM

LineEnable
+
FirstBlock[7:0]

Add[23:4]

1024 x 35-bit

Add[23:22]

Overflow
C

LastBlock[7:0]

pBufferOffset

Block
valid
comp.

PhyAdd[22:4]
Valid

pBufferOffset[22:4]

MSv43802V1

Line block decoder
The line block decoder is generating the block number and the line number according the
address.

Look up RAM
The look up RAM is a 1024 x 35-bit RAM with the following fields:
•

1-bit line enable

•

8-bit first valid block

•

8-bit last valid block

•

18-bit for line offset

As the RAM is bigger than a word, each entry is split into two words on the memory map.
The write access are done in two steps:
1.

Write the first word with enable/first valid block/last valid block in the GFXMMU_LUTxL
memory location (internally buffered).

2.

Write the second word with line offset in the GFXMMU_LUTxH memory location
(effective write into the memory together with the internally buffered value).

A write in the LUT can happen any time but it can lead to inconsistencies if a master is using
the MMU at the same time. As the CPU has the priority during LUT programming, this may
slow down MMU calculation.
There is no restriction during read operations, but this may slow down CPU as the MMU has
the priority on LUT accesses.

Block validation/comparator
This block is checking if the block is valid.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Chrom-GRC (GFXMMU)
A block is considered as valid (physically mapped) when:
•

Line is enable.

•

The block number is greater or equal to the first valid block.

•

The block number is lower or equal to the last valid block.

When the block is valid, the physical address generated is considered as correct.
If the result of the MMU evaluation is not valid, the write operations are ignored, and read
operations return the default 32-bit value stored in the default value (DV) field of the
GFXMMU default value register (GFXMMU_DVR).

Block offset address calculation within the buffer
The block number is added to the line offset to get the offset of the block within the physical
buffer.
As a consequence, the line offset stored in the LUT is given by the following formula:
Line offset = [(Number of visible blocks already used) - (1st visible block)] x block size
with:
•

The maximum value for the line offset is when all the block of all the line are used. As
the consequence the line offset for the last line can be maximum:
1023 x 256 x 16 = 0x3F:F00x

•

The minimum value for the line offset is when the last block of the first line is the first
valid block: -255*16 = - 0xFFx i.e 0x3F:F01x

As the consequence the full range of the line offset entry of the LUT is used.
Carry is not taken into account as this stage to be able to perform negative offset
calculations (values from 0x3F:F01x to 0x3F:FFFx)
As the block offset is within a 4-Mbyte buffer, the address generated is 22-bit wide.

Block offset address calculation
Once the offset of the block within the buffer as been calculated, this value is added to the
offset of the block respective to the physical buffer base address.
The offset of the blocks are defined in registers as shown in Figure 100:
Figure 100. Block validation/comparator implementation
Add[23:22]

PBufferOffset
pBuffer0Offset[22:4]
pBuffer1Offset[22:4]

pBufferOffset[22:4]

pBuffer2Offset[22:4]
pBuffer3Offset[22:4]
MSv43803V1

The resulting address and the buffer offset address must be on 23-bit.
The carry is taken into account to trigger address overflow. The carry is propagated to the
GFXMMU status register (GFXMMU_SR) to set the buffer x overflow flag (BxOF).

RM0456 Rev 6

<!-- pagebreak -->

