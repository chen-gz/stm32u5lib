RM0456 Rev 6

RM0456

On-the-fly decryption engine (OTFDEC)

Bit 2 KEIE: Key error interrupt enable
This bit is read and written by application. It controls the OTFDEC interrupt generation when
KEIF flag status is set.
0: Interrupt generation on key error flag KEIF is disabled (masked).
1: Interrupt generation on key error flag KEIF is enabled (not masked).
Bit 1 XONEIE: Execute-only execute-never error interrupt enable
This bit is read and written by application. It controls the OTFDEC interrupt generation when
XONEIF flag status is set.
0: Interrupt generation on execute-only error XONEIF is disabled (masked).
1: Interrupt generation on execute-only error XONEIF is enabled (not masked).
Bit 0 SEIE: Security error interrupt enable
This bit is read and written by application. It controls the OTFDEC interrupt generation when
SEIF flag status is set.
0: Interrupt generation on security error SEIF is disabled (masked).
1: Interrupt generation on security error SEIF is enabled (not masked).

52.6.15

OTFDEC register map

Res.

Reset value

Reset value

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

REG1_START_ADDR[31:0]
0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

REG1_END_ADDR[31:0]
0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

REG1_NONCE[31:0]
0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

REG1_NONCE[63:32]
0

0

0

0

0

0

0

0

0

0

0

0

0

0

OTFDEC_
R1KEYR0
Reset value

REG_EN

0

CONFIGLOCK

0

OTFDEC_
R1NONCER1
Reset value

0x34

0

OTFDEC_
R1NONCER0
Reset value

0x30

0

OTFDEC_
R1ENDADDR
Reset value

0x2C

0

KEYCRC[7:0]

OTFDEC_
R1STARTADDR
Reset value

0x28

REG1_VERSION[15:0]

KEYLOCK

OTFDEC_
R1CFGR1

Reset value
0x24

Reserved
MODE[1:0]

Reserved

Res.

0x20

0

Res.

0x140x1C

PRIV

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

OTFDEC_
PRIVCFGR

Reserved
Res.

Reserved
Res.

0x10

0

Res.

0x040x0C

ENC

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

OTFDEC_CR

Res.

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

0x00

Register
name

Res.

Offset

Res.

Table 492. OTFDEC register map and reset values

0

0

0

0

0

REG1_KEY[31:0]
0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

RM0456 Rev 6

0

0

0

0

<!-- pagebreak -->

2060

On-the-fly decryption engine (OTFDEC)

RM0456

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

REG_EN

0

KEYLOCK

0

CONFIGLOCK

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

KEYCRC[7:0]

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

REG2_START_ADDR[31:0]
0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

REG2_END_ADDR[31:0]
0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

REG2_NONCE[31:0]
0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

REG2_NONCE[63:32]
0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

REG2_KEY[31:0]
0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

REG2_KEY[63:32]
0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

REG2_KEY[95:64]
0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

REG2_KEY[127:96]
0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

REG3_VERSION[15:0]

0

0

0

0

0

0

0

0

0

0

0

KEYCRC[7:0]

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

REG_EN

Reserved

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

REG3_START_ADDR[31:0]
0

0

0

0

0

0

0

0

0

0

0

0

OTFDEC_
R3ENDADDR

<!-- pagebreak -->

0

REG2_VERSION[15:0]

OTFDEC_
R3STARTADDR

Reset value

0

Reserved

OTFDEC_
R3CFGR

Reset value

0

0

Reserved

Reset value

0

REG1_KEY[127:96]

OTFDEC_
R2KEYR3
Reset value

0x88

0

OTFDEC_
R2KEYR2

0x74 0x7C

0x84

0

OTFDEC_
R2KEYR1

Reset value

0x80

0

OTFDEC_
R2KEYR0

Reset value

0x70

0

OTFDEC_
R2NONCER1

Reset value

0x6C

0

OTFDEC_
R2NONCER0

Reset value

0x68

0

OTFDEC_
R2ENDADDR

Reset value

0x64

0

OTFDEC_
R2STARTADDR

Reset value

0x60

0

REG1_KEY[95:64]

OTFDEC_
R2CFGR

Reset value

0x5C

0

Reserved

Reset value

0x58

0

KEYLOCK

Reset value

0x54

0

OTFDEC_
R1KEYR3

0x44 0x4C

0x50

0

CONFIGLOCK

Reset value
0x40

0

MODE[1:0]

0x3C

0

OTFDEC_
R1KEYR2

MODE[1:0]

Reset value

Res.

REG1_KEY[63:32]

Res.

OTFDEC_
R1KEYR1

Res.

0x38

Register
name

Res.

Offset

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

Table 492. OTFDEC register map and reset values (continued)

0

0

0

0

0

0

0

0

REG3_END_ADDR[31:0]
0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

RM0456 Rev 6

0

0

0

0

0

RM0456

On-the-fly decryption engine (OTFDEC)

OTFDEC_
R3NONCER0

REG3_NONCE[31:0]

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

REG3_KEY[63:32]
0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

REG3_KEY[95:64]
0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

REG3_KEY[127:96]
0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

Reserved

REG4_VERSION[15:0]

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

KEYCRC[7:0]

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

REG4_START_ADDR[31:0]
0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

REG4_END_ADDR[31:0]
0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

REG4_NONCE[31:0]
0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

REG4_NONCE[63:32]
0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

REG4_KEY[31:0]
0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

REG4_KEY[63:32]
0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

REG4_KEY[95:64]
0

0

0

0

0

0

0

0

0

0

0

0

0

OTFDEC_
R4KEYR3

Reserved

0

REG3_KEY[31:0]

OTFDEC_
R4KEYR2

Reset value
0x0D40x2FC

0

OTFDEC_
R4KEYR1

Reset value
0xD0

0

OTFDEC_
R4KEYR0

Reset value
0xCC

0

OTFDEC_
R4NONCER1

Reset value
0xC8

0

OTFDEC_
R4NONCER0

Reset value
0xC4

0

OTFDEC_
R4ENDADDR

Reset value
0xC0

0

OTFDEC_
R4STARTADDR

Reset value
0xBC

0

REG3_NONCE[63:32]

OTFDEC_
R4CFGR

Reset value
0xB8

0

Reserved

Reset value
0xB4

0

REG_EN

0xB0

0

OTFDEC_
R3KEYR3
Reset value

0xA4 0xAC

0

OTFDEC_
R3KEYR2
Reset value

0xA0

0

OTFDEC_
R3KEYR1
Reset value

0x9C

0

OTFDEC_
R3KEYR0
Reset value

0x98

0

KEYLOCK

Reset value
0x94

0

CONFIGLOCK

0x90

0

OTFDEC_
R3NONCER1

MODE[1:0]

Reset value

Res.

0x8C

Register
name

Res.

Offset

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

Table 492. OTFDEC register map and reset values (continued)

0

0

0

0

0

0

REG4_KEY[127:96]
0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

Reserved

RM0456 Rev 6

<!-- pagebreak -->

2060

0x308
OTFDEC_IER

<!-- pagebreak -->

