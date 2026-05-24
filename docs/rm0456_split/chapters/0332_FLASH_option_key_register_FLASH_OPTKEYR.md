363

Embedded flash memory (FLASH)

7.9.4

RM0456

FLASH option key register (FLASH_OPTKEYR)
Address offset: 0x10
Reset value: 0x0000 0000
Access: one wait state; word access
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

OPTKEY[31:16]
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

OPTKEY[15:0]
w

w

Bits 31:0 OPTKEY[31:0]: Option-byte key
The following values must be written consecutively to unlock the FLASH_OPTR register
allowing option byte programming/erasing operations:
KEY1: 0x0819 2A3B
KEY2: 0x4C5D 6E7F

7.9.5

FLASH bank 1 power-down key register (FLASH_PDKEY1R)
Address offset: 0x18
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

PDKEY1[31:16]
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

PDKEY1[15:0]
w

w

Bits 31:0 PDKEY1[31:0]: Bank 1 power-down key
The following values must be written consecutively to unlock PDREQ1 bit in FLASH_ACR:
PDKEY1_1: 0x0415 2637
PDKEY1_2: 0xFAFB FCFD

<!-- pagebreak -->

