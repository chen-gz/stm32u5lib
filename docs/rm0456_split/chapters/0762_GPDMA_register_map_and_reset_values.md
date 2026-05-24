763

General purpose direct memory access controller (GPDMA)

RM0456

Bit 27 UDA: Update GPDMA_CxDAR register from memory
This bit is used to control the update of GPDMA_CxDAR from the memory during the link
transfer.
0:no GPDMA_CxDAR update
1: GPDMA_CxDAR update
Bit 26 UT3: Update GPDMA_CxTR3 from memory
This bit controls the update of GPDMA_CxTR3 from the memory during the link transfer.
0: no GPDMA_CxTR3 update
1: GPDMA_CxTR3 update
Bit 25 UB2: Update GPDMA_CxBR2 from memory
This bit controls the update of GPDMA_CxBR2 from the memory during the link transfer.
0: no GPDMA_CxBR2 update
1: GPDMA_CxBR2 update
Bits 24:17 Reserved, must be kept at reset value.
Bit 16 ULL: Update GPDMA_CxLLR register from memory
This bit is used to control the update of GPDMA_CxLLR from the memory during the link
transfer.
0: no GPDMA_CxLLR update
1: GPDMA_CxLLR update
Bits 15:2 LA[15:2]: pointer (16-bit low-significant address) to the next linked-list data structure
If UT1 = UT2 = UB1 = USA = UDA = ULL = 0 and if LA[15:2] = 0, the current LLI is the last
one. The channel transfer is completed without any update of the linked-list GPDMA register
file.
Else, this field is the pointer to the memory address offset from which the next linked-list data
structure is automatically fetched from, once the data transfer is completed, in order to
conditionally update the linked-list GPDMA internal register file (GPDMA_CxTR1,
GPDMA_CxTR2, GPDMA_CxBR1, GPDMA_CxSAR, GPDMA_CxDAR, and
GPDMA_CxLLR).
Note: The user must program the pointer to be 32-bit aligned. The two low-significant bits are
write ignored.
Bits 1:0 Reserved, must be kept at reset value.

17.8.20

GPDMA register map

<!-- pagebreak -->

RM0456 Rev 6

1

0
SEC0
PRIV0

0

0

0

0

0

0

0

0

0

0

0
LOCK0

2

SEC1
PRIV1

0

LOCK1

3

SEC2
PRIV2

0

LOCK2

4

SEC3
PRIV3

0

LOCK3

5

SEC4
PRIV4

0

LOCK4

6

SEC5
PRIV5

7

SEC6

Res.

0

LOCK5

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

GPDMA_
RCFGLOCKR

Res.

0x08

Res.

Reset value

PRIV6

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

GPDMA_
PRIVCFGR

Res.

0x04

LOCK6

8

SEC7

0

PRIV7

SEC8

0

LOCK7

9
SEC9

0

PRIV8

0

PRIV9

0

LOCK8

0

LOCK9

11

10
SEC10

0

PRIV10

0

LOCK10

12

SEC11

0

PRIV11

0

LOCK11

Res.

13

17

Res.

SEC12

18

Res.

0

PRIV12

19

Res.

0

LOCK12

20

Res.

14

21

Res.

SEC13

22

Res.

0

PRIV13

23

Res.

0

LOCK13

24

Res.

15

25

Res.

SEC14

26

Res.

0
PRIV14

27

Res.

0

Reset value

LOCK14

28

Res.

16

29

Res.

SEC15

30

GPDMA_
SECCFGR

PRIV15

31

0x00

LOCK15

Register name

Res.

Offset

Res.

Table 144. GPDMA register map and reset values

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

RM0456

General purpose direct memory access controller (GPDMA)

1

0

MIS1

MIS0

Reset value

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

0

0

0

0

0

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
IDLEF

Res.
Res.
Res.

Res.
Res.
Res.

Res.
Res.
Res.

Res.
Res.
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

SUSP

RESET

EN

SDW_LOG2
[1:0]

0

0

0

REQSEL[6:0]
0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

BNDT[15:0]
0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

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

Res.

0

0

0

BNDT[15:0]
0

BRC[10:0]
0

0

SWREQ

0

0

DREQ

0

0

BREQ

0
Res.

TRIGSEL[5:0]

0

0
Res.

0

SBL_1[5:0]

Res.

0

Res.

0

Res.

0

0

TRIGM
[1:0]

0

Res.

0
SINC

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

1
Res.

Res.

TCF
TCF

HTF
HTF

TCIE

0

PAM[1:0]

LSM

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

SA[31:0]
0

0

0

0

0

0

0

0

0

0

0

0

DA[31:0]
0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

DAO[12:0]

0xA8+0x80 * x GPDMA_CxBR2
(x = 12 to 15)
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

UT1

0xCC+0x80 * x GPDMA_CxLLR
(x = 0 to 11)
Reset value
0

UT2

UB1

USA

UDA

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

ULL

0

0

0

0

UB1

USA

UDA

UT3

UB2

0

UT2

0

UT1

0

0

0

0

0

0

0

SAO[12:0]

BRDAO[15:0]
0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

LA[15:2]
0

0

0

0

0

0

ULL

Res.

Res.

Res.

Res.

Res.

Res.

Res.

0
Res.

0

BRSAO[15:0]

0

0

LA[15:2]
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

Res.

Res.

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

0xA0+0x80 * x GPDMA_CxDAR
(x = 0 to 15)
Reset value
0

0xCC+0x80 * x GPDMA_CxLLR
(x = 12 to 15)
Reset value
0

0

SBX

LAP

0

0

SAP

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

SSEC

Res.

TRIGPOL
[1:0]

DINC

Res.

0

0

0

DDW_LOG2
[1:0]

Res.

DBX

Res.

Res.

DHX

0

Res.

0

0

HTIE

Res.

0

0

0

Res.

Res.

0

0

Res.

Res.

0

0

DTEF

Res.

0

0x9C+0x80 * x GPDMA_CxSAR
(x = 0 to 15)
Reset value
0

0

Res.

Res.
Res.
SDEC

Res.

Res.
DDEC

Res.

Res.

DAP

Res.

BRSDEC

0

BRDDEC

0

Res.

DSEC

0

TCEM
[1:0]

0

DBL_1[5:0]

0

DTEF

Res.

Res.

0

0

0

DTEIE

Res.

PRIO
[1:0]

0

0

0

Res.

Res.

0

Res.

Res.

0

ULEF

Res.
Res.

0

0
ULEF

Res.
Res.

0

0

ULEIE

Res.
Res.

0

Res.

Res.
Res.

0

USEF

Res.
Res.

0

0

USEF

Res.
Res.

Res.

0
Res.

Reset value

FIFOL[7:0]

0

USEIE

Res.

GPDMA_CxSR

0xA4+0x80 * x GPDMA_CxTR3
(x = 12 to 15)
Reset value

0

Res.

0x5C+0x80 * x GPDMA_CxFCR
(x = 0 to 15)
Reset value

Reset value

0

SUSPF

0

0x98+0x80 * x GPDMA_CxBR1
(x = 12 to 15)

0

SUSPF

0

0x98+0x80 * x GPDMA_CxBR1
(x = 0 to 11)
Reset value

0

SUSPIE

0

Reset value

0

Res.

0

0x94+0x80 * x GPDMA_CxTR2
(x = 0 to 15)

0

TOF

0

Reset value

0

TOF

0

0x90+0x80 * x GPDMA_CxTR1
(x = 0 to 15)

0

TOIE

0

0x64+0x80 * x GPDMA_CxCR
(x = 0 to 15)
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

0

Res.

0

Res.

LBA[31:16]
0

0x60+0x80 * x
(x = 0 to 15)

0

Res.

GPDMA_
CxLBAR
Reset value

0
Reserved

Res.

0x50+0x80 * x
(x = 0 to 15)

Reserved

Res.

0x14 - 0x4C

MIS0

2
MIS2

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

GPDMA_SMISR

Res.

0x10

MIS1

3
MIS3

0

MIS2

4
MIS4

0

MIS3

5
MIS5

0

MIS4

6
MIS6

0

MIS5

7
MIS7

0

MIS6

8
MIS8

0

MIS7

9
MIS9

0

MIS8

0

MIS9

11

10
MIS10

0

MIS10

12

MIS11

0

MIS11

13

17
Res.

MIS12

18
Res.

0

MIS12

19
Res.

14

20
Res.

MIS13

21
Res.

0

MIS13

22
Res.

15

23
Res.

MIS14

24
Res.

0

MIS14

25
Res.

16

26
Res.

MIS15

27
Res.

0

MIS15

28
Res.

0

Res.

29
Res.

0

Reset value

Res.

30

GPDMA_MISR

Res.

31

0x0C

Register name

Res.

Offset

Res.

Table 144. GPDMA register map and reset values (continued)

Refer to Section 2.3: Memory organization for the register boundary addresses.

RM0456 Rev 6

<!-- pagebreak -->

