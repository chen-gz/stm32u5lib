275

Global TrustZone controller (GTZC)

RM0456

5.4

GTZC functional description

5.4.1

GTZC block diagram
The figure below describes the combined feature of TZSC, MPCBB and TZIC. Each
sub-block is controlled by its own AHB configuration port.
The TZSC defines which peripheral is secure and/or privileged.The privileged configuration
bit of a peripheral can be modified by a secure privileged transaction when the peripheral is
configured as secure. Otherwise, a privileged transaction (nonsecure) is sufficient.
On the opposite, the secure configuration bit of a peripheral can be modified only with a
secure privileged transaction if the peripheral is configured as privileged. Otherwise, a
secure transaction (unprivileged) is sufficient.
The secure configuration bit of a given ram block can be modified only with a secure
privileged transaction if the same RAM block is configured as privileged. Otherwise, a
secure transaction (unprivileged) is sufficient.
The TZIC gathers illegal events generated within the system when an illegal access is
detected. TZIC can then generate a secure interrupt towards the CPU if needed.
Figure 19. GTZC block diagram
(from option bytes)
TZEN
AHB

GTZC

TZSC

Secure/non-secure

SECCFGR
PRIVCFGR

Privileged/unprivileged

MPCWMxzCFGR
MPCWMxzR

Secure/non-secure
Privileged/unprivileged

To peripherals

To external memories
and backup SRAM

TZSC_ILA_event

MPCBB
AHB

CFGLOCKR
SECCFGR
PRIVCFGR

Secure/non-secure
Privileged/unprivileged

To internal SRAMs

MPCBB_ILA_event

TZIC
AHB

GTZC
(global ILA interrupt
to NVIC)

IER
SR
FCR
TZIC_ILA_event

N x ILA_event
(from peripherals)

<!-- pagebreak -->

