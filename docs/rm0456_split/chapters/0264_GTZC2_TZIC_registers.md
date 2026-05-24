275

Global TrustZone controller (GTZC)

5.10

RM0456

GTZC2 TZIC registers
All registers are accessed only by words (32-bit).

5.10.1

GTZC2 TZIC interrupt enable register 1 (GTZC2_TZIC_IER1)
Address offset: 0x000
Reset value: 0x0000 0000
Secure privilege access only.
This register is used to enable interrupt of illegal access.

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

DAC1I
ADF1IE
E
rw

rw

Res.

VREFB ADC4I
UFIE
E
rw

COMPI OPAMP LPTIM4 LPTIM3 LPTIM1
LPUAR
I2C3IE
SPI3IE
E
IE
IE
IE
IE
T1IE

rw

rw

rw

rw

Bits 31:13 Reserved, must be kept at reset value.
Bit 12 ADF1IE: illegal access interrupt enable for ADF1
0: interrupt disabled
1: interrupt enabled
Bit 11 DAC1IE: illegal access interrupt enable for DAC1
0: interrupt disabled
1: interrupt enabled
Bit 10 Reserved, must be kept at reset value.
Bit 9 VREFBUFIE: illegal access interrupt enable for VREFBUF
0: interrupt disabled
1: interrupt enabled
Bit 8 ADC4IE: illegal access interrupt enable for ADC4
0: interrupt disabled
1: interrupt enabled
Bit 7 COMPIE: illegal access interrupt enable for COMP
0: interrupt disabled
1: interrupt enabled
Bit 6 OPAMPIE: illegal access interrupt enable for OPAMP
0: interrupt disabled
1: interrupt enabled
Bit 5 LPTIM4IE: illegal access interrupt enable for LPTIM4
0: interrupt disabled
1: interrupt enabled
Bit 4 LPTIM3IE: illegal access interrupt enable for LPTIM3
0: interrupt disabled
1: interrupt enabled

<!-- pagebreak -->

