139

Memory and bus architecture

RM0456

This architecture is shown in the figure below.
Figure 2. SmartRun domain architecture
Main
bus matrix

LPDMA1
Legend

AHB3
peripherals

MPCBB4

Bus multiplexer

Master interface

Fast bus multiplexer

Slave interface

MPCBBx: block-based memory protection controller

SRAM4

SRD 32-bit bus matrix

2.2

MSv63648V1

Arm TrustZone security architecture
The security architecture is based on Arm TrustZone with the Armv8-M mainline extension.
The TrustZone security is activated by the TZEN option bit in the FLASH_OPTR register.
When the TrustZone is enabled, the SAU (security-attribution unit) and IDAU
(implementation-defined-attribution unit) define the access permissions based on secure
and nonsecure states.
•

SAU: Up to eight SAU configurable regions are available for security attribution.

•

IDAU: provides a first memory partition as nonsecure or nonsecure callable attributes.
The IDAU memory map partition is not configurable and fixed by hardware
implementation (refer to Figure 3 to Figure 6 in Section 2.3.2: Memory map and
register boundary addresses). It is then combined with the results from the SAU
security attribution and the higher security state is selected.

Based on IDAU security attribution, the flash memory, system SRAMs, and peripherals
memory space are aliased twice for secure and nonsecure states. The external memories
space is not aliased.
The table below shows an example of typical eight SAU regions mapping based on IDAU
regions. The user can split and choose the secure, nonsecure or NSC regions for external
memories as needed.
Table 3. Example of memory map security attribution versus SAU configuration regions
Region
description

Address range

IDAU
security
attribution

SAU security
attribution typical
configuration

Final security
attribution

Code - external
memories

0x0000_0000 0x07FF_FFFF

Nonsecure

Secure, nonsecure or
NSC(1)

Secure, nonsecure,
or NSC

0x0800_0000 0x0BFF_FFFF

Nonsecure

Nonsecure

Nonsecure

0x0C00_0000 0x0FFF_FFFF

NSC

Secure or NSC

Secure or NSC

Code - flash
memory and SRAM

<!-- pagebreak -->

