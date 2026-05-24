RM0456 Rev 6

RM0456

72.14

USB on-the-go full-speed (OTG_FS)

OTG_FS control and status registers
By reading from and writing to the control and status registers (CSRs) through the AHB
slave interface, the application controls the OTG_FS controller. These registers are 32 bits
wide, and the addresses are 32-bit block aligned. The OTG_FS registers must be accessed
by words (32 bits).
CSRs are classified as follows:
•

Core global registers

•

Host-mode registers

•

Host global registers

•

Host port CSRs

•

Host channel-specific registers

•

Device-mode registers

•

Device global registers

•

Device endpoint-specific registers

•

Power and clock-gating registers

•

Data FIFO (DFIFO) access registers

Only the core global, power and clock-gating, data FIFO access, and host port control and
status registers can be accessed in both host and device modes. When the OTG_FS
controller is operating in one mode, either device or host, the application must not access
registers from the other mode. If an illegal access occurs, a mode mismatch interrupt is
generated and reflected in the core interrupt register (MMIS bit in the OTG_GINTSTS
register). When the core switches from one mode to the other, the registers in the new mode
of operation must be reprogrammed as they would be after a power-on reset.

72.14.1

CSR memory map
The host and device mode registers occupy different addresses. All registers are
implemented in the AHB clock domain.

Global CSR map
These registers are available in both host and device modes.
Table 753. Core global control and status registers (CSRs)
Acronym

Address
offset

Register name

OTG_GOTGCTL

0x000

Section 72.15.1: OTG control and status register (OTG_GOTGCTL)

OTG_GOTGINT

0x004

Section 72.15.2: OTG interrupt register (OTG_GOTGINT)

OTG_GAHBCFG

0x008

Section 72.15.3: OTG AHB configuration register (OTG_GAHBCFG)

OTG_GUSBCFG

0x00C

Section 72.15.4: OTG USB configuration register (OTG_GUSBCFG)

OTG_GRSTCTL

0x010

Section 72.15.5: OTG reset register (OTG_GRSTCTL)

OTG_GINTSTS

0x014

Section 72.15.6: OTG core interrupt register (OTG_GINTSTS)

OTG_GINTMSK

0x018

Section 72.15.7: OTG interrupt mask register (OTG_GINTMSK)

RM0456 Rev 6

<!-- pagebreak -->

