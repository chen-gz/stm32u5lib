2015

Secure AES coprocessor (SAES)

RM0456

Bits 31:0 KEY[63:32]: Cryptographic key, bits [63:32]
Refer to the SAES_KEYR0 register for description of the KEY[255:0] bitfield.

50.7.7

SAES key register 2 (SAES_KEYR2)
Address offset: 0x18
Reset value: 0x0000 0000

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

KEY[95:80]
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

KEY[79:64]
w

w

Bits 31:0 KEY[95:64]: Cryptographic key, bits [95:64]
Refer to the SAES_KEYR0 register for description of the KEY[255:0] bitfield.

50.7.8

SAES key register 3 (SAES_KEYR3)
Address offset: 0x1C
Reset value: 0x0000 0000

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

w

w

w

w

w

w

w

w

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

KEY[127:112]
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

KEY[111:96]
w

w

w

w

w

w

w

w

w

Bits 31:0 KEY[127:96]: Cryptographic key, bits [127:96]
Refer to the SAES_KEYR0 register for description of the KEY[255:0] bitfield.

50.7.9

SAES initialization vector register 0 (SAES_IVR0)
Address offset: 0x20
Reset value: 0x0000 0000

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

IVI[31:16]
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

IVI[15:0]

<!-- pagebreak -->

rw

RM0456 Rev 6

RM0456

Secure AES coprocessor (SAES)

Bits 31:0 IVI[31:0]: Initialization vector input, bits [31:0]
Refer to Section 50.4.13: SAES initialization vector registers on page 1996 for description of the
IVI[127:0] bitfield.
The initialization vector is only used in chaining modes other than ECB.
The SAES_IVRx registers may be written only when the SAES peripheral is disabled

50.7.10

SAES initialization vector register 1 (SAES_IVR1)
Address offset: 0x24
Reset value: 0x0000 0000

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

IVI[63:48]
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

rw

rw

rw

rw

rw

rw

rw

IVI[47:32]
rw

rw

rw

rw

rw

rw

rw

rw

rw

Bits 31:0 IVI[63:32]: Initialization vector input, bits [63:32]
Refer to the SAES_IVR0 register for description of the IVI[128:0] bitfield.

50.7.11

SAES initialization vector register 2 (SAES_IVR2)
Address offset: 0x28
Reset value: 0x0000 0000

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

IVI[95:80]
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

IVI[79:64]
rw

Bits 31:0 IVI[95:64]: Initialization vector input, bits [95:64]
Refer to the SAES_IVR0 register for description of the IVI[128:0] bitfield.

50.7.12

SAES initialization vector register 3 (SAES_IVR3)
Address offset: 0x2C
Reset value: 0x0000 0000

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

IVI[127:112]
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

rw

rw

rw

rw

rw

rw

rw

rw

rw

8

7

6

5

4

3

2

1

0

rw

rw

rw

rw

rw

rw

rw

IVI[111:96]
rw

rw

rw

rw

rw

rw

rw

rw

rw

RM0456 Rev 6

<!-- pagebreak -->

2015

Secure AES coprocessor (SAES)

RM0456

Bits 31:0 IVI[127:96]: Initialization vector input, bits [127:96]
Refer to the SAES_IVR0 register for description of the IVI[128:0] bitfield.

50.7.13

SAES key register 4 (SAES_KEYR4)
Address offset: 0x30
Reset value: 0x0000 0000

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

KEY[159:144]
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

KEY[143:128]
w

w

w

w

w

w

w

w

w

Bits 31:0 KEY[159:128]: Cryptographic key, bits [159:128]
Refer to the SAES_KEYR0 register for description of the KEY[255:0] bitfield.

50.7.14

SAES key register 5 (SAES_KEYR5)
Address offset: 0x34
Reset value: 0x0000 0000

31

30

29

28

27

26

25

24

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

23

22

21

20

19

18

17

16

w

w

w

w

w

w

w

w

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

KEY[191:176]

KEY[175:160]
w

w

w

w

w

w

w

w

w

Bits 31:0 KEY[191:160]: Cryptographic key, bits [191:160]
Refer to the SAES_KEYR0 register for description of the KEY[255:0] bitfield.

50.7.15

SAES key register 6 (SAES_KEYR6)
Address offset: 0x38
Reset value: 0x0000 0000

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

KEY[223:208]
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

KEY[207:192]
w

w

Bits 31:0 KEY[223:192]: Cryptographic key, bits [223:192]
Refer to the SAES_KEYR0 register for description of the KEY[255:0] bitfield.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Secure AES coprocessor (SAES)

50.7.16

SAES key register 7 (SAES_KEYR7)
Address offset: 0x3C
Reset value: 0x0000 0000

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

KEY[255:240]
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

KEY[239:224]
w

w

Bits 31:0 KEY[255:224]: Cryptographic key, bits [255:224]
Refer to the SAES_KEYR0 register for description of the KEY[255:0] bitfield.

Note:

The key registers from 4 to 7 are used only when the key length of 256 bits is selected. They
have no effect when the key length of 128 bits is selected (only key registers 0 to 3 are used
in that case).

50.7.17

SAES interrupt enable register (SAES_IER)
Address offset: 0x300

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

RNGEIE

KEIE

RWEIE

CCFIE

Reset value: 0x0000 0000

rw

rw

rw

rw

Bits 31:4 Reserved, must be kept at reset value.
Bit 3 RNGEIE: RNG error interrupt enable
This bit enables or disables (masks) the SAES interrupt generation when RNGEIF (RNG error flag)
is set.
0: Disabled (masked)
1: Enabled (not masked)
Bit 2 KEIE: Key error interrupt enable
This bit enables or disables (masks) the SAES interrupt generation when KEIF (key error flag) is
set.
0: Disabled (masked)
1: Enabled (not masked)
Bit 1 RWEIE: Read or write error interrupt enable
This bit enables or disables (masks) the SAES interrupt generation when RWEIF (read and/or write
error flag) is set.
0: Disabled (masked)
1: Enabled (not masked)

RM0456 Rev 6

<!-- pagebreak -->

2015

Secure AES coprocessor (SAES)

RM0456

Bit 0 CCFIE: Computation complete flag interrupt enable
This bit enables or disables (masks) the SAES interrupt generation when CCF (computation
complete flag) is set.
0: Disabled (masked)
1: Enabled (not masked)

50.7.18

SAES interrupt status register (SAES_ISR)
Address offset: 0x304

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

RNGEIF

KEIF

RWEIF

CCF

Reset value: 0x0000 0000

r

r

r

r

Bits 31:4 Reserved, must be kept at reset value.
Bit 3 RNGEIF: RNG error interrupt flag
This read-only bit is set by hardware when an error is detected on RNG bus interface (e.g. bad
entropy).
0: RNG bus is functional
1: Error detected on RNG bus interface (random seed fetching error)
RNGEIE bit is cleared when application sets the corresponding bit of SAES_ICR register. An
interrupt is generated if the RNGEIE bit has been previously set in the SAES_IER register. Clearing
this bit triggers the reload of a new random number from RNG peripheral.
Bit 2 KEIF: Key error interrupt flag
This read-only bit is set by hardware when key information failed to load into key registers or key
register usage is forbidden.
0: No key error detected
1: Key information failed to load into key registers, or key register use is forbidden
Setting the corresponding bit of the SAES_ICR register clears the KEIF and generates interrupt if
the KEIE bit of the SAES_IER register is set.
KEIF is triggered upon any of the following errors:
–SAES fails to load the DHUK (KEYSEL = 001 or 100).
–SAES fails to load the BHK (KEYSEL = 010 or 100) respecting the correct order.
–AES fails to load the key shared by SAES peripheral (KMOD = 10).
– When KEYVALID = 1 and (KEYPROT = 1 or KEYSEL is not 0x0), the security context of the
application that loads the key (secure or nonsecure) does not match the security attribute of the
access to SAES_CR or SAES_DOUT. In this case, KEYVALID and EN bits are cleared.
–SAES_KEYRx register write does not respect the correct order. (For KEYSIZE = 0, SAES_KEYR0
then SAES_KEYR1 then SAES_KEYR2 then SAES_KEYR3 register, or reverse. For
KEYSIZE = 1, SAES_KEYR0 then SAES_KEYR1 then SAES_KEYR2 then SAES_KEYR3 then
SAES_KEYR4 then SAES_KEYR5 then SAES_KEYR6 then SAES_KEYR7, or reverse).
KEIF must be cleared by the application software, otherwise KEYVALID cannot be set.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Secure AES coprocessor (SAES)

Bit 1 RWEIF: Read or write error interrupt flag
This read-only bit is set by hardware when a RDERR or a WRERR error flag is set in the SAES_SR
register.
0: No read or write error detected
1: Read or write error detected (see SAES_SR register for details)
RWEIF bit is cleared when application sets the corresponding bit of SAES_ICR register. An interrupt
is generated if the RWEIE bit has been previously set in the SAES_IER register.
This flags has no meaning when key derivation mode is selected.
Bit 0 CCF: Computation complete flag
This flag indicates whether the computation is completed:
0: Not completed
1: Completed
The flag is set by hardware upon the completion of the computation. It is cleared by software, upon
setting the CCF bit of the SAES_ISR register.
Upon the flag setting, an interrupt is generated if enabled through the CCFIE bit of the SAES_IER
register.
The flag is significant only when the DMAOUTEN bit is 0. It may stay high when DMA_EN is 1.

50.7.19

SAES interrupt clear register (SAES_ICR)
Address offset: 0x308

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

RNGEIF

KEIF

RWEIF

CCF

Reset value: 0x0000 0000

w

w

w

w

Bits 31:4 Reserved, must be kept at reset value.
Bit 3 RNGEIF: RNG error interrupt flag clear
Application must set this bit to clear the RNGEIF status bit in SAES_ISR register.
Bit 2 KEIF: Key error interrupt flag clear
Setting this bit clears the KEIF status bit of the SAES_ISR register.
Bit 1 RWEIF: Read or write error interrupt flag clear
Setting this bit clears the RWEIF status bit of the SAES_ISR register, and both RDERR and
WRERR flags in the SAES_SR register.
Bit 0 CCF: Computation complete flag clear
Setting this bit clears the CCF status bit of the SAES_SR and SAES_ISR registers.

RM0456 Rev 6

<!-- pagebreak -->

