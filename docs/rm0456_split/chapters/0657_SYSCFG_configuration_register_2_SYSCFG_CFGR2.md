RM0456 Rev 6

RM0456

System configuration controller (SYSCFG)

15.3.6

SYSCFG configuration register 2 (SYSCFG_CFGR2)
Address offset: 0x14
Reset value: 0x0000 0000
When the system is secure (TZEN = 1), this register can be protected against nonsecure
access by setting the CLASSBSEC bit in the SYSCFG_SECCFGR register. When
CLASSBSEC bit is set, only secure access is allowed: nonsecure read/write access is
RAZ/WI and generates an illegal access event.
When the system is not secure (TZEN = 0), there is no access restriction. This register can
be read and written by privileged and unprivileged access.

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

ECCL

PVDL

SPL

CLL

rs

rs

rs

rs

Bits 31:4 Reserved, must be kept at reset value.
Bit 3 ECCL: ECC lock
This bit is set by software and cleared only by a system reset. It can be used to enable and
lock the FLASH ECC double error signal connection to the TIM1/8/15/16/17 break input.
0: ECC double error disconnected from TIM1/8/15/16/17 break input
1: ECC double error connected to TIM1/8/15/16/17 break input
Bit 2 PVDL: PVD lock enable bit
This bit is set by software and cleared only by a system reset. It can be used to enable and
lock the PVD connection to the TIM1/8/15/16/17 break input, as well as the PVDE and
PVDLS[2:0] in the PWR register.
0: PVD interrupt disconnected from TIM1/8/15/16/17 break input. PVDE and PVDLS[2:0] bits
can be programmed by the application.
1: PVD interrupt connected to TIM1/8/15/16/17 break input. PVDE and PVDLS[2:0] bits are
read only.
Bit 1 SPL: SRAM ECC lock bit
This bit is set by the software and cleared only by a system reset. It can be used to enable
and lock the SRAM ECC double error signal connection to TIM1/8/15/16/17 break inputs.
0: SRAM double error disconnected from TIM1/8/15/16/17 break inputs
1: SRAM double error connected to TIM1/8/15/16/17 break inputs
Bit 0 CLL: Cortex-M33 LOCKUP (HardFault) output enable
This bit is set by the software and cleared only by a system reset. It can be used to enable
and lock the connection of Cortex-M33 LOCKUP (HardFault) output to TIM1/8/15/16/17
break input.
0: Cortex-M33 LOCKUP output disconnected from TIM1/8/15/16/17 break inputs
1: Cortex-M33 LOCKUP output connected to TIM1/8/15/16/17 break inputs

RM0456 Rev 6

<!-- pagebreak -->

