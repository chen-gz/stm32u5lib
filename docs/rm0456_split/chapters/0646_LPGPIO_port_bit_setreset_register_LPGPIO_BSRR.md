647

Low-power general-purpose I/Os (LPGPIO)

14.4.4

RM0456

LPGPIO port bit set/reset register (LPGPIO_BSRR)
Address offset: 0x18
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

BR15

BR14

BR13

BR12

BR11

BR10

BR9

BR8

BR7

BR6

BR5

BR4

BR3

BR2

BR1

BR0

w

w

w

w

w

w

w

w

w

w

w

w

w

w

w

w

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

BS15

BS14

BS13

BS12

BS11

BS10

BS9

BS8

BS7

BS6

BS5

BS4

BS3

BS2

BS1

BS0

w

w

w

w

w

w

w

w

w

w

w

w

w

w

w

w

Bits 31:16 BRy: Reset I/O pin y (y = 15 to 0)
These bits are write-only. A read to these bits returns zero.
0: No action on the corresponding ODy bit
1: Reset the corresponding ODy bit.
Note: If both BSy and BRy are set, BSy has priority.
Bits 15:0 BSy: Port x set I/O pin y (y = 15 to 0)
These bits are write-only. A read to these bits returns zero.
0: No action on the corresponding ODy bit
1: Set the corresponding ODy bit.

14.4.5

LPGPIO port bit reset register (LPGPIO_BRR)
Address offset: 0x28
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

BR15

BR14

BR13

BR12

BR11

BR10

BR9

BR8

BR7

BR6

BR5

BR4

BR3

BR2

BR1

BR0

w

w

w

w

w

w

w

w

w

w

w

w

w

w

w

w

Bits 31:16 Reserved, must be kept at reset value.
Bits 15:0 BRy: Reset I/O pin y (y = 15 to 0)
These bits are write-only. A read to these bits returns zero.
0: No action on the corresponding ODy bit
1: Reset the corresponding ODy bit.

<!-- pagebreak -->

