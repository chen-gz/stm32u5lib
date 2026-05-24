RM0456 Rev 6

RM0456

Voltage reference buffer (VREFBUF)

36.5.2

VREFBUF calibration control register (VREFBUF_CCR)
Address offset: 0x04
Reset value: 0x0000 00XX

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
rw

rw

rw

rw

rw

TRIM[5:0]
rw

Bits 31:6 Reserved, must be kept at reset value.
Bits 5:0 TRIM[5:0]: Trimming code
The TRIM code is a 6-bit unsigned data (minimum 000000, maximum 111111) that is set and
updated according the mechanism described below.
Reset:
TRIM[5:0] is automatically initialized with the VRS = 0 trimming value stored in the flash
memory during the production test.
VRS change:
TRIM[5:0] is automatically initialized with the trimming value (corresponding to VRS setting)
stored in the flash memory during the production test.
Write in TRIM[5:0]:
User can modify the TRIM[5:0] with an arbitrary value. This is permanently disabling the
control of the trimming value with VRS (until the device is reset).
Note: If the user application performs the trimming, the trimming code must start from 000000
to 111111 in ascending order.

36.5.3

VREFBUF register map
The following table gives the VREFBUF register map and the reset values.

Res.

Res.

Res.

Res.

Res.

Res.

1

0
ENVR

12
Res.
Res.

2

13
Res.
Res.

HIZ

14
Res.
Res.

3

15
Res.
Res.

0

Res.

16
Res.
Res.

4

17
Res.
Res.

0

VRR

18
Res.
Res.

Res.

Reset value

5

19
Res.

6

20
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

VREFBUF_CCR

Res.

0x04

0
Res.

Reset value

VRS[2:0]

21
Res.

7

22
Res.

8

23
Res.

Res.

24
Res.

9

25
Res.

Res.

26
Res.

Res.

27
Res.

11

28
Res.

10

29
Res.

Res.

30

VREFBUF_CSR

Res.

31

0x00

Register name

Res.

Offset

Res.

Table 353. VREFBUF register map and reset values

1

0

0

TRIM[5:0]
x

x

x

x

x

x

Refer to Section 2.3: Memory organization for the register boundary addresses.

RM0456 Rev 6

<!-- pagebreak -->

