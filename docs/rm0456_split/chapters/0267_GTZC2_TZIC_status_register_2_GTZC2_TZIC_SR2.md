RM0456 Rev 6

r

r

r

r

RM0456

Global TrustZone controller (GTZC)

Bit 9 VREFBUFF: illegal access flag for VREFBUF
0: no illegal access event
1: illegal access event
Bit 8 ADC4F: illegal access flag for ADC4
0: no illegal access event
1: illegal access event
Bit 7 COMPF: illegal access flag for COMP
0: no illegal access event
1: illegal access event
Bit 6 OPAMPF: illegal access flag for OPAMP
0: no illegal access event
1: illegal access event
Bit 5 LPTIM4F: illegal access flag for LPTIM4
0: no illegal access event
1: illegal access event
Bit 4 LPTIM3F: illegal access flag for LPTIM3
0: no illegal access event
1: illegal access event
Bit 3 LPTIM1F: illegal access flag for LPTIM1
0: no illegal access event
1: illegal access event
Bit 2 I2C3F: illegal access flag for I2C3
0: no illegal access event
1: illegal access event
Bit 1 LPUART1F: illegal access flag for LPUART1
0: no illegal access event
1: illegal access event
Bit 0 SPI3F: illegal access flag for SPI3
0: no illegal access event
1: illegal access event

5.10.4

GTZC2 TZIC status register 2 (GTZC2_TZIC_SR2)
Address offset: 0x014
Reset value: 0x0000 0000
Secure privileged access only.

31

30

29

28

27

26

25

24

MPCB
SRAM4
B4_RE
F
GF

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

3

2

1

0

RTCF

SYSCF
GF

r

r

Res.

Res.

Res.

Res.

Res.

Res.

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

TZIC2F

TZSC2
F

Res.

Res.

Res.

Res.

Res.

Res.

Res.

EXTIF

LPDMA
1F

RCCF

r

r

r

r

r

RM0456 Rev 6

PWRF TAMPF
r

r

<!-- pagebreak -->

275

Global TrustZone controller (GTZC)

RM0456

Bits 31:26 Reserved, must be kept at reset value.
Bit 25 MPCBB4_REGF: illegal access flag for MPCBB4 registers
0: no illegal access event
1: illegal access event
Bit 24 SRAM4F: illegal access flag for SRAM4
0: no illegal access event
1: illegal access event
Bits 23:16 Reserved, must be kept at reset value.
Bit 15 TZIC2F: illegal access flag for GTZC2 TZIC registers
0: no illegal access event
1: illegal access event
Bit 14 TZSC2F: illegal access flag for GTZC2 TZSC registers
0: no illegal access event
1: illegal access event
Bits 13:7 Reserved, must be kept at reset value.
Bit 6 EXTIF: illegal access flag for EXTI
0: no illegal access event
1: illegal access event
Bit 5 LPDMA1F: illegal access flag for LPDMA
0: no illegal access event
1: illegal access event
Bit 4 RCCF: illegal access flag for RCC
0: no illegal access event
1: illegal access event
Bit 3 PWRF: illegal access flag for PWR
0: no illegal access event
1: illegal access event
Bit 2 TAMPF: illegal access flag for TAMP
0: no illegal access event
1: illegal access event
Bit 1 RTCF: illegal access flag for RTC
0: no illegal access event
1: illegal access event
Bit 0 SYSCFGF: illegal access flag for SYSCFG
0: no illegal access event
1: illegal access event

<!-- pagebreak -->

