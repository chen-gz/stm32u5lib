275

Global TrustZone controller (GTZC)

5.6.10

RM0456

GTZC1 TZSC memory x sub-region B watermark register
(GTZC1_TZSC_MPCWMxBR)
Address offset: 0x4C + 0x10 * (x - 1) (x = 1, 2, 5, 6)
Reset value: 0x0000 0000
Secure privilege access only.
When SUBB_START + SUBB_LENGTH is higher than the maximum size allowed for the
memory, a saturation of SUBB_LENGTH is applied automatically.
When an overlap of sub-region A and B exists, secure/privileged attributes of both subregions apply on the common section (see Section 5.4.3).

Note:

Some registers are only available on some devices in the STM32U5 series. Refer to the
device datasheet for availability of its associated memory region.

31

30

29

28

Res.

Res.

Res.

Res.

27

26

25

24

23

22

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

Res.

Res.

Res.

Res.

Res.

21

20

19

18

17

16

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

rw

rw

rw

rw

SUBB_LENGTH[11:0]

SUBB_START[10:0]
rw

rw

rw

rw

rw

rw

rw

Bits 31:28 Reserved, must be kept at reset value.
Bits 27:16 SUBB_LENGTH[11:0]: Length of sub-region B in region x
This field defines the length of the sub-region B, to be multiplied by the granularity defined
in Table 31.
When SUBB_START + SUBB_LENGTH is higher than the maximum size allowed for the
memory, a saturation of SUBB_LENGTH is applied automatically.
If SUBB_LENGTH = 0, the sub-region B is disabled.(SREN bit in
GTZC1_TZSC_MPCMWxBCFGR is cleared).
Bits 15:11 Reserved, must be kept at reset value.
Bits 10:0 SUBB_START[10:0]: Start of sub-region B in region x
This field defines the address offset of the sub-region B, to be multiplied by the granularity
defined in Table 31, versus the start of the region x.
External memories that are watermark controlled, start fully nonsecure at reset when
TZEN = 0. When TZEN = 1, external memories start fully secure (inverted reset-value).

5.6.11

GTZC1 TZSC register map

Reset value

<!-- pagebreak -->

LPTIM2SEC

I2C4SEC

CRSSEC

I2C2SEC

I2C1SEC

UART5SEC

UART4SEC

USART3SEC

USART2SEC

SPI2SEC

IWDGSEC

WWDGSEC

TIM7SEC

TIM6SEC

TIM5SEC

TIM4SEC

TIM3SEC

TIM2SEC

0

FDCAN1SEC

0

UCPD1SEC

USART6SEC

0

Res.

I2C5SEC

Res.

Res.

Res.

Res.

I2C6SEC

Reset value

Res.

GTZC1_TZSC_
SECCFGR1

Reserved

Res.

Reserved

Res.

0x010

0

Res.

0x0040x00C

LCK

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

GTZC1_TZSC_CR

Res.

Offset

0x000

Register name

Res.

Table 39. GTZC1 TZSC register map and reset values

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

RM0456 Rev 6

0x050

GTZC1_TZSC_
MPCWM2ACFGR

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

Res.

RM0456 Rev 6
0
0
0
0

Res.

Res.

Res.

Res.

0
0
0
0
0
0
0
0

Reserved

TIM2PRIV

0
0

Res.

TIM1PRIV

TIM3PRIV

0

Res.

FMACSEC
MDF1SEC

0

CORDICSEC

OTGSEC
DCMISEC

0

CRCSEC

AESSEC
0

TSCSEC

HASHSEC
0

DMA2DSEC

RNGSEC
0

ICACHE_REGSEC

PKASEC
0

ADC12SEC

SAESSEC
0

DCACHE1_REGSEC

OCTOSPIMSEC
0

0
0
0
0

0
0

MDF1PRIV

TIM4PRIV

0
SPI1PRIV

TIM5PRIV

0
TIM8PRIV

TIM6PRIV

0

Res.

SDMMC1SEC
0

FMACPRIV

TIM7PRIV

0

Res.

SDMMC2SEC
0

CORDICPRIV

WWDGPRIV

0

Res.

FSMC_REGSEC
0

0
0
0
0

Res.

SPI2PRIV
IWDGPRIV

0
USART1PRIV

USART2PRIV
0
TIM15PRIV

USART3PRIV
0
TIM16PRIV

UART4PRIV
0
TIM17PRIV

UART5PRIV
0
SAI1PRIV

I2C1PRIV
0

SAI2PRIV

I2C2PRIV
0

DSIPRIV

CRSPRIV
0

LTDCUSBPRIV

I2C4PRIV
0

GFXTIMPRIV

LPTIM2PRIV

0

Res.

OCTOSPI1_REGSEC
0

CRCPRIV

0

Res.

0
TSCPRIV

0
DMA2DPRIV

0

Res.

0

Res.

0
ICACHE_REGPRIV

OTGPRIV
DCMIPRIV

0

Res.

AESPRIV

0
ADC12PRIV

HASHPRIV

0
DCACHE1_REGPRIV

RNGPRIV

0

Res.

PKAPRIV

0

SEC

Res.

SAESPRIV

FDCAN1PRIV

0

Res.

OCTOSPI2_REGSEC
0

SUBA_START[10:0]

0

0

0

0

0

0
SREN

0

Res.

Res.

OCTOSPIMPRIV

0

Res.

Res.

SDMMC1PRIV

0

PRIV

Res.

SDMMC2PRIV

UCPD1PRIV

0

Res.

Res.

RAMCFGSEC
0

SRLOCK

0

SEC

Res.

Res.

Res.

Res.

Res.

FSMC_REGPRIV

Res.

GPU2DSEC

0

0
0

0
0
0

Res.

0

Res.

Reset value
0

PRIV

Res.

Res.

Res.

Res.

Res.

Res.

OCTOSPI1_REGPRIV

USART6PRIV

Res.

GFXMMUSEC

0

SUBB_START[10:0]

SREN

0

Res.

Res.

Res.

OCTOSPI2_REGPRIV

I2C5PRIV

Res.

GFXMMU_REGSEC

0

0

0

0

0

0

SREN

0

Res.

Res.

Res.

RAMCFGPRIV

I2C6PRIV

Res.

Res.

HSPI1_REGSEC

0

SRLOCK

0

Res.

Res.

Res.

GPU2DPRIV

Res.

Res.

Res.

DCACHE2_REGSEC

Res.
JPEGSEC

0

Res.

0

Res.

Res.

Res.

GFXMMUPRIV

Res.

Res.

Res.

Res.

Res.
0

SRLOCK

0

Res.

Reset value

0

SEC

SUBB_LENGTH[11:0]

Res.

0

PRIV

0

Res.

Res.

GFXMMU_REGPRIV

Reset value

Res.

0

Res.

Res.

Res.

HSPI1_REGPRIV

Res.

Res.

Res.

Res.

Res.

Register name
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

0x014
GTZC1_TZSC_
SECCFGR2
Res.

DSISEC
LTDCUSBSEC
SAI2SEC
SAI1SEC
TIM17SEC
TIM16SEC
TIM15SEC
USART1SEC
TIM8SEC
SPI1SEC
TIM1SEC

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

Res.

Res.

Offset

GFXTMSEC

Reset value

Res.

0

Res.

0

Res.

0

Res.

0

Res.

SUBA_LENGTH[11:0]

Res.

0

Res.

Res.

Reserved

Res.

0

Res.

Res.

DCACHE2_REGPRIV

Res.
JPEGPRIV

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

0

Res.

Res.

Res.

Res.

Reset value

Res.

0

Res.

Reset value

Res.

0x04C

GTZC1_TZSC_
MPCWM1BR

Res.

Reset value

Res.

GTZC1_TZSC_
MPCWM1BCFGR

Res.

Reset value

Res.

0x048

Res.

0x44
GTZC1_TZSC_
MPCWM1AR
Res.

Reset value

Res.

GTZC1_TZSC_
MPCWM1ACFGR

Res.

Reset value

0x040
Res.

0x02C0x03C
GTZC1_TZSC_
PRIVFGR3

Res.

0x028
GTZC1_TZSC_
PRIVCFGR2

Res.

GTZC1_TZSC_
PRIVCFGR1

Res.

0x020

Res.

Reserved

Res.

0x01C

Res.

0x024
GTZC1_TZSC_
SECCFGR3

Res.

0x018

Res.

RM0456
Global TrustZone controller (GTZC)

Table 39. GTZC1 TZSC register map and reset values (continued)

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

275

Reset value

0x098

GTZC1_TZSC_
MPCWM6BCFGR

0x09C

GTZC1_TZSC_
MPCWM6BR

Reset value

<!-- pagebreak -->

