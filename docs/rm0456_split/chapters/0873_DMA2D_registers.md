Event flag

Enable control bit

Configuration error

CEIF

CEIE

CLUT transfer complete

CTCIF

CTCIE

RM0456 Rev 6

RM0456

Chrom-ART Accelerator controller (DMA2D)
Table 182. DMA2D interrupt requests (continued)
Interrupt event

Event flag

Enable control bit

CLUT access error

CAEIF

CAEIE

Transfer watermark

TWF

TWIE

Transfer complete

TCIF

TCIE

Transfer error

TEIF

TEIE

20.5

DMA2D registers

20.5.1

DMA2D control register (DMA2D_CR)
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

CEIE

CTCIE

CAEIE

TWIE

TCIE

TEIE

Res.

LOM

Res.

Res.

Res.

ABORT

SUSP

START

rw

rw

rw

rw

rw

rw

rs

rw

rs

rw

18

17

16

MODE[2:0]

Bits 31:19 Reserved, must be kept at reset value.
Bits 18:16 MODE[2:0]: DMA2D mode
This bit is set and cleared by software. It cannot be modified while a transfer is ongoing.
000: Memory-to-memory (FG fetch only)
001: Memory-to-memory with PFC (FG fetch only with FG PFC active)
010: Memory-to-memory with blending (FG and BG fetch with PFC and blending)
011: Register-to-memory (no FG nor BG, only output stage active)
100: Memory-to-memory with blending and fixed color FG (BG fetch only with FG and
BG PFC active)
101: Memory-to-memory with blending and fixed color BG (FG fetch only with FG and
BG PFC active)
Others: Reserved
Bits 15:14 Reserved, must be kept at reset value.
Bit 13 CEIE: Configuration error (CE) interrupt enable
This bit is set and cleared by software.
0: CE interrupt disabled
1: CE interrupt enabled
Bit 12 CTCIE: CLUT transfer complete (CTC) interrupt enable
This bit is set and cleared by software.
0: CTC interrupt disabled
1: CTC interrupt enabled

RM0456 Rev 6

<!-- pagebreak -->

891

Chrom-ART Accelerator controller (DMA2D)

RM0456

Bit 11 CAEIE: CLUT access error (CAE) interrupt enable
This bit is set and cleared by software.
0: CAE interrupt disabled
1: CAE interrupt enabled
Bit 10 TWIE: Transfer watermark (TW) interrupt enable
This bit is set and cleared by software.
0: TW interrupt disabled
1: TW interrupt enabled
Bit 9 TCIE: Transfer complete (TC) interrupt enable
This bit is set and cleared by software.
0: TC interrupt disabled
1: TC interrupt enabled
Bit 8 TEIE: Transfer error (TE) interrupt enable
This bit is set and cleared by software.
0: TE interrupt disabled
1: TE interrupt enabled
Bit 7 Reserved, must be kept at reset value.
Bit 6 LOM: Line offset mode
This bit configures how the line offset is expressed (pixels or bytes) for the foreground,
background and output.
This bit is set and cleared by software. It can not be modified while a transfer is ongoing.
0: Line offsets expressed in pixels
1: Line offsets expressed in bytes
Bits 5:3 Reserved, must be kept at reset value.
Bit 2 ABORT: Abort
This bit can be used to abort the current transfer. This bit is set by software, and
is automatically reset by hardware when START = 0.
0: No transfer abort requested
1: Transfer abort requested
Bit 1 SUSP: Suspend
This bit can be used to suspend the current transfer. This bit is set and reset by software.
It is automatically reset by hardware when START = 0.
0: Transfer not suspended
1: Transfer suspended
Bit 0 START: Start
This bit can be used to launch the DMA2D according to parameters loaded in the various
configuration registers. This bit is automatically reset by the following events:
–
at the end of the transfer
–
when the data transfer is aborted by the user by setting ABORT in this register
–
when a data transfer error occurs
–
when the data transfer has not started due to a configuration error, or another
transfer operation already ongoing (automatic CLUT loading)

<!-- pagebreak -->

