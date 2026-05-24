935

Extended interrupts and event controller (EXTI)

RM0456

Bits 25:0 EMx: CPU wake-up with event generation mask on event input x (x = 25 to 0)
When EXTI_SECCFGR.SECx is disabled, EMx can be accessed with nonsecure and secure
access.
When EXTI_SECCFGR.SECx is enabled, EMx can only be accessed with secure access.
Nonsecure write to this bit x is discarded and nonsecure read returns 0.
When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and
unprivileged access.
When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged
access. Unprivileged write to this bit is discarded.
0: Wake-up with event generation from line x is masked.
1: Wake-up with event generation from line x is unmasked.
Note: EM25, EM24, and EM23 bits are only available on some devices in the STM32U5
Series. Refer to the EXTI line connections table for its availability. If not present,
consider this bit as reserved and keep at reset value.

23.6.12

EXTI register map

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

EXTI_RTSR1

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

Reset value

0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0

EXTI_FTSR1

Res.
Res.
Res.
Res.
Res.
Res.
FT25
FT24
FT23
FT22
FT21
FT20
FT19
FT18
FT17
FT16
FT15
FT14
FT13
FT12
FT11
FT10
FT9
FT8
FT7
FT6
FT5
FT4
FT3
FT2
FT1
FT0

Reset value

0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0

EXTI_SWIER1

Res.
Res.
Res.
Res.
Res.
Res.
SWI25
SWI24
SWI23
SWI22
SWI21
SWI20
SWI19
SWI18
SWI17
SWI16
SWI15
SWI14
SWI13
SWI12
SWI11
SWI10
SWI9
SWI8
SWI7
SWI6
SWI5
SWI4
SWI3
SWI2
SWI1
SWI0

Reset value

0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0

EXTI_RPR1

Res.
Res.
Res.
Res.
Res.
Res.
RPIF25
RPIF24
RPIF23
RPIF22
RPIF21
RPIF20
RPIF19
RPIF18
RPIF17
RPIF16
RPIF15
RPIF14
RPIF13
RPIF12
RPIF11
RPIF10
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

Reset value

0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0

EXTI_FPR1

Res.
Res.
Res.
Res.
Res.
Res.
FPIF25
FPIF24
FPIF23
FPIF22
FPIF21
FPIF20
FPIF19
FPIF18
FPIF17
FPIF16
FPIF15
FPIF14
FPIF13
FPIF12
FPIF11
FPIF10
FPIF9
FPIF8
FPIF7
FPIF6
FPIF5
FPIF4
FPIF3
FPIF2
FPIF1
FPIF0

Reset value

0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0

EXTI_
SECCFGR1

Res.
Res.
Res.
Res.
Res.
Res.
SEC25
SEC24
SEC23
SEC22
SEC21
SEC20
SEC19
SEC18
SEC17
SEC16
SEC15
SEC14
SEC13
SEC12
SEC11
SEC10
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

Reset value

0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0

EXTI_
PRIVCFGR1

Res.
Res.
Res.
Res.
Res.
Res.
PRIV25
PRIV24
PRIV23
PRIV22
PRIV21
PRIV20
PRIV19
PRIV18
PRIV17
PRIV16
PRIV15
PRIV14
PRIV13
PRIV12
PRIV11
PRIV10
PRIV9
PRIV8
PRIV7
PRIV6
PRIV5
PRIV4
PRIV3
PRIV2
PRIV1
PRIV0

Table 194. EXTI register map and reset values
Register
name

Reset value

0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0

Offset
0x000

0x004

0x008

0x00C

0x010

0x014

0x018
0x0200x05C
0x060

Reserved
EXTI_EXTICR1
Reset value

0x064

EXTI_EXTICR2
Reset value

<!-- pagebreak -->

Reserved
EXTI3[7:0]

EXTI2[7:0]

EXTI1[7:0]

EXTI0[7:0]

0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0
EXTI7[7:0]

EXTI6[7:0]

EXTI5[7:0]

EXTI4[7:0]

0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0

RM0456 Rev 6

RM0456

Extended interrupts and event controller (EXTI)

Offset

0x068

Register
name
EXTI_EXTICR3
Reset value

0x0740x07C
0x080

0x084

EXTI10[7:0]

EXTI9[7:0]

EXTI8[7:0]

0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0
EXTI14[7:0]

EXTI13[7:0]

EXTI2[7:0]

Reset value

0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0

EXTI_LOCKR

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

EXTI15[7:0]

Reset value

0

Reserved

Reserved

EXTI_IMR1

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

0x070

EXTI_EXTICR4

EXTI11[7:0]

Reset value

0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0

EXTI_EMR1

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

0x06C

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

Table 194. EXTI register map and reset values (continued)

Reset value

0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0

Refer to Section 2.3 for the register boundary addresses.

RM0456 Rev 6

<!-- pagebreak -->

