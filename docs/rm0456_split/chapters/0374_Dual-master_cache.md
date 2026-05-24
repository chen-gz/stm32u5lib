381

Instruction cache (ICACHE)

RM0456

If ever the cache line where the refill data must be written is already valid, the targeted
cache line must be invalidated first. This is true whatever the direct map or n-way set
associative cache mode.

8.4.9

Dual-master cache
The ICACHE implements a dual-port AHB master on the main AHB bus matrix: master1 and
master2 ports. This is used to split the traffic going to different destination memories.
The nonremapped traffic goes systematically to master1 port. The remapped traffic can be
routed on the master2 port by programming MSTSEL in ICACHE_CRRx (on a region basis).
The code can typically be fetched as follows:
•

internal flash memory and internal SRAM on master1 port (Fast bus)

•

external flash memory/RAM on master2 port (Slow bus)

For systems not implementing external memories, the traffic to the internal flash memory
can be decoupled from the traffic to the internal SRAM (when remapped by the ICACHE).
This feature is used to prevent further processor stalls on misses.
Alongside with hit-under-miss, this dual-master feature allows the processor to have an
alternative path in case of fetching from different memories.

8.4.10

ICACHE security
The ICACHE implements an Armv8-M TrustZone.
ICACHE configuration registers are protected at system level.

8.4.11

ICACHE maintenance
The software can invalidate the whole content of the ICACHE by programming CACHEINV
in the ICACHE_CR register.
When CACHEINV = 1, the ICACHE control logic sets the BUSYF flag in ICACHE_SR and
launches the invalidate cache operation, resetting each TAG valid bit to 0 (one valid bit per
cache line). CACHEINV is automatically cleared.
Once the invalidate operation is finished (all valid bits reset to 0), the ICACHE automatically
clears BUSYF, and sets BSYENDF in the ICACHE_SR register.
If enabled on this flag condition (BSYENDIE = 1 in ICACHE_IER), the ICACHE interrupt
is raised. Then, the (empty) cache is available again.

<!-- pagebreak -->

