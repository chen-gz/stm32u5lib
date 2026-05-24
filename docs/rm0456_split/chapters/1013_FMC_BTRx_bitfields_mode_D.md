Bit number

Bit name

Value to set

31

FMCEN

0x1

30:24

Reserved

0x000

23:22

NBLSET[1:0]

As needed

20

CCLKEN

As needed

19

CBURSTRW

0x0 (no effect in Asynchronous mode)

18:16

CPSIZE

0x0 (no effect in Asynchronous mode)

15

ASYNCWAIT

14

EXTMOD

0x1

13

WAITEN

0x0 (no effect in Asynchronous mode)

12

WREN

As needed

11

WAITCFG

Don’t care

10

Reserved

0x0

9

WAITPOL

Meaningful only if bit 15 is 1

8

BURSTEN

0x0

Set to 1 if the memory supports this feature. Otherwise keep at 0.

RM0456 Rev 6

RM0456

Flexible static memory controller (FSMC)
Table 235. FMC_BCRx bitfields (mode D) (continued)
Bit number

Bit name

Value to set

7

Reserved

0x1

6

FACCEN

Set according to memory support

5:4

MWID

As needed

3:2

MTYP

As needed

1

MUXEN

0x0

0

MBKEN

0x1

Table 236. FMC_BTRx bitfields (mode D)
Bit number

Bit name

Value to set

31:30

DATAHLD

Duration of the data hold phase (DATAHLD HCLK cycles for read
accesses).

29:28

ACCMOD

0x3

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

Duration of the middle phase of the read access (ADDHLD HCLK
cycles)

3:0

ADDSET

Duration of the first access phase (ADDSET HCLK cycles) for read
accesses. Minimum value for ADDSET is 1.

Table 237. FMC_BWTRx bitfields (mode D)
Bit number

Bit name

Value to set

31:30

DATAHLD

Duration of the data hold phase (DATAHLD+1 HCLK cycles for write
accesses).

29:28

ACCMOD

0x3

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

Duration of the middle phase of the write access (ADDHLD HCLK
cycles)

3:0

ADDSET

Duration of the first access phase (ADDSET HCLK cycles) for write
accesses. Minimum value for ADDSET is 1.

RM0456 Rev 6

<!-- pagebreak -->

1045

Flexible static memory controller (FSMC)

RM0456

Muxed mode - multiplexed asynchronous access to NOR flash memory
Figure 132. Muxed read access waveforms
Memory transaction
A[25:16]

NADV

NBL[x:0]

NEx

NOE

NWE

High

Lower address

AD[15:0]

NBLSET
HCLK
cycles

ADDSET HCLK cycles

Data driven by memory

ADDHLD
HCLK
cycles

DATAST HCLK cycles

DATAHLD
HCLK cycles
MSv41685V1

<!-- pagebreak -->

