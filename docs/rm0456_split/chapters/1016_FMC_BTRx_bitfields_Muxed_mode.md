1045

Flexible static memory controller (FSMC)

RM0456

Table 238. FMC_BCRx bitfields (Muxed mode) (continued)
Bit number

Bit name

Value to set

6

FACCEN

0x1

5:4

MWID

As needed

3:2

MTYP

0x2 (NOR flash memory) or 0x1(PSRAM)

1

MUXEN

0x1

0

MBKEN

0x1

Table 239. FMC_BTRx bitfields (Muxed mode)
Bit number

Bit name

Value to set

31:30

DATAHLD

Duration of the data hold phase (DATAHLD HCLK cycles for read
accesses, DATAHLD+1 HCLK cycles for write accesses).

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

Duration of the second access phase (DATAST HCLK cycles).

7:4

ADDHLD

Duration of the middle phase of the access (ADDHLD HCLK cycles).

3:0

ADDSET

Duration of the first access phase (ADDSET HCLK cycles). Minimum
value for ADDSET is 1.

WAIT management in asynchronous accesses
If the asynchronous memory asserts the WAIT signal to indicate that it is not yet ready to
accept or to provide data, the ASYNCWAIT bit has to be set in FMC_BCRx register.
If the WAIT signal is active (high or low depending on the WAITPOL bit), the second access
phase (Data setup phase), programmed by the DATAST bits, is extended until WAIT
becomes inactive. Unlike the data setup phase, the first access phases (Address setup and
Address hold phases), programmed by the ADDSET and ADDHLD bits, are not WAIT
sensitive and so they are not prolonged.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Flexible static memory controller (FSMC)
The data setup phase must be programmed so that WAIT can be detected 4 HCLK cycles
before the end of the memory transaction. The following cases must be considered:
1.

The memory asserts the WAIT signal aligned to NOE/NWE which toggles:
DATAST ≥ ( 4 × HCLK ) + max_wait_assertion_time

2.

The memory asserts the WAIT signal aligned to NEx (or NOE/NWE not toggling):
if
max_wait_assertion_time > address_phase + hold_phase
then:

DATAST ≥ ( 4 × HCLK ) + ( max_wait_assertion_time – address_phase – hold_phase )
otherwise
DATAST ≥ 4 × HCLK
where max_wait_assertion_time is the maximum time taken by the memory to assert
the WAIT signal once NEx/NOE/NWE is low.
Figure 134 and Figure 135 show the number of HCLK clock cycles that are added to the

memory access phase after WAIT is released by the asynchronous memory (independently
of the above cases).
Figure 134. Asynchronous wait during a read access waveforms
Memory transaction

A[25:0]

address phase

data setup phase

NEx

NWAIT

don’t care

don’t care

NOE

data driven by memory

D[15:0]

4HCLK

MS30463V2

1. NWAIT polarity depends on WAITPOL bit setting in FMC_BCRx register.

RM0456 Rev 6

<!-- pagebreak -->

