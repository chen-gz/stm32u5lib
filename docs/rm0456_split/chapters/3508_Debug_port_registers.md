RM0456 Rev 6

r

r

r

RM0456

Debug support (DBG)

CTI CoreSight peripheral identity register 1 (CTI_CIDR1)
Address offset: 0xFF4
Reset value: 0x0000 0090
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

7

6

5

4

3

2

1

0

15

14

13

12

11

10

9

8

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

CLASS[3:0]
r

r

r

PREAMBLE[11:8]
r

r

r

r

r

Bits 31:8 Reserved, must be kept at reset value.
Bits 7:4 CLASS[3:0]: component identification bits [15:12] - component class
0x9: CoreSight component
Bits 3:0 PREAMBLE[11:8]: component identification bits [11:8]
0x0: common identification value

CTI CoreSight component identity register 2 (CTI_CIDR2)
Address offset: 0xFF8
Reset value: 0x0000 0005
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
r

r

r

r

r

r

r

PREAMBLE[19:12]
r

Bits 31:8 Reserved, must be kept at reset value.
Bits 7:0 PREAMBLE[19:12]: component identification bits [23:16]
0x05: common identification value

CTI CoreSight component identity register 3 (CTI_CIDR3)
Address offset: 0xFFC
Reset value: 0x0000 00B1
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

7

6

5

4

3

2

1

0

r

r

r

15

14

13

12

11

10

9

8

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

PREAMBLE[27:20]
r

r

r

r

r

Bits 31:8 Reserved, must be kept at reset value.

RM0456 Rev 6

<!-- pagebreak -->

3631

0x13C

CTI_CHOUTSTSR

<!-- pagebreak -->

Reset value

RM0456 Rev 6
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

Res.

Reset value

Reset value

Reset value

0

0
0

Reset value

CHOUTSTATUS[3:0]

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

TRIGINEN
[3:0]

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

Reset value

CHINSTATUS
[3:0]

0

Res.

0

Res.

0

Res.

0

Res.

Reset value

Res.

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

Reset value

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

X X X X

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

Reset value

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

Reserved

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

Reserved

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

Reset value

CTI_INEN0R to
CTI_INEN7R
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

Reserved

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

CTI_CHINSTSR

Res.

0x138
CTI_TRGOSTSR

Res.

0x134
CTI_TRGISTSR

Res.

0x0C00x12C

Res.

0x130
CTI_OUTEN0R to
CTI_OUTEN7R

Res.

0x0A0 to
0x0BC

Res.

0x0400x09C

Res.

0x020 to
0x03C
CTI_APPPULSER

Res.

0x01C
CTI_APPCLEAR

Res.

0x018
CTI_APPSETR

Res.

0x014
CTI_INTACKR

Res.

0x004
0x00C

Res.

0x010

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0
0

GLBEN

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

CTI_CONTROLR

Res.

0x000

Res.

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

Offset Register name

Res.

75.11.2

Res.

Debug support (DBG)
RM0456

Bits 7:0 PREAMBLE[27:20]: component identification bits [31:24]
0xB1: common identification value

CTI register map
Table 808. CTI register map and reset values

Reset value
0

Reserved
INTACK[7:0]

X X X X X X X X
APPSET[3:0]

APPCLEAR
[3:0]

0

0

0

0

0

0

APPPULSE
[3:0]

0

0

Reserved

TRIGOUTEN
[3:0]
0

Reserved

TRIGINSTATUS[7:0]

TRIGOUTSTATUS[7:0]
0

0

0

0

0

0

0xFFC
CTI_CIDR3

Reset value

RM0456 Rev 6
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

Reset value
1
1
1
1

CTI_PIDR2
REVISION
[3:0]
JEDEC
JEP106ID
[6:4]

0
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

0
1
0
0
0
0
0
0
1
0
0
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

NUMCH[3:0]

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

PARTNUM
[11:8]

Res.

Reserved

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

CTI_PIDR1
JEP106ID
[3:0]

Res.

0

Res.

NUMTRIG[7:0]

Reset value
0

Reset value
0

Reset value
0

Reset value
0

Reset value

Reset value
0

1

0

0

0

0

0

0

0

CLASS[3:0]
PREAMBLE
[11:8]

1
0

0

0

0

Res.

Res.

Res.

Reserved

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

Reset value

Reset value

Res.

Reset value

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

CTI_CIDR2

Res.

0xFF8
CTI_CIDR1

Res.

0xFF4
CTI_CIDR0

Res.

0xFF0
CTI_PIDR3

Res.

0xFEC

Res.

0xFE8

Res.

0xFE4
CTI_PIDR0

Res.

0xFE0

Res.

0xFD40xFDC
CTI_PIDR4

Res.

0xFD0
CTI_DEVTYPER

Res.

0xFCC
CTI_DEVIDR

Res.

0xFC8

Res.

0x1440xFC4

0

0

1

0

0

0

0

0

1

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

CTI_GATER

Res.

0x140

Res.

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

Offset Register name

Res.

RM0456
Debug support (DBG)

Table 808. CTI register map and reset values (continued)

Reset value

0

SUB[3:0]
0
1

SIZE[3:0]
0

1

0

0

0

1

0

1

GATEEN[3:0]
1

0

0

0

0

0

1

0

0

1

0

1

1

0

0

1

0

1

0

1

0

0

0

1

0

0

0

1

Reserved
EXTMUXNUM
[4:0]
0

MAJOR[3:0]

JEP106CON
[3:0]

0

Reserved
PARTNUM[7:0]

0
0

0
1

1

1

REVAND[3:0] CMOD[3:0]

PREAMBLE[7:0]

0
0

0
1

PREAMBLE[19:12]
0

1

PREAMBLE[27:20]

1

Refer to Table 798: Processor ROM table for register boundary addresses.

<!-- pagebreak -->

3631

Debug support (DBG)

75.12

RM0456

Microcontroller debug unit (DBGMCU)
The DBGMCU is a component containing a number of registers that control the power and
clock behavior in debug mode. It allows the debugger (or the software):

75.12.1

•

to maintain the clock and power to the processor cores when in low-power modes
(Sleep, Stop, or Standby mode)

•

to maintain the clock and power to the system debug and trace components when in
low-power modes

•

to stop the clock to certain peripherals (SMBUS timeout, watchdogs, timers, RTC)
when either processor core is stopped in debug mode

Device ID
The DBGMCU includes an identity code register, DBGMCU_IDCODE. This register
contains the ID code for the device. Debug tools can locate this register via the CoreSight
discovery procedure described in Table 75.5.

75.12.2

Low-power mode emulation
When the device enters either Stop mode (clocks are stopped) or Standby mode (core
power is switched off), the debugger can no longer access the debug access port and loses
the connection with the device. To avoid this, the debugger (or software) can set
DBG_STANDBY and/or DBG_STOP in DBGMCU_CR. These bits, when set, maintain the
clock and power to the processor while the device is in the corresponding low-power mode.
The processor remains in Sleep mode, and exits the low-power mode in the normal way.
Peripheral devices continue to operate, so the device behaviour may not be identical to
the one in the actual low-power mode.

75.12.3

Peripheral clock freeze
The DBGMCU peripheral clock freeze registers allow the operation of certain peripherals to
be suspended in debug mode. The peripheral units which support this feature are listed
in the table below.
Table 809. Peripheral clock freeze control bits
Bus

APB1L

<!-- pagebreak -->

Control register

DBGMCU_APB1LFZR

Peripheral

Description

I2C2

I2C2 SMBUS timeout

I2C1

I2C1 SMBUS timeout

IWDG

Independent watchdog

WWDG

Window watchdog

TIM7

General purpose timer 7

TIM6

General purpose timer 6

TIM5

General purpose timer 5

TIM4

General purpose timer 4

TIM3

General purpose timer 3

TIM2

General purpose timer 2

RM0456 Rev 6

RM0456

Debug support (DBG)
Table 809. Peripheral clock freeze control bits (continued)
Bus

APB1H

APB2

APB3

Control register

DGBMCU_APB1HFZR

DBGMCU_APB2FZR

DBGMCU_APB3FZR

Peripheral

Description

LPTIM2

Low power timer 2

I2C4

I2C4 SMBUS timeout

I2C5

I2C5 SMBUS timeout

I2C6

I2C6 SMBUS timeout

TIM17

General purpose timer 17

TIM16

General purpose timer 16

TIM15

General purpose timer 15

TIM8

General purpose timer 8

TIM1

General purpose timer 1

RTC

Real time clock

LPTIM4

Low power timer 4

LPTIM3

Low power timer 3

LPTIM1

Low power timer 1

I2C3

I2C3 SMBUS timeout

AHB1

DBGMCU_AHB1FZR

GPDMA 0 to 15

General purpose DMA channels 0 to 15

AHB3

DBGMCU_AHB3FZR

LPDMA 0 to 3

Low power DMA channels 0 to 3

Each peripheral unit or DMA channel has a corresponding control bit, DBG_xxx_STOP,
where xxx is the acronym of the peripheral (or DMA channel). The control bits are organized
in DBGMCU_zzzFZR registers, where zzz corresponds to the name of the bus (AHB or
APB). For example, DBGMCU_APB1LFZR contains the control bits for peripherals on the
APB1L bus.
The control bit, when set, causes the corresponding peripheral operation to be suspended
when the CPU is stopped in debug (HALTED = 1), according to the table below:
Table 810. Peripheral behavior in debug mode
HALTED

DBG_xxx_STOP

Peripheral behaviour

0

X

The operation continues.

1

0

The operation continues.

1

1

The operation is suspended.

The accessibility of DBG_xxx_STOP bits by the debugger depends on the state of the
authentication signal spiden.
When spiden = 1 (secure privilege debug enabled), all bits can be modified by a secure
access. Only bits corresponding to nonsecure peripherals (or DMA channels) can be
modified by a nonsecure access. All bits can be read by both nonsecure or secure
accesses.

RM0456 Rev 6

<!-- pagebreak -->

3631

Debug support (DBG)

RM0456

When spiden = 0 (secure privilege debug disabled), only nonsecure accesses are possible
(secure access requests by the debugger are converted to nonsecure by the CPU). Only
bits corresponding to nonsecure peripherals (or DMA channels) can be modified. All bits can
be read. This is summarized in the table below.
Table 811. Debugger access to freeze register bits
Peripheral xxx
status

spiden

Nonsecure

0
Secure

Nonsecure
1
Secure

Access security
attribute

DBG_xxx_STOP
can be modified?

DBG_xxx_STOP
can be read?

Nonsecure

Yes

Yes

(1)

Yes(1)

Secure

Yes

Nonsecure

No

Yes

Secure

No(1)

Yes(1)

Nonsecure

Yes

Yes

Secure

Yes

Yes

Nonsecure

No

Yes

Secure

Yes

Yes

1. When spiden = 0, secure access requests by the debugger are converted to nonsecure.

The status (secure or nonsecure) of a TrustZone–aware peripheral or a DMA channel, is
signaled to the DBGMCU by the peripheral.
The CPU access to the DBG_xxx_STOP bits does not depend on spiden. This access
depends only on the security status of the peripheral. The bits corresponding to a secure
peripheral (or DMA channel) can only be modified by a secure access (when CPU is in
secure state).

75.12.4

DBGMCU registers
The DBGMCU registers are not reset by a system reset, only by a power-on reset. They are
accessible to the debugger via the AHB access port, and to software, at base address
0xE004 4000.

DBGMCU identity code register (DBGMCU_IDCODE)
Address offset: 0x00
Reset value: 0xXXXX 6XXX
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

REV_ID[15:0]
r

r

r

r

r

r

r

r

r

r

r

r

r

r

r

r

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
r

r

r

r

r

r

r

r

r

r

<!-- pagebreak -->

DEV_ID[11:0]

RM0456 Rev 6

r

r

RM0456

Debug support (DBG)

Bits 31:16 REV_ID[15:0]: revision
For STM32U5Fx/5Gx
0x1001: revision Z
For STM32U59x/5Ax
0x3001: revision X
0x3002: revision W
For STM32U575/585
0x2001: revision X
0x3001: revision W
0x3007: revision U
For STM32U535/545
0x1001: revision Z
Bits 15:12 Reserved, must be kept at reset value.
Bits 11:0 DEV_ID[11:0]: device identification
0x455: STM32U535/545
0x476: STM32U5Fx/5Gx
0x481: STM32U59x/5Ax
0x482: STM32U575/585

DBGMCU configuration register (DBGMCU_CR)
Address offset: 0x04
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

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

TRACE_MODE
[1:0]
rw

rw

TRACE TRACE
_EN
_IOEN
rw

rw

Res.

DBG_S
DBG_S
TANDB
TOP
Y
rw

Res.

rw

Bits 31:8 Reserved, must be kept at reset value.
Bits 7:6 TRACE_MODE[1:0]: trace pin assignment
0x0: trace pins assigned for asynchronous mode (TRACESWO)
0x1: trace pins assigned for synchronous mode with a port width of 1 (TRACECK,
TRACED0)
0x2: trace pins assigned for synchronous mode with a port width of 2 ((TRACECK,
TRACED0-1)
0x3: trace pins assigned for synchronous mode with a port width of 4 ((TRACECK,
TRACED0-3)

RM0456 Rev 6

<!-- pagebreak -->

