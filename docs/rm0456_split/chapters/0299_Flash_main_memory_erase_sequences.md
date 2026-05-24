RM0456 Rev 6

RM0456
Note:

Embedded flash memory (FLASH)
FLASH_NSCR and FLASH_SECCR cannot be written when the BSY bits are set.
Any attempt to write them with BSY bits set, causes the AHB bus to stall until the BSY bits
are cleared.

Wait for data-to-write flags (WDW)
The WDW flags in FLASH_NSSR and FLASH_SECSR are both set when a secure or
non-secure write access has been done in the write buffer. They are cleared when BSY
flags are set (meaning that the write buffer is freed and the programming operation actually
starts in the flash memory) or in case of error.
The software must ensure that the four words in the same quad-word are all written.

Flash secure and nonsecure busy flags
BSY flags in FLASH_NSSR and FLASH_SECSR are both set when a secure or nonsecure
flash operation is started:

7.3.6

•

Erase operation: setting STRT in FLASH_NSCR) or FLASH_SECCR.

•

Write operation: setting PG in FLASH_NSCR or FLASH_SECCR, and writing a
quad-word in the flash memory.

•

Option-byte programming: setting OPTSTRT in FLASH_NSCR.

Flash main memory erase sequences
The flash memory erase operation can be performed at page level, bank level or on the
whole flash memory (mass erase). Mass erase does not affect the information block
(system flash, OTP and option bytes). The erase operation is either secure or nonsecure.

Page erase
To erase a page, follow the procedure below:
1.

Check that no flash memory operation is ongoing by checking BSY in FLASH_NSSR
or FLASH_SECSR.

2.

Check and clear all error programming flags due to a previous programming.
If not, PGSERR is set.

3.

Set PER bit and select the page to erase (PNB) with the associated bank (BKER)
in FLASH_NSCR or FLASH_SECCR.

4.

Set STRT in FLASH_NSCR or FLASH_SECCR.

5.

Wait for BSY to be cleared in FLASH_NSSR or FLASH_SECSR.

Bank 1 or bank 2 mass erase
To perform a bank mass erase, follow the procedure below:
1.

Check that no flash memory operation is ongoing by checking BSY in FLASH_NSSR or
FLASH_SECSR.

2.

Check and clear all error programming flags due to a previous programming. If not,
PGSERR is set.

3.

Set the MER1 or MER2 bit (depending on the bank) in FLASH_NSCR or
FLASH_SECCR. Both banks can be selected in the same operation, in that case it
corresponds to a mass erase.

4.

Set STRT in FLASH_NSCR or FLASH_SECCR.

5.

Wait for BSY bit to be cleared in FLASH_NSSR or FLASH_SECSR.
RM0456 Rev 6

<!-- pagebreak -->

