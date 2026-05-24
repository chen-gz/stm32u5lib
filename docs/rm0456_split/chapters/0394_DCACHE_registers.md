400

Data cache (DCACHE)

RM0456

9.7

DCACHE registers

9.7.1

DCACHE control register (DCACHE_CR)
Address offset: 0x000
Reset value: 0x0000 0000

31
HBURS
T

30

29

28

27

26

25

24

Res.

Res.

Res.

Res.

Res.

Res.

Res.

14

13

12

11

10

9

8

Res.

START
CMD

23

rw
15
Res.

Res.

Res.

w

CACHECMD[2:0]
rw

rw

22

21

20

19

18

17

rw

rw

rw

rw

rw

rw

rw

rw

7

6

5

4

3

2

1

0

Res.

CACHE
INV

EN

w

rw

Res.

Res.

Res.

Res.

Res.

rw

Bit 31 HBURST: output burst type for cache master port read accesses
Can be set by software, only when EN = 0.
Master port write accesses are always done in INCR burst type.
0: WRAP
1: INCR
Bits 30:24 Reserved, must be kept at reset value.
Bit 23 WMISSMRST: write-miss monitor reset
0: release the cache miss monitor reset (needed to enable the counting)
1: reset cache write-miss monitor
Bit 22 WHITMRST: write-hit monitor reset
0: release the cache miss monitor reset (needed to enable the counting)
1: reset cache write-hit monitor
Bit 21 WMISSMEN: write-miss monitor enable
0: cache write-miss monitor switched off. Stopping the monitor does not reset it.
1: cache write-miss monitor enabled
Bit 20 WHITMEN: write-hit monitor enable
0: cache write-hit monitor switched off. Stopping the monitor does not reset it.
1: cache write-hit monitor enabled
Bit 19 RMISSMRST: read-miss monitor reset
0: release the cache miss monitor reset (needed to enable the counting)
1: reset cache read-miss monitor
Bit 18 RHITMRST: read-hit monitor reset
0: release the cache miss monitor reset (needed to enable the counting)
1: reset cache read-hit monitor
Bit 17 RMISSMEN: read-miss monitor enable
0: cache read-miss monitor switched off. Stopping the monitor does not reset it.
1: cache read-miss monitor enabled
Bit 16 RHITMEN: read-hit monitor enable
0: cache read-hit monitor switched off. Stopping the monitor does not reset it.
1: cache read-hit monitor enabled

<!-- pagebreak -->

