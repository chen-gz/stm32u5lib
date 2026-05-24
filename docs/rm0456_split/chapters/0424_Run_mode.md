482

Power control (PWR)

10.7.3

RM0456

Run mode
Slowing down system clocks
In Run mode, the speed of the system clocks (SYSCLK, HCLK, PCLK) can be reduced by
programming the prescaler registers. These prescalers can also be used to slow down the
peripherals before entering the Sleep mode.
For more details, refer to Section 11: Reset and clock control (RCC).

Peripheral clock gating
In Run mode, the HCLK and PCLK for individual peripherals and memories can be stopped
at any time to reduce the power consumption.
To further reduce the power consumption in Sleep mode, the peripheral clocks can be
disabled prior to executing the WFI or WFE instructions.
The peripheral clock gating is controlled by the RCC_AHBxENR and RCC_APBxENR
registers.
Disabling the peripherals clocks in Sleep mode can be performed automatically by resetting
the corresponding bit in the RCC_AHBxSMENR and RCC_APBxSMENR registers. This bit
must be set for the peripherals requesting clocks in Stop mode for autonomous DMA
transfers or to generate a wake-up interrupt.
Disabling the peripherals autonomous clock in Stop 2 mode can be performed automatically
by resetting the corresponding bit in RCC_AHB3SMENR and RCC_APB3SMENR.

10.7.4

Low-power modes
Entering into a low-power mode
The MCU enters in low-power modes by executing the WFI (wait for interrupt), or WFE
except for Stop 3 mode (wait for event) instructions, or when the SLEEPONEXIT bit in the
Cortex-M33 system control register is set on Return from ISR.
Entering into a low-power mode through WFI or WFE is executed only if no interrupt is
pending or no event is pending.

Caution:

The peripherals with autonomous mode feature are able to generate an AHB or APB clock
request, depending on their internal events. If a clock request is present when WFI or WFE
is executed, the low-power mode entry is delayed until the clock request is released.

Exiting a low-power mode
The way the MCU exits Sleep or Stop mode depends on the way the low-power mode was
entered:
•
If the WFI instruction or Return from ISR was used to enter the low-power mode, any
peripheral interrupt acknowledged by the NVIC can wake up the device.

<!-- pagebreak -->

