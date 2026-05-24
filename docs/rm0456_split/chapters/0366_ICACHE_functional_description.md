381

Instruction cache (ICACHE)

8.4

RM0456

ICACHE functional description
The purpose of the instruction cache is to cache instruction fetches or instruction memories
loads, coming from the processor. As such, the ICACHE only manages cacheable read
transactions and does not manage cacheable write transactions.
The noncacheable transactions (both read and write ones) bypass the ICACHE.
For the error management purpose, in case a write cacheable transaction is presented
(this only happens in case of bad software programming), the ICACHE sets an error flag
and, if enabled, raises an interrupt to the processor.

8.4.1

ICACHE block diagram

AHB

Figure 27. ICACHE block diagram

Configuration
slave port

Control

Region 1 cfg

Region 3 cfg

Miss monitor

Status

Master1
port
Cache control logic
Cache
FSM

pLRU-t

REMAP

AHB

Master2
port

Main AHB

AHB

Hit monitor

Master port interface

Execution
port

Region 2 cfg

Execution port interface

C-AHB

Cortex-M33

Configuration interface
Region 0 cfg

AHB

icache_it
Cache memory port

Cache
TAG
memories

ICACHE

n ways

Cache
data
memories
n ways

MSv48191V3

<!-- pagebreak -->

