0
0
0
0
0
0
BKCMP4P
BKCMP3P
BKCMP2P
BKCMP1P
BKINP
BKCMP8E
BKCMP7E
BKCMP6E
BKCMP5E
BKCMP4E
BKCMP3E
BKCMP2E
BKCMP1E
BKINE

0
0
0
0
0
0
0
0
0
1

TIMx_AF2
OCRSEL
[2:0]
Res.
Res.
BK2CMP4P
BK2CMP3P
BK2CMP2P
BK2CMP1P

0
0
0
0

0
0
0
0
0

Reset value

0
0

0
0

0

TIMx_DMAR

0
0

Reserved

DBSS[3:0]

0

0

RM0456 Rev 6
0
0

0

0
0

Res.

0
0

0

TI2SEL[3:0]

0
0
0
0

DBL[4:0]

DMAB[31:0]
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

IPOS
[1:0]
0
0

Res.

24
23
22
21
20

Res.
Res.
Res.
Res.
Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

6
5
4
3
2
1
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

OC5M
[2:0]
OC5PE
OC5FE

8

9

10

11

12

13

14

15

16

17

18

7

CCR6[19:0]

0

OC5CE

0

Res.

25

Res.

Res.

19

26

Res.

Res.

CCR5[19:0]

Res.

0

Res.

Res.

Res.

0

Res.

0

0

Res.

OC6FE

0

Res.

Res.

OC6PE

0

Res.

0

Res.
BK2INE

0

Res.

Res.

0

Res.

0

BK2CMP1E

Res.

0

Res.

0

BK2CMP2E

0

Res.

OC6CE
0

Res.

0

BK2CMP3E

Res
ETRSEL
[3:0]
Res.

Res.

OC5M[3]
0

DTAE

0

Res.

Res.

Res.

Res.

Res.

0

BK2CMP4E

0

0

BK2CMP5E

TI3SEL[3:0]

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
1

Res.

0

0
0

0

BK2CMP6E

TI4SEL[3:0]
0

Res.

Res.

PW[7:0]

0

Res.

0
0

OC6M
[2:0]

DTPE

Reset value
0

BK2CMP7E

0

Res.

Res.

Res.

0

0

Res.

0

0

0

BK2INP

0

Res.

Res.

Res.

0

0

BK2CMP8E

0

Res.

Res.

Res.

Reset value
0

Res.

0

Res.

Res.

27

Res.

TIMx_CCR6
0

Res.

0

Res.

OC6M[3]

28

Res.

0

Res.

Res.

Res.

Res.

Res.

Res.

Res.

29

GC5C1

0

Res.

Res.

Res.

Res.

Res.

Res.

0

Res.

Res.

30

GC5C2

0

Res.

31

GC5C3

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

TIMx_CCR5

Res.

0

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Register name

Res.

0

Res.

Res.

Res.

PWPRSC[
2:0]

Res.

0

Res.
0

Res.

0

Res.
0

Res.

0

Res.

Res.

0

0

Res.

0

Res.

Res.

Res.

Res.

Reset value

Res.

0

Res.

0

Res.

0x068..
0x3D8
Res.

Reset value

Res.

Reset value

Reset value
Res.

Reset value

Res.

0x064

Res.

0x060
TIMx_AF1

Res.

TIMx_TISEL
Res.

TIMx_ECR

Res.

0x058

Res.

0x05C
TIMx_DTR2

Res.

0x054
TIMx_CCMR3

Res.

0x050

Res.

0x04C

Res.

0x048

Res.

Offset

Res.

0
0
0
0

0
0

Res.
Res.

0
0

IBLK
[1:0]
IDIR
[1:0]
IE

0
0
0
0
0

Res.

DTGF[7:0]

FIDX

0

0

Res.

Advanced-control timers (TIM1/TIM8)
RM0456

Table 555. TIMx register map and reset values (continued)

TI1SEL[3:0]

0

0

Refer to Section 2.3: Memory organization for the register boundary addresses.
0
0

0

0
0
0
0

DBA[4:0]

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

RM0456

General-purpose timers (TIM2/TIM3/TIM4/TIM5)

55

General-purpose timers (TIM2/TIM3/TIM4/TIM5)

55.1

TIM2/TIM3/TIM4/TIM5 introduction
The general-purpose timers consist of a 16-bit or 32-bit autoreload counter driven by a
programmable prescaler.
They can be used for a variety of purposes, including measuring the pulse lengths of input
signals (input capture) or generating output waveforms (output compare and PWM).
Pulse lengths and waveform periods can be modulated from a few microseconds to several
milliseconds using the timer prescaler and the RCC clock controller prescalers.
The timers are completely independent, and do not share any resources. They can be
synchronized together as described in Section 55.4.23: Timer synchronization.

55.2

TIM2/TIM3/TIM4/TIM5 main features
General-purpose TIMx timer features include:
•

16-bit or 32-bit up, down, up/down autoreload counter.

•

16-bit programmable prescaler used to divide (also “on the fly”) the counter clock
frequency by any factor between 1 and 65535.

•

Up to four independent channels for:
–

Input capture.

–

Output compare.

–

PWM generation (edge- and center-aligned modes).

–

One-pulse mode output.

•

Synchronization circuit to control the timer with external signals and to interconnect
several timers.

•

Interrupt/DMA generation on the following events:
–

Update: counter overflow/underflow, counter initialization (by software or
internal/external trigger).

–

Trigger event (counter start, stop, initialization, or count by internal/external
trigger).

–

Input capture.

–

Output compare.

•

Supports incremental (quadrature) encoder and hall-sensor circuitry for positioning
purposes.

•

Trigger input for external clock or cycle-by-cycle current management.

RM0456 Rev 6

<!-- pagebreak -->

