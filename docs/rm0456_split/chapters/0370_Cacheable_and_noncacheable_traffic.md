381

Instruction cache (ICACHE)

8.4.6

RM0456

Cacheable and noncacheable traffic
The ICACHE is developed for the Cortex®-M33 core. It is placed on the C-AHB bus, and
thus caches the code memory region, ranging from address 0x0000 0000 to 0x1FFF FFFF
of the memory map.
To make some other memory regions cacheable, the ICACHE supports a memory-regionremapping feature. It is used to define up to four external memory regions, which addresses
have an alias in the code region. Addressing these external memory regions through their
code alias address allows the memory request to be routed to the C-AHB bus, and to be
managed by the ICACHE.
Any external memory space physically mapped at an address in the range
[0x6000 0000:0xAFFF FFFF] can be aliased with an address in the range
[0x0000 0000:0x07FF FFFF] or [0x1000 0000:0x1FFF FFFF].
For a given memory request in the code region, the ICACHE implements the address
remapping functionality first. If aliased, it is the remapped address, which is then cached,
and, if needed, provided to the master port to address the main AHB bus matrix.
The destination physical address does not need further manipulation on the AHB bus.
The remapping functionality is also available for noncacheable traffic, and when the cache
is disabled.
Further details on address remapping are provided in Section 8.4.7.
An incoming memory request to the ICACHE is defined as cacheable according to its AHB
transaction memory lookup attribute, as shown in Table 84. This AHB attribute depends on
the MPU (memory protection unit) programming for the addressed region.
Table 84. ICACHE cacheability for AHB transaction
AHB lookup attribute

Cacheability

1

Cacheable

0

Noncacheable

In the case of a noncacheable access (either a noncacheable read or a noncacheable
write), the ICACHE is bypassed, meaning that the AHB transaction is propagated
unchanged to the master output port, except the transaction address, which may be
modified due to the address remapping feature (see Section 8.4.7).
The bypass, and eventual remap logic, does not increase the latency of the access to the
targeted memory.
In the case of a cacheable access, the ICACHE behaves as explained in Section 8.4.8.
Cacheable memory regions are defined and programmed by the user in the MPU that is
responsible for the generation of the AHB attribute signals for any transaction addressing a
given region.

<!-- pagebreak -->

