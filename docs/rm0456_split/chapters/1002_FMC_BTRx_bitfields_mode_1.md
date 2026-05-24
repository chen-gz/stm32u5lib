1045

Flexible static memory controller (FSMC)

RM0456

Table 224. FMC_BCRx bitfields (mode 1) (continued)
Bit number

Bit name

Value to set

5:4

MWID

As needed

3:2

MTYP

As needed, exclude 0x2 (NOR flash memory)

1

MUXE

0x0

0

MBKEN

0x1

Table 225. FMC_BTRx bitfields (mode 1)

<!-- pagebreak -->

Bit number

Bit name

Value to set

31:30

DATAHLD

Duration of the data hold phase (DATAHLD HCLK cycles for read
accesses, DATAHLD+1 HCLK cycles for write accesses).

29:28

ACCMOD

Don’t care

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

Duration of the second access phase (DATAST HCLK cycles).

7:4

ADDHLD

Don’t care

3:0

ADDSET

Duration of the first access phase (ADDSET HCLK cycles).
Minimum value for ADDSET is 0.

RM0456 Rev 6

RM0456

Flexible static memory controller (FSMC)

Mode A - SRAM/FRAM/PSRAM (CRAM) OE toggling
Figure 123. Mode A read access waveforms
Memory transaction
A[25:0]

NBL[x:0]

NEx

NOE

NWE

High

Data bus

Data driven by memory

NBLSET
HCLK
cycles

ADDSET HCLK cycles

DATAST HCLK cycles

DATAHLD
HCLK cycles
MSv41681V1

1. NBL[1:0] are driven low during the read access

Figure 124. Mode A write access waveforms
Memory transaction
A[25:0]

NBL[x:0]

NEx

NOE

NWE

Data bus

Data driven by controller

NBLSET
HCLK
cycles

ADDSET HCLK cycles

DATAST HCLK cycles

DATAHLD +1
HCLK cycles
MSv41665V1

The differences compared with Mode 1 are the toggling of NOE and the independent read
and write timings.

RM0456 Rev 6

<!-- pagebreak -->

