400

Data cache (DCACHE)
–

RM0456

nonshareable

These AHB attributes cannot be propagated from the slave port (as it is the case for all
other transactions emitted by the DCACHE) because the evicting transaction has no
relation with the initial missing transaction. Setting of the AHB attributes is fixed, except
for the privileged bit that is copied from the TAG privilege bit of the evicted line.

9.4.7

DCACHE security
The DCACHE implements an Armv8-M TrustZone.
DCACHE configuration registers are protected at system level.

9.4.8

DCACHE maintenance
The DCACHE features several maintenance operations that the software can programmed
in DCACHE_CR control register:
•

Full invalidate: invalidates the whole cache, non interruptible task.
The software can invalidate the whole DCACHE content by programming CACHEINV
in DCACHE_CR.
When CACHEINV = 1, the DCACHE control logic sets BUSYF flag in DCACHE_SR
status register, and performs the operation of cache invalidation, resetting each TAG
valid bit to 0 (one valid bit per cache line). Each dirty and privilege bits are also reset to
0 during cache invalidation to prevent unknown values at next cache line validation.
CACHEINV is automatically cleared.
Once the full invalidate operation is finished, the DCACHE automatically clears BUSYF
flag, and sets BSYENDF in DCACHE_SR.
If enabled on this flag condition (BSYENDIE = 1 in DCACHE_IER), the DCACHE
interrupt is raised. Then, the (empty) cache is available again.
This full invalidate operation is not interruptible, meaning that the cache does not treat
any cacheable request while BUSYF = 1. However, non-cacheable traffic is treated
(since the request address is not compared to TAG ones), the DCACHE being
bypassed in the same clock cycle (same behavior as when the DCACHE is disabled).

•

Invalidate range: invalidates a certain range of addresses in the cache, background
task (interruptible).
The software can invalidate a given data region in the DCACHE by programming
STARTCMD = 1 and CACHECMD = 0b010 in DCACHE_CR, after the address range
was programmed into DCACHE_CMDRSADDRR (range start address) and
DCACHE_CMDREADDRR (range end address).
The DCACHE control logic then parses the whole TAG memory. If the read line
address (TAG address + line index) falls in the programmed address range
(DCACHE_CMDRSADDRR ≤ Line Addr ≤ DCACHE_CMDREADDRR), the

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Data cache (DCACHE)
corresponding cache line is invalidated (line TAG bits cleared,
valid bit = dirty bit = privilege bit = 0).
When STARTCMD is set, the DCACHE control logic sets BUSYCMDF in DCACHE_SR
and launches the invalidate range operation. STARTCMD is also automatically cleared.
Once the operation is finished (all TAG memory parsed), the DCACHE automatically
clears BUSYCMDF and sets CMDENDF in DCACHE_SR.
If enabled on this flag condition (CMDENDIE = 1 in DCACHE_IER), the DCACHE
interrupt is raised.
During this invalidate range operation, the DCACHE is interruptible, meaning it can
accept new incoming requests that take higher priority than the invalidation process.
The TAG memory is accessed for invalidate range operation only if not already
accessed by an external cache request. This implies that invalidate range execution is
usually not performed in one go, but can be interrupted.
•

Clean range: cleans a certain range of addresses in the cache, background task
(interruptible).
Cleaning a cache line means making sure that the main memory content is up-to-date
with the data, which may have been modified in the cache. The clean operation
consists in performing the write-back in the main memory of the cache lines that are
tagged as “dirty” (the ones with TAG dirty bit set).
The software can clean a given data region in DCACHE by programming
STARTCMD = 1, and CACHECMD = 0b001 in DCACHE_CR, after the address range
was programmed into DCACHE_CMDRSADDRR (range start address) and
DCACHE_CMDREADDRR (range end address).
The DCACHE control logic then parses the whole TAG memory. If the read line
address (TAG address + line index) falls in the programmed address range
(DCACHE_CMDRSADDRR ≤ Line Addr ≤ DCACHE_CMDREADDRR), and the
corresponding line is dirty, this line is cleaned, meaning the whole cache line is
written-back in the memory through the DCACHE master port, and its TAG dirty bit
is cleared.
When STARTCMD is set, the DCACHE control logic sets BUSYCMDF in DCACHE_SR
and launches the clean range operation. STARTCMD is also automatically cleared.
Once the operation is finished (all TAG memory parsed), the DCACHE automatically
clears BUSYCMDF and sets CMDENDF in DCACHE_SR.
If enabled on this flag condition (CMDENDIE = 1 in DCACHE_IER), the DCACHE
interrupt is raised.
During this clean range operation, the DCACHE is interruptible, meaning it can accept
new incoming requests that take higher priority than the cleaning process. The TAG
memory is accessed for clean range operation only if not already accessed by an
external cache request. This implies that clean range execution is usually not
performed in one go, but can be interrupted.
It is under the software responsibility that no bus initiator attempts to change the
content of the region being cleaned until clean range is completed. For that, the
software can take advantage of BUSYCMDF flag in DCACHE_SR, and can poll this
flag to prevent any spurious access to the area being cleaned.

RM0456 Rev 6

<!-- pagebreak -->

