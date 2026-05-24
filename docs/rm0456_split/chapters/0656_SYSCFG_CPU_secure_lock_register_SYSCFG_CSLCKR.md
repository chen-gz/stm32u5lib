667

System configuration controller (SYSCFG)

RM0456

Bit 1 LOCKNSMPU: Nonsecure MPU registers lock
This bit is set by software and cleared only by a system reset. When set, this bit disables
write access to nonsecure MPU_CTRL_NS, MPU_RNR_NS and MPU_RBAR_NS registers.
0: Nonsecure MPU registers write enabled
1: Nonsecure MPU registers write disabled
Bit 0 LOCKNSVTOR: VTOR_NS register lock
This bit is set by software and cleared only by a system reset.
0: VTOR_NS register write enabled
1: VTOR_NS register write disabled

15.3.5

SYSCFG CPU secure lock register (SYSCFG_CSLCKR)
Address offset: 0x10
Reset value: 0x0000 0000
This register is used to lock the configuration of PRIS and BFHFNMINS in AIRCR register,
SAU, secure MPU, and VTOR_S registers.
When the system is secure (TZEN = 1), this register can be written only when the access is
secure. A nonsecure read/write access is RAZ/WI and generates an illegal access event.
When the system is not secure (TZEN = 0), this register is RAZ/WI. This register can be
read and written by privileged access only. Unprivileged access is RAZ/WI.

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

Res.

Res.

LOCKS
LOCKS LOCKS
VTAIR
AU
MPU
CR
rs

rs

rs

Bits 31:3 Reserved, must be kept at reset value.
Bit 2 LOCKSAU: SAU registers lock
This bit is set by software and cleared only by a system reset. When set, this bit disables
write access to SAU_CTRL, SAU_RNR, SAU_RBAR and SAU_RLAR registers.
0: SAU registers write enabled
1: SAU registers write disabled
Bit 1 LOCKSMPU: Secure MPU registers lock
This bit is set by software and cleared only by a system reset. When set, this bit disables
write access to secure MPU_CTRL, MPU_RNR and MPU_RBAR registers.
0: Secure MPU registers writes enabled
1: Secure MPU registers writes disabled
Bit 0 LOCKSVTAIRCR: VTOR_S register and AIRCR register bits lock
This bit is set by software and cleared only by a system reset. When set, this bit disables
write access to VTOR_S register, PRIS and BFHFNMINS bits in the AIRCR register.
0: VTOR_S register PRIS and BFHFNMINS bits in AIRCR register write enabled
1: VTOR_S register PRIS and BFHFNMINS bits in AIRCR register write disabled

<!-- pagebreak -->

