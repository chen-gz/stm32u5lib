RM0456 Rev 6

RM0456

Reset and clock control (RCC)

Bit 3 TIM5SMEN: TIM5 clock enable during Sleep and Stop modes
This bit is set and cleared by software.
0: TIM5 clocks disabled by the clock gating during Sleep and Stop modes
1: TIM5 clocks enabled by the clock gating during Sleep and Stop modes
Bit 2 TIM4SMEN: TIM4 clock enable during Sleep and Stop modes
This bit is set and cleared by software.
0: TIM4 clocks disabled by the clock gating during Sleep and Stop modes
1: TIM4 clocks enabled by the clock gating during Sleep and Stop modes
Bit 1 TIM3SMEN: TIM3 clock enable during Sleep and Stop modes
This bit is set and cleared by software.
0: TIM3 clocks disabled by the clock gating during Sleep and Stop modes
1: TIM3 clocks enabled by the clock gating during Sleep and Stop modes
Bit 0 TIM2SMEN: TIM2 clock enable during Sleep and Stop modes
This bit is set and cleared by software.
0: TIM2 clocks disabled by the clock gating during Sleep and Stop modes
1: TIM2 clocks enabled by the clock gating during Sleep and Stop modes

11.8.42

RCC APB1 peripheral clocks enable in Sleep and
Stop modes register 2 (RCC_APB1SMENR2)
Address offset: 0x0C8
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

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

UCPD1
SMEN

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

FDCAN
1SMEN

Res.

I2C6S
MEN

Res.

I2C4S
MEN

Res.

rw

Res.

Res.

Res.

Res.

Res.

rw

rw

I2C5S LPTIM2
MEN
SMEN
rw

rw

Res.

Res.

rw

Bits 31:24 Reserved, must be kept at reset value.
Bit 23 UCPD1SMEN: UCPD1 clock enable during Sleep and Stop modes
This bit is set and cleared by software.
0: UCPD1 clocks disabled by the clock gating during Sleep and Stop modes
1: UCPD1 clocks enabled by the clock gating during Sleep and Stop modes
Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and keep it at reset value.
Bits 22:10 Reserved, must be kept at reset value.

RM0456 Rev 6

<!-- pagebreak -->

609

Reset and clock control (RCC)

RM0456

Bit 9 FDCAN1SMEN: FDCAN1 clock enable during Sleep and Stop modes
This bit is set and cleared by software.
0: FDCAN1 clocks disabled by the clock gating during Sleep and Stop modes
1: FDCAN1 clocks enabled by the clock gating during Sleep and Stop modes
Note: This bit is only available on some devices in the STM32U5 series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and kept at reset value.
Bit 8 Reserved, must be kept at reset value.
Bit 7 I2C6SMEN: I2C6 clock enable during Sleep and Stop modes
This bit is set and cleared by software
0: I2C6 clocks disabled by the clock gating during Sleep and Stop modes
1: I2C6 clocks enabled by the clock gating during Sleep and Stop modes
Note: This bit must be set to allow the peripheral to wake up from Stop modes.
This bit is only available on some devices in the STM32U5 Series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and keep it at reset value.
Bit 6 I2C5SMEN: I2C5 clock enable during Sleep and Stop modes
This bit is set and cleared by software
0: I2C5 clocks disabled by the clock gating during Sleep and Stop modes
1: I2C5 clocks enabled by the clock gating during Sleep and Stop modes
Note: This bit must be set to allow the peripheral to wake up from Stop modes.
This bit is only available on some devices in the STM32U5 Series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and keep it at reset value.
Bit 5 LPTIM2SMEN: LPTIM2 clock enable during Sleep and Stop modes
This bit is set and cleared by software.
0: LPTIM2 clocks disabled by the clock gating during Sleep and Stop modes
1: LPTIM2 clocks enabled by the clock gating during Sleep and Stop modes
Note: This bit must be set to allow the peripheral to wake up from Stop modes.
Bits 4:2 Reserved, must be kept at reset value.
Bit 1 I2C4SMEN: I2C4 clock enable during Sleep and Stop modes
This bit is set and cleared by software
0: I2C4 clocks disabled by the clock gating during Sleep and Stop modes
1: I2C4 clocks enabled by the clock gating during Sleep and Stop modes
Note: This bit must be set to allow the peripheral to wake up from Stop modes.
Bit 0 Reserved, must be kept at reset value.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Reset and clock control (RCC)

11.8.43

RCC APB2 peripheral clocks enable in Sleep and Stop modes register
(RCC_APB2SMENR)
Address offset: 0x0CC
Reset value: 0xFFFF FFFF
Access: word, half-word, and byte access
This register only configures the clock gating, not the clock source itself. When a bit is set
in Stop mode, the corresponding peripheral clock is enabled only when a peripheral
(this one or another) requests the AHB or APB clock (refer to Section 11.4.24).

31

30

29

28

27

26

DSISM LTDCS
EN
MEN

25

24

23

22

21

20

19

GFXTI
MSME
N

USBS
MEN

Res.

SAI2S
MEN

SAI1S
MEN

Res.

Res.

18

17

16

TIM17S TIM16S TIM15S
MEN
MEN
MEN

Res.

Res.

Res.

Res.

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

5

4

3

2

1

0

SPI1S
MEN

TIM1S
MEN

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

Res.

USART TIM8S
1SMEN MEN
rw

rw

Bits 31:28 Reserved, must be kept at reset value.
Bit 27 DSISMEN: DSI clock enable during Sleep and Stop modes
This bit is set and cleared by software.
0: DSI clocks disabled by the clock gating during Sleep and Stop modes
1: DSI clocks enabled by the clock gating during Sleep and Stop modes
Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and keep it at reset value.
Bit 26 LTDCSMEN: LTDC clock enable during Sleep and Stop modes
This bit is set and cleared by software.
0: LTDC clocks disabled by the clock gating during Sleep and Stop modes
1: LTDC clocks enabled by the clock gating during Sleep and Stop modes
Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and keep it at reset value.
Bit 25 GFXTIMSMEN: GFXTIM clock enable during Sleep and Stop modes
This bit is set and cleared by software.
0: GFXTIM clocks disabled by the clock gating during Sleep and Stop modes
1: GFXTIM clocks enabled by the clock gating during Sleep and Stop modes
Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and keep it at reset value.
Bit 24 USBSMEN: USB clock enable during Sleep and Stop modes
This bit is set and cleared by software.
0: USB clocks disabled by the clock gating during Sleep and Stop modes
1: USB clocks enabled by the clock gating during Sleep and Stop modes
Note: This bit is only available on STM32U535/545 devices, it is reserved on other devices in
the STM32U5 Series. If not present, consider this bit as reserved and keep it at reset
value.

RM0456 Rev 6

<!-- pagebreak -->

609

Reset and clock control (RCC)

RM0456

Bit 23 Reserved, must be kept at reset value.
Bit 22 SAI2SMEN: SAI2 clock enable during Sleep and Stop modes
This bit is set and cleared by software.
0: SAI2 clocks disabled by the clock gating during Sleep and Stop modes
1: SAI2 clocks enabled by the clock gating during Sleep and Stop modes
Note: This bit is only available on some devices in the STM32U5 Series.Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and keep it at reset value.
Bit 21 SAI1SMEN: SAI1 clock enable during Sleep and Stop modes
This bit is set and cleared by software.
0: SAI1 clocks disabled by the clock gating during Sleep and Stop modes
1: SAI1 clocks enabled by the clock gating during Sleep and Stop modes
Bits 20:19 Reserved, must be kept at reset value.
Bit 18 TIM17SMEN: TIM17 clock enable during Sleep and Stop modes
This bit is set and cleared by software.
0: TIM17 clocks disabled by the clock gating during Sleep and Stop modes
1: TIM17 clocks enabled by the clock gating during Sleep and Stop modes
Bit 17 TIM16SMEN: TIM16 clock enable during Sleep and Stop modes
This bit is set and cleared by software.
0: TIM16 clocks disabled by the clock gating during Sleep and Stop modes
1: TIM16 clocks enabled by the clock gating during Sleep and Stop modes
Bit 16 TIM15SMEN: TIM15 clock enable during Sleep and Stop modes
This bit is set and cleared by software.
0: TIM15 clocks disabled by the clock gating during Sleep and Stop modes
1: TIM15 clocks enabled by the clock gating during Sleep and Stop modes
Bit 15 Reserved, must be kept at reset value.
Bit 14 USART1SMEN: USART1 clock enable during Sleep and Stop modes
This bit is set and cleared by software.
0: USART1clocks disabled by the clock gating during Sleep and Stop modes
1: USART1clocks enabled by the clock gating during Sleep and Stop modes
Note: This bit must be set to allow the peripheral to wake up from Stop modes.
Bit 13 TIM8SMEN: TIM8 clock enable during Sleep and Stop modes
This bit is set and cleared by software.
0: TIM8 clocks disabled by the clock gating during Sleep and Stop modes
1: TIM8 clocks enabled by the clock gating during Sleep and Stop modes
Bit 12 SPI1SMEN: SPI1 clock enable during Sleep and Stop modes
This bit is set and cleared by software.
0: SPI1 clocks disabled by the clock gating during Sleep and Stop modes
1: SPI1 clocks enabled by the clock gating during Sleep and Stop modes
Note: This bit must be set to allow the peripheral to wake up from Stop modes.
Bit 11 TIM1SMEN: TIM1 clock enable during Sleep and Stop modes
This bit is set and cleared by software.
0: TIM1 clocks disabled by the clock gating during Sleep and Stop modes
1: TIM1 clocks enabled by the clock gating during Sleep and Stop modes
Bits 10:0 Reserved, must be kept at reset value.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Reset and clock control (RCC)

11.8.44

RCC APB3 peripheral clock enable in Sleep and Stop modes register
(RCC_APB3SMENR)
Address offset: 0x0D0
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

I2C3S
MEN

LPUAR
T1SME
N

SPI3S
MEN

rw

rw

rw

COMP OPAMP LPTIM4 LPTIM3 LPTIM1
SMEN SMEN SMEN SMEN SMEN
rw

rw

rw

rw

Res.

Res.

Res.

rw

20

RTCAP
VREFS
BSME
MEN
N
rw

rw

5

4
Res.

19

18

17

16

Res.

Res.

Res.

Res.

3

2

1

0

Res.

SYSCF
GSME
N

Res.

Res.

rw

Bits 31:22 Reserved, must be kept at reset value.
Bit 21 RTCAPBSMEN: RTC and TAMP APB clock enable during Sleep and Stop modes
This bit is set and cleared by software.
0: RTC and TAMP APB clock disabled by the clock gating during Sleep and Stop modes
1: RTC and TAMP APB clock enabled by the clock gating during Sleep and Stop modes
Note: This bit must be set to allow the peripheral to wake up from Stop modes.
Bit 20 VREFSMEN: VREFBUF clock enable during Sleep and Stop modes
This bit is set and cleared by software.
0: VREFBUF clocks disabled by the clock gating during Sleep and Stop modes
1: VREFBUF clocks enabled by the clock gating during Sleep and Stop modes
Bits 19:16 Reserved, must be kept at reset value.
Bit 15 COMPSMEN: COMP clock enable during Sleep and Stop modes
This bit is set and cleared by software.
0: COMP clocks disabled by the clock gating during Sleep and Stop modes
1: COMP clocks enabled by the clock gating during Sleep and Stop modes
Bit 14 OPAMPSMEN: OPAMP clock enable during Sleep and Stop modes
This bit is set and cleared by software.
0: OPAMP clocks disabled by the clock gating during Sleep and Stop modes
1: OPAMP clocks enabled by the clock gating during Sleep and Stop modes
Bit 13 LPTIM4SMEN: LPTIM4 clock enable during Sleep and Stop modes
This bit is set and cleared by software.
0: LPTIM4 clocks disabled by the clock gating during Sleep and Stop modes
1: LPTIM4 clocks enabled by the clock gating during Sleep and Stop modes
Note: This bit must be set to allow the peripheral to wake up from Stop modes.

RM0456 Rev 6

<!-- pagebreak -->

609

Reset and clock control (RCC)

RM0456

Bit 12 LPTIM3SMEN: LPTIM3 clock enable during Sleep and Stop modes
This bit is set and cleared by software.
0: LPTIM3 clocks disabled by the clock gating during Sleep and Stop modes
1: LPTIM3 clocks enabled by the clock gating during Sleep and Stop modes
Note: This bit must be set to allow the peripheral to wake up from Stop modes.
Bit 11 LPTIM1SMEN: LPTIM1 clock enable during Sleep and Stop modes
This bit is set and cleared by software.
0: LPTIM1 clocks disabled by the clock gating during Sleep and Stop modes
1: LPTIM1 clocks enabled by the clock gating during Sleep and Stop modes
Note: This bit must be set to allow the peripheral to wake up from Stop modes.
Bits 10:8 Reserved, must be kept at reset value.
Bit 7 I2C3SMEN: I2C3 clock enable during Sleep and Stop modes
This bit is set and cleared by software.
0: I2C3 clocks disabled by the clock gating during Sleep and Stop modes
1: I2C3 clocks enabled by the clock gating during Sleep and Stop modes
Note: This bit must be set to allow the peripheral to wake up from Stop modes.
Bit 6 LPUART1SMEN: LPUART1 clock enable during Sleep and Stop modes
This bit is set and cleared by software.
0: LPUART1 clocks disabled by the clock gating during Sleep and Stop modes
1: LPUART1 clocks enabled by the clock gating during Sleep and Stop modes
Note: This bit must be set to allow the peripheral to wake up from Stop modes.
Bit 5 SPI3SMEN: SPI3 clock enable during Sleep and Stop modes
This bit is set and cleared by software.
0: SPI3 clocks disabled by the clock gating during Sleep and Stop modes
1: SPI3 clocks enabled by the clock gating during Sleep and Stop modes
Note: This bit must be set to allow the peripheral to wake up from Stop modes.
Bits 4:2 Reserved, must be kept at reset value.
Bit 1 SYSCFGSMEN: SYSCFG clock enable during Sleep and Stop modes
This bit is set and cleared by software.
0: SYSCFG clocks disabled by the clock gating during Sleep and Stop modes
1: SYSCFG clocks enabled by the clock gating during Sleep and Stop modes
Bit 0 Reserved, must be kept at reset value.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Reset and clock control (RCC)

11.8.45

RCC SmartRun domain peripheral autonomous mode register
(RCC_SRDAMR)
Address offset: 0x0D8
Reset value: 0x0000 0000
Access: no wait state; word, half-word, and byte access

31
SRAM4
AMEN

30
Res.

rw
15

14

29

28

27

rw

25

24

23

22

Res.

Res.

Res.

21

20

RTCAP
VREFA
BAME
MEN
N

19

18

17

16

Res.

Res.

Res.

Res.

rw

rw

rw

rw

rw

rw

rw

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

I2C3A
MEN

LPUAR
T1AME
N

SPI3A
MEN

Res.

Res.

Res.

Res.

Res.

rw

rw

rw

COMP OPAMP LPTIM4 LPTIM3 LPTIM1
AMEN AMEN AMEN AMEN AMEN
rw

26

LPGPI
ADF1A LPDMA DAC1A
ADC4A
O1AME
MEN 1AMEN MEN
MEN
N

rw

rw

rw

Bit 31 SRAM4AMEN: SRAM4 autonomous mode enable in Stop 0/1/2 mode
This bit is set and cleared by software.
0: SRAM4 autonomous mode disabled during Stop 0/1/2 mode
1: SRAM4 autonomous mode enabled during Stop 0/1/2 mode
Bit 30 Reserved, must be kept at reset value.
Bit 29 ADF1AMEN: ADF1 autonomous mode enable in Stop 0/1/2 mode
This bit is set and cleared by software.
0: ADF1 autonomous mode disabled during Stop 0/1/2 mode
1: ADF1 autonomous mode enabled during Stop 0/1/2 mode
Note: This bit must be set to allow the peripheral to wake up from Stop modes.
Bit 28 LPDMA1AMEN: LPDMA1 autonomous mode enable in Stop 0/1/2 mode
This bit is set and cleared by software.
0: LPDMA1 autonomous mode disabled during Stop 0/1/2 mode
1: LPDMA1 autonomous mode enabled during Stop 0/1/2 mode
Note: This bit must be set to allow the peripheral to wake up from Stop modes.
Bit 27 DAC1AMEN: DAC1 autonomous mode enable in Stop 0/1/2 mode
This bit is set and cleared by software.
0: DAC1 autonomous mode disabled during Stop 0/1/2 mode
1: DAC1 autonomous mode enabled during Stop 0/1/2 mode
Note: This bit must be set to allow the peripheral to wake up from Stop modes.
Bit 26 LPGPIO1AMEN: LPGPIO1 autonomous mode enable in Stop 0/1/2 mode
This bit is set and cleared by software.
0: LPGPIO1 autonomous mode disabled during Stop 0/1/2 mode
1: LPGPIO1 autonomous mode enabled during Stop 0/1/2 mode
Bit 25 ADC4AMEN: ADC4 autonomous mode enable in Stop 0/1/2 mode
This bit is set and cleared by software.
0: ADC4 autonomous mode disabled during Stop 0/1/2 mode
1: ADC4 autonomous mode enabled during Stop 0/1/2 mode
Note: This bit must be set to allow the peripheral to wake up from Stop modes.
Bits 24:22 Reserved, must be kept at reset value.

RM0456 Rev 6

<!-- pagebreak -->

609

Reset and clock control (RCC)

RM0456

Bit 21 RTCAPBAMEN: RTC and TAMP autonomous mode enable in Stop 0/1/2 mode
This bit is set and cleared by software.
0: RTC and TAMP autonomous mode disabled during Stop 0/1/2mode
1: RTC and TAMP autonomous mode enabled during Stop 0/1/2 mode
Note: This bit must be set to allow the peripheral to wake up from Stop modes.
Bit 20 VREFAMEN: VREFBUF autonomous mode enable in Stop 0/1/2 mode
This bit is set and cleared by software.
0: VREFBUF autonomous mode disabled during Stop 0/1/2 mode
1: VREFBUF autonomous mode enabled during Stop 0/1/2 mode
Bits 19:16 Reserved, must be kept at reset value.
Bit 15 COMPAMEN: COMP autonomous mode enable in Stop 0/1/2 mode
This bit is set and cleared by software.
0: COMP autonomous mode disabled during Stop 0/1/2 mode
1: COMP autonomous mode enabled during Stop 0/1/2 mode
Bit 14 OPAMPAMEN: OPAMP autonomous mode enable in Stop 0/1/2 mode
This bit is set and cleared by software.
0: OPAMP autonomous mode disabled during Stop 0/1/2 mode
1: OPAMP autonomous mode enabled during Stop 0/1/2 mode
Bit 13 LPTIM4AMEN: LPTIM4 autonomous mode enable in Stop 0/1/2 mode
This bit is set and cleared by software.
0: LPTIM4 autonomous mode disabled during Stop 0/1/2 mode
1: LPTIM4 autonomous mode enabled during Stop 0/1/2 mode
Note: This bit must be set to allow the peripheral to wake up from Stop modes.
Bit 12 LPTIM3AMEN: LPTIM3 autonomous mode enable in Stop 0/1/2 mode
This bit is set and cleared by software.
0: LPTIM3 autonomous mode disabled during Stop 0/1/2 mode
1: LPTIM3 autonomous mode enabled during Stop 0/1/2 mode
Note: This bit must be set to allow the peripheral to wake up from Stop modes.
Bit 11 LPTIM1AMEN: LPTIM1 autonomous mode enable in Stop 0/1/2 mode
This bit is set and cleared by software.
0: LPTIM1 autonomous mode disabled during Stop 0/1/2 mode
1: LPTIM1 autonomous mode enabled during Stop 0/1/2 mode
Note: This bit must be set to allow the peripheral to wake up from Stop modes.
Bits 10:8 Reserved, must be kept at reset value.
Bit 7 I2C3AMEN: I2C3 autonomous mode enable in Stop 0/1/2 mode
This bit is set and cleared by software.
0: I2C3 autonomous mode disabled during Stop 0/1/2 mode
1: I2C3 autonomous mode enabled during Stop 0/1/2 mode
Note: This bit must be set to allow the peripheral to wake up from Stop modes.
Bit 6 LPUART1AMEN: LPUART1 autonomous mode enable in Stop 0/1/2 mode
This bit is set and cleared by software.
0: LPUART1 autonomous mode disabled during Stop 0/1/2 mode
1: LPUART1 autonomous mode enabled during Stop 0/1/2 mode
Note: This bit must be set to allow the peripheral to wake up from Stop modes.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Reset and clock control (RCC)

Bit 5 SPI3AMEN: SPI3 autonomous mode enable in Stop 0,1, 2 mode
This bit is set and cleared by software.
0: SPI3 autonomous mode disabled during Stop 0/1/2 mode
1: SPI3 autonomous mode enabled during Stop 0/1/2 mode
Note: This bit must be set to allow the peripheral to wake up from Stop modes.
Bits 4:0 Reserved, must be kept at reset value.

11.8.46

RCC peripherals independent clock configuration register 1
(RCC_CCIPR1)
Address offset: 0x0E0
Reset value: 0x0000 0000
Access: no wait states; word, half-word, and byte access

31

30

29

TIMICSEL[2:0]

28
Res.

rw

rw

rw

15

14

13

12

27

26

ICLKSEL[1:0]

23

22

21

20

SPI1SEL[1:0]

19

18

LPTIM2SEL[1:0]

17

16

SPI2SEL[1:0]

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

rw

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

I2C2SEL[1:0]

I2C1SEL[1:0]

rw

rw

rw

rw

24

rw

I2C4SEL[1:0]
rw

25

FDCAN1SEL[1:0 SYSTICKSEL[1:
]
0]

rw

UART5SEL[1:0]
rw

USART3SEL[1:0 USART2SEL[1:0 USART1SEL[1:0
UART4SEL[1:0]
]
]
]

rw

rw

rw

rw

rw

rw

rw

rw

rw

Bits 31:29 TIMICSEL[2:0]: Clock sources for TIM16,TIM17, and LPTIM2 internal input capture
When TIMICSEL2 is set, the TIM16, TIM17, and LPTIM2 internal input capture can be
connected either to HSI/256, MSI/4, or MSI/1024. Depending on TIMICSEL[1:0] value, MSI
is either MSIK or MSIS.
When TIMICSEL2 is cleared, the HSI, MSIK, and MSIS clock sources cannot be selected
as TIM16, TIM17, or LPTIM2 internal input capture.
0xx: HSI, MSIK and MSIS dividers disabled
100: HSI/256, MSIS/1024 and MSIS/4 generated and can be selected by TIM16, TIM17, and
LPTIM2 as internal input capture
101: HSI/256, MSIS/1024 and MSIK/4 generated and can be selected by TIM16, TIM17, and
LPTIM2 as internal input capture
110: HSI/256, MSIK/1024 and MSIS/4 generated and can be selected by TIM16, TIM17 ,and
LPTIM2 as internal input capture
111: HSI/256, MSIK/1024 and MSIK/4 generated and can be selected by TIM16, TIM17, and
LPTIM2 as internal input capture
Note: The clock division must be disabled (TIMICSEL configured to 0xx) before selecting or
changing a clock sources division.
Bit 28 Reserved, must be kept at reset value.
Bits 27:26 ICLKSEL[1:0]: Intermediate clock source selection
These bits are used to select the clock source for the OTG_FS, the USB, and the SDMMC.
00: HSI48 clock selected
01: PLL2 “Q” (pll2_q_ck) selected
10: PLL1 “Q” (pll1_q_ck) selected
11: MSIK clock selected

RM0456 Rev 6

<!-- pagebreak -->

609

Reset and clock control (RCC)

RM0456

Bits 25:24 FDCAN1SEL[1:0]: FDCAN1 kernel clock source selection
These bits are used to select the FDCAN1 kernel clock source.
00: HSE clock selected
01: PLL1“Q” (pll1_q_ck) selected
10: PLL2 “P” (pll2_p_ck) selected
11: reserved
Bits 23:22 SYSTICKSEL[1:0]: SysTick clock source selection
These bits are used to select the SysTick clock source.
00: HCLK/8 selected
01: LSI selected
10: LSE selected
11: reserved
Note: When LSE or LSI is selected, the AHB frequency must be at least four times higher than
the LSI or LSE frequency. In addition, a jitter up to one HCLK cycle is introduced, due to
the LSE or LSI sampling with HCLK in the SysTick circuitry.
Bits 21:20 SPI1SEL[1:0]: SPI1 kernel clock source selection
These bits are used to select the SPI1 kernel clock source.
00: PCLK2 selected
01: SYSCLK selected
10: HSI16 selected
11: MSIK selected
Note: The SPI1 is functional in Stop 0 and Stop 1 mode only when the kernel clock is HSI16 or
MSIK.
Bits 19:18 LPTIM2SEL[1:0]: Low-power timer 2 kernel clock source selection
These bits are used to select the LPTIM2 kernel clock source.
00: PCLK1 selected
01: LSI selected
10: HSI16 selected
11: LSE selected
Note: The LPTIM2 is functional in Stop 0 and Stop 1 mode only when the kernel clock is LSI,
LSE or HSI16 if HSIKERON = 1.
Bits 17:16 SPI2SEL[1:0]: SPI2 kernel clock source selection
These bits are used to select the SPI2 kernel clock source.
00: PCLK1 selected
01: SYSCLK selected
10: HSI16 selected
11: MSIK selected
Note: The SPI2 is functional in Stop 0 and Stop 1 mode only when the kernel clock is HSI16 or
MSIK.
Bits 15:14 I2C4SEL[1:0]: I2C4 kernel clock source selection
These bits are used to select the I2C4 kernel clock source.
00: PCLK1 selected
01: SYSCLK selected
10: HSI16 selected
11: MSIK selected
Note: The I2C4 is functional in Stop 0 and Stop 1 modes only when the kernel clock is
HSI16 or MSIK.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Reset and clock control (RCC)

Bits 13:12 I2C2SEL[1:0]: I2C2 kernel clock source selection
These bits are used to select the I2C2 kernel clock source.
00: PCLK1 selected
01: SYSCLK selected
10: HSI16 selected
11: MSIK selected
Note: The I2C2 is functional in Stop 0 and Stop 1 mode sonly when the kernel clock is
HSI16 or MSIK.
Bits 11:10 I2C1SEL[1:0]: I2C1 kernel clock source selection
These bits are used to select the I2C1 kernel clock source.
00: PCLK1 selected
01: SYSCLK selected
10: HSI16 selected
11: MSIK selected
Note: The I2C1 is functional in Stop 0 and Stop 1 mode sonly when the kernel clock is
HSI16 or MSIK.
Bits 9:8 UART5SEL[1:0]: UART5 kernel clock source selection
These bits are used to select the UART5 kernel clock source.
00: PCLK1 selected
01: SYSCLK selected
10: HSI16 selected
11: LSE selected
Note: The UART5 is functional in Stop 0 and Stop 1 modes only when the kernel clock is
HSI16 or LSE.
Bits 7:6 UART4SEL[1:0]: UART4 kernel clock source selection
These bits are used to select the UART4 kernel clock source.
00: PCLK1 selected
01: SYSCLK selected
10: HSI16 selected
11: LSE selected
Note: The UART4 is functional in Stop 0 and Stop 1 modes only when the kernel clock is
HSI16 or LSE.
Bits 5:4 USART3SEL[1:0]: USART3 kernel clock source selection
These bits are used to select the USART3 kernel clock source.
00: PCLK1 selected
01: SYSCLK selected
10: HSI16 selected
11: LSE selected
Note: The USART3 is functional in Stop 0 and Stop 1 modes only when the kernel clock is
HSI16 or LSE.

RM0456 Rev 6

<!-- pagebreak -->

609

Reset and clock control (RCC)

RM0456

Bits 3:2 USART2SEL[1:0]: USART2 kernel clock source selection
These bits are used to select the USART2 kernel clock source.
00: PCLK1 selected
01: SYSCLK selected
10: HSI16 selected
11: LSE selected
Note: The USART2 is functional in Stop 0 and Stop 1 modes only when the kernel clock is
HSI16 or LSE.
This bitfield is only available on some devices in the STM32U5 Series. Refer to the
device datasheet for availability of its associated peripheral. If not present, consider this
bitfield as reserved and keep it at reset value.
Bits 1:0 USART1SEL[1:0]: USART1 kernel clock source selection
These bits are used to select the USART1 kernel clock source.
00: PCLK2 selected
01: SYSCLK selected
10: HSI16 selected
11: LSE selected
Note: The USART1 is functional in Stop 0 and Stop 1 modes only when the kernel clock is
HSI16 or LSE.

11.8.47

RCC peripherals independent clock configuration register 2
(RCC_CCIPR2)
Address offset: 0x0E4
Reset value: 0x0000 0000
Access: no wait state; word, half-word, and byte access

31

30

OTGHSSEL[1:0]
rw

rw

15

14

DSISE
L

SDMM
CSEL

rw

rw

29

28

27

Res.

Res.

I2C6SEL[1:0]

I2C5SEL[1:0]

rw

rw

rw

rw

rw

rw

rw

rw

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

Res.

Res.

RNGSEL[1:0]
rw

rw

26

SAESS
EL
rw

25

24

23

HSPI1SEL[1:0]

SAI2SEL[2:0]
rw

rw

22

21

OCTOSPISEL[1:
0]

SAI1SEL[2:0]
rw

rw

rw

20

rw

19
Res.

18

17

16

LTDCS USART6SEL[1:0
EL
]
rw

rw

rw

2

1

0

MDF1SEL[2:0]
rw

rw

rw

Bits 31:30 OTGHSSEL[1:0]: OTG_HS PHY kernel clock source selection
These bits are used to select the OTG_HS PHY kernel clock source.
00: HSE selected
01: PLL1 “P” (pll1_p_ck) selected. If selecting this option, then only HSE input should be
selected in PLL1SRC.
10: HSE/2 selected
11: PLL1 “P” divided by 2 (pll1_p_ck/2) selected. If selecting this option, then only HSE input
should be selected in PLL1SRC.
Note: This bitfield is only available on some devices in the STM32U5 Series. Refer to the
device datasheet for availability of its associated peripheral. If not present, consider this
bitfield as reserved and keep it at reset value.
Bits 29:28 Reserved, must be kept at reset value.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Reset and clock control (RCC)

Bits 27:26 I2C6SEL[1:0]: I2C6 kernel clock source selection
These bits are used to select the I2C6 kernel clock source.
00: PCLK1 selected
01: SYSCLK selected
10: HSI16 selected
11: MSIK selected
Note: The I2C6 is functional in Stop 0 and Stop 1 modes only when the kernel clock is
HSI16 or MSIK.
This bitfield is only available on some devices in the STM32U5 Series. Refer to the
device datasheet for availability of its associated peripheral. If not present, consider this
bitfield as reserved and keep it at reset value.
Bits 25:24 I2C5SEL[1:0]: I2C5 kernel clock source selection
These bits are used to select the I2C5 kernel clock source.
00: PCLK1 selected
01: SYSCLK selected
10: HSI16 selected
11: MSIK selected
Note: The I2C5 is functional in Stop 0 and Stop 1 modes only when the kernel clock is
HSI16 or MSIK.
This bitfield is only available on some devices in the STM32U5 Series. Refer to the
device datasheet for availability of its associated peripheral. If not present, consider this
bitfield as reserved and keep it at reset value.
Bits 23:22 HSPI1SEL[1:0]: HSPI1 kernel clock source selection
These bits are used to select the HSPI1 kernel clock source.
00: SYSCLK selected
01: PLL1 “Q” (pll1_q_ck) selected, can be up to 200 MHz
10: PLL2 “Q” (pll2_q_ck) selected, can be up to 200 MHz
11: PLL3 “R” (pll3_r_ck) selected, can be up to 200 MHz
Note: This bitfield is only available on some devices in the STM32U5 Series. Refer to the
device datasheet for availability of its associated peripheral. If not present, consider this
bitfield as reserved and keep it at reset value.
Bits 21:20 OCTOSPISEL[1:0]: OCTOSPI1 and OCTOSPI2 kernel clock source selection
These bits are used to select the OCTOSPI1 and OCTOSPI2 kernel clock source.
00: SYSCLK selected
01: MSIK selected
10: PLL1 “Q” (pll1_q_ck) selected, can be up to 200 MHz
11: PLL2 “Q” (pll2_q_ck) selected, can be up to 200 MHz
Bit 19 Reserved, must be kept at reset value.
Bit 18 LTDCSEL: LTDC kernel clock source selection
This bit is used to select the LTDC kernel clock source.
0: PLL3 “R” (pll3_r_ck) selected
1: PLL2 “R” (pll2_r_ck) selected
Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and keep it at reset value.

RM0456 Rev 6

<!-- pagebreak -->

609

Reset and clock control (RCC)

RM0456

Bits 17:16 USART6SEL[1:0]: USART6 kernel clock source selection
These bits are used to select the USART6 kernel clock source.
00: PCLK1 selected
01: SYSCLK selected
10: HSI16 selected
11: LSE selected
Note: The USART6 is functional in Stop 0 and Stop 1 modes only when the kernel clock is
HSI16 or LSE.
This bitfield is only available on some devices in the STM32U5 Series. Refer to the
device datasheet for availability of its associated peripheral. If not present, consider this
bitfield as reserved and keep it at reset value.
Bit 15 DSISEL: DSI kernel clock source selection
This bit is used to select the DSI kernel clock source.
0: PLL3 “P” (pll3_p_ck) selected
1: DSI PHY PLL output selected
Note: This bit is only available on some devices in the STM32U5 Series.
Refer to the device datasheet for availability of its associated peripheral.
If not present, consider this bit as reserved and keep it at reset value.
Bit 14 SDMMCSEL: SDMMC1 and SDMMC2 kernel clock source selection
This bit is used to select the SDMMC kernel clock source. It is recommended to change it
only after reset and before enabling the SDMMC.
0: ICLK clock selected
1: PLL1 “P” (pll1_p_ck) selected, in case higher than 48 MHz is needed (for SDR50 mode)
Bits 13:12 RNGSEL[1:0]: RNG kernel clock source selection
These bits are used to select the RNG kernel clock source.
00: HSI48 selected
01: HSI48 / 2 selected, can be used in range 4
10: HSI16 selected
11: reserved
Bit 11 SAESSEL: SAES kernel clock source selection
This bit is used to select the SAES kernel clock source.
0: SHSI selected
1: SHSI / 2 selected, can be used in range 4
Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and keep it at reset value.
Bits 10:8 SAI2SEL[2:0]: SAI2 kernel clock source selection
These bits are used to select the SAI2 kernel clock source.
000: PLL2 “P” (pll2_p_ck) selected
001: PLL3 “P” (pll3_p_ck) selected
010: PLL1 “P” (pll1_p_ck) selected
011: input pin AUDIOCLK selected
100: HSI16 clock selected
others: reserved
Note: If the selected clock is the external clock and this clock is stopped, a switch to another
clock is impossible.
This bitfield is only available on some devices in the STM32U5 Series. Refer to the
device datasheet for availability of its associated peripheral. If not present, consider this
bitfield as reserved and keep it at reset value.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Reset and clock control (RCC)

Bits 7:5 SAI1SEL[2:0]: SAI1 kernel clock source selection
These bits are used to select the SAI1 kernel clock source.
000: PLL2 “P” (pll2_p_ck) selected
001: PLL3 “P” (pll3_p_ck) selected
010: PLL1 “P” (pll1_p_ck) selected
011: input pin AUDIOCLK selected
100: HSI16 clock selected
others: reserved
Note: If the selected clock is the external clock and this clock is stopped, a switch to another
clock is impossible.
Bits 4:3 Reserved, must be kept at reset value.
Bits 2:0 MDF1SEL[2:0]: MDF1 kernel clock source selection
These bits are used to select the MDF1 kernel clock source.
000: HCLK selected
001: PLL1 “P” (pll1_p_ck) selected
010: PLL3 “Q” (pll3_q_ck) selected
011: input pin AUDIOCLK selected
100: MSIK clock selected
others: reserved

11.8.48

RCC peripherals independent clock configuration register 3
(RCC_CCIPR3)
Address offset: 0x0E8
Reset value: 0x0000 0000
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

20

19

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
DAC1S
EL
rw

14

13

12

ADCDACSEL[2:0]
rw

rw

rw

11

10

9

8

LPTIM34SEL[1:0
LPTIM1SEL[1:0]
]
rw

rw

rw

7

6

I2C3SEL[1:0]

rw

rw

rw

5
Res.

4

3

18

17

16

ADF1SEL[2:0]
rw

rw

rw

2

1

0

SPI3SEL[1:0]

LPUART1SEL[2:0]

rw

rw

rw

rw

rw

Bits 31:19 Reserved, must be kept at reset value.
Bits 18:16 ADF1SEL[2:0]: ADF1 kernel clock source selection
These bits are used to select the ADF1 kernel clock source.
000: HCLK selected
001: PLL1 “P” (pll1_p_ck) selected
010: PLL3 “Q” (pll3_q_ck) selected
011: input pin AUDIOCLK selected
100: MSIK clock selected
others: reserved
Note: The ADF1 is functional in Stop 0, Stop 1, and Stop 2 modes only when the kernel clock
is AUDIOCLK or MSIK.

RM0456 Rev 6

<!-- pagebreak -->

609

Reset and clock control (RCC)

RM0456

Bit 15 DAC1SEL: DAC1 sample-and-hold clock source selection
This bit is used to select the DAC1 sample-and-hold clock source.
0: LSE selected
1: LSI selected
Bits 14:12 ADCDACSEL[2:0]: ADC1, ADC2, ADC4 and DAC1 kernel clock source selection
These bits are used to select the ADC1, ADC2, ADC4, and DAC1 kernel clock source.
000: HCLK clock selected
001: SYSCLK selected
010: PLL2 “R” (pll2_r_ck) selected
011: HSE clock selected
100: HSI16 clock selected
101: MSIK clock selected
others: reserved
Note: The ADC1, ADC2, ADC4, and DAC1 are functional in Stop 0, Stop 1, and Stop 2 modes
only when the kernel clock is HSI16 or MSIK (only ADC4 and DAC1 are functional
in Stop 2 mode).
Bits 11:10 LPTIM1SEL[1:0]: LPTIM1 kernel clock source selection
These bits are used to select the LPTIM1 kernel clock source.
00: MSIK clock selected
01: LSI selected
10: HSI16 selected
11: LSE selected
Note: The LPTIM1 is functional in Stop 0, Stop 1, and Stop 2 modes only when the kernel
clock is LSI, LSE, HSI16 with HSIKERON = 1, or MSIK with MSIKERON = 1.
Bits 9:8 LPTIM34SEL[1:0]: LPTIM3 and LPTIM4 kernel clock source selection
These bits are used to select the LPTIM3 and LPTIM4 kernel clock source.
00: MSIK clock selected
01: LSI selected
10: HSI selected
11: LSE selected
Note: The LPTIM3 and LPTIM4 are functional in Stop 0, Stop 1, and Stop 2 modes only when
the kernel clock is LSI, LSE, HSI16 with HSIKERON = 1, or MSIK with MSIKERON = 1.
Bits 7:6 I2C3SEL[1:0]: I2C3 kernel clock source selection
These bits are used to select the I2C3 kernel clock source.
00: PCLK3 selected
01: SYSCLK selected
10: HSI16 selected
11: MSIK selected
Note: The I2C3 is functional in Stop 0, Stop 1, and Stop 2 modes only when the kernel clock is
HSI16 or MSIK.
Bit 5 Reserved, must be kept at reset value.
Bits 4:3 SPI3SEL[1:0]: SPI3 kernel clock source selection
These bits are used to select the SPI3 kernel clock source.
00: PCLK3 selected
01: SYSCLK selected
10: HSI16 selected
11: MSIK selected
Note: The SPI3 is functional in Stop 0, Stop 1, and Stop 2 modes only when the kernel clock is
HSI16 or MSIK.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Reset and clock control (RCC)

Bits 2:0 LPUART1SEL[2:0]: LPUART1 kernel clock source selection
These bits are used to select the LPUART1 kernel clock source.
000: PCLK3 selected
001: SYSCLK selected
010: HSI16 selected
011: LSE selected
100: MSIK selected
others: reserved
Note: The LPUART1 is functional in Stop 0, Stop 1, and Stop 2 modes only when the kernel
clock is HSI16, LSE, or MSIK.

11.8.49

RCC backup domain control register (RCC_BDCR)
Address offset: 0x00F0
Backup domain reset value: 0x0000 0000 (for STM32U575/585)
Backup domain reset value: 0x0000 X000 (for the other STM32U5 Series devices)
Reset by backup domain reset, except LSCOSEL, LSCOEN, and BDRST that are reset only
by backup domain power-on reset, and LSESYSEN and LSESYSRDY that are reset by
power-on reset.
Access: 0 ≤ wait state ≤ 3; word, half-word, and byte access
Wait states are inserted in case of successive accesses to this register.

Note:

31

These register bits are outside of the core domain. After reset, these bits are then
write-protected, and DBP must be set in PWR_BDCR1 before these can be modified
(see Section 10: Power control (PWR) for further information). Any internal or external reset
does not have any effect on these bits.
30

29

28

27
LSIRD
Y

26

25

24

Res.

Res.

Res.

LSIPR
EDIV
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

RTCEN
rw

Res.

Res.

LSEGF LSESY
ON
SRDY
rw

r

LSCOS LSCOE
LSION
EL
N

Res.

RTCSEL[1:0]
rw

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

BDRST

7

6

5

4

3

2

1

rw

LSESY LSECS LSECS
SEN
SD
SON

rw

rw

r

rw

LSEDRV[1:0]
rw

rw

0

LSEBY LSERD
LSEON
P
Y
rw

r

rw

Bits 31:29 Reserved, must be kept at reset value.
Bit 28 LSIPREDIV: Low-speed clock divider configuration
This bit is set and cleared by software to enable the LSI division. It can be written only when
the LSI is disabled (LSION = 0 and LSIRDY = 0). If the LSI was previously enabled, it is
necessary to wait for at least 60 μs after clearing LSION bit (synchronization time for LSI to
be really disabled), before writing LSIPREDIV. The LSIPREDIV cannot be changed if the LSI
is used by the IWDG or by the RTC.
0: LSI not divided
1: LSI divided by 128

RM0456 Rev 6

<!-- pagebreak -->

609

Reset and clock control (RCC)

RM0456

Bit 27 LSIRDY: LSI oscillator ready
This bit is set and cleared by hardware to indicate when the LSI oscillator is stable.
After LSION is cleared, LSIRDY goes low after three internal low-speed oscillator clock
cycles. This bit is set when the LSI is used by IWDG or RTC, even if LSION = 0.
0: LSI oscillator not ready
1: LSI oscillator ready
Bit 26 LSION: LSI oscillator enable
This bit is set and cleared by software. The LSI oscillator is disabled 60 µs maximum after
the LSION bit is cleared.
0: LSI oscillator OFF
1: LSI oscillator ON
Bit 25 LSCOSEL: Low-speed clock output selection
This bit is set and cleared by software.
0: LSI clock selected
1: LSE clock selected
Bit 24 LSCOEN: Low-speed clock output (LSCO) enable
This bit is set and cleared by software.
0: LSCO disabled
1: LSCO enabled
Bits 23:17 Reserved, must be kept at reset value.
Bit 16 BDRST: Backup domain software reset
This bit is set and cleared by software.
0: Reset not activated
1: Reset the entire backup domain.
Bit 15 RTCEN: RTC and TAMP clock enable
This bit is set and cleared by software.
0: RTC and TAMP clock disabled
1: RTC and TAMP clock enabled
Bits 14:13 Reserved, must be kept at reset value.
Bit 12 LSEGFON: LSE clock glitch filter enable
This bit is set and cleared by hardware to enable the LSE glitch filter. It can be written only
when the LSE is disabled (LSEON = 0 and LSERDY = 0).
0: LSE glitch filter disabled
1: LSE glitch filter enabled
Bit 11 LSESYSRDY: LSE system clock (LSESYS) ready
This bit is set and cleared by hardware to indicate when the LSE system clock is
stable.When LSESYSEN is set, this LSESYSRDY flag is set after two LSE clock cycles.
The LSE clock must be already enabled and stable (LSEON and LSERDY are set).
When the LSEON bit is cleared, LSERDY goes low after six external low-speed oscillator
clock cycles.
0: LSESYS clock not ready
1: LSESYS clock ready
Bit 10 Reserved, must be kept at reset value.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Reset and clock control (RCC)

Bits 9:8 RTCSEL[1:0]: RTC and TAMP clock source selection
This bit is set by software to select the clock source for the RTC and TAMP. Once the RTC
and TAMP clock source has been selected, it cannot be changed anymore unless
the backup domain is reset, or unless a failure is detected on LSE (LSECSSD is set).
BDRST bit can be used to reset them.
00: No clock selected
01: LSE oscillator clock selected
10: LSI oscillator clock selected
11: HSE oscillator clock divided by 32 selected
Bit 7 LSESYSEN: LSE system clock (LSESYS) enable
This bit is set by software to enable always the LSE system clock generated by RCC, which
can be used by any peripheral when its source clock is the LSE, or at system level if one of
LSCOSEL, MCO, or MSI PLL mode is needed.
0: LSE can be used only for RTC, TAMP, and CSS on LSE.
1: LSE can be used by any other peripheral or function.
Bit 6 LSECSSD: CSS on LSE failure detection
This bit is set by hardware to indicate when a failure is detected by the CCS on the external
32 kHz oscillator (LSE).
0: No failure detected on LSE
1: Failure detected on LSE
Bit 5 LSECSSON: CSS on LSE enable
This bit is set by software to enable the CSS on LSE. It must be enabled after the LSE
oscillator is enabled (LSEON bit enabled) and ready (LSERDY flag set by hardware), and
after the RTCSEL bit is selected.
Once enabled, this bit cannot be disabled, except after a LSE failure detection
(LSECSSD = 1). In that case, the software must disable this LSECSSON bit.
0: CSS on LSE OFF
1: CSS on LSE ON
Bits 4:3 LSEDRV[1:0]: LSE oscillator drive capability
This bitfield is set by software to modulate the drive capability of the LSE oscillator. It can be
written only when the external 32 kHz oscillator is disabled (LSEON = 0 and LSERDY = 0).
00: ‘Xtal mode’ lower driving capability
01: ‘Xtal mode’ medium-low driving capability
10: ‘Xtal mode’ medium-high driving capability
11: ‘Xtal mode’ higher driving capability
Note: The oscillator is in ‘Xtal mode’ when it is not in bypass mode.
Bit 2 LSEBYP: LSE oscillator bypass
This bit is set and cleared by software to bypass oscillator in debug mode. It can be written
only when the external 32 kHz oscillator is disabled (LSEON = 0 and LSERDY = 0).
0: LSE oscillator not bypassed
1: LSE oscillator bypassed
Bit 1 LSERDY: LSE oscillator ready
This bit is set and cleared by hardware to indicate when the external 32 kHz oscillator is
stable. After LSEON is cleared, this LSERDY bit goes low after six external low-speed
oscillator clock cycles.
0: LSE oscillator not ready
1: LSE oscillator ready

RM0456 Rev 6

<!-- pagebreak -->

609

Reset and clock control (RCC)

RM0456

Bit 0 LSEON: LSE oscillator enable
This bit is set and cleared by software.
0: LSE oscillator off
1: LSE oscillator on

11.8.50

RCC control/status register (RCC_CSR)
Address offset: 0x0F4
Reset value: 0x0C00 4400
Reset by system reset, except reset flags by power reset only.
Access: 0 ≤ wait state ≤ 3; word, half-word, and byte access
Wait states are inserted in case of successive accesses to this register.

31

30

29

28

LPWR WWDG IWDGR SFTRS
RSTF
RSTF
STF
TF

27
BORR
STF

26

25

PINRS OBLRS
TF
TF

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

MSISSRANGE[3:0]
rw

rw

rw

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

RMVF

Res.

Res.

Res.

Res.

Res.

Res.

Res.

rw
8

MSIKSRANGE[3:0]
rw

rw

rw

rw

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

rw

Bit 31 LPWRRSTF: Low-power reset flag
This bit is set by hardware when a reset occurs due to a Stop, Standby, or Shutdown mode
entry, whereas the corresponding NRST_STOP, NRST_STBY, or NRST_SHDW option bit is
cleared. This bit is cleared by writing to the RMVF bit.
0: No illegal low-power mode reset occurred
1: Illegal low-power mode reset occurred
Bit 30 WWDGRSTF: Window watchdog reset flag
This bit is set by hardware when a window watchdog reset occurs. It is cleared by writing
to the RMVF bit.
0: No window watchdog reset occurred
1: Window watchdog reset occurred
Bit 29 IWDGRSTF: Independent watchdog reset flag
This bit is set by hardware when an independent watchdog reset domain occurs. It is cleared
by writing to the RMVF bit.
0: No independent watchdog reset occurred
1: Independent watchdog reset occurred
Bit 28 SFTRSTF: Software reset flag
This bit is set by hardware when a software reset occurs. It is cleared by writing to RMVF.
0: No software reset occurred
1: Software reset occurred
Bit 27 BORRSTF: Brownout reset or an exit from Shutdown mode reset flag
This bit is set by hardware when a brownout reset or an exit from Shutdown mode reset
occurs. It is cleared by writing to the RMVF bit.
0: No BOR/exit from Shutdown mode reset occurred
1: BOR/exit from Shutdown mode reset occurred

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Reset and clock control (RCC)

Bit 26 PINRSTF: NRST pin reset flag
This bit is set by hardware when a reset from the NRST pin occurs. It is cleared by writing
to the RMVF bit.
0: No reset from NRST pin occurred
1: Reset from NRST pin occurred
Bit 25 OBLRSTF: Option-byte loader reset flag
This bit is set by hardware when a reset from the option-byte loading occurs. It is cleared
by writing to the RMVF bit.
0: No reset from option-byte loading occurred
1: Reset from option-byte loading occurred
Bit 24 Reserved, must be kept at reset value.
Bit 23 RMVF: Remove reset flag
This bit is set by software to clear the reset flags.
0: No effect
1: Clear the reset flags.
Bits 22:16 Reserved, must be kept at reset value.
Bits 15:12 MSISSRANGE[3:0]:MSIS range after Standby mode
This bitfield is set by software to chose the MSIS frequency at startup. It is used after exiting
Standby mode until MSIRGSEL is set. After a NRST pin or a power-on reset or when exiting
Shutdown mode, the range is always 4 MHz. MSISSRANGE can be written only when
MSIRGSEL = 1.
0100: range 4 around 4M Hz (reset value)
0101: range 5 around 2 MHz
0110: range 6 around 1.33 MHz
0111: range 7 around 1 MHz
1000: range 8 around 3.072 MHz
others: reserved
Note: Changing this bitfield does not change the current MSIS frequency.
Bits 11:8 MSIKSRANGE[3:0]:MSIK range after Standby mode
This bit is set by software to chose the MSIK frequency at startup. It is used after exiting
Standby mode until MSIRGSEL is set. After a NRST pin or a power-on reset or when exiting
Shutdown mode, the range is always 4 MHz. MSIKSRANGE can be written only when
MSIRGSEL = 1.
0100: range 4 around 4M Hz (reset value)
0101: range 5 around 2 MHz
0110: range 6 around 1.33 MHz
0111: range 7 around 1 MHz
1000: range 8 around 3.072 MHz
others: reserved
Note: Changing this bitfield does not change the current MSIK frequency.
Bits 7:0 Reserved, must be kept at reset value.

RM0456 Rev 6

<!-- pagebreak -->

609

Reset and clock control (RCC)

11.8.51

RM0456

RCC secure configuration register (RCC_SECCFGR)
Address offset: 0x110
Reset value: 0x0000 0000
Access: no wait state; word, half-word, and byte access
When the system is secure (TZEN = 1), this register can be written only by a secure
privileged access if SPRIV = 1, and by a secure privileged or unprivileged access
if SPRIV = 0. A nonsecure write access generates an illegal access event and data is not
written. This register can be read by secure or nonsecure, privilege or unprivileged access.
When the system is not secure (TZEN = 0), this register is read as 0, and the register write
is ignored.

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

PLL3S
EC

PLL2S
EC

rw

rw

RMVFS HSI48S ICLKS
EC
EC
EC
rw

rw

rw

PLL1S PRESC SYSCL LSESE
EC
SEC
KSEC
C
rw

rw

rw

Bits 31:13 Reserved, must be kept at reset value.
Bit 12 RMVFSEC: Remove reset flag security
This bit is set and reset by software.
0: nonsecure
1: secure
Bit 11 HSI48SEC: HSI48 clock configuration and status bit security
This bit is set and reset by software.
0: nonsecure
1: secure
Bit 10 ICLKSEC: Intermediate clock source selection security
This bit is set and reset by software.
0: nonsecure
1: secure
Bit 9 PLL3SEC: PLL3 clock configuration and status bit security
This bit is set and reset by software.
0: nonsecure
1: secure
Bit 8 PLL2SEC: PLL2 clock configuration and status bit security
Set and reset by software.
0: nonsecure
1: secure
Bit 7 PLL1SEC: PLL1 clock configuration and status bit security
This bit is set and reset by software.
0: nonsecure
1: secure

<!-- pagebreak -->

RM0456 Rev 6

rw

LSISE
C
rw

MSISE HSESE HSISE
C
C
C
rw

rw

rw

RM0456

Reset and clock control (RCC)

Bit 6 PRESCSEC: AHBx/APBx prescaler configuration bits security
This bit is set and reset by software.
0: nonsecure
1: secure
Bit 5 SYSCLKSEC: SYSCLK clock selection, STOPWUCK bit, clock output on MCO configuration
security
This bit is set and reset by software.
0: nonsecure
1: secure
Bit 4 LSESEC: LSE clock configuration and status bit security
This bit is set and reset by software.
0: nonsecure
1: secure
Bit 3 LSISEC: LSI clock configuration and status bit security
This bit is set and reset by software.
0: nonsecure
1: secure
Bit 2 MSISEC: MSI clock configuration and status bit security
This bit is set and reset by software.
0: nonsecure
1: secure
Bit 1 HSESEC: HSE clock configuration bits, status bit and HSE_CSS security
This bit is set and reset by software.
0: nonsecure
1: secure
Bit 0 HSISEC: HSI clock configuration and status bit security
This bit is set and reset by software.
0: nonsecure
1: secure

11.8.52

RCC privilege configuration register (RCC_PRIVCFGR)
Address offset: 0x114
Reset value: 0x0000 0000
Access: no wait state; word, half-word, and byte access
This register can be written only by a privileged access. It can be read by privileged or
unprivileged access.

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

NSPRI
V

SPRIV

rw

rw

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

Bits 31:2 Reserved, must be kept at reset value.

RM0456 Rev 6

<!-- pagebreak -->

