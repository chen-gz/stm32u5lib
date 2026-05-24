988

Filter math accelerator (FMAC)

26.4.5

RM0456

FMAC control register (FMAC_CR)
Address offset: 0x10
Reset value: 0x0000 0000
Access: word access

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

RESET

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

DMA
WEN

DMA
REN

Res.

SAT
IEN

UNFL
IEN

OVFL
IEN

WIEN

RIEN

rw

rw

rw

rw

rw

rw

rw

rw

CLIP
EN

Res.

rw

Res.

Res.

Res.

Res.

Res.

Bits 31:17 Reserved, must be kept at reset value.
Bit 16 RESET: Reset FMAC unit
This resets the write and read pointers, the internal control logic, the FMAC_SR register and
the FMAC_PARAM register, including the START bit if active. Other register settings are not
affected. This bit is reset by hardware.
0: Reset inactive
1: Reset active
Bit 15 CLIPEN: Enable clipping
0: Clipping disabled. Values at the output of the accumulator which exceed the q1.15 range,
wrap.
1: Clipping enabled. Values at the output of the accumulator which exceed the q1.15 range
are saturated to the maximum positive or negative value (+1 or -1) according to the sign.
Bits 14:10 Reserved, must be kept at reset value.
Bit 9 DMAWEN: Enable DMA write channel requests
0: Disable. No DMA requests are generated
1: Enable. DMA requests are generated while the X1 buffer is not full.
This bit can only be modified when START= 0 in the FMAC_PARAM register. A read returns
the current state of the bit.
Bit 8 DMAREN: Enable DMA read channel requests
0: Disable. No DMA requests are generated
1: Enable. DMA requests are generated while the Y buffer is not empty.
This bit can only be modified when START= 0 in the FMAC_PARAM register. A read returns
the current state of the bit.
Bits 7:5 Reserved, must be kept at reset value.
Bit 4 SATIEN: Enable saturation error interrupts
0: Disabled. No interrupts are generated upon saturation detection.
1: Enabled. An interrupt request is generated if the SAT flag is set
This bit is set and cleared by software. A read returns the current state of the bit.
Bit 3 UNFLIEN: Enable underflow error interrupts
0: Disabled. No interrupts are generated upon underflow detection.
1: Enabled. An interrupt request is generated if the UNFL flag is set
This bit is set and cleared by software. A read returns the current state of the bit.

<!-- pagebreak -->

