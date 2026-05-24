763

Low-power direct memory access controller (LPDMA)

18

Low-power direct memory access controller
(LPDMA)

18.1

LPDMA introduction

RM0456

The low-power direct memory access (LPDMA) controller is a bus master and system
peripheral.
The LPDMA is used to perform programmable data transfers between memory-mapped
peripherals and/or memories via linked-lists, upon the control of an off-loaded CPU.

18.2

LPDMA main features
•

Single bidirectional AHB master

•

Memory-mapped data transfers from a source to a destination:
Peripheral-to-memory

–

Memory-to-peripheral

–

Memory-to-memory

–

Peripheral-to-peripheral

•

Autonomous data transfers during Stop mode (see Section 18.4.17)

•

Transfers arbitration based on a 4-grade programmed priority at channel level:
–

One high-priority traffic class, for time-sensitive channels (queue 3)

–

Three low-priority traffic classes, with a weighted round-robin allocation for non
time-sensitive channels (queues 0, 1, 2)

•

Per channel event generation, on any of the following events: transfer complete, half
transfer complete, data transfer error, user setting error, link transfer error, completed
suspension and trigger overrun

•

Per channel interrupt generation, with separately programmed interrupt enable per
event

•

4 concurrent LPDMA channels:

•

<!-- pagebreak -->

