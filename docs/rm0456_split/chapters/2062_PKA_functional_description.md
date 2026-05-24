2093

Public key accelerator (PKA)

RM0456

53.3

PKA functional description

53.3.1

PKA block diagram
Figure 508. PKA block diagram
PKA

32-bit AHB bus

Banked registers (main)

PKA_CR

control

PKA_SR

status

AHB
interface

clear

PKA_CLRFR
32-bit access

667x64-bit
PKA RAM

pka_hclk
pka_it

IRQ
interface

Control
logic

PKA core
32-bit

32-bit x 2

pka_itamp_out
MS45419V2

53.3.2

PKA internal signals
Table 493 lists the internal signals available at the PKA level, not necessarily available on
product bonding pads.
Table 493. Internal input/output signals
Signal name

Signal type

pka_hclk

Digital input

AHB bus clock

pka_it

Digital output

Public key accelerator IP global interrupt request

Digital output

PKA internal tamper event signal to TAMP (XOR-ed),
triggered when an unexpected fault occurs while PKA
manipulates secrets, or when the programmed input point is
not found on the input curve (ECDSA signature and ECC
scalar multiplication only).
This signal is asserted as soon as a fault is detected. When
asserted, read access to PKA registers are reset to 0 and
writes are ignored.
The signal is de-asserted when PKA memory is cleared.

pka_itamp_out

53.3.3

Description

PKA reset and clocks
PKA is clocked on the AHB bus clock. When the PKA peripheral reset signal is released
PKA_RAM is cleared automatically, taking 667 clock cycles. During this time the setting of
bit EN in PKA_CR register is ignored. Once EN bit is set, refer to Section 53.3.6 for details
about PKA initialization sequence.
According to the security policy applied to the device, PKA RAM can also be reset following
a tamper event. Refer to Tamper detection and response in the System security section (if
applicable to this product).

<!-- pagebreak -->

