1045

Flexible static memory controller (FSMC)

RM0456

Table 231. FMC_BWTRx bitfields (mode 2/B)

Note:

Bit number

Bit name

Value to set

31:30

DATAHLD

Duration of the data hold phase (DATAHLD+1 HCLK cycles for write
accesses).

29:28

ACCMOD

0x1 if Extended mode is set

27:24

DATLAT

Don’t care

23:20

CLKDIV

Don’t care

19:16

BUSTURN

Time between NEx high to NEx low (BUSTURN HCLK).

15:8

DATAST

Duration of the access second phase (DATAST HCLK cycles) for
write accesses.

7:4

ADDHLD

Don’t care

3:0

ADDSET

Duration of the access first phase (ADDSET HCLK cycles) for write
accesses. Minimum value for ADDSET is 0.

The FMC_BWTRx register is valid only if the Extended mode is set (mode B), otherwise its
content is don’t care.

Mode C - NOR flash - OE toggling
Figure 128. Mode C read access waveforms
Memory transaction
A[25:0]

NADV

NEx

NOE

NWE

High

D[15:0]

Data driven by memory

ADDSET HCLK cycles

DATAST HCLK cycles

DATAHLD
HCLK cycles
MSv41682V1

<!-- pagebreak -->

