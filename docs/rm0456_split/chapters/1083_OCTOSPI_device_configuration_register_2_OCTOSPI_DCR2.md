RM0456 Rev 6

RM0456

Octo-SPI interface (OCTOSPI)

28.7.3

OCTOSPI device configuration register 2 (OCTOSPI_DCR2)
Address offset: 0x000C
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

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

18

17

16

WRAPSIZE[2:0]
rw

rw

rw

2

1

0

rw

rw

rw

PRESCALER[7:0]
rw

rw

rw

rw

rw

Bits 31:19 Reserved, must be kept at reset value.
Bits 18:16 WRAPSIZE[2:0]: Wrap size
This bitfield indicates the wrap size to which the memory is configured. For memories which
have a separate command for wrapped instructions, this bitfield indicates the wrap-size
associated with the command held in the OCTOSPI1_WPIR register.
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
0: FCLK = FKERNEL, kernel clock used directly as OCTOSPI CLK (prescaler bypassed). In this
case, if the DTR mode is used, it is mandatory to provide to the OCTOSPI a kernel clock that
has 50% duty-cycle.
1: FCLK = FKERNEL/2
2: FCLK = FKERNEL/3
...
255: FCLK = FKERNEL/256
For odd clock division factors, the CLK duty cycle is not 50 %. The clock signal remains low
one cycle longer than it stays high.

RM0456 Rev 6

<!-- pagebreak -->

