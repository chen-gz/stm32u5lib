363

Embedded flash memory (FLASH)

7.9.13

RM0456

FLASH option register (FLASH_OPTR)
Address offset: 0x40
Reset value: 0xXXXX XXXX (bits 0 to 31 loaded with values from the flash memory at OBL)
ST production value: 0x1FEF F8AA
Access: no wait state when no option bytes modification is ongoing; word, half-word, and
byte access.
This register is nonsecure. It can be read and written by both secure and nonsecure access.
This register can be protected against unprivileged access when NSPRIV = 1
in FLASH_PRIVCFGR register.

31

30

TZEN

IO_VD
DIO2_
HSLV

29

28

27

IO_VD
PA15_ NBOO
D_HSL
PUPEN
T0
V

26

25

24

23

22

21

20

19

18

17

16

NSWB SRAM2 SRAM2 SRAM3 BKPRA DUALB SWAP_ WWDG IWDG_ IWDG_ IWDG_
OOT0 _RST
_ECC _ECC M_ECC ANK
BANK
_SW STDBY STOP
SW

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

15

14

13

12

11

10

9

8

7

6

5

4

3

2

1

0

rw

rw

rw

SRAM_ NRST_ NRST_ NRST_
RST
SHDW STDBY STOP
rw

rw

rw

rw

Res.

BOR_LEV[2:0]
rw

rw

RDP[7:0]
rw

rw

rw

rw

rw

rw

Bit 31 TZEN: Global TrustZone security enable
0: Global TrustZone security disabled
1: Global TrustZone security enabled
Bit 30 IO_VDDIO2_HSLV: High-speed IO at low VDDIO2 voltage configuration bit
This bit can be set only with VDDIO2 below 2.7 V.
0: High-speed IO at low VDDIO2 voltage feature disabled (VDDIO2 can exceed 2.7 V)
1: High-speed IO at low VDDIO2 voltage feature enabled (VDDIO2 remains below 2.7 V)
Bit 29 IO_VDD_HSLV: High-speed IO at low VDD voltage configuration bit
This bit can be set only with VDD below 2.7V
0: High-speed IO at low VDD voltage feature disabled (VDD can exceed 2.7 V)
1: High-speed IO at low VDD voltage feature enabled (VDD remains below 2.7 V)
Bit 28 PA15_PUPEN: PA15 pull-up enable
0: USB power delivery dead-battery enabled/TDI pull-up deactivated
1: USB power delivery dead-battery disabled/TDI pull-up activated
Bit 27 NBOOT0: NBOOT0 option bit
0: NBOOT0 = 0
1: NBOOT0 = 1
Bit 26 NSWBOOT0: Software BOOT0
0: BOOT0 taken from the option bit NBOOT0
1: BOOT0 taken from PH3/BOOT0 pin
Bit 25 SRAM2_RST: SRAM2 erase when system reset
0: SRAM2 erased when a system reset occurs
1: SRAM2 not erased when a system reset occurs
Bit 24 SRAM2_ECC: SRAM2 ECC detection and correction enable
0: SRAM2 ECC check enabled
1: SRAM2 ECC check disabled

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Embedded flash memory (FLASH)

Bit 23 SRAM3_ECC: SRAM3 ECC detection and correction enable
0: SRAM3 ECC check enabled
1: SRAM3 ECC check disabled
Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and keep it at reset value.
Bit 22 BKPRAM_ECC: Backup RAM ECC detection and correction enable
0: Backup RAM ECC check enabled
1: Backup RAM ECC check disabled
Bit 21 DUALBANK: Dual-bank configuration
– 2-Mbyte flash memory devices for STM32U59x/5Ax/5Fx/5Gx
– 1-Mbyte flash memory devices for STM32U575/585
– 256-Kbyte and 128-Kbyte flash memory devices for STM32U535/545
0: Single-bank flash memory with contiguous address in bank 1
1: Dual-bank flash memory with contiguous addresses
Bit 20 SWAP_BANK: Swap banks
0: Bank 1 and bank 2 addresses not swapped
1: Bank 1 and bank 2 addresses swapped
Bit 19 WWDG_SW: Window watchdog selection
0: Hardware window watchdog selected
1: Software window watchdog selected
Bit 18 IWDG_STDBY: Independent watchdog counter freeze in Standby mode
0: Independent watchdog counter frozen in Standby mode
1: Independent watchdog counter running in Standby mode
Bit 17 IWDG_STOP: Independent watchdog counter freeze in Stop mode
0: Independent watchdog counter frozen in Stop mode
1: Independent watchdog counter running in Stop mode
Bit 16 IWDG_SW: Independent watchdog selection
0: Hardware independent watchdog selected
1: Software independent watchdog selected
Bit 15 SRAM_RST: All SRAMs (except SRAM2 and BKPSRAM) erase upon system reset
0: All SRAMs (except SRAM2 and BKPSRAM) erased when a system reset occurs
1: All SRAMs (except SRAM2 and BKPSRAM) not erased when a system reset occurs
Bit 14 NRST_SHDW: Reset generation in Shutdown mode
0: Reset generated when entering the Shutdown mode
1: No reset generated when entering the Shutdown mode
Bit 13 NRST_STDBY: Reset generation in Standby mode
0: Reset generated when entering the Standby mode
1: No reset generate when entering the Standby mode
Bit 12 NRST_STOP: Reset generation in Stop mode
0: Reset generated when entering the Stop mode
1: No reset generated when entering the Stop mode
Bit 11 Reserved, must be kept at reset value.

RM0456 Rev 6

<!-- pagebreak -->

363

Embedded flash memory (FLASH)

RM0456

Bits 10:8 BOR_LEV[2:0]: BOR reset level
These bits contain the VDD supply level threshold that activates/releases the reset.
000: BOR level 0 (reset level threshold around 1.7 V)
001: BOR level 1 (reset level threshold around 2.0 V)
010: BOR level 2 (reset level threshold around 2.2 V)
011: BOR level 3 (reset level threshold around 2.5 V)
100: BOR level 4 (reset level threshold around 2.8 V)
Bits 7:0 RDP[7:0]: Readout protection level
0xAA: Level 0 (readout protection not active)
0x55: Level 0.5 (readout protection not active, only nonsecure debug access is possible).
Only available when TrustZone is active (TZEN = 1)
0xCC: Level 2 (chip readout protection active)
Others: Level 1 (memories readout protection active)
Note: Refer to Section 7.6.2 for more details.

7.9.14

FLASH nonsecure boot address 0 register
(FLASH_NSBOOTADD0R)
Address offset: 0x44
Reset value: 0xXXXX XXXX
(Option bytes loaded with values from the flash memory at reset release)
ST production value: 0x0800 007F
Access: no wait state when no option bytes modification is ongoing; word, half-word, and
byte access.
This register can not be written if OPTLOCK bit is set. This register is nonsecure. It can be
read and written by both secure and nonsecure access. This register can be protected
against unprivileged access when NSPRIV = 1 in FLASH_PRIVCFGR register.

31

30

29

28

27

26

25

24

23

22

21

20

19

18

17

16

NSBOOTADD0[24:9]
rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

15

14

13

12

11

10

9

8

7

6

5

4

3

2

1

0

Res.

Res.

Res.

Res.

Res.

Res.

Res.

rw

rw

rw

rw

rw

rw

rw

NSBOOTADD0[8:0]
rw

rw

Bits 31:7 NSBOOTADD0[24:0]: Nonsecure boot base address 0
The nonsecure boot memory address can be programmed to any address in the valid
address range with a granularity of 128 bytes. These bits correspond to address [31:7].
NSBOOTADD0 option bytes are selected following the BOOT0 pin or NSWBOOT0 state.
Examples:
NSBOOTADD0[24:0] = 0x0100000: Boot from nonsecure flash memory (0x0800 0000)
NSBOOTADD0[24:0] = 0x017F200: Boot from system memory bootloader (0x0BF9 0000)
NSBOOTADD0[24:0] = 0x0400000: Boot from nonsecure SRAM1 on S-Bus (0x2000 0000)
Bits 6:0 Reserved, must be kept at reset value.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Embedded flash memory (FLASH)

7.9.15

FLASH nonsecure boot address 1 register
(FLASH_NSBOOTADD1R)
Address offset: 0x48
Reset value: 0xXXXX XXXX
(option bytes loaded with values from the flash memory at reset release)
ST production value: 0x0BF9 007F
Access: no wait state when no option bytes modification is ongoing; word, half-word, and
byte access.
This register can not be written if OPTLOCK bit is set.This register is nonsecure. It can be
read and written by both secure and nonsecure access. This register can be protected
against unprivileged access when NSPRIV = 1 in FLASH_PRIVCFGR register.

31

30

29

28

27

26

25

24

23

22

21

20

19

18

17

16

NSBOOTADD1[24:9]
rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

15

14

13

12

11

10

9

8

7

6

5

4

3

2

1

0

Res.

Res.

Res.

Res.

Res.

Res.

Res.

rw

rw

rw

rw

rw

rw

rw

NSBOOTADD1[8:0]
rw

rw

Bits 31:7 NSBOOTADD1[24:0]: Nonsecure boot address 1
The nonsecure boot memory address can be programmed to any address in the valid
address range with a granularity of 128 bytes. These bits correspond to address [31:7].
NSBOOTADD1 option bytes are selected following the BOOT0 pin or NSWBOOT0 state.
Examples:
NSBOOTADD1[24:0] = 0x0100000: Boot from nonsecure flash memory (0x0800 0000)
NSBOOTADD1[24:0] = 0x017F200: Boot from system memory bootloader (0x0BF9 0000)
NSBOOTADD1[24:0] = 0x0400000: Boot from nonsecure SRAM1 on S-Bus (0x2000 0000)
Bits 6:0 Reserved, must be kept at reset value.

RM0456 Rev 6

<!-- pagebreak -->

