1045

Flexible static memory controller (FSMC)

RM0456

Table 245. 8-bit NAND flash (continued)
FMC signal name

I/O

Function

NCE

O

Chip select

NOE(= NRE)

O

Output enable (memory signal name: read enable, NRE)

NWE

O

Write enable

NWAIT/INT

I

NAND flash ready/busy input signal to the FMC

Theoretically, there is no capacity limitation as the FMC can manage as many address
cycles as needed.

16-bit NAND flash memory
Table 246. 16-bit NAND flash
FMC signal name

I/O

Function

A[17]

O

NAND flash address latch enable (ALE) signal

A[16]

O

NAND flash command latch enable (CLE) signal

D[15:0]

I/O

16-bit multiplexed, bidirectional address/data bus

NCE

O

Chip select

NOE(= NRE)

O

Output enable (memory signal name: read enable, NRE)

NWE

O

Write enable

NWAIT/INT

I

NAND flash ready/busy input signal to the FMC

Theoretically, there is no capacity limitation as the FMC can manage as many address
cycles as needed.

27.7.2

NAND flash supported memories and transactions
Table 247 shows the supported devices, access modes and transactions. Transactions not
allowed (or not supported) by the NAND flash controller are shown in gray.
Table 247. Supported memories and transactions
Device

NAND 8-bit

<!-- pagebreak -->

