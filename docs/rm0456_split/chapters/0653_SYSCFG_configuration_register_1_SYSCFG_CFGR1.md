RM0456 Rev 6

RM0456

System configuration controller (SYSCFG)

15.3.2

SYSCFG configuration register 1 (SYSCFG_CFGR1)
Address offset: 0x04
Reset value: 0x0000 0000 (for STM32U535/545/575/585)
Reset value: 0x1000 0000 (for STM32U59x/5Ax/5Fx/5Gx)
When the system is secure (TZEN = 1), this register can be a mix of secure and nonsecure
bits depending on the ADC security configuration bit in GTZC peripheral and GPIO security
bits. A nonsecure read/write access on secured bits is RAZ/WI.
When the system is not secure (TZEN = 0), there is no access restriction. This register can
be read and written by privileged and unprivileged access.

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

ENDCAP[1:0]

Res.

Res.

Res.

Res.

PB9_F
MP

PB8_F
MP

PB7_F
MP

PB6_F
MP

rw

rw

rw

rw

11

10

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

SRAM
CACHE
D

15

14

13

12

rw

Res.

Res.

Res.

Res.

Res.

Res.

rw

rw

9

8

ANAS BOOST
WVDD
EN
rw

rw

Bits 31:29 Reserved, must be kept at reset value.
Bit 28 SRAMCACHED: Enable internal SRAMs cachability by DCACHE2
0: SRAMx are not cached by DCACHE2
1: SRAMx are cached by DCACHE2
Note: This bit is only available on some devices of the STM32U5 Series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and keep it at reset value.
Bits 27:26 Reserved, must be kept at reset value.
Bits 25:24 ENDCAP[1:0]: Enable decoupling capacitance on HSPI supply
00: Capacitance OFF
01: 1/3 capacitance
10: 2/3 capacitance
11: Full capacitance
Note: This bitfield is only available on some devices of the STM32U5 Series. Refer to the
device datasheet for availability of its associated peripheral. If not present, consider this
bitfield as reserved and keep it at reset value.
Bits 23:20 Reserved, must be kept at reset value.
Bit 19 PB9_FMP: Fast-mode Plus driving capability activation on PB9
This bit can be read and written only with secure access if PB9 is secure in GPIOB. This bit
enables the Fm+ driving mode for PB9 when PB9 is not used by the I2C peripheral. This can
be used to drive a LED for instance.
0: PB9 pin operates in standard mode.
1: Fm+ mode is enabled on PB9 pin and the speed control is bypassed.

RM0456 Rev 6

<!-- pagebreak -->

