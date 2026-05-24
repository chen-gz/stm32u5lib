RM0456 Rev 6

RM0456

Instruction cache (ICACHE)
The following table gives a summary of ICACHE main parameters for TAG memory when
the direct-mapped cache operating mode is selected.
Table 83. TAG memory dimensioning parameters for direct-mapped cache mode
Parameter

Value

Example

S Kbytes = s bytes (s = 1024 x S)

8 Kbytes = 8192 bytes

1

1

L-byte = l-bit (l = 8 x L)

16-byte = 128-bit

Number of cache lines

LpW = s / L lines

512 lines

Address byte offset size

B = log2(L) bit

4-bit

Address way index size

W = log2(LpW) bit

9-bit

TAG address size

T = (32 - W - B) bit

19-bit

Cache size
Cache number of ways
Cache line size

All cache operations (such as read, refill, remapping, invalidation) remain the same in the
direct-mapped configuration. The only difference is the absence of a replacement algorithm
in case of line eviction (as explained in Section 8.4.8): only one way (the unique one) is
possible for any data refill.

8.4.5

ICACHE enable
To activate the ICACHE, the EN bit must be set to 1 in ICACHE_CR.
When the ICACHE is disabled, it is bypassed and all transactions are copied from the slave
port to the master ports in the same clock cycle.
It is recommended to initialize or modify the main memory content (region to be later
cached) with the ICACHE disabled, and to enable the ICACHE only when this region
remains unchanged (an enabled ICACHE detects cacheable write transactions as errors).
To ensure performance determinism, it is recommended to wait for the end of a potential
cache invalidate procedure before enabling the ICACHE. This procedure occurs when the
hardware reset signal is released, when CACHEINV is set, or when EN is cleared in
ICACHE_CR. During the procedure, BUSYF is set in ICACHE_SR, and once finished,
BUSYF is cleared and BSYENDF is set in the same register (raising the ICACHE interrupt if
enabled on such a busy end condition).
The software must test BUSYF and/or BSYENDF values before enabling the ICACHE.
Else, if the ICACHE is enabled before the end of an invalidate procedure, any cache access
(while BUSYF = 1) is treated as noncacheable, and its performance depends on the main
memory access time.
The address remapping is performed, whether the ICACHE is enabled or not, if the input
transaction address falls into the memory regions defined and enabled in ICACHE_CRRx
(see Figure 29: ICACHE remapping address mechanism).
The ICACHE is by default disabled at boot.

RM0456 Rev 6

<!-- pagebreak -->

