275

Global TrustZone controller (GTZC)

RM0456

Bit 8 CSAI2F: clear the illegal access flag for SAI2
0: no action
1: status flag cleared
Note: This bit is only available on some devices in the STM32U5 series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and kept at reset value.
Bit 7 CSAI1F: clear the illegal access flag for SAI1
0: no action
1: status flag cleared
Bit 6 CTIM17F: clear the illegal access flag for TIM7
0: no action
1: status flag cleared
Bit 5 CTIM16F: clear the illegal access flag for TIM6
0: no action
1: status flag cleared
Bit 4 CTIM15F: clear the illegal access flag for TIM5
0: no action
1: status flag cleared
Bit 3 CUSART1F: clear the illegal access flag for USART1
0: no action
1: status flag cleared
Bit 2 CTIM8F: clear the illegal access flag for TIM8
0: no action
1: status flag cleared
Bit 1 CSPI1F: clear the illegal access flag for SPI1
0: no action
1: status flag cleared
Bit 0 CTIM1F: clear the illegal access flag for TIM1
0: no action
1: status flag cleared

5.7.11

GTZC1 TZIC flag clear register 3 (GTZC1_TZIC_FCR3)
Address offset: 0x028
Reset value: 0x0000 0000
Secure privilege access only.

31
Res.

30
Res.

15

14

29
Res.

13

28

27

26

CDCA CHSPI
CJPEG
CHE2_ 1_REG
F
REGF
F

25
CGFX
MMU_
REGF

24

23

22

21

20

19

18

COCT COCT CFSM
CGFX CGPU2 CRAM
CSDM
OSPI2_ OSPI1_ C_REG
MMUF
DF
CFGF
MC2F
REGF REGF
F

<!-- pagebreak -->

w

w

16

CSDM
MC1F

COCT
OSPIM
F

w

w

w

w

w

w

w

w

w

w

w

w

w

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

CDCA CICAC
CSAES
CRNG CHASH
CDCMI CADC1
CDMA2
CFMA
CPKAF
CAESF COTGF
CHE1_ HE_RE
CTSCF CCRCF
F
F
F
F
2F
DF
CF
REGF
GF
w

17

w

w

w

w

w

w

RM0456 Rev 6

w

w

w

w

w

CCOR CMDF1
DICF
F
w

w

RM0456

Global TrustZone controller (GTZC)

Bits 31:29 Reserved, must be kept at reset value.
Bit 28 CJPEGF: clear the illegal access flag for JPEG
0: no action
1: status flag cleared
Note: This bit is only available on some devices in the STM32U5 series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and kept at reset value.
Bit 27 CDCACHE2_REGF: clear the illegal access flag for DCACHE2 registers
0: no action
1: status flag cleared
Note: This bit is only available on some devices in the STM32U5 series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and kept at reset value.
Bit 26 CHSPI1_REGF: clear the illegal access flag for HSPI1 registers
0: no action
1: status flag cleared
Note: This bit is only available on some devices in the STM32U5 series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and kept at reset value.
Bit 25 CGFXMMU_REGF: clear the illegal access flag for GFXMMU registers
0: no action
1: status flag cleared
Note: This bit is only available on some devices in the STM32U5 series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and kept at reset value.
Bit 24 CGFXMMUF: clear the illegal access flag for GFXMMU
0: no action
1: status flag cleared
Note: This bit is only available on some devices in the STM32U5 series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and kept at reset value.
Bit 23 CGPU2DF: clear the illegal access flag for GPU2D
0: no action
1: status flag cleared
Note: This bit is only available on some devices in the STM32U5 series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and kept at reset value.
Bit 22 CRAMCFGF: clear the illegal access flag for RAMCFG
0: no action
1: status flag cleared
Bit 21 COCTOSPI2_REGF: clear the illegal access flag for OCTOSPI2 registers
0: no action
1: status flag cleared
Note: This bit is only available on some devices in the STM32U5 series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and kept at reset value.

RM0456 Rev 6

<!-- pagebreak -->

275

Global TrustZone controller (GTZC)

RM0456

Bit 20 COCTOSPI1_REGF: clear the illegal access flag for OCTOSPI1 registers
0: no action
1: status flag cleared
Bit 19 CFSMC_REGF: clear the illegal access flag for FSMC registers
0: no action
1: status flag cleared
Note: This bit is only available on some devices in the STM32U5 series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and kept at reset value.
Bit 18 CSDMMC2F: clear the illegal access flag for SDMMC1
0: no action
1: status flag cleared
Note: This bit is only available on some devices in the STM32U5 series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and kept at reset value.
Bit 17 CSDMMC1F: clear the illegal access flag for SDMMC2
0: no action
1: status flag cleared
Bit 16 COCTOSPIMF: clear the illegal access flag for OCTOSPIM
0: no action
1: status flag cleared
Note: This bit is only available on some devices in the STM32U5 series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and kept at reset value.
Bit 15 CSAESF: clear the illegal access flag for SAES
0: no action
1: status flag cleared
Note: This bit is only available on some devices in the STM32U5 series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and kept at reset value.
Bit 14 CPKAF: clear the illegal access flag for PKA
0: no action
1: status flag cleared
Note: This bit is only available on some devices in the STM32U5 series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and kept at reset value.
Bit 13 CRNGF: clear the illegal access flag for RNG
0: no action
1: status flag cleared
Bit 12 CHASHF: clear the illegal access flag for HASH
0: no action
1: status flag cleared
Bit 11 CAESF: clear the illegal access flag for AES
0: no action
1: status flag cleared
Note: This bit is only available on some devices in the STM32U5 series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and kept at reset value.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Global TrustZone controller (GTZC)

Bit 10 COTGF: clear the illegal access flag for OTG_FS or OTG_HS
0: no action
1: status flag cleared
Note: This bit is only available on some devices in the STM32U5 series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and kept at reset value.
Bit 9 CDCMIF: clear the illegal access flag for DCMI and PSSI
0: no action
1: status flag cleared
Bit 8 CADC12F: clear the illegal access flag for ADC1 and ADC2
0: no action
1: status flag cleared
Bit 7 CDCACHE1_REGF: clear the illegal access flag for DCACHE1 registers
0: no action
1: status flag cleared
Bit 6 CICACHE_REGF: clear the illegal access flag for ICACHE registers
0: no action
1: status flag cleared
Bit 5 CDMA2DF: clear the illegal access flag for register of DMA2D
0: no action
1: status flag cleared
Note: This bit is only available on some devices in the STM32U5 series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and kept at reset value.
Bit 4 CTSCF: clear the illegal access flag for TSC
0: no action
1: status flag cleared
Bit 3 CCRCF: clear the illegal access flag for CRC
0: no action
1: status flag cleared
Bit 2 CFMACF: clear the illegal access flag for FMAC
0: no action
1: status flag cleared
Bit 1 CCORDICF: clear the illegal access flag for CORDIC
0: no action
1: status flag cleared
Bit 0 CMDF1F: clear the illegal access flag for MDF1
0: no action
1: status flag cleared

RM0456 Rev 6

<!-- pagebreak -->

