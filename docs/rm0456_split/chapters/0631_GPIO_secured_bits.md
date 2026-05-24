RM0456 Rev 6

RM0456

General-purpose I/Os (GPIO)
After reset, all GPIO ports are secure.
Table 128 gives a summary of the I/O port secured bits following the security configuration
bit in the GPIO_SECCFGR register. When the I/O bit port is configured as secure:
•

Secured bits: read and write operations are only allowed by a secure access. Non
secure-read or write accesses on secured bits are RAZ/WI. There is no illegal access
event generated.

•

Nonsecure bits: no restriction. Read and write operations are allowed by both secure
and nonsecure accesses.

When the TrustZone security is disabled (TZEN = 0 in FLASH_OPTR register), all registers
bits are nonsecure. The GPIOx_SECCFGR register is RAZ/WI.
Table 128. GPIO secured bits
Secure configuration bit

SECy = 1
in GPIOx_SECCFGR(1)

Secured bit

Register name

MODEy[1:0]

GPIOx_MODER

OTy

GPIOx_OTYPER

OSPEEDy[1:0]

GPIOx_OSPEEDR

PUPDy[1:0]

GPIOx_PUPDR

IDy

GPIOx_IDR

ODy

GPIOx_ODR

BSy and BRy

GPIOx_BSRR

LCKy

GPIOx_LCKR

BRy

GPIOx_BRR

Nonsecure access on
secure bits

RAZ/WI

GPIOx_AFRH

AFSELy[3:0]

GPIOx_AFRL

HSLVy

GPIOx_HSLVR

1. The number of GPIOx ports varies in the STM32U5 Series devices. Refer to the product datasheet for availability of
a particular port. If not present, consider the associated bits as reserved and keep them at the reset value.

13.3.19

Privileged and unprivileged modes
All GPIO registers can be read and written by privileged and unprivileged accesses,
whatever the security state (secure or nonsecure).

13.3.20

High-speed low-voltage mode (HSLV)
Some I/Os have the capability to increase their maximum speed at low voltage by
configuring them in HSLV mode. The I/O HSLV bit controls whether the I/O output speed is
optimized to operate at 3.3 V (default setting) or at 1.8 V (HSLV = 1).

Caution:

The I/O HSLV configuration bit must not be set if the I/O supply (VDD or VDDIO2) is above
2.7 V. Setting it while the voltage is higher than 2.7 V can damage the device. The I/O HSLV
bit can be set only when the corresponding option bit is activated (IO_VDD_HSLV or
IO_VDDIO2_HSLV depending on the I/O supply, refer to Section 7.4: FLASH option bytes).

RM0456 Rev 6

<!-- pagebreak -->

642

General-purpose I/Os (GPIO)

RM0456

There is no hardware protection associated to this feature so it is recommended to use it
only as a static configuration for fixed I/O supply.
Caution:

Setting this bit when the I/O is configured in Fm+ mode is forbidden. An I/O is in Fm+ mode
when it is configured as I2C alternate function, with FMP=1 in I2C_CR1 register. PB6, PB7,
PB6, PB9 can also be in Fm+ mode when PB6_FMP, PB7_FMP, PB8_FMP, PB9_FMP,
respectively, is set in SYSCFG_CFGR1 register.

Caution:

On STM32U59x/5Ax/5Fx/5Gx devices, HSLVy bits in GPIOx_HLSVR must be programmed
with the same value within the following pairs: PF4/PF5, PI3/PI4, or PI6/PI7. Each of these
pairs can be used as a differential clock for the OCTOSPIs or HSPI, but this caution applies
even if these pairs are used for other purposes.

13.3.21

I/O compensation cell
The I/O commutation slew rate (tfall/trise) can be adapted by software depending on process,
voltage, and temperatures conditions, in order to reduce the I/O noise on power supply.
Refer to Section 15: System configuration controller (SYSCFG) for more details.

<!-- pagebreak -->

