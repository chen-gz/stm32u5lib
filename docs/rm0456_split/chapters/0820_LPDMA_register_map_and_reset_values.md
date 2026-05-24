821

Low-power direct memory access controller (LPDMA)

RM0456

Bits 26:17 Reserved, must be kept at reset value.
Bit 16 ULL: Update LPDMA_CxLLR register from memory
This bit is used to control the update of the LPDMA_CxLLR register from the memory during the link
transfer.
0: no LPDMA_CxLLR update
1: LPDMA_CxLLR update
Bits 15:2 LA[15:2]: pointer (16-bit low-significant address) to the next linked-list data structure
If UT1 = UT2 = UB1 = USA = UDA = ULL = 0 and if LA[15:2] = 0, the current LLI is the last one. The
channel transfer is completed without any update of the linked-list DMA register file.
Else, this field is the pointer to the memory address offset from which the next linked-list data
structure is automatically fetched from, once the data transfer is completed, in order to conditionally
update the linked-list DMA internal register file (LPDMA_CxTR1, LPDMA_CxTR2, LPDMA_CxBR1,
LPDMA_CxSAR, LPDMA_CxDAR and LPDMA_CxLLR).
Note: The user must program the pointer to be 32-bit aligned. The two low-significant bits are write
ignored.
Bits 1:0 Reserved, must be kept at reset value.

18.8.16

LPDMA register map

SEC1

SEC0

0

0

0

0

MIS2

MIS1

MIS0

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

0

Res.

0

MIS3

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

MIS0

0

MIS1

0

MIS2

0

MIS3

0

Res.

0

Res.

0

Res.

LOCK0

PRIV0

SEC2

PRIV1

0

LOCK1

0

LOCK2

PRIV2

Res.

SEC3

0

LOCK3

PRIV3

Res.
0

Res.

Res.

Res.
0

Res.

Res.

Res.
0

Res.

Res.

0

Res.

0

RM0456 Rev 6

Res.

Res.

Res.

Res.

IDLEF
EN

0

Res.

0

SUSP

0

RESET

0

Res.

0

Res.

0

1
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

0

Res.

Res.

Res.
LSM
0

0

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

PRIO[1:0]
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

Reset value

Res.

Res.

0

Res.

Res.

0

Res.

Res.

0

Res.

Res.

Res.

Res.

TCF

Res.

0

TCF

Res.

0

TCIE

Res.

Res.

Res.

<!-- pagebreak -->

HTF

Res.

LPDMA_CxCR

0

HTF

Res.

0x064+
0x080 * x
(x = 0 to 3)

0

Reset value

HTIE

Res.

LPDMA_CxSR

Res.

Res.

0x060+
0x080 * x
(x = 0 to 3)

DTEF

LPDMA_CxFCR

DTEF

0x05C+
0x080 * x
(x = 0 to 3)

DTEIE

0

Res.

0

ULEF

0

ULEF

0

ULEIE

0

Res.

0

USEF

0

USEF

0

USEIE

0

LBA[31:16]

Res.

0

SUSPF

0

SUSPF

0

SUSPIE

0

Res.

0

TOF

0

TOF

0

TOIE

Reset value

Res.

LPDMA_CxLBAR

Res.

0x050+
0x080 * x
(x = 0 to 3)

Reset value

0

Res.

Res.

Reserved

0

Res.

Reset value
0x014 0x04C

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

LPDMA_SMISR

Res.

0x010

Res.

Reset value

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

LPDMA_MISR

Res.

0x00C

Res.

Reset value

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

LPDMA_
RCFGLOCKR

Res.

0x008

Res.

Reset value

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

LPDMA_PRIVCFGR

Res.

0x004

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

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

LPDMA_SECCFGR

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

Table 156. LPDMA register map and reset values

0

0

0

RM0456

Low-power direct memory access controller (LPDMA)

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

SDW_LOG2[1:0]

Res.

Res.

REQSEL
[4:0]

Res.

0

Res.

0

Res.

0

0

Res.

0

0

SINC

Res.

Res.

Res.

Res.

Res.

Res.

Res.

PAM

0

0

Res.

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

0

0

0

0

0

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

Reset value

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

LPDMA_CxLLR

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

SA[31:0]

UT1

Reset value

0

0

0

Res.

0

BREQ

0

0
SWREQ

0

0
Res.

0

Res.

SSEC

TRIGM
[1:0]

0

Res.

DDW_LOG2[1:0]
Res.

TRIGSEL
[4:0]

0

Res.

Res.

Res.

0

Res.

Res.

Res.

DINC

Res.

Res.

Res.

Res.

Res.

TRIGPOL
[1:0]

Res.

Res.

Res.

Res.

0

Res.

0x0CC+
0x080 * x
(x = 0 to 3)

Res.

LPDMA_CxDAR

Res.

0x0A0+
0x080 * x
(x = 0 to 3)

0
Res.

LPDMA_CxSAR

0
Res.

0x09C+
0x080 * x
(x = 0 to 3)

Res.

LPDMA_CxBR1

Res.

0
Res.

0

Res.

TCEM
[1:0]

Reset value
0x098+
0x080 * x
(x = 0 to 3)

Res.

LPDMA_CxTR2

0

Res.

0x094+
0x080 * x
(x = 0 to 3)

Res.

0

Res.

Reset value

Res.

LPDMA_CxTR1

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

0x090+
0x080 * x
(x = 0 to 3)

Res.

Offset Register name

DSEC

Table 156. LPDMA register map and reset values (continued)

0

Reset value

0

0

0

0

0

0

DA[31:0]

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

Refer to Section 2.3 for the register boundary addresses.

RM0456 Rev 6

<!-- pagebreak -->

