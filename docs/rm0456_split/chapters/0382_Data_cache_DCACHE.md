381

Data cache (DCACHE)

RM0456

9

Data cache (DCACHE)

9.1

DCACHE introduction
The data cache (DCACHE) is introduced on S-AHB system bus of the Cortex®-M33
processor, or on an AHB bus driven by a master peripheral, to improve the performance of
data traffic to/from external memories.
Some specific features, like hit-under-miss and critical-word-first refill policy, optimize
performance on external memories data accesses.

9.2

DCACHE main features
The main features of DCACHE are described below:
•

•

•

•

Bus interface
–

One 32-bit AHB slave port, the system port (input from Cortex®-M33 S-AHB
system interface, or input from AHB bus driven by the port M0 of GPU2D)

–

One 32-bit AHB master port (output to main AHB bus matrix)

–

One 32-bit AHB slave port for control (input from AHB peripherals interconnect,
for access to DCACHE registers)

Cache access
–

0 wait-state on hits

–

Hit-under-miss capability: ability to serve processor requests (access to cached
data) during an ongoing line refill due to a previous cache miss

–

Optimized cache line refill thanks to WRAP bursts of the size of the cache line
(such as WRAP4 for 128-bit cache line)

–

2-way set-associative

–

Supports both write-back and write-through policies (selectable with AHB
bufferable attribute)

–

Read and write-back always allocate

–

Write-through always non-allocate (write-around)

–

Supports byte, half-word, and word writes

Replacement and refill
–

pLRU-t replacement policy (pseudo-least-recently-used, based on binary tree),
algorithm with best complexity/performance balance

–

Critical-word-first refill policy for read transactions, minimizing processor stalls

–

Possibility to configure burst type of all AHB memory transactions: INCRw or
WRAPw (size w aligned on cache line size)

Performance counters
The DCACHE implements four performance counters:

<!-- pagebreak -->

