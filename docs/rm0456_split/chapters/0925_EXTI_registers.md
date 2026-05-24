RM0456 Rev 6

RM0456

Extended interrupts and event controller (EXTI)

23.6

EXTI registers
The EXTI register map is divided in the following sections:
Table 193. EXTI register map sections
Address offset

Description

0x000 - 0x01C

General configurable event [22:0] configuration

0x060 - 0x06C

EXTI IO port mux selection

0x070

EXTI protection lock configuration

0x080 - 0x0BC

CPU input event configuration

All registers can be accessed with word (32-bit), half-word (16-bit), and byte (8-bit) access.

23.6.1

EXTI rising trigger selection register (EXTI_RTSR1)
Address offset: 0x000
Reset value: 0x0000 0000
This register contains only bits for configurable events.

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

RT25

RT24

RT23

RT22

RT21

RT20

RT19

RT18

RT17

RT16

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

RT15

RT14

RT13

RT12

RT11

RT10

RT9

RT8

RT7

RT6

RT5

RT4

RT3

RT2

RT1

RT0

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
Bits 25:0 RTx: Rising trigger event configuration bit of configurable event input x(1) (x = 25 to 0)
When EXTI_SECCFGR.SECx is disabled, RTx can be accessed with nonsecure and secure
access.
When EXTI_SECCFGR.SECx is enabled, RTx can only be accessed with secure access.
Nonsecure write to this bit x is discarded and nonsecure read returns 0.
When EXTI_PRIVCFGR.PRIVx is disabled, RTx can be accessed with unprivileged and
privileged access.
When EXTI_PRIVCFGR.PRIVx is enabled, RTx can only be accessed with privileged
access. Unprivileged write to this bit x is discarded, unprivileged read returns 0.
0: Rising trigger disabled (for event and interrupt) for input line
1: Rising trigger enabled (for event and interrupt) for input line
Note: RT25, RT24, and RT23 bits are only available on some devices in the STM32U5
Series. Refer to the EXTI line connections table for its availability. If not present,
consider this bit as reserved and keep at reset value.
1. The configurable event inputs are edge triggered. No glitch must be generated on these inputs. If a rising edge on the
configurable event input occurs during writing of the register, the associated pending bit is not set. Rising and falling edge
triggers can be set for the same configurable event input. In this case, both edges generate a trigger.

RM0456 Rev 6

<!-- pagebreak -->

