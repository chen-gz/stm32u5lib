RM0456 Rev 6

RM0456

9.5

Data cache (DCACHE)

DCACHE low-power modes
At product level, the DCACHE reduces the power consumption by loading/storing data
from/to the internal DCACHE most of the time, rather than from the bigger and then more
power consuming main memories. This reduction is even much higher, since the cached
main memories are external.

9.6

DCACHE error management and interrupts
A transaction initiated on the DCACHE master port may return an error (a write attempt into
a read-only memory, for instance). If the master port request was propagated from a slave
port request, the error is propagated back to the slave port. If ever the master port request is
initiated by the DCACHE itself (a cache line is written back into the main memory because
of an eviction or a clean operation), the DCACHE receives this functional error and flags it
internally by setting the ERRF flag in DCACHE_SR.
In such a case, an interrupt is generated if the corresponding interrupt enable bit is set
(ERRIE = 1 in DCACHE_IER).
Another case of potential interrupt generation is at the end of a full invalidate operation:
when the cache busy state is finished, the DCACHE sets BSYENDF flag in DCACHE_SR.
An interrupt is then generated if the corresponding interrupt enable bit is set (BSYENDIE = 1
in DCACHE_IER).
Last case is at the end of invalidate and/or clean range operations: when the command
busy state is finished, the DCACHE sets CMDENDF flag in DCACHE_SR.
An interrupt is also generated if the corresponding interrupt enable bit is set
(CMDENDIE = 1 in DCACHE_IER).
All DCACHE interrupt sources raise the same and unique interrupt signal, dcache_it, and
then use the same interrupt vector.
Table 93. DCACHE interrupts

Interrupt vector

DCACHE

Interrupt event

Event flag

Enable control bit

Interrupt clear method

Functional error

ERRF
in DCACHE_SR

ERRIE
in DCACHE_IER

Set CERRF to 1
in DCACHE_FCR

End of busy state
(full invalidate finished)

BSYENDF
in DCACHE_SR

BSYENDIE
in DCACHE_IER

Set CBSYENDF to 1
in DCACHE_FCR

End of cache operations
(address range based)

CMDENDF
in DCACHE_SR

CMDENDIE
in DCACHE_IER

Set CCMDENDF to 1
in DCACHE_FCR

The DCACHE also propagates all AHB bus errors (such as security issues, address
decoding issues) from master port back to the S-AHB slave port.

RM0456 Rev 6

<!-- pagebreak -->

