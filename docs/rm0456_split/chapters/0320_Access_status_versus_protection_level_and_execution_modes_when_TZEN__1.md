363

Embedded flash memory (FLASH)
•

RM0456

Level 2: no debug
When the readout protection level 2 is set:
–

The protection level 1 is guaranteed.

–

All debug features are disabled
. if OEM2 key has not been provided, JTAG and SWD are definitively disabled.
. if OEM2 key has been provided under a lower RDP protection, JTAG and SWD
remain enabled under reset only to interface with DBGMCU_SR,
DBGMCU_DBG_AUTH_HOST and DBGMCU_DBG_AUTH_DEVICE registers to
obtain device identification and provide OEM2 key to request RDP regression.

Note:

–

The boot from SRAM (boot RAM mode) and the boot from system memory (boot
loader mode) are no longer available.

–

Boot from RSS is possible.

–

When booting from main flash or RSS, all operations are allowed on the flash
main memory. Read, erase and program accesses to flash memory and SRAMs
from user code are allowed.

–

Option bytes cannot be programmed nor erased except the SWAP_BANK option
bit. Thus, the level 2 cannot be removed: it is an irreversible operation unless
an OEM2 key has been provisioned (refer to OEM2 RDP lock mechanism).

The debug feature is also disabled under reset.
STMicroelectronics is not able to perform analysis on defective parts on which the level 2
protection has been set. Regress parts to RDP level 1 before returning them for analysis
(refer to OEM2 RDP lock mechanism).
Table 67. Access status versus protection level and execution modes when TZEN = 1
Area

Flash main memory

System memory (3)

Option bytes(4)

OTP

<!-- pagebreak -->

Debug/bootloader(1)

RDP
level

User execution (boot from flash)
Read

Write

Erase

Read

Write

Erase

0.5

Yes

Yes

Yes

Yes(2)

Yes(2)

Yes(2)

1

Yes

Yes

Yes

No

No

No(6)

2

Yes

Yes

Yes

N/A

N/A

N/A

0.5

Yes

No

No

Yes

No

No

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

0.5

Yes

Yes (6)

N/A

Yes

Yes(5)(6)

N/A

1

Yes

Yes(6)

N/A

Yes

Yes(5)(6)

N/A

2

Yes

No(7)

N/A

N/A

N/A

N/A

0.5

Yes

Yes(8)

N/A

Yes

Yes(8)

N/A

1

Yes

Yes(8)

N/A

Yes

Yes(8)

N/A

2

Yes

Yes(8)

N/A

N/A

N/A

N/A

RM0456 Rev 6

RM0456

Embedded flash memory (FLASH)

Table 67. Access status versus protection level and execution modes when TZEN = 1 (continued)
Area

Backup registers

SRAM2/backup RAM

OTFDEC regions
(OCTOSPI)

Debug/bootloader(1)

RDP
level

User execution (boot from flash)
Read

Write

Erase

Read

Write

Erase

0.5

Yes

Yes

N/A

Yes(2)

Yes(2)

N/A(9)

1

Yes

Yes

N/A

No

No

N/A(9)

2

Yes

Yes

N/A

N/A

N/A

N/A

0.5

Yes

Yes

N/A

Yes(2)

Yes(2)

N/A(10)

1

Yes

Yes

N/A

No

No

N/A(10)

2

Yes

Yes

N/A

N/A

N/A

N/A

0.5

Yes

Yes

Yes

No

Yes

Yes(11)

1

Yes

Yes

Yes

No

Yes

Yes(11)

2

Yes

Yes

Yes

N/A

N/A

N/A

1. When the protection level 2 is active, the debug port and the bootloader mode are disabled.
2. Depends on TrustZone security access rights.
3. The system memory is only read-accessible, whatever the protection level (0, 1 or 2) and execution mode.
4. Option bytes are only accessible through the flash registers interface and OPTSTRT bit.
5. The bootloader can only modify nonsecure option bytes.
6. The flash main memory is erased when the RDP option byte regresses from level 1 to level 0.
7. SWAP_BANK option bit can be modified.
8. OTP can only be written once.
9. The backup registers are erased when RDP changes from level 1 to level 0 and when RDP changes from level 1 to
level 0.5.
10. All SRAMs are erased when RDP changes from level 1 to level 0 and when RDP changes from level 1 to level 0.5.
11. The OTFDEC keys are erased when the RDP option byte changes from level 1 to level 0 and when RDP changes from
level 1 to level 0.5.

Device life cycle managed by readout protection (RDP) transitions
It is easy to move from level 0 or level 0.5 to level 1 by changing the value of the RDP byte
to any value (except 0xCC). By programming the 0xCC value in the RDP byte, it is possible
to go to level 2 either directly from level 0 or from level 0.5 or from level 1. Once in level 2, it
is no longer possible to modify the readout protection level unless an OEM2 key is
provisioned (refer to OEM2 RDP lock mechanism).
When the RDP is reprogrammed to the value 0xAA to move from level 1 to level 0, a mass
erase of the flash main memory and all SRAMs is performed. The backup registers, the
OTFDEC keys, ICACHE, DCACHE, and PKA SRAM are also erased. The OTP area is not
erased.
At RDP level 0.5, it is not possible to request RDP level 0. Instead, a RDP increase to
level 1 followed by a RDP regression to level 0 is required.
When the RDP is programmed to the value 0x55 to move from level 1 to level 0.5, a partial
mass erase of the flash main memory is performed. Only nonsecure watermark-based

RM0456 Rev 6

<!-- pagebreak -->

363

Embedded flash memory (FLASH)

RM0456

areas are erased (even if it is defined as secure by block-based). The backup registers, the
OTFDEC keys, ICACHE, DCACHE, PKA SRAM, and all SRAMs are mass erased.
The OTP area is not erased. The RDP level 0.5 and partial nonsecure erase are only
available when TrustZone is active.
Note:

Full mass erase is performed only when level 1 is active and level 0 requested. When the
protection level is increased (0 to 0.5, 0 to 1, 0.5 to 1, 1 to 2, 0 to 2 or 0.5 to 2), there is no
mass erase.
To validate the readout protection level change, the option bytes must be reloaded through
the OBL_LAUNCH bit in FLASH nonsecure control register (FLASH_NSCR).
Before launching a RDP regression, the software must invalidate the ICACHE and wait for
the BUSYF bit to get cleared.
Figure 25. RDP level transition scheme when TrustZone is disabled (TZEN = 0)
Write
RDP ;$$DQG
RDP [&&
Level 1
RDP 0xAA
RDP 0xCC
Write
RDP = 0xCC

2(0

Level 2
RDP = 0xCC

2(0

Write
RDP ;$$DQG
RDP [&&

Write
RDP = 0xCC

Write
RDP = 0xAA
Mass erase

Level 0
RDP = 0xAA
Write
RDP = 0xAA

5'3LQFUHDVHRSWLRQE\WHVPRGLILFDWLRQ
5'3UHJUHVVLRQ IXOOPDVVHUDVH ±FDQEHGRQHE\GHEXJLQWHUIDFHRUE\
ERRWORDGHU5HJUHVVLRQFDQEHEORFNHGE\2(0NH\
5HJUHVVLRQQRWSRVVLEOHE\GHIDXOW±FDQEHDOORZHGE\GHEXJLQWHUIDFHZLWK
2(0NH\LISUHYLRXVO\SURYLVLRQHGLQORZHU5'3OHYHOV
5'3XQFKDQJHGRSWLRQE\WHVPRGLILFDWLRQ
5'3XQFKDQJHG2QO\6:$3B%$1.RSWLRQELWFDQEHPRGLILHG

<!-- pagebreak -->

RM0456 Rev 6

MSv62609V3

RM0456

Embedded flash memory (FLASH)
Figure 26. RDP level transition scheme when TrustZone is enabled (TZEN = 1)
Write
RDP ;$$DQG
RDP [DQG
RDP [&&
2(0

Level 1
RDP 0xAA
RDP 0x
RDP 0xCC

2(0
Write
RDP = 0xAA
Mass erase

2(0
Level 0.5
RDP = 0x

Level 2
RDP = 0xCC

Level 0
RDP = 0xAA
Write
RDP = 0xCC

5'3LQFUHDVHRSWLRQE\WHVPRGLILFDWLRQ
5'3UHJUHVVLRQ±FDQRQO\EHGRQHE\GHEXJLQWHUIDFHRUE\
ERRWORDGHU)XOOPDVVHUDVH VHFXUHDQGQRQVHFXUH 
5HJUHVVLRQFDQEHEORFNHGE\2(0NH\
5'3UHJUHVVLRQFDQEHGRQHE\GHEXJLQWHUIDFHE\ERRWORDGHU
RUE\VHFXUHILUPZDUH3DUWLDOPDVVHUDVH QRQVHFXUHRQO\ 
5HJUHVVLRQFDQEHEORFNHGE\2(0NH\

Write
RDP = 0xAA
5HJUHVVLRQQRWSRVVLEOHE\GHIDXOW±FDQEH
DOORZHGE\GHEXJLQWHUIDFHZLWK2(0NH\LI
SUHYLRXVO\SURYLVLRQHGLQORZHU5'3OHYHOV
5'3XQFKDQJHG2QO\6:$3B%$1.RSWLRQELW
FDQEHPRGLILHG
5'3XQFKDQJHGRSWLRQE\WHVPRGLILFDWLRQ
MSv62610V3

OEM1/OEM2 lock activation
Two 64-bit keys (OEM1KEY and OEM2KEY) can be defined in order to lock the RDP
regression. Each 64-bit key is coded on two registers: FLASH_OEM1KEYR1 (resp.
FLASH_OEM2KEYR1), and FLASH_OEM1KEYR2 (resp. FLASH_OEM2KEYR2).
OEM1KEY and OEM2KEY cannot be read through these registers. They are read as zero.
OEM1KEYcan be modified:
•

in readout protection level 0

•

in readout protection level 0.5 or 1 if OEM1LOCK = 0 in FLASH_NSSR

OEM2KEYcan be modified:
•

in readout protection level 0 or 0.5

•

in readout protection level 1 if OEM2LOCK = 0 in FLASH_NSSR

When attempting to modify FLASH_OEM1KEYR1, FLASH_OEM1KEYR2 (or
FLASH_OEM2KEYR1, FLASH_OEM2KEYR2) without following these rules, the user option
modification is not done, and the OPTWERR bit is set.
In order to activate OEM1 lock mechanism, the following steps are needed:
•

Check that the OEM1LOCK bit is not set or that the readout protection is at level 0.

•

Write a 64-bit key in FLASH_OEM1KEYR1 and FLASH_OEM1KEYR2.

•

Launch option modification by setting the OPTSTRT bit in FLASH_NSCR.

•

Wait for the BSY bit to be cleared and check that OPTWERR is not set.

RM0456 Rev 6

<!-- pagebreak -->

363

Embedded flash memory (FLASH)

RM0456

•

Set the OBL_LAUNCH option bit to start option bytes loading or perform a power-on
reset.

•

Check that OEM1LOCK is set.

In order to activate OEM2 lock mechanism, the following steps are needed:

Note:

•

Check that the OEM2LOCK bit is not set or that the readout protection is at level 0 or
0.5.

•

Write a 64-bit key in FLASH_OEM2KEYR1 and FLASH_OEM2KEYR2.

•

Launch option modification by setting the OPTSTRT bit in FLASH_NSCR.

•

Wait for the BSY bit to be cleared and check that OPTWERR is not set.

•

Set the OBL_LAUNCH option bit to start option bytes loading or perform a power-on
reset.

•

Check that OEM2LOCK is set.

The OEM1KEY and OEM2KEY must not contain only 1 or only 0.

OEM1 RDP lock mechanism
The OEM1 RDP lock mechanism is active when the OEM1LOCK bit is set. It blocks the
RDP level 1 to RDP level 0 regression.
In order to regress from RDP level 1 to RDP level 0, the following unlock sequence must be
applied:
•

Shift OEM1KEY[31:0] then OEM1KEY[63:32] through JTAG or SWD in the
DBGMCU_DBG_AUTH_HOST register.

•

If this key matches the OEM1KEY value, the RDP regression can be launched by
setting the OPTSTRT bit.

•

If the key does not match the OEM1KEY value, the RDP regression and any access to
the flash memory are blocked until a next power-on reset.

Attempting to regress from RDP level 1 to RDP level 0 without following this sequence sets
the OPTWERR option bit and the option bytes remain unchanged.
When the lock mechanism is not activated (OEM1LOCK =0), the regression from RDP level
1 to RDP level 0 is always granted.

OEM2 RDP lock mechanism
The OEM2 RDP lock mechanism is active when the OEM2LOCK bit is set. It allows the
following actions:
•

Block RDP level 1 to RDP level 0.5 regression.

•

Authorize RDP level 2 to RDP level 1 regression.

In order to regress from RDP level 1 to RDP level 0.5, the following unlock sequence must
be applied:

<!-- pagebreak -->

