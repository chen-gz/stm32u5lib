RM0456 Rev 6

RM0456

Data cache (DCACHE)

9.7.4

DCACHE flag clear register (DCACHE_FCR)
Address offset: 0x00C
Reset value: 0x0000 0000

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

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

CCMD
ENDF

Res.

CERRF

CBSYE
NDF

Res.

w

w

18

17

w

Bits 31:5 Reserved, must be kept at reset value.
Bit 4 CCMDENDF: clear command end flag
Set by software.
0: no effect
1: clears CMDENDF flag in DCACHE_SR
Bit 3 Reserved, must be kept at reset value.
Bit 2 CERRF: clear cache error flag
Set by software.
0: no effect
1: clears ERRF flag in DCACHE_SR
Bit 1 CBSYENDF: clear full invalidate busy end flag
Set by software.
0: no effect
1: clears BSYENDF flag in DCACHE_SR
Bit 0 Reserved, must be kept at reset value.

9.7.5

DCACHE read-hit monitor register (DCACHE_RHMONR)
Address offset: 0x010
Reset value: 0x0000 0000

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

16

RHITMON[31:16]
r

r

r

r

r

r

r

r

r

r

r

r

r

r

r

r

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

r

r

r

r

r

r

r

RHITMON[15:0]
r

r

Bits 31:0 RHITMON[31:0]: cache read-hit monitor counter

RM0456 Rev 6

<!-- pagebreak -->

