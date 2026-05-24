363

Embedded flash memory (FLASH)

7.9.34

RM0456

FLASH privilege block based bank 2 register x
(FLASH_PRIVBB2Rx)
Address offset: 0xF0 + 0x4 * (x - 1), (x = 1 to 8)
Reset value: 0x0000 0000
Access: no wait state; word, half-word, and byte access
This register is privilege. It can be read written only by a privileged access. This register can
be protected against nonsecure access (refer to Table 76).

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

PRIV2
BB31

PRIV2
BB30

PRIV2
BB29

PRIV2
BB28

PRIV2
BB27

PRIV2
BB26

PRIV2
BB25

PRIV2
BB24

PRIV2
BB23

PRIV2
BB22

PRIV2
BB21

PRIV2
BB20

PRIV2
BB19

PRIV2
BB18

PRIV2
BB17

PRIV2
BB16
rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

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

PRIV2
BB15

PRIV2
BB14

PRIV2
BB13

PRIV2
BB12

PRIV2
BB11

PRIV2
BB10

PRIV2
BB9

PRIV2
BB8

PRIV2
BB7

PRIV2
BB6

PRIV2
BB5

PRIV2
BB4

PRIV2
BB3

PRIV2
BB2

PRIV2
BB1

PRIV2
BB0

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

Bits 31:0 PRIV2BBi: page privileged/unprivileged attribution (i = 31 to 0)
Each bit is used to set one page security attribution in bank 2.
0: Page (32 * (x - 1) + i) in bank 2 accessible by unprivileged access
1: Page (32 * (x - 1) + i) in bank 2 only accessible by privileged access

7.9.35

FLASH register map

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

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

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
PDKEY1[31:0]
0

0

0

0

0

0

0

0

0

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

FLASH_NSSR

Res.

Res.

Res.

Res.

Res.

PD2

PD1

OEM2LOCK

OEM1LOCK

WDW

BSY

Res.

Res.

OPTWERR

Res.

Res.

Res.

Res.

Res.

PGSERR

SIZERR

PGAERR

WRPERR

PROGERR

PDKEY2[31:0]

Res.

FLASH_PDKEY2R

0

0

X

X

X

X

0

0

0

0

0

Reset value

<!-- pagebreak -->

0

OPTKEY[31:0]

FLASH_PDKEY1R

0x20

0

Res.

0

0

SECKEY[31:0]

Reserved

Reset value

0

EOP

0

Res.

0

0

Res.

0x1C

0

FLASH_OPTKEYR

0x14
0x18

0

0

Res.

0x10

0

0

Res.

0x0C

Reset value
FLASH_SECKEYR

Res.

0

Reserved
NSKEY[31:0]

0

Res.

0x08

Reserved
FLASH_NSKEYR

0

LATENCY
[3:0]

OPERR

0x04

Res.

0

Res.

0

Res.

LPM

0

PRFTEN

PDREQ1

0

Reset value

Res.

PDREQ2

Res.

SLEEP_PD

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

FLASH_ACR

0x00

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

Table 79. FLASH register map and reset values

RM0456 Rev 6

0

0x54

FLASH_
SECWM1R2

Reset value

X
NSBOOTADD0[24:0]

X

X

Reset value

X

X

X

X

X

X

X

X

X

X

X

X
X

X

X

X
X

X

X

X

X
X

X

X

X

X

RM0456 Rev 6
WWDG_SW
IWDG_STDBY
IWDG_STOP
IWDG_SW
SRAM_RST

X
X
X
X
X
X
X
X
X
X
X
X
X
X

NSBOOTADD1[24:0]

SECBOOTADD0[24:0]

SECWM1_PEND[7:0]
X
X
X

X
X
X
X

X

X

X

HDP1_PEND[7:0]
X
X
X
X
X

X

X

X

X

X

X

X

X
Res.

X

BOOT_LOCK

X
Res.

X
Res.

X

Res.

X

Res.

X
Res.

X X X X X X X X

Res.

PG

MER1
PER
PG

OPERR
EOP

Res.

0

Res.

ADDR_ECC[20:0]

PER

PNB[7:0]
MER1

WRPERR

SIZERR
PGAERR
PROGERR

PGSERR

Res.

Res.

Res.

Res.

Res.

Res.

Res.

0

Res.

BKER

Res.

BSY
Res.

WDW

Res.

Res.

Res.

Res.

Res.

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

BKER

Res.

Res.

BWR

BWR
Res.

STRT
MER2

STRT
MER2

Res.

Res.

Res.

Res.

Res.

OPTSTRT

Res.

Res.

Res.

Res.

Res.

Res.

Res.

EOPIE

Res.

OBL_LAUNCH

Res.

ERRIE

0

Res.

X

Res.

X

Res.

X

0

Res.

X

0

Res.

X

BOR_LEV[2:0]

X

0

Res.

BK_ECC

BK_OP

0

Res.

SYSF_ECC

Res.

EOPIE

ECCCIE
Res.

ERRIE

Res.

Res.

Res.

Res.

Res.

Res.

SYSF_OP

Res.

Res.

Res.

0

Res.

X

Res.

X
X

Res.

X
X

Res.

Reserved
X

Res.

X

Res.

X

Res.

X

Res.

Res.

SWAP_BANK

X
NRST_STOP

DUALBANK

X
NRST_SHDW

BKPRAM_ECC

X
0

0

0

Res.

X
X
X
X

0

PNB[7:0]

0

Res.

X
X
X
X

0

0

Res.

FLASH_
SECBOOTADD0R
X
X
X

NRST_STDBY

SRAM3_ECC

Reserved
0

0

0

Res.

X
X
X
x
0
0

0

Res.

X
X
0
0

0

Res.

FLASH_
NSBOOTADD1R
X

0

0

Res.

FLASH_
NSBOOTADD0R
X

0

0

Res.

X
X
X
0
0

0

Res.

X
X
X
0

0

Res.

X
X
0

Res.

SRAM2_ECC

X
0

Res.

SRAM2_RST
0

0

Res.

X
X
0

0

Res.

X
X
0

0

Res.

X
X
0

0

Res.

X
X
0

0

Res.

X
X
Res.

Reset value

Res.

X

Res.

Reset value
X
Res.

0
0

Res.

X

Res.

Reset value
NSWBOOT0

ECCC

0

Res.

ECCD

Reset value
Res.

FLASH_ECCR
INV

Res.

1

Res.

LOCK

Reset value

Res.

LOCK
OPTLOCK

FLASH_SECCR
0

Res.

X

FLASH_
SECWM1R1

Res.

Reset value
NBOOT0

FLASH_OPTR
PA15_PUPEN

Reset value

IO_VDD_HSLV

FLASH_OPSR
CODE_OP[2:0]
1

Res.

0x50

Reset value

Res.

0x4C
1

Res.

0x48

Res.

0x44

Res.

0x40
Reset value

Res.

0x380x3C
FLASH_NSCR

Res.

0x34

TZEN

0x30
FLASH_SECSR

IO_VDDIO2_HSLV

0x2C

Res.

0x28

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

0x24

HDP1EN

RM0456
Embedded flash memory (FLASH)

Table 79. FLASH register map and reset values (continued)

0
0

0
0
0

0
0
0
0
0
0
0
0

0
0
0
0
0
0
0
0

ADDR_OP[20:0]

X X X X X X X X

RDP[7:0]

X

SECWM1_PSTRT[7:0]

X X X X X X X X

X

<!-- pagebreak -->

363

Embedded flash memory (FLASH)

RM0456

0x70

X

X

X

WRP2B_PEND[7:0]
X

X

X

X

X

X

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

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

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

WRP2A_PSTRT[7:0]
X X X X X X X X

Res.

X

Res.

X

Res.

Res.

X

Res.

Res.
X

Res.

X

Res.

X

Res.

X

Res.

X

Res.

X

SECWM2_PSTRT[7:0]
X X X X X X X X

Res.

Res.

X

Res.

X

Res.

X

Res.

X

Res.

X

Res.

X

Res.

X

Res.

Res.

X

WRP1B_PSTRT[7:0]
X X X X X X X X

Res.

X

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
Res.
Res.
Res.
Res.

X

X X X X X X X X

Res.

Res.

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

X

WRP2B_PSTRT[7:0]
X X X X X X X X

OEM1KEY[31:0]
0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

OEM1KEY[63:32]
0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

OEM2KEY[31:0]
0

0

0

0

0

0

0

0

0

0

0

0

0

FLASH_
OEM2KEYR2

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

FLASH_
SECB1BRx

SEC1BB31

SEC1BB30

SEC1BB29

SEC1BB28

SEC1BB27

SEC1BB26

SEC1BB25

SEC1BB24

SEC1BB23

SEC1BB22

SEC1BB21

SEC1BB20

SEC1BB19

SEC1BB18

SEC1BB17

SEC1BB16

SEC1BB15

SEC1BB14

SEC1BB13

SEC1BB12

SEC1BB11

SEC1BB10

SEC1BB9

SEC1BB8

SEC1BB7

SEC1BB6

SEC1BB5

SEC1BB4

SEC1BB3

SEC1BB2

SEC1BB1

SEC1BB0

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

FLASH_
SECBB2Rx

SEC2BB30

SEC2BB29

SEC2BB28

SEC2BB27

SEC2BB26

SEC2BB25

SEC2BB24

SEC2BB23

SEC2BB22

SEC2BB21

SEC2BB20

SEC2BB19

SEC2BB18

SEC2BB17

SEC2BB16

SEC2BB15

SEC2BB14

SEC2BB13

SEC2BB12

SEC2BB11

SEC2BB10

SEC2BB9

SEC2BB8

SEC2BB7

SEC2BB6

SEC2BB5

SEC2BB4

SEC2BB3

SEC2BB2

SEC2BB1

SEC2BB0

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

FLASH_
SECHDPCR

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

HDP2_ACCDIS

HDP1_ACCDIS

OEM2KEY[63:32]

0

0

Reset value

<!-- pagebreak -->

X

SEC2BB31

0xC0

X

Res.

0xA0 +
0x4 *
(x - 1) (x
= 1 to 8)
Last
address:
0xBC

X

WRP1A_PSTRT[7:0]

Res.

0x80 +
0x4 *
(x - 1) (x
= 1 to 8)
Last
address:
0x9C

X

FLASH_
OEM2KEYR1
Reset value

0x7C

X

FLASH_
OEM1KEYR2
Reset value

0x78

X

FLASH_
OEM1KEYR1
Reset value

0x74

Res.

X

Res.

Reset value

Res.

FLASH_WRP2BR

X

WRP2A_PEND[7:0]
X

Res.

0x6C

Res.

X
Res.

Reset value

X

HDP2_PEND[7:0]
X

Res.

FLASH_WRP2AR

UNLOCK

0x68

Res.

HDP2EN
X

Res.

Reset value

Res.

FLASH_
SECWM2R2

0x64

X

SECWM2_PEND[7:0]
X

UNLOCK

Reset value

X

WRP1B_PEND[7:0]
X

Res.

0x60

Res.

X

FLASH_
SECWM2R1

Res.

Reset value

WRP1A_PEND[7:0]
X

Res.

FLASH_WRP1BR

Res.

UNLOCK
X

Res.

Reset value

UNLOCK

0x5C

FLASH_WRP1AR

Res.

0x58

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

Table 79. FLASH register map and reset values (continued)

RM0456 Rev 6

PRIV1BB29
PRIV1BB28
PRIV1BB27
PRIV1BB26
PRIV1BB25
PRIV1BB24
PRIV1BB23
PRIV1BB22
PRIV1BB21
PRIV1BB20
PRIV1BB19
PRIV1BB18
PRIV1BB17
PRIV1BB16
PRIV1BB15
PRIV1BB14
PRIV1BB13
PRIV1BB12
PRIV1BB11
PRIV1BB10
PRIV1BB9
PRIV1BB8
PRIV1BB7
PRIV1BB6
PRIV1BB5
PRIV1BB4
PRIV1BB3
PRIV1BB2
PRIV1BB1
PRIV1BB0

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

FLASH_
PRIVBB2Rx
PRIV2BB29
PRIV2BB28
PRIV2BB27
PRIV2BB26
PRIV2BB25
PRIV2BB24
PRIV2BB23
PRIV2BB22
PRIV2BB21
PRIV2BB20
PRIV2BB19
PRIV2BB18
PRIV2BB17
PRIV2BB16
PRIV2BB15
PRIV2BB14
PRIV2BB13
PRIV2BB12
PRIV2BB11
PRIV2BB10
PRIV2BB9
PRIV2BB8
PRIV2BB7
PRIV2BB6
PRIV2BB5
PRIV2BB4
PRIV2BB3
PRIV2BB2
PRIV2BB1
PRIV2BB0

0xF0 +
0x4 *
(x - 1) (x
= 1 to 8)
Last
address:
0x10C
FLASH_
PRIVBB1Rx
PRIV1BB30

0xD0 +
0x4 *
(x - 1) (x
= 1 to 8)
Last
address:
0xEC
PRIV1BB31

0xC8

PRIV2BB30

0xC4

PRIV2BB31

FLASH_
PRIVCFGR

Reserved

RM0456 Rev 6

SPRIV

Reset value
NSPRIV

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

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

Offset Register name
Res.

RM0456
Embedded flash memory (FLASH)

Table 79. FLASH register map and reset values (continued)

Reserved
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

Refer to Section 2.3 for the register boundary addresses.

<!-- pagebreak -->

