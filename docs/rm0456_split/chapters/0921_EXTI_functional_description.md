RM0456 Rev 6

RM0456

23.3

Extended interrupts and event controller (EXTI)

EXTI functional description
The events features are controlled from register bits as follows:
•

23.3.1

Active trigger edge enabled
–

by rising edge selection in EXTI_RTSR1

–

by falling edge selection in EXTI_FTSR1

•

Software trigger in EXTI_SWIER1

•

Interrupt pending flag in EXTI_RPR1 and EXTI_FPR1

•

CPU wake-up and interrupt enable in EXTI_IMR1

•

CPU wake-up and event enable in EXTI_IMR1

EXTI configurable event input wake-up
The figure below is a detailed representation of the logic associated with configurable event
inputs that wake up the CPU subsystem bus clocks and generate an EXTI pending flag and
interrupt to the CPU, and/or a CPU wake-up event.
Figure 102. Configurable event trigger logic CPU wake-up

AHB
interface
hclk

Peripheral interface
Falling
trigger
selection
register

Rising
trigger
selection
register

Software
interrupt
event
register

CPU
event
mask
register

CPU
interrupt
mask
register

Pending
request
register
it_exti_per(y)

Configurable
event
input(y)

Asynchronous edge
detection circuit
rst

Delay

hclk

EVG

Rising edge
detect pulse
generator

ck_fclk_c

hclk
(1)

Rising
edge
detectrst

CPU event(y)

Other CPU events(x,y)

c_evt_rst
c_evt_exti

CPU
rising edge
detect pulse
generator

c_event

Other CPU wake-ups
CPU wake-up(y)

Synch

hclk

Other wake-ups
wake-up(y)

EXTI

c_wake-up

sys_wake-up

MSv62643V1

1. Only for the input events that support CPU rxev generation c_event.

The software interrupt event register allows configurable events to be triggered by software,
writing the corresponding register bit, whatever the edge selection setting.
The configurable event active trigger edge (or both edges) is selected and enabled in the
rising/falling edge selection registers.
The CPU has its dedicated wake-up (interrupt) mask register and a dedicated event mask
registers. When the event is enabled, it is generated to the CPU. All events for the CPU are
ORed together into a single CPU event signal. The event pending registers (EXTI_RPR and
EXTI_FPR) are not set for an unmasked CPU event.
The configurable events have unique interrupt pending request registers. The pending
register is only set for an unmasked interrupt. Each configurable event provides a common
interrupt to the CPU. The configurable event interrupts must be acknowledged by software
in the EXTI_RPR and/or EXTI_FPR registers.

RM0456 Rev 6

<!-- pagebreak -->

