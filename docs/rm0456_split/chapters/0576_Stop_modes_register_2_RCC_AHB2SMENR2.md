609

Reset and clock control (RCC)

11.8.39

RM0456

RCC AHB2 peripheral clock enable in Sleep and
Stop modes register 2 (RCC_AHB2SMENR2)
Address offset: 0x0B8
Reset value: 0xFFFF FFFF
Access: no wait state; word, half-word and byte access
This register only configures the clock gating, not the clock source itself.
When a bit is set in Stop mode, the corresponding peripheral clock is enabled only when a
peripheral (this one or another) requests AHB or APB clock (refer to Section 11.4.24).

31

30

SRAM5 SRAM6
SMEN SMEN

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

HSPI1
SMEN

Res.

Res.

Res.

OCTOS
PI2SM
EN

Res.

Res.

Res.

OCTOS
PI1SM
EN

Res.

Res.

Res.

FSMCS
MEN

rw

rw

rw

rw

Bit 31 SRAM5SMEN: SRAM5 clock enable during Sleep and Stop modes
This bit is set and cleared by software.
0: SRAM5 clocks disabled by the clock gating during Sleep and Stop modes
1: SRAM5 clocks enabled by the clock gating during Sleep and Stop modes
Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and keep it at reset value.
Bit 30 SRAM6SMEN: SRAM6 clock enable during Sleep and Stop modes
This bit is set and cleared by software.
0: SRAM6 clocks disabled by the clock gating during Sleep and Stop modes
1: SRAM6 clocks enabled by the clock gating during Sleep and Stop modes
Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and keep it at reset value.
Bits 29:13 Reserved, must be kept at reset value.
Bit 12 HSPI1SMEN: HSPI1 clock enable during Sleep and Stop modes
This bit is set and cleared by software.
0: HSPI1 clocks disabled by the clock gating during Sleep and Stop modes
1: HSPI1 clocks enabled by the clock gating during Sleep and Stop modes
Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and keep it at reset value.
Bits 11:9 Reserved, must be kept at reset value.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Reset and clock control (RCC)

Bit 8 OCTOSPI2SMEN: OCTOSPI2 clock enable during Sleep and Stop modes
This bit is set and cleared by software.
0: OCTOSPI2 clocks disabled by the clock gating during Sleep and Stop modes
1: OCTOSPI2 clocks enabled by the clock gating during Sleep and Stop modes
Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and keep it at reset value.
Bits 7:5 Reserved, must be kept at reset value.
Bit 4 OCTOSPI1SMEN: OCTOSPI1 clock enable during Sleep and Stop modes
This bit is set and cleared by software.
0: OCTOSPI1 clocks disabled by the clock gating during Sleep and Stop modes
1: OCTOSPI1 clocks enabled by the clock gating during Sleep and Stop modes
Bits 3:1 Reserved, must be kept at reset value.
Bit 0 FSMCSMEN: FSMC clock enable during Sleep and Stop modes
This bit is set and cleared by software.
0: FSMC clocks disabled by the clock gating during Sleep and Stop modes
1: FSMC clocks enabled by the clock gating during Sleep and Stop modes
Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and keep it at reset value.

11.8.40

RCC AHB3 peripheral clock enable in Sleep and Stop modes register
(RCC_AHB3SMENR)
Address offset: 0x0BC
Reset value: 0xFFFF FFFF
Access: no wait state; word, half-word, and byte access
This register only configures the clock gating, not the clock source itself. When a bit is set
in Stop mode, the corresponding peripheral clock is enabled only when a peripheral
(this one or another) requests the AHB or APB clock (refer to Section 11.4.24).

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

SRAM4
SMEN

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

LPGPI
O1SME
N

rw
15
Res.

Res.

Res.

GTZC2
SMEN
rw

Res.

ADF1S LPDMA
MEN 1SMEN
rw

Res.

Res.

rw

DAC1S ADC4S
MEN
MEN
rw

rw

Res.

Res.

PWRS
MEN
rw

rw

Bit 31 SRAM4SMEN: SRAM4 clock enable during Sleep and Stop modes
This bit is set and cleared by software.
0: SRAM4 clocks disabled by the clock gating during Sleep and Stop modes
1: SRAM4 clocks enabled by the clock gating during Sleep and Stop modes
Bits 30:13 Reserved, must be kept at reset value.

RM0456 Rev 6

<!-- pagebreak -->

609

Reset and clock control (RCC)

RM0456

Bit 12 GTZC2SMEN: GTZC2 clock enable during Sleep and Stop modes
This bit is set and cleared by software.
0: GTZC2 clock disabled by the clock gating during Sleep and Stop modes
1: GTZC2 clock enabled by the clock gating during Sleep and Stop modes
Bit 11 Reserved, must be kept at reset value.
Bit 10 ADF1SMEN: ADF1 clock enable during Sleep and Stop modes
This bit is set and cleared by software.
0: ADF1 clock disabled by the clock gating during Sleep and Stop modes
1: ADF1 clock enabled by the clock gating during Sleep and Stop modes
Note: This bit must be set to allow the peripheral to wake up from Stop modes.
Bit 9 LPDMA1SMEN: LPDMA1 clock enable during Sleep and Stop modes
This bit is set and cleared by software.
0: LPDMA1 clock disabled by the clock gating during Sleep and Stop modes
1: LPDMA1 clock enabled by the clock gating during Sleep and Stop modes
Note: This bit must be set to allow the peripheral to wake up from Stop modes.
Bits 8:7 Reserved, must be kept at reset value.
Bit 6 DAC1SMEN: DAC1 clock enable during Sleep and Stop modes
This bit is set and cleared by software.
0: DAC1 clock disabled by the clock gating during Sleep and Stop modes
1: DAC1 clock enabled by the clock gating during Sleep and Stop modes
Note: This bit must be set to allow the peripheral to wake up from Stop modes.
Bit 5 ADC4SMEN: ADC4 clock enable during Sleep and Stop modes
This bit is set and cleared by software.
0: ADC4 clock disabled by the clock gating during Sleep and Stop modes
1: ADC4 clock enabled by the clock gating during Sleep and Stop modes
Note: This bit must be set to allow the peripheral to wake up from Stop modes.
Bits 4:3 Reserved, must be kept at reset value.
Bit 2 PWRSMEN: PWR clock enable during Sleep and Stop modes
This bit is set and cleared by software.
0: PWR clock disabled by the clock gating during Sleep and Stop modes
1: PWR clock enabled by the clock gating during Sleep and Stop modes
Bit 1 Reserved, must be kept at reset value.
Bit 0 LPGPIO1SMEN: LPGPIO1 enable during Sleep and Stop modes
This bit is set and cleared by software.
0: LPGPIO1 clock disabled by the clock gating during Sleep and Stop modes
1: LPGPIO1 clock enabled by the clock gating during Sleep and Stop modes

<!-- pagebreak -->

