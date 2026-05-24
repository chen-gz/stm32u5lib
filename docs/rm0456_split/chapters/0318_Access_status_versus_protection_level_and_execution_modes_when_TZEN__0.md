363

Embedded flash memory (FLASH)

RM0456

Table 65. Access status versus protection level and execution modes when TZEN = 0
Area

RDP
level

User execution (boot from flash)

Debug/boot from RAM/ bootloader(1)

Read

Write

Erase

Read

Write

Erase

1

Yes

Yes

Yes

No

No

No(4)

2

Yes

Yes

Yes

N/A

N/A

N/A

1

Yes

No

No

Yes

No

No

2

Yes

No

No

N/A

N/A

N/A

1

Yes

Yes(4)

N/A

Yes

Yes(4)

N/A

2

Yes

No(5)

N/A

N/A

N/A

N/A

1

Yes

Yes(6)

N/A

Yes

Yes(6)

N/A

2

Yes

Yes(6)

N/A

N/A

N/A

N/A

1

Yes

Yes

N/A

No

No

N/A(7)

2

Yes

Yes

N/A

N/A

N/A

N/A

1

Yes

Yes

N/A

No

No

N/A(8)

2

Yes

Yes

N/A

N/A

N/A

N/A

1

Yes

Yes

Yes

No

Yes

Yes(9)

2

Yes

Yes

Yes

N/A

N/A

N/A

Flash main memory

System memory (2)

Option bytes(3)

OTP

Backup registers

SRAM2/backup RAM

OTFDEC regions
(OCTOSPI)

1. When the protection level 2 is active, the debug port, the boot from RAM and the boot from system memory are disabled.
2. The system memory is only read-accessible, whatever the protection level (0, 1 or 2) and execution mode.
3. Option bytes are only accessible through the FLASH registers interface and OPTSTRT bit.
4. The flash main memory is erased when the RDP option byte changes from level 1 to level 0.
5. SWAP_BANK option bit can be modified.
6. OTP can only be written once.
7. The backup registers are erased when RDP changes from level 1 to level 0.
8. All SRAMs are erased when RDP changes from level 1 to level 0.
9. The OTFDEC keys are erased when the RDP option byte changes from level 1 to level 0.

Readout protection levels when TrustZone is enabled
There are four levels of readout protection from no protection (level 0) to maximum
protection or no debug (level 2). The flash memory is protected according to the RDP option
byte value shown in the table below.
Table 66. Flash memory readout protection status (TZEN = 1)

<!-- pagebreak -->

RDP byte value

Readout protection level

0xAA

Level 0

0x55

Level 0.5

RM0456 Rev 6

RM0456

Embedded flash memory (FLASH)
Table 66. Flash memory readout protection status (TZEN = 1) (continued)

•

RDP byte value

Readout protection level

Any value except 0xAA or 0x55 or 0xCC

Level 1

0xCC

Level 2

Level 0: no protection
Read, program and erase operations into the flash main memory area are possible.
The option bytes, the SRAMs and the backup registers are also accessible by all
operations.
–

•

RSS mode: when booting from RSS, the debug access is disabled while
executing RSS code.

Level 0.5: nonsecure debug only
All read and write operations (if no write protection is set) from/to the nonsecure flash
memory are possible. The debug access to secure area is prohibited. Debug access to
nonsecure area remains possible.

•

–

User mode: code executing in user mode (boot flash) can access the flash main
memory, option bytes, SRAMs and backup registers with all operations (read,
erase, program).

–

Nonsecure debug mode: nonsecure debug is possible when the CPU is in
nonsecure state. The secure flash memory, the secure backup registers and
SRAMs area are inaccessible; the nonsecure flash memory, the nonsecure
backup registers and the nonsecure SRAMs area remain accessible for debug
purpose.

–

RSS mode: when booting from RSS, the debug access is disabled while
executing RSS code.

–

Boot RAM mode: boot from SRAM is not possible.

Level 1: readout protection
When the readout protection level 1 is set:
–

User mode: code executing in user mode (boot flash) can access the flash main
memory, option bytes, SRAMs and backup registers with all operations (read,
erase, program).

–

Nonsecure debug mode: nonsecure debug is possible when the CPU is in
nonsecure state. However, an intrusion is detected in case of debug access: the
flash main memory, the backup registers, the backup RAM and the SRAM2 are
totally inaccessible; any read or write access to the flash main memory generates
a bus error and a hard fault interrupt. The on-the-fly decryption region (OTFDEC
on OCTOSPI) is read as zero.

–

RSS mode: when booting from RSS, the debug access is disabled while
executing RSS code.

–

Boot RAM mode: boot from SRAM is not possible.

RM0456 Rev 6

<!-- pagebreak -->

