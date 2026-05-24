RM0456 Rev 6

RM0456

Global TrustZone controller (GTZC)

Bit 7 IWDGIE: illegal access interrupt enable for IWDG
0: interrupt disabled
1: interrupt enabled
Bit 6 WWDGIE: illegal access interrupt enable for WWDG
0: interrupt disabled
1: interrupt enabled
Bit 5 TIM7IE: illegal access interrupt enable for TIM7
0: interrupt disabled
1: interrupt enabled
Bit 4 TIM6IE: illegal access interrupt enable for TIM6
0: interrupt disabled
1: interrupt enabled
Bit 3 TIM5IE: illegal access interrupt enable for TIM5
0: interrupt disabled
1: interrupt enabled
Bit 2 TIM4IE: illegal access interrupt enable for TIM4
0: interrupt disabled
1: interrupt enabled
Bit 1 TIM3IE: illegal access interrupt enable for TIM3
0: interrupt disabled
1: interrupt enabled
Bit 0 TIM2IE: illegal access interrupt enable for TIM2
0: interrupt disabled
1: interrupt enabled

5.7.2

GTZC1 TZIC interrupt enable register 2 (GTZC1_TZIC_IER2)
Address offset: 0x004
Reset value: 0x0000 0000
Secure privileged access only.
This register is used to enable interrupt of illegal access.

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
MIE

DSIIE

SAI1IE

TIM17I
E

TIM16I
E

rw

rw

rw

rw

rw

Res.

Res.

Res.

LTDCU
SAI2IE
SBIE
rw

rw

TIM15I USART
TIM8IE SPI1IE TIM1IE
E
1IE
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

Bit 11 GFXTIMIE: illegal access interrupt enable for GFXTIM
0: interrupt disabled
1: interrupt enabled
Note: This bit is only available on some devices in the STM32U5 series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and kept at reset value.
Bit 10 DSIIE: illegal access interrupt enable for DSI
0: interrupt disabled
1: interrupt enabled
Note: This bit is only available on some devices in the STM32U5 series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and kept at reset value.
Bit 9 LTDCUSBIE: illegal access interrupt enable for LTDC or USB
0: interrupt disabled
1: interrupt enabled
Note: This bit controls the LTDC on STM32U59x/5Ax/5Fx/5Gx. It controls the USB
on STM32U535/545. It is reserved on STM32U575/585.
Bit 8 SAI2IE: illegal access interrupt enable for SAI2
0: interrupt disabled
1: interrupt enabled
Note: This bit is only available on some devices in the STM32U5 series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and kept at reset value.
Bit 7 SAI1IE: illegal access interrupt enable for SAI1
0: interrupt disabled
1: interrupt enabled
Bit 6 TIM17IE: illegal access interrupt enable for TIM7
0: interrupt disabled
1: interrupt enabled
Bit 5 TIM16IE: illegal access interrupt enable for TIM6
0: interrupt disabled
1: interrupt enabled
Bit 4 TIM15IE: illegal access interrupt enable for TIM5
0: interrupt disabled
1: interrupt enabled
Bit 3 USART1IE: illegal access interrupt enable for USART1
0: interrupt disabled
1: interrupt enabled
Bit 2 TIM8IE: illegal access interrupt enable for TIM8
0: interrupt disabled
1: interrupt enabled
Bit 1 SPI1IE: illegal access interrupt enable for SPI1
0: interrupt disabled
1: interrupt enabled

<!-- pagebreak -->

