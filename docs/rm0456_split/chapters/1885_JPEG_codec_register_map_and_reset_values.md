18

RM0456 Rev 6

RM0456

JPEG codec (JPEG)

46.5.19

JPEG codec register map
The following table summarizes the JPEG codec registers. Refer to the register boundary
addresses table for the JPEG codec register base address.

Res.

START

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

JPEG_CONFR0

Res.

0x000

Res.

Offset Register name

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

Table 455. JPEG codec register map and reset values

JPEG_CONFR4

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.
Res.

Res.

Res.

Res.

Res.

Res.

Res.

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

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

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

0

0

0

0

0

0

0

0

0

0

0

0x0480x04C
0x0500x08C

0

Reset value

0

0

0

0

0

0

NB[3:0]
0

0

VSF[3:0]

0

0

NF[1:0]

Res.
0

0

0

NB[3:0]

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

JCEN

0

0

0

0

1

1

Res.

Res.

Res.

0

Res.

0

Res.

IFTIE
IFTF

IFNFIE

0

CEOCF

IFNFF

OFTIE

0

OFTF

OFNEIE

0

OFNEF

EOCIE

0

EOCF

HPDIE

0

HPDF

Res.

Res.

Res.

0

CHPDF

Res.
Res.

0

COF

Res.
Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

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

Res.

IDMAEN

Res.

Res.

ODMAEN

0
Res.

IFF

0

Res.

OFF

Res.

Res.

Res.

Res.

Res.

Res.

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

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

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

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

DATAIN[31:0]
0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

DATAOUT[31:0]
0

0

0

0

0

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
JPEG_QMEM0

0

Reserved

JPEG_DOR
Reset value

0

Res.

0x044

0

NB[3:0]

VSF[3:0]
0

0

NB[3:0]

VSF[3:0]

HSF[3:0]
0

JPEG_DIR
Reset value

Res.

Res.

Res.

Res.

Res.

Res.
0

HSF[3:0]
0

Reserved

0

VSF[3:0]

HSF[3:0]
0

Res.

Res.

HSF[3:0]

Res.

0

Reset value
0x040

0

HD

0

JPEG_CFR

0

HD

0

Reset value

0x038

0

HD

0

0

HD

0

0

Res.

0

Res.

0

0

HA

0

Res.

0

0

HA

0

0

HA

0

0

HA

0

0

QT[1:0]

0

0

QT[1:0]

0

JPEG_SR

0

0

QT[1:0]

0

Reset value
0x034

0

0

QT[1:0]

0

0

JPEG_CR

DE

0

Res.

0

0

Res.

0x030

0

Res.

0

Reset value
0x0200x02C

COLSPACE
[1:0]

0

0

JPEG_CONFR7

0

Res.

0

Reset value
0x01C

0

Res.

0

0

JPEG_CONFR6

0

Res.

0

Reset value
0x018

0

Res.

0

XSIZE[15:0]

Reset value

JPEG_CONFR5

0

0

Reset value
0x014

NS[1:0]

0

Res.

0

HDR

0

Res.

Res.

0

Res.

Res.

0

Res.

0x010

0

Res.

0x00C

0

NMCU[25:0]
0

JPEG_CONFR3

0

Res.

Res.

Reset value

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

Reset value
JPEG_CONFR2

Res.

YSIZE[15:0]

Res.

0x008

JPEG_CONFR1

Res.

0x004

0
Res.

Reset value

0

0

0

0

Reserved
QCOEF{4*y+3}[7:0]

QCOEF{4*y+2}[7:0]

QCOEF{4*y+1}[7:0]

QCOEF{4*y}[7:0]

X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X

RM0456 Rev 6

<!-- pagebreak -->

1886

JPEG codec (JPEG)

RM0456

0x1500x18C

QCOEF{4*y+1}[7:0]

QCOEF{4*y}[7:0]

QCOEF{4*y+2}[7:0]

QCOEF{4*y+1}[7:0]

QCOEF{4*y}[7:0]

QCOEF{4*y+3}[7:0]

QCOEF{4*y+2}[7:0]

QCOEF{4*y+1}[7:0]

QCOEF{4*y}[7:0]

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X
Res.

Reset value

QCOEF{4*y+3}[7:0]

Res.

JPEG_QMEM3

QCOEF{4*y+2}[7:0]

X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X

Res.

0x1100x14C

Reset value

Res.

JPEG_QMEM2

QCOEF{4*y+3}[7:0]

X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X

Res.

0x0D00x10F

Reset value

Res.

JPEG_QMEM1

Res.

0x0900x0CC

Res.

Offset Register name

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

Table 455. JPEG codec register map and reset values (continued)

DATA{x}
[99:96]

DATA{x}[95:64]

JPEG_HUFFMIN

DATA{x}[63:32]
DATA{x}[31:0]

Reset value

Reset value

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

HCODE{2*y}[7:0]

Res.

Res.

HLEN{2*y}
[3:0]

HCODE{2*y+1}[7:0]

Res.

X X X X X X X X X X X X
HCODE{2*y}[7:0]

X X X X X X X X X X X X

RM0456 Rev 6

Res.

Res.

HCODE{2*y+1}[7:0]

Res.

HLEN{2*y}
[3:0]

HCODE{2*y+1}[7:0]

Res.

X X X X X X X X X X X X
Res.

X X X X X X X X X X X X

Res.

Res.

HLEN{2*y}
[3:0]

X X X X X X X X X X X X
Res.

Res.

Res.

Res.
Res.

Res.

Res.

HLEN{2*y}
[3:0]

Res.

Res.

Res.

Res.
Res.

X X X X X X X X X X X X

Res.

Res.

Res.

X X X X X X X X X X X X
HLEN{2*y+1}
[3:0]

HCODE{2*y+1}[7:0]

Refer to Section 2.3 for the register boundary addresses.

<!-- pagebreak -->

