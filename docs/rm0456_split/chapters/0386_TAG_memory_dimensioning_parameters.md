400

Data cache (DCACHE)

RM0456

The privilege bit indicates if the data is managed by a privileged entity. It is assigned
according to the value of AHB privileged attribute at the input slave port, for the first access
to this line (it is written only during the line refill, on read miss or write-back miss).
The privilege bit holds the same polarity as the privileged attribute: 1 for privileged access,
0 for unprivileged access.
Privilege bits are reset when the cache is invalidated, and after the DCACHE reset is
released.
When a cacheable transaction is received at input slave port, its AHB address (HADDR_in)
is split into the following fields (see the table below for B and W values):
•

HADDR_in[B-1:0]: address byte offset, indicates which byte to select within a cache
line.

•

HADDR_in[B+W-1:B]: address way index, indicates which cache line to select inside
each way.

•

HADDR_in[31:B+W]: tag address, to be compared to TAG memory address to check if
the requested data is already available (meaning valid) within the DCACHE

Table 91 gives DCACHE main parameters for TAG memory dimensioning.
Figure 31 shows the functional view of TAG and data memories, for an n-way set
associative DCACHE.
Table 91. TAG memory dimensioning parameters
Parameter

Value

Example

S Kbytes = s bytes (s = 1024 x S)

4 Kbytes = 4096 bytes

n

2

L-byte = l-bit (l = 8 x L)

16-byte = 128-bit

LpW = s / (n x L) lines/way

128 lines/way

Address byte offset size

B = log2(L) bit

4-bit

Address way index size

W = log2(LpW) bit

7-bit

TAG address size

T = (32 - W - B) bit

21-bit

Cache size
Cache number of ways
Cache line size
Number of cache lines (per way)

<!-- pagebreak -->

