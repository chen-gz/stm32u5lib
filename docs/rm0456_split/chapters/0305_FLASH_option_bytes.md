RM0456 Rev 6

RM0456

Embedded flash memory (FLASH)
The software must check the status of the flash memory and take corrective actions.
This must be done after each system reset before any other programming or erase
operation is performed.
The table below describes the corrective action to be taken according to the status provided
by CODE_OP field in FLASH_OPSR.
Table 56. Flash operation interrupted by a system reset

CODE_OP

Operation
interrupted

0x0

No operation

Reserved

0x1

Single write

ADDR_OP BK_OP

SYSF_OP Page erase and single write at same location

0x2

Burst write

ADDR_OP BK_OP

SYSF_OP Page erase and burst write at same location

0x3

Page erase

ADDR_OP BK_OP

Reserved

Erase same page

0x4

Bank erase

Reserved

Reserved

Erase same bank

0x5

Mass erase

Reserved

Mass erase

0x6

Option change

Reserved

Option change

Address

Bank

BK_OP

0x7

Note:

System
FLASH

Corrective action
None

Reserved

For single and burst write, it is mandatory to perform a page erase because the current flash
memory locations may no longer be writable. Consequently, the remaining page content
must be saved before page erase and restored afterwards.
For OTP write, it is not possible to perform a page erase. The OTP quad-word is lost.
For burst write, ADDR_OP gives the first address of burst. User must restart the same burst
operation.
For page erase, ADDR_OP gives the first address of erased page.

7.4

FLASH option bytes

7.4.1

Option bytes description
The option bytes are configured by the end user depending on the application requirements.
As a configuration example, the watchdog may be selected in hardware or software mode
(refer to Section 7.4.2). The user option bytes are accessible through the flash memory
registers.
Table 57 describes the organization of all user option bytes available in flash memory
registers.

RM0456 Rev 6

<!-- pagebreak -->

