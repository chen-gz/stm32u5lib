RM0456 Rev 6

RM0456

8.4.2

Instruction cache (ICACHE)

ICACHE reset and clocks
The ICACHE is clocked on the Cortex®-M33 C-AHB bus clock.
When the ICACHE reset signal is released, a cache invalidate procedure is automatically
launched, making the ICACHE busy (ICACHE_SR = 0x0000 0001).
When this procedure is finished:
•

the ICACHE is invalidated: “cold cache”, with all cache line valid bits = 0 (ICACHE must
be filled up)

•

ICACHE_SR = 0x0000 0002 (reflecting the cache is no longer busy)

•

the ICACHE is disabled: the EN bit in ICACHE_CR holds its reset state (=0).

Note:

When disabled, the ICACHE is bypassed, except the remapping mechanism that is still
functional: slave input requests (remapped or not) are forwarded to the master port(s).

8.4.3

ICACHE TAG memory
The ICACHE TAG memory contains:
•

address tags that indicate which data are contained in the cache data memories

•

validity bits

There is one valid bit per cache line (per way).
The valid bit is set when a cache line is refilled (after a miss).
Valid bits are reset in any of the below cases:
•

after the ICACHE reset is released

•

when the cache is disabled, by setting EN = 0 in ICACHE_CR (by software)

•

when executing an ICACHE invalidate command, by setting CACHEINV = 1
in ICACHE_CR (by software)

When a cacheable transaction is received at the execution input port, its AHB address
(HADDR_in) is split into the following fields (see Table 82 for B and W definitions):
•

HADDR_in[B-1:0]: address byte offset, indicates which byte to select inside
a cache line.

•

HADDR_in[B+W-1:B]: address way index, indicates which cache line to select
inside each way.

•

HADDR_in[31:B+W]: tag address, to be compared to the TAG memory address
to check if the requested data is already available (meaning valid) inside the ICACHE.

The following table gives a summary of the ICACHE main parameters for TAG memory
dimensioning. Figure 28 shows the functional view of TAG and data memories, for an n-way
set associative ICACHE.
Table 82. TAG memory dimensioning parameters
for n-way set associative operating mode (default)
Parameter
Cache size
Cache number of ways
Cache line size

Value

Example

S Kbytes = s bytes (s = 1024 x S)

8 Kbytes = 8192 bytes

n

2

L-byte = l-bit (l = 8 x L)

16-byte = 128-bit

RM0456 Rev 6

<!-- pagebreak -->

