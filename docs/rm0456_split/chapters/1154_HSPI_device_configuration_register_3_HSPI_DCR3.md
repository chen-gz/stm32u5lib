1177

Hexadeca-SPI interface (HSPI)

RM0456

Bits 18:16 WRAPSIZE[2:0]: Wrap size
This bitfield indicates the wrap size to which the memory is configured. For memories, which
have a separate command for wrapped instructions, this bitfield indicates the wrap-size
associated with the command held in HSPI_WPIR.
000: Wrapped reads are not supported by the memory.
010: External memory supports wrap size of 16 bytes.
011: External memory supports wrap size of 32 bytes.
100: External memory supports wrap size of 64 bytes.
101: External memory supports wrap size of 128 bytes.
Others: Reserved
Bits 15:8 Reserved, must be kept at reset value.
Bits 7:0 PRESCALER[7:0]: Clock prescaler
This bitfield defines the scaler factor for generating the CLK based on the kernel clock
(value + 1).
0: FCLK = FKERNEL, kernel clock used directly as HSPI CLK (prescaler bypassed). In this
case, if the DTR mode is used, it is mandatory to provide to the HSPI a kernel clock that has
50% duty-cycle.
1: FCLK = FKERNEL/2
2: FCLK = FKERNEL/3
...
255: FCLK = FKERNEL/256
For odd clock division factors, the CLK duty cycle is not 50%. The clock signal remains low
one cycle longer than it stays high.
Writing this bitfield automatically starts a new calibration of high-speed interface DLL
at the start of next transfer, except in case HSPI_CALSOR or HSPI_CALSIR have been
written in the meantime. BUSY stays high during the whole calibration execution.

30.7.4

HSPI device configuration register 3 (HSPI_DCR3)
Address offset: 0x010
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

23

22

21

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

20

19

rw

rw

18

17

16

rw

CSBOUND[4:0]
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

Bits 31:21 Reserved, must be kept at reset value.
Bits 20:16 CSBOUND[4:0]: NCS boundary
This bitfield enables the transaction boundary feature. When active, a minimum value of
three is recommended.
The NCS is released on each boundary of 2CSBOUND bytes.
0: NCS boundary disabled
Others: NCS boundary set to 2CSBOUND bytes
Bits 15:0 Reserved, must be kept at reset value.

<!-- pagebreak -->

