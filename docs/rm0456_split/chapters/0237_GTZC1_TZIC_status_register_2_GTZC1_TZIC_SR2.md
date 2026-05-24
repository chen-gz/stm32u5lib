RM0456 Rev 6

RM0456

Global TrustZone controller (GTZC)

Bit 5 TIM7F: illegal access flag for TIM7
0: no illegal access event
1: illegal access event
Bit 4 TIM6F: illegal access flag for TIM6
0: no illegal access event
1: illegal access event
Bit 3 TIM5F: illegal access flag for TIM5
0: no illegal access event
1: illegal access event
Bit 2 TIM4F: illegal access flag for TIM4
0: no illegal access event
1: illegal access event
Bit 1 TIM3F: illegal access flag for TIM3
0: no illegal access event
1: illegal access event
Bit 0 TIM2F: illegal access flag for TIM2
0: no illegal access event
1: illegal access event

5.7.6

GTZC1 TZIC status register 2 (GTZC1_TZIC_SR2)
Address offset: 0x014
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

GFXTI
MF

DSIF

LTDCU
SBF

SAI2F

SAI1F

USART
1F

TIM8F

SPI1F

TIM1F

r

r

r

r

r

r

r

r

r

TIM17F TIM16F TIM15F
r

r

r

Bits 31:12 Reserved, must be kept at reset value.
Bit 11 GFXTIMF: illegal access flag for GFXTIM
0: no illegal access event
1: illegal access event
Note: This bit is only available on some devices in the STM32U5 series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and kept at reset value.
Bit 10 DSIF: illegal access flag for DSI
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

Bit 9 LTDCUSBF: illegal access flag for LTDC or USB
0: no illegal access event
1: illegal access event
Note: This bit flags the LTDC on STM32U59x/5Ax/5Fx/5Gx. It flags the USB
on STM32U535/545. It is reserved on STM32U575/585.
Bit 8 SAI2F: illegal access flag for SAI2
0: no illegal access event
1: illegal access event
Note: This bit is only available on some devices in the STM32U5 series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and kept at reset value.
Bit 7 SAI1F: illegal access flag for SAI1
0: no illegal access event
1: illegal access event
Bit 6 TIM17F: illegal access flag for TIM7
0: no illegal access event
1: illegal access event
Bit 5 TIM16F: illegal access flag for TIM6
0: no illegal access event
1: illegal access event
Bit 4 TIM15F: illegal access flag for TIM5
0: no illegal access event
1: illegal access event
Bit 3 USART1F: illegal access flag for USART1
0: no illegal access event
1: illegal access event
Bit 2 TIM8F: illegal access flag for TIM8
0: no illegal access event
1: illegal access event
Bit 1 SPI1F: illegal access flag for SPI1
0: no illegal access event
1: illegal access event
Bit 0 TIM1F: illegal access flag for TIM1
0: no illegal access event
1: illegal access event

<!-- pagebreak -->

