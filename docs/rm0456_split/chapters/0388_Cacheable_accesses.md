400

Data cache (DCACHE)

RM0456
Table 92. DCACHE cacheability for AHB transaction

AHB lookup attribute

AHB bufferable attribute

Cacheability

0

x

Read and write: non cacheable

1

0

Read: cacheable
Write: (cacheable) write-through

1

1

Read: cacheable
Write: (cacheable) write-back

In case of noncacheable access, the DCACHE is bypassed, meaning that the AHB
transaction is propagated unchanged to the master output port.
The bypass does not increase the latency of the access to the targeted memory.
In case of cacheable access, the DCACHE behaves as explained in Section 9.4.6.
Cacheable memory regions are defined and programmed by the user in the MPU,
responsible for the generation of the AHB attribute signals for any transaction addressing a
given region.

9.4.6

Cacheable accesses
When the DCACHE receives a cacheable transaction from Cortex®-M33 or from another
bus master peripheral on its slave port, the DCACHE checks if the address requested is
present in its TAG memory and if the corresponding cache line is valid.
For read transaction, there are three alternatives:
•

The address is present inside the TAG memory, the cache line is valid: cache read hit,
the data is read from cache and provided to the processor in the same cycle.

•

The address is not present in the TAG memory: cache read miss, the data is read
from the main memory and provided to the processor, and a cache line refill is
performed.
The critical-word-first refill policy insures minimum wait cycles for the processor, since
read data can be provided while cache is still performing cache line refill (associated
latency is the latency of fetching one word from main memory).
The kind of burst generated on the DCACHE master bus depends on HBURST bit
in DCACHE_CR: either INCRw or WRAPw (w being the cache line width, in words).
The AHB transaction attributes are also propagated from the slave input (missing)
request to the master output refill request.

•

The address is not present in the TAG memory but belongs to the refill burst from main
memory that is currently ongoing: cache read hit as well (hit-under-miss feature).
Whatever the line refill is due to a read or write (missing) transaction, the DCACHE can
provide the requested read data as soon as the data is available at its master interface,
thus avoiding a miss (with data fetch from main memory).

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Data cache (DCACHE)
For write-back transaction (write transaction, with write-back bufferable attribute), there are
three alternatives as well:
•

The address is present inside the TAG memory, cache line is valid: cache write-back
hit, the data is written in cache.

•

The address is not present in the TAG memory (or the cache line is not valid): cache
write-back miss.
First, a line allocation is performed by reading the entire cache line data from main
memory. The kind of burst generated on the DCACHE master bus for this line refill
depends on HBURST bit in DCACHE_CR: either INCRw or WRAPw (w being the
cache line width, in words), and the AHB transaction attributes are propagated from the
slave port initial request.
Once the refilled line is written in the DCACHE, the initial data provided on the slave
port is written in this DCACHE line (it overwrites the data part of the cache line that was
refilled just before).

•

The address is not present in the TAG memory but belongs to the refill burst from main
memory that is currently ongoing: cache write-back hit as well (hit-under-miss
feature).
Whatever the line refill is due to a read or write (missing) transaction, the DCACHE can
write incoming data directly inside the refilled line, thus avoiding a miss (with refill from
main memory).

For write-through transaction (write transaction, with write-through bufferable attribute),
only two alternatives exist:
•

The address is present inside the TAG memory, cache line is valid: cache writethrough hit, the data is written both in cache and in main memory (through master
port).

•

The address is not present in the TAG memory (or the cache line is not valid): cache
write-through miss, the data incoming at slave port is written only in main memory
(unlike the write-back miss, there is no line allocation and data written in cache).

In case of cache refill (due to cache miss), the DCACHE selects which cache line is written
with the refill data: as a 2-way set associative cache, one line among two can be used
(line pointed by the address index, in each of the two ways). The way selection is based
on a pLRU-t replacement algorithm that points, for each index, on the way candidate for the
next refill.
If the cache line where the refill data must be written is already valid, the targeted cache line
must be evicted first:
•

If the dirty tag of this line equals 0 (clean data), the line is simply invalidated.

•

If it equals 1 (dirty data), the line must be written back in the main memory.
The DCACHE generates a burst write transaction on its master port, with burst type set
to INCRw (w being the cache line width, in words), and with AHB memory transaction
attribute signals set as below:
–

data (not instruction)

–

privileged = TAG privilege bit

–

write-back (even if it does not care)

–

normal memory

–

cacheable

–

allocate (even if it does not care)

RM0456 Rev 6

<!-- pagebreak -->

