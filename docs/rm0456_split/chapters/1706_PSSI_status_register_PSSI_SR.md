1710

Parallel synchronous slave interface (PSSI)

RM0456

Bit 14 ENABLE: PSSI enable
0: PSSI disabled
1: PSSI enabled
The contents of the FIFO are flushed when ENABLE is cleared to 0.
Note: When ENABLE=1, the content of PSSI_CR must not be changed, except for the
ENABLE bit itself. All configuration bits can change as soon as ENABLE changes from
0 to 1.
The DMA controller and all PSSI configuration registers must be programmed correctly
before setting the ENABLE bit to 1.
The ENABLE bit and the DCMI ENABLE bit (bit 15 of DCMI_CR) must not be set to 1 at
the same time.
Bits 13:12 Reserved, must be kept at reset value.
Bits 11:10 EDM[1:0]: Extended data mode
00: Interface captures 8-bit data on every parallel data clock
01: Reserved, must not be selected
10: Reserved, must not be selected
11: The interface captures 16-bit data on every parallel data clock
Bit 9 Reserved, must be kept at reset value.
Bit 8 RDYPOL: Ready (PSSI_RDY) polarity
This bit indicates the level on the PSSI_RDY pin when the data are not valid on the parallel
interface.
0: PSSI_RDY active low (0 indicates that the receiver is ready to receive)
1: PSSI_RDY active high (1 indicates that the receiver is ready to receive)
Bit 7 Reserved, must be kept at reset value.
Bit 6 DEPOL: Data enable (PSSI_DE) polarity
This bit indicates the level on the PSSI_DE pin when the data are not valid on the parallel
interface.
0: PSSI_DE active low (0 indicates that data is valid)
1: PSSI_DE active high (1 indicates that data is valid)
Bit 5 CKPOL: Parallel data clock polarity
This bit configures the capture edge of the parallel clock or the edge used for driving outputs,
depending on OUTEN.
0: Falling edge active for inputs or rising edge active for outputs
1: Rising edge active for inputs or falling edge active for outputs.
Bits 4:0 Reserved, must be kept at reset value.

42.5.2

PSSI status register (PSSI_SR)
Address offset: 0x04
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

3

2

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

RTT1B RTT4B
r

<!-- pagebreak -->

