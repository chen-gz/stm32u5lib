363

Embedded flash memory (FLASH)

RM0456

automatically when PG bit is set, and disabled automatically when PG bit is cleared, except
if the HSI16 is previously enabled with HSION in RCC_CR register.
No option bytes modification nor erase request is allowed when WDW bit is set.
Programming is possible only if the privileged and security attributes are respected
(see Section 7.7).

7.3.8

Flash memory endurance
Each flash memory page can be written and erased 10 000 or 100 000 times. A maximum of
32 pages (256 Kbytes) per bank feature this increased endurance of 100 kcycles. This
enhanced endurance can be used for data storage that usually needs more intensive
cycling capability than code storage.
Any flash page can be chosen to be cycled up to 100 kcycles. As soon as a page is above
10 kcycles, it is considered as high cycling page (even if not yet at 100 kcycles). The
application must take care not to exceed 32 pages cycled more than 10 000 times.
For STM32U535/545, as it fits a maximum of 32 pages (256 Kbytes) per bank, the entire
flash memory is 100-kcycle capable.

7.3.9

Flash memory errors flags
Flash programming errors
Several kind of errors can be detected during secure and nonsecure operations. In case of
error, the flash memory operation (programming or erasing) is aborted.
The secure errors flags are only set during a secure operation and nonsecure flags are only
set during a nonsecure operation.
•

PROGERR: secure/nonsecure programming error
It is set when the word to program is pointing to an address:

•

–

not previously erased

–

already fully programmed to 0

–

already partially programmed (contains 0 and 1) and the new value to program
is not full zero

–

for OTP programming, when the address is already partially programmed
(contains 0 and 1)

SIZERR: secure/nonsecure size programming error
Only 32-bit data can be written. SIZERR flag is set if a byte or a half-word is written.

•

PGAERR: secure/nonsecure alignment programming error
It is set when the first word to be programmed is not aligned with a quad-word address,
or the second, third or forth word does not belong to the same quad-word address.
For burst programming, it is set when the first word to be programmed is not aligned
on a 8 *quad-word address or if the following word writes are not done at consecutive
32-bit addresses.

•

PGSERR: programming sequence error
PGSERR is set if one of the following conditions occurs during a erase or program
operation:
–

<!-- pagebreak -->

A data is written when PG is cleared.

RM0456 Rev 6

RM0456

Embedded flash memory (FLASH)

•

–

A program operation is requested during erase: PG is set while MER1,MER2,
or PER is set.

–

In the erase sequence, PG is set while STRT is already set.

–

In the erase sequence, if STRT is set while MER1, MER2, and PER are cleared.

–

If page and mass erase are requested at the same time, STRT and PER are set
while MER1 or MER2 is set.

–

If an operation is started while the write buffer is waiting for the next data, STRT
or OPTSTRT is set while WDW is already set.

–

If STRT and OPTSTRT are set at the same time.

–

A nonsecure PGSERR is set if the nonsecure STRT bit is set by a secure access.

–

A secure PGSERR is set if PROGERR, SIZERR, PGAERR, WRPER,R or
PGSERR is already set due to a previous programming error.

–

A nonsecure PGSERR is set if PROGERR, SIZERR, PGAERR, WRPERR,
PGSERR, or OPTWERR is already set due to a previous programming error.

WRPERR: write protection error
–

•

Refer to Table 68 to Table 71 for all the conditions of WRPERR flag setting.

OPTWERR: option bytes write error
OPTWERR is set if when user option bytes are modified with an invalid configuration. It
is set when attempting:
–

to program an invalid secure watermark-based area. Refer to Table 59

–

to set or clear the TZEN option bit when RDP is not at correct level (refer to Rules
for modifying specific option bytes)

–

to clear the BOOT_LOCK option bit when RDP is not at correct level (refer to
Rules for modifying specific option bytes)

–

to modify SWAP_BANK option bit while BOOT_LOCK and TZEN are set

–

to modify SECBOOTADD0 option bit while BOOT_LOCK is set

–

to modify DUALBANK option bit while BOOT_LOCK and TZEN are set

–

to modify SECWM1Rx (resp. SECWM2Rx) while HDP1_ACCDIS
(resp. HDP2_ACCDIS) is set

–

to modify the option bytes, except the SWAP_BANK option bit, when RDP is set
to level 2

–

to regress from RDP level 0.5 to RDP level 0

–

to modify OEM1KEYRx while RDP level is 0.5 or 1 and OEM1LOCK bit is set

–

to modify OEM2KEYRx while RDP level is 1 and OEM2LOCK bit is set

–

to regress from RDP level 1 to RDP level 0 while OEM1LOCK bit is set and a
wrong OEM1KEY is shifted through JTAG or SWD

–

to regress from RDP level 1 to RDP level 0.5 while OEM2LOCK bit is set and
a wrong OEM2KEY is shifted through JTAG or SWD

–

to modify WRPxyR while its UNLOCK bit is cleared

–

to set the UNLOCK bit in the WRPxyR when RDP is not at correct level
(refer to Rules for modifying specific option bytes)

RM0456 Rev 6

<!-- pagebreak -->

