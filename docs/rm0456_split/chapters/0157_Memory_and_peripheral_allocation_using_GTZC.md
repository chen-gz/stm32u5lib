RM0456 Rev 6

RM0456

System security

Programming security attributes
In Cortex-M33, the static implementation defined attribution unit (IDAU) works in conjunction
with the programmable security attribution unit (SAU) to assign a specific security attribute
(S, NS, or NSC) to a specific address, as shown in the table below.
Table 8. Configuring security attributes with IDAU and SAU
IDAU security attribution

Nonsecure

Secure-NSC

SAU security attribution(1)

Final security attribution

Secure

Secure

Secure-NSC

Secure-NSC

Nonsecure

Nonsecure

Secure

Secure

Nonsecure

Secure-NSC

1. Defined regions are aligned to 32-byte boundaries.

The SAU can only be configured by the Cortex-M33 in the secure-privilege state. When the
TrustZone is enabled, the SAU defaults all addresses as secure (S). A secure boot
application can then program the SAU to create NSC or NS regions, as shown in the
previous table.
Note:

The SAU/IDAU settings are applicable only to the Cortex-M33. The other masters like DMA
are not affected by these policies.
A memory space not covered by an SAU region is fixed as secure.
For more information on memory security attribution using IDAU/SAU, refer to the
application note TrustZone features on STM32L5 and STM32U5 Series (AN5347).

3.5.4

Memory and peripheral allocation using GTZC
Global TrustZone framework architecture
On top of the Armv8-M TrustZone security extension in Cortex-M33, the devices embed
complementary security features that reinforce, in a flexible way, the isolation between
the secure and the nonsecure worlds. Unlike the SAU/IDAU, the GTZC can protect legacy
memories and peripherals against nonsecure transactions coming from other masters
than the Cortex-M33.

RM0456 Rev 6

<!-- pagebreak -->

191

System security

RM0456
Figure 10. Global TrustZone framework and TrustZone awareness

Config

Device boundary
Other bus
master

CPU

DMA

SAU+IDAU
MPU

Flash memory

SRAM

OTFDEC

External
Flash memory
External
SRAM

Peripheral

Peripheral

Peripheral

I/O

I/O

Peripheral

Global TrustZone framework
TrustZone-aware peripherals
MSv64450V1

Securing peripherals with TZSC
When the TrustZone security is active, a peripheral is either securable through the TZSC
in GTZC, or is natively TrustZone-aware, as shown in the previous figure:
•

A securable peripheral or memory is protected by an AHB/APB firewall gate, that is
controlled by the TrustZone security controller (TZSC).

•

A TrustZone-aware peripheral or memory is connected directly to the AHB or APB
interconnect, implementing a specific TrustZone behavior, such as a subset of secure
registers, or a secure memory area.

When a securable peripheral is made secure-only with the GTZC, if this peripheral is master
on the interconnect (such as SDMMC), it automatically issues secure transactions. The
SDMMC is an example of securable master. TrustZone-aware AHB masters like
Cortex-M33 or DMAs, drive a secure signal in the AHB interconnect, according to their
security mode, independently to the GTZC.

<!-- pagebreak -->

