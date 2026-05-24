910

Chrom-GRC (GFXMMU)

RM0456

Example of calculation
Consider the following configuration for virtual buffer 0:
•

First visible block of line 0: block 7

•

Number of visible block in line 0: 10

•

First visible block of line 1: block 6

•

Number of visible block in line 1: 12

•

Address of the physical buffer: 0xC020:0000

The configuration must be:
•

The base address of the physical buffer 0: 0xC000:0000

•

The offset of buffer 0: 0x20:0000

•

First visible block of line 0: block 7

•

Last visible block of line 0: block 16

•

Block 0 offset of line 0: (0 - 7) x 0x10 = -0x70 = 0x3F:FF90

•

First visible block of line 1: block 6

•

Last visible block of line 1: block 17

•

Block 0 offset of line 1: (10 - 6) x 0x10 = (0xA - 0x6) x 0x10 = 0x40

As a consequence:
•

the physical address of block 7 of line 0 is:

0xC000:0000 + 0x20:0000 + (0x3F:FF90 + 0x70 without carry) = 0xC020:0000
•

the physical address of block 16 of line 0 is:

0xC000:0000 + 0x20:0000 + (0x3F:FF90 + 0x100 without carry) = 0xC020:0090
•

the physical address of block 6 of line 1 is:

0xC000:0000 + 0x20:0000 + (0x40 + 0x60 without carry) = 0xC020:00A0
•

the physical address of block 17 of line 1 is:

0xC000:0000 + 0x20:0000 + (0x40 + 0x110 without carry) = 0xC020:0150

21.4.3

Cache and prefetch mechanism
The GFXMMU integrated cache targets internal or external RAM devices storing the
graphical frame buffer.

Master accessing the GFXMMU
Several masters are supposed to access memories through the GFXMMU:

<!-- pagebreak -->

•

When the CPU is accessing the framebuffer, it is because it is performing a
read/modify/write of a single pixel in the frame buffer. As a consequence the granularity
of the R/M/W is less or equal to a word and a data cache is necessary. As the frame
buffer is scanned linearly, it is realistic to says that the next data that is requested by
the CPU within a buffer, is the next pixel. A prefetch mechanism can anticipate this
efficiently.

•

When Chrom ART or the LTDC are accessing the framebuffer, they are generating long
accesses up to 128 bytes. A cache may not be necessary in this case as successive
operation is done only once.

RM0456 Rev 6

RM0456

Chrom-GRC (GFXMMU)
As a consequence the cache is aimed for CPU and must not be used together with DMA2D
or LTDC. It is recommended to use the cache only with the CPU and manage coherency by
software, flushing the cache when the CPU operations are finished.
On STM32U599/5A9, it is also recommended to use the cache with the GPU when the
framebuffer is configured in RGB888 mode.

Cache enabling
The cache is enabled setting the CE (cache enable) bit of the GFXMMU control register
(GFXMMU_CR).

Cache lock mechanism
It is possible to lock the cache by setting the cache lock bit (CL) and configuring the cache
lock buffer (CLB) field of the GFXMMU configuration register (GFXMMU_CR).
When the cache is locked on a given buffer, all the other buffer can not be cached.
An operation on the locked buffered can be cachable according to attribute of the master
request, or can be always cachable setting the force caching (FC) bit of the GFXMMU
configuration register (GFXMMU_CR). The force caching mechanism is only available when
the cache is locked to a buffer. The force caching (FC) bit of the GFXMMU configuration
register (GFXMMU_CR) is automatically reset when the cache lock (CL) bit of the GFXMMU
configuration register (GFXMMU_CR) is reset.

Cache line size
A cache line size is 16 bytes.

Number of cache lines
The number of line is reduced as much as possible taking into account that CPU is
performing linear accesses.
As a consequence three lines are needed:
•

one line for the current access

•

one line for the previous access (as pixels can be split into two 16-byte blocks)

•

one line for the prefetched access

Prefetch mechanism
Because the CPU performs most of the time, linear accesses to the frame buffer, a prefetch
mechanism is provided in order to automatically retrieve the next cache line from the
memory.
The prefetch mechanism can be disabled setting the prefetch disable (PD) bit of the
GFXMMU configuration register (GFXMMU_CR).
When the prefetch mechanism is disabled, only the TAG of the line dedicated for prefetching
is updated but the data are retrieved from the memory.

RM0456 Rev 6

<!-- pagebreak -->

