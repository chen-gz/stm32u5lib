1674

Audio digital filter (ADF)

40.8.14

RM0456

ADF SAD ambient noise level register (ADF_SADANLVR)
Address offset: 0x0C4
Reset value: 0x0000 0000
This register contains the ambient noise level computed by the SAD.

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

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

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

r

r

r

r

r

r

r

Res.

ANLVL[14:0]
r

r

r

r

r

r

r

r

Bits 31:15 Reserved, must be kept at reset value.
Bits 14:0 ANLVL[14:0]: Ambient noise level estimation
This bitfield is set by hardware. It contains the latest ambient noise level computed by the
SAD. To refresh this bitfield, the SDLVLF flag must be cleared.

40.8.15

ADF digital filter data register 0 (ADF_DFLT0DR)
Address offset: 0x0F0
Reset value: 0x0000 0000
This register is used to read the data processed by the digital filter.

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

r

r

r

r

r

r

r

DR[23:8]
r

r

r

r

15

14

13

12

r

r

r

r

11

10

9

8

r

DR[7:0]
r

r

r

r

r

r

r

7

6

5

4

3

2

1

0

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

r

Bits 31:8 DR[23:0]: Data processed by DFT0
Bits 7:0 Reserved, must be kept at reset value.

40.8.16

ADF register map

2

1

0

Res.

TRGO

12
Res.

3

13
Res.

Res.

14
Res.

4

15
Res.

Res.

16
Res.

5

17
Res.

Res.

18
Res.

6

19
Res.

Res.

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

ADF_GCR

Res.

31

0x000

Register name

Res.

Offset

Res.

Table 409. ADF register map and reset values

<!-- pagebreak -->

0

0

0

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

Res.

Res.

Res.

Res.

Res.

TRGSENS
0

CKGDEN

0

CCK0EN

0

CCK1EN

0

CKGMOD

0

CCK0DIR

0

TRGSRC
[3:0]

CCK1DIR

0

CCKDIV
[3:0]

Res.

0

Res.

Reset value

PROCDIV[6:0]

Res.

ADF_CKGCR

Res.

0x004

0
CKGACTIVE

Reset value

0

0

0

0x0B0

0x0B4

ADF_DFLT0ISR

Reserved

RM0456 Rev 6
Res.

0
0

Res.

0

0

0

0

0

Reserved
0
0
0
0
0

0

0
FTHIE

0

0
0
0

FTHF

0

DOVRIE

0

DOVRF

0
0
0

Res.
RSFLTBYP

Res.

DATSRC[1:0]

Res.

0
0
0
0

Res.
FTH
DMAEN
DFLTEN

0

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

1
0

SITFEN

4
2

5
3

6
SITFMOD
[1:0]
SCKSRC
[1:0]

7
Res.

8

9

10

11

12

13

14

15

16

17

18

19

20

21

22

23

24

25

26

27

28

29

30

31

Res.

Reserved

Res.

0

Res.

Res.

0

Res.

ACQMOD
[2:0]

Res.

0

RXNEF

0

0
RSFLTD

0
CICMOD[2:0]

Res.
TRGSENS

Res.

0

Res.

0

Res.

0

Res.

MCICD[8:0]
Res.

0

HPFBYP

HPFC[1:0]

Res.

1

Res.

0

Res.

Reserved
0

Res.

Reset value

Res.

0

Res.

Res.

1

Res.

Res.

0

Res.

0

Res.

1

Res.

SATIE

0

SATF

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

1

Res.

CKABIE

0

CKABF

0

Res.

TRGSRC
[3:0]

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

1

Res.

RFOVRIE

0

RFOVRF

Res.

Res.

0

Res.

0

STH[4:0]

Res.

SDDETIE

Reset value
0

SDDETF

Reset value
SDLVLIE

Res.

0

Res.

Res.

Res.

0

SDLVLF

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

0

Res.

Res.

Reserved

Res.

Res.

Reserved

Res.

Res.

Res.

0

Res.

SCALE[5:0]

Res.

Res.

0

Res.

Res.

NBDIS[7:0]

Res.

Res.

Res.

0

Res.
0

Res.

Res.

Res.

0

Res.
0

Res.

Res.

Res.

0

Res.
0

Res.

Res.

Res.

Res.
0

Res.

Res.

Res.
0

Res.

0

Res.

Reset value

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

0

Res.

ADF_DFLT0IER
Res.

0

Res.

Reset value

Res.

ADF_DLY0CR
Res.

ADF_
DFLT0RSFR
Res.

ADF_DFLT0CICR
Res.

0

Res.

DFLTRUN

0

Res.

DFLTACTIVE

Reset value
Res.

SITFACTIVE

ADF_DFLT0CR
Res.

BSMXACTIVE

0

Res.

0x0AC
Reset value

Res.

0x0A8
ADF_BSMX0CR

Res.

0x0A4
0

Res.

0x094 0x0A0
Reset value

Res.

0x090
ADF_SITF0CR

Res.

0x08C

Res.

0x088

Res.

0x084

Res.

0x080

SKPBF

Reserved

Res.

0x008 0x07C

Res.

Register name

Res.

Offset

Res.

RM0456
Audio digital filter (ADF)

Table 409. ADF register map and reset values (continued)

0
0

0

BSSEL[4:0]

0
0
0

0
0

SKPDLY[6:0]

0

0

Reserved

<!-- pagebreak -->

1674

Audio digital filter (ADF)

RM0456

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

RM0456 Rev 6

0

1

SADEN

2

DATCAP[1:0]
SNTHR
[2:0]

4

3
DETCFG
Res.

Res.

HGOVR
[2:0]
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
0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

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

0

0

0

Res.

Res.

0

0

0

Res.

Res.

0

Refer to Section 2.3 for the register boundary addresses.

<!-- pagebreak -->

