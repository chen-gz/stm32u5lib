< 2.4 V

RM0456 Rev 6

BOOSTEN
0
1

ANASWVDD
0
1
0

RM0456

System configuration controller (SYSCFG)

15.3.3

SYSCFG FPU interrupt mask register (SYSCFG_FPUIMR)
Address offset: 0x08
Reset value: 0x0000 001F
When the system is secure (TZEN = 1), this register can be protected against nonsecure
access by setting the FPUSEC bit in the SYSCFG_SECCFGR register: a nonsecure
read/write access is RAZ/WI and generates an illegal access event.
When the system is not secure (TZEN = 0), there is no access restriction. This register can
be read and written by privileged access only. Unprivileged access is RAZ/WI.

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
rw

rw

rw

rw

FPU_IE[5:0]
rw

rw

Bits 31:6 Reserved, must be kept at reset value.
Bits 5:0 FPU_IE[5:0]: Floating point unit interrupts enable bits
FPU_IE[5]: Inexact interrupt enable (interrupt disable at reset)
FPU_IE[4]: Input abnormal interrupt enable
FPU_IE[3]: Overflow interrupt enable
FPU_IE[2]: Underflow interrupt enable
FPU_IE[1]: Divide-by-zero interrupt enable
FPU_IE[0]: Invalid operation Interrupt enable

15.3.4

SYSCFG CPU nonsecure lock register (SYSCFG_CNSLCKR)
Address offset: 0x0C
Reset value: 0x0000 0000
This register is used to lock the configuration of nonsecure MPU and VTOR_NS registers.
This register can be read and written by privileged access only. Unprivileged access is
RAZ/WI.

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

Res.

LOCKN LOCKN
SMPU SVTOR
rs

rs

Bits 31:2 Reserved, must be kept at reset value.

RM0456 Rev 6

<!-- pagebreak -->

