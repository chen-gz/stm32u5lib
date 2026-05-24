275

Global TrustZone controller (GTZC)

RM0456

Bits 19:0 SPLCKy: Security/privilege configuration lock for super-block (y = 51 to 32)
This bit is set by software and can be cleared only by system reset.
0: GTZC1_MPCBBz_SECCFGRy and GTZC1_MPCBBz_PRIVCFGRy can be written.
1: Writes to GTZC1_MPCBBz_SECCFGRy and GTZC1_MPCBBz_PRIVCFGRy are ignored

5.8.4

GTZC1 SRAMz MPCBB security configuration for super-block x
register (GTZC1_MPCBBz_SECCFGRx) (z = 1, 2, 3, 5, 6)
Address offset: 0x100 + 0x04 * x, (x = 0 to 51)
Reset value: 0xFFFF FFFF
The given reset value is valid when TZEN = 1. The reset value is 0x0000 0000 when
TZEN = 0.
Write access to this register is secure only. Any read is allowed.

Note:

31

Some registers are only available on some devices in the STM32U5 series. Refer to the
device datasheet for availability of its associated memory region.
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

SEC31 SEC30 SEC29 SEC28 SEC27 SEC26 SEC25 SEC24 SEC23 SEC22 SEC21 SEC20 SEC19 SEC18 SEC17 SEC16
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

SEC9

SEC8

SEC7

SEC6

SEC5

SEC4

SEC3

SEC2

SEC1

SEC0

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

SEC15 SEC14 SEC13 SEC12 SEC11 SEC10
rw

rw

rw

rw

rw

rw

Bits 31:0 SECy: Security configuration for block y (y = 31 to 0)
0: Nonsecure access only to block y, belonging to super-block x. Secure access is also
allowed if the SRWILADIS bit is set in GTZC1_MPCBBz_CR.
1: Secure access only to block y, belonging to super-block x.
Unprivileged write to this bit is ignored if PRIVy bit is set in GTZC1_MPCBBz_PRIVCFGRx.
Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCKR1/2.

5.8.5

GTZC1 SRAMz MPCBB privileged configuration for super-block x
register (GTZC1_MPCBBz_PRIVCFGRx) (z = 1, 2, 3, 5, 6)
Address offset: 0x200 + 0x04 * x, (x = 0 to 51)
Reset value: 0xFFFF FFFF
The given reset value is valid when TZEN = 1. The reset value is 0x0000 0000 when
TZEN = 0.
Write access to this register is privileged only. Any read is allowed.

Note:

<!-- pagebreak -->

