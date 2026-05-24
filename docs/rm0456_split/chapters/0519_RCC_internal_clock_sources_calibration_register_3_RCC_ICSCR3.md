RM0456 Rev 6

RM0456

Reset and clock control (RCC)

Bits 14:10 MSITRIM1[4:0]: MSI clock trimming for ranges 4 to 7
These bits provide an additional user-programmable trimming value that is added to the
factory-programmed calibration trim value MSIRC1[4:0] bits. It can be programmed to adjust
to voltage and temperature variations that influence the frequency of the MSI.
Bits 9:5 MSITRIM2[4:0]: MSI clock trimming for ranges 8 to 11
These bits provide an additional user-programmable trimming value that is added to the
factory-programmed calibration trim value MSIRC2[4:0] bits. It can be programmed to adjust
to voltage and temperature variations that influence the frequency of the MSI.
Bits 4:0 MSITRIM3[4:0]: MSI clock trimming for ranges 12 to 15
These bits provide an additional user-programmable trimming value that is added to the
factory-programmed calibration trim value MSIRC3[4:0] bits. It can be programmed to adjust
to voltage and temperature variations that influence the frequency of the MSI.

Note:

The hardware auto calibration with LSE must not be used in conjunction with software
calibration.

11.8.4

RCC internal clock sources calibration register 3 (RCC_ICSCR3)
Address offset: 0x010
Reset value: 0x0010 0XXX
X is factory-programmed.
Access: no wait state; word, half-word, and byte access

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

Res.

Res.

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

Res.

Res.

Res.

Res.
r

r

r

r

r

20

19

18

17

16

HSITRIM[4:0]
rw

rw

rw

rw

rw

4

3

2

1

0

r

r

r

r

r

HSICAL[11:0]
r

r

Bits 31:21 Reserved, must be kept at reset value.
Bits 20:16 HSITRIM[4:0]: HSI clock trimming
These bits provide an additional user-programmable trimming value. It can be programmed
to adjust to voltage and temperature variations that influence the frequency of the HSI.
Bits 15:12 Reserved, must be kept at reset value.
Bits 11:0 HSICAL[11:0]: HSI clock calibration
These bits are initialized at startup with the factory-programmed HSI calibration trim value.

RM0456 Rev 6

<!-- pagebreak -->

