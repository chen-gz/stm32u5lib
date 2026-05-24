RM0456 Rev 6

RM0456

Extended interrupts and event controller (EXTI)
Figure 101. EXTI block diagram

AHB interface

exti_ilac

Registers

exti[15:0]

EXTImux

GPIO

hclk

IOPort

To interconnect

sys_wake-up

PWR

c_wake-up

Configurable event(15:0)

Peripherals

it_exti_per(y)

Configurable event(y)

Masking

events

Event
trigger

c_evt_exti

Pulse

c_event

c_evt_rst

EVG

Wake-up

c_fclk

EXTI

rxev
nvic(x)

CPU

MSv62642V1

Table 188. EXTI signals
Name

I/O

Description

AHB interface

I/O

EXTI register bus interface. When one event is configured to enable security, the AHB
interface supports secure accesses.

hclk

I

AHB bus clock and EXTI system clock

Configurable
event(y)

I

Asynchronous wake-up events from peripherals that do not have an associated interrupt
and flag in the peripheral

exti_ilac

O

Illegal access event

IOPort(n)

I

GPIOs block IO ports[15:0]

exti[15:0]

O

EXTI GPIO output port to trigger other peripherals

it_exti_per (y)

O

Interrupts to the CPU associated with configurable event (y)

c_evt_exti

O

High-level sensitive event output for CPU, synchronous to hclk

c_evt_rst

I

Asynchronous reset input to clear c_evt_exti

sys_wakeup

O

Asynchronous system wake-up request to PWR for ck_sys and hclk

c_wakeup

O

Wake-up request to PWR for CPU, synchronous to hclk

Table 189. EVG signals
Name

I/O

Description

c_fclk

I

CPU free running clock

c_evt_in

I

High-level sensitive events input from EXTI, asynchronous to CPU clock

c_event

O

Event pulse, synchronous to CPU clock

c_evt_rst

O

Event reset signal, synchronous to CPU clock

RM0456 Rev 6

<!-- pagebreak -->

