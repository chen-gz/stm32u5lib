RM0456 Rev 6

RM0456

RAM configuration controller (RAMCFG)
•

All internal SRAMs are erased by hardware in case of readout protection (RDP) level
regression to level 0.5 or level 0. Refer to Section 7: Embedded flash memory (FLASH)
for more details.

•

The SRAM2 is erased when a system reset occurs if the SRAM2_RST option bit is
selected in the flash memory user option bytes. SRAM1/3/4/5/6 are erased when
a system reset occurs if the SRAM_RST option bit is selected in the flash memory user
option bytes. Refer to Section 7: Embedded flash memory (FLASH) for more details.

•

The SRAM2 and optionally the BKPSRAM are protected by the tamper detection
circuit, and are erased by hardware in case of tamper detection. BKPSRAM is also
erased by hardware in case of a backup domain reset. Refer to Section 64: Tamper
and backup registers (TAMP) for more details.

•

The RAMCFG embeds the registers related to the internal SRAMs ECC, write
protection, wait-state configuration, and software erase.

The table below summarizes the features supported by each internal SRAM.
Table 46. Internal SRAMs features
SRAM feature

SRAM1 SRAM2 SRAM3 SRAM4 SRAM5 SRAM6 BKPSRAM

DMA accessibility in Stop 0/1 modes

X

X

X

X

X

X

X

DMA accessibility in Stop 2 mode

-

-

-

X

-

-

-

Optional retention in Standby mode

-

X

-

-

-

-

X

Optional retention in VBAT mode

-

-

-

-

-

-

X

Erased with RDP regression

X

X

X

X

X

X

X

Erased or blocked by tamper detection

-

X

-

-

-

-

X(1)

Optionally erased with system reset

X

X

X

X

X

X

-

Software erase

X

X

X

X

X

X

X

ECC

-

X

X

-

-

-

X

Write protection

-

X

-

-

-

-

-

Wait states

X

X

X

X

X

X

X

1. Optional: BKPSRAM can be configured to be erased or not on tamper detection.

6.3.2

Error code correction (SRAM2, SRAM3, BKPSRAM)
The ECC is supported by SRAM2/3 and BKPSRAM when enabled with the SRAM2_ECC,
SRAM3_ECC, and BKPRAM_ECC user option bits. Refer to Section 7: Embedded flash
memory (FLASH) for more details.
Seven ECC bits are added per 32 bits of SRAM, allowing two bits error detection, and
one bit error correction on memory read access.
As the ECC is calculated and checked for a 32-bit word, the byte and half-word write
accesses are managed by the SRAM interface by first reading the whole word, then write
the word again with the new byte/half-word value. ECC double errors are also detected
during these byte or half-word AHB write accesses (read/modify/write done by interface).
The byte or half-word write access latency is WSC[2:0] + 2 AHB clock cycles
(see Section 6.3.4).

RM0456 Rev 6

<!-- pagebreak -->

290

RAM configuration controller (RAMCFG)
Caution:

RM0456

In case of a byte or half-word write on SRAM with ECC, the read/modify/write operation is
done in a buffer. The buffer content is written into the SRAM two AHB clock cycles after the
SRAM AHB is released (when SRAM is no more accessed).

Single and double ECC errors
When a single error is detected, it is automatically corrected, and SEDC/CSEDC bits are set
in RAMCFG_MxISR and RAMCFG_MxICR respectively. The associated ECC single error
address RAMCFG_MxSEAR is updated only if the single error interrupt is enabled (SEIE bit
of RAMCFG_MxIER is set), and if the ALE bit is set in RAMCFG_MxCR.
Caution:

Single errors cannot be detected when the SEDC bit is set.
When a double error is detected, DED and CDED bits are set in RAMCFG_MxISR and
RAMCFG_MxICR respectively. The associated ECC double error address
RAMCFG_MxDEAR is updated only if the double error interrupt is enabled (DEIE bit of
RAMCFG_MxIER is set) or NMI is enabled by ECCNMI, and if the ALE bit is set in
RAMCFG_MxCR.

Caution:

Double errors cannot be detected when the DED bit is set.

SRAM3 ECC specific management
When the ECC is enabled for SRAM3, only the first 256 Kbytes of SRAM3 are with ECC.
The next 192 Kbytes for STM32U575/585 or 512 Kbytes for STM32U59x/5Ax/5Fx/5Gx are
without ECC, and the last block is used to store the ECC, so cannot be used for application.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

RAM configuration controller (RAMCFG)
The figure below shows the SRAM areas, when SRAM2 and SRAM3 ECC are enabled.
Figure 22. SRAM1, SRAM2 with ECC and SRAM3 with ECC memory map
STM32U535/545/575/585

STM32U59x/5Ax/5Fx/5Gx

SRAM3(1)

64 Kbytes
0x8 0000
0x7 FFFF

64 Kbytes
6 x 64
Kbytes

SRAM3

64 Kbytes
0x11 0000
0x10 FFFF
64 Kbytes

64 Kbytes

SRAM with ECC

64 Kbytes

64 Kbytes

64 Kbytes

64 Kbytes

64 Kbytes

64 Kbytes
56 Kbytes

0x3 0000

8 Kbytes

0x2 FFFF

SRAM1

64 Kbytes

64 Kbytes

64 Kbytes
0x0

SRAM2

SRAM without ECC

SRAM2

0x4 0000
0x3 FFFF

SRAM without ECC

64 Kbytes

64 Kbytes
0x19 0000
0x18 FFFF

SRAM with ECC

64 Kbytes

0x19 FFFF

0x0D 0000
0x0C FFFF

56 Kbytes

0x0C 0000

8 Kbytes

0x0B FFFF

64 Kbytes
10 x 64
Kbytes

SRAM1

64 Kbytes
0x0

1. SRAM3 is not available on STM32U535/545 devices.

SRAM without ECC

0xB 0000
0xA FFFF

Reserved
(ECC
storage)

64 Kbytes

SRAM without ECC

0xB FFFF

Reserved
(ECC
storage)

Address offset

Address offset

MSv65674V2

When ECC is enabled by user option bits, the ECCE bit is automatically set after system
reset in the related RAMCFG_MxCR.
The ECC can be deactivated by executing the following software sequence:
1.

Write 0xAE in RAMCFG_MxECCKEYR.

2.

Write 0x75 in RAMCFG_MxECCKEYR.

3.

Write 0 in the ECCE bit of RAMCFG_MxCR.

In case ECC is deactivated (ECCE = 0), the SRAM3 ECC storage area (from offset
0xB0000 to offset 0xBFFFF) can be read and written as other SRAM3 areas. In order to test
the ECC mechanism, only the first 256 Kbytes of SRAM3 can be modified, 1 or 2 bits
by word (for single or double error test respectively).

RM0456 Rev 6

<!-- pagebreak -->

