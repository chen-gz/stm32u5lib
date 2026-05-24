RM0456 Rev 6

r

1

0

Res.

Res.

RM0456

Parallel synchronous slave interface (PSSI)

Bits 31:4 Reserved, must be kept at reset value.
Bit 3 RTT1B: FIFO is ready to transfer one byte
1: FIFO is ready for a one byte (32-bit) transfer. In receive mode, this means that at least
one valid data byte is in the FIFO. In transmit mode, this means that there is at least one
byte free in the FIFO.
0: FIFO is not ready for a 1-byte transfer
Bit 2 RTT4B: FIFO is ready to transfer four bytes
1: FIFO is ready for a four-byte (32-bit) transfer. In receive mode, this means that at least
four valid data bytes are in the FIFO. In transmit mode, this means that there are at least
four bytes free in the FIFO.
0: FIFO is not ready for a four-byte transfer
Bits 1:0 Reserved, must be kept at reset value.

42.5.3

PSSI raw interrupt status register (PSSI_RIS)
Address offset: 0x08
Reset value: 0x0000 0000
PSSI_RIS gives the raw interrupt status. This register is read-only. When read, it returns the
status of the corresponding interrupt before masking with the PSSI_IER register value.

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

OVR_R
IS

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

r

Bits 31:2 Reserved, must be kept at reset value.
Bit 1 OVR_RIS: Data buffer overrun/underrun raw interrupt status
0: No overrun/underrun occurred
1: An overrun/underrun occurred: overrun in receive mode, underrun in transmit mode.
This bit is cleared by writing a 1 to the OVR_ISC bit in PSSI_ICR.
Bit 0 Reserved, must be kept at reset value.

RM0456 Rev 6

<!-- pagebreak -->

