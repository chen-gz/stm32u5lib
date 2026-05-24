RM0456 Rev 6

RM0456

Filter math accelerator (FMAC)

Bit 2 OVFLIEN: Enable overflow error interrupts
0: Disabled. No interrupts are generated upon overflow detection.
1: Enabled. An interrupt request is generated if the OVFL flag is set
This bit is set and cleared by software. A read returns the current state of the bit.
Bit 1 WIEN: Enable write interrupt
0: Disabled. No write interrupt requests are generated.
1: Enabled. An interrupt request is generated while the X1 buffer FULL flag is not set.
This bit is set and cleared by software. A read returns the current state of the bit.
Bit 0 RIEN: Enable read interrupt
0: Disabled. No read interrupt requests are generated.
1: Enabled. An interrupt request is generated while the Y buffer EMPTY flag is not set.
This bit is set and cleared by software. A read returns the current state of the bit.

26.4.6

FMAC status register (FMAC_SR)
Address offset: 0x14
Reset value: 0x0000 0001
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

X1
FULL

Y
EMPTY

r

r

Res.

Res.

Res.

Res.

Res.

SAT

UNFL

OVFL

r

r

r

Res.

Res.

Res.

Res.

Res.

Bits 31:11 Reserved, must be kept at reset value.
Bit 10 SAT: Saturation error flag
Saturation occurs when the result of an accumulation exceeds the numeric range of the
accumulator.
0: No saturation detected
1: Saturation detected. If the SATIEN bit is set, an interrupt is generated.
This flag is cleared by a reset of the unit.
Bit 9 UNFL: Underflow error flag
An underflow occurs when a read is made from FMAC_RDATA when no valid data is
available in the Y buffer.
0: No underflow detected
1: Underflow detected. If the UNFLIEN bit is set, an interrupt is generated.
This flag is cleared by a reset of the unit.
Bit 8 OVFL: Overflow error flag
An overflow occurs when a write is made to FMAC_WDATA when no free space is available
in the X1 buffer.
0: No overflow detected
1: Overflow detected. If the OVFLIEN bit is set, an interrupt is generated.
This flag is cleared by a reset of the unit.
Bits 7:2 Reserved, must be kept at reset value.

RM0456 Rev 6

<!-- pagebreak -->

