642

General-purpose I/Os (GPIO)

13.4.9

RM0456

GPIO alternate function low register (GPIOx_AFRL) (x = A to J)
Address offset: 0x20
Reset value: 0x0000 0000

31

30

29

28

27

AFSEL7[3:0]

26

25

24

23

AFSEL6[3:0]

22

21

20

19

AFSEL5[3:0]

18

17

16

AFSEL4[3:0]

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

rw

rw

rw

rw

rw

rw

AFSEL3[3:0]
rw

rw

rw

AFSEL2[3:0]
rw

rw

AFSEL1[3:0]
rw

rw

AFSEL0[3:0]
rw

rw

rw

Bits 31:0 AFSELy[3:0]: Alternate function selection for port x I/O pin y (y = 7 to 0)
These bits are written by software to configure alternate function I/Os.
0000: AF0
0001: AF1
0010: AF2
0011: AF3
0100: AF4
0101: AF5
0110: AF6
0111: AF7
1000: AF8
1001: AF9
1010: AF10
1011: AF11
1100: AF12
1101: AF13
1110: AF14
1111: AF15
Note: This field is reserved and must be kept at reset value when the corresponding I/O is not
available on the selected package.

13.4.10

GPIO alternate function high register (GPIOx_AFRH) (x = A to J)
Address offset: 0x24
Reset value: 0x0000 0000

31

30

29

28

27

AFSEL15[3:0]
rw
15

rw

rw

rw

rw

14

13

12

11

AFSEL11[3:0]
rw

<!-- pagebreak -->

rw

rw

26

25

24

23

AFSEL14[3:0]
rw

rw

rw

rw

10

9

8

7

AFSEL10[3:0]
rw

rw

rw

rw

22

21

20

19

AFSEL13[3:0]
rw

rw

rw

rw

6

5

4

3

AFSEL9[3:0]
rw

rw

RM0456 Rev 6

rw

rw

18

17

16

AFSEL12[3:0]
rw

rw

rw

2

1

0

AFSEL8[3:0]
rw

rw

rw

rw

rw

RM0456

General-purpose I/Os (GPIO)

Bits 31:0 AFSELy[3:0]: Alternate function selection for port x I/O pin y (y = 15 to 8)
These bits are written by the software to configure alternate function I/Os.
0000: AF0
0001: AF1
0010: AF2
0011: AF3
0100: AF4
0101: AF5
0110: AF6
0111: AF7
1000: AF8
1001: AF9
1010: AF10
1011: AF11
1100: AF12
1101: AF13
1110: AF14
1111: AF15
Note: This field is reserved and must be kept at reset value when the corresponding I/O is
not available on the selected package.

13.4.11

GPIO port bit reset register (GPIOx_BRR) (x = A to J)
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
Bits 15:0 BRy: Port x reset IO pin y (y = 15 to 0)
These bits are write-only. A read to these bits returns the value 0x0000.
0: No action on the corresponding ODy bit
1: Reset the corresponding ODy bit
Note: This bit is reserved and must be kept at reset value when the corresponding I/O is not
available on the selected package.

RM0456 Rev 6

<!-- pagebreak -->

642

General-purpose I/Os (GPIO)

13.4.12

RM0456

GPIO high-speed low-voltage register (GPIOx_HSLVR) (x = A to J)
Address offset: 0x2C
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

HSLV
15

HSLV
14

HSLV
13

HSLV
12

HSLV
11

HSLV
10

HSLV9

HSLV8

HSLV7

HSLV6

HSLV5

HSLV4

HSLV3

HSLV2

HSLV1

HSLV0

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
Bits 15:0 HSLVy: Port x high-speed low-voltage configuration (y = 15 to 0)
These bits are written by software to optimize the I/O speed when the I/O supply is low.
Each bit is active only if the corresponding IO_VDD_HSLV/IO_VDDIO2_HSLV user option bit
is set. It must be used only if the I/O supply voltage is below 2.7 V.
Setting these bits when the I/O supply (VDD or VDDIO2) is higher than 2.7 V may be
destructive. Setting this bit when the I/O is configured in Fm+ mode is forbidden.
0: I/O speed optimization disabled
1: I/O speed optimization enabled
Note: Not all I/Os support the HSLV mode. Refer to the I/O structure in the corresponding
datasheet for the list of I/Os supporting this feature. Other I/Os HSLV configuration must
be kept at reset value.
This bit is reserved and must be kept at reset value when the corresponding I/O is not
available on the selected package.

<!-- pagebreak -->

