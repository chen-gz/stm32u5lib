139

Memory and bus architecture

RM0456

Table 4. Securable peripherals by TZSC (continued)
Bus

APB1

Peripheral

STM32U535/545

STM32U575/585 STM32U59x/5Ax STM32U5Fx/5Gx

I2C6

-

-

X

X

I2C5

-

-

X

X

USART6

-

-

X

X

UCPD1

-

X

X

X

FDCAN1

X

X

X

X(3)

LPTIM2

X

X

X

X

I2C4

X

X

X

X

CRS

X

X

X

X

I2C2

X

X

X

X

I2C1

X

X

X

X

UART5

X

X

X

X

UART4

X

X

X

X

USART3

X

X

X

X

USART2

-

X

X

X

SPI2

X

X

X

X

IWDG

X

X

X

X

WWDG

X

X

X

X

TIM7

X

X

X

X

TIM6

X

X

X

X

TIM5

X

X

X

X

TIM4

X

X

X

X

TIM3

X

X

X

X

TIM2

X

X

X

X

1. ICACHE and DCACHE1 are TrustZone-aware peripherals, regardless if access to their registers is secured by TZSC.
2. Only one COMP on STM32U535/545.
3. FDCAN1 is not present on STM32U5Fx devices.

Table 5. TrustZone aware peripherals
Bus

AHB3

<!-- pagebreak -->

Peripheral

STM32U535/545

STM32U575/585

GTZC2

X

X

X

X

EXTI

X

X

X

X

LPDMA1

X

X

X

X

RCC

X

X

X

X

PWR

X

X

X

X

LPGPIO1

X

X

X

X

RM0456 Rev 6

STM32U59x/5Ax STM32U5Fx/5Gx

RM0456

Memory and bus architecture
Table 5. TrustZone aware peripherals (continued)

Bus

AHB2

AHB1

APB3

Peripheral

STM32U535/545

STM32U575/585

(1)

STM32U59x/5Ax STM32U5Fx/5Gx

OTFDEC1

X

X

X

X

OTFDEC2(1)

-

X

X

X

GPIOJ

-

-

X

X

GPIOI

-

X

X

X

GPIOH

X

X

X

X

GPIOG

X

X

X

X

GPIOF

-

X

X

X

GPIOE

X

X

X

X

GPIOD

X

X

X

X

GPIOC

X

X

X

X

GPIOB

X

X

X

X

GPIOA

X

X

X

X

GTZC1

X

X

X

X

FLASH

X

X

X

X

GPDMA1

X

X

X

X

TAMP

X

X

X

X

RTC

X

X

X

X

SYSCFG

X

X

X

X

1. Always secure when TZEN = 1.

RM0456 Rev 6

<!-- pagebreak -->

