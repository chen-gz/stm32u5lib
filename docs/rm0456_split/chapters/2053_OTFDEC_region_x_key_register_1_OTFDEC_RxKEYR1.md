RM0456 Rev 6

RM0456

On-the-fly decryption engine (OTFDEC)

52.6.9

OTFDEC region x key register 1 (OTFDEC_RxKEYR1)
Address offset: 0x38 + 0x30 * (x - 1), (x = 1 to 4)
Reset value: 0x0000 0000
Nonsecure AHB write access (HNONSEC = 1) is discarded if the TrustZone security is
enabled in the product.
Unprivileged writes are ignored if PRIV bit is set in OTFDEC_PRIVCFGR.

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

REG_KEY[63:48]
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

REG_KEY[47:32]
w

w

w

w

w

w

w

w

w

Bits 31:0 REG_KEY[63:32]: Region key, bits [63:32]
Refer to the OTFDEC_RxKEYR0 register for description of the KEY[127:0] bitfield.

52.6.10

OTFDEC region x key register 2 (OTFDEC_RxKEYR2)
Address offset: 0x3C + 0x30 * (x - 1), (x = 1 to 4)
Reset value: 0x0000 0000
Nonsecure AHB write access (HNONSEC = 1) is discarded if the TrustZone security is
enabled in the product.
Unprivileged writes are ignored if PRIV bit is set in OTFDEC_PRIVCFGR.

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

REG_KEY[95:80]
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

REG_KEY[79:64]
w

w

w

w

w

w

w

w

w

Bits 31:0 REG_KEY[95:64]: Region key, bits [95:64]
Refer to the OTFDEC_RxKEYR0 register for description of the KEY[127:0] bitfield.

RM0456 Rev 6

<!-- pagebreak -->

2060

On-the-fly decryption engine (OTFDEC)

52.6.11

RM0456

OTFDEC region x key register 3 (OTFDEC_RxKEYR3)
Address offset: 0x40 + 0x30 * (x - 1), (x = 1 to 4)
Reset value: 0x0000 0000
Nonsecure AHB write access (HNONSEC = 1) is discarded if the TrustZone security is
enabled in the product.
Unprivileged writes are ignored if PRIV bit is set in OTFDEC_PRIVCFGR.

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

REG_KEY[127:112]
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

REG_KEY[111:96]
w

w

w

w

w

w

w

w

w

Bits 31:0 REG_KEY[127:96]: Region key, bits [127:96]
Refer to the OTFDEC_RxKEYR0 register for description of the KEY[127:0] bitfield.

52.6.12

OTFDEC interrupt status register (OTFDEC_ISR)
Address offset: 0x300
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

XONEIF

SEIF

Unprivileged reads return zero if PRIV bit is set in OTFDEC_PRIVCFGR.

r

r

r

Bits 31:3 Reserved, must be kept at reset value.
Bit 2 KEIF: Key error interrupt flag status
This bit is set by hardware and read only by application. The bit is set when a read access
occurs on an encrypted region, while its key registers is null or not properly initialized
(KEYCRC = 0x0).
This bit is cleared when the application sets in OTFDEC_ICR the corresponding bit to 1.
0: OTFDEC operates properly.
1: Read access detected on an enabled encrypted region with its key registers null or not
properly initialized (KEYCRC = 0x0). OTFDEC returns a zeroed value for the read, and an
optional interrupt is generated if bit KEIE is set to 1 in OTFDEC_IER.
After KEIF is set any subsequent read to the region with bad key registers returns a zeroed
value. This state remains until those key registers are properly initialized (KEYCRC not
zero).

<!-- pagebreak -->

RM0456 Rev 6

RM0456

On-the-fly decryption engine (OTFDEC)

Bit 1 XONEIF: Execute-only execute-never error interrupt flag status
This bit is set by hardware and read only by application. This bit is set when a read access
and not an instruction fetch is detected on any encrypted region with MODE bits set to 11.
Lastly, XONEIF is also set when an execute access is detected while encryption mode is
enabled.
This bit is cleared when application sets in OTFDEC_ICR the corresponding bit to 1.
0: No execute-only error status. No interrupt pending.
1: Read access detected on one region with MODE bits set to 11 or execute access detected
while ENC = 1. OTFDEC returns a zeroed value for the illegal access, and an optional
interrupt is generated if bit XONEIE is set to 1 in OTFDEC_IER.
Bit 0 SEIF: Security error interrupt flag status
This bit is set by hardware and read only by application. This bit is set when at least one
security error has been detected.
This bit is cleared when application sets in OTFDEC_ICR the corresponding bit to 1.
0: No security error status. No interrupt pending.
1: Security error flag status, with interrupt pending. Actual interrupt generation is dependent
on OTFDEC_IER corresponding bit SEIE.

52.6.13

OTFDEC interrupt clear register (OTFDEC_ICR)
Address offset: 0x304
Reset value: 0x0000 0000
Nonsecure AHB write access (HNONSEC = 1) is discarded if the TrustZone security is
enabled in the product.

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

XONEIF

SEIF

Unprivileged writes are ignored if PRIV bit is set in OTFDEC_PRIVCFGR.

w

w

w

Bits 31:3 Reserved, must be kept at reset value.

RM0456 Rev 6

<!-- pagebreak -->

2060

On-the-fly decryption engine (OTFDEC)

RM0456

Bit 2 KEIF: Key error interrupt flag clear
This bit is written by application, and always read as 0.
0: KEIF flag status is not affected.
1: KEIF flag status is cleared in OTFDEC_ISR.
Note: Clearing KEIF does not solve the source of the problem (bad key registers). To be able
to access again any encrypted region, OTFDEC key registers must be properly
initialized again.
Bit 1 XONEIF: Execute-only execute-never error interrupt flag clear
This bit is written by application, and always read as 0.
0: XONEIF flag status is not affected.
1: XONEIF flag status is cleared in OTFDEC_ISR.
Bit 0 SEIF: Security error interrupt flag clear
This bit is written by application, and always read as 0.
0: SEIF flag status is not affected.
1: SEIF flag status is cleared in OTFDEC_ISR.

52.6.14

OTFDEC interrupt enable register (OTFDEC_IER)
Address offset: 0x308
Reset value: 0x0000 0000
Nonsecure AHB write access (HNONSEC = 1) is discarded if the TrustZone security is
enabled in the product.

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

XONEIE

SEIE

Unprivileged reads return zero and unprivileged writes are ignored if PRIV bit is set in
OTFDEC_PRIVCFGR.

rw

rw

rw

Bits 31:3 Reserved, must be kept at reset value.

<!-- pagebreak -->

