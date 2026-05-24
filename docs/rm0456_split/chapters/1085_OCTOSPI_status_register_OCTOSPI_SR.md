rw

rw

rw

rw

rw

rw

rw

RM0456 Rev 6

RM0456

Octo-SPI interface (OCTOSPI)

Bits 31:0 REFRESH[31:0]: Refresh rate
This bitfield enables the refresh rate feature. The NCS is released every REFRESH + 1 clock
cycles for writes, and REFRESH + 4 clock cycles for reads. These two values can be
extended with few clock cycles when refresh occurs during a byte transmission in single-,
dual- or quad-SPI mode, because the byte transmission must be completed.
0: Refresh disabled
Others: Maximum communication length is set to REFRESH + 1 clock cycles.
Note: REFRESH count is based on the divided clock period: if OCTOSPI_DCR2
PRESCALER bitfield is changed, the REFRESH field must be updated accordingly.

28.7.6

OCTOSPI status register (OCTOSPI_SR)
Address offset: 0x0020
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

Res.

Res.

FLEVEL[5:0]
r

r

r

r

r

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

BUSY

TOF

SMF

FTF

TCF

TEF

r

r

r

r

r

r

r

Bits 31:14 Reserved, must be kept at reset value.
Bits 13:8 FLEVEL[5:0]: FIFO level
This bitfield gives the number of valid bytes that are being held in the FIFO. FLEVEL = 0
when the FIFO is empty, and 32 when it is full.
In automatic status-polling mode, FLEVEL is zero.
Bits 7:6 Reserved, must be kept at reset value.
Bit 5 BUSY: Busy
This bit is set when an operation is ongoing. It is cleared automatically when the operation
with the external device is finished and the FIFO is empty.
Bit 4 TOF: Timeout flag
This bit is set when timeout occurs. It is cleared by writing 1 to CTOF.
Bit 3 SMF: Status match flag
This bit is set in automatic status-polling mode when the unmasked received data matches
the corresponding bits in the match register (OCTOSPI_PSMAR).
It is cleared by writing 1 to CSMF.
Bit 2 FTF: FIFO threshold flag
In indirect mode, this bit is set when the FIFO threshold has been reached, or if there is any
data left in the FIFO after the reads from the external device are complete.
It is cleared automatically as soon as the threshold condition is no longer true.
In automatic status-polling mode, this bit is set every time the status register is read, and the
bit is cleared when the data register is read.
Bit 1 TCF: Transfer complete flag
This bit is set in indirect mode when the programmed number of data has been transferred or
in any mode when the transfer has been aborted.It is cleared by writing 1 to CTCF.

RM0456 Rev 6

<!-- pagebreak -->

