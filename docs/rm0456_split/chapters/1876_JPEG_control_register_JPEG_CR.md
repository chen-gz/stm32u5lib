1886

JPEG codec (JPEG)

46.5.6

RM0456

JPEG control register (JPEG_CR)
Address offset: 0x030
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

OFF

IFF

Res.

Res.

Res.

Res.

HPDIE

EOCIE

OFNEIE

OFTIE

IFNFIE

IFTIE

JCEN

w

w

rw

rw

rw

rw

rw

rw

rw

ODMAEN IDMAEN
rw

rw

Bits 31:15 Reserved, must be kept at reset value.
Bit 14 OFF: Output FIFO flush
This bit flushes the output FIFO.
0: No effect
1: Output FIFO is flushed
Note: Reads always return 0.
Bit 13 IFF: Input FIFO flush
This bit flushes the input FIFO.
0: No effect
1: Input FIFO is flushed
Note: Reads always return 0.
Bit 12 ODMAEN: Output DMA enable
Enables DMA request generation for the output FIFO.
0: Disabled
1: Enabled
Bit 11 IDMAEN: Input DMA enable
Enables DMA request generation for the input FIFO.
0: Disabled
1: Enabled
Bits 10:7 Reserved, must be kept at reset value.
Bit 6 HPDIE: Header parsing done interrupt enable
This bit enables interrupt generation upon the completion of the header parsing operation.
0: Disabled
1: Enabled
Bit 5 EOCIE: End of conversion interrupt enable
This bit enables interrupt generation at the end of conversion.
0: Disabled
1: Enabled
Bit 4 OFNEIE: Output FIFO not empty interrupt enable
This bit enables interrupt generation when the output FIFO is not empty.
0: Disabled
1: Enabled

<!-- pagebreak -->

