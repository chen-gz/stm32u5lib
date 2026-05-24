RM0456 Rev 6

RM0456

13.3.2

General-purpose I/Os (GPIO)

I/O pin alternate function multiplexer and mapping
The device I/O pins are connected to on-board peripherals/modules through a multiplexer
that allows only one peripheral alternate function (AF) connected to an I/O pin at a time. In
this way, there is no conflict between peripherals available on the same I/O pin.
Each I/O pin has a multiplexer with up to 16 alternate function inputs (AF0 to AF15) that can
be configured through the GPIOx_AFRL (for pin 0 to 7) and GPIOx_AFRH (for pin 8 to 15)
registers:
•

After reset, the multiplexer selection is alternate function 0 (AF0). The I/Os are
configured in alternate function mode through GPIOx_MODER register.

•

The specific alternate function assignments for each pin are detailed in the device
datasheet.

In addition to this flexible I/O multiplexing architecture, each peripheral has alternate
functions mapped onto different I/O pins to optimize the number of peripherals available in
smaller packages.
To use an I/O in a given configuration, the user must proceed as follows:
•

Debug function: after each device reset, these pins are assigned as alternate function
pins immediately usable by the debugger host.

•

GPIO: configure the desired I/O as output, input, or analog in GPIOx_MODER.

•

Peripheral alternate function:

•

–

Connect the I/O to the desired AFx in one of GPIOx_AFRL or GPIOx_AFRH.

–

Select the type, pull-up/pull-down, and output speed via GPIOx_OTYPER,
GPIOx_PUPDR, and GPIOx_OSPEEDR respectively.

–

Configure the desired I/O as an alternate function in GPIOx_MODER.

Cortex-M33 alternate function (EVENTOUT):
–

•

The Cortex-M33 output EVENTOUT signal can be used by configuring the I/O to
output at AF15. An event can be signaled through the configuring pin after
executing SEV instruction.

Additional functions:
–

For the ADC, DAC, OPAMP and COMP, configure the desired I/O in analog mode
in GPIOx_MODER, and configure the required function in the ADC, DAC, OPAMP,
and COMP registers.

–

For the additional functions like RTC, WKUPx, and oscillators, configure the
required function in the related RTC, PWR, and RCC registers. These functions
have priority over the configuration in the standard GPIO registers.

Refer to the “Alternate function mapping” table in the device datasheet for the detailed
mapping of the alternate function I/O pins.

13.3.3

I/O port control registers
Each of the GPIO ports has four 32-bit memory-mapped control registers (GPIOx_MODER,
GPIOx_OTYPER, GPIOx_OSPEEDR, GPIOx_PUPDR) to configure up to 16 I/Os:
•

GPIOx_MODER is used to select the I/O mode (input, output, AF, analog).

•

GPIOx_OTYPER and GPIOx_OSPEEDR are used to select the output type (push-pull
or open-drain) and speed.

•

GPIOx_PUPDR is used to select the push-up/pull-down whatever the I/O direction.

RM0456 Rev 6

<!-- pagebreak -->

