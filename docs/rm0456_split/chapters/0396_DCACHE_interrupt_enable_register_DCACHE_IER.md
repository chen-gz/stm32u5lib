400

Data cache (DCACHE)

RM0456

Bit 3 BUSYCMDF: command busy flag
0: cache not busy on a CACHECMD command
1: cache busy on a CACHECMD command (clean and/or invalidate an address range)
Bit 2 ERRF: cache error flag
Cleared by writing DCACHE_FCR.CERRF = 1.
0: no error
1: an error occurred during the operation (eviction or clean operation write-back error).
Bit 1 BSYENDF: full invalidate busy end flag
Cleared by writing DCACHE_FCR.CBSYENDF = 1.
0: cache busy or in idle
1: full invalidate CACHEINV operation finished
Bit 0 BUSYF: full invalidate busy flag
0: cache not busy on a CACHEINV operation
1: cache executing a full invalidate CACHEINV operation

9.7.3

DCACHE interrupt enable register (DCACHE_IER)
Address offset: 0x008
Reset value: 0x0000 0000

31

30

29

28

27

26

25

24

23

22

21

20

19

18

17

16

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

15

14

13

12

11

10

9

8

7

6

5

4

3

2

1

0

Res.

CMDE
NDIE

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

rw

Res.

BSYEN
ERRIE
DIE
rw

Res.

rw

Bits 31:5 Reserved, must be kept at reset value.
Bit 4 CMDENDIE: interrupt enable on command end
Set by software to enable an interrupt generation at the end of a cache command (clean
and/or invalidate an address range)
0: interrupt disabled on command end
1: interrupt enabled on command end
Bit 3 Reserved, must be kept at reset value.
Bit 2 ERRIE: interrupt enable on cache error
Set by software to enable an interrupt generation in case of cache functional error (eviction
or clean operation write-back error)
0: interrupt disabled on error
1: interrupt enabled on error
Bit 1 BSYENDIE: interrupt enable on busy end
Set by SW to enable an interrupt generation at the end of a cache full invalidate operation.
0: Interrupt disabled on busy end
1: Interrupt enabled on busy end
Bit 0 Reserved, must be kept at reset value.

<!-- pagebreak -->

