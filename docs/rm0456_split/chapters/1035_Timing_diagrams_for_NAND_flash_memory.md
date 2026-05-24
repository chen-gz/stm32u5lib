AHB
Memory
Allowed/
data size data size not allowed

Mode

R/W

Asynchronous

R

8

8

Y

-

Asynchronous

W

8

8

Y

-

Asynchronous

R

16

8

Y

Split into 2 FMC accesses

Asynchronous

W

16

8

Y

Split into 2 FMC accesses

Asynchronous

R

32

8

Y

Split into 4 FMC accesses

Asynchronous

W

32

8

Y

Split into 4 FMC accesses

RM0456 Rev 6

Comments

RM0456

Flexible static memory controller (FSMC)
Table 247. Supported memories and transactions (continued)
Device

NAND 16-bit

27.7.3

AHB
Memory
Allowed/
data size data size not allowed

Mode

R/W

Comments

Asynchronous

R

8

16

Y

-

Asynchronous

W

8

16

N

-

Asynchronous

R

16

16

Y

-

Asynchronous

W

16

16

Y

-

Asynchronous

R

32

16

Y

Split into 2 FMC accesses

Asynchronous

W

32

16

Y

Split into 2 FMC accesses

Timing diagrams for NAND flash memory
The NAND flash memory bank is managed through a set of registers:
•

Control register: FMC_PCR

•

Interrupt status register: FMC_SR

•

ECC register: FMC_ECCR

•

Timing register for Common memory space: FMC_PMEM

•

Timing register for Attribute memory space: FMC_PATT

Each timing configuration register contains three parameters used to define number of
HCLK cycles for the three phases of any NAND flash access, plus one parameter that
defines the timing for starting driving the data bus when a write access is performed.
Figure 139 shows the timing parameter definitions for common memory accesses, knowing
that Attribute memory space access timings are similar.
Figure 139. NAND flash controller waveforms for common memory access
HCLK
A[25:0]
NCEx
High

NREG,
NIOW,
NIOR
NWE,
NOE

MEMxSET
+1

MEMxWAIT + 1

MEMxHOLD

(1)
MEMxHIZ + 1

write_data
read_data

Valid

MS33733V3

1. NOE remains high (inactive) during write accesses. NWE remains high (inactive) during read accesses.
2. For write access, the hold phase delay is (MEMHOLD) HCLK cycles and for read access is
(MEMHOLD + 2) HCLK cycles.

RM0456 Rev 6

<!-- pagebreak -->

