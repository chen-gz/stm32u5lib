381

Instruction cache (ICACHE)

8.6

RM0456

ICACHE error management and interrupts
If an unsupported cacheable write request is detected (functional error), the ICACHE
generates an error by setting the ERRF flag in ICACHE_SR. An interrupt is generated if the
corresponding interrupt enable bit is set (ERRIE = 1 in ICACHE_IER).
The other possible interrupt generation is at the end of a cache invalidation operation.
When the cache-busy state is finished, the ICACHE sets the BSYENDF flag in
ICACHE_SR. An interrupt is generated if the corresponding interrupt enable bit is set
(BSYENDIE = 1 in ICACHE_IER).
All ICACHE interrupt sources raise the same and unique interrupt signal, icache_it, and then
use the same interrupt vector.
Table 87. ICACHE interrupts

Interrupt vector

ICACHE

Interrupt event

Event flag

Enable control bit

Interrupt clear method

Functional error

ERRF
in ICACHE_SR

ERRIE
in ICACHE_IER

Set CERRF to 1
in ICACHE_FCR

End of busy state
(invalidate finished)

BSYENDF
in ICACHE_SR

BSYENDIE
in ICACHE_IER

Set CBSYENDF to 1
in ICACHE_FCR

The ICACHE also propagates all AHB bus errors (such as security issues, address
decoding issues) from the master1 or master2 port back to the execution port.

8.7

ICACHE registers

8.7.1

ICACHE control register (ICACHE_CR)
Address offset: 0x000
Reset value: 0x0000 0004

31
Res.

30
Res.

15
Res.

14
Res.

29
Res.

13
Res.

28
Res.

12
Res.

27
Res.

11
Res.

26
Res.

10
Res.

25
Res.

9
Res.

24
Res.

23
Res.

8
Res.

7
Res.

22
Res.

6
Res.

21
Res.

5
Res.

20

19

18

17

16

Res.

MISSM
RST

HITM
RST

MISSM
EN

HITM
EN

rw

rw

rw

rw

4
Res.

3

2

1

0

Res.

WAY
SEL

CACHE
INV

EN

rw

w

rw

Bits 31:20 Reserved, must be kept at reset value.
Bit 19 MISSMRST: miss monitor reset
0: release the cache miss monitor reset (needed to enable the counting)
1: reset cache miss monitor
Bit 18 HITMRST: hit monitor reset
0: release the cache miss monitor reset (needed to enable the counting)
1: reset cache hit monitor

<!-- pagebreak -->

