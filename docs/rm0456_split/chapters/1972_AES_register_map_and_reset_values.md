1973

AES hardware accelerator (AES)

RM0456

Bits 31:3 Reserved, must be kept at reset value.
Bit 2 KEIF: Key error interrupt flag clear
Setting this bit clears the KEIF status bit of the AES_ISR register.
Bit 1 RWEIF: Read or write error interrupt flag clear
Setting this bit clears the RWEIF status bit of the AES_ISR register, and both RDERR and WRERR
flags in the AES_SR register.
Bit 0 CCF: Computation complete flag clear
Setting this bit clears the CCF status bit of the AES_SR and AES_ISR registers.

49.7.21

AES register map

0x008

0x00C

0x010

0x014

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

<!-- pagebreak -->

EN

DATATYPE[1:0]
WRERR

RDERR

CCF

0

Res.

MODE[1:0]

0

BUSY

Res.

CHMOD[1:0]
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

KEY[31:0]
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

0

0

0

0

0

0

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

0

0

0

0

0

0

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

0

0

0

0

0

0

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

AES_IVR3
Reset value

0

DOUT[31:0]

AES_IVR2
Reset value

0x02C

0

AES_IVR1
Reset value

0x028

0

AES_IVR0
Reset value

0x024

0

AES_KEYR3
Reset value

0x020

0

AES_KEYR2
Reset value

0x01C

0

AES_KEYR1
Reset value

0x018

0

AES_KEYR0
Reset value

0

DIN[31:0]
0

AES_DOUTR
Reset value

0

0

AES_DINR
Reset value

0
Res.

Res.

Res.

Res.

Reset value

KEYVALID

Res.

Res.

DMAINEN
Res.

Res.

DMAOUTEN
Res.

GCMPH[1:0].

0

Res.

0

Res.

Res.

0

Res.

Res.

0

0
Res.

Res.

Res.

Res.

CHMOD[2]

Res.

0

Res.

0

Res.

0

Res.

0

KEYSIZE

0

Res.

KMOD[0]
0
Res.

NPBLB[3:0]

KMOD[1]
0
Res.

Res.

Res.

Res.

Res.

Res.

AES_SR

0x004

Res.

0

Res.

IPRST

Reset value

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

AES_CR

0x000

Res.

Offset

Res.

Register

Res.

Table 478. AES register map and reset values

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

RM0456 Rev 6

0

0

0

RM0456

AES hardware accelerator (AES)

AES_KEYR4

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

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

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

KEY[255:224]
0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

SUSP[31:0]
0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

SUSP[31:0]
0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

SUSP[31:0]
0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

SUSP[31:0]
0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

SUSP[31:0]
0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

SUSP[31:0]
0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

SUSP[31:0]
0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

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

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

AES_ISR

Res.

0x304

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

Res.

Res.

Res.

Res.

Res.

AES_ICR

Res.

0x308

Res.

Reset value

Res.

Res.
Res.

CCFIE

Res.
Res.

0
CCF

Res.
Res.

0

0

0

0
CCF

Res.
Res.

Res.

Res.
Res.

0

Reset value

RWEIF

Res.

AES_IER

RWEIF

Reserved

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

Res.

0

Res.

0

Res.

0

RWEIE Res.

SUSP[31:0]

Res.

0x300

0

AES_SUSP7R
Reset value

0x0600x2FF

0

AES_SUSP6R
Reset value

0x05C

0

AES_SUSP5R
Reset value

0x058

0

AES_SUSP4R
Reset value

0x054

0

AES_SUSP3R
Reset value

0x050

0

AES_SUSP2R
Reset value

0x04C

0

AES_SUSP1R
Reset value

0x048

0

AES_SUSP0R
Reset value

0x044

0

AES_KEYR7
Reset value

0x040

0

AES_KEYR6
Reset value

0x03C

0

Res.

Reset value
0x038

0

KEIE

0x034

0

KEIF

Reset value
AES_KEYR5

KEIF

0x030

Register

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

Table 478. AES register map and reset values (continued)

0

0

0

Refer to Section 2.3 on page 140 for the register boundary addresses.

RM0456 Rev 6

<!-- pagebreak -->

