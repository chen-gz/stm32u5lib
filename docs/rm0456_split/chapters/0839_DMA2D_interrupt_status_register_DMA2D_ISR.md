RM0456 Rev 6

16

MODE[2:0]
rw

RM0456

Chrom-ART Accelerator controller (DMA2D)

Bit 9 TCIE: Transfer complete interrupt enable
This bit is set and cleared by software.
0: TC interrupt disabled
1: TC interrupt enabled
Bit 8 TEIE: Transfer error interrupt enable
This bit is set and cleared by software.
0: TE interrupt disabled
1: TE interrupt enabled
Bit 7 Reserved, must be kept at reset value.
Bit 6 LOM: Line offset mode
This bit configures how is expressed the line offset (pixels or bytes) for the foreground,
background and output.
This bit is set and cleared by software. It can not be modified while a transfer is ongoing.
0: Line offsets expressed in pixels
1: Line offsets expressed in bytes
Bits 5:3 Reserved, must be kept at reset value.
Bit 2 ABORT: Abort
This bit can be used to abort the current transfer. This bit is set by software and is
automatically reset by hardware when the START bit is reset.
0: No transfer abort requested
1: Transfer abort requested
Bit 1 SUSP: Suspend
This bit can be used to suspend the current transfer. This bit is set and reset by software. It is
automatically reset by hardware when the START bit is reset.
0: Transfer not suspended
1: Transfer suspended
Bit 0 START: Start
This bit can be used to launch the DMA2D according to the parameters loaded in the various
configuration registers. This bit is automatically reset by the following events:
- at the end of the transfer
- when the data transfer is aborted by the user application (setting ABORT in DMA2D_CR)
- when a data transfer error occurs
- when the data transfer has not started due to a configuration error or another transfer
operation already ongoing (automatic CLUT loading).

19.5.2

DMA2D interrupt status register (DMA2D_ISR)
Address offset: 0x004
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

CEIF

CTCIF

CAEIF

TWIF

TCIF

TEIF

r

r

r

r

r

r

RM0456 Rev 6

<!-- pagebreak -->

