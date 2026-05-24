609

Reset and clock control (RCC)

RM0456

The table below shows the list of peripherals with autonomous mode capability.
Table 116. Autonomous peripherals
Domain

Peripheral

Autonomous in
Stop 0, 1 modes

Autonomous in
Stop 2 mode

Associated
DMA

U(S)ARTx (x = 1 to 6)
SPIx (x = 1,2)
CPU domain
(CD)

I2Cx (x = 1,2,4,5,6)
LPTIM2

Yes(1)

No

GPDMA1

MDF1
GPDMA1

-

Associated
SRAM
SRAM1
SRAM2
SRAM3
SRAM4(2)
SRAM5
SRAM6

LPUART1
SPI3
I2C3
SmartRun
domain (SRD)

LPTIMx (x = 1,3,4)
ADF1

Yes (3)

Yes(3)

LPDMA1

SRAM4

DAC1
ADC4
LPDMA1

-

1. Enabled if both xxEN and xxSMEN bits of the peripheral are set (xx = instance name)
2. SRAM4 belongs to SmartRun domain (SRD) but can be addressed by GPDMA 1 in Stop 0 and Stop 1 modes.
3. Enabled if all xxEN, xxSMEN, and xxAMEN bits of the peripheral are set (xx = instance name)

For peripherals in the CPU domain, the autonomous mode is enabled in Stop 0 and Stop 1
modes if both xxEN and xxSMEN bits of the peripheral are set.
For peripherals in SmartRun domain, the autonomous mode is enabled in Stop 0, Stop 1,
and Stop 2 modes if both xxEN and xxSMEN bits of the peripheral are set, plus xxAMEN bit
of the peripheral in RCC_SRDAMR.
If an autonomous peripheral requests its kernel clock in Stop 0, Stop 1, or Stop 2 mode,
the internal oscillator (HSI16 or MSI) is woken up if it was off, and the kernel clock is
propagated only to the peripheral requesting it. When the peripheral releases its kernel
clock request, the HSI16 or MSI is switched off if no other peripheral requests it.
If an autonomous peripheral belonging to CPU domain requests its bus clock (AHB1, AHB2,
APB1, or APB2 clock) in Stop 0 or Stop 1 mode, the internal oscillator (HSI16 or MSI
depending on STOPWUCK value in RCC_CFGR1) is woken up if it was off, and the system
clock is propagated to all peripherals configured with both xxEN = xxSMEN = 1.
If an autonomous peripheral belonging to SmartRun domain requests its bus clock (AHB3 or
APB3 clock) in Stop 0, Stop 1, or Stop 2 mode, the internal oscillator (HSI16 or MSI
depending on STOPWUCK value in RCC_CFGR1) is woken up if it was off, and
HCLK3/PCLK3 clocks are propagated to all peripherals of the SmartRun domain configured
with xxEN = xxSMEN = xxAMEN = 1.

<!-- pagebreak -->

