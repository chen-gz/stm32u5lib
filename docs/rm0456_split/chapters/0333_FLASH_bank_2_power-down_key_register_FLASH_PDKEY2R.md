RM0456 Rev 6

RM0456

Embedded flash memory (FLASH)

7.9.6

FLASH bank 2 power-down key register (FLASH_PDKEY2R)
Address offset: 0x1C
Reset value: 0x0000 0000
Access: no wait state; word access
This register is nonsecure. It can be read and written by both secure and nonsecure access.
This register can be protected against unprivileged access when NSPRIV = 1
in FLASH_PRIVCFGR register.

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

PDKEY2[31:16]
w

w

w

w

w

w

w

w

w

w

w

w

w

w

w

w

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

w

w

w

w

w

w

w

w

w

w

w

w

w

w

PDKEY2[15:0]
w

w

Bits 31:0 PDKEY2[31:0]: Bank 2 power-down key
The following values must be written consecutively to unlock PDREQ2 bit in FLASH_ACR:
PDKEY2_1: 0x4051 6273
PDKEY2_2: 0xAFBF CFDF

7.9.7

FLASH nonsecure status register (FLASH_NSSR)
Address offset: 0x20
Reset value: 0x000X 0000
Access: no wait state; word, half-word and byte access
This register is nonsecure. It can be read and written by both secure and nonsecure access.
This register can be protected against unprivileged access when NSPRIV = 1
in FLASH_PRIVCFGR register.

31
Res.

15
Res.

30
Res.

29
Res.

14

13

Res.

OPTW
ERR
rc_w1

28
Res.

12
Res.

27
Res.

11
Res.

26
Res.

10
Res.

25
Res.

9
Res.

24
Res.

23
Res.

8
Res.

7

22
Res.

6

21

20

PD2

PD1

r

r

5

4

19

WDW

BSY

r

r

3

2

1

0

Res.

OPER
R

EOP

rc_w1

rc_w1

rc_w1

rc_w1

rc_w1

16

r

PROG
ERR

rc_w1

17

r

PGSER SIZER PGAER WRPE
R
R
R
RR
rc_w1

18

OEM2L OEM1L
OCK
OCK

Bits 31:22 Reserved, must be kept at reset value.
Bit 21 PD2: Bank 2 in power-down mode
This bit indicates that the flash memory bank 2 is in power-down state. It is reset when
bank 2 is in normal mode or being awaken.
Bit 20 PD1: Bank 1 in power-down mode
This bit indicates that the flash memory bank 1 is in power-down state. It is reset when
bank 1 is in normal mode or being awaken.

RM0456 Rev 6

<!-- pagebreak -->

363

Embedded flash memory (FLASH)

RM0456

Bit 19 OEM2LOCK: OEM2 lock
This bit indicates that the OEM2 RDP key read during the OBL is not virgin. When set,
the OEM2 RDP lock mechanism is active.
Bit 18 OEM1LOCK: OEM1 lock
This bit indicates that the OEM1 RDP key read during the OBL is not virgin. When set,
the OEM1 RDP lock mechanism is active.
Bit 17 WDW: Nonsecure wait data to write
This bit indicates that the flash memory write buffer has been written by a secure or
non-secure operation. It is set when the first data is stored in the buffer and cleared when
the write is performed in the flash memory.
Bit 16 BSY: Nonsecure busy
This indicates that a flash memory secure or nonsecure operation is in progress. This bit is
set at the beginning of a flash operation and reset when the operation finishes or when
an error occurs.
Bits 15:14 Reserved, must be kept at reset value.
Bit 13 OPTWERR: Option write error
This bit is set by hardware when the options bytes are written with an invalid configuration.
It is cleared by writing 1.
Refer to Section 7.3.9 for full conditions of error flag setting.
Bits 12:8 Reserved, must be kept at reset value.
Bit 7 PGSERR: Nonsecure programming sequence error
This bit is set by hardware when programming sequence is not correct. It is cleared
by writing 1. Refer to Section 7.3.9 for full conditions of error flag setting.
Bit 6 SIZERR: Nonsecure size error
This bit is set by hardware when the size of the access is a byte or half-word during a
nonsecure program sequence. Only quad-word programming is allowed by means of
successive word accesses. This bit is cleared by writing 1.
Bit 5 PGAERR: Nonsecure programming alignment error
This bit is set by hardware when the first word to be programmed is not aligned with
a quad-word address, or the second, third or forth word does not belong to the same
quad-word address. This bit is cleared by writing 1.
Bit 4 WRPERR: Nonsecure write protection error
This bit is set by hardware when a nonsecure address to be erased/programmed belongs to
a write-protected part (by WRP, HDP or RDP level 1) of the flash memory. This bit is cleared
by writing 1. Refer to Section 7.3.9 for full conditions of error flag setting.
Bit 3 PROGERR: Nonsecure programming error
This bit is set by hardware when a nonsecure quad-word address to be programmed
contains a value different from all 1 before programming, except if the data to write is all 0.
This bit is cleared by writing 1.
Bit 2 Reserved, must be kept at reset value.
Bit 1 OPERR: Nonsecure operation error
This bit is set by hardware when a flash memory nonsecure operation (program/erase)
completes unsuccessfully. This bit is set only if nonsecure error interrupts are enabled
(NSERRIE = 1). This bit is cleared by writing 1.

<!-- pagebreak -->

