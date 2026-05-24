400

Data cache (DCACHE)

RM0456

Table 90. DCACHE features for STM32U59x/5Ax/5Fx/5Gx (continued)
Features

9.4

DCACHE1/2

Cache line width

32 bytes

Data size of AHB Master interface

32 bits

DCACHE functional description
The purpose of the data cache is to cache external memory data loads and stores, coming
from the processor or from another bus master peripheral. These accesses include the
instruction fetches that may occur at an external memory address. The DCACHE manages
both read and write transactions.

9.4.1

DCACHE block diagram
Figure 30. DCACHE block diagram

AHB

Configuration
slave port

CMD range start @

Control

CMD range end @

Status

Cache control logic
Cache
FSM

Maintenance
operations

pLRU-t

Master
port
AHB

Main AHB

AHB

Write-hit monitor
Write-miss monitor

Master port interface

Input port

Read-hit monitor
Read-miss monitor

Slave port interface

GPU2D
Cortex-M33

S-AHB or M0 port

Configuration interface

dcache_it
Cache memory port

Cache
TAG
Memories
DCACHE

n ways

Cache
data
Memories
n ways
MSv72344V1

<!-- pagebreak -->

