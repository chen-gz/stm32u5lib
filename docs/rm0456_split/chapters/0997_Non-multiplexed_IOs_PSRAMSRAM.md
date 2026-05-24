RM0456 Rev 6

RM0456

Flexible static memory controller (FSMC)

NOR flash memory, 16-bit multiplexed I/Os
Table 220. 16-bit multiplexed I/O NOR flash memory
FMC signal name

I/O

Function

CLK

O

Clock (for synchronous access)

A[25:16]

O

Address bus

AD[15:0]

I/O

16-bit multiplexed, bidirectional address/data bus (the 16-bit address
A[15:0] and data D[15:0] are multiplexed on the databus)

NE[x]

O

Chip select, x = 1..4

NOE

O

Output enable

NWE

O

Write enable

NL(=NADV)

O

Latch enable (this signal is called address valid, NADV, by some NOR
flash devices)

NWAIT

I

NOR flash wait input signal to the FMC

The maximum capacity is 512 Mbits.

PSRAM/FRAM/SRAM, non-multiplexed I/Os
Table 221. Non-multiplexed I/Os PSRAM/SRAM
FMC signal name

I/O

Function

CLK

O

Clock (only for PSRAM synchronous access)

A[25:0]

O

Address bus

D[15:0]

I/O

Data bidirectional bus

NE[x]

O

Chip select, x = 1..4 (called NCE by PSRAM (CellularRAM™ i.e. CRAM))

NOE

O

Output enable

NWE

O

Write enable

NL(= NADV)

O

Address valid only for PSRAM input (memory signal name: NADV)

NWAIT

I

PSRAM wait input signal to the FMC

NBL[1:0]

O

Byte lane output. Byte 0 and Byte 1 control (upper and lower byte enable)

The maximum capacity is 512 Mbits.

PSRAM, 16-bit multiplexed I/Os
Table 222. 16-Bit multiplexed I/O PSRAM
FMC signal name

I/O

Function

CLK

O

Clock (for synchronous access)

A[25:16]

O

Address bus

AD[15:0]

I/O

16-bit multiplexed, bidirectional address/data bus (the 16-bit address
A[15:0] and data D[15:0] are multiplexed on the databus)

RM0456 Rev 6

<!-- pagebreak -->

