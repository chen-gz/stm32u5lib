275

Global TrustZone controller (GTZC)

RM0456

CADC4F

CCOMPF

COPAMPF

CLPTIM4F

CLPTIM3F

CLPTIM1F

CI2C3F

CLPUART1F

CSPI3F

0

0

0

0

0

0

0

0

0

Res.

Res.

CEXTIF

CLPDMA1F

CRCCF

CPWRF

CTAMPF

CRTCF

CSYSCFGF

Res.

CVREFBUFF
0

Res.

CDAC1F
0

Res.

Res.

0

Res.

0

CADF1F

CTZSC2F

0

Res.

CTZIC2F

Res.

Res.

Res.

Res.

Res.

Res.

0

Res.

0

Res.

CSRAM4F

Reset value

CMPCBB4_REGF

Res.

Res.

Res.

Res.

Res.

GTZC2_TZIC_
FCR2

0x024

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

GTZC2_TZIC_
FCR1

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

0x020

Res.

Register name

Res.

Offset

Res.

Table 43. GTZC2 TZIC register map and reset values (continued)

0

0

0

0

0

0

0

Refer to Table 30: GTZC2 subblocks address offset.

5.11

GTZC2 MPCBB4 registers
All registers are accessed only by words (32-bit).

5.11.1

GTZC2 SRAM4 MPCBB control register (GTZC2_MPCBB4_CR)
Address offset: 0x000
Reset value: 0x0000 0000
Secure privileged access only.

31

30

INVSE
SRWIL
CSTAT
ADIS
E

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

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

GLOCK
rs

Bit 31 SRWILADIS: secure read/write illegal access disable
This bit disables the detection of an illegal access when a secure read/write transaction
access a nonsecure blocks of the block-based SRAM (secure fetch on nonsecure block is
always considered illegal).
0: enabled, secure read/write access not allowed on nonsecure SRAM block
1: disabled, secure read/write access allowed on nonsecure SRAM block
Bit 30 INVSECSTATE: SRAMx clocks security state
This bit is used to define the internal SRAMs clocks control in RCC as secure or not.
0: SRAMs clocks are secure if a secure area exists in the MPCBB. It is nonsecure if there is
no secure area.
1: SRAMs clocks are nonsecure even if a secure area exists in the MPCBB, and secure
even if no secure block is set in the MPCBB.
Bits 29:1 Reserved, must be kept at reset value.

<!-- pagebreak -->

