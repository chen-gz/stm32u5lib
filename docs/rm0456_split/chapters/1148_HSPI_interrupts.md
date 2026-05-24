1177

Hexadeca-SPI interface (HSPI)

RM0456

Table 269. Address alignment cases (continued)
Memory type

16-bit data bus memory
or dual-octal memory
with WDM

Transaction
type

Constraint
on
address(1)

Impact if constraint
on address not
respected

Constraint
on number
of bytes(1)

Impact if constraint
on bytes not
respected

IND read
32-bit

Aligned

ADDR[1:0] is assumed
to be 00.(4)

N×4

DLR[1:0] is assumed
to be 11.(5)

None

None

None

None

Aligned

ADDR[1:0] is assumed
to be 00.(4)

N×4

DLR[1:0] is assumed
to be 11.(5)

None

None

None

None

MM read
IND write(8)
MM write
IND read
32-bit
MM read

16-bit HyperBus

IND write(9)
MM write
1. To be respected by the software.
2. IND = indirect mode.
3. MM = memory-mapped mode.
4. Extra data at transfer start.
5. Extra data at transfer end.
6. WDM = write data mask.

7. If the FTHRES bitfield is set to the maximum value with DLR value greater than the data burst length, and if the DMA is
enabled or the interrupt based on FIFO THRESHOLD Flag is enabled (FTF), the address must be modulo 2 aligned in DTR
mode when the initiator (DMA, CPU, ...) is writing the data with a burst length equal to the FIFO size.
8. If the FTHRES bitfield is set to the maximum value with DLR value greater than the data burst length, and if the DMA is
enabled or the interrupt based on FIFO THRESHOLD Flag is enabled (FTF), the address must be modulo 4 aligned in DTR
mode or modulo 2 in SDR mode when the initiator (DMA, CPU, ...) is writing the data with a burst length equal to the FIFO
size.
9. If the FTHRES bitfield is set to the maximum value with DLR value greater than the data burst length, and if the DMA is
enabled or the interrupt based on FIFO THRESHOLD Flag is enabled (FTF), the address must be modulo 4 aligned in DTR
mode when the initiator (DMA, CPU, ...) is writing the data with a burst length equal to the FIFO size.

30.6

HSPI interrupts
An interrupt can be produced on the following events:
•

timeout

•

status match

•

FIFO threshold

•

transfer complete

•

transfer error

Separate interrupt enable bits are available to provide more flexibility.
Table 270. HSPI interrupt requests
Interrupt event

<!-- pagebreak -->

