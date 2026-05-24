RM0456 Rev 6

RM0456

General-purpose I/Os (GPIO)

13.4.8

GPIO port configuration lock register (GPIOx_LCKR) (x = A to J)
Address offset: 0x1C
Reset value: 0x0000 0000
This register is used to lock the configuration of the port bits when a correct write sequence
is applied to bit 16 (LCKK). The value of bits [15:0] is used to lock the configuration of the
GPIO. During the write sequence, the value of LCKR[15:0] must not change. When the
LOCK sequence has been applied on a port bit, the value of this port bit can no longer be
modified until the next MCU reset or peripheral reset.

Note:

A specific write sequence is used to write to GPIOx_LCKR. Only word access (32-bit long)
is allowed during this locking sequence.
Each lock bit freezes a specific configuration register (control and alternate function
registers).

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

LCKK

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

LCK15

LCK14

LCK13

LCK12

LCK11

LCK10

LCK9

LCK8

LCK7

LCK6

LCK5

LCK4

LCK3

LCK2

LCK1

LCK0

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

rw

rw

rw

rw

rw

rw

Bits 31:17 Reserved, must be kept at reset value.
Bit 16 LCKK: Lock key
This bit can be read any time. It can only be modified using the lock key write sequence.
0: Port configuration lock key not active
1: Port configuration lock key active. The GPIOx_LCKR register is locked until the next MCU
reset or peripheral reset.
- LOCK key write sequence:
WR LCKR[16] = 1 + LCKR[15:0]
WR LCKR[16] = 0 + LCKR[15:0]
WR LCKR[16] = 1 + LCKR[15:0]
- LOCK key read
RD LCKR[16] = 1 (this read operation is optional but it confirms that the lock is active)
Note: During the lock key write sequence, the value of LCK[15:0] must not change.
Any error in the lock sequence aborts the LOCK.
After the first lock sequence on any bit of the port, any read access on the LCKK bit
returns 1 until the next MCU reset or peripheral reset.
Bits 15:0 LCKy: Port x lock I/O pin y (y = 15 to 0)
These bits are read/write but can only be written when the LCKK bit is 0
0: Port configuration not locked
1: Port configuration locked
Note: This bit is reserved and must be kept at reset value when the corresponding I/O is not
available on the selected package.

RM0456 Rev 6

<!-- pagebreak -->

