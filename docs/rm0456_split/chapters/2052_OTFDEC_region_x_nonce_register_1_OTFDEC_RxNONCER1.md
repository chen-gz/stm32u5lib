2060

On-the-fly decryption engine (OTFDEC)

52.6.7

RM0456

OTFDEC region x nonce register 1 (OTFDEC_RxNONCER1)
Address offset: 0x30 + 0x30 * (x - 1), (x = 1 to 4)
Reset value: 0x0000 0000
Nonsecure AHB write access (HNONSEC = 1) is discarded if the TrustZone security is
enabled in the product.
Unprivileged reads return zero and unprivileged writes are ignored if PRIV bit is set in
OTFDEC_PRIVCFGR.

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

REG_NONCE[63:48]
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

REG_NONCE[47:32]
rw

rw

rw

rw

rw

rw

rw

rw

rw

Bits 31:0 REG_NONCE[63:32]: Region nonce, bits [63:32]
Refer to the OTFDEC_RxNONCER0 register for description of the NONCE[63:0] bitfield.

52.6.8

OTFDEC region x key register 0 (OTFDEC_RxKEYR0)
Address offset: 0x34 + 0x30 * (x - 1), (x = 1 to 4)
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

REG_KEY[31:16]
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

REG_KEY[15:0]
w

w

w

w

w

w

w

w

w

Bits 31:0 REG_KEY[31:0]: Region key, bits [31:0]
This register must be written before the region corresponding REG_EN bit in
OTFDEC_RxCFGR is set.
Reading this register returns a zero value. Writing to this register is discarded if performed while the
region CONFIGLOCK or KEYLOCK bit is set in the OTFDEC_RxCFGR.
Note: When application successfully changes MODE bits in OTFDEC_RxCFGR and
OTFDEC_RxKEYR, and associated KEYCRC are erased.

<!-- pagebreak -->

