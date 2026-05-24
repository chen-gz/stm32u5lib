1973

AES hardware accelerator (AES)

RM0456

Bits 31:0 KEY[63:32]: Cryptographic key, bits [63:32]
Refer to the AES_KEYR0 register for description of the KEY[255:0] bitfield.

49.7.7

AES key register 2 (AES_KEYR2)
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
Refer to the AES_KEYR0 register for description of the KEY[255:0] bitfield.

49.7.8

AES key register 3 (AES_KEYR3)
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
Refer to the AES_KEYR0 register for description of the KEY[255:0] bitfield.

49.7.9

AES initialization vector register 0 (AES_IVR0)
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

AES hardware accelerator (AES)

Bits 31:0 IVI[31:0]: Initialization vector input, bits [31:0]
Refer to Section 49.4.16: AES initialization vector registers on page 1955 for description of the
IVI[127:0] bitfield.
The initialization vector is only used in chaining modes other than ECB.
The AES_IVRx registers may be written only when the AES peripheral is disabled

49.7.10

AES initialization vector register 1 (AES_IVR1)
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
Refer to the AES_IVR0 register for description of the IVI[128:0] bitfield.

49.7.11

AES initialization vector register 2 (AES_IVR2)
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
Refer to the AES_IVR0 register for description of the IVI[128:0] bitfield.

49.7.12

AES initialization vector register 3 (AES_IVR3)
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

1973

AES hardware accelerator (AES)

RM0456

Bits 31:0 IVI[127:96]: Initialization vector input, bits [127:96]
Refer to the AES_IVR0 register for description of the IVI[128:0] bitfield.

49.7.13

AES key register 4 (AES_KEYR4)
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
Refer to the AES_KEYR0 register for description of the KEY[255:0] bitfield.

49.7.14

AES key register 5 (AES_KEYR5)
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
Refer to the AES_KEYR0 register for description of the KEY[255:0] bitfield.

49.7.15

AES key register 6 (AES_KEYR6)
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
Refer to the AES_KEYR0 register for description of the KEY[255:0] bitfield.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

AES hardware accelerator (AES)

49.7.16

AES key register 7 (AES_KEYR7)
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
Refer to the AES_KEYR0 register for description of the KEY[255:0] bitfield.

Note:

The key registers from 4 to 7 are used only when the key length of 256 bits is selected. They
have no effect when the key length of 128 bits is selected (only key registers 0 to 3 are used
in that case).

49.7.17

AES suspend registers (AES_SUSPxR)
Address offset: 0x040 + 0x4 * x, (x = 0 to 7)
Reset value: 0x0000 0000
These registers contain the complete internal register states of the AES processor when the
AES processing of the current task is suspended to process a higher-priority task.
Upon suspend, the software reads and saves the AES_SUSPxR register contents (where x
is from 0 to 7) into memory, before using the AES processor for the higher-priority task.
Upon completion, the software restores the saved contents back into the corresponding
suspend registers, before resuming the original task.

Note:

These registers are used only when GCM, GMAC, or CCM chaining mode is selected.
These registers can be read only when AES is enabled. Reading these registers while AES
is disabled returns 0x0000 0000.

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

SUSP[31:16]
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

SUSP[15:0]
rw

rw

rw

rw

rw

rw

rw

rw

rw

Bits 31:0 SUSP[31:0]: AES suspend
Upon suspend operation, this bitfield of the corresponding AES_SUSPxR register takes the value of
one of internal AES registers.

RM0456 Rev 6

<!-- pagebreak -->

1973

AES hardware accelerator (AES)

49.7.18

RM0456

AES interrupt enable register (AES_IER)
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

Res.

KEIE

RWEIE

CCFIE

Reset value: 0x0000 0000

rw

rw

rw

Bits 31:3 Reserved, must be kept at reset value.
Bit 2 KEIE: Key error interrupt enable
This bit enables or disables (masks) the AES interrupt generation when KEIF (key error flag) is set.
0: Disabled (masked)
1: Enabled (not masked)
Bit 1 RWEIE: Read or write error interrupt enable
This bit enables or disables (masks) the AES interrupt generation when RWEIF (read and/or write
error flag) is set.
0: Disabled (masked)
1: Enabled (not masked)
Bit 0 CCFIE: Computation complete flag interrupt enable
This bit enables or disables (masks) the AES interrupt generation when CCF (computation
complete flag) is set.
0: Disabled (masked)
1: Enabled (not masked)

49.7.19

AES interrupt status register (AES_ISR)
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

Res.

KEIF

RWEIF

CCF

Reset value: 0x0000 0000

r

r

r

Bits 31:3 Reserved, must be kept at reset value.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

AES hardware accelerator (AES)

Bit 2 KEIF: Key error interrupt flag
This read-only bit is set by hardware when key information failed to load into key registers.
0: No key error detected
1: Key information failed to load into key registers
Setting the corresponding bit of the AES_ICR register clears the KEIF and generates interrupt if the
KEIE bit of the AES_IER register is set.
KEIF is triggered upon any of the following errors:
–AES_KEYRx register write does not respect the correct order. (For KEYSIZE = 0, AES_KEYR0
then AES_KEYR1 then AES_KEYR2 then AES_KEYR3 register, or reverse. For KEYSIZE = 1,
AES_KEYR0 then AES_KEYR1 then AES_KEYR2 then AES_KEYR3 then AES_KEYR4 then
AES_KEYR5 then AES_KEYR6 then AES_KEYR7, or reverse).
KEIF must be cleared by the application software, otherwise KEYVALID cannot be set.
Bit 1 RWEIF: Read or write error interrupt flag
This read-only bit is set by hardware when a RDERR or a WRERR error flag is set in the AES_SR
register.
0: No read or write error detected
1: Read or write error detected (see AES_SR register for details)
RWEIF bit is cleared when application sets the corresponding bit of AES_ICR register. An interrupt
is generated if the RWEIE bit has been previously set in the AES_IER register.
This flags has no meaning when key derivation mode is selected.
Bit 0 CCF: Computation complete flag
This flag indicates whether the computation is completed:
0: Not completed
1: Completed
The flag is set by hardware upon the completion of the computation. It is cleared by software, upon
setting the CCF bit of the AES_ICR register.
Upon the flag setting, an interrupt is generated if enabled through the CCFIE bit of the AES_IER
register.
The flag is significant only when the DMAOUTEN bit is 0. It may stay high when DMA_EN is 1.

49.7.20

AES interrupt clear register (AES_ICR)
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

Res.

KEIF

RWEIF

CCF

Reset value: 0x0000 0000

w

w

w

RM0456 Rev 6

<!-- pagebreak -->

