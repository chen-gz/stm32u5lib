•

write protection (WRP)

•

readout protection (RDP)

RM0456 Rev 6

RM0456

Embedded flash memory (FLASH)
•

•

7.6.1

additional secure protections when TrustZone is active (refer to Section 7.5)
–

up to two secure watermark-based non-volatile areas

–

up to two secure hide protection areas

–

secure block-based volatile areas with page granularity

privileged block-based volatile areas with page granularity (refer to Section 7.5.6)

Write protection (WRP)
The user area in flash memory can be protected against unwanted write operations.
Two write-protected (WRP) areas can be defined in each bank, with page granularity.
Each area is defined by a start page offset and an end page offset related to the physical
flash bank base address. These offsets are defined in the WRP address registers:
FLASH_WRP1AR, FLASH_WRP1BR, FLASH_WRP2AR, and FLASH_WRP2BR.
The bank “x” WRP “y” area (x = 1,2 and y = A,B) is defined as follows:
•

from the address: bank “x” base address + [WRPxy_PSTRT x 0x2000] (included)

•

to the address: bank “x” base address + [(WRPxy_PEND+1) x 0x2000] (excluded)

For example, to protect by WRP from the address 0x0806 2000 (included) to the address
0x0807 3FFF (included):
•

If the banks are not swapped, FLASH_WRP1AR register must be programmed with:
–

WRP1A_PSTRT = 0x31

–

WRP1A_PEND = 0x39

WRP1B_PSTRT and WRP1B_PEND in FLASH_WRP1BR can be used instead (area
“B” in bank 1).
•

If the two banks are swapped, the protection must apply to bank 2, and
FLASH_WRP2AR register must be programmed with:
–

WRP2A_PSTRT = 0x31

–

WRP2A_PEND = 0x39

WRP2B_PSTRT and WRP2B_PEND in FLASH_WRP2BR can be used instead (area
“B in bank 2).
Note:

For more details on the bank swapping mechanism, refer to Section 7.5.8.
When WRP is active, protected flash memory pages cannot be erased or programmed.
Consequently, a software mass erase cannot be performed if one area is write-protected.
If an erase/program operation to a write-protected part of the flash memory is attempted, the
secure or nonsecure write protection error flag (WRPERR) is set in the FLASH_NSSR or
FLASH_SECSR register. This flag is also set for any write access to the following:

Note:

•

system flash memory

•

OTP area

When the memory readout protection level 1 is selected (RDP level = 1), it is not possible to
program or erase the flash memory (secure or nonsecure) if the CPU debug features are
connected (JTAG or single wire) or boot code is being executed from RAM or system flash
memory, even if WRP is not activated.
When the memory readout protection level 0.5 is selected (RDP level = 0.5), it is not
possible to program or erase the flash secure memory if the CPU debug features are
connected (JTAG or single wire), even if WRP is not activated.

RM0456 Rev 6

<!-- pagebreak -->

