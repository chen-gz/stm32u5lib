1045

Flexible static memory controller (FSMC)

RM0456

Figure 131. Mode D write access waveforms
Memory transaction
A[25:0]

NADV

NBL[x:0]

NEx

NOE

NWE

Data driven by controller

Data bus

NBLSET
HCLK
cycles

ADDSET HCLK cycles

ADDHLD
HCLK cycles

DATAST HCLK cycles

DATAHLD +1
HCLK cycles
MSv41684V1

The differences with mode 1 are the toggling of NOE that goes on toggling after NADV
changes and the independent read and write timings.
Table 235. FMC_BCRx bitfields (mode D)

<!-- pagebreak -->

