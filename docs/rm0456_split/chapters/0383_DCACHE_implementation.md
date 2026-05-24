–

Two hit-monitor counters (32-bit): number of read hits, number of write hits

–

Two miss-monitor counters (16-bit): number of read misses, number of write
misses

RM0456 Rev 6

RM0456

Data cache (DCACHE)
•

Error management
–

9.3

Possibility to detect error for master port request initiated by DCACHE itself (a
cache line written back into main memory, because of an eviction or a clean
operation), to flag this error, and optionally to raise an interrupt

•

TrustZone security support

•

Maintenance operations
–

Cache invalidate: full cache invalidation, fast command, non-interruptible

–

Cache invalidate range: invalidates cache lines whose address belongs to defined
range, background task, interruptible

–

Cache clean range: cleans cache lines (if dirty bit = 1, write back line, then clear
dirty bit) whose address belongs to defined range, background task, interruptible

–

Cache clean and invalidate range: cleans and invalidates cache lines (if dirty
bit = 1, write back line, then invalidate it) whose address belongs to defined range,
background task, interruptible

DCACHE implementation
The DCACHE1 is placed on Cortex®-M33 S-AHB bus, and caches only the external RAM
memory region (OCTOSPI, HSPI, and FMC), in the address range [0x6000 0000:0xAFFF
FFFF] of the memory map.
Indeed, by placing a bus matrix demultiplexing node in front of the DCACHE1, S-AHB bus
memory requests addressing SRAM region or peripherals region (respectively in ranges
[0x2000 0000:0x3FFF FFFF] and [0x4000 0000:0x5FFF FFFF]) are routed directly to the
main AHB bus matrix, and the DCACHE1 is bypassed.
In STM32U59x/5Ax/5Fx/5Gx, the DCACHE2 is placed on the AHB bus driven by
the port M0 of GPU2D, and caches all the memory regions accessed by it.
All GPU2D transactions are cacheable, except transactions to internal SRAMs that can be
made cacheable or non-cacheable, depending on the related configuration programmed in
the system configuration controller.
As the GPU2D traffic is non-secure, the DCACHE2 does not support TrustZone.
Table 89. DCACHE features for STM32U535/545/575/585
Features

DCACHE1

Number of ways

2

Cache size

4 Kbytes

Cache line width

16 bytes

Data size of AHB Master interface

32 bits

Table 90. DCACHE features for STM32U59x/5Ax/5Fx/5Gx
Features
Number of ways

DCACHE1/2
4

Cache size

16 Kbytes

RM0456 Rev 6

<!-- pagebreak -->

