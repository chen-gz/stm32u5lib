Reset value

Reserved

Reset value

Reserved

Reserved

RM0456 Rev 6
Res.
Res.
Res.
Res.
Res.
Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

X

Res.

Res.

Res.
X

0
1

Res.

Res.

Res.

TYPE1

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

TYPE0

Res.

RS

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

X X

0
1

Res.

Res.

Res.

BB

CCI

Reset value

INSTEN[1:0]

Res.

X X X X X

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

ATB

LPOVERRIDE

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

COND[5:0]

Res.

Res.

Res.

Res.

ISTALL

Res.

INSTPRIORITY

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

SEL1[3:0]

Res.

Res.

Res.

Reserved

Res.

Res.

Res.

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

X

Res.

Res.

Res.

Res.

Reset value

Res.

Res.

Res.

Res.

Res.

X

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Reset value

Res.

Res.

Res.

Res.

Reserved

Res.

Res.

Res.

Res.

Res.

Reset value

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Reserved

Res.

Res.

Res.

Res.

Res.

IDLE

PMSTABLE

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

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

0x03C

Res.

Reset value
Res.

ETM_CCCTLR
Res.

ETM_SYNCPR

Res.

0x030

Res.

0x038
ETM_STALLCTLR

Res.

0x028

Res.

0x034
ETM_
EVENTCTL1R

Res.

0x02C
ETM_
EVENTCTL0R

Res.

0x0140x01C

Res.

0x024
ETM_CONFIGR

Res.

0x020
ETM_STATR

Res.

0x00C

Res.

0x008

Res.

0x010
EN

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

ETM_PRGCTLR

Res.

0x004

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

Offset Register name

Res.

75.9.2

Res.

Debug support (DBG)
RM0456

Bits 7:0 PREAMBLE[27:20]: component identification bits [31:24]
0xB1: common identification value

ETM register map

The ETM registers are accessed by the debugger at address range 0xE0041000 to
0xE0041FFC.
Table 804. ETM register map and reset values

Reset value

Reserved
0

X X

X X X X X X X X

Reserved

SEL0[3:0]

X X X X

Reserved
X X

LEVEL[3:0]

Reserved
X X X X

PERIOD[4:0]

THRESHOLD[11:0]
0

Reserved

X X X X X X X X X X X X

TRACEID[6:0]

X X X X X X X

RM0456

Debug support (DBG)

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

ETM_IDR9
0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

ETM_IDR10

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

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

SSSTATUS

TRCRESET

Res.

Res.

TRCERR
0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

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

0

0

0

0

0

0

0

0

0

0

0

0

0

SUPPORT
[3:0]

NUMCONDSPC[31:0]
0

0

0

0

0

0

0

0

0

0

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

Reserved
Res.

Reserved

Reset value

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

EXLEVEL_S
[3:0]

Res.

1

0

1

0

0

0

0

RM0456 Rev 6

1

1

0

VMIDSIZE[4:0]

Res.

0

TRCCCI

TRCCOND

TRCBB

TRCDATA[1:0]

RETSTACK

0

1

0

0

0

0

0

1

0

CIDSIZE[4:0]
0

0

0

0

0

Res.

Res.

NUMEVENT
[1:0]
0

Res.

0

0

REVISION
[3:0]

Res.

0

Res.

0

DASIZE[4:0]

1

1

TRCARCHM TRCARCHMI
AJ
N
[3:0]
[3:0]

Res.

DVSIZE[4:0]

1

0

Res.

Res.

Res.

0

1

0

CONDTYPE
[1:0]

Res.

QSUPP[1:0]
0
Res.

Res.

TRCEXDATA

Res.
Res.

Res.

Res.

Res.

0

1

Res.

0

NUMPROC[2:0]

NOOVERFLOW

Reset value

0

Res.

Res.

CCSIZE[3:0]

ETM_IDR3

0

TRCERR

ETM_IDR2

0

0

SYNCPR

0

STALLCTL

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

1

SYSSTALL

0

Reset value

0x1EC

Res.

Reset value

Res.

DESIGNER[7:0]

Res.

0x1E8

ETM_IDR1

Res.

0x1E4

1

Res.

Reset value

COMMOPT

ETM_IDR0

0
Reserved

Res.

0x1E0

Reserved

Res.

0x1C40x1DC

INSTP0[1:0]

ETM_IMSPECR0

0

NUMCONDKEY[31:0]

ETM_IDR13
Reset value

0

NUMP1SPC[31:0]

ETM_IDR12
Reset value

0

NUMP1KEY[31:0]

ETM_IDR11
Reset value

0

Res.

Reset value

0

NUMP0KEY[31:0]

Res.

Reset value

0

Res.

0

Res.

0x1C0

0

Res.

0x1980x1BC

0

Res.

0x194

MAXSPEC[31:0]
0

Res.

0x190

ETM_IDR8
Reset value

Res.

0x18C

Reserved

Res.

0x188

Reserved

Res.

0x184

X X X X X X X X X X X X X X X X

Res.

0x180

X X X X X X X X

VALUE[15:0]

Reset value
0x144 to
0x17C

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

ETM_CNTRLDVR0

X X X

EVENT[7:0]

Reserved
Res.

Reserved
Res.

0x140

X X X X

Res.

0x0840x13C

EXLEVEL_S
[3:0]

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

ETM_VICTLR

Res.

0x080

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

Table 804. ETM register map and reset values (continued)

0

0

1

IASIZE[4:0]
0

0

0

1

0

0

0

1

0

0

CCITMIN[11:0]

0

0

0

0

0

0

0

0

<!-- pagebreak -->

