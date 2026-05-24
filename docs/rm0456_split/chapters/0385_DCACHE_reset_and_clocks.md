RM0456 Rev 6

RM0456

9.4.2

Data cache (DCACHE)

DCACHE reset and clocks
The DCACHE is clocked on the Cortex®-M33 S-AHB bus clock.
When the DCACHE reset signal is released, a cache invalidate procedure is automatically
launched, making the DCACHE busy (DCACHE_SR = 0x0000 0001).
When this procedure is finished:
•

The DCACHE is invalidated: cold cache, with all cache line valid, dirty, and privilege
bits = 0 (DCACHE must be filled up)

•

DCACHE_SR = 0x0000 0002 (reflecting the cache is no longer busy)

•

The DCACHE is disabled: the EN bit in DCACHE_CR holds its reset state (= 0).

Note:

When disabled, the DCACHE is bypassed: slave input requests are just forwarded to the
master port.

9.4.3

DCACHE TAG memory
The DCACHE TAG memory contains:
•

address tags that indicate which data are contained in the cache data memories

•

validity bits

•

dirty bits

•

privilege bits

There is one valid bit, one dirty bit, and one privilege bit per cache line (per way).
The valid bit enables/disables access to the data cache line: if the line is not valid, the data
access (read or write) is performed in the main memory.
The valid bit is set when the cache line is written (refilled by either a read miss or
a write-back miss).
Valid bits are reset in any of the below cases:
•

after the DCACHE reset is released

•

when the cache is disabled, by setting EN = 0 in DCACHE_CR (by software)

•

when executing one of the DCACHE invalidate commands, setting by software
CACHEINV = 0, or CACHECMD = 0b010 or 0b011 in DCACHE_CR
(see Section 9.4.8).

The dirty bit indicates that the cache line has up-to-date values with respect to the main
memory content (the cache has last right value, the main memory is not up to date).
The dirty bit is set when the cache line is written by a slave port write transaction
(only in case of an access with write-back attribute).
Dirty bits are reset in any of the below cases:
•

after the DCACHE reset is released

•

when a line refill is performed on a read miss (on a write-back miss, the refilled cache
line is modified by the written data, and dirty bit = 1)

•

when the cache invalidation is performed

•

when executing one of the DCACHE clean operations (cache line written back to the
main memory), setting by software CACHECMD = 0b001 or 0b011 in DCACHE_CR
(see Section 9.4.8).

RM0456 Rev 6

<!-- pagebreak -->

