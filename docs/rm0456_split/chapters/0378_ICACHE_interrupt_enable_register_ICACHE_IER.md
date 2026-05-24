381

Instruction cache (ICACHE)

8.7.3

RM0456

ICACHE interrupt enable register (ICACHE_IER)
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

ERRIE

BSYEN
DIE

Res.

rw

rw

Bits 31:3 Reserved, must be kept at reset value.
Bit 2 ERRIE: interrupt enable on cache error
Set by software to enable an interrupt generation in case of cache functional error (cacheable
write access)
0: interrupt disabled on error
1: interrupt enabled on error
Bit 1 BSYENDIE: interrupt enable on busy end
Set by software to enable an interrupt generation at the end of a cache invalidate operation.
0: interrupt disabled on busy end
1: interrupt enabled on busy end
Bit 0 Reserved, must be kept at reset value.

8.7.4

ICACHE flag clear register (ICACHE_FCR)
Address offset: 0x00C
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

CERRF

CBSY
ENDF

Res.

w

w

Bits 31:3 Reserved, must be kept at reset value.
Bit 2 CERRF: clear cache error flag
Set by software.
0: no effect
1: clears ERRF flag in ICACHE_SR
Bit 1 CBSYENDF: clear busy end flag
Set by software.
0: no effect
1: clears BSYENDF flag in ICACHE_SR.

<!-- pagebreak -->

