275

Global TrustZone controller (GTZC)
•

RM0456

to 0x0000 0000, making these internal memories block nonsecure and unprivileged by
default when TrustZone security is disabled at system level (TZEN = 0)

For external memories and backup SRAM:
•

all GTZC_TZSC_MPCWMxzR registers are set to 0x0000 0000, making these
memories secure and privileged by default when TrustZone security is enabled a
system level (TZEN = 1).

•

GTZC_TZSC_MPCWMxzR registers are not accessible, and these memories are
nonsecure and unprivileged by default when TrustZone security is disabled at system
level (TZEN = 0).

Secure boot code can then program the security settings, making components secure or not
as needed.

5.5

GTZC interrupts
TZIC is a secure peripheral, thus it systematically generates an illegal access event when
accessed by a nonsecure access. The MPCBB and TZSC are TrustZone-aware
peripherals, meaning that secure and nonsecure registers co-exist within the peripheral.
Table 38. GTZC interrupt request
Event flag

Enable control bit

Interrupt clear
method

Exit
Sleep
mode

Exit
Exit
Stop Standby
mode
mode

All flags in
GTZC_TZIC_SRx

All bits in
GTZC_TZIC_IERx

Write 1 in the bit
GTZC_TZIC_FCRx

Yes

Yes

Interrupt Interrupt
acronym event
Illegal
access

GTZC

5.6

No

GTZC1 TZSC registers
All registers are accessed only by words (32-bit).

5.6.1

GTZC1 TZSC control register (GTZC1_TZSC_CR)
Address offset: 0x000
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

LCK
rs

Bits 31:1 Reserved, must be kept at reset value.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Global TrustZone controller (GTZC)

Bit 0 LCK: lock the configuration of GTZC1_TZSC_SECCFGRx and GTZC1_TZSC_PRIVCFGRx
registers until next reset
This bit is cleared by default and once set, it can not be reset until system reset.
0: configuration of all GTZC1_TZSC_SECCFGRx and GTZC1_TZSC_PRIVCFGRx registers
not locked
1: configuration of all GTZC1_TZSC_SECCFGRx and GTZC1_TZSC_PRIVCFGRx registers
locked

5.6.2

GTZC1 TZSC secure configuration register 1
(GTZC1_TZSC_SECCFGR1)
Address offset: 0x010
Reset value: 0x0000 0000
Write-secure access only.
This register can be written only by secure privileged transaction when corresponding
GTZC1_TZSC_PRIVCFGR register signal is set to 1. If a given PRIV bit is not set, the
equivalent SEC bit can be written by secure unprivileged transaction.
Read accesses are authorized for any type of transactions, secure or not, privileged or not.

31
Res.

15

30
Res.

14

29
Res.

13

28
Res.

12

27
Res.

11

26
Res.

10

25
Res.

9

24
Res.

23

22

21

I2C6SE I2C5SE USART
C
C
6SEC

8

rw

rw

rw

7

6

5

CRSSE I2C2SE I2C1SE UART5 UART4 USART USART SPI2SE IWDGS WWDG TIM7S
C
C
C
SEC
SEC
3SEC
2SEC
C
EC
SEC
EC
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

20
Res.

19

18

17

16

UCPD1 FDCAN LPTIM2 I2C4SE
SEC
1SEC
SEC
C
rw

rw

rw

rw

4

3

2

1

0

TIM6S
EC

TIM5S
EC

TIM4S
EC

TIM3S
EC

TIM2S
EC

rw

rw

rw

rw

rw

Bits 31:24 Reserved, must be kept at reset value.
Bit 23 I2C6SEC: secure access mode for I2C6
0: nonsecure
1: secure
Note: This bit is only available on some devices in the STM32U5 series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and kept at reset value.
Bit 22 I2C5SEC: secure access mode for I2C5
0: nonsecure
1: secure
Note: This bit is only available on some devices in the STM32U5 series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and kept at reset value.
Bit 21 USART6SEC: secure access mode for USART6
0: nonsecure
1: secure
Note: This bit is only available on some devices in the STM32U5 series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and kept at reset value.
Bit 20 Reserved, must be kept at reset value.

RM0456 Rev 6

<!-- pagebreak -->

275

Global TrustZone controller (GTZC)

RM0456

Bit 19 UCPD1SEC: secure access mode for UCPD1
0: nonsecure
1: secure
Note: This bit is only available on some devices in the STM32U5 series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and kept at reset value.
Bit 18 FDCAN1SEC: secure access mode for FDCAN1
0: nonsecure
1: secure
Note: This bit is only available on some devices in the STM32U5 series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and kept at reset value.
Bit 17 LPTIM2SEC: secure access mode for LPTIM2
0: nonsecure
1: secure
Bit 16 I2C4SEC: secure access mode for I2C4
0: nonsecure
1: secure
Bit 15 CRSSEC: secure access mode for CRS
0: nonsecure
1: secure
Bit 14 I2C2SEC: secure access mode for I2C2
0: nonsecure
1: secure
Bit 13 I2C1SEC: secure access mode for I2C1
0: nonsecure
1: secure
Bit 12 UART5SEC: secure access mode for UART5
0: nonsecure
1: secure
Bit 11 UART4SEC: secure access mode for UART4
0: nonsecure
1: secure
Bit 10 USART3SEC: secure access mode for USART3
0: nonsecure
1: secure
Bit 9 USART2SEC: secure access mode for USART2
0: nonsecure
1: secure
Note: This bit is only available on some devices in the STM32U5 series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and kept at reset value.
Bit 8 SPI2SEC: secure access mode for SPI2
0: nonsecure
1: secure

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Global TrustZone controller (GTZC)

Bit 7 IWDGSEC: secure access mode for IWDG
0: nonsecure
1: secure
Bit 6 WWDGSEC: secure access mode for WWDG
0: nonsecure
1: secure
Bit 5 TIM7SEC: secure access mode for TIM7
0: nonsecure
1: secure
Bit 4 TIM6SEC: secure access mode for TIM6
0: nonsecure
1: secure
Bit 3 TIM5SEC: secure access mode for TIM5
0: nonsecure
1: secure
Bit 2 TIM4SEC: secure access mode for TIM4
0: nonsecure
1: secure
Bit 1 TIM3SEC: secure access mode for TIM3
0: nonsecure
1: secure
Bit 0 TIM2SEC: secure access mode for TIM2
0: nonsecure
1: secure

5.6.3

GTZC1 TZSC secure configuration register 2
(GTZC1_TZSC_SECCFGR2)
Address offset: 0x014
Reset value: 0x0000 0000
Write-secure access only.
This register can be written only by secure privileged transaction when corresponding
GTZC1_TZSC_PRIVCFGR register signal is set to 1. If a given PRIV is not set, the
equivalent SEC bit can be written by secure unprivileged transaction.
Read accesses are authorized for any type of transactions, secure or not, privileged or not.

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

GFXTI
MSEC

Res.

Res.

Res.

rw

DSISE LTDCU SAI2SE SAI1SE TIM17S TIM16S TIM15S USART TIM8S SPI1SE TIM1S
C
SBSEC
C
C
EC
EC
EC
1SEC
EC
C
EC
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

Bits 31:12 Reserved, must be kept at reset value.

RM0456 Rev 6

<!-- pagebreak -->

275

Global TrustZone controller (GTZC)

RM0456

Bit 11 GFXTIMSEC: secure access mode for GFXTIM
0: nonsecure
1: secure
Note: This bit is only available on some devices in the STM32U5 series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and kept at reset value.
Bit 10 DSISEC: secure access mode for DSI
0: nonsecure
1: secure
Note: This bit is only available on some devices in the STM32U5 series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and kept at reset value.
Bit 9 LTDCUSBSEC: secure access mode for LTDC or USB
0: nonsecure
1: secure
Note: This bit is secure for the LTDC on STM32U59x/5Ax/5Fx/5Gx. It is secure for the USB
on STM32U535/545. It is reserved on STM32U575/585.
Bit 8 SAI2SEC: secure access mode for SAI2
0: nonsecure
1: secure
Note: This bit is only available on some devices in the STM32U5 series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and kept at reset value.
Bit 7 SAI1SEC: secure access mode for SAI1
0: nonsecure
1: secure
Bit 6 TIM17SEC: secure access mode for TIM7
0: nonsecure
1: secure
Bit 5 TIM16SEC: secure access mode for TIM6
0: nonsecure
1: secure
Bit 4 TIM15SEC: secure access mode for TIM5
0: nonsecure
1: secure
Bit 3 USART1SEC: secure access mode for USART1
0: nonsecure
1: secure
Bit 2 TIM8SEC: secure access mode for TIM8
0: nonsecure
1: secure
Bit 1 SPI1SEC: secure access mode for SPI1
0: nonsecure
1: secure

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Global TrustZone controller (GTZC)

Bit 0 TIM1SEC: secure access mode for TIM1
0: nonsecure
1: secure

5.6.4

GTZC1 TZSC secure configuration register 3
(GTZC1_TZSC_SECCFGR3)
Address offset: 0x018
Reset value: 0x0000 0000
Write-secure access only.
This register can be written only by secure privileged transaction when corresponding
GTZC1_TZSC_PRIVCFGR register signal is set to 1. If a given PRIV is not set, the
equivalent SEC bit can be written by secure unprivileged transaction.
Read accesses are authorized for any type of transactions, secure or not, privileged or not.

31

30

29

Res.

Res.

Res.

15

14

13

SAESS PKASE
EC
C
rw

rw

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

DCAC HSPI1_ GFXM GFXM
OCTOS OCTOS FSMC_
OCTOS
JPEGS
GPU2D RAMC
SDMM SDMM
HE2_R REGSE MU_RE MUSE
PI2_RE PI1_RE REGSE
PIMSE
EC
SEC FGSEC
C2SEC C1SEC
EGSEC
C
GSEC
C
GSEC GSEC
C
C
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

rw

rw

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

DCAC ICACH
RNGS HASHS AESSE OTGSE DCMIS ADC12
DMA2D TSCSE CRCSE FMACS CORDI MDF1S
HE1_R E_REG
EC
EC
C
C
EC
SEC
SEC
C
C
EC
CSEC
EC
EGSEC SEC
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

rw

rw

rw

Bits 31:29 Reserved, must be kept at reset value.
Bit 28 JPEGSEC: secure access mode for JPEG
0: nonsecure
1: secure
Note: This bit is only available on some devices in the STM32U5 series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and kept at reset value.
Bit 27 DCACHE2_REGSEC: secure access mode for DCACHE2 registers
0: nonsecure
1: secure
Note: This bit is only available on some devices in the STM32U5 series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and kept at reset value.
Bit 26 HSPI1_REGSEC: secure access mode for HSPI1 registers
0: nonsecure
1: secure
Note: This bit is only available on some devices in the STM32U5 series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and kept at reset value.

RM0456 Rev 6

<!-- pagebreak -->

275

Global TrustZone controller (GTZC)

RM0456

Bit 25 GFXMMU_REGSEC: secure access mode for GFXMMU registers
0: nonsecure
1: secure
Note: This bit is only available on some devices in the STM32U5 series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and kept at reset value.
Bit 24 GFXMMUSEC: secure access mode for GFXMMU
0: nonsecure
1: secure
Note: This bit is only available on some devices in the STM32U5 series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and kept at reset value.
Bit 23 GPU2DSEC: secure access mode for GPU2D
0: nonsecure
1: secure
Note: This bit is only available on some devices in the STM32U5 series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and kept at reset value.
Bit 22 RAMCFGSEC: secure access mode for RAMCFG
0: nonsecure
1: secure
Bit 21 OCTOSPI2_REGSEC: secure access mode for OCTOSPI2 registers
0: nonsecure
1: secure
Note: This bit is only available on some devices in the STM32U5 series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and kept at reset value.
Bit 20 OCTOSPI1_REGSEC: secure access mode for OCTOSPI1 registers
0: nonsecure
1: secure
Bit 19 FSMC_REGSEC: secure access mode for FSMC registers
0: nonsecure
1: secure
Note: This bit is only available on some devices in the STM32U5 series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and kept at reset value.
Bit 18 SDMMC2SEC: secure access mode for SDMMC1
0: nonsecure
1: secure
Note: This bit is only available on some devices in the STM32U5 series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and kept at reset value.
Bit 17 SDMMC1SEC: secure access mode for SDMMC2
0: nonsecure
1: secure

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Global TrustZone controller (GTZC)

Bit 16 OCTOSPIMSEC: secure access mode for OCTOSPIM
0: nonsecure
1: secure
Note: This bit is only available on some devices in the STM32U5 series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and kept at reset value.
Bit 15 SAESSEC: secure access mode for SAES
0: nonsecure
1: secure
Note: This bit is only available on some devices in the STM32U5 series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and kept at reset value.
Bit 14 PKASEC: secure access mode for PKA
0: nonsecure
1: secure
Note: This bit is only available on some devices in the STM32U5 series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and kept at reset value.
Bit 13 RNGSEC: secure access mode for RNG
0: nonsecure
1: secure
Bit 12 HASHSEC: secure access mode for HASH
0: nonsecure
1: secure
Bit 11 AESSEC: secure access mode for AES
0: nonsecure
1: secure
Note: This bit is only available on some devices in the STM32U5 series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and kept at reset value.
Bit 10 OTGSEC: secure access mode for OTG_FS or OTG_HS
0: nonsecure
1: secure
Note: This bit is only available on some devices in the STM32U5 series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and kept at reset value.
Bit 9 DCMISEC: secure access mode for DCMI and PSSI
0: nonsecure
1: secure
Bit 8 ADC12SEC: secure access mode for ADC1 and ADC2
0: nonsecure
1: secure
Bit 7 DCACHE1_REGSEC: secure access mode for DCACHE1 registers
0: nonsecure
1: secure

RM0456 Rev 6

<!-- pagebreak -->

275

Global TrustZone controller (GTZC)

RM0456

Bit 6 ICACHE_REGSEC: secure access mode for ICACHE registers
0: nonsecure
1: secure
Bit 5 DMA2DSEC: secure access mode for register of DMA2D
0: nonsecure
1: secure
Note: This bit is only available on some devices in the STM32U5 series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and kept at reset value.
Bit 4 TSCSEC: secure access mode for TSC
0: nonsecure
1: secure
Bit 3 CRCSEC: secure access mode for CRC
0: nonsecure
1: secure
Bit 2 FMACSEC: secure access mode for FMAC
0: nonsecure
1: secure
Bit 1 CORDICSEC: secure access mode for CORDIC
0: nonsecure
1: secure
Bit 0 MDF1SEC: secure access mode for MDF1
0: nonsecure
1: secure

5.6.5

GTZC1 TZSC privilege configuration register 1
(GTZC1_TZSC_PRIVCFGR1)
Address offset: 0x020
Reset value: 0x0000 0000
Write-privileged access only.
This register can be read or written only by secure privileged transaction when
corresponding GTZC1_TZSC_SECCFGR register signal is set to1. If a given SEC bit is not
set, the equivalent PRIV bit can be read/written by nonsecure privileged transaction.
Read accesses are authorized for any type of transactions, secure or not, privileged or not.

31

30

29

28

27

26

25

24

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

CRSPR I2C2PR I2C1PR UART5 UART4 USART USART
IV
IV
IV
PRIV
PRIV
3PRIV 2PRIV
rw

rw

rw

rw

rw

rw

rw

SPI2P
RIV

23

21

rw

rw

rw

7

6

5

IWDGP WWDG TIM7P
RIV
PRIV
RIV

rw

rw

Bits 31:24 Reserved, must be kept at reset value.

<!-- pagebreak -->

22

I2C6PR I2C5PR USART
IV
IV
6PRIV

RM0456 Rev 6

rw

rw

20
Res.

19

18

17

16

UCPD1 FDCAN LPTIM2 I2C4PR
PRIV
1PRIV
PRIV
IV
rw

rw

rw

4

3

2

1

rw
0

TIM6P
RIV

TIM5P
RIV

TIM4P
RIV

TIM3P
RIV

TIM2P
RIV

rw

rw

rw

rw

rw

RM0456

Global TrustZone controller (GTZC)

Bit 23 I2C6PRIV: privileged access mode for I2C6
0: unprivileged
1: privileged
Note: This bit is only available on some devices in the STM32U5 series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and kept at reset value.
Bit 22 I2C5PRIV: privileged access mode for I2C5
0: unprivileged
1: privileged
Note: This bit is only available on some devices in the STM32U5 series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and kept at reset value.
Bit 21 USART6PRIV: privileged access mode for USART6
0: unprivileged
1: privileged
Note: This bit is only available on some devices in the STM32U5 series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and kept at reset value.
Bit 20 Reserved, must be kept at reset value.
Bit 19 UCPD1PRIV: privileged access mode for UCPD1
0: unprivileged
1: privileged
Note: This bit is only available on some devices in the STM32U5 series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and kept at reset value.
Bit 18 FDCAN1PRIV: privileged access mode for FDCAN1
0: unprivileged
1: privileged
Note: This bit is only available on some devices in the STM32U5 series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and kept at reset value.
Bit 17 LPTIM2PRIV: privileged access mode for LPTIM2
0: unprivileged
1: privileged
Bit 16 I2C4PRIV: privileged access mode for I2C4
0: unprivileged
1: privileged
Bit 15 CRSPRIV: privileged access mode for CRS
0: unprivileged
1: privileged
Bit 14 I2C2PRIV: privileged access mode for I2C2
0: unprivileged
1: privileged
Bit 13 I2C1PRIV: privileged access mode for I2C1
0: unprivileged
1: privileged

RM0456 Rev 6

<!-- pagebreak -->

275

Global TrustZone controller (GTZC)

RM0456

Bit 12 UART5PRIV: privileged access mode for UART5
0: unprivileged
1: privileged
Bit 11 UART4PRIV: privileged access mode for UART4
0: unprivileged
1: privileged
Bit 10 USART3PRIV: privileged access mode for USART3
0: unprivileged
1: privileged
Bit 9 USART2PRIV: privileged access mode for USART2
0: unprivileged
1: privileged
Note: This bit is only available on some devices in the STM32U5 series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and kept at reset value.
Bit 8 SPI2PRIV: privileged access mode for SPI2
0: unprivileged
1: privileged
Bit 7 IWDGPRIV: privileged access mode for IWDG
0: unprivileged
1: privileged
Bit 6 WWDGPRIV: privileged access mode for WWDG
0: unprivileged
1: privileged
Bit 5 TIM7PRIV: privileged access mode for TIM7
0: unprivileged
1: privileged
Bit 4 TIM6PRIV: privileged access mode for TIM6
0: unprivileged
1: privileged
Bit 3 TIM5PRIV: privileged access mode for TIM5
0: unprivileged
1: privileged
Bit 2 TIM4PRIV: privileged access mode for TIM4
0: unprivileged
1: privileged
Bit 1 TIM3PRIV: privileged access mode for TIM3
0: unprivileged
1: privileged
Bit 0 TIM2PRIV: privileged access mode for TIM2
0: unprivileged
1: privileged

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Global TrustZone controller (GTZC)

5.6.6

GTZC1 TZSC privilege configuration register 2
(GTZC1_TZSC_PRIVCFGR2)
Address offset: 0x024
Reset value: 0x0000 0000
Write-privileged access only.
This register can be read or written only by secure privileged transaction when
corresponding GTZC1_TZSC_SECCFGR register signal is set to1. If a given SEC bit is not
set, the equivalent PRIV bit can be read/written by nonsecure privileged transaction.
Read accesses are authorized for any type of transactions, secure or not, privileged or not.

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

SPI1P
RIV

TIM1P
RIV

rw

rw

Res.

Res.

Res.

Res.

LTDCU
GFXTI DSIPRI
SBPRI
MPRIV
V
V
rw

rw

rw

SAI2P
RIV

SAI1P TIM17P TIM16P TIM15P USART TIM8P
RIV
RIV
RIV
RIV
1PRIV
RIV

rw

rw

rw

rw

rw

rw

rw

Bits 31:12 Reserved, must be kept at reset value.
Bit 11 GFXTIMPRIV: privileged access mode for GFXTIM
0: unprivileged
1: privileged
Note: This bit is only available on some devices in the STM32U5 series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and kept at reset value.
Bit 10 DSIPRIV: privileged access mode for DSI
0: unprivileged
1: privileged
Note: This bit is only available on some devices in the STM32U5 series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and kept at reset value.
Bit 9 LTDCUSBPRIV: privileged access mode for LTDC or USB
0: unprivileged
1: privileged
Note: This bit privileges the LTDC on STM32U59x/5Ax/5Fx/5Gx. It privileges the USB
on STM32U535/545. It is reserved on STM32U575/585.
Bit 8 SAI2PRIV: privileged access mode for SAI2
0: unprivileged
1: privileged
Note: This bit is only available on some devices in the STM32U5 series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and kept at reset value.
Bit 7 SAI1PRIV: privileged access mode for SAI1
0: unprivileged
1: privileged

RM0456 Rev 6

<!-- pagebreak -->

275

Global TrustZone controller (GTZC)

RM0456

Bit 6 TIM17PRIV: privileged access mode for TIM17
0: unprivileged
1: privileged
Bit 5 TIM16PRIV: privileged access mode for TIM16
0: unprivileged
1: privileged
Bit 4 TIM15PRIV: privileged access mode for TIM15
0: unprivileged
1: privileged
Bit 3 USART1PRIV: privileged access mode for USART1
0: unprivileged
1: privileged
Bit 2 TIM8PRIV: privileged access mode for TIM8
0: unprivileged
1: privileged
Bit 1 SPI1PRIV: privileged access mode for SPI1PRIV
0: unprivileged
1: privileged
Bit 0 TIM1PRIV: privileged access mode for TIM1
0: unprivileged
1: privileged

5.6.7

GTZC1 TZSC privilege configuration register 3
(GTZC1_TZSC_PRIVCFGR3)
Address offset: 0x028
Reset value: 0x0000 0000
Write-privileged access only.
This register can be read or written only by secure privileged transaction when
corresponding GTZC1_TZSC_SECCFGR register signal is set to1. If a given SEC bit is not
set, the equivalent PRIV bit can be read/written by nonsecure privileged transaction.
Read accesses are authorized for any type of transactions, secure or not, privileged or not.

31

30

29

Res.

Res.

Res.

15

14

13

28

27

rw

rw

25

24

23

21

20

19

18

17

16

SDMM OCTOS
C1PRI PIMPRI
V
V

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

rw

rw

OTGP
RIV
rw

DCAC
ICACH
DCMIP ADC12 HE1_R
DMA2D TSCPR
E_REG
RIV
PRIV EGPRI
PRIV
IV
PRIV
V
rw

rw

rw

Bits 31:29 Reserved, must be kept at reset value.

<!-- pagebreak -->

22

12

SAESP PKAPR RNGP HASHP AESPR
RIV
IV
RIV
RIV
IV
rw

26

DCAC
HSPI1_ GFXM GFXM
RAMC OCTOS OCTOS FSMC_ SDMM
JPEGP HE2_R
GPU2D
REGP MU_RE MUPRI
FGPRI PI2_RE PI1_RE REGP C2PRI
RIV
EGPRI
PRIV
RIV
GPRIV
V
V
GPRIV GPRIV
RIV
V
V

RM0456 Rev 6

rw

rw

rw

CRCP FMACP CORDI MDF1P
RIV
RIV
CPRIV
RIV
rw

rw

rw

rw

RM0456

Global TrustZone controller (GTZC)

Bit 28 JPEGPRIV: privileged access mode for JPEG
0: unprivileged
1: privileged
Note: This bit is only available on some devices in the STM32U5 series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and kept at reset value.
Bit 27 DCACHE2_REGPRIV: privileged access mode for DCACHE2 registers
0: unprivileged
1: privileged
Note: This bit is only available on some devices in the STM32U5 series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and kept at reset value.
Bit 26 HSPI1_REGPRIV: privileged access mode for HSPI1 registers
0: unprivileged
1: privileged
Note: This bit is only available on some devices in the STM32U5 series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and kept at reset value.
Bit 25 GFXMMU_REGPRIV: privileged access mode for GFXMMU registers
0: unprivileged
1: privileged
Note: This bit is only available on some devices in the STM32U5 series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and kept at reset value.
Bit 24 GFXMMUPRIV: privileged access mode for GFXMMU
0: unprivileged
1: privileged
Note: This bit is only available on some devices in the STM32U5 series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and kept at reset value.
Bit 23 GPU2DPRIV: privileged access mode for GPU2D
0: unprivileged
1: privileged
Note: This bit is only available on some devices in the STM32U5 series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and kept at reset value.
Bit 22 RAMCFGPRIV: privileged access mode for RAMCFG
0: unprivileged
1: privileged
Bit 21 OCTOSPI2_REGPRIV: privileged access mode for OCTOSPI2
0: unprivileged
1: privileged
Note: This bit is only available on some devices in the STM32U5 series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and kept at reset value.
Bit 20 OCTOSPI1_REGPRIV: privileged access mode for OCTOSPI1
0: unprivileged
1: privileged

RM0456 Rev 6

<!-- pagebreak -->

275

Global TrustZone controller (GTZC)

RM0456

Bit 19 FSMC_REGPRIV: privileged access mode for FSMC registers
0: unprivileged
1: privileged
Note: This bit is only available on some devices in the STM32U5 series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and kept at reset value.
Bit 18 SDMMC2PRIV: privileged access mode for SDMMC1
0: unprivileged
1: privileged
Note: This bit is only available on some devices in the STM32U5 series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and kept at reset value.
Bit 17 SDMMC1PRIV: privileged access mode for SDMMC2
0: unprivileged
1: privileged
Bit 16 OCTOSPIMPRIV: privileged access mode for OCTOSPIM
0: unprivileged
1: privileged
Note: This bit is only available on some devices in the STM32U5 series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and kept at reset value.
Bit 15 SAESPRIV: privileged access mode for SAES
0: unprivileged
1: privileged
Note: This bit is only available on some devices in the STM32U5 series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and kept at reset value.
Bit 14 PKAPRIV: privileged access mode for PKA
0: unprivileged
1: privileged
Note: This bit is only available on some devices in the STM32U5 series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and kept at reset value.
Bit 13 RNGPRIV: privileged access mode for RNG
0: unprivileged
1: privileged
Bit 12 HASHPRIV: privileged access mode for HASH
0: unprivileged
1: privileged
Bit 11 AESPRIV: privileged access mode for AES
0: unprivileged
1: privileged
Note: This bit is only available on some devices in the STM32U5 series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and kept at reset value.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Global TrustZone controller (GTZC)

Bit 10 OTGPRIV: privileged access mode for OTG_FS or OTG_HS
0: unprivileged
1: privileged
Note: This bit is only available on some devices in the STM32U5 series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and kept at reset value.
Bit 9 DCMIPRIV: privileged access mode for DCMI and PSSI
0: unprivileged
1: privileged
Bit 8 ADC12PRIV: privileged access mode for ADC1 and ADC2
0: unprivileged
1: privileged
Bit 7 DCACHE1_REGPRIV: privileged access mode for DCACHE1 registers
0: unprivileged
1: privileged
Bit 6 ICACHE_REGPRIV: privileged access mode for ICACHE registers
0: unprivileged
1: privileged
Bit 5 DMA2DPRIV: privileged access mode for register of DMA2D
0: unprivileged
1: privileged
Note: This bit is only available on some devices in the STM32U5 series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and kept at reset value.
Bit 4 TSCPRIV: privileged access mode for TSC
0: unprivileged
1: privileged
Bit 3 CRCPRIV: privileged access mode for CRC
0: unprivileged
1: privileged
Bit 2 FMACPRIV: privileged access mode for FMAC
0: unprivileged
1: privileged
Bit 1 CORDICPRIV: privileged access mode for CORDIC
0: unprivileged
1: privileged
Bit 0 MDF1PRIV: privileged access mode for MDF1
0: unprivileged
1: privileged

RM0456 Rev 6

<!-- pagebreak -->

