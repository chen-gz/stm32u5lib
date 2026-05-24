1177

Hexadeca-SPI interface (HSPI)

RM0456

Bit 2 FTF: FIFO threshold flag
In indirect mode, this bit is set when the FIFO threshold has been reached, or if there is any
data left in the FIFO after the reads from the external device are complete.
It is cleared automatically as soon as the threshold condition is no longer true.
In automatic status-polling mode this bit is set every time the status register is read, and the
bit is cleared when the data register is read.
Bit 1 TCF: Transfer complete flag
This bit is set in indirect mode when the programmed number of data has been transferred or
in any mode when the transfer has been aborted.It is cleared by writing 1 to CTCF.
Bit 0 TEF: Transfer error flag
This bit is set in indirect mode when an invalid address is being accessed in indirect mode.
It is cleared by writing 1 to CTEF.

30.7.7

HSPI flag clear register (HSPI_FCR)
Address offset: 0x024
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
Writing 1 clears the TOF flag in the HSPI_SR register.
Bit 3 CSMF: Clear status match flag
Writing 1 clears the SMF flag in the HSPI_SR register.
Bit 2 Reserved, must be kept at reset value.
Bit 1 CTCF: Clear transfer complete flag
Writing 1 clears the TCF flag in the HSPI_SR register.
Bit 0 CTEF: Clear transfer error flag
Writing 1 clears the TEF flag in the HSPI_SR register.

<!-- pagebreak -->

