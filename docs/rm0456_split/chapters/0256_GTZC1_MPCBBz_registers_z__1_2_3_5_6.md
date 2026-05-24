275

Global TrustZone controller (GTZC)

RM0456

CGPDMA1F

0

CFLASHF

0

CFLASH_REGF

COTFDEC1F

Res.

COTFDEC2F

Res.

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

CTZIC1F

0

CTZSC1F

0

COCTOSPI1_MEMFE

0

CBKPSRAMIF

0

CFSMC_MEMFE

0

COCTOSPI2_MEMIF

0

Res.

0

CHSPI1_MEMIF

0

CSRAM6F

0

CSRAM1F

CMPCBB3_REGF

0

CMPCBB6_REGF

CSRAM5F

Reset value

CSRAM2F

GTZC1_TZIC_
FCR4

CMPCBB1_REGF

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

0x02C

CSRAM3F

Register name

CMPCBB2_REGF

Offset

CMPCBB5_REGF

Table 40. GTZC1 TZIC register map and reset values (continued)

0

0

0

Refer to Table 29: GTZC1 subblocks address offset.

5.8

GTZC1 MPCBBz registers (z = 1, 2, 3, 5, 6)
All registers are accessed only by words (32-bit).

5.8.1

GTZC1 SRAMz MPCBB control register
(GTZC1_MPCBBz_CR) (z = 1, 2, 3, 5, 6)
Address offset: 0x000
Reset value: 0x0000 0000
Secure privileged access only.

Note:

Some registers are only available on some devices in the STM32U5 series. Refer to the
device datasheet for availability of its associated memory region.

31

30

INVSE
SRWIL
CSTAT
ADIS
E
rw

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

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

GLOCK
rs

Bit 31 SRWILADIS: secure read/write illegal access disable
This bit disables the detection of an illegal access when a secure read/write transaction
access a nonsecure blocks of the block-based SRAM (secure fetch on nonsecure block is
always considered illegal).
0: enabled, secure read/write access not allowed on nonsecure SRAM block
1: disabled, secure read/write access allowed on nonsecure SRAM block
Bit 30 INVSECSTATE: SRAMx clocks security state
This bit is used to define the internal SRAMs clocks control in RCC as secure or not.
0: SRAMs clocks are secure if a secure area exists in the MPCBB. It is nonsecure if there is
no secure area.
1: SRAMs clocks are nonsecure even if a secure area exists in the MPCBB, and secure
even if no secure block is set in the MPCBB.
Bits 29:1 Reserved, must be kept at reset value.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Global TrustZone controller (GTZC)

Bit 0 GLOCK: lock the control register of the MPCBB until next reset
This bit is cleared by default and once set, it can not be reset until system reset.
0: control register not locked
1: control register locked

5.8.2

GTZC1 SRAMz MPCBB configuration lock register 1
(GTZC1_MPCBBz_CFGLOCKR1) (z = 1, 2, 3, 5, 6)
Address offset: 0x010
Reset value: 0x0000 0000
Secure privileged access only.

Note:

31

Some registers are only available on some devices in the STM32U5 series. Refer to the
device datasheet for availability of its associated memory region.
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

SPLCK SPLCK SPLCK SPLCK SPLCK SPLCK SPLCK SPLCK SPLCK SPLCK SPLCK SPLCK SPLCK SPLCK SPLCK SPLCK
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
rs

rs

rs

rs

rs

rs

rs

rs

rs

rs

rs

rs

rs

rs

rs

rs

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

SPLCK SPLCK SPLCK SPLCK SPLCK SPLCK SPLCK SPLCK SPLCK SPLCK SPLCK SPLCK SPLCK SPLCK SPLCK SPLCK
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
rs

rs

rs

rs

rs

rs

rs

rs

rs

rs

rs

rs

rs

rs

rs

rs

.

Bits 31:0 SPLCKy: Security/privilege configuration lock for super-block (y = 31 to 0)
This bit is set by software and can be cleared only by system reset.
0: GTZC1_MPCBBz_SECCFGRy and GTZC1_MPCBBz_PRIVCFGRy can be written.
1: Writes to GTZC1_MPCBBz_SECCFGRy and GTZC1_MPCBBz_PRIVCFGRy are ignored

5.8.3

GTZC1 SRAMz MPCBB configuration lock register 2
(GTZC1_MPCBBz_CFGLOCKR2) (z = 1, 2, 3, 5, 6)
Address offset: 0x014
Reset value: 0x0000 0000
Secure privileged access only.

Note:

31
Res.

15

Some registers are only available on some devices in the STM32U5 series. Refer to the
device datasheet for availability of its associated memory region.
30

29

Res.

Res.

14

13

28
Res.

12

27
Res.

11

26
Res.

10

25
Res.

9

24
Res.

23
Res.

8

7

22
Res.

6

21
Res.

5

20
Res.

4

19

18

17

16

SPLCK SPLCK SPLCK SPLCK
51
50
49
48
rs

rs

rs

rs

3

2

1

0

SPLCK SPLCK SPLCK SPLCK SPLCK SPLCK SPLCK SPLCK SPLCK SPLCK SPLCK SPLCK SPLCK SPLCK SPLCK SPLCK
47
46
45
44
43
42
41
40
39
38
37
36
35
34
33
32
rs

rs

rs

rs

rs

rs

rs

rs

rs

rs

rs

rs

rs

rs

rs

rs

.

Bits 31:20 Reserved, must be kept at reset value.

RM0456 Rev 6

<!-- pagebreak -->

