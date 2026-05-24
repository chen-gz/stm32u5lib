r

RM0456 Rev 6

RM0456

Hash processor (HASH)

Bits 31:0 Hx[31:0]: Hash data x
Refer to Section 51.6.4: HASH digest registers introduction.

HASH digest register x (HASH_HRx)
Address offset: 0x310 + 0x4 * x, (x = 0 to 4)
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

r

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

r

r

r

r

r

r

r

19

18

17

16

Hx[31:16]
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

Hx[15:0]
r

r

r

r

r

r

r

r

r

Bits 31:0 Hx[31:0]: Hash data x
Refer to Section 51.6.4: HASH digest registers introduction.

HASH supplementary digest register x (HASH_HRx)
Address offset: 0x310 + 0x4 * x, (x = 5 to 7)
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

Hx[31:16]

Hx[15:0]
r

r

r

r

r

r

r

r

r

Bits 31:0 Hx[31:0]: Hash data x
Refer to Section 51.6.4: HASH digest registers introduction.

51.6.5

HASH interrupt enable register (HASH_IMR)
Address offset: 0x20
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

Res.

Res.

Res.

DCIE

DINIE

rw

rw

RM0456 Rev 6

<!-- pagebreak -->

