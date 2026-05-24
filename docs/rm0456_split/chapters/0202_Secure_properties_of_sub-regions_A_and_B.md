275

Global TrustZone controller (GTZC)

RM0456

A control register for each sub-region can be used to enable/disable the watermark memory
protection controller as well as defining the right attributes of each sub-region.
Figure 20. Watermark memory protection controller (region x/sub-regions A and B)
Region x in secure privileged by default

Secure privileged
SUBB_START + SUBB_LENGTH
SUBB_START
SUBA_START + SUBA_LENGTH

Nonsecure unprivileged
Secure privileged

Sub-region B

Nonsecure privileged

Sub-region A

SUBA_START

Secure privileged
MSv63635V1

In the figure above, region x represents the external memory or backup SRAM region (such
as FSMC bank, OCTOSPI1, OCTOSPI2, HSPI1 or BKPSRAM). Secure and privileged
attributes of sub-regions A and B are independently configurable. When no sub-regions are
defined or enabled on the region x, then the default attribute of the region x is set as
“secure-privileged”.
The tables below describe the secure/privileged properties of the common area of subregion A and B when an overlapNonsecure exists.
Table 36. Secure properties of sub-regions A and B
Sub-region A

Sub-region B

Properties of overlapped
region A and B

Nonsecure

Nonsecure

Nonsecure

Nonsecure

Secure

Nonsecure

Secure

Nonsecure

Nonsecure

Secure

Secure

Secure

Table 37. Privileged properties of sub-regions A and B

<!-- pagebreak -->

