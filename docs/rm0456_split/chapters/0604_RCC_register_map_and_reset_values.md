609

Reset and clock control (RCC)

RM0456

Bit 1 NSPRIV: RCC nonsecure function privilege configuration
This bit is set and reset by software. It can be written only by privileged access, secure or
nonsecure.
0: Read and write to RCC nonsecure functions can be done by privileged or unprivileged
access.
1: Read and write to RCC nonsecure functions can be done by privileged access only.
Bit 0 SPRIV: RCC secure function privilege configuration
This bit is set and reset by software. It can be written only by a secure privileged access.
0: Read and write to RCC secure functions can be done by privileged or unprivileged access.
1: Read and write to RCC secure functions can be done by privileged access only.

11.8.53

RCC register map

0

0

RCC_ICSCR2

Res.

Res.

Res.

Res.

Res.

Res.

<!-- pagebreak -->

MSISRDY

MSIKERON

MSISON

MSIKON

MSIPLLEN

MSIKRDY

1

0

1

MSITRIM0[4:0]

MSITRIM1[4:0]

MSITRIM2[4:0]

MSITRIM3[4:0]

0

1

0

0

1

1

Res.

X X X X X X X X X X X X X

Res.

X

MSICAL3[4:0]

1

0

0

0

0

0

0

0

Res.

Res.

Res.

Res.

HSITRIM[4:0]
1
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

X X X X

0

0

0

0

0

0

0

0

0

0

HSICAL[11:0]
X X X X X X X X X X X X

RM0456 Rev 6

HSI48CAL[8:0]

SWS[1:0]

0

0

SW[1:0]

STOPWUCK

HPRE[3:0]

0

0

0

0

0

0

0

PPRE3
[2:0]

Res.

Res.

PPRE1
[2:0]

Res.

0

Res.

0

Res.

0

Res.

Res.

STOPKERWUCK

Res.

Res.

Res.
0
Res.

0

0

Res.

Res.

PPRE2
[2:0]

Res.

Res.

Res.
Res.

0
Res.

1

Res.

Res.

Res.

Res.

1

Res.

0

DPRE[2:0]

Res.

0

Res.

0
Res.

Res.
AHB1DIS

Res.
AHB2DIS1

Res.

Res.
APB1DIS

0
APB3DIS

Res.

Res.

Res.

Res.
Res.

Res.

AHB2DIS2

Res.

0
Res.

Res.

Res.

Res.

0
Res.

Res.
Res.

APB2DIS

Res.
Res.

Res.

Res.

Res.

0
Res.

0
Res.

0

Res.

0

Res.

0

Res.

Res.

MCOSEL
[3:0]

MCOPRE
[2:0]
0

Res.

Res.

0

Res.

Res.

0

AHB3DIS

Reset value

Res.

RCC_CFGR3

0

Reserved

Reset value

0x024

1

X X X X X X X X X

Reset value

0x020

1

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

X

Reserved

RCC_CFGR2

0

Res.

Res.

Res.

X

Reset value

RCC_CFGR1

0

Res.

0

0x01C

0

MSICAL2[4:0]

Res.

0

Res.

1

MSICAL1[4:0]

Res.

0

MSICAL0[4:0]

Res.

0

0x018

MSIPLLSEL

0

HSION

0

MSIPLLFAST

0

HSIKERON

0

Res.

HSI48ON

0

HSIRDY

SHSION

HSI48RDY

0

Res.

0

RCC_CRRCR

0

Res.

MSIBIAS
Res.

1

Res.

MSIRGSEL

0

Res.

MSIKRANGE
[3:0]

.MSISRANGE
[3:0]

Reset value

RCC_ICSCR1

RCC_ICSCR3

HSEON

0

SHSIRDY

HSERDY

0

Reserved

Reset value
0x014

0

Reserved

Reset value
0x010

CSSON

0

HSEBYP

Res.

HSEEXT

PLL1ON
0

Res.

PLL1RDY
0

Res.

PLL2ON
0

Res.

0x00C

0

Res.

0x008

0

Res.

0x004

0

PLL3ON

Reset value

PLL2RDY

RCC_CR

PLL3RDY

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

0x000

Register
name

Res.

Offset

Res.

Table 119. RCC register map and reset values

0

0

0

0

0x054

RCC_CIFR

Reset value

RM0456 Rev 6
0

Reset value
Reserved
0
0
0
0
0

0

0

0

0

0
0
0
0
0
Res.

0
Res.

0

LSIRDYIE

0

LSERDYIE

PLL3FRACN[12:0]

0

0

0

0

0

0

0

0

0

LSIRDYF

1

LSERDYF

0

Res.

0

HSIRDYIE

0
0

MSISRDYIE

0
0

HSIRDYF

0
0
1

MSISRDYF

0
0
0

HSERDYIE

PLL3Q[6:0]
0
0

HSERDYF

0
0

0

0

0

0

0

1

PLL1FRACN[12:0]
0

1

0

1

0
0
0

0
0
0
0

PLL2P[6:0]
PLL2N[8:0]

PLL2FRACN[12:0]
0
0
0
0

0
0
0
0

PLL3P[6:0]

0
0
0
0

Res.

0
Res.

PLL1P[6:0]

PLL3N[8:0]

Res.

0

Res.

PLL3M[3:0]

Res.

0
0

0
0

PLL1RGE[1:0]
PLL1SRC[1:0]

PLL1FRACEN

Res.

Res.

Res.

0
PLL2SRC[1:0]

PLL2RGE[1:0]

Res.

Res.

0

PLL3SRC[1:0]

Res.

PLL2FRACEN

0
PLL3RGE[1:0]

Res.

0

PLL3FRACEN

Res.

PLL2M[3:0]

Res.

0

Res.

0

Res.

0

Res.

0

Res.

Res.

0
.PLL1MBOOST
[3:0]

PLL1PEN
0

PLL1M[3:0]

Res.

0

HSI48RDYIE

0
0
0

1

HSI48RDYF

0
0

0

PLL1RDYIE

0
0

0

PLL2RDYIE

PLL2Q[6:0]
0

0

0

PLL1RDYF

0

0

0

0

PLL2RDYF

0

Res.

Res.

Res.

PLL2PEN

PLL3PEN

PLL1QEN

PLL2QEN

PLL3QEN

0

0

Res.

0

0

PLL3RDYIE

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

0
0

PLL3RDYF

Reset value

Res.

Res.

Res.

PLL1REN

PLL2REN

Res.
PLL3REN

Res.

Res.

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

RCC_
PLL1CFGR
Res.

Register
name

0
0

Res.

0
0

CSSF

Reset value
0

SHSIRDYIE

0

MSIKRDYIE

1

Res.

PLL1Q[6:0]

SHSIRDYF

0

Res.

0

Res.

Res.

Res.

0

Res.

1

Res.

0

Res.

Res.

Res.

Res.

0

Res.

0

Res.

0

Res.

Res.

Res.

Res.

Res.

0

Res.

1

Res.

0

Res.

Res.

Res.

Res.

0

Res.

0

Res.

0

Res.

Res.

Res.

Res.

0

Res.

0

Res.

0

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

0

MSIKRDYF

Reset value

Res.

Res.

Reserved

Res.

0

Res.

0

Res.

Res.

Reset value

Res.

0

Res.

0

Res.

Res.

Res.

Res.

Res.

Res.

Reset value

Res.

Res.

Res.

0

Res.

Res.

Res.

Res.

Res.

Reset value

Res.

Res.

Res.

1

Res.

1

Res.

Res.

PLL2R[6:0]

Res.

0

Res.
1

Res.

PLL3R[6:0]
Res.

0

Res.
0

Res.

0

Res.

0

Res.
0

Res.

0

Res.

1

Res.
0

Res.

0

Res.

0

Res.
1

Res.

1

Res.

0

Res.
0

Res.

Res.
0

Res.

Res.

Res.

Res.

Res.

0

Res.

Res.

PLL1R[6:0]

Res.

Res.

Res.

RCC_CIER

Res.

0x050

Res.

0x04C
RCC_
PLL3FRACR
0

Res.

Reset value

Res.

0x048
RCC_
PLL3DIVR

Res.

0x044
RCC_
PLL2FRACR

Res.

Reset value

Res.

Reset value

Res.

0x040
RCC_
PLL2DIVR

Res.

0x03C
RCC_
PLL1FRACR

Res.

0x038
RCC_
PLL1DIVR

Res.

0x034
RCC_
PLL3CFGR

Res.

0x030
RCC_
PLL2CFGR

Res.

0x02C

Res.

0x028

Res.

Offset

Res.

RM0456
Reset and clock control (RCC)

Table 119. RCC register map and reset values (continued)

PLL1N[8:0]

0
0

0
0

0
0
0

0
0
0

0

0

0
0
0

0
0
0

0

0

0

0

0

0

0

0

0

<!-- pagebreak -->

609

0x07C

<!-- pagebreak -->

RCC_
APB2RSTR

0

0

0

0

0

0

TIM16RST

TIM15RST

RM0456 Rev 6

0

0

0

0

0

0

0

0
TIM3RST
TIM2RST

Res.

0
LPGPIO1RST

Res.
Res.

GPIOARST

OCTOSPI1RST

GPIOBRST

Res.

0
0

Res.

Res.

0
FSMCRST

0

GPIOCRST

0

0
0
0

Res.

0

I2C4RST

Res.

GPIOERST
GPIODRST

0

Res.

TIM4RST

0

Res.

Res.

0

Res.

GPIOFRST

0

Res.

TIM5RST

GPIOHRST
GPIOGRST

0

Res.

LSERDYC
LSIRDYC

0
0
0

CORDICRST

0

GPDMA1RST

HSIRDYC
MSISRDYC

0

FMACRST

HSERDYC

0

Res.

Res.

PLL1RDYC

Res.

HSI48RDYC

PLL2RDYC

Res.

Res.
PLL3RDYC

Res.

0

MDF1RST

0

Res.

TIM6RST

0

0

Res.

ADC4RST

GPIOIRST
0

OCTOSPI2RST

CSSC

Res.
Res.

MSIKRDYC

Res.

0

Res.

0

Res.

LPTIM2RST

0

Res.

Res.
DAC1RST

0

Res.

Res.

Res.

GPIOJRST
0

Res.

SHSIRDYC

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

Res.

0

Res.

I2C5RST

Res.

0

TIM7RST

I2C6RST

0

Res.

0

Res.

Res.

0

Res.

LPDMA1RST

0

Res.

Res.

0

CRCRST

Res.

Res.

Reserved
0

FDCAN1RST

Res.
ADC12RST
0

Res.

Res.

DCMI_PSSIRST.

Res.

OTGRST

JPEGRST

Res.

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

RCC_CICR
Res.

Register
name

0

Res.

Res.
ADF1RST

Reserved
0

Res.

Res.

Reset value

Res.

Res.

HSPI1RST

Res.
0

Res.

Reset value

Res.

Res.

0

Res.

Res.

Res.

Res.

Reset value

Res.

Res.

Res.

Res.

0

SPI2RST

Res.

TSCRST

AESRST

Res.

RAMCFGRST

HASHRST

Res.

0

Res.

Res.

Res.

Res.

Res.

DMA2DRST

RNGRST

Res.

0

TIM1RST

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

GFXMMURST

PKARST

0

0

SPI1RST

0

Res.

Res.

0

GPU2DRST

SAESRST

0

0

TIM8RST

USART2RST

0

Res.

Res.

OCTOSPIMRST

0

Res.

Res.

0

0

USART1RST

USART3RST

0
Res.

UART4RST

0

Res.

UART5RST

0

Res.

Res.

Res.
0

Res.

TIM17RST

Res.

I2C1RST

0

Res.

Res.

OTFDEC1RST

Res.

Reset value

Res.

SAI1RST

I2C2RST

OTFDEC2RST

Res.

Res.

Res.

Res.

Res.

0

Res.

Res.

Res.

Res.

Res.

SDMMC1RST

Res.

Res.
SDMMC2RST

Res.

Res.

0

SAI2RST

Reset value
Res.

CRSRST

0

UCPD1RST

USART6RST

0

Res.

Reset value

Res.

Res.

Res.

Res.

Res.

Res.

Res.

0

Res.

Res.

Res.

Res.

Res.

0

Res.

USBRST

Reset value

GFXTIMRST

RCC_
APB1RSTR2

LTDCRST

RCC_
APB1RSTR1

Res.

0x074

Res.

Reserved

Res.

0x070

DSIRST

0x078
RCC_
AHB3RSTR
Res.

Reset value

Res.

0x06C
RCC_
AHB2RSTR2

Res.

0x068
RCC_
AHB2RSTR1

Res.

0x064

Res.

RCC_
AHB1RSTR

Res.

0x060

Res.

Reserved

Res.

0x05C

Res.

0x058

Res.

Offset

Res.

Reset and clock control (RCC)
RM0456

Table 119. RCC register map and reset values (continued)

0
0
0
0

0

0

0

0x0A4

0x0A8

0x0AC

RCC_
APB2ENR

RCC_
APB3ENR

Reserved

Reset value

0

0

0

0

0

0

RM0456 Rev 6

Res.

Res.

Res.

Res.

Res.

Res.

Res.

LPUART1EN

SPI3EN

Res.

Res.

Res.

SYSCFGEN

Res.

0

Res.

0

I2C3EN

0

Res.

0

Res.

0

Res.

0

Res.

0
TIM3EN
TIM2EN

0
0
0
0

I2C4EN

0

0

LPGPIO1EN

0

Res.

GPIOGEN
GPIOFEN
GPIOEEN
GPIODEN
GPIOCEN
GPIOBEN
GPIOAEN

0
0
0
0
0
0
0

Res.
Res.
OCTOSPI1EN
Res.
Res.
Res.
FSMCEN

SPI3RST

Res.

FMACEN
CORDICEN
GPDMA1EN

Res.

SYSCFGRST

Res.

Res.

Res.

LPUART1RST

Res.
Res.

I2C3RST

Res.

Res.
Res.

MDF1EN

1

PWREN

GPIOHEN
0

Res.

FLASHEN.

0

Res.

TIM4EN

GPIOIEN
0

OCTOSPI2EN

Res.

Res.

LPTIM1RST

0

Res.

GPIOJEN
0

Res.

LPTIM3RST
Res.
Res.

LPTIM4RST
CRCEN.

0

Res.

ADC4EN

0

Res.

0
TIM5EN

0
0

Res.

0
TIM6EN

0

Res.

DAC1EN

Res.

0

Res.

Res.
0

TIM7EN

I2C5EN
LPTIM2EN

0
I2C6EN

0

Res.

0

Res.

LPDMA1EN

Reserved
0

Res.

ADC12EN

Res.
0

Res.

Res.

DCMI_PSSIEN.

0

Res.

FDCAN1EN

0
ADF1EN

Res.

HSPI1EN

0

Res.

WWDGEN

GTZC2EN

0

Res.

0

Res.

Res.

OPAMPRST

COMPRST

Res.

Res.

Res.

Res.

VREFRST

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

0

Res.

TIM1EN

0

LPTIM1EN

Res.

Res.

0

Res.

Reserved

Res.

JPEGEN

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

RCC_
APB3RSTR
Res.

Register
name

0

Res.

SPI1EN

0

LPTIM3EN

Res.

OTGEN

Res.
Res.

0

Res.

Res.

TSCEN
0

Res.

0

Res.

RAMCFGEN
0

SPI2EN

OTGHSPHYEN

0

Res.

DMA2DEN

0

Res.

Res.

AESEN

0

Res.

GFXMMUEN

0

Res.

Res.

HASHEN

0

Res.

0

TIM8EN

Res.

Res.

Res.

PKAEN
RNGEN

0

Res.

0

USART1EN

Res.

Res.

Res.

GPU2DEN
0

LPTIM4EN

Res.

SAESEN

0

Res.

DCACHE2EN

Res.

Res.

1

OPAMPEN

0
Res.

0

Res.

USART2EN

0

Res.

TIM15EN

0

Res.

UART4EN
USART3EN

0

Res.

Res.

OCTOSPIMEN

0

Res.

Res.

Res.
GTZC1EN

Reserved

COMPEN

TIM16EN

0

Res.

Res.

TIM17EN

0

Res.

UART5EN

0

Res.

Res.

Res.

0

Res.

0

VREFEN

I2C1EN

0

Res.

Res.

OTFDEC1EN

Res.

Res.
OTFDEC2EN

Res.

Res.

Res.

Res.

BKPSRAMEN

Reset value

Res.

SAI1EN

0

RTCAPBEN

I2C2EN

Res.

Res.

Res.

Res.

SDMMC1EN

Res.
Res.

SDMMC2EN

Res.

0

Res.

Res.

Res.

Res.

Res.

Res.

0

SAI2EN

Reset value
UCPD1EN

CRSEN

0

Res.

USART6EN

0

Res.

Reset value

Res.

USBEN

Res.

1

GFXTIMEN

SRAM4EN

Reset value

Res.

RCC_
AHB3ENR

Res.

1
0

Res.

SRAM6EN

1
0

Res.

SRAM5EN

Reset value

LTDCEN

RCC_
AHB2ENR2

Res.

1

Res.

SRAM2EN

1
Res.

SRAM3EN

Reset value
Res.

RCC_
AHB2ENR1

Res.
1

Res.

1

Res.

Res.

Res.

1

Res.

Reset value
Res.

SRAM1EN
DCACHE1EN

Reset value

DSIEN

RCC_
APB1ENR2
Res.

RCC_
AHB1ENR

Res.

0x0A0

Res.

RCC_
APB1ENR1

Res.

0x09C

Res.

Reserved

Res.

0x098

Res.

0x094

Res.

0x090

Res.

0x08C

Res.

0x088

Res.

0x084

Res.

0x080

Res.

Offset

Res.

RM0456
Reset and clock control (RCC)

Table 119. RCC register map and reset values (continued)

0

0
0
0
0

0

0

0

0

0

0

0

Reserved

<!-- pagebreak -->

609

0x0CC

0x0D0

RCC_
APB2SMENR

RCC_
APB3SMENR

0x0D4

<!-- pagebreak -->

Reserved

1

1

1

1

1

TIM15SMEN

RM0456 Rev 6

1

Reserved

1

1

1

1

1

Res.

1

Res.

TIM4SMEN
TIM3SMEN
TIM2SMEN

1
1
1
1

Res.
I2C4SMEN

1

1

LPGPIO1SMEN

Res.

PWRSMEN

Res.

1

Res.

TIM5SMEN

Res.

ADC4SMEN

Res.

1
1

Res.

Res.

Res.

1

TIM6SMEN

1
1

Res.

1

Res.

Res.

Res.

LPTIM2SMEN

Res.
DAC1SMEN
1

Res.

Res.
1

TIM7SMEN

I2C5SMEN

Res.

1

Res.

Res.

SPI3SMEN

I2C6SMEN

Res.

ADC12SMEN
GPIOJSMEN
GPIOISMEN
GPIOHSMEN
GPIOGSMEN
GPIOFSMEN
GPIOESMEN
GPIODSMEN
GPIOCSMEN
GPIOBSMEN
GPIOASMEN

1
1
1
1
1
1
1
1
1
1

Res.
OCTOSPI2SMEN
Res.
Res.
Res.
OCTOSPI1SMEN
Res.
Res.
Res.
FSMCSMEN

Res.

DCMI_PSSISMEN

1

Res.

Res.

HSPI1SMEN

Res.

FMACSMEN
CORDICSMEN
GPDMA1SMEN

Res.

Res.

Res.

Res.

FLASHSMEN

Res.

Res.

Res.

CRCSMEN

MDF1SMEN

1

SYSCFGSMEN

Res.

1

Res.

Res.

LPDMA1SMEN
1

Res.

1

I2C3SMEN

1
FDCAN1SMEN

Res.

GTZC2SMEN

Res.

TSCSMEN
JPEGSMEN

1

LPUART1SMEN

Res.

1

Res.

1
ADF1SMEN

Reserved
1

Res.

WWDGSMEN

1

Res.

1

Res.

Res.

Res.

1

Res.

TIM1SMEN

1

LPTIM1SMEN

Res.

Res.

Res.

OTGSMEN
1

Res.

1

Res.

Res.

1

Res.

DMA2DSMEN
RAMCFGSMEN

1

Res.

AESSMEN
OTGHSPHYSMEN

1

Res.

1

SPI2SMEN

Res.

Res.

RNGSMEN
HASHSMEN

1

Res.

GPU2DSMEN
GFXMMUSMEN

1

Res.

Res.

PKASMEN
1

Res.

1

Res.

Res.

Res.

Res.

SAESSMEN
1

Res.

Res.
DCACHE2SMEN

1

Res.

SPI1SMEN

1

LPTIM3SMEN

Res.

OCTOSPIMSMEN
1

Res.

Res.

1

Res.

TIM8SMEN

1

LPTIM4SMEN

Res.

Res.

Res.

Res.

Res.

OTFDEC1SMEN

Res.

Res.

GTZC1SMEN

Res.

Res.

Res.

1

Res.

USART1SMEN

1

OPAMPSMEN

Res.

Res.

USART2SMEN

1

Res.

USART3SMEN

1

COMPSMEN

UART4SMEN

1

Res.

TIM16SMEN

1

Res.

UART5SMEN

1

Res.

Reserved

Res.

TIM17SMEN

1

Res.

Res.

Res.

1

VREFSMEN

I2C1SMEN

1

Res.

Res.

Res.

Res.
OTFDEC2SMEN

Res.

Res.

Res.

SDMMC1SMEN

Res.
Res.

SDMMC2SMEN

Res.

1

Res.

SAI1SMEN

Res.
I2C2SMEN

1

Res.

Res.

Res.

Res.

Res.

Res.

1

RTCAPBSMEN

Reset value
UCPD1SMEN

CRSSMEN

1

Res.

Res.

1
1

SAI2SMEN

USART6SMEN

1

Res.

Reset value

Res.

USBSMEN

1

Res.

Res.

Res.

SRAM4SMEN

Reset value
1

Res.

Res.

RCC_
AHB3SMENR
1

Res.

GFXTIMSMEN

1

Res.

Reset value

DSISMEN

1

ICACHESMEN

SRAM6SMEN

1

BKPSRAMSMEN

SRAM5SMEN

Reset value
Res.

RCC_
AHB2SMENR2
Res.

1

Res.

SRAM2SMEN

1
1

LTDCSMEN

Res.

Res.

SRAM3SMEN

Reset value
1

Res.

Res.

Res.

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

RCC_
AHB2SMENR1
SRAM1SMEN

Reset value
DCACHE1SMEN

RCC_
AHB1SMENR

1

Res.

Res.

Res.

Register
name

1

Res.

Reset value

Res.

RCC_
APB1SMENR2

Res.

0x0C8
RCC_
APB1SMENR1

Res.

0x0C4
Res.

0x0C0

Res.

0x0BC

Res.

0x0B8

Res.

0x0B4

Res.

0x0B0

Res.

Offset

Res.

Reset and clock control (RCC)
RM0456

Table 119. RCC register map and reset values (continued)

1
1
1
1

1

1

1

0x114

RCC_
PRIVCFGR

RM0456 Rev 6
Res.
Res.
Res.
Res.
Res.
Res.
Res.
Res.

PLL2SEC
PLL1SEC
PRESCSEC
SYSCLKSEC
LSESEC
LSISEC
MSISEC
HSESEC
HSISEC

0
0
0
0
0
0
0
0
0
0
0
0

Res.

Res.

Res.

Res.

Res.

Res.

Res.

NSPRIV

SPRIV

MSIKSRANGE
[3:0]

MSISSRANGE
[3:0]

PLL3SEC

Reset value
0

Res.

Reset value
ICLKSEC

0

Res.
0

HSI48SEC
1

RMVFSEC
0

Res.

0

Res.

0

Res.

Reserved
1

Res.

0

Res.

Res.

Res.

Res.

UART5SEL
[1:0]
UART4SEL
[1:0]

0
0
0
0
0
0
0

ADF1SEL
[2:0]

0
0
0
0
0
0
0
0
0

Res.

I2C1SEL
[1:0]

SAESSEL

0

LPTIM1SEL
[1:0]

I2C2SEL
[1:0]

Res.

0

0
0
0
0
0

LSERDY
LSEON

0

USART2SEL
[1:0]
USART1SEL
[1:0]

0

USART3SEL
[1:0]

LPUART1AMEN
SPI3AMEN

0
0

0
0
0
0

0

0
0

0
0
0
0
0

Res.

Res.

Res.

Res.

Res.

I2C3AMEN

Res.

Res.

0

LPUART1SEL
[2:0]

Res.

0

Res.

0

0
SPI3SE
L1:0]

0

MDF1SEL
[2:0]

Res.

0
0
LSEBYP

0

LSEDRV[1:0]

0

LSECSSD

0

LSECSSON

0
I2C3SEL
[1:0]

0

LSESYSEN

LPTIM1AMEN

0

Res.

LPTIM3AMEN

0

LPTIM34SE
L[1:0]

OPAMPAMEN
LPTIM4AMEN

COMPAMEN

Res.

Res.

Res.

Res.

VREFAMEN

0

RTCSEL[1:0]

LSEGFON
LSESYSRDY

ADCDACSEL
[2:0]

Res.

SAI2SEL SAI1SEL
[2:0]
[2:0]

Res.

RNGSEL
[1:0]

0

SDMMCSEL

0

Res.

0

Res.

I2C4SEL
[1:0]

0

DSISEL

0

DAC1SEL

0

RTCEN

SPI2SEL
[1:0]

0

USART6SEL
[1:0]

0

BDRST

0

Res.

LPTIM2SE
L[1:0]
0

LTDCSEL

0

Res.

SPI1SEL
[1:0]

RTCAPBAMEN

Res.

Res.

0

Res.

Res.

Res.

0

Res.

Res.

Res.

Reserved

Res.

Res.

Reset value

Res.

OCTOSPISE
L[1:0]

0

Res.

Res.

0
0

Res.

Res.

Res.

Res.

Res.

0

Res.

Res.

Res.

Res.

SYSTICKSE
L

HSPI1SEL
[1:0]

ADC4AMEN

0

Res.

Res.

Res.

Res.

Res.
0

Res.

0
0

Res.

0

Res.

0

Res.

0

0
Res.

0

0

RMVF

FDCANSEL
[1:0]

I2C5SEL
[1:0]

0

0

Res.

1
Res.

Res.

0

Res.

1
LSCOEN

0
LSCOSEL

0
0

Res.

OBLRSTF

DAC1AMEN
LPGPIO1AMEN
0

Res.

ICLKSEL
[1:0]

I2C6SEL
[1:0]

0

0

Res.

Res.

Res.
0

ADF1AMEN

0

LPDMA1AMEN
0

Res.

0
LSION

PINRSTF

0
LSIRDY

BORRSTF

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
0

Res.

SRAM4AMEN

Reset value

Res.

0

Res.
0

Res.

TIMICSEL
[2:0]

0
0

Res.

Res.

Reset value
Res.

Res.

0

Res.

Res.

OTGHSSEL
[1:0]

0

LSIPREDIV

Res.

Res.

RCC_
SRDAMR

0

Res.

RCC_
SECCFGR
SFTRSTF

Reset value
IWDGRSTF

RCC_CSR
Res.

Reset value

Res.

RCC_BDCR
Res.

Register
name

0

Res.

0x110
RCC_
CCIPR2
0

Res.

0x0F80x10C
0

Res.

0x0F4
Reset value

Res.

0x0F0
RCC_
CCIPR3

LPWRRSTF

0x0E8

WWDGRSTF

0x0E4
RCC_
CCIPR1

Res.

0x0E0

Res.

0x0D8

Res.

Offset

Res.

RM0456
Reset and clock control (RCC)

Table 119. RCC register map and reset values (continued)

0

0

0
0
0

0
0
0
0
0
0
0
0

0

0

Refer to Section 2.3 for the register boundary addresses.

<!-- pagebreak -->

