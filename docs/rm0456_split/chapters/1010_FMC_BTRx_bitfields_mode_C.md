1045

Flexible static memory controller (FSMC)

RM0456

Table 232. FMC_BCRx bitfields (mode C) (continued)
Bit number

Bit name

Value to set

5:4

MWID

As needed

3:2

MTYP

0x02 (NOR flash memory)

1

MUXEN

0x0

0

MBKEN

0x1

Table 233. FMC_BTRx bitfields (mode C)
Bit number

Bit name

Value to set

31:30

DATAHLD

Duration of the data hold phase (DATAHLD HCLK cycles for read
accesses).

29:28

ACCMOD

0x2

27:24

DATLAT

0x0

23:20

CLKDIV

0x0

19:16

BUSTURN

Time between NEx high to NEx low (BUSTURN HCLK).

15:8

DATAST

Duration of the second access phase (DATAST HCLK cycles) for
read accesses.

7:4

ADDHLD

Don’t care

3:0

ADDSET

Duration of the first access phase (ADDSET HCLK cycles) for read
accesses. Minimum value for ADDSET is 0.

Table 234. FMC_BWTRx bitfields (mode C)

<!-- pagebreak -->

Bit number

Bit name

Value to set

31:30

DATAHLD

Duration of the data hold phase (DATAHLD+1 HCLK cycles for write
accesses).

29:28

ACCMOD

0x2

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

Duration of the second access phase (DATAST HCLK cycles) for
write accesses.

7:4

ADDHLD

Don’t care

3:0

ADDSET

Duration of the first access phase (ADDSET HCLK cycles) for write
accesses. Minimum value for ADDSET is 0.

RM0456 Rev 6

RM0456

Flexible static memory controller (FSMC)

Mode D - asynchronous access with extended address
Figure 130. Mode D read access waveforms
Memory transaction
A[25:0]

NADV

NBL[x:0]

NEx

NOE

NWE

High

Data driven by memory

Data bus

NBLSET
HCLK
cycles

ADDSET HCLK cycles

ADDHLD
HCLK
cycles

DATAST HCLK cycles

DATAHLD
HCLK cycles
MSv41683V1

RM0456 Rev 6

<!-- pagebreak -->

