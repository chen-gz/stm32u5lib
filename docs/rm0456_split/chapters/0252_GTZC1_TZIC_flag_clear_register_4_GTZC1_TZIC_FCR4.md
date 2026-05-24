275

Global TrustZone controller (GTZC)

5.7.12

RM0456

GTZC1 TZIC flag clear register 4 (GTZC1_TZIC_FCR4)
Address offset: 0x02C
Reset value: 0x0000 0000
Secure privilege access only.

31
CMPC
BB5_R
EGF

30

29

CSRA
M5F

CMPC
BB3_R
EGF

28

27

CSRA
M3F

CMPC
BB2_R
EGF

26

25

24

23

CSRA
M2F

CMPC
BB1_R
EGF

CSRA
M1F

CMPC
BB6_R
EGF

22
CSRA
M6F

w

w

w

w

w

w

w

w

w

w

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

CTZIC1 CTZSC
F
1F
w

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

w

21
Res.

5
Res.

20

19

18

CHSPI COCT
CBKPS
1_MEM OSPI2_
RAMF
F
MEMF

17

16

CFSM COCT
C_ME OSPI1_
MF
MEMF

w

w

w

w

w

4

3

2

1

0

CFLAS
COTFD COTFD CFLAS
CGPD
H_REG
EC2F
EC1F
HF
MA1F
F
w

w

w

w

w

Bit 31 CMPCBB5_REGF: clear the illegal access flag for MPCBB5 registers
0: no action
1: status flag cleared
Note: This bit is only available on some devices in the STM32U5 series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and kept at reset value.
Bit 30 CSRAM5F: clear the illegal access flag for SRAM5
0: no action
1: status flag cleared
Note: This bit is only available on some devices in the STM32U5 series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and kept at reset value.
Bit 29 CMPCBB3_REGF: clear the illegal access flag for MPCBB3 registers
0: no action
1: status flag cleared
Note: This bit is only available on some devices in the STM32U5 series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and kept at reset value.
Bit 28 CSRAM3F: clear the illegal access flag for SRAM3
0: no action
1: status flag cleared
Note: This bit is only available on some devices in the STM32U5 series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and kept at reset value.
Bit 27 CMPCBB2_REGF: clear the illegal access flag for MPCBB2 registers
0: no action
1: status flag cleared
Bit 26 CSRAM2F: clear the illegal access flag for SRAM2
0: no action
1: status flag cleared

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Global TrustZone controller (GTZC)

Bit 25 CMPCBB1_REGF: clear the illegal access flag for MPCBB1 registers
0: no action
1: status flag cleared
Bit 24 CSRAM1F: clear the illegal access flag for SRAM1
0: no action
1: status flag cleared
Bit 23 CMPCBB6_REGF: clear the illegal access flag for MPCBB6 registers
0: no action
1: status flag cleared
This bit is only available on some devices in the STM32U5 series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and kept at reset value.
Bit 22 CSRAM6F: clear the illegal access flag for SRAM6
0: no action
1: status flag cleared
Note: This bit is only available on some devices in the STM32U5 series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and kept at reset value.
Bit 21 Reserved, must be kept at reset value.
Bit 20 CHSPI1_MEMF: clear the illegal access flag for HSPI1 memory bank
0: no action
1: status flag cleared
Note: This bit is only available on some devices in the STM32U5 series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and kept at reset value.
Bit 19 COCTOSPI2_MEMF: clear the illegal access flag for OCTOSPI2 memory bank
0: no action
1: status flag cleared
Note: This bit is only available on some devices in the STM32U5 series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and kept at reset value.
Bit 18 CBKPSRAMF: clear the illegal access flag for MPCWM3 (BKPSRAM) memory bank
0: no action
1: status flag cleared
Bit 17 CFSMC_MEMF: clear the illegal access flag for MPCWM2 (FSMC NAND) and MPCWM3
(FSMC NOR)
0: no action
1: status flag cleared
Note: This bit is only available on some devices in the STM32U5 series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and kept at reset value.
Bit 16 COCTOSPI1_MEMF: clear the illegal access flag for MPCWM1 (OCTOSPI1) memory bank
0: no action
1: status flag cleared
Bit 15 CTZIC1F: clear the illegal access flag for GTZC1 TZIC registers
0: no action
1: status flag cleared

RM0456 Rev 6

<!-- pagebreak -->

