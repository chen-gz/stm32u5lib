275

Global TrustZone controller (GTZC)

RM0456

Bit 5 DMA2DIE: illegal access interrupt enable for register of DMA2D
0: interrupt disabled
1: interrupt enabled
Note: This bit is only available on some devices in the STM32U5 series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and kept at reset value.
Bit 4 TSCIE: illegal access interrupt enable for TSC
0: interrupt disabled
1: interrupt enabled
Bit 3 CRCIE: illegal access interrupt enable for CRC
0: interrupt disabled
1: interrupt enabled
Bit 2 FMACIE: illegal access interrupt enable for FMAC
0: interrupt disabled
1: interrupt enabled
Bit 1 CORDICIE: illegal access interrupt enable for CORDIC
0: interrupt disabled
1: interrupt enabled
Bit 0 MDF1IE: illegal access interrupt enable for MDF1
0: interrupt disabled
1: interrupt enabled

5.7.4

GTZC1 TZIC interrupt enable register 4 (GTZC1_TZIC_IER4)
Address offset: 0x00C
Reset value: 0x0000 0000
Secure privileged access only.
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
IE
IE
IE
IE
IE
GIE
GIE
GIE
GIE
GIE
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

TZIC1I TZSC1I
E
E
rw

Res.

Res.

Res.

Res.

Res.

Res.

Res.

rw

Res.

21
Res.

5
Res.

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
MEMIE
AMIE MEMIE
EMIE
EMIE
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

FLASH
OTFDE OTFDE FLASHI
_REGI
C2IE
C1IE
E
E
rw

rw

rw

rw

GPDM
A1IE
rw

Bit 31 MPCBB5_REGIE: illegal access interrupt enable for MPCBB5 registers
0: interrupt disabled
1: interrupt enabled
Note: This bit is only available on some devices in the STM32U5 series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and kept at reset value.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Global TrustZone controller (GTZC)

Bit 30 SRAM5IE: illegal access interrupt enable for SRAM5
0: interrupt disabled
1: interrupt enabled
Note: This bit is only available on some devices in the STM32U5 series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and kept at reset value.
Bit 29 MPCBB3_REGIE: illegal access interrupt enable for MPCBB3 registers
0: interrupt disabled
1: interrupt enabled
Note: This bit is only available on some devices in the STM32U5 series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and kept at reset value.
Bit 28 SRAM3IE: illegal access interrupt enable for SRAM3
0: interrupt disabled
1: interrupt enabled
Note: This bit is only available on some devices in the STM32U5 series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and kept at reset value.
Bit 27 MPCBB2_REGIE: illegal access interrupt enable for MPCBB2 registers
0: interrupt disabled
1: interrupt enabled
Bit 26 SRAM2IE: illegal access interrupt enable for SRAM2
0: interrupt disabled
1: interrupt enabled
Bit 25 MPCBB1_REGIE: illegal access interrupt enable for MPCBB1 registers
0: interrupt disabled
1: interrupt enabled
Bit 24 SRAM1IE: illegal access interrupt enable for SRAM1
0: interrupt disabled
1: interrupt enabled
Bit 23 MPCBB6_REGIE: illegal access interrupt enable for MPCBB6 registers
0: interrupt disabled
1: interrupt enabled
Note: This bit is only available on some devices in the STM32U5 series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and kept at reset value.
Bit 22 SRAM6IE: illegal access interrupt enable for SRAM6 registers
0: interrupt disabled
1: interrupt enabled
Note: This bit is only available on some devices in the STM32U5 series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and kept at reset value.
Bit 21 Reserved, must be kept at reset value.

RM0456 Rev 6

<!-- pagebreak -->

275

Global TrustZone controller (GTZC)

RM0456

Bit 20 HSPI1_MEMIE: illegal access interrupt enable for HSPI1 memory bank
0: interrupt disabled
1: interrupt enabled
Note: This bit is only available on some devices in the STM32U5 series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and kept at reset value.
Bit 19 OCTOSPI2_MEMIE: illegal access interrupt enable for OCTOSPI2 memory bank
0: interrupt disabled
1: interrupt enabled
Note: This bit is only available on some devices in the STM32U5 series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and kept at reset value.
Bit 18 BKPSRAMIE: illegal access interrupt enable for MPCWM3 (BKPSRAM) memory bank
0: interrupt disabled
1: interrupt enabled
Bit 17 FSMC_MEMIE: illegal access interrupt enable for MPCWM2 (FSMC NAND) and MPCWM3
(FSMC NOR)
0: interrupt disabled
1: interrupt enabled
Note: This bit is only available on some devices in the STM32U5 series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and kept at reset value.
Bit 16 OCTOSPI1_MEMIE: illegal access interrupt enable for MPCWM1 (OCTOSPI1) memory bank
0: interrupt disabled
1: interrupt enabled
Bit 15 TZIC1IE: illegal access interrupt enable for GTZC1 TZIC registers
0: interrupt disabled
1: interrupt enabled
Bit 14 TZSC1IE: illegal access interrupt enable for GTZC1 TZSC registers
0: interrupt disabled
1: interrupt enabled
Bits 13:5 Reserved, must be kept at reset value.
Bit 4 OTFDEC2IE: illegal access interrupt enable for OTFDEC2
0: interrupt disabled
1: interrupt enabled
Note: This bit is only available on some devices in the STM32U5 series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and kept at reset value.
Bit 3 OTFDEC1IE: illegal access interrupt enable for OTFDEC1
0: interrupt disabled
1: interrupt enabled
Bit 2 FLASHIE: illegal access interrupt enable for flash memory
0: interrupt disabled
1: interrupt enabled
Bit 1 FLASH_REGIE: illegal access interrupt enable for FLASH registers
0: interrupt disabled
1: interrupt enabled

<!-- pagebreak -->

