910

Chrom-GRC (GFXMMU)

RM0456

Cache maintenance operation
When the cache is working in non force caching mode, this is the case when the force
caching (FC) bit of the GFXMMU configuration register (GFXMMU_CR) is reset, then line
eviction is done:
•

Each time a non cachable access is performed on the buffer having data cached

•

Each time a miss occurs

As a consequence the cache maintenance operation can be naturally automatic.
Nevertheless, it is possible to force a flush if the cache setting the force flush (FF) bit of the
GFXMMU cache control register (GFXMMU_CCR). When flushing the cache, all the dirty
entries are sent to the write buffer, and all the dirty bit of the TAGs are reset. But the entries
are not invalidate.
It is also possible to invalidate the cache entries setting the force invalidate (FI) bit of the
GFXMMU cache control register (GFXMMU_CCR). This does not send the dirty entries to
the write buffer (modification done in the cache are lost). This just resets the ID field of the
TAGs (unused state)
Setting the two force flush (FF) bits and forcing invalidate (FI) of the GFXMMU cache control
register (GFXMMU_CCR) trigs the following operations:
•

flush the cache, all the dirty entries are sent to the write buffer

•

reset the dirty bit of the TAGs

•

reset the ID field of the TAGs

When any of these two operations is done while the flash is having transaction already in
the write buffer, the write buffer continues its operations.
The force flush (FF) and force invalidate (FI) are reset automatically when all the operations
are finished (write buffer empty) creating a synchronization barrier.

21.4.4

Address cache
A specific cache can be activated to retain the latest MMU lookup results in order to reduce
latencies due to the evaluation process. This address cache is only active on a selected
virtual buffer.
This lookup cache is activated with address cache enable (ACE) bit in GFXMMU control
register (GFXMMU_CR).
The cached virtual buffer is selected with address cache lock buffer (ACLB) bitfield in
GFXMMU control register (GFXMMU_CR).

<!-- pagebreak -->

