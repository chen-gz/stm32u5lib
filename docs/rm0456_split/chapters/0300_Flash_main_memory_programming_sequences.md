363

Embedded flash memory (FLASH)
6.

RM0456

The MER1 or MER2 bits can be cleared if no more bank erase is requested.

Mass erase
To perform a mass erase, follow the procedure below:

Note:

1.

Check that no flash memory operation is ongoing by checking BSY in FLASH_NSSR or
FLASH_SECSR.

2.

Check and clear all nonsecure error programming flags due to a previous
programming. If not, the PGSERR bit is set.

3.

Set MER1 bit and MER2 bits in FLASH_NSCR or FLASH_SECCR.

4.

Set STRT in FLASH_NSCR or FLASH_SECCR.

5.

Wait for BSY bit to be cleared in FLASH_NSSR or FLASH_SECSR.

6.

The MER1 and MER2 bit can be cleared if no more mass erase is requested.

The internal oscillator HSI16 (16 MHz) is enabled automatically when the STRT bit is set,
and disabled automatically when the STRT bit is cleared, except if the HSI16 is previously
enabled with HSION in RCC_CR.
To erase a page, a bank or to perform a mass erase, the software must have sufficient
privilege (see Table 73 and Table 74).

7.3.7

Flash main memory programming sequences
The flash memory is programmed 137 bits at a time (128-bit data + 9 bits ECC).
Programming in a previously programmed address is not allowed except if the data to write
is full zero, and any attempt sets PROGERR flag in FLASH_NSSR or FLASH_SECSR.
It is only possible to program quad-word (4 x 32-bit data).
•

Any attempt to write byte or half-word sets SIZERR flag in FLASH_NSSR or
FLASH_SECSR.

•

Any attempt to write a quad-word that is not aligned with a quad-word address sets
PGAERR flag in FLASH_NSSR or FLASH_SECSR.

Flash programming
The flash memory programming sequence is as follows:

<!-- pagebreak -->

1.

Check that no flash main memory operation is ongoing by checking BSY in
FLASH_NSSR or FLASH_SECSR.

2.

Check that the write buffer is empty by checking WDW in FLASH_NSSR or
FLASH_SECSR.

3.

Check and clear all error programming flags due to a previous programming. If not,
PGSERR is set.

4.

Set PG bit in FLASH_NSCR or FLASH_SECCR.

5.

Perform the data write operation at the desired flash memory address, or in the OTP
area. Only a quad-word can be programmed and OTP can be only programmed in
non-secure access:
–

Write a first word in an address aligned on a quad-word address. WDW bits in
FLASH_NSSR and FLASH_SECSR are set to indicate that more data can be
written in the write buffer.

–

Write the second, third and fourth word in the same quad-word.

RM0456 Rev 6

RM0456

Note:

Embedded flash memory (FLASH)
6.

The BSY bit gets set. WDW is reset automatically.

7.

Wait until BSY is cleared in FLASH_NSSR or FLASH_SECSR. The software must
make sure that BSY is set or WDW is cleared before waiting for BSY to get cleared.

8.

If the EOP flag is set in FLASH_NSSR or FLASH_SECSR (meaning that the
programming operation has succeeded and the EOPIE bit is set), it must be cleared
by software.

9.

Clear PG in FLASH_NSCR or FLASH_SECCR) if there is no more programming
request.

When the flash memory interface received a good sequence (a quad-word), programming is
automatically launched and BSY bits are set. The internal oscillator HSI16 (16 MHz) is
enabled automatically when PG bit is set, and disabled automatically when PG bit is
cleared, except if the HSI16 is previously enabled with HSION in RCC_CR.
No option bytes modification nor erase request is allowed when WDW bit is set.
Programming is possible only if the privileged and security attributes are respected
(refer to Section 7.7).
If the user needs to program only one word, the quad-word must be completed with the
erase value 0xFFFF FFFF to launch automatically the programming.
ECC is calculated from the quad-word to program.

Flash burst programming (8 quad-words)
The flash memory burst programming sequence is as follows:

Note:

1.

Check that no flash main memory operation is ongoing by checking BSY bit
in FLASH_NSSR or FLASH_SECSR.

2.

Check that the write buffer is empty by checking WDW in FLASH_NSSR or
FLASH_SECSR.

3.

Check and clear all error programming flags due to a previous programming. If not,
PGSERR is set.

4.

Set BWR and PG bits in FLASH_NSCR or FLASH_SECCR.

5.

Perform the data write operation at the desired flash memory address, or in the OTP
area. Only 8 quad-words can be programmed:
–

Write a first 32-bit word in an address aligned on a 8 * quad-word address
(multiple of 0x80). WDW bits in FLASH_NSSR and FLASH_SECSR are set to
indicate that more data can be written in the write buffer.

–

Write the 31 other 32-bit words consecutively.

6.

The BSY bit gets set. WDW is reset automatically.

7.

Wait until BSY is cleared in FLASH_NSSR or FLASH_SECSR). The software must
make sure that BSY is set or WDW is cleared before waiting for BSY to get cleared.

8.

If EOP flag is set in FLASH_NSSR or FLASH_SECSR (meaning that the programming
operation has succeeded and EOPIE is set), it has to be cleared by software.

9.

Clear BWR and PG bits in FLASH_NSCR or FLASH_SECCR if there is no more
programming request.

When the flash memory interface received a good sequence, programming is automatically
launched and the BSY bits are set. The internal oscillator HSI16 (16 MHz) is enabled

RM0456 Rev 6

<!-- pagebreak -->

