RM0456 Rev 6

RM0456

Global TrustZone controller (GTZC)

5.7.7

GTZC1 TZIC status register 3 (GTZC1_TZIC_SR3)
Address offset: 0x018
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

DCAC
GFXM
OCTOS OCTOS
HSPI1_
GFXM GPU2D RAMC
FSMC_ SDMM
JPEGF HE2_R
MU_RE
PI2_RE PI1_RE
REGF
MUF
F
FGF
REGF
C2F
EGF
GF
GF
GF

17

16

SDMM OCTOS
C1F
PIMF

Res.

Res.

Res.

r

r

r

r

r

r

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

8

7

6

5

4

3

2

1

0

SAESF

PKAF

RNGF

HASHF

AESF

OTGF

DCMIF

TSCF

CRCF

FMACF

r

r

r

r

r

r

r

r

r

r

DCAC ICACH
ADC12
DMA2D
HE1_R E_REG
F
F
EGF
F
r

r

r

r

CORDI
MDF1F
CF
r

r

Bits 31:29 Reserved, must be kept at reset value.
Bit 28 JPEGF: illegal access flag for JPEG
0: no illegal access event
1: illegal access event
Note: This bit is only available on some devices in the STM32U5 series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and kept at reset value.
Bit 27 DCACHE2_REGF: illegal access flag for DCACHE2 registers
0: no illegal access event
1: illegal access event
Note: This bit is only available on some devices in the STM32U5 series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and kept at reset value.
Bit 26 HSPI1_REGF: illegal access flag for HSPI1 registers
0: no illegal access event
1: illegal access event
Note: This bit is only available on some devices in the STM32U5 series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and kept at reset value.
Bit 25 GFXMMU_REGF: illegal access flag for GFXMMU registers
0: no illegal access event
1: illegal access event
Note: This bit is only available on some devices in the STM32U5 series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and kept at reset value.
Bit 24 GFXMMUF: illegal access flag for GFXMMU
0: no illegal access event
1: illegal access event
Note: This bit is only available on some devices in the STM32U5 series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and kept at reset value.

RM0456 Rev 6

<!-- pagebreak -->

275

Global TrustZone controller (GTZC)

RM0456

Bit 23 GPU2DF: illegal access flag for GPU2D
0: no illegal access event
1: illegal access event
Note: This bit is only available on some devices in the STM32U5 series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and kept at reset value.
Bit 22 RAMCFGF: illegal access flag for RAMCFG
0: no illegal access event
1: illegal access event
Bit 21 OCTOSPI2_REGF: illegal access flag for OCTOSPI2 registers
0: no illegal access event
1: illegal access event
Note: This bit is only available on some devices in the STM32U5 series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and kept at reset value.
Bit 20 OCTOSPI1_REGF: illegal access flag for OCTOSPI1 registers
0: no illegal access event
1: illegal access event
Bit 19 FSMC_REGF: illegal access flag for FSMC registers
0: no illegal access event
1: illegal access event
Note: This bit is only available on some devices in the STM32U5 series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and kept at reset value.
Bit 18 SDMMC2F: illegal access flag for SDMMC1
0: no illegal access event
1: illegal access event
Note: This bit is only available on some devices in the STM32U5 series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and kept at reset value.
Bit 17 SDMMC1F: illegal access flag for SDMMC2
0: no illegal access event
1: illegal access event
Bit 16 OCTOSPIMF: illegal access flag for OCTOSPIM
0: no illegal access event
1: illegal access event
Note: This bit is only available on some devices in the STM32U5 series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and kept at reset value.
Bit 15 SAESF: illegal access flag for SAES
0: no illegal access event
1: illegal access event
Note: This bit is only available on some devices in the STM32U5 series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and kept at reset value.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Global TrustZone controller (GTZC)

Bit 14 PKAF: illegal access flag for PKA
0: no illegal access event
1: illegal access event
Note: This bit is only available on some devices in the STM32U5 series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and kept at reset value.
Bit 13 RNGF: illegal access flag for RNG
0: no illegal access event
1: illegal access event
Bit 12 HASHF: illegal access flag for HASH
0: no illegal access event
1: illegal access event
Bit 11 AESF: illegal access flag for AES
0: no illegal access event
1: illegal access event
Note: This bit is only available on some devices in the STM32U5 series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and kept at reset value.
Bit 10 OTGF: illegal access flag for OTG_FS or OTG_HS
0: no illegal access event
1: illegal access event
Note: This bit is only available on some devices in the STM32U5 series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and kept at reset value.
Bit 9 DCMIF: illegal access flag for DCMI and PSSI
0: no illegal access event
1: illegal access event
Bit 8 ADC12F: illegal access flag for ADC1 and ADC2
0: no illegal access event
1: illegal access event
Bit 7 DCACHE1_REGF: illegal access flag for DCACHE1 registers
0: no illegal access event
1: illegal access event
Bit 6 ICACHE_REGF: illegal access flag for ICACHE registers
0: no illegal access event
1: illegal access event
Bit 5 DMA2DF: illegal access flag for register of DMA2D
0: no illegal access event
1: illegal access event
Note: This bit is only available on some devices in the STM32U5 series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and kept at reset value.
Bit 4 TSCF: illegal access flag for TSC
0: no illegal access event
1: illegal access event

RM0456 Rev 6

<!-- pagebreak -->

