2015

Secure AES coprocessor (SAES)

50.7.20

RM0456

SAES register map

0x008

0x00C

0x010

0x014

EN

DATATYPE[1:0]

MODE[1:0]

0
CCF

0
RDERR

0
WRERR

0

Res.

Res.

0

BUSY

Res.

KEYVALID

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

KMOD[1:0]
Res.

Res.
0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

DOUT[31:0]
0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

KEY[31:0]
0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

KEY[63:32]
0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

KEY[95:64]
0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

KEY[127:96]
0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

IVI[31:0]
0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

IVI[63:32]
0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

IVI[95:64]
0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

IVI[127:96]
0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

KEY[159:128]
0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

KEY[191:160]
0

0

0

0

0

0

0

0

0

0

0

0

0

0

SAES_KEYR6
Reset value

0

0

0

0

KEY[223:192]
0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

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

Reserved

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

Res.

Res.

KEY[255:224]

Res.

SAES_KEYR7

Res.

0x03C

0

SAES_KEYR5
Reset value

0x038

0

SAES_KEYR4
Reset value

0x034

0

SAES_IVR3
Reset value

0x030

0

SAES_IVR2
Reset value

0x02C

0

SAES_IVR1
Reset value

0x028

0

SAES_IVR0
Reset value

0x024

0

SAES_KEYR3
Reset value

0x020

0

SAES_KEYR2
Reset value

0x01C

0

SAES_KEYR1
Reset value

0x018

0

SAES_KEYR0
Reset value

0

DIN[31:0]

SAES_DOUTR
Reset value

0

0

SAES_DINR
Reset value

CHMOD[1:0]

Res.

Reset value

Res.

Res.

Res.

Res.

0

Res.

Res.

KEYSEL[2:0]

0

Res.

Res.

0

DMAINEN

Res.

0

DMAOUTEN

SAES_SR

0x004

0

Res.

0

Res.

0

Res.

0

Res.

0

Res.

0

CHMOD[2]

0

KEYSIZE

0

KEYPROT

0

Res.

IPRST

Reset value

0x000

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

SAES_CR

Res.

Offset

KSHAREID[1:0]

Register

Res.

Table 484. SAES register map and reset values

0x0400x2FF

<!-- pagebreak -->

RM0456 Rev 6

0x308
SAES_ICR

RM0456 Rev 6

KEIF
RWEIF
CCF

0
0
0

KEIF
CCF

Res.

Res.

Res.

0
RWEIF

Reset value
RNGEIF

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

SAES_IER
Res.

KEIE
RWEIE
CCFIE

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

Register

RNGEIE

Reset value

RNGEIF

Reset value
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

SAES_ISR

Res.

0x304
Res.

0x300

Res.

Offset

Res.

RM0456
Secure AES coprocessor (SAES)

Table 484. SAES register map and reset values (continued)

0
0
0
0

0
0
0
0

Refer to Section 2.3 on page 140 for the register boundary addresses.

<!-- pagebreak -->

