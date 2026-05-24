RM0456 Rev 6

RM0456

JPEG codec (JPEG)

Bit 3 OFTIE: Output FIFO threshold interrupt enable
This bit enables interrupt generation when the output FIFO reaches a threshold.
0: Disabled
1: Enabled
Bit 2 IFNFIE: Input FIFO not full interrupt enable
This bit enables interrupt generation when the input FIFO is not empty.
0: Disabled
1: Enabled
Bit 1 IFTIE: Input FIFO threshold interrupt enable
This bit enables interrupt generation when the input FIFO reaches a threshold.
0: Disabled
1: Enabled
Bit 0 JCEN: JPEG core enable
This bit enables the JPEG codec core.
0: Disabled (internal registers are reset).
1: Enabled (internal registers are accessible).

46.5.7

JPEG status register (JPEG_SR)
Address offset: 0x034
Reset value: 0x0000 0006

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

COF

HPDF

EOCF

OFNEF

OFTF

IFNFF

IFTF

Res.

r

r

r

r

r

r

r

Bits 31:8 Reserved, must be kept at reset value.
Bit 7 COF: Codec operation flag
This bit flags code/decode operation in progress.
0: Not in progress
1: In progress
Bit 6 HPDF: Header parsing done flag
In decode mode, this bit flags the completion of header parsing and updating internal
registers.
0: Not completed
1: Completed
Bit 5 EOCF: End of conversion flag
This bit flags the completion of encode/decode process and data transfer to the output FIFO.
0: Not completed
1: Completed

RM0456 Rev 6

<!-- pagebreak -->

