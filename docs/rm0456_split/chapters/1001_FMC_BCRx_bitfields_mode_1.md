RM0456 Rev 6

RM0456

Flexible static memory controller (FSMC)
Figure 122. Mode 1 write access waveforms
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

The DATAHLD time at the end of the read and write transactions guarantee the address and
data hold time after the NOE/NWE rising edge. The DATAST value must be greater than
zero (DATAST > 0).
Table 224. FMC_BCRx bitfields (mode 1)
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

0x0

13

WAITEN

0x0 (no effect in Asynchronous mode)

12

WREN

As needed

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

Don’t care

Set to 1 if the memory supports this feature. Otherwise keep at 0.

RM0456 Rev 6

<!-- pagebreak -->

