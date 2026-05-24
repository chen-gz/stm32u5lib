400

Data cache (DCACHE)

9.7.6

RM0456

DCACHE read-miss monitor register (DCACHE_RMMONR)
Address offset: 0x014
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

RMISSMON[15:0]
r

r

Bits 31:16 Reserved, must be kept at reset value.
Bits 15:0 RMISSMON[15:0]: cache read-miss monitor counter

9.7.7

DCACHE write-hit monitor register (DCACHE_WHMONR)
Address offset: 0x020
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

WHITMON[31:16]
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

r

r

r

r

r

r

r

r

r

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

WHITMON[15:0]
r

r

r

r

r

r

r

r

r

Bits 31:0 WHITMON[31:0]: cache write-hit monitor counter

9.7.8

DCACHE write-miss monitor register (DCACHE_WMMONR)
Address offset: 0x024
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

r

r

r

r

r

r

r

WMISSMON[15:0]
r

r

r

r

r

r

r

r

r

Bits 31:16 Reserved, must be kept at reset value.
Bits 15:0 WMISSMON[15:0]: cache write-miss monitor counter

<!-- pagebreak -->

