w

w

w

BLUE[7:0]
w

w

w

w

RM0456 Rev 6

w

w

w

w

RM0456

LCD-TFT display controller (LTDC)

Bits 31:24 CLUTADD[7:0]: CLUT address
These bits configure the CLUT address (color position within the CLUT) of each RGB value.
Bits 23:16 RED[7:0]: Red value
These bits configure the red value.
Bits 15:8 GREEN[7:0]: Green value
These bits configure the green value.
Bits 7:0 BLUE[7:0]: Blue value
These bits configure the blue value.

43.7.26

LTDC register map

Res.

Res.

Res.

Res.

Res.
Res.

Res.

Res.

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

Res.

Res.

Res.

0

0

LTDCEN

Res.

Res.

1

0

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

DGW[2:0]

DBW[2:0]

0

1

0

0

0

1

0

0

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

Res.

Res.

Res.

Res.

0

RM0456 Rev 6

TERRIE

FUIE

LIE

0

0

0

0

TERRIF

FUIF

LIF

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

RRIE

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

LTDC_ISR

Res.

0x038

Res.

Reset value

RRIF

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

LTDC_IER

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

BCBLUE[7:0]

Reserved
Res.

0x034

0

Reserved
Res.

0x030

0

BCGREEN[7:0]

0

Res.

Reset value

BCRED[7:0]

0

Res.

Res.

Res.

Res.

Res.

Res.

LTDC_BCCR

Res.

0x02C

Res.

Reset value

IMR

Res.

Res.

0

0

TOTALH[10:0]
0

0
Res.

Res.

0

DRW[2:0]

0

Res.

Res.

0

0

VBR

Res.

0

0

Res.

0

0

Res.

0

0

Res.

0

0

Res.

0

0

Res.

0

0

Res.

0

0

Res.

0

0

Res.

0

Res.

0

Res.

0

DEN

0

0

AAH[10:0]

Res.

0

0

Res.

0

0

Res.

0

0

Res.

0

Res.

0

Res.

0

Res.

0

0

AVBP[10:0]
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

Res.
0

Res.

0

Res.

0

Res.

0

VSH[10:0]
0

Res.

0

Res.

0

Res.

Res.

0

Res.

Res.

0

Res.

0

Res.

LTDC_SRCR

0

Res.

0

0

Res.

0

Res.

PCPOL

0

0

Res.

0

Res.

DEPOL

0

0

Res.

VSPOL

0

0

TOTALW[11:0]

Res.

HSPOL

Reset value

Res.

0x024

LTDC_GCR

Res.

Reset value

0x018

0

Res.

Res.

Res.

LTDC_TWCR

0

AAW[11:0]
0

Res.

Reset value

0x014

0

Res.

Res.

Res.

LTDC_AWCR

0

AHBP[11:0]
0

Res.

Reset value

0x010

0

Res.

Res.

LTDC_BPCR

Res.

0x00C

0
Res.

Reset value

HSW[11:0]

Res.

Res.

LTDC_SSCR

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

0x008

Register
name

Res.

Offset

Res.

Table 434. LTDC register map and reset values

0

0

0

0

<!-- pagebreak -->

1741

LCD-TFT display controller (LTDC)

RM0456

CTERRIF

CFUIF

CLIF

Res.

CRRIF

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

LTDC_ICR

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

0x03C

Register
name

Res.

Offset

Res.

Table 434. LTDC register map and reset values (continued)

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

LTDC_LIPCR

Res.

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

LTDC_CDSR

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

VDES

0

HDES

0

VSYNCS

0

1

1

1

1

Res.

0

Res.

CYPOS[15:0]

Reset value

Res.

CXPOS[15:0]

0

Res.

LTDC_CPSR

0

Res.

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

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

PF[2:0]

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

0

1

1

0

0

CONSTA[7:0]
1

1

1

1

1

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

LTDC_L1BFCR

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

BF1[2:0]

Res.

Res.

Res.

Res.

Res.

DCBLUE[7:0]

Res.

DCGREEN[7:0]

Res.

DCRED[7:0]

0

Res.

DCALPHA[7:0]

BF2[2:0]

0

0

0

0

0

0

0

0

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

Reset value

<!-- pagebreak -->

0

0

0

0

0

0

1

1

0

0

0

0

0

0

0

0

0

0

1

1

1

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

CFBLNBR[10:0]
0

RM0456 Rev 6

0

CFBLL[12:0]

Res.

0

Res.

0

Res.

0

CFBP[12:0]

Res.

0

Res.

0

Res.

0

Res.

0

Res.

Res.

Res.

Res.

LTDC_L1CFBLNR

0

Res.

0

Res.

0

Res.

0

Res.

Reset value
LTDC_L1CFBLR

Res.

CFBADD[31:0]

Res.

LTDC_L1CFBAR

Res.

Reserved

Res.

Reserved

Reset value

0x0B4

0

0

Reset value

0x0B0

0

0

1

LTDC_L1DCCR

0x0A40x0A8
0x0AC

0

CKBLUE[7:0]

Res.

0x0A0

0

Res.

CKGREEN[7:0]

Reset value
0x09C

0

0
Res.

LTDC_L1CACR

0

WVSTPOS[10:0]
0

CKRED[7:0]

0

Res.

Res.
Res.

Res.

0

0

Res.

0

0

Res.

0

0
Res.

0

Res.

0

Res.

0

Reset value

0x098

0

WHSTPOS[11:0]

Res.

0

Res.

Res.

Res.

Res.

Res.

Res.

LTDC_L1PFCR

0

WVSPPOS[10:0]

Reset value

0x094

0

Res.

0

Res.

0

Res.

Res.

Res.

Res.

Res.

0x090

Res.

Reset value
LTDC_L1CKCR

0

Res.

Res.

Res.

Res.

LTDC_L1WVPCR

0
Res.

Reset value
0x08C

0

0
WHSPPOS[11:0]

Res.

0x088

LEN

Reset value
LTDC_L1WHPCR

Res.

CLUTEN

Res.

Res.

Res.

Res.

Res.
Res.

Res.

Res.
Res.

Res.

Res.
Res.

Res.

Res.
Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.
Res.

Res.

Res.
Res.

Res.

Res.

Res.

Res.

Res.

LTDC_L1CR

Reserved

Res.

0x084

Reserved

Res.

0x04C0x080

COLKEN

0x048

0

Res.

0x044

LIPOS10:0]

HSYNCS

0x040

Res.

Reset value

0

0

0

0

0

0

0

RM0456

LCD-TFT display controller (LTDC)

Offset

Register
name

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

Table 434. LTDC register map and reset values (continued)

0x0B80x0C0

Reserved

Reserved

0

0

0

GREEN[7:0]
0

0

0

0

BLUE[7:0]

0

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

0

0

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

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

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

PF[2:0]

0x144

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

0

0

1

1

0

0

CONSTA[7:0]
1

1

1

1

1

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

LTDC_L2BFCR

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

BF1[2:0]

Res.

Res.

Res.

Res.

Res.

DCBLUE[7:0]

Res.

DCGREEN[7:0]

Res.

DCRED[7:0]

BF2[2:0]

0

0

0

0

0

0

0

0

0

0

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

CFBP[12:0]

0

0

0

0

0

1

0

0

0

0

0

0

0

1

1

1

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

CFBLNBR[10:0]
0

0

0

0

0

0

0

Reserved

LTDC_L2CLUTWR
Reset value

1

0

Reserved

0

CFBLL[12:0]

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

Res.

Res.

Res.

LTDC_L2CFBLNR

0

Res.

LTDC_L2CFBLR

0

Res.

0

Res.

0

Res.

0

Res.

Reset value

Res.

CFBADD[31:0]

Res.

LTDC_L2CFBAR

Res.

Reserved

Res.

Reserved

Reset value
0x1380x140

0

0

Reset value

Reset value

0x134

0

0

DCALPHA[7:0]

Res.

0x130

0

CKBLUE[7:0]

1

LTDC_L2DCCR

Res.

0x12C

0

0

Reset value
0x1240x128

0

Res.

CKGREEN[7:0]

0

Res.

0x120

0

WVSTPOS[10:0]

Reset value
0x11C

0

0
Res.

LTDC_L2CACR

0

Res.

CKRED[7:0]

0

Reset value

0x118

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

LTDC_L2PFCR

0

WVSPPOS[10:0]

Reset value

0x114

0

WHSTPOS[11:0]

Res.

0

0

Res.

0

0

Res.

0

0

Res.

0

Res.

0

Res.

Res.

Res.

Res.

Res.

LTDC_L2CKCR

Res.

Reset value
0x110

0

Res.

Res.

Res.

Res.

LTDC_L2WVPCR

0
Res.

Reset value
0x10C

0

0
WHSPPOS[11:0]

Res.

0x108

0

LEN

Res.

Res.

Reset value
LTDC_L2WHPCR

0

COLKEN

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.
Res.

Res.

Res.
Res.

LTDC_L2CR

0

Reserved
Res.

Reserved

0

Res.

0

Res.

0

Res.

0

CLUTEN

RED[7:0]
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

CLUTADD[7:0]
0

Res.

0x104

Reset value

Res.

0x0C80x100

LTDC_L1CLUTWR

Res.

0x0C4

CLUTADD[7:0]
0

0

0

0

0

0

RED[7:0]
0

0

0

0

0

0

0

GREEN[7:0]
0

0

0

0

0

0

0

0

0

BLUE[7:0]
0

0

0

0

0

0

0

0

Refer to Section 2.3 for the register boundary addresses.

RM0456 Rev 6

<!-- pagebreak -->

