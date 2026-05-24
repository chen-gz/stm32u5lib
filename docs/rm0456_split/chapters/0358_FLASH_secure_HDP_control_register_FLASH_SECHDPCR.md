363

Embedded flash memory (FLASH)

7.9.31

RM0456

FLASH secure HDP control register (FLASH_SECHDPCR)
Address offset: 0xC0
Reset value: 0x0000 0000
Access: no wait state; word, half-word, and byte access
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

Res.

Res.

Res.

Res.

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

Res.

Res.

HDP2_ HDP1_
ACCDI ACCDI
S
S
rs

rs

Bits 31:2 Reserved, must be kept at reset value.
Bit 1 HDP2_ACCDIS: HDP2 area access disable
When set, this bit is only cleared by a system reset.
0: Access to HDP2 area granted
1: Access to HDP2 area denied (SECWM2Ry option-byte modification blocked, see Rules
for modifying specific option bytes)
Bit 0 HDP1_ACCDIS: HDP1 area access disable
When set, this bit is only cleared by a system reset.
0: Access to HDP1 area granted
1: Access to HDP1 area denied (SECWM1Ry option-byte modification blocked, see Rules
for modifying specific option bytes)

7.9.32

FLASH privilege configuration register (FLASH_PRIVCFGR)
Address offset: 0xC4.
Reset value: 0x0000 0000
Access: no wait state; word, half-word, and byte access
This register can be read by both privileged and unprivileged access. NSPRIV is a
non-secure bit. SPRIV is a secure bit.

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

Res.

Res.

Res.

Res.

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

Res.

NSPRI
V

SPRIV

rw

rw

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Bits 31:2 Reserved, must be kept at reset value.

<!-- pagebreak -->

RM0456 Rev 6

Res.

Res.

Res.

Res.

RM0456

Embedded flash memory (FLASH)

Bit 1 NSPRIV: Privileged protection for nonsecure registers
This bit can be read by both privileged or unprivileged, secure and nonsecure access.
0: Nonsecure FLASH registers can be read and written by privileged or unprivileged access.
1: Nonsecure FLASH registers can be read and written by privileged access only.
The NSPRIV bit can be written by a secure or nonsecure privileged access. A secure or
nonsecure unprivileged write access on NSPRIV bit is ignored.
Bit 0 SPRIV: Privileged protection for secure registers
This bit can be accessed only when TrustZone is enabled (TZEN = 1). This bit can be read
by both privileged or unprivileged, secure and nonsecure access.
0: Secure FLASH registers can be read and written by privileged or unprivileged access.
1: Secure FLASH registers can be read and written by privileged access only.
The SPRIV bit can be written only by a secure privileged access. A nonsecure write access
on SPRIV bit is ignored. A secure unprivileged write access on SPRIV bit is ignored.

7.9.33

FLASH privilege block based bank 1 register x
(FLASH_PRIVBB1Rx)
Address offset: 0xD0 + 0x4 * (x - 1), (x = 1 to 8)
Reset value: 0x0000 0000
Access: no wait state; word, half-word, and byte access
This register is privileged. It can be read written only by a privileged access. This register
can be protected against nonsecure access (refer to Table 76).

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

PRIV1
BB31

PRIV1
BB30

PRIV1
BB29

PRIV1
BB28

PRIV1
BB27

PRIV1
BB26

PRIV1
BB25

PRIV1
BB24

PRIV1
BB23

PRIV1
BB22

PRIV1
BB21

PRIV1
BB20

PRIV1
BB19

PRIV1
BB18

PRIV1
BB17

PRIV1
BB16

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

3

2

1

0

PRIV1
BB15

PRIV1
BB14

PRIV1
BB13

PRIV1
BB12

PRIV1
BB11

PRIV1
BB10

PRIV1
BB9

PRIV1
BB8

PRIV1
BB7

PRIV1
BB6

PRIV1
BB5

PRIV1
BB4

PRIV1
BB3

PRIV1
BB2

PRIV1
BB1

PRIV1
BB0

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

rw

rw

rw

rw

Bits 31:0 PRIV1BBi: page privileged/unprivileged attribution (i = 31 to 0)
Each bit is used to set one page privilege attribution in bank 1.
0: Page (32 * (x - 1) + i) in bank 1 accessible by unprivileged access
1: Page (32 * (x - 1) + i) in bank 1 only accessible by privileged access

RM0456 Rev 6

<!-- pagebreak -->

