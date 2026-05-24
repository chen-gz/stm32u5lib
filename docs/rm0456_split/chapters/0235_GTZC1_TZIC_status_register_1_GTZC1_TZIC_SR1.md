RM0456 Rev 6

RM0456

Global TrustZone controller (GTZC)

Bit 0 GPDMA1IE: illegal access interrupt enable for GPDMA1
0: interrupt disabled
1: interrupt enabled

5.7.5

GTZC1 TZIC status register 1 (GTZC1_TZIC_SR1)
Address offset: 0x010
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

I2C5F

USART
6F

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

I2C6F
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

CRSF

I2C2F

I2C1F

SPI2F

IWDGF

r

r

r

r

r

UART5 UART4 USART USART
F
F
3F
2F
r

r

r

r

WWDG
TIM7F
F
r

r

20
Res.

19

18

17

16

UCPD1 FDCAN LPTIM2
F
1F
F

I2C4F

r

r

r

r

4

3

2

1

0

TIM6F

TIM5F

TIM4F

TIM3F

TIM2F

r

r

r

r

r

Bits 31:24 Reserved, must be kept at reset value.
Bit 23 I2C6F: illegal access flag for I2C6
0: no illegal access event
1: illegal access event
Note: This bit is only available on some devices in the STM32U5 series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and kept at reset value.
Bit 22 I2C5F: illegal access flag for I2C5
0: no illegal access event
1: illegal access event
Note: This bit is only available on some devices in the STM32U5 series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and kept at reset value.
Bit 21 USART6F: illegal access flag for USART6
0: no illegal access event
1: illegal access event
Note: This bit is only available on some devices in the STM32U5 series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and kept at reset value.
Bit 20 Reserved, must be kept at reset value.
Bit 19 UCPD1F: illegal access flag for UCPD1
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

Bit 18 FDCAN1F: illegal access flag for FDCAN1
0: no illegal access event
1: illegal access event
Note: This bit is only available on some devices in the STM32U5 series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and kept at reset value.
Bit 17 LPTIM2F: illegal access flag for LPTIM2
0: no illegal access event
1: illegal access event
Bit 16 I2C4F: illegal access flag for I2C4
0: no illegal access event
1: illegal access event
Bit 15 CRSF: illegal access flag for CRS
0: no illegal access event
1: illegal access event
Bit 14 I2C2F: illegal access flag for I2C2
0: no illegal access event
1: illegal access event
Bit 13 I2C1F: illegal access flag for I2C1
0: no illegal access event
1: illegal access event
Bit 12 UART5F: illegal access flag for UART5
0: no illegal access event
1: illegal access event
Bit 11 UART4F: illegal access flag for UART4
0: no illegal access event
1: illegal access event
Bit 10 USART3F: illegal access flag for USART3
0: no illegal access event
1: illegal access event
Bit 9 USART2F: illegal access flag for USART2
0: no illegal access event
1: illegal access event
Note: This bit is only available on some devices in the STM32U5 series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and kept at reset value.
Bit 8 SPI2F: illegal access flag for SPI2
0: no illegal access event
1: illegal access event
Bit 7 IWDGF: illegal access flag for IWDG
0: no illegal access event
1: illegal access event
Bit 6 WWDGF: illegal access flag for WWDG
0: no illegal access event
1: illegal access event

<!-- pagebreak -->

