RAMCFG_
M7ERKEYR

RM0456 Rev 6

Refer to Section 2.3 for the register boundary addresses.
0

Reset value
Res.
Res.
Res.
Res.
Res.

Res.
Res.
Res.
Res.
Res.

Reserved
Res.

0

Res.

0

Res.

Reserved

Res.

Res.
Res.
Res.
Res.
Res.
Res.

Res.
Res.
Res.
Res.
Res.

SRAMER

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Reserved

Res.

0

Res.

SRAMBUSY

Res.

Res.

Res.

0

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Reserved
Res.

Reset value

Res.

SRAMER

Res.

Res.

Res.

Res.

Reset value

SRAMBUSY

Res.

Res.

Res.

Res.

Res.

Res.

Res.

0

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Reset value

Res.

Reset value
Res.

Reserved
Res.

0

Res.

0

Res.

Reserved

Res.

WSC[2:0]
Res.

Reserved

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

WSC[2:0]

Res.

0

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

0

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Reserved

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Reserved

Res.

Res.

Reset value

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Reset value

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

0x18C
to
0x1A4

Res.

RAMCFG_M7ISR
Res.

0x184

Res.

RAMCFG_M7CR

Res.

0x188
RAMCFG_
M6ERKEYR

Res.

0x168
Res.

0x14C
to 0x164

Res.

0x180
RAMCFG_M6ISR

Res.

0x144

Res.

0x148
RAMCFG_M6CR

Res.

0x12C
to
0x13C

Res.

0x140
Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

RAMCFG
_M5ERKEYR

Res.

0x128

Res.

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

Offset Register name

Res.

RAM configuration controller (RAMCFG)
RM0456

Table 50. RAMCFG register map and reset values (continued)

ERASEKEY[7:0]

0
0
0
0

0
0
0
0

0
0
0
0

0

0

0

0

0

0

0
0

0

0

Reserved

ERASEKEY[7:0]
0

ERASEKEY[7:0]
0

RM0456

Embedded flash memory (FLASH)

7

Embedded flash memory (FLASH)

7.1

FLASH introduction
The flash memory interface manages accesses to the flash memory, maximizing throughput
to the CPU, instruction cache and DMAs. It implements the flash memory erase and
program operations as well as the read and write protection mechanisms. It also
implements the security and privilege access control features. It is optimized in terms of
power consumption with dedicated modes when the MCU is in low-power modes.

7.2

FLASH main features
•

Up to 4 Mbytes of flash memory supporting read-while-write capability (RWW).

•

Memory organization
–

Dual bank architecture (bank 1 and bank 2)

–

Main memory: up to 2 Mbytes per bank

–

Information block: 64.5 Kbytes in bank 1

•

128-bit wide data read with prefetch

•

Standard and burst programming modes

•

Read, program and erase operations in all voltage ranges

•

10 kcycles endurance on all flash memory. 100 kcycles on up to 256 Kbytes per bank

•

Page erase, bank erase and mass erase (both banks)

•

Bank swapping: the user flash memory address mapping of each bank can be
swapped.

•

Product security activated by TrustZone option bit (TZEN)

•

Device life cycle managed by readout protection option byte (RDP)

•

Four write protection areas (two per bank)

•

TrustZone support:
–

Two secure areas (1 per bank)

–

Two secure HDP (hide protection) areas part of the secure areas (one per bank)

•

Configurable protection against unprivileged accesses with flash page granularity

•

Error code correction: 9-bit ECC per 128-bit quad-word allowing two bits error detection
and one bit error correction

•

Option-byte loader

•

Advanced low-power modes (low-power read mode, bank power-down mode)

RM0456 Rev 6

<!-- pagebreak -->

