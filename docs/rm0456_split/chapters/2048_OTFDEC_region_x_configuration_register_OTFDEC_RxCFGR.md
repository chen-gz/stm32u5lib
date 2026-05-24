2060

On-the-fly decryption engine (OTFDEC)

52.6.2

RM0456

OTFDEC privileged access control configuration register
(OTFDEC_PRIVCFGR)
Address offset: 0x10
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

Res.

Res.

PRIV
rw

Bits 31:1 Reserved, must be kept at reset value.
Bit 0 PRIV: Privileged access protection.
0: No additional protection is added on OTFDEC register accesses.
1: An additional protection is added when accessing all registers except
OTFDEC_PRIVCFGR:
–
Unprivileged read accesses to registers return zeros
–
Unprivileged write accesses to registers are ignored.
Note: This bit can only be written in privileged mode. There is no limitations on reads.

52.6.3

OTFDEC region x configuration register (OTFDEC_RxCFGR)
Address offset: 0x20 + 0x30 * (x - 1), (x = 1 to 4)
Reset value: 0x0000 0000
Nonsecure AHB write access (HNONSEC = 1) is discarded if the TrustZone security is
enabled in the product.
Unprivileged reads return zero and unprivileged writes are ignored if PRIV bit is set in
OTFDEC_PRIVCFGR.
Writes are ignored if CONFIGLOCK bit is set to 1.

31

30

29

28

27

26

25

24

23

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

rw

rw

rw

6

5

4

3

2

1

REG_VERSION[15:0]

KEYCRC[7:0]
r

r

<!-- pagebreak -->

r

r

r

Res.
r

r

Res.

r

RM0456 Rev 6

MODE[1:0]
rw

rw

Res.

CONFI
KEYLOCK
GLOCK
rs

rs

0
REG_
EN
rw

RM0456

On-the-fly decryption engine (OTFDEC)

Bits 31:16 REG_VERSION[15:0]: region firmware version
This 16-bit bitfield must be correctly initialized before the region corresponding REG_EN bit is
set in OTFDEC_RxCFGR.
Bits 15:8 KEYCRC[7:0]: region key 8-bit CRC
When KEYLOCK = 0, KEYCRC bitfield is automatically computed by hardware while loading
the key of this region in this exact sequence: KEYR0 then KEYR1 then KEYR2 then finally
KEYR3 (all written once). A new computation starts as soon as a new valid sequence is
initiated, and KEYCRC is read as zero until a valid sequence is completed.
When KEYLOCK = 1, KEYCRC remains unchanged until the next reset.
CRC computation is an 8-bit checksum using the standard CRC-8-CCITT algorithm
X8 + X2 + X + 1 (according the convention). Source code is available in Section 52.5.4.
This field is read only.
Note: CRC information is updated only after the last bit of the key has been written.
Bits 7:6 Reserved, must be kept at reset value.
Bits 5:4 MODE[1:0]: operating mode
This bitfield selects the OTFDEC operating mode for this region:
10: All read accesses are decrypted (instruction or data).
11: Enhanced encryption mode is activated, and only instruction accesses are decrypted
Others: Reserved
When MODE ≠ 11, the standard AES encryption mode is activated.
When either of the MODE bits are changed, the region key and associated CRC are zeroed.
Bit 3 Reserved, must be kept at reset value.
Bit 2 KEYLOCK: region key lock
0: Writes to this region KEYRx registers are allowed.
1: Writes to this region KEYRx registers are ignored until next OTFDEC reset. KEYCRC
bitfield is locked.
Note: This bit is set once: if this bit is set, it can only be reset to 0 if the OTFDEC is reset.
Bit 1 CONFIGLOCK: region config lock
0: Writes to this region OTFDEC_RxCFGR, OTFDEC_RxSTARTADDR,
OTFDEC_RxENDADDR and OTFDEC_RxNONCERy registers are allowed.
1: Writes to this region OTFDEC_RxCFGR, OTFDEC_RxSTARTADDR,
OTFDEC_RxENDADDR and OTFDEC_RxNONCERy registers are ignored until next
OTFDEC reset.
Note: This bit is set once. If this bit is set, it can only be reset to 0 if OTFDEC is reset. Setting
this bit forces KEYLOCK bit to 1.
Bit 0 REG_EN: region on-the-fly decryption enable
0: On-the-fly decryption is disabled for this region.
1: On-the-fly decryption is enabled for this region. Data are XORed with the corresponding
keystream.
Note: Garbage is decrypted if region context (version, key, nonce) is not valid when this
bit is set.

RM0456 Rev 6

<!-- pagebreak -->

