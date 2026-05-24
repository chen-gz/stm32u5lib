191

System security

3.10.1

RM0456

Life-cycle management with readout protection (RDP)
The readout protection mechanism (full hardware feature) controls the access to the
devices debug, test and provisioned secrets, as summarized in the table below.
Table 21. Typical product life-cycle phases

RDP protection level

Level
Device open
0

Debug

Comments

Boot address must target a secure area when TrustZone is enabled
(secure SRAM, secure flash memory, RSS in system flash memory).
and
nonsecure Both OEM1 and OEM2 unlocking keys can be provisioned in the flash
memory user options. The DHUK in the SAES peripheral is the same for
all devices.

Secure(1)

Boot address must target a secure area when TrustZone is enabled
(secure user or system flash memory). Boot on SRAM is not permitted.
Access to nonsecure flash memory is allowed when debug is connected.
Both OEM1 and OEM2 unlocking keys can be provisioned in the flash
memory user options. The DHUK is different for every device.

Device partially
Level
(2) closed
0.5
(closed-secure)

Nonsecure
only

Device
Level
memories
1
protected

Boot address must target the secure user flash memory.
Nonsecure Accesses to nonsecure flash memory, encrypted flash memory(3),
SRAM2, and backup registers are not allowed when debug is connected.
only
(conditioned) Both OEM1 and OEM2 unlocking keys can be provisioned in the flash
memory user options. The DHUK is different for every device.

Level
Device closed
2

None
(JTAG fuse)

Boot address must target the user flash memory (secure if TZEN = 1).
Option bytes are read-only, hence RDP level 2 cannot be changed, unless
the OEM2 unlocking key is activated (see Table 22). The DHUK is
different for every device.

1. Debug is not available when executing RSS code.
2. Only applicable when TrustZone security is activated in the product.
3. External flash memory area decrypted on-the-fly with the OTFDEC.

Note:

<!-- pagebreak -->

OEM1KEY option byte can be modified when OEM1LOCK = 0 (RDP = 0.5 or 1 only).
OEM2KEY option byte can be modified when OEM2LOCK = 0 (RDP = 1 only).

RM0456 Rev 6

RM0456

System security
The supported transitions, summarized in the figure below, can be requested
(when available) through the debug interface or via the system bootloader.
Figure 15. RDP level transition scheme

TrustZone disabled

Conditioned on
OEM key2 (optional)

TrustZone enabled
RDP
Level1

Conditioned on
OEM key2 (optional)

Conditioned on OEM
key1 (optional),
full flash erase

Conditioned on
OEM key2 (optional),
partial flash erase
(nonsecure only)

RDP
Level 1

RDP
Level 2

Conditioned on
OEM Key1 (optional),
full flash erase

RDP
Level0.5
RDP
Level 0

RDP
Level2

RDP
Level0
MSv66958V2

As shown in the previous figure, the user flash memory is automatically erased, either
partially or in totality, during an RDP regression from RDP1. Those regressions can be
conditioned to dedicated 64-bit password keys, if provisioned by the OEM (see next
subsection). During the regression from RDP level 1 to RDP level 0.5, only nonsecure
embedded flash memory is erased, keeping functional, for example, the secure boot and
the secure firmware update. In all regressions from level 1, the OTP area in the flash
memory is kept, all SRAMS and targeted device secrets are erased. Hence, no secrets
must be stored in the OTP as they are revealed after a regression to RDP level 0. These
secrets, also erased as response to tamper, are defined in Section 3.7.3.
Note:

Enabling TrustZone using the option byte TZEN is only possible when RDP level is 0.
For more details on RDP, refer to Section 7: Embedded flash memory (FLASH).

RDP unlocking sequences
The use of the two OEM password keys described in the last figure is further described
hereafter.
Note:

The devices support both permanent RDP level 2 (legacy mode) or password-based RDP
level 2 regression to level 1. This level 2 regression does not erase the application code,
and it does not change the RDP level 1 protections in place.

RM0456 Rev 6

<!-- pagebreak -->

