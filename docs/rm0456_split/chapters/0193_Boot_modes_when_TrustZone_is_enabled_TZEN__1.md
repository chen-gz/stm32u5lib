RM0456 Rev 6

RM0456

Boot modes
Table 26. Boot modes when TrustZone is enabled (TZEN = 1)

BOOT
_LOCK

NBOOT0 BOOT0 NSWBOOT0
RSS
FLASH_
pin
FLASH_
command
OPTR[27]
PH3
OPTR[26]

-

0

1

0

-

1

1

0

0

1

Boot address
option-byte
selection

Boot area

ST
programmed
default value

Secure boot address
Flash
SECBOOTADD0 defined by user option
memory:
[24:0]
bytes
0x0C00 0000
SECBOOTADD0[24:0]
N/A

RSS:
0x0FF8 0000

RSS

Secure boot address
Flash
SECBOOTADD0 defined by user option
memory:
[24:0]
bytes
0x0C00 0000
SECBOOTADD0[24:0]

1

-

0

0

0

-

0

0

N/A

RSS

RSS:
0x0FF8 0000

-

-

-

≠0

N/A

RSS

RSS:
0x0FF8 0000

-

-

-

-

Secure boot address
Flash
SECBOOTADD0 defined by user option
memory:
[24:0]
bytes
0x0C00 0000
SECBOOTADD0[24:0]

The boot address option bytes are used to program any boot memory address. However,
the allowed address space depends on flash memory read protection RDP level.
If the programmed boot memory address is out of the allowed memory mapped area when
RDP level is 0.5 or more, the default boot fetch address is forced either in the secure flash
memory or the nonsecure flash memory depending on the TrustZone security option as
described in the table below.
Table 27. Boot space versus RDP protection
RDP
0

TZEN = 1

TZEN = 0

Any boot address

Any boot address

0.5
1

2

N/A
Boot address only in RSS: 0x0FF80000 or
in secure flash memory:
– 0x0C00 0000-0x0C07 FFFF on STM32U535/545
– 0x0C00 0000-0x0C1F FFFF on STM32U575/585
– 0x0C00 0000-0x0C3F FFFF
on STM32U59x/5Ax/5Fx/5Gx
Otherwise, the forced boot address is 0x0FF8 0000.

Any boot address
Boot address only in flash memory:
– 0x0800 0000-0x0807 FFFF on STM32U535/545
– 0x0800 0000-0x081F FFFF on STM32U575/585
– 0x0800 0000-0x083F FFFF
on STM32U59x/5Ax/5Fx/5Gx
Otherwise, the forced boot address is 0x0800 0000.

The BOOT0 value (either coming from the pin or the option bit) is latched upon reset
release. It is up to the user to set nBOOT0 or BOOT0 values to select the required boot
mode.

RM0456 Rev 6

<!-- pagebreak -->

194

Boot modes

RM0456

The BOOT0 pin or user option bit (depending on NSWBOOT0 in FLASH_OPTR) is also
resampled when exiting Standby mode. Consequently, the BOOT0 pin or user option bit
must be kept in the required boot mode configuration in Standby mode. After the startup
delay, the selection of the boot area is done before releasing the processor reset.
PH3/BOOT0 GPIO is configured as follows:
•

in input mode during the complete reset phase, if NSWBOOT0 is set in FLASH_OPTR,
and then switches automatically in analog mode after reset is released (BOOT0 pin)

•

in input mode from the reset phase to the completion of the option byte loading,
if NSWBOOT0 is cleared in FLASH_OPTR (BOOT0 value coming from the option bit),
and then switches automatically to the analog mode even if the reset phase
is not complete

Embedded bootloader
The embedded bootloader is located in the system memory, programmed by ST during
production. Refer to the application note STM32 microcontroller system memory boot mode
(AN2606).

Embedded root security services (RSS)
The embedded RSS are located in the secure information block, programmed by ST during
production. Refer to the application note STM32 MCUs secure firmware install (SFI)
overview (AN4992).

<!-- pagebreak -->

