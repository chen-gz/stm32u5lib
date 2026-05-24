3020

Serial audio interface (SAI)

RM0456

Bit 15 Reserved, must be kept at reset value.
Bits 14:12 DLYM2R[2:0]: Delay line for second microphone of pair 2
This bitfield is set and cleared by software.
000: No delay
001: Delay of 1 TSAI_CK period
010: Delay of 2 TSAI_CK periods
...
111: Delay of 7 TSAI_CK periods
This bitfield can be changed on-the-fly.
Note: This bitfield can be used only if D2 line is available.Refer to Section 69.3: SAI implementation to
check if it is available.
Bit 11 Reserved, must be kept at reset value.
Bits 10:8 DLYM2L[2:0]: Delay line for first microphone of pair 2
This bitfield is set and cleared by software.
000: No delay
001: Delay of 1 TSAI_CK period
010: Delay of 2 TSAI_CK periods
...
111: Delay of 7 TSAI_CK periods
This bitfield can be changed on-the-fly.
Note: This bitfield can be used only if D2 line is available.Refer to Section 69.3: SAI implementation to
check if it is available.
Bit 7 Reserved, must be kept at reset value.
Bits 6:4 DLYM1R[2:0]: Delay line adjust for second microphone of pair 1
This bitfield is set and cleared by software.
000: No delay
001: Delay of 1 TSAI_CK period
010: Delay of 2 TSAI_CK periods
...
111: Delay of 7 TSAI_CK periods
This bitfield can be changed on-the-fly.
Note: This bitfield can be used only if D1 line is available.Refer to Section 69.3: SAI implementation to
check if it is available.
Bit 3 Reserved, must be kept at reset value.
Bits 2:0 DLYM1L[2:0]: Delay line adjust for first microphone of pair 1
This bitfield is set and cleared by software.
000: No delay
001: Delay of 1 TSAI_CK period
010: Delay of 2 TSAI_CK periods
...
111: Delay of 7 TSAI_CK periods
This bitfield can be changed on-the-fly.
Note: This bitfield can be used only if D1 line is available.Refer to Section 69.3: SAI implementation to
check if it is available.

<!-- pagebreak -->

RM0456 Rev 6

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

SAI_PDMCR

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

CKEN4

CKEN3

CKEN2

CKEN1

Res.

Res.

Reset value

SAI_xDR

0

0

0

0

Reset value

RM0456 Rev 6
0
0
0

Reset value
FREQ
WCKCFG
MUTEDET
OVRUDR

0
0
0
1
0
0
0

Res.

0

OVRUDRIE

0

DATA[31:0]

0

0

0

0

0

0

COVRUDR

0

0

0

0

0

0

0

0

PDMEN

MUTEDETIE

0

CMUTEDET

0

Res.

WCKCFGIE

FSALL[6:0]

CWCKCFG

MONO
SYNCEN[1:0]

TRIS
FFLUS

0
0
0
0

0

FTH[2:0]

0

MUTE

0

MODE[1:0]

PRTCFG[1:0]

Res.

DS[2:0]

CKSTR
LSBFIRST
1

MUTE VAL

0

Res.

0

FREQIE

OUTDRIV

Res.

0

Res.

0

CNRDYIE

Reset value

CNRDY

0

0

Res.

0

0

AFSDETIE

0

AFSDET

0

SLOTSZ[1:0]

0

LFSDETIE

CPL

COMP[1:0]

SAIEN

Res.

Res.

DMAEN

Res.

NODIV
Res.

MUTECN[5:0]

LFSDET

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

Res.

0

Res.

NBSLOT[3:0]

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

0

Res.

Res.

Res.

0

Res.

SAI_xIM

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

SLOTEN[15:0]
Res.

FSDEF

0
0

Res.

0

Res.

FSPOL
0

Res.

0

Res.

FSOFF

Reset value

Res.

0

Res.

Res.

Res.

Res.

0

Res.

0

Res.

Reset value
0

Res.

0

Res.

Res.

0

MCKDIV[5:0]

0

Res.

0

Res.

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

Res.
0

FLVL[2:0]

0

Res.

Res.

OSR

0

Res.

Res.

Res.

Res.

Res.

Res.

0

Res.

Res.

MCKEN

0

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

0

Res.

Res.

Res.

Res.
0

CCNRDY

0

Res.

Res.

0

Res.

Res.

Res.

Res.

Res.

Reset value

MICNBR[1:0]

0

Res.

Res.

Res.

0

Res.

SAI_xSLOTR

Res.

Res.

Res.

0

Res.

Res.

Res.

Reset value

Res.

Res.

Res.

Res.

19
18
17
16
15
14
13
12

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

1

2
SYNCIN[1:0]

3
Res.

4

5
Res..

SYNCOUT[1:0]

6

20

Res.

7

21

Res.

Res.

22

Res.

8

23

Res.

Res.

24

Res.

9

25

Res.

Res.

26

Res.

Res.

27

Res.

11

28

Res.

10

29

Res.

Res.

30

Res.

Res.

31

SAI_GCR

CAFSDET

0

Res.

SAI_xCLRFR
Res.

SAI_xSR
Res.

Register name

CLFSDET

0

Res.

0x44

Reset value

Res.

0x20 or 0x40

Res.

0x1C or
0x3C
SAI_xFRCR

Res.

0x18 or 0x38

Res.

0x14 or 0x34
Reset value

Res.

0x10 or 0x30

Res.

0x0C or
0x2C
SAI_xCR2

Res.

0x08 or 0x28
SAI_xCR1

Res.

0x04 or 0x24

Res.

0x0000

Res.

Offset

Res.

69.6.20

Res.

RM0456
Serial audio interface (SAI)

SAI register map
Table 716. SAI register map and reset values

0
0

0
0

FRL[7:0]

0
0
0

1
1
1

FBOFF[4:0]

0
0
0
0
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

3020

Serial audio interface (SAI)

RM0456

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

Refer to Section 2.3: Memory organization for the register boundary addresses.

<!-- pagebreak -->

RM0456 Rev 6

0

0

1

2

DLYM1L[2:0]

Res.

3

4

5

6

DLYM1R[2:0]

Res.

7

8

9
DLYM2L[2:0]

Res.

11

10

12

13

14

DLYM2R[2:0]

Res.

15

16

17

18

DLYM3L[2:0]

Res.

19

20

21

22

DLYM3R[2:0]

Res.

23

24

25

26

DLYM4L[2:0]

Res.

27

28

29

30

SAI_PDMDLY

DLYM4R[2:0]

0x48

Register name

31

Offset

Res.

Table 716. SAI register map and reset values (continued)

0

0

0

RM0456

70

FD controller area network (FDCAN)

FD controller area network (FDCAN)
This section applies to STM32U53xxx/54xxx/57xxx/58xxx/59xxx/5Axxx/5Gxxx devices only.

70.1

Introduction
The controller area network (CAN) subsystem (see Figure 881) consists of one CAN
module, a shared message RAM, and a configuration block. Refer to the memory map for
the base address of each of these parts.
The modules (FDCAN) are compliant with ISO 11898-1: 2015 (CAN protocol specification
version 2.0 part A, B) and CAN FD protocol specification version 1.0.
A 0.8-Kbyte message RAM per FDCAN instance is used for filtering, transmitting event
FIFOs, and receiving and transmitting FIFOs.

RM0456 Rev 6

<!-- pagebreak -->

