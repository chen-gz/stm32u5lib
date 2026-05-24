Mode

R/W

AHB
data
size

Memory
data size

Allowed/
not
allowed

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

Asynchronous
page

R

-

16

N

Mode is not supported

Synchronous

R

8

16

N

-

Synchronous

R

16

16

Y

-

Synchronous

R

32

16

Y

-

RM0456 Rev 6

RM0456

Flexible static memory controller (FSMC)
Table 223. NOR flash/PSRAM: example of supported memories
and transactions (continued)
Device

PSRAM
(multiplexed
I/Os and nonmultiplexed
I/Os)

SRAM and
ROM

27.6.3

Mode

R/W

AHB
data
size

Memory
data size

Allowed/
not
allowed

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

Y

Use of byte lanes NBL[1:0]

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

Asynchronous
page

R

-

16

N

Mode is not supported

Synchronous

R

8

16

N

-

Synchronous

R

16

16

Y

-

Synchronous

R

32

16

Y

-

Synchronous

W

8

16

Y

Use of byte lanes NBL[1:0]

Synchronous

W

16/32

16

Y

-

Asynchronous

R

8 / 16

16

Y

-

Asynchronous

W

8 / 16

16

Y

Use of byte lanes NBL[1:0]

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
Use of byte lanes NBL[1:0]

General timing rules
Signals synchronization

27.6.4

•

All controller output signals change on the rising edge of the internal clock (HCLK)

•

In Synchronous mode (read or write), all output signals change on the rising edge of
HCLK. Whatever the CLKDIV value, all outputs change as follows:
–

NOEL/NWEL/ NEL/NADVL/ NADVH /NBLL/ Address valid outputs change on the
falling edge of FMC_CLK clock.

–

NOEH/ NWEH / NEH/ NOEH/NBLH/ Address invalid outputs change on the rising
edge of FMC_CLK clock.

NOR flash/PSRAM controller asynchronous transactions
Asynchronous static memories (NOR flash, PSRAM, SRAM, FRAM)
•

Signals are synchronized by the internal clock HCLK. This clock is not issued to the
memory

RM0456 Rev 6

<!-- pagebreak -->

1045

Flexible static memory controller (FSMC)

RM0456

•

The FMC always samples the data before de-asserting the NOE signal. This
guarantees that the memory data hold timing constraint is met (minimum Chip Enable
high to data transition is usually 0 ns)

•

If the Extended mode is enabled (EXTMOD bit is set in the FMC_BCRx register), up to
four extended modes (A, B, C and D) are available. It is possible to mix A, B, C and D
modes for read and write operations. For example, read operation can be performed in
mode A and write in mode B.

•

If the Extended mode is disabled (EXTMOD bit is reset in the FMC_BCRx register), the
FMC can operate in mode 1 or mode 2 as follows:
–

Mode 1 is the default mode when SRAM/PSRAM memory type is selected (MTYP
= 0x0 or 0x01 in the FMC_BCRx register)

–

Mode 2 is the default mode when NOR memory type is selected (MTYP = 0x10 in
the FMC_BCRx register).

Mode 1 - SRAM/FRAM/PSRAM (CRAM)
The next figures show the read and write transactions for the supported modes followed by
the required configuration of FMC_BCRx, and FMC_BTRx/FMC_BWTRx registers.
Figure 121. Mode 1 read access waveforms
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
MSv41664V1

<!-- pagebreak -->

