935

Extended interrupts and event controller (EXTI)

RM0456

Bits 7:0 EXTI{4*(m-1)}[7:0]: EXTI{4*(m-1)} GPIO port selection
These bits are written by software to select the source input for EXTI{4*(m-1)} external
interrupt.
When EXTI_SECCFGR.SEC{4*(m-1)} is disabled, this field can be accessed with nonsecure
and secure access.
When EXTI_SECCFGR.SEC{4*(m-1)} is enabled, this field can only be accessed with
secure access. Nonsecure write is discarded and nonsecure read returns 0.
When EXTI_PRIVCFGR.PRIV{4*(m-1)} is disabled, this field can be accessed with
privileged and unprivileged access.
When EXTI_PRIVCFGR.PRIV{4*(m-1)} is enabled, this field can only be accessed with
privilege access. Unprivileged write to this bit is discarded.
0x00: PA[{4*(m-1)}] pin
0x01: PB[{4*(m-1)}] pin
0x02: PC[{4*(m-1)}] pin
0x03: PD[{4*(m-1)}] pin
0x04: PE[{4*(m-1)}] pin
0x05: PF[{4*(m-1)}] pin
0x06: PG[{4*(m-1)}] pin
0x07: PH[{4*(m-1)}] pin
0x08: PI[{4*(m-1)}] pin
0x09: PJ[{4*(m-1)}] pin
Others: reserved

23.6.9

EXTI lock register (EXTI_LOCKR)
Address offset: 0x070
Reset value: 0x0000 0000
This register provides write access security: a nonsecure write access is ignored, a read
access returns zero data, and both generates an illegal access event.

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

Res.

LOCK
rs

Bits 31:1 Reserved, must be kept at reset value.
Bit 0 LOCK: Global security and privilege configuration registers (EXTI_SECCFGR and
EXTI_PRIVCFGR) lock
This bit is written once after reset.
0: Security and privilege configuration open, can be modified.
1: Security and privilege configuration locked, can no longer be modified.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Extended interrupts and event controller (EXTI)

23.6.10

EXTI CPU wake-up with interrupt mask register (EXTI_IMR1)
Address offset: 0x080
Reset value: 0x0000 0000
This register contains bits for configurable events.

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

IM25

IM24

IM23

IM22

IM21

IM20

IM19

IM18

IM17

IM16

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

IM15

IM14

IM13

IM12

IM11

IM10

IM9

IM8

IM7

IM6

IM5

IM4

IM3

IM2

IM1

IM0

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

Bits 31:26 Reserved, must be kept at reset value.
Bits 25:0 IMx: CPU wake-up with interrupt mask on event input x (1) (x = 25 to 0)
When EXTI_SECCFGR.SECx is disabled, IMx can be accessed with nonsecure and secure
access.
When EXTI_SECCFGR.SECx is enabled, IMx can only be accessed with secure access.
Nonsecure write to this bit is discarded and nonsecure read returns 0.
When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and
unprivileged access.
When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged
access. Unprivileged write to this bit is discarded.
0: Wake-up with interrupt request from input event x is masked.
1: Wake-up with interrupt request from input event x is unmasked.
Note: IM25, IM24, and IM23 bits are only available on some devices in the STM32U5 Series.
Refer to the EXTI line connections table for its availability. If not present, consider this
bit as reserved and keep at reset value.
1. The reset value for configurable event inputs is set to 0 in order to disable the interrupt by default.

23.6.11

EXTI CPU wake-up with event mask register (EXTI_EMR1)
Address offset: 0x084
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

EM25

EM24

EM23

EM22

EM21

EM20

EM19

EM18

EM17

EM16

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

EM15

EM14

EM13

EM12

EM11

EM10

EM9

EM8

EM7

EM6

EM5

EM4

EM3

EM2

EM1

EM0

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

Bits 31:26 Reserved, must be kept at reset value.

RM0456 Rev 6

<!-- pagebreak -->

