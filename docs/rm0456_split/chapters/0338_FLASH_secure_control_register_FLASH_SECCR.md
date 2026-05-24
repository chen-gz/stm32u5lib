363

Embedded flash memory (FLASH)

RM0456

Bits 10:3 PNB[7:0]: Nonsecure page number selection
These bits select the page to erase.
00000000: page 0
00000001: page 1
...
00011111: page 31 (upper page for STM32U535/545)
...
01111111: page 127 (upper page for STM32U575/585)
...
11111111: page 255 (upper page for STM32U59x/5Ax/5Fx/5Gx)
Bit 2 MER1: Nonsecure bank 1 mass erase
This bit triggers the bank 1 nonsecure mass erase (all bank 1 user pages) when set.
Bit 1 PER: Nonsecure page erase
0: Nonsecure page erase disabled
1: Nonsecure page erase enabled
Bit 0 PG: Nonsecure programming
0: Nonsecure FLASH programming disabled
1: Nonsecure FLASH programming enabled

7.9.10

FLASH secure control register (FLASH_SECCR)
Address offset: 0x2C
Reset value: 0x8000 0000
Access: no wait state when no flash memory operation is ongoing; word, half-word,
and byte access
This register can only be written when BSY or OBL_LAUNCH is reset. Otherwise, the write
access stalls until the BSY bits are reset.
This register is secure. It can be read and written only by secure access. A nonsecure
read/write access is RAZ/WI. This register can be protected against unprivileged access
when SPRIV = 1 in FLASH_PRIVCFGR register.

31

30

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

LOCK

Res.

INV

Res.

Res.

Res.

ERRIE

EOPIE

Res.

Res.

Res.

Res.

Res.

Res.

Res.

STRT

rw

rw

15

14

13

12

11

10

9

8

7

6

5

4

3

2

1

0

MER2

BWR

Res.

Res.

BKER

MER1

PER

PG

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rs

rw

rw

rs

PNB[7:0]
rw

Bit 31 LOCK: Secure lock
This bit is set only. When set, this register is locked. It is cleared by hardware after detecting
the unlock sequence in FLASH_SECKEYR register.
In case of an unsuccessful unlock operation, this bit remains set until the next system reset.
Bit 30 Reserved, must be kept at reset value.
Bit 29 INV: Flash memory security state invert
This bit inverts the flash memory security state.
Bits 28:26 Reserved, must be kept at reset value.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Embedded flash memory (FLASH)

Bit 25 ERRIE: Secure error interrupt enable
This bit enables the interrupt generation when OPERR = 1 in FLASH_SECSR.
0: Secure OPERR error interrupt disabled
1: Secure OPERR error interrupt enabled
Bit 24 EOPIE: Secure End of operation interrupt enable
This bit enables the interrupt generation when EOP = 1 in FLASH_SECSR.
0: Secure EOP Interrupt disabled
1: Secure EOP Interrupt enabled
Bits 23:17 Reserved, must be kept at reset value.
Bit 16 STRT: Secure start
This bit triggers a secure erase operation when set. If MER1, MER2,and PER bits are reset
and the STRT bit is set, PGSERR is set in FLASH_SECSR (this condition is forbidden).
This bit is set only by software and is cleared when BSY is cleared in FLASH_SECSR.
Bit 15 MER2: Secure bank 2 mass erase
This bit triggers the bank 2 secure mass erase (all bank 2 user pages) when set.
Bit 14 BWR: Secure burst write programming mode
When set, this bit selects the burst write programming mode.
Bits 13:12 Reserved, must be kept at reset value.
Bit 11 BKER: Secure bank selection for page erase
0: Bank 1 selected for secure page erase
1: Bank 2 selected for secure page erase
Bits 10:3 PNB[7:0]: Secure page number selection
These bits select the page to erase.
00000000: page 0
00000001: page 1
...
00011111: page 31 (upper page for STM32U535/545)
...
01111111: page 127 (upper page for STM32U575/585)
...
11111111: page 255 (upper page for STM32U59x/5Ax/5Fx/5Gx)
Bit 2 MER1: Secure bank 1 mass erase
This bit triggers the bank 1 secure mass erase (all bank 1 user pages) when set.
Bit 1 PER: Secure page erase
0: Secure page erase disabled
1: Secure page erase enabled
Bit 0 PG: Secure programming
0: Secure FLASH programming disabled
1: Secure FLASH programming enabled

RM0456 Rev 6

<!-- pagebreak -->

