1886

JPEG codec (JPEG)

46.5.3

RM0456

JPEG codec configuration register 2 (JPEG_CONFR2)
Address offset: 0x008
Reset value: 0x0000 0000

31

30

29

28

27

26

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

NMCU[25:16]
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

rw

NMCU[15:0]
rw

rw

rw

rw

rw

rw

rw

rw

rw

Bits 31:26 Reserved, must be kept at reset value.
Bits 25:0 NMCU[25:0]: Number of MCUs
For encoding: this field defines the number of MCU units minus 1 to encode.
For decoding: this field indicates the number of complete MCU units minus 1 to be decoded
(this field is updated after the JPEG header parsing). If the decoded image size has not a X or
Y size multiple of 8 or 16 (depending on the sub-sampling process), the resulting incomplete
or empty MCU must be added to this value to get the total number of MCUs generated.

46.5.4

JPEG codec configuration register 3 (JPEG_CONFR3)
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

XSIZE[15:0]
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

Bits 31:16 XSIZE[15:0]: X size
This field defines the number of pixels per line.
Bits 15:0 Reserved, must be kept at reset value.

<!-- pagebreak -->

