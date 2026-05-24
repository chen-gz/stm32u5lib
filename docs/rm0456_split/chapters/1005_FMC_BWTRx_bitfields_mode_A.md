Bit number

Bit name

Value to set

31:30

DATAHLD

Duration of the data hold phase (DATAHLD HCLK cycles for read
accesses).

29:28

ACCMOD

0x0

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

Duration of the second access phase (DATAST HCLK cycles) for read
accesses.

7:4

ADDHLD

Don’t care

3:0

ADDSET

Duration of the first access phase (ADDSET HCLK cycles) for read
accesses.
Minimum value for ADDSET is 0.

RM0456 Rev 6

RM0456

Flexible static memory controller (FSMC)
Table 228. FMC_BWTRx bitfields (mode A)
Bit number

Bit name

Value to set

31:30

DATAHLD

Duration of the data hold phase (DATAHLD+1 HCLK cycles for write
accesses).

29:28

ACCMOD

0x0

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

Duration of the second access phase (DATAST HCLK cycles) for write
accesses.

7:4

ADDHLD

Don’t care

3:0

ADDSET

Duration of the first access phase (ADDSET HCLK cycles) for write
accesses.
Minimum value for ADDSET is 0.

Mode 2/B - NOR flash
Figure 125. Mode 2 and mode B read access waveforms
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
MSv41678V1

RM0456 Rev 6

<!-- pagebreak -->

1045

Flexible static memory controller (FSMC)

RM0456

Figure 126. Mode 2 write access waveforms
Memory transaction
A[25:0]

NADV

NEx

NOE

NWE

Data bus

Data driven by controller

ADDSET HCLK cycles

DATAST HCLK cycles

DATAHLD +1
HCLK cycles
MSv41679V1

Figure 127. Mode B write access waveforms
Memory transaction
A[25:0]

NADV

NEx

NOE

NWE

Data bus

Data driven by controller

ADDSET HCLK cycles

DATAST HCLK cycles

DATAHLD +1
HCLK cycles
MSv41680V1

The differences with mode 1 are the toggling of NWE and the independent read and write
timings when extended mode is set (mode B).

<!-- pagebreak -->

