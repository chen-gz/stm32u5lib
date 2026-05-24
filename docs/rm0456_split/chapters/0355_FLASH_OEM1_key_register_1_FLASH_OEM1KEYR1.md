RM0456 Rev 6

RM0456

Embedded flash memory (FLASH)

7.9.25

FLASH OEM1 key register 1 (FLASH_OEM1KEYR1)
Address offset: 0x70
Reset value: 0x0000 0000
Access: no wait state when no option bytes modification is ongoing; word, half-word, and
byte access
This register is nonsecure. It can be written by both secure and nonsecure access.
This register is read as zero. It can be protected against unprivileged access when
NSPRIV = 1 in FLASH_PRIVCFGR register.

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

OEM1KEY[31:16]
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

OEM1KEY[15:0]
w

w

Bits 31:0 OEM1KEY[31:0]: least significant bytes of the OEM1 key

7.9.26

FLASH OEM1 key register 2 (FLASH_OEM1KEYR2)
Address offset: 0x74
Reset value: 0x0000 0000
Access: no wait state when no option bytes modification is ongoing; word, half-word, and
byte access
This register is nonsecure. It can be written by both secure and nonsecure access.
This register is read as zero. It can be protected against unprivileged access when
NSPRIV = 1 in FLASH_PRIVCFGR register.

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

OEM1KEY[63:48]
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

OEM1KEY[47:32]
w

w

Bits 31:0 OEM1KEY[63:32]: most significant bytes of the OEM1key

RM0456 Rev 6

<!-- pagebreak -->

