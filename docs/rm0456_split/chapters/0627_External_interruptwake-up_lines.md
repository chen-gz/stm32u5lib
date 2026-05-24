RM0456 Rev 6

RM0456

General-purpose I/Os (GPIO)
common to the alternate function input and alternate function output, a single channel is
selected for the alternate function input/output of a given I/O.
To know which functions are multiplexed on each GPIO pin, refer to the device datasheet.

13.3.8

External interrupt/wake-up lines
All ports have external interrupt capability. To use external interrupt lines, the port can be
configured in input, output, or alternate function mode (the port must not be configured in
analog mode). Refer to Section 23: Extended interrupts and event controller (EXTI).

13.3.9

Input configuration
When the I/O port is programmed as input:
•

The output buffer is disabled.

•

The Schmitt trigger input is activated.

•

The pull-up and pull-down resistors are activated depending on the value
in GPIOx_PUPDR.

•

The data present on the I/O pin are sampled into the input data register every
AHB clock cycle.

•

A read access to the input data register provides the I/O state.

The figure below shows the input configuration of the I/O port bit.
Figure 46. Input floating/pull-up/pull-down configurations

Read/write

on

Input data register
Output data register

Write

Bit set/reset registers

Read

VDDIOx
TTL Schmitt trigger
on/off

Pull
up

on/off

ESD
Pull
down protection

Input driver
Output driver

I/O pin

VSS

VSS

Protection diode
VSS

MSv63602V1

13.3.10

Output configuration
When the I/O port is programmed as output:
•

The output buffer is enabled:
–

Open-drain mode: a 0 in the output register activates the N-MOS whereas a 1 in
the output register leaves the port in Hi-Z (the P-MOS is never activated).

–

Push-pull mode: a 0 in the output register activates the N-MOS whereas a 1 in the
output register activates the P-MOS.

•

The Schmitt trigger input is activated.

•

The pull-up and pull-down resistors are activated depending on the value in the
GPIOx_PUPDR register.

RM0456 Rev 6

<!-- pagebreak -->

642

General-purpose I/Os (GPIO)

RM0456

•

The data present on the I/O pin are sampled into the input data register every AHB
clock cycle.

•

A read access to the input data register gets the I/O state.

•

A read access to the output data register gets the last written value.

The figure below shows the output configuration of the I/O port bit.
Figure 47. Output configuration

Write

Input data register

on
VDDIOx
TTL Schmitt trigger

Read/write

Pull
up

on/off

ESD
Pull
down protection

I/O pin

Output driver

VDDIOx
P- MO S

Output
control

VSS

VSS

Protection diode
VSS

N- MO S

Push-pull or
open-drain

VSS

13.3.11

on/off
Input driver

Output data register

Bit set/reset registers

Read

MSv63641V1

Alternate function configuration
When the I/O port is programmed as the alternate function:
•

The output buffer can be configured in open-drain or push-pull mode.

•

The output buffer is driven by the signals coming from the peripheral (transmitter
enable and data).

•

The Schmitt trigger input is activated.

•

The weak pull-up and pull-down resistors are activated or not depending on the value
in the GPIOx_PUPDR register.

•

The data present on the I/O pin are sampled into the input data register every AHB
clock cycle.

•

A read access to the input data register gets the I/O state.

The figure below shows the alternate function configuration of the I/O port bit.
Figure 48. Alternate function configuration
Alternate function input

Input data register

To on-chip
peripheral

Read/write

Output data register

Write

Bit set/reset registers

Read

on
VDDIOx
TTL Schmitt trigger
Pull
up

on/off

Pull ESD
down protection

Input driver

I/O pin

Output driver

VDDIOx
P-MOS

Output
control

From on-chip Alternate function output
peripheral

VSS

VSS

Protection diode
VSS

N-MOS

VSS

<!-- pagebreak -->

on/off

Push-pull or
open-drain
MSv63642V2

RM0456 Rev 6

RM0456

13.3.12

General-purpose I/Os (GPIO)

Analog configuration
When the I/O port is programmed as analog configuration:
•

The output buffer is disabled.

•

The Schmitt trigger input is deactivated, providing zero consumption for every analog
value of the I/O pin. The output of the Schmitt trigger is forced to a constant value (0).

•

The weak pull-up resistors are disabled by hardware. The weak pull-down is
configurable.

•

Read access to the input data register gets the value 0.

The figure below shows the high-impedance, analog-input configuration of the I/O port bits.
Figure 49. High-impedance analog configuration
To/from on-chip
peripheral

Input data register

Analog

Read/write

Output data register

Write

Bit set/reset registers

Read

off

TTL Schmitt trigger

Input driver
Output driver

I/O pin
Pull
down
VSS

ESD
protection

VSS

Protection diode

VSS

MSv74119V1

13.3.13

Using the HSE or LSE oscillator pins as GPIOs
When the HSE or LSE oscillator is switched off (default state after reset), the related
oscillator pins can be used as normal GPIOs.
When the HSE or LSE oscillator is switched on (by setting the HSEON or LSEON bit in the
RCC_CSR register), the oscillator takes control of its associated pins and the GPIO
configuration of these pins has no effect.
When the oscillator is configured in a user external clock mode, only the pin is reserved for
clock input, and the OSC_OUT or OSC32_OUT pin can still be used as normal GPIO.

13.3.14

Using the GPIO pins in the RTC supply domain
The PC13/PC14/PC15 GPIO functionality is lost when the core supply domain is powered
off (when the device enters Standby mode). In this case, if their GPIO configuration is not
bypassed by the RTC configuration, these pins are set in an analog input mode.
For details about I/O control by the RTC, refer to Section 63.3: RTC functional description.

RM0456 Rev 6

<!-- pagebreak -->

642

General-purpose I/Os (GPIO)

13.3.15

RM0456

Using PH3 as GPIO
PH3 may be used as boot pin (BOOT0) or as a GPIO. Depending on the nSWBOOT0 bit in
the user option byte, PH3 switches from the input mode to the analog input mode:

13.3.16

•

After the option byte loading phase if nSWBOOT0 = 1.

•

After reset if nSWBOOT0 = 0.

Using PA11 and PA12 as GPIOs (STM32U59x/5Ax/5Fx/5Gx only)
PA11 and PA12 provide OTG_HS additional functions. There are constraints to use PA11
and PA12 as standard GPIOs or alternate functions. Refer to Section 10.7.12: USB power
management in low-power modes (STM32U59x/5Ax/5Fx/5Gx only).

13.3.17

OPAMPx_VINM dedicated pins
The OPAMPx_VINM dedicated pins are three-volt tolerant and are supplied by VDDA. These
pins do not feature a complete TT structure as shown in Figure 45, but a direct connection
to OPAMPx. The OPAMPx_VINM dedicated pins are available on specific packages only
(refer to the device datasheet for availability of these pins).

13.3.18

TrustZone security
The TrustZone security is activated by the TZEN option bit in the FLASH_OPTR. When the
TrustZone is active (TZEN = 1), each I/O pin of GPIO port can be individually configured as
secure through the GPIOx_SECCFGR register.
When the selected I/O pin is configured as secure, its corresponding configuration bits for
alternate function, mode selection, I/O data are secure against a nonsecure access. In case
of nonsecure access, these bits are RAZ/WI. The GPIO clock and reset control bits in the
RCC are automatically configured as secure as soon as at least one I/O in the GPIO is
secure.
The I/Os with peripherals functions are also conditioned by the peripheral security
configuration (see Section 5: Global TrustZone controller (GTZC) for more details):
•

For peripherals for which the I/O pin selection is done through alternate functions
registers: if the peripheral is configured as secure, it cannot be connected to a
nonsecure I/O pin. If this is not respected, the input data to the secure peripheral is
forced to 0 (I/O input pin value is ignored) and the output pin value is forced to 0, thus
avoiding any secure information leak through nonsecure I/Os.

•

For I/Os with analog switches, directly controlled by peripherals (such as ADC for
instance): If the I/O is secure, the I/O analog switch cannot be controlled by a
nonsecure peripheral. If this is not respected, the switch remains open. This prevent
the redirection of secure data to a nonsecure peripheral or I/O through analog path.
Refer to Section 3: System security for more details.

•

Some of the paths between I/Os “additional functions” and peripherals are not blocked
if the I/O is secure and the peripheral is nonsecure. Therefore it is recommended to
configure those peripherals as secure even when not used by the application. Refer to
Section 3: System security for the list of concerned peripherals. When the path has a
security control, it follows the same rule as I/O selection through alternate functions.

Refer to the device pins definition table in datasheet for more information about peripherals
alternate functions and additional functions mapping.

<!-- pagebreak -->

