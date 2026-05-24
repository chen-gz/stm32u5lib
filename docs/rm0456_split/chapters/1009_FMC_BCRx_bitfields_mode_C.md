RM0456 Rev 6

RM0456

Flexible static memory controller (FSMC)
Figure 129. Mode C write access waveforms
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

The differences compared with mode 1 are the toggling of NOE and the independent read
and write timings.
Table 232. FMC_BCRx bitfields (mode C)
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

Don’t care

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

7

Reserved

0x1

6

FACCEN

0x1

Set to 1 if the memory supports this feature. Otherwise keep at 0.

RM0456 Rev 6

<!-- pagebreak -->

