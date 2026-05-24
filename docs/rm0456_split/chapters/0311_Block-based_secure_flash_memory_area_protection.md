If the banks are not swapped, the option bytes registers must be programmed with:
–

SECWM1_PSTRT = 0x2

–

HDP1_PEND = 0x3

If the two banks are swapped, the protection must apply to bank 2 and the option bytes
registers must be programmed with:
–

SECWM2_PSTRT = 0x2

–

HDP2_PEND = 0x3

For more details on the bank swapping mechanism, refer to Section 7.5.8.

RM0456 Rev 6

RM0456

Embedded flash memory (FLASH)
If an invalid secure HDP area is defined as described in the table below, the OPTWERR flag
error is set and option bytes modification is discarded.
Table 60. Secure hide protection

HDPx watermark option-byte values (x = 1,2)
HDPxEN = 0

HDPxEN = 1

HDP area

-

No secure HDP area

SECWMx_PSTRT ≤ HDPx_PEND
≤ SECWMx_PEND

The area between SECWMx_PSTRT and HDPx_PEND is
secure HDP protected.

Others

Invalid secure area. The HDP area is defined outside the
secure area.

The table below summarizes the possible secure and HPD protection area configurations.
Table 61. Secure and HDP protections
Secure and HDP watermark option-byte values
Protections area
HDPxEN

Option bytes

x

SECWMx_PSTRT > SECWMx_PEND No secure area

0

No secure HDP area
Secure between SECWMx_PSTRT and
SECWMx_PSTRT ≤ SECWMx_PEND SECWMx_PEND
– If SECWMx_PSTRT = SECWMx_PEND, one page defined
by SECWMx_PSTRT is secure protected.

1

7.5.4

SECWMx_PSTRT ≤ HDPx_PEND
≤ SECWMx_PEND

The area between SECWMx_PSTRT and HDPx_PEND is
secure HDP protected.
– If SECWMx_PSTRT = HDPx_PEND, one page defined by
HDPx_PEND is secure HDP protected.

Others

Invalid secure area. The HDP area is defined outside the
secure area.

Block-based secure flash memory area protection
Any page can be programmed on-the-fly as secure or nonsecure using the block-based
configuration registers. FLASH_SECBB1Rx (resp. FLASH_SECBB2Rx) are used
to configure the security attribute for pages in bank 1 (resp. bank 2).
When the page security attribute, bit i in SECyBBRx, is set, the security attribute is the same
as the secure watermark-based area. The secure page is only accessible by a secure
access.
If SECyBBi bit is set or reset for a page already included in a secure watermark-based area,
the page keeps the watermark-based protection security attributes.
To modify a block-based page security attribution, the following actions are recommended:
•

Check that no flash memory operation is ongoing on the related page.

•

Add an ISB instruction after modifying the page security attribute bit i in SECyBBRx.

RM0456 Rev 6

<!-- pagebreak -->

