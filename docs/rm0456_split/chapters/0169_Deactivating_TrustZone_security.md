All SRAMs are secure, as defined in GTZC/MPCBB (see Section 3.5.4). The
secure boot code can change this security setup, making the blocks secure or not.
All memory devices connected to the FSMC and OCTOSPIs are secure, as
defined in GTZC/MPCWM (see Section 3.5.4). The secure-boot code can change
this security setup, making the components secure or not.

•

All GPIOs are secure.

•

All LP/GPDMA channels are nonsecure.

•

Backup registers are nonsecure.

RM0456 Rev 6

RM0456

System security
•

Peripherals and GTZC:
–

Securable peripherals are nonsecure and unprivileged.

–

TrustZone-aware peripherals are nonsecure, with their secure configuration
registers being secure.

–

All illegal access interrupts in GTZC/TZIC are disabled.

Note:

Refer to Section 3.5.4 for the list of securable and TrustZone-aware peripherals.

3.5.7

Deactivating TrustZone security
Once TrustZone is activated, it can only be deactivated during an RDP regression to level 0.

Note:

Such RDP regression triggers the erase of embedded memories (SRAM2, flash memory),
and the reset of all peripherals, including OTFDEC and all cryptographic engines.
After the TrustZone deactivation, most of the features mentioned in Section 3.5 are no
longer available:

Note:

•

The nonvolatile secure area of the embedded flash memory is deactivated, including
the HDP area.

•

The NVIC only manages nonsecure interrupts.

•

All secure registers in TrustZone-aware peripherals are RAZ/WI.

When the TrustZone is deactivated, the resource isolation using privilege stays available
(see Section 3.6.3 for details).
For more information, refer to the application note Arm TrustZone features for STM32L5 and
STM32U5 Series (AN5347).

3.6

Other resource isolations
These are hardware mechanisms offering an additional level of isolation on top of the
TrustZone technology.

3.6.1

Temporal isolation using secure hide protection (HDP)
When the TrustZone security is enabled (TZEN = 1), the embedded flash memory allows an
HDP area per watermarked-secure area of each bank (8-Kbyte page granularity) to be
defined. The code executed in this HDP area, with its related data and keys, can be hidden
after boot until the next system reset.

RM0456 Rev 6

<!-- pagebreak -->

