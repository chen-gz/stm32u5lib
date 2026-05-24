2037

Hash processor (HASH)

51.6.8

RM0456

HASH register map

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

0

0

0

0

0

0

0

0

0

0

0

HASH_STR

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

DCAL

Res.

Res.

Res.

0

HASH_HRA0

0

0

NBLW[4:0]
0

0

0

0

0

H0[31:0]
0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

HASH_HRA1

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

H1[31:0]
0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

HASH_HRA2

0

H2[31:0]
0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

HASH_HRA3
Reset value

2

0

Reset value

Res.

3

0

0

Reset value

INIT

0

0

Reset value

4

DMAE
0

Reset value

0

H3[31:0]
0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

HASH_IMR

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

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

0

0

0

0

0

0

0

0

0

0

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

Res.

0

Res.

DINIS

0

DCIS

0

DMAS

0

Res.

0

CS0[31:0]
0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

HASH_CSRx
Reset value

0

0

Reserved

HASH_CSR0
Reset value

0

0
BUSY

Reserved

0

Res.

DINNE
0

NBWP0

NBWE0
0

NBWP1

NBWE1
0

Res.

NBWE2
0

NBWP2

0x0F8 +
0x4 * x,
(x=1 to 53)
Last address:
0x1CC

0

NBWP3

0x0F8

0

NBWP4

0x28- 0xF4

NBWE3

Reset value

NBWE4

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

HASH_SR

Res.

0x24

Res.

Reset value

DINIE

0

DCIE

Reset value

Res.

H4[31:0]

Res.

HASH_HRA4

0

Res.

0x20

0

Res.

0x1C

0

Res.

0x18

0

Res.

0x14

0

Res.

0x10

0

DATAIN[31:16]

Reset value
0x0C

5

0

6

0

DATATYPE

0

7

0

MODE

8

0

Res.

0

Res.

0x08

HASH_DIN

9

0

NBW[3:0]

12

0

Res.

0x04

11

13

DINNE

0

Reset value

10

14

.MDMAT

20
Res.

15

21
Res.

Res.

22
Res.

16

23
Res.

Res.

24
Res.

17

25
Res.

LKEY

26
Res.

18

27
Res.

ALGO0

28
Res.

19

29
Res.

ALGO[1]

30

HASH_CR

Res.

31

0x00

Register name

Res.

Offset

Res.

Table 489. HASH register map and reset values

0

0

0

CSx[31:0]
0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

...
0x1D00x30C
0x310

0x314

<!-- pagebreak -->

Reserved

Reserved

HASH_HR0
Reset value

H0[31:0]
0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

HASH_HR1
Reset value

0

0

H1[31:0]
0

0

0

0

0

0

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

0

RM0456

Hash processor (HASH)

8

7

6

5

4

3

2

1

0

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

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

H3[31:0]
0

0

H4[31:0]
0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

H5[31:0]
0

0

H6[31:0]
0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

HASH_HR7
Reset value

9

0x32C

0

HASH_HR6
Reset value

11

0x328

0

HASH_HR5
Reset value

10

0x324

0

HASH_HR4
Reset value

0

H2[31:0]

HASH_HR3
Reset value

12

0x320

Reset value

13

0x31C

HASH_HR2

14

0x318

Register name

30

Offset

31

Table 489. HASH register map and reset values (continued)

0

0

H7[31:0]
0

0

Refer to Section 2.3: Memory organization for the register boundary addresses.

RM0456 Rev 6

<!-- pagebreak -->

