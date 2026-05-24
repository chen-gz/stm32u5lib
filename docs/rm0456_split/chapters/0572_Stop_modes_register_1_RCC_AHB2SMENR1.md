609

Reset and clock control (RCC)

11.8.38

RM0456

RCC AHB2 peripheral clock enable in Sleep and
Stop modes register 1 (RCC_AHB2SMENR1)
Address offset: 0x0B4
Reset value: 0xFFFF FFFF
Access: no wait state; word, half-word, and byte access
This register only configures the clock gating, not the clock source itself. When a bit is set
in Stop mode, the corresponding peripheral clock is enabled only when a peripheral
(this one or another) requests the AHB or APB clock (refer to Section 11.4.24).

31

30

SRAM3 SRAM2
SMEN SMEN
rw

rw

15

14

OTGH
SPHYS
MEN

OTGS
MEN

rw

rw

29
Res.

28

27

SDMM SDMM
C2SME C1SME
N
N

26
Res.

Res.

9

rw

rw

13

12

11

10

Res.

DCMI_
PSSIS
MEN

Res.

ADC12
SMEN

rw

25

rw

24

23

OTFDE OTFDE
C2SME C1SME
N
N
rw

rw

8

7

22
Res.

6

21

20

19

18

17

16

OCTOS
SAESS PKASM RNGS HASHS AESSM
PIMSM
MEN
EN
MEN
MEN
EN
EN
rw

rw

rw

rw

rw

rw

5

4

3

2

1

0

GPIOJ GPIOIS GPIOH GPIOG GPIOF GPIOE GPIOD GPIOC GPIOB GPIOA
SMEN
MEN
SMEN SMEN SMEN SMEN SMEN SMEN SMEN SMEN
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

Bit 31 SRAM3SMEN: SRAM3 clock enable during Sleep and Stop modes
This bit is set and cleared by software.
0: SRAM3 clocks disabled by the clock gating during Sleep and Stop modes
1: SRAM3 clocks enabled by the clock gating during Sleep and Stop modes
Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and keep it at reset value.
Bit 30 SRAM2SMEN: SRAM2 clock enable during Sleep and Stop modes
This bit is set and cleared by software.
0: SRAM2 clocks disabled by the clock gating during Sleep and Stop modes
1: SRAM2 clocks enabled by the clock gating during Sleep and Stop modes
Bit 29 Reserved, must be kept at reset value.
Bit 28 SDMMC2SMEN: SDMMC2 clock enable during Sleep and Stop modes
This bit is set and cleared by software.
0: SDMMC2 clocks disabled by the clock gating during Sleep and Stop modes
1: SDMMC2 clocks enabled by the clock gating during Sleep and Stop modes
Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and keep it at reset value.
Bit 27 SDMMC1SMEN: SDMMC1 clock enable during Sleep and Stop modes
This bit is set and cleared by software.
0: SDMMC1 clocks disabled by the clock gating during Sleep and Stop modes
1: SDMMC1 clocks enabled by the clock gating during Sleep and Stop modes
Bits 26:25 Reserved, must be kept at reset value.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Reset and clock control (RCC)

Bit 24 OTFDEC2SMEN: OTFDEC2 clock enable during Sleep and Stop modes
This bit is set and cleared by software.
0: OTFDEC2 clocks disabled by the clock gating during Sleep and Stop modes
1: OTFDEC2 clocks enabled by the clock gating during Sleep and Stop modes
Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and keep it at reset value.
Bit 23 OTFDEC1SMEN: OTFDEC1 clock enable during Sleep and Stop modes
This bit is set and cleared by software.
0: OTFDEC1 clocks disabled by the clock gating during Sleep and Stop modes
1: OTFDEC1 clocks enabled by the clock gating during Sleep and Stop modes
Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and keep it at reset value.
Bit 22 Reserved, must be kept at reset value.
Bit 21 OCTOSPIMSMEN: OCTOSPIM clock enable during Sleep and Stop modes
This bit is set and cleared by software.
0: OCTOSPIM clocks disabled by the clock gating during Sleep and Stop modes
1: OCTOSPIM clocks enabled by the clock gating during Sleep and Stop modes
Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and keep it at reset value.
Bit 20 SAESSMEN: SAES accelerator clock enable during Sleep and Stop modes
This bit is set and cleared by software.
0: SAES clocks disabled by the clock gating during Sleep and Stop modes
1: SAES clocks enabled by the clock gating during Sleep and Stop modes
Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and keep it at reset value.
Bit 19 PKASMEN: PKA clock enable during Sleep and Stop modes
This bit is set and cleared by software.
0: PKA clocks disabled by the clock gating during Sleep and Stop modes
1: PKA clocks enabled by the clock gating during Sleep and Stop modes
Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and keep it at reset value.
Bit 18 RNGSMEN: RNG clock enable during Sleep and Stop modes
This bit is set and cleared by software.
0: RNG clocks disabled by the clock gating during Sleep and Stop modes
1: RNG clocks enabled by the clock gating during Sleep and Stop modes
Bit 17 HASHSMEN: HASH clock enable during Sleep and Stop modes
This bit is set and cleared by software
0: HASH clocks disabled by the clock gating during Sleep and Stop modes
1: HASH clocks enabled by the clock gating during Sleep and Stop modes

RM0456 Rev 6

<!-- pagebreak -->

609

Reset and clock control (RCC)

RM0456

Bit 16 AESSMEN: AES clock enable during Sleep and Stop modes
This bit is set and cleared by software
0: AES clocks disabled by the clock gating during Sleep and Stop modes
1: AES clocks enabled by the clock gating during Sleep and Stop modes
Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and keep it at reset value.
Bit 15 OTGHSPHYSMEN: OTG_HS PHY clock enable during Sleep and Stop modes
This bit is set and cleared by software
0: OTG_HS PHY clocks disabled by the clock gating during Sleep and Stop modes
1: OTG_HS PHY clocks enabled by the clock gating during Sleep and Stop modes
Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and keep it at reset value.
Bit 14 OTGSMEN: OTG_FS and OTG_HS clocks enable during Sleep and Stop modes
This bit is set and cleared by software.
0: OTG_FS and OTG_HS clocks disabled by the clock gating during Sleep and Stop modes
1: OTG_FS and OTG_HS clocks enabled by the clock gating during Sleep and Stop modes
Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and keep it at reset value.
Bit 13 Reserved, must be kept at reset value.
Bit 12 DCMI_PSSISMEN: DCMI and PSSI clock enable during Sleep and Stop modes
This bit is set and cleared by software.
0: DCMI and PSSI clocks disabled by the clock gating during Sleep and Stop modes
1: DCMI and PSSI clocks enabled by the clock gating during Sleep and Stop modes
Bit 11 Reserved, must be kept at reset value.
Bit 10 ADC12SMEN: ADC1 and ADC2 clock enable during Sleep and Stop modes
This bit is set and cleared by software.
0: ADC1 and ADC2 clocks disabled by the clock gating during Sleep and Stop modes
1: ADC1 and ADC2 clocks enabled by the clock gating during Sleep and Stop modes
Note: This bit impacts ADC1 in STM32U535/545/575/585 and ADC1/ADC2
in STM32U59x/5Ax/5Fx/5Gx.
Bit 9 GPIOJSMEN: I/O port J clock enable during Sleep and Stop modes
This bit is set and cleared by software.
0: I/O port J clocks disabled by the clock gating during Sleep and Stop modes
1: I/O port J clocks enabled by the clock gating during Sleep and Stop modes
Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and keep it at reset value.
Bit 8 GPIOISMEN: I/O port I clocks enable during Sleep and Stop modes
This bit is set and cleared by software.
0: I/O port I clocks disabled by the clock gating during Sleep and Stop modes
1: I/O port I clocks enabled by the clock gating during Sleep and Stop modes
Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and keep it at reset value.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Reset and clock control (RCC)

Bit 7 GPIOHSMEN: I/O port H clocks enable during Sleep and Stop modes
This bit is set and cleared by software.
0: I/O port H clocks disabled by the clock gating during Sleep and Stop modes
1: I/O port H clocks enabled by the clock gating during Sleep and Stop modes
Bit 6 GPIOGSMEN: I/O port G clocks enable during Sleep and Stop modes
This bit is set and cleared by software.
0: I/O port G clocks disabled by the clock gating during Sleep and Stop modes
1: I/O port G clocks enabled by the clock gating during Sleep and Stop modes
Bit 5 GPIOFSMEN: I/O port F clocks enable during Sleep and Stop modes
This bit is set and cleared by software.
0: I/O port F clocks disabled by the clock gating during Sleep and Stop modes
1: I/O port F clocks enabled by the clock gating during Sleep and Stop modes
Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and keep it at reset value.
Bit 4 GPIOESMEN: I/O port E clocks enable during Sleep and Stop modes
This bit is set and cleared by software.
0: I/O port E clocks disabled by the clock gating during Sleep and Stop modes
1: I/O port E clocks enabled by the clock gating during Sleep and Stop modes
Bit 3 GPIODSMEN: I/O port D clocks enable during Sleep and Stop modes
This bit is set and cleared by software.
0: I/O port D clocks disabled by the clock gating during Sleep and Stop modes
1: I/O port D clocks enabled by the clock gating during Sleep and Stop modes
Bit 2 GPIOCSMEN: I/O port C clocks enable during Sleep and Stop modes
This bit is set and cleared by software.
0: I/O port C clocks disabled by the clock gating during Sleep and Stop modes
1: I/O port C clocks enabled by the clock gating during Sleep and Stop modes
Bit 1 GPIOBSMEN: I/O port B clocks enable during Sleep and Stop modes
This bit is set and cleared by software.
0: I/O port B clocks disabled by the clock gating during Sleep and Stop modes
1: I/O port B clocks enabled by the clock gating during Sleep and Stop modes
Bit 0 GPIOASMEN: I/O port A clocks enable during Sleep and Stop modes
This bit is set and cleared by software.
0: I/O port A clocks disabled by the clock gating during Sleep and Stop modes
1: I/O port A clocks enabled by the clock gating during Sleep and Stop modes

RM0456 Rev 6

<!-- pagebreak -->

