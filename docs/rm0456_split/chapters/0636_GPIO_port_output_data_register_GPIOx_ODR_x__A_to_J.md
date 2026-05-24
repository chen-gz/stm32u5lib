642

General-purpose I/Os (GPIO)

13.4.6

RM0456

GPIO port output data register (GPIOx_ODR) (x = A to J)
Address offset: 0x14
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

OD15

OD14

OD13

OD12

OD11

OD10

OD9

OD8

OD7

OD6

OD5

OD4

OD3

OD2

OD1

OD0

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

Bits 31:16 Reserved, must be kept at reset value.
Bits 15:0 ODy: Port output data I/O pin y (y = 15 to 0)
These bits can be read and written by software.
Note: For atomic bit set/reset, these bits can be individually set and/or reset by writing
to GPIOx_BSRR or GPIOx_BRR (x = A to J).
This bit is reserved and must be kept at reset value when the corresponding I/O is not
available on the selected package.

13.4.7

GPIO port bit set/reset register (GPIOx_BSRR) (x = A to J)
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

Bits 31:16 BRy: Port x reset I/O pin y (y = 15 to 0)
These bits are write-only. A read to these bits returns the value 0x0000.
0: No action on the corresponding ODy bit
1: Resets the corresponding ODy bit
Note: If both BSy and BRy are set, BSy has priority.
This bit is reserved and must be kept at reset value when the corresponding I/O is not
available on the selected package.
Bits 15:0 BSy: Port x set I/O pin y (y = 15 to 0)
These bits are write-only. A read to these bits returns the value 0x0000.
0: No action on the corresponding ODy bit
1: Sets the corresponding ODy bit
Note: The bit is reserved and must be kept at reset value when the corresponding I/O is not
available on the selected package.

<!-- pagebreak -->

