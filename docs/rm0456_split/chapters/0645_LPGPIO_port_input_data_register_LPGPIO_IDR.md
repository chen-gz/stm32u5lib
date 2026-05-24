RM0456 Rev 6

RM0456

Low-power general-purpose I/Os (LPGPIO)

Bits 31:16 Reserved, must be kept at reset value.
Bits 15:0 MODEy: Configuration I/O pin y (y = 15 to 0)
These bits are written by software to configure the I/O mode.
0: Input mode
1: Output mode

14.4.2

LPGPIO port input data register (LPGPIO_IDR)
Address offset: 0x10
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

ID15

ID14

ID13

ID12

ID11

ID10

ID9

ID8

ID7

ID6

ID5

ID4

ID3

ID2

ID1

ID0

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

Bits 31:16 Reserved, must be kept at reset value.
Bits 15:0 IDy: Input data I/O pin y (y = 15 to 0)
These bits are read-only. They contain the input value of the corresponding I/O port.

14.4.3

LPGPIO port output data register (LPGPIO_ODR)
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
Bits 15:0 ODy: Output data I/O pin y (y = 15 to 0)
These bits can be read and written by software.
Note: For atomic bit set/reset, these OD bits can be individually set and/or reset by writing to
the LPGPIO_BSRR or LPGPIO_BRR registers.

RM0456 Rev 6

<!-- pagebreak -->

