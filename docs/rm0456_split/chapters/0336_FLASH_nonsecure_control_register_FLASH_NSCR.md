363

Embedded flash memory (FLASH)

RM0456

Bit 4 WRPERR: Secure write protection error
This bit is set by hardware when a secure address to be erased/programmed belongs to
a write-protected part (by WRP, HDP or RDP level 1) of the flash memory.This bit is cleared
by writing 1. Refer to Section 7.3.9 for full conditions of error flag setting.
Bit 3 PROGERR: Secure programming error
This bit is set by hardware when a secure quad-word address to be programmed contains
a value different from all 1 before programming, except if the data to write is all 0. This bit is
cleared by writing 1.
Bit 2 Reserved, must be kept at reset value.
Bit 1 OPERR: Secure operation error
This bit is set by hardware when a flash memory secure operation (program/erase)
completes unsuccessfully. This bit is set only if secure error interrupts are enabled
(SECERRIE = 1). This bit is cleared by writing 1.
Bit 0 EOP: Secure end of operation
This bit is set by hardware when one or more flash memory secure operation
(program/erase) has been completed successfully. This bit is set only if the secure end of
operation interrupts are enabled (EOPIE = 1 in FLASH_SECCR). This bit is cleared
by writing 1.

7.9.9

FLASH nonsecure control register (FLASH_NSCR)
Address offset: 0x28
Reset value: 0xC000 0000
Access: no wait state when no flash memory operation is ongoing; word, half-word,
and byte access
This register can only be written when BSY or OBL_LAUNCH is reset. Otherwise, the write
access is stalled until BSY bits are reset.
This register is nonsecure. It can be read and written by both secure and nonsecure access.
This register can be protected against unprivileged access when NSPRIV = 1
in FLASH_PRIVCFGR register.

31
LOCK

30
OPTLO
CK

29

28

27

26

25

24

23

22

21

20

19

18

17

16

Res.

Res.

OBL_L
AUNC
H

Res.

ERRIE

EOPIE

Res.

Res.

Res.

Res.

Res.

Res.

OPTST
RT

STRT

rs

rs

6

5

4

3

2

1

0

MER1

PER

PG

rw

rw

rw

rw

rw

rw

rs

rs

15

14

13

12

rc_w1
11

MER2

BWR

Res.

Res.

BKER

rw

rw

rw

rw

rw

10

9

8

7

rw

rw

rw

rw

PNB[7:0]
rw

Bit 31 LOCK: Nonsecure lock
This bit is set only. When set, the FLASH_NSCR register is locked. It is cleared by hardware
after detecting the unlock sequence in FLASH_NSKEYR register.
In case of an unsuccessful unlock operation, this bit remains set until the next system reset.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Embedded flash memory (FLASH)

Bit 30 OPTLOCK: Option lock
This bit is set only. When set, all bits concerning user options in FLASH_NSCR register are
locked. This bit is cleared by hardware after detecting the unlock sequence. LOCK bit
in FLASH_NSCR must be cleared before doing the unlock sequence for OPTLOCK bit.
In case of an unsuccessful unlock operation, this bit remains set until the next reset.
Bits 29:28 Reserved, must be kept at reset value.
Bit 27 OBL_LAUNCH: Force the option-byte loading
When set to 1, this bit forces the option byte reloading. This bit is cleared only when the
option-byte loading is complete. It cannot be written if OPTLOCK is set.
0: Option-byte loading complete
1: Option-byte loading requested
Bit 26 Reserved, must be kept at reset value.
Bit 25 ERRIE: Nonsecure error interrupt enable
This bit enables the interrupt generation when OPERR = 1 in FLASH_NSSR .
0: Nonsecure OPERR error interrupt disabled
1: Nonsecure OPERR error interrupt enabled
Bit 24 EOPIE: Nonsecure end of operation interrupt enable
This bit enables the interrupt generation when EOP = 1 in FLASH_NSSR.
0: Nonsecure EOP Interrupt disabled
1: Nonsecure EOP Interrupt enabled
Bits 23:18 Reserved, must be kept at reset value.
Bit 17 OPTSTRT: Options modification start
This bit triggers an options operation when set. It can not be written if OPTLOCK bit is set.
This bit is set only by software, and is cleared when the BSY bit is cleared in FLASH_NSSR.
Bit 16 STRT: Nonsecure start
This bit triggers a nonsecure erase operation when set. If MER1, MER2, and PER bits are
reset and the STRT bit is set, PGSERR is set in FLASH_NSSR (this condition is forbidden).
This bit is set only by software and is cleared when BSY is cleared in FLASH_NSSR.
Bit 15 MER2: Nonsecure bank 2 mass erase
This bit triggers the bank 2 nonsecure mass erase (all bank 2 user pages) when set.
Bit 14 BWR: Nonsecure burst write programming mode
When set, this bit selects the burst write programming mode.
Bits 13:12 Reserved, must be kept at reset value.
Bit 11 BKER: Nonsecure bank selection for page erase
0: Bank 1 selected for nonsecure page erase
1: Bank 2 selected for nonsecure page erase

RM0456 Rev 6

<!-- pagebreak -->

