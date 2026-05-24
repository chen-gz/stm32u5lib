RM0456 Rev 6

RM0456

Extended interrupts and event controller (EXTI)

Bits 31:26 Reserved, must be kept at reset value.
Bits 25:0 SWIx: Software interrupt on event x (x = 25 to 0)
When EXTI_SECCFGR.SECx is disabled, SWIx can be accessed with nonsecure and
secure access.
When EXTI_SECCFGR.SECx is enabled, SWIx can only be accessed with secure access.
Nonsecure write to this SWI x is discarded, nonsecure read returns 0.
When EXTI_PRIVCFGR.PRIVx is disabled, SWIx can be accessed with unprivileged and
privileged access.
When EXTI_PRIVCFGR.PRIVx is enabled, SWIx can only be accessed with privileged
access. Unprivileged write to this SWIx is discarded, unprivileged read returns 0.
A software interrupt is generated independent from the setting in EXTI_RTSR and
EXTI_FTSR. It always returns 0 when read.
0: Writing 0 has no effect.
1: Writing 1 triggers a rising edge event on event x. This bit is auto cleared by hardware.
Note: SW25, SW24, and SW23 bits are only available on some devices in the STM32U5
Series. Refer to the EXTI line connections table for its availability. If not present,
consider this bit as reserved and keep at reset value.

23.6.4

EXTI rising edge pending register (EXTI_RPR1)
Address offset: 0x00C
Reset value: 0x0000 0000
This register contains only bits for configurable events.

31

30

29

28

27

26

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

RPIF25 RPIF24 RPIF23 RPIF22 RPIF21 RPIF20 RPIF19 RPIF18 RPIF17 RPIF16
rc_w1

rc_w1

rc_w1

rc_w1

rc_w1

rc_w1

rc_w1

rc_w1

rc_w1

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

RPIF15 RPIF14 RPIF13 RPIF12 RPIF11 RPIF10

RPIF9

RPIF8

RPIF7

RPIF6

RPIF5

RPIF4

RPIF3

RPIF2

RPIF1

RPIF0

rc_w1

rc_w1

rc_w1

rc_w1

rc_w1

rc_w1

rc_w1

rc_w1

rc_w1

rc_w1

rc_w1

rc_w1

rc_w1

rc_w1

rc_w1

rc_w1

rc_w1

Bits 31:26 Reserved, must be kept at reset value.
Bits 25:0 RPIFx: configurable event inputs x rising edge pending bit (x = 25 to 0)
When EXTI_SECCFGR.SECx is disabled, RPIFx can be accessed with nonsecure and
secure access.
When EXTI_SECCFGR.SECx is enabled, RPIFx can only be accessed with secure access.
Nonsecure write to this RPIFx is discarded, nonsecure read returns 0.
When EXTI_PRIVCFGR.PRIVx is disabled, RPIFx can be accessed with unprivileged and
privileged access.
When EXTI_PRIVCFGR.PRIVx is enabled, RPIFx can only be accessed with privileged
access. Unprivileged write to this RPIFx is discarded, unprivileged read returns 0.
0: No rising edge trigger request occurred
1: Rising edge trigger request occurred
This bit is set when the rising edge event or an EXTI_SWIER software trigger arrives on the
configurable event line. This bit is cleared by writing 1 to it.
Note: RPIF25, RPIF24, and RPIF23 bits are only available on some devices in the STM32U5
Series. Refer to the EXTI line connections table for its availability.
If not present, consider this bit as reserved and keep at reset value.

RM0456 Rev 6

<!-- pagebreak -->

