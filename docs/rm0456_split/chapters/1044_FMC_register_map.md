1045

Flexible static memory controller (FSMC)

27.7.8

RM0456

FMC register map

ACCMOD[1:0]
0

0

ACCMOD[1:0]

0

0

0

ACCMOD[1:0]

0

0

0

CLKDIV[3:0]

BUSTURN
[3:0]

1

1

1

1

1

1

1

1

1

CLKDIV[3:0]

BUSTURN
[3:0]

1

1

1

1

1

1

1

1

1

<!-- pagebreak -->

0

0

1

1

CLKDIV[3:0]

BUSTURN
[3:0]

1

1

1

CLKDIV[3:0]

BUSTURN
[3:0]

0

1

1

1

1

1

1

1

1

1

1

1

1

CNTB4EN

CNTB3EN

CNTB2EN

CNTB1EN

1

Res.

1

Res.

1

Res.

1

Res.

1

Res.

1

Res.

1

Res.

1

Res.

1

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

0

1

DATLAT[3:0]

Res.

DATAHLD[1:0]

Reset value

1

DATLAT[3:0]

ACCMOD[1:0]

FMC_BWTR1

1

DATLAT[3:0]

Reset value

0x104

1

Res.

ACCMOD[1:0]

0

DATLAT[3:0]

1

1

RM0456 Rev 6

1

2

1

0

MUXEN

MBKEN

MUXEN

MBKEN

0

0

1

0

1

1

1

1

1

1

1

1

1

1

1

MUXEN

MBKEN

0

1

0

0

1

0

MWID MTYP
[1:0] [1:0]

MBKEN

0

0

1

0

1

0

1

1

1

1

1

1

1

1

1

1

1

1

1

0

0

0

1

1

1

1

1

1

1

1

ADDHLD[3:0] ADDSET[3:0]

1

1

1

1

1

1

1

1

1

1

ADDHLD[3:0] ADDSET[3:0]

1

1

DATAST[7:0]

1

1

ADDHLD[3:0] ADDSET[3:0]

DATAST[7:0]

1

0

0

DATAST[7:0]

1

1

MWID MTYP
[1:0] [1:0]

DATAST[7:0]

1

1

1

1

1

1

1

1

ADDHLD[3:0] ADDSET[3:0]

1

1

1

1

1

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

CSCOUNT[15:0]
0

0

BUSTURN
[3:0]
1

1

MUXEN

0

3

1

4

WAITCFG

1

5

WREN

0

0

6

WAITEN

0

0

7

EXTMOD

0

FACCEN

ASYNCWAIT

0

0

FACCEN

CPSIZE
[2:0]

0

FACCEN

0

0

FACCEN

1

8

1

Res.

WAITCFG

0

1

MWID MTYP
[1:0] [1:0]

Res.

WREN

0

1

Res.

WAITEN

0

0

1

Res.

EXTMOD

0

9

ASYNCWAIT

0

WAITPOL

CPSIZE
[2:0]

BURSTEN

0

0

1

WAITPOL

1

0

BURSTEN

1

1

WAITPOL

WAITCFG

0

0

BURSTEN

11

WREN

0

0

WAITPOL

12

WAITCFG

WAITEN

0

0

MWID MTYP
[1:0] [1:0]

BURSTEN

13

WREN

EXTMOD

0

10

14

WAITEN

ASYNCWAIT

0

Res.

15

EXTMOD

CPSIZE
[2:0]

Res.

16

ASYNCWAIT

0

Res.

18

1

0

Res.

19

1

0

17

20

CBURSTRW

0

CBURSTRW

Res.
Res.

Res.

0

CBURSTRW

FMC_
PCSCNTR

Res.

0

CBURSTRW

0

Res.

21

0

Res.

CCLKEN

0

Res.

0

Res.

Reset value

Res.

0

Res.

FMC_BTR4

Res.

0

Res.

0

Res.

22

Reset value

Res.

WFDIS

FMC_BTR3

Res.

0

0

Res.

0

Res.

CPSIZE
[2:0]

Res.

23

Reset value

Res.

NBL
SET
[1:0]

Res.

24

FMC_BTR2

Res.

25

Res.

0

Res.

Reset value

Res.

26

FMC_BTR1

Res.

0

Res.

27

Reset value

Res.

NBL
SET
[1:0]

Res.

28

FMC_BCR4

Res.

0

Res.

0

Res.

29

0

Res.

30

Reset value

Res.

NBL
SET
[1:0]

Reset value

Res.

FMC_BCR3

FMC_BCR1

FMCEN

0

Res.

0x20

FMCEN

0x1C

0

FMCEN

0x14

0

FMCEN

0x0C

Reset value

DATAHLD[1:0]

0x04

NBL
SET
[1:0]

DATAHLD[1:0]

0x18

FMC_BCR2

DATAHLD[1:0]

0x10

0

DATAHLD[1:0]

0x08

0

Res.

0x00

Register name
reset value

Res.

Offset

31

Table 249. FMC register map and reset values

0

0

0

0

0

0

DATAST[7:0]

1

1

1

1

1

1

0

0

ADDHLD[3:0] ADDSET[3:0]

1

1

1

1

1

1

1

1

1

1

RM0456

Flexible static memory controller (FSMC)

1

1

1

1

1

1

1

1

0x88
0x8C
0x94

Reset value

1

1

FMC_PATT
Reset value

1

1

1

1

0

0

1

1

ATTHIZ[7:0]
1

1

1

1

1

1

1

1

1

0

0

1

1

1

1

1

1

FMC_ECCR
Reset value

0

0

1

1

0

0

0

1

1

1

0

1

1

0

1

2

3

4

1

1

1

1

PWID
[1:0]

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

MEMSETx[7:0]
0

0

1

1

ATTWAIT[7:0]
0

5

1

Res.

0

1

PWAITEN

0

1

1

PBKEN

Res.

1

1

1

MEMWAITx[7:0]

ATTHOLD[7:0]

1

PTYP

Res.

MEMHOLDx[7:0]

1

1

1
MEMHIZx[7:0]

1

IREN

Res.

TCLR[3:0]

1

Reset value
FMC_PMEM

1

ILEN

0

1

IFEN

0

1

ECCEN

0

1

Res.

0

1

Res.

0

1

Res.

0

Res.

TAR[3:0]

1

1

Res.

1

1

ADDHLD[3:0] ADDSET[3:0]

Res.

1

1

ADDHLD[3:0] ADDSET[3:0]

DATAST[7:0]

1

1

IRS

1

1

ILS

1

1

IFS

1

6

7

8

9

11

10
1

Res.

1

1

Res.

1

1

FEMPT

1

BUSTURN
[3:0]
1

12

13

14

15

17

16
1

Res.

Res.

Res.

1

DATAST[7:0]

Res.

Res.

Res.

Res.
Res.

1

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

ADDHLD[3:0] ADDSET[3:0]

Res.

Res.

Res.

Res.

Res.

Res.
Res.

Res.

18

19

21

22

23

24

25

26

27

20
Res.

Res.

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

1

ECCPS
[2:0]

Res.

Res.

Res.

Res.

FMC_SR

1

1

Reset value
0x84

1

0
Res.

0

1

DATAST[7:0]

BUSTURN
[3:0]
1

Res.

0

Res.

28

29

ACCMOD[1:0]
ACCMOD[1:0]

0

Res.

FMC_PCR

Res.

0

0

Res.

Reset value

0

1

Res.

FMC_BWTR4

0

Res.

0

0

BUSTURN
[3:0]

Res.

Reset value

ACCMOD[1:0]

30

DATAHLD[1:0]

FMC_BWTR3

0

Res.

0x80

0
DATAHLD[1:0]

0x11C

Reset value

DATAHLD[1:0]

0x114

FMC_BWTR2

Res.

0x10C

Register name
reset value

Res.

Offset

31

Table 249. FMC register map and reset values (continued)

1

1

1

1

ATTSET[7:0]

1

1

1

1

0

0

1

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

ECCx[31:0]
0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

Refer to Section 2.3 on page 140 for the register boundary addresses.

RM0456 Rev 6

<!-- pagebreak -->

