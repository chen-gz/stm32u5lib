609

Reset and clock control (RCC)

11.3.2

RM0456

System reset
A system reset sets all registers to their reset values except the reset flags in RCC_CSR,
and the registers in the backup domain.
A system reset is generated when one of the following events occurs:
•

a low level on the NRST pin (external reset)

•

a window watchdog event (WWDG reset)

•

an independent watchdog event (IWDG reset)

•

a software reset (SW reset) (see Software reset)

•

a low-power mode security reset (see Low-power mode security reset)

•

an option-byte loader reset (see Option byte loader reset)

•

a brownout reset

The reset source can be identified by checking the reset flags in RCC_CSR.
These sources act on the NRST pin and this pin is always kept low during the delay phase.
The reset service routine vector is selected via the boot option bytes.
The system reset signal provided to the device is output on the NRST pin. The pulse
generator guarantees a minimum reset pulse duration of 20 µs for each internal reset
source. In case of an external reset, the reset pulse is generated while the NRST pin is
asserted low.
In case on an internal reset, the internal pull-up RPU is deactivated in order to save the
power consumption through the pull-up resistor.
Figure 37. Simplified diagram of the reset circuit
VDD

RPU
System reset
External
reset

Filter
NRST
Pulse
generator
(PLQȝV

WWDG reset
IWDG reset
Software reset
Low-power manager reset
Option-byte loader reset
BOR

MSv69133V1

Software reset
The SYSRESETREQ bit in Cortex-M33 application interrupt and reset control register must
be set to force a software reset on the device.

<!-- pagebreak -->

