917

Extended interrupts and event controller (EXTI)

23

RM0456

Extended interrupts and event controller (EXTI)
The extended interrupts and event controller (EXTI) manages the individual CPU and
system wake-up through configurable event inputs. It provides wake-up requests to the
power control and generates an interrupt request to the CPU NVIC and events to the CPU
event input. For the CPU, an additional event generation block (EVG) is needed to generate
the CPU event signal.
The EXTI wake-up requests allow the system to be woken up from Stop modes.
The interrupt request and event request generation can be used also in Run modes.
The EXTI also includes the EXTI mux IO port selection.

23.1

EXTI main features
The EXTI main features are the following:
•

26 input events supported

•

All event inputs allow the possibility to wake up the system.

•

Events that do not have an associated wake-up flag in the peripheral, have a flag in the
EXTI, and generate an interrupt to the CPU from the EXTI.

•

Events can be used to generate a CPU wake-up event.

The configurable events have the following features:

23.2

•

Selectable active trigger edge

•

Interrupt pending status register bits independent for the rising and falling edge

•

Individual interrupt and event generation mask, used for conditioning the CPU
wake-up, interrupt, and event generation

•

Software trigger possibility

•

Secure events: The access to control and configuration bits of secure input events can
be made secure and or privilege.

•

EXTI IO port selection

EXTI block diagram
The EXTI consists of a register block accessed via an AHB interface, the event input trigger
block, the masking block, and the EXTI mux as shown in Figure 101.
The register block contains all the EXTI registers.
The event input trigger block provides event input edge trigger logic.
The masking block provides the event input distribution to the different wake-up, interrupt
and event outputs, and their masking.
The EXTI mux provides the IO port selection on to the EXTI event signal.

<!-- pagebreak -->

