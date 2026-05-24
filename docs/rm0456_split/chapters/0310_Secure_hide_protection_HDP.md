363

Embedded flash memory (FLASH)

RM0456

Table 59. Secure watermark-based area (continued)
Secure watermark option-byte values (x = 1,2)

Secure watermark protection area

SECWMx_PSTRT = SECWMx_PEND

One page defined by SECWMx_PSTRT is secure watermarkbased protected

SECWMx_PSTRT < SECWMx_PEND

The area between SECWMx_PSTRT and SECWMx_PEND is
secure watermark-based protected.

Caution:

Switching a flash memory area from secure to no-secure does not erase its content.
The user secure software must perform the needed operation to erase the secure area
before switching an area to nonsecure attribute whenever is needed. It is also
recommended to flush the instruction cache.

7.5.3

Secure hide protection (HDP)
The secure HDP area is part of the flash memory watermark-based secure area. Access to
the hide-protection area can be denied by setting HDPx_ACCDIS in FLASH_SECHDPCR.
When HDPx_ACCDIS is set, instruction fetch, data read, write, and erase operations on this
HDP area are denied. For example, software code in the secure-flash hide-protected area
can be executed only once, and deny any further access to this area until next system reset.
HDPx_ACCDIS can be only cleared by a system reset.

Note:

The software must take any appropriate action to protect the HDP code before resetting the
HDPxEN bit such as erasing the HDP area and flushing the instruction cache.
One non-volatile secure HDP area per bank can be defined with a page granularity.
The secure HDP area is enabled by HDPxEN (x = 1,2 for area 1 and area 2).
When HDPxEN is reset, there is no HDP area. The HDPxEN bit can be set or reset on the
fly by the secure firmware if HDPx_ACCDIS bit is reset. If HDPx_ACCDIS is set, HDPxEN
and secure watermark configuration cannot be modified until next system reset.
The secure HDP area size is defined by the end-page offset using HDPx_PEND option
bytes while the start-page offset is already defined by SECWMx_PSTRT option bytes.These
offsets are defined in the secure watermark registers address registers:
FLASH_SECWM1R1, FLASH_SECWM1R2, FLASH_SECWM2R1, and
FLASH_SECWM2R2.
For example, to protect by HDP from the address 0x0C00 4000 (included) to the address
0x0C00 5FFF (included):
•

•

Note:

<!-- pagebreak -->

