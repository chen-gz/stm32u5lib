1104

Octo-SPI interface (OCTOSPI)

RM0456

Bit 0 TEF: Transfer error flag
This bit is set in indirect mode when an invalid address is being accessed in indirect mode.
It is cleared by writing 1 to CTEF.

28.7.7

OCTOSPI flag clear register (OCTOSPI_FCR)
Address offset: 0x0024
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

CTOF

CSMF

Res.

CTCF

CTEF

w

w

w

w

Bits 31:5 Reserved, must be kept at reset value.
Bit 4 CTOF: Clear timeout flag
Writing 1 clears the TOF flag in the OCTOSPI_SR register.
Bit 3 CSMF: Clear status match flag
Writing 1 clears the SMF flag in the OCTOSPI_SR register.
Bit 2 Reserved, must be kept at reset value.
Bit 1 CTCF: Clear transfer complete flag
Writing 1 clears the TCF flag in the OCTOSPI_SR register.
Bit 0 CTEF: Clear transfer error flag
Writing 1 clears the TEF flag in the OCTOSPI_SR register.

28.7.8

OCTOSPI data length register (OCTOSPI_DLR)
Address offset: 0x0040
Reset value: 0x0000 0000
This register can be modified only when BUSY = 0.

31

30

29

28

27

26

25

24

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

23

22

21

20

19

18

17

16

rw

rw

rw

rw

rw

rw

rw

rw

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

DL[31:16]

DL[15:0]
rw

rw

<!-- pagebreak -->

