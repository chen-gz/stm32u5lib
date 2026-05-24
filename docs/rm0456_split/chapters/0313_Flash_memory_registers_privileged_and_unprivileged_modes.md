RM0456 Rev 6

RM0456

7.5.7

Embedded flash memory (FLASH)

Flash memory registers privileged and unprivileged modes
The flash memory registers can be read and written by privileged and unprivileged
accesses depending on SPRIV and NSPRIV bits in FLASH_PRIVCFGR, with the following
rules:
•

When the SPRIV (resp. NSPRIV) is reset, all secure (resp. nonsecure) flash memory
registers can be read and written by both privileged or unprivileged access.

•

When the SPRIV (resp. NSPRIV) is set, all secure (resp. nonsecure) flash memory
registers can be read and written by privileged access only. Unprivileged access to a
privileged registers is RAZ/WI.

Table 72 summarizes the flash memory registers access control.

7.5.8

Flash memory bank attributes in case of bank swap
The SWAP_BANK option bit modifies the address of each bank in the memory map. When
SWAP_BANK is reset, the flash memory bank 1 is at the lower address range. When
SWAP_BANK is set, the flash memory bank 1 is at the higher address range.
Flash memory bank attributes follow their bank so there is no need to modify the following
registers when swapping banks:

Note:

•

FLASH secure watermark y register x FLASH_SECWMyRx

•

FLASH write protection x area y FLASH_WRPxyR (refer to Section 7.6.1)

•

FLASH secure block based bank y register x FLASH_SECyBBRx

•

FLASH privilege block based bank y register x FLASH_PRIVyBBRx

•

PDREQx bits in FLASH_ACR

•

PDx bits in FLASH_NSSR

BK_ECC bit in FLASH_ECCR always refers to bank 1 (resp. bank 2) when it is low
(resp. high), whatever SWAP_BANK value.
BK_OP bit in FLASH_OPSR always refers to bank 1 (resp. bank 2) when it is low
(resp. high), whatever SWAP_BANK value.

RM0456 Rev 6

<!-- pagebreak -->

