275

Global TrustZone controller (GTZC)

RM0456

Bit 3 CRCF: illegal access flag for CRC
0: no illegal access event
1: illegal access event
Bit 2 FMACF: illegal access flag for FMAC
0: no illegal access event
1: illegal access event
Bit 1 CORDICF: illegal access flag for CORDIC
0: no illegal access event
1: illegal access event
Bit 0 MDF1F: illegal access flag for MDF1
0: no illegal access event
1: illegal access event

5.7.8

GTZC1 TZIC status register 4 (GTZC1_TZIC_SR4)
Address offset: 0x01C
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

23

22

MPCB
MPCB
MPCB
MPCB
MPCB
SRAM5
SRAM3
SRAM2
SRAM1
SRAM6
B5_RE
B3_RE
B2_RE
B1_RE
B6_RE
F
F
F
F
F
GF
GF
GF
GF
GF

21
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

TZIC1F

TZSC1
F

Res.

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

20

19

18

17

16

OCTOS
OCTOS
HSPI1_
BKPSR FSMC_
PI2_M
PI1_M
MEMF
AMF
MEMF
EMF
EMF
r

r

r

r

r

4

3

2

1

0

OTFDE OTFDE FLASH FLASH GPDM
C2F
C1F
F
_REGF
A1F
r

r

r

r

r

Bit 31 MPCBB5_REGF: illegal access flag for MPCBB5 registers
0: no illegal access event
1: illegal access event
Note: This bit is only available on some devices in the STM32U5 series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and kept at reset value.
Bit 30 SRAM5F: illegal access flag for SRAM5
0: no illegal access event
1: illegal access event
Note: This bit is only available on some devices in the STM32U5 series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and kept at reset value.
Bit 29 MPCBB3_REGF: illegal access flag for MPCBB3 registers
0: no illegal access event
1: illegal access event
Note: This bit is only available on some devices in the STM32U5 series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and kept at reset value.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Global TrustZone controller (GTZC)

Bit 28 SRAM3F: illegal access flag for SRAM3
0: no illegal access event
1: illegal access event
Note: This bit is only available on some devices in the STM32U5 series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and kept at reset value.
Bit 27 MPCBB2_REGF: illegal access flag for MPCBB2 registers
0: no illegal access event
1: illegal access event
Bit 26 SRAM2F: illegal access flag for SRAM2
0: no illegal access event
1: illegal access event
Bit 25 MPCBB1_REGF: illegal access flag for MPCBB1 registers
0: no illegal access event
1: illegal access event
Bit 24 SRAM1F: illegal access flag for SRAM1
0: no illegal access event
1: illegal access event
Bit 23 MPCBB6_REGF: illegal access flag for MPCBB6 registers
0: no illegal access event
1: illegal access event
Note: This bit is only available on some devices in the STM32U5 series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and kept at reset value.
Bit 22 SRAM6F: illegal access flag for SRAM6
0: no illegal access event
1: illegal access event
Note: This bit is only available on some devices in the STM32U5 series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and kept at reset value.
Bit 21 Reserved, must be kept at reset value.
Bit 20 HSPI1_MEMF: illegal access flag for HSPI1 memory bank
0: no illegal access event
1: illegal access event
Note: This bit is only available on some devices in the STM32U5 series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and kept at reset value.
Bit 19 OCTOSPI2_MEMF: illegal access flag for OCTOSPI2 memory bank
0: no illegal access event
1: illegal access event
Note: This bit is only available on some devices in the STM32U5 series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and kept at reset value.
Bit 18 BKPSRAMF: illegal access flag for MPCWM3 (BKPSRAM) memory bank
0: no illegal access event
1: illegal access event

RM0456 Rev 6

<!-- pagebreak -->

275

Global TrustZone controller (GTZC)

RM0456

Bit 17 FSMC_MEMF: illegal access flag for MPCWM2 (FSMC NAND) and MPCWM3 (FSMC NOR)
0: no illegal access event
1: illegal access event
Note: This bit is only available on some devices in the STM32U5 series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and kept at reset value.
Bit 16 OCTOSPI1_MEMF: illegal access flag for MPCWM1 (OCTOSPI1) memory bank
0: no illegal access event
1: illegal access event
Bit 15 TZIC1F: illegal access flag for GTZC1 TZIC registers
0: no illegal access event
1: illegal access event
Bit 14 TZSC1F: illegal access flag for GTZC1 TZSC registers
0: no illegal access event
1: illegal access event
Bits 13:5 Reserved, must be kept at reset value.
Bit 4 OTFDEC2F: illegal access flag for OTFDEC2
0: no illegal access event
1: illegal access event
Note: This bit is only available on some devices in the STM32U5 series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and kept at reset value.
Bit 3 OTFDEC1F: illegal access flag for OTFDEC1
0: no illegal access event
1: illegal access event
Bit 2 FLASHF: illegal access flag for flash memory
0: no illegal access event
1: illegal access event
Bit 1 FLASH_REGF: illegal access flag for FLASH registers
0: no illegal access event
1: illegal access event
Bit 0 GPDMA1F: illegal access flag for GPDMA1
0: no illegal access event
1: illegal access event

<!-- pagebreak -->

