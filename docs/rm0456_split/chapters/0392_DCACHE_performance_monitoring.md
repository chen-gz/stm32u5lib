400

Data cache (DCACHE)

RM0456

Alternatively it can also rely on the command end flag (CMDENDF) or on the DCACHE
interrupt to detect the end of the clean range execution.
•

Clean and invalidate range: cleans and invalidates a certain range of addresses in
the cache, background task (interruptible).
This operation cleans the “dirty” cache lines that belong to the operation address range
(the same as the clean range operation), and also invalidates all the (valid) cache lines
that belong to this address range (whether they are dirty or not).
The software can launch this clean and invalidate range operation, by programming
STARTCMD = 1, and CACHECMD = 0b011 in DCACHE_CR, after the address range
was programmed into DCACHE_CMDRSADDRR (range start address) and
DCACHE_CMDREADDRR (range end address).
This sets and clears the same flags, and potentially the same interrupt as invalidate
range or clean range operations.

9.4.9

DCACHE performance monitoring
The DCACHE provides the following monitors for performance analysis:
•

The two 32-bit read-hit and write-hit monitors count the AHB transactions at
the DCACHE input (slave port) that do not generate a transaction on the DCACHE
output (master port).
These monitors also take into account all accesses whose address is present in the
TAG memory, or in the refill buffer (due to a previous miss, and whose data is coming,
or is soon to come, from the cache master port) (see Section 9.4.6).

•

The two 16-bit read-miss and write-miss monitors count the AHB transactions at
the DCACHE input (slave port) that generate a transaction on the DCACHE output
(master port).
These monitors also take into account all accesses whose address is not present
neither in the TAG memory, nor in the refill buffer.

Upon reaching their maximum values, the monitors do not wrap over.
The software can perform the following tasks:
•

Enable/stop the read (write) hit monitor, through R(W)HITMEN in DCACHE_CR.

•

Reset the read (write) hit monitor, by setting R(W)HITMRST in DCACHE_CR.

•

Enable/stop the read (write) miss monitor, through R(W)MISSMEN in DCACHE_CR.

•

Reset the read (write) miss monitor, by setting R(W)MISSMRST in DCACHE_CR.

To reduce power consumption, these monitors are disabled (stopped) by default.

9.4.10

DCACHE boot
The DCACHE is disabled (EN = 0 in DCACHE_CR) at boot.
Once the boot is finished, the DCACHE can be enabled (software setting EN = 1
in DCACHE_CR).

<!-- pagebreak -->

