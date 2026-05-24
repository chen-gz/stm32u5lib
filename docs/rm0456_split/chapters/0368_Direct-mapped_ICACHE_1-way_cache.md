381

Instruction cache (ICACHE)

RM0456

Table 82. TAG memory dimensioning parameters
for n-way set associative operating mode (default) (continued)
Parameter

Value

Example

Number of cache lines (per way)

LpW = s / (n x L) lines / way

256 lines / way

Address byte offset size

B = log2(L) bit

4-bit

Address way index size

W = log2(LpW) bit

8-bit

TAG address size

T = (32 - W - B) bit

20-bit

Figure 28. ICACHE TAG and data memories functional view
T-bit

AHB_address
(HADDR_in)

W-bit

TAG

Index

Offset

way selection
(for replacement)

pLRU-t

Vn-1
V0

TAG_Way(n-1)

Data_Way(n-1)

TAG_Way0

Data_Way0

Data memory

n ways

T-bit

LpW lines / way

LpW lines / way

TAG memory

n ways

B-bit

l-bit

==
==

Cache hit/miss, in Way(n-1)
Cache hit/miss, in Way0
MSv48192V2

8.4.4

Direct-mapped ICACHE (1-way cache)
The default configuration (at reset) is an n-way set associative cache (WAYSEL = 1
in ICACHE_CR), but the user can configure the ICACHE as direct mapped by writing
WAYSEL = 0 (only possible when the cache is disabled, EN = 0 in ICACHE_CR).

<!-- pagebreak -->

