917

Nested vectored interrupt controller (NVIC)

22.3

RM0456

Interrupt and exception vectors
The grey rows in the table below describe the vectors without specific position.

Position

Priority

Table 187. STM32U5 series vector table(1)
Type of
priority

-

-

-

-

-4

Fixed

-

-2

-

Acronym

-

Description

Address

Reserved

0x0000 0000

Reset

Reset

0x0000 0004

Fixed

NMI

Non maskable interrupt. The RCC clock security
system (CSS) is linked to the NMI vector.

0x0000 0008

-3
or
-1

Fixed

Secure HardFault

Secure HardFault

0x0000 000C

-

-1

Fixed

Nonsecure HardFault

Nonsecure HardFault, all classes of fault

0x0000 000C

-

0

Settable MemManage

Memory management

0x0000 0010

-

1

Settable BusFault

Pre-fetch fault, memory access fault

0x0000 0014

-

2

Settable UsageFault

Undefined instruction or illegal state

0x0000 0018

-

3

Settable SecureFault

Secure fault

0x0000 001C

-

-

-

Reserved

0x0000 0020 0x0000 0028

-

4

-

SVC

System service call via SWI instruction

0x0000 002C

-

5

-

Debug Monitor

Debug monitor

0x0000 0030

-

-

-

Reserved

0x0000 0034

-

6

Settable PendSV

Pendable request for system service

0x0000 0038

-

7

Settable SysTick

System tick timer

0x0000 003C

0

8

Settable WWDG

Window watchdog interrupt

0x0000 0040

1

9

Settable PVD_PVM

Programmable voltage detector/peripheral voltage
monitor

0x0000 0044

2

10

Settable RTC

RTC global nonsecure interrupts

0x0000 0048

3

11

Settable RTC_S

RTC global secure interrupts

0x0000 004C

4

12

Settable TAMP

Tamper global interrupts

0x0000 0050

5

13

Settable RAMCFG

RAM configuration global interrupt

0x0000 0054

6

14

Settable FLASH

Flash memory nonsecure global interrupt

0x0000 0058

7

15

Settable FLASH_S

Flash memory secure global interrupt

0x0000 005C

8

16

Settable GTZC

GTZC1/GTZC2 global interrupt

0x0000 0060

9

17

Settable RCC

RCC nonsecure global interrupt

0x0000 0064

10

18

Settable RCC_S

RCC secure global interrupt

0x0000 0068

11

19

Settable EXTI0

EXTI Line0 interrupt

0x0000 006C

<!-- pagebreak -->

-

-

RM0456 Rev 6

RM0456

Nested vectored interrupt controller (NVIC)

Position

Priority

Table 187. STM32U5 series vector table(1) (continued)
Type of
priority

12

20

Settable EXTI1

EXTI Line1 interrupt

0x0000 0070

13

21

Settable EXTI2

EXTI Line2 interrupt

0x0000 0074

14

22

Settable EXTI3

EXTI Line3 interrupt

0x0000 0078

15

23

Settable EXTI4

EXTI Line4 interrupt

0x0000 007C

16

24

Settable EXTI5

EXTI Line5 interrupt

0x0000 0080

17

25

Settable EXTI6

EXTI Line6 interrupt

0x0000 0084

18

26

Settable EXTI7

EXTI Line7 interrupt

0x0000 0088

19

27

Settable EXTI8

EXTI Line8 interrupt

0x0000 008C

20

28

Settable EXTI9

EXTI Line9 interrupt

0x0000 0090

21

29

Settable EXTI10

EXTI Line10 interrupt

0x0000 0094

22

30

Settable EXTI11

EXTI Line11 interrupt

0x0000 0098

23

31

Settable EXTI12

EXTI Line12 interrupt

0x0000 009C

24

32

Settable EXTI13

EXTI Line13 interrupt

0x0000 00A0

25

33

Settable EXTI14

EXTI Line14 interrupt

0x0000 00A4

26

34

Settable EXTI15

EXTI Line15 interrupt

0x0000 00A8

27

35

Settable IWDG

Independent watchdog interrupt

0x0000 00AC

28

36

Settable SAES

Secure AES

0x0000 00B0

29

37

Settable GPDMA1_CH0

GPDMA1 channel 0 global interrupt

0x0000 00B4

30

38

Settable GPDMA1_CH1

GPDMA1 channel 1 global interrupt

0x0000 00B8

31

39

Settable GPDMA1_CH2

GPDMA1 channel 2 global interrupt

0x0000 00BC

32

40

Settable GPDMA1_CH3

GPDMA1 channel 3 global interrupt

0x0000 00C0

33

41

Settable GPDMA1_CH4

GPDMA1 channel 4 global interrupt

0x0000 00C4

34

42

Settable GPDMA1_CH5

GPDMA1 channel 5 global interrupt

0x0000 00C8

35

43

Settable GPDMA1_CH6

GPDMA1 channel 6 global interrupt

0x0000 00CC

36

44

Settable GPDMA1_CH7

GPDMA1 channel 7 global interrupt

0x0000 00D0

37

45

Settable ADC12

ADC12 (14 bits) global interrupt

0x0000 00D4

38

46

Settable DAC1

DAC1 global interrupt

0x0000 00D8

39

47

Settable FDCAN1_IT0

FDCAN1 interrupt 0

0x0000 00DC

40

48

Settable FDCAN1_IT1

FDCAN1 interrupt 1

0x0000 00E0

41

49

TIM1_BRK
Settable TIM1_TERR
TIM1_IERR

TIM1 break
TIM1 transition error
TIM1 index error

0x0000 00E4

42

50

Settable TIM1_UP

TIM1 update

0x0000 00E8

Acronym

Description

RM0456 Rev 6

Address

<!-- pagebreak -->

917

Nested vectored interrupt controller (NVIC)

RM0456

Position

Priority

Table 187. STM32U5 series vector table(1) (continued)
Type of
priority

43

51

TIM1_TRG_COM
Settable TIM1_DIR
TIM1_IDX

TIM1 trigger and commutation
TIM1 direction change interrupt
TIM1 index

0x0000 00EC

44

52

Settable TIM1_CC

TIM1 capture compare interrupt

0x0000 00F0

45

53

Settable TIM2

TIM2 global interrupt

0x0000 00F4

46

54

Settable TIM3

TIM3 global interrupt

0x0000 00F8

47

55

Settable TIM4

TIM4 global interrupt

0x0000 00FC

48

56

Settable TIM5

TIM5 global interrupt

0x0000 0100

49

57

Settable TIM6

TIM6 global interrupt

0x0000 0104

50

58

Settable TIM7

TIM7 global interrupt

0x0000 0108

51

59

TIM8_BRK
Settable TIM8_TERR
TIM8_IERR

TIM8 break interrupt
TIM8 transition error
TIM8 index error

0x0000 010C

52

60

Settable TIM8_UP

TIM8 update interrupt

0x0000 0110

53

61

TIM8_TRG_COM
Settable TIM8_DIR
TIM8_IDX

TIM8 trigger and commutation interrupt
TIM8 direction change interrupt
TIM8 Index

0x0000 0114

54

62

Settable TIM8_CC

TIM8 capture compare interrupt

0x0000 0118

55

63

Settable I2C1_EV

I2C1 event interrupt

0x0000 011C

56

64

Settable I2C1_ER

I2C1 error interrupt

0x0000 0120

57

65

Settable I2C2_EV

I2C2 event interrupt

0x0000 0124

58

66

Settable I2C2_ER

I2C2 error interrupt

0x0000 0128

59

67

Settable SPI1

SPI1 global interrupt

0x0000 012C

60

68

Settable SPI2

SPI2 global interrupt

0x0000 0130

61

69

Settable USART1

USART1 global interrupt

0x0000 0134

62

70

Settable USART2

USART2 global interrupt

0x0000 0138

63

71

Settable USART3

USART3 global interrupt

0x0000 013C

64

72

Settable UART4

UART4 global interrupt

0x0000 0140

65

73

Settable UART5

UART5 global interrupt

0x0000 0144

66

74

Settable LPUART1

LPUART1 global interrupt

0x0000 0148

67

75

Settable LPTIM1

LPTIM1 global interrupt

0x0000 014C

68

76

Settable LPTIM2

LPTIM2 global interrupt

0x0000 0150

69

77

Settable TIM15

TIM15 global interrupt

0x0000 0154

70

78

Settable TIM16

TIM16 global interrupt

0x0000 0158

71

79

Settable TIM17

TIM16 global interrupt

0x0000 015C

<!-- pagebreak -->

Acronym

Description

RM0456 Rev 6

Address

RM0456

Nested vectored interrupt controller (NVIC)

Position

Priority

Table 187. STM32U5 series vector table(1) (continued)
Type of
priority

72

80

Settable COMP

73

81

Settable

74

82

75

Acronym

Description

Address

COMP1/COMP2

0x0000 0160

USB/OTG_FS/OTG_HS global interrupt

0x0000 0164

Settable CRS

Clock recovery system global interrupt

0x0000 0168

83

Settable FMC

FSMC global interrupt

0x0000 016C

76

84

Settable OCTOSPI1

OCTOSPI1 global interrupt

0x0000 0170

77

85

Settable PWR_S3WU

PWR wake-up from Stop 3 interrupt

0x0000 0174

78

86

Settable SDMMC1

SDMMC1 global interrupt

0x0000 0178

79

87

Settable SDMMC2

SDMMC2 global interrupt

0x0000 017C

80

88

Settable GPDMA1_CH8

GPDMA1 channel 8 interrupt

0x0000 0180

81

89

Settable GPDMA1_CH9

GPDMA1 channel 9 interrupt

0x0000 0184

82

90

Settable GPDMA1_CH10

GPDMA1 channel 10 interrupt

0x0000 0188

83

91

Settable GPDMA1_CH11

GPDMA1 channel 11 interrupt

0x0000 018C

84

92

Settable GPDMA1_CH12

GPDMA1 channel 12 interrupt

0x0000 0190

85

93

Settable GPDMA1_CH13

GPDMA1 channel 13 interrupt

0x0000 0194

86

94

Settable GPDMA1_CH14

GPDMA1 channel 14 interrupt

0x0000 0198

87

95

Settable GPDMA1_CH15

GPDMA1 channel 15 interrupt

0x0000 019C

88

96

Settable I2C3_EV

I2C3 event interrupt

0x0000 01A0

89

97

Settable I2C3_ER

I2C3 error interrupt

0x0000 01A4

90

98

Settable SAI1

SAI1 global interrupt

0x0000 01A8

91

99

Settable SAI2

SAI2 global interrupt

0x0000 01AC

92

100

Settable TSC

TSC global interrupt

0x0000 01B0

93

101

Settable AES

AES global interrupt

0x0000 01B4

94

102

Settable RNG

RNG global interrupt

0x0000 01B8

95

103

Settable FPU

Floating point interrupt

0x0000 01BC

96

104

Settable HASH

HASH interrupt

0x0000 01C0

97

105

Settable PKA

PKA global interrupt

0x0000 01C4

98

106

Settable LPTIM3

LPTIM3 global interrupt

0x0000 01C8

99

107

Settable SPI3

SPI3 global interrupt

0x0000 01CC

100 108

Settable I2C4_ER

I2C4 error interrupt

0x0000 01D0

101 109

Settable I2C4_EV

I2C4 event interrupt

0x0000 01D4

102 110

Settable MDF1_FLT0

MDF1 filter 0 global interrupt

0x0000 01D8

103 111

Settable MDF1_FLT1

MDF1 filter 1 global interrupt

0x0000 01DC

USB/OTG_FS/
OTG_HS

RM0456 Rev 6

<!-- pagebreak -->

917

Nested vectored interrupt controller (NVIC)

RM0456

Priority

Position

Table 187. STM32U5 series vector table(1) (continued)
Type of
priority

Acronym

Description

Address

104 112

Settable MDF1_FLT2

MDF1 filter 2 global interrupt

0x0000 01E0

105 113

Settable MDF1_FLT3

MDF1 filter 3 global interrupt

0x0000 01E4

106 114

Settable UCPD1

UCPD1 global interrupt

0x0000 01E8

107 115

Settable ICACHE

Instruction cache global interrupt

0x0000 01EC

108 116

Settable OTFDEC1

OTFDEC1 secure global interrupt

0x0000 01F0

109 117

Settable OTFDEC2

OTFDEC2 secure global interrupt

0x0000 01F4

110 118

Settable LPTIM4

LPTIM4 global interrupt

0x0000 01F8

111 119

Settable DCACHE1

Data cache global interrupt

0x0000 01FC

112 120

Settable ADF1_FLT0

ADF1 filter 0 global interrupt

0x0000 0200

113 121

Settable ADC4

ADC4 (12 bits) global interrupt

0x0000 0204

114 122

Settable LPDMA1_CH0

LPDMA1 SmartRun channel 0 global interrupt

0x0000 0208

115 123

Settable LPDMA1_CH1

LPDMA1 SmartRun channel 1 global interrupt

0x0000 020C

116 124

Settable LPDMA1_CH2

LPDMA1 SmartRun channel 2 global interrupt

0x0000 0210

117 125

Settable LPDMA1_CH3

LPDMA1 SmartRun channel 3 global interrupt

0x0000 0214

118 126

Settable DMA2D

DMA2D global interrupt

0x0000 0218

119 127

Settable DCMI_PSSI

DCMI/PSSI global interrupt

0x0000 021C

120 128

Settable OCTOSPI2

OCTOSPI2 global interrupt

0x0000 0220

121 129

Settable MDF1_FLT4

MDF1 filter 4 global interrupt

0x0000 0224

122 130

Settable MDF1_FLT5

MDF1 filter 5 global interrupt

0x0000 0228

123 131

Settable CORDIC

CORDIC interrupt

0x0000 022C

124 132

Settable FMAC

FMAC interrupt

0x0000 0230

125 133

Settable

LSECSS interrupt(2)
MSI PLL unlock interrupt(2)

0x0000 0234

126 134

Settable USART6

USART6 global interrupt

0x0000 0238

127 135

Settable I2C5_ER

I2C5 error interrupt

0x0000 023C

128 136

Settable I2C5_EV

I2C5 event interrupt

0x0000 0240

129 137

Settable I2C6_ER

I2C6 error interrupt

0x0000 0244

130 138

Settable I2C6_EV

I2C6 event interrupt

0x0000 0248

131 139

Settable HSPI1

Hexadeca-SPI1 global interrupt

0x0000 024C

132 140

Settable GPU2D_IRQ

GPU2D interrupt

0x0000 0250

133 141

Settable GPU2D_IRQSYS

GPU2D system interrupt

0x0000 0254

134 142

Settable GFXMMU

GFXMMU global error interrupt

0x0000 0258

135 143

Settable LCD_TFT

LTDC global interrupt

0x0000 025C

<!-- pagebreak -->

LSECSS
MSI_PLL_UNLOCK

RM0456 Rev 6

RM0456

Nested vectored interrupt controller (NVIC)

Priority

Position

Table 187. STM32U5 series vector table(1) (continued)
Type of
priority

Acronym

Description

Address

136 144

Settable LCD_TFT_ERR

LTDC global error interrupt

0x0000 0260

137 145

Settable DSIHOST

DSI global interrupt

0x0000 0264

138 146

Settable DCACHE2

DCACHE 2 global interrupt

0x0000 0268

139 147

Settable GFXTIM

GFXTIM global interrupt

0x0000 026C

140 148

Settable JPEG

JPEG sync interrupt

0x0000 0270

1. Some interrupt lines are only available on some STM32U5 Series devices. Refer to the device datasheet for availability of
associated peripheral. If not present, consider this interrupt line as reserved.
2. Reserved in STM32U575/585 rev. X devices. LSECSS and MSI_PLL_UNLOCK interrupt lines are available in all other
revisions of STM32U575/585 and on all other STM32U5 Series devices.

RM0456 Rev 6

<!-- pagebreak -->

