2577

Independent watchdog (IWDG)

61.7.6

RM0456

IWDG early wake-up interrupt register (IWDG_EWCR)
Address offset: 0x14
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

EWIE

EWIC

Res.

Res.

rw

w

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

EWIT[11:0]
rw

Bits 31:16 Reserved, must be kept at reset value.
Bit 15 EWIE: Watchdog early interrupt enable
Set and reset by software.
0: The early interrupt interface is disabled.
1: The early interrupt interface is enabled.
The EWU bit in the IWDG status register (IWDG_SR) must be reset to be able to change the
value of this bit.
Bit 14 EWIC: Watchdog early interrupt acknowledge
The software must write a 1 into this bit in order to acknowledge the early wake-up interrupt
and to clear the EWIF flag. Writing 0 has not effect, reading this flag returns a 0.
Bits 13:12 Reserved, must be kept at reset value.
Bits 11:0 EWIT[11:0]: Watchdog counter window value
These bits are write access protected (see Section 61.4.6). They are written by software to
define at which position of the IWDCNT down-counter the early wake-up interrupt must be
generated. The early interrupt is generated when the IWDCNT is lower or equal to
EWIT[11:0] - 1.
EWIT[11:0] must be bigger than 1.
An interrupt is generated only if EWIE = 1.
The EWU bit in the IWDG status register (IWDG_SR) must be reset to be able to change the
reload value.
Note: Reading this register returns the Early wake-up comparator value and the Interrupt
enable bit from the VDD voltage domain. This value may not be up to date/valid if a write
operation to this register is ongoing, hence the value read from this register is valid only
when the EWU bit in the IWDG status register (IWDG_SR) is reset.

<!-- pagebreak -->

