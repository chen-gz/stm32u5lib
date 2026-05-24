1500

Comparator (COMP)

RM0456
Figure 322. Scaler

VREF_COMP

VREFINT

+
3/4 VREF_COMP
1/2 VREF_COMP
1/4 VREF_COMP

INMSEL < 3
MSv63603V1

37.5

COMP low-power modes
Table 361. Comparator behavior in the low-power modes
Mode
Sleep

No effect on the comparators.
Comparator interrupts cause the device to exit Sleep mode.

Stop

No effect on the comparators.
Comparator interrupts cause the device to exit Stop mode.

Standby

37.6

Description

The COMP registers are powered down and must be reinitialized after exiting
Standby mode.

COMP interrupts
The comparator outputs are internally connected to the extended interrupts and events
controller (EXTI). Each comparator has its own EXTI line and can generate either interrupts
or events. The same mechanism is used to exit the low-power modes.
Refer to Section 23: Extended interrupts and event controller (EXTI) for more details.
To enable the COMPx interrupt, follow this sequence:

<!-- pagebreak -->

