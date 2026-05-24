363

Embedded flash memory (FLASH)

7.9.18

RM0456

FLASH secure watermark1 register 2 (FLASH_SECWM1R2)
Address offset: 0x54
Reset value: 0xXXXX XXXX
(bits loaded with values from the flash memory at OBL)
ST production value: 0x7FE0 7FE0 (for STM32U535/545)
0x7F80 7F80 (for STM32U575/585)
0x7F00 7F00 (for STM32U59x/5Ax/5Fx/5Gx)
Access: no wait state when no option bytes modification is ongoing; word, half-word, and
byte access
This register can not be written if OPTLOCK bit is set. This register is secure. It can be read
and written only by secure access. A nonsecure read/write access is RAZ/WI. This register
can be protected against unprivileged access when SPRIV = 1 in FLASH_PRIVCFGR.

31

30

29

28

27

26

25

24

HDP1E
N

Res.

Res.

Res.

Res.

Res.

Res.

Res.
rw

rw

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

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

rw

23

22

21

20

19

18

17

16

rw

rw

rw

rw

3

2

1

0

Res.

Res.

Res.

Res.

HDP1_PEND[7:0]

Bit 31 HDP1EN: Hide protection first area enable
0: No HDP area 1
1: HDP first area enabled
Bits 30:24 Reserved, must be kept at reset value.
Bits 23:16 HDP1_PEND[7:0]: End page of first hide protection area
This field contains the last page of the HDP area in bank 1. This field is limited to 7 bits for
STM32U575/585 and 5 bits for STM32U535/545.
Bits 15:0 Reserved, must be kept at reset value.

<!-- pagebreak -->

