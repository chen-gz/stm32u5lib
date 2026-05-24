1886

JPEG codec (JPEG)

46.4

RM0456

JPEG codec interrupts
An interrupt can be produced on the following events:
•

input FIFO threshold reached

•

input FIFO not full

•

output FIFO threshold reached

•

output FIFO not empty

•

end of conversion

•

header parsing done

Separate interrupt enable bits are available for flexibility.
Table 454. JPEG codec interrupt requests
Interrupt event

Event flag

Enable Control bit

Input FIFO threshold reached

IFTF

IFTIE

Input FIFO not full

IFNFF

IFNFIE

Output FIFO threshold reached

OFTF

OFTIE

Output FIFO not empty

OFNEF

OFNEIE

End of conversion

EOCF

EOCIE

Header parsing done

HPDF

HPDIE

46.5

JPEG codec registers

46.5.1

JPEG codec control register (JPEG_CONFR0)
Address offset: 0x000
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

Res.

START
w

Bits 31:1 Reserved, must be kept at reset value.
Bit 0 START: Start
This bit start or stop the encoding or decoding process.
0: Stop/abort
1: Start
Note: Reads always return 0.

<!-- pagebreak -->

