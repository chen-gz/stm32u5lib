RM0456 Rev 6

RM0456

Low-power direct memory access controller (LPDMA)

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

MIS3

MIS2

MIS1

MIS0

r

r

r

r

Bits 31:4 Reserved, must be kept at reset value.
Bits 3:0 MISx: masked interrupt status of channel x (x = 3 to 0)
0: no interrupt occurred on nonsecure channel x
1: an interrupt occurred on nonsecure channel x

18.8.5

LPDMA secure masked interrupt status register (LPDMA_SMISR)
Address offset: 0x010
Reset value: 0x0000 0000
This is a secure read register, containing the masked interrupt status bit MISx for each
secure channel x (LPDMA_SECCFGR.SECx = 1). It is a logical OR of all the LPDMA_CxSR
flags, each source flag being enabled by the corresponding LPDMA_CxCR interrupt enable
bit.
Every bit is de-asserted by hardware when securely writing 1 to the corresponding
LPDMA_CxFCR flag clear bit.
This register does not contain any information about a nonsecure channel.
This register can mix privileged and unprivileged information, depending on the privileged
state of each channel LPDMA_PRIVCFGR.PRIVx. A privileged software can read the full
secure interrupt status. An unprivileged software is restricted to read the status of
unprivileged and secure channels, other privileged bit fields returning zero.

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

MIS3

MIS2

MIS1

MIS0

r

r

r

r

Bits 31:4 Reserved, must be kept at reset value.
Bits 3:0 MISx: masked interrupt status of the secure channel x (x = 3 to 0)
0: no interrupt occurred on the secure channel x
1: an interrupt occurred on the secure channel x

RM0456 Rev 6

<!-- pagebreak -->

