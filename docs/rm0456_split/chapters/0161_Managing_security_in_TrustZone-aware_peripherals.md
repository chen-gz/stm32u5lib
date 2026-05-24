RM0456 Rev 6

RM0456

System security
For each illegal event source, a status flag and a clear bit exist. Each illegal event can be
masked, not generating an interrupt toward the NVIC.

Note:

By default, all events are masked.

3.5.5

Managing security in TrustZone-aware peripherals
This section gives more details on how the security is implemented in the TrustZone-aware
peripherals listed in the previous section.

Embedded flash memory
When the TrustZone security is enabled through option bytes (TZEN = 1), the whole flash
memory is secure after reset and the following protections, shown in the figure below, are
available to the application:
•

•

nonvolatile user secure areas, defined with nonvolatile secure user option bytes
–

watermark-based secure only area (x2)

–

secure hide protection (HDP) area, stickily hidden after boot (x2)

volatile user secure pages, defined with volatile secure registers (lost after reset)
–

Note:

Any page set as nonsecure (example: outside the watermark-based secure only
area), can be set as secure on-the-fly using the block-based configuration
registers.

All areas are aligned on the flash memory page granularity.
The flash memory area can be configured as secure while it is tagged as nonsecure in
Cortex-M33 IDAU/SAU. In this case, nonsecure accesses by the CPU to the flash memory
are denied.
Erase or program operations can be available to secure (resp. nonsecure) code only for
secure (resp. nonsecure) pages or memory. A flash memory is considered secure if at least
one page is secure.

RM0456 Rev 6

<!-- pagebreak -->

191

System security

RM0456
Figure 11. Flash memory TrustZone protections
TrustZone disabled

TrustZone enabled
Nonsecure pages
Secure pages
Nonsecure pages
Secure pages
Nonsecure pages

User flash memory

Flash memory-S (*)
Flash memory-S(*)
(HDP)

Boot

Boot

Read-only system
Flash memory

Bootloader

Bootloader-NS

Hidden

RSS(*) (HDP)

Boot
Boot

(*): nonvolatile security configuration

MSv64451V1

As shown above, when TrustZone is activated (TZEN = 1), the application code can use the
HDP area that is part of the flash memory watermark-based secure area. Indeed, when the
application sets the HDPx_ACCDIS bit, data read, write, and instruction fetch on this HDP
area is denied until the next system reset.
For example, the software code in the secure flash memory HDP area can be executed only
once, with any further access to this area denied until the next system reset. Additionally,
any flash memory page belonging to an active HDP area cannot be erased anymore.
When the TrustZone is deactivated (TZEN = 0), the volatile/non-volatile secure area
features are deactivated and all secure registers are RAZ/WI.
See Section 7: Embedded flash memory (FLASH) for more details.

On-the-fly encryption/decryption (OTFDEC)
When the TrustZone security is activated (TZEN = 1), the OTFDEC can only be initialized
by secure applications. Each of the four encrypted regions, once the configuration is
confirmed, can be write-locked until the next power-on-reset.
Note:

Any application (secure or nonsecure) can verify the initialization context of each OTFDEC
region (including the CRC of the keys), by reading the peripheral registers.
Key registers in each OTFDEC are write-only.
See Section 3.9.3 for more details on this cryptographic engine.

Direct memory access controllers (LPDMA and GPDMA)
When a DMA channel x is defined as secure (SECx = 1 in LP/GPDMA_SECCFGR), the
source and destination transfers can be independently set as secure or nonsecure by a
secure application using SSEC and DSEC bits in LP/GPDMA_CxTR1.

<!-- pagebreak -->

