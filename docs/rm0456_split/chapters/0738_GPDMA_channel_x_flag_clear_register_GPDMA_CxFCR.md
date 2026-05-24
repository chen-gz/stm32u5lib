763

General purpose direct memory access controller (GPDMA)

RM0456

This channel-based register is the linked-list base address of the memory region, for a given
channel x, from which the LLIs describing the programmed sequence of the GPDMA
transfers, are conditionally and automatically updated.
This 64-Kbyte aligned channel x linked-list base address is offset by the 16-bit
GPDMA_CxLLR register that defines the word-aligned address offset for each LLI.
31

30

29

28

27

26

25

24

rw

rw

rw

rw

rw

rw

rw

rw

15

14

13

12

11

10

9

8

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

23

22

21

20

19

18

17

16

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

Res.

Res.

Res.

Res.

Res.

Res.

Res.

LBA[31:16]

Bits 31:16 LBA[31:16]: linked-list base address of GPDMA channel x
Bits 15:0 Reserved, must be kept at reset value.

17.8.7

GPDMA channel x flag clear register (GPDMA_CxFCR)
Address offset: 0x5C+ 0x80 * x (x = 0 to 15)
Reset value: 0x0000 0000
This is a write register, secure or nonsecure depending on the secure state of channel x
(GPDMA_SECCFGR.SECx) and privileged or unprivileged, depending on the privileged
state of the channel x (GPDMA_PRIVCFGR.PRIVx).

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

TOF

SUSPF

USEF

ULEF

DTEF

HTF

TCF

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

w

w

w

w

w

w

w

Bits 31:15 Reserved, must be kept at reset value.
Bit 14 TOF: trigger overrun flag clear
0: no effect
1: corresponding TOF flag cleared
Bit 13 SUSPF: completed suspension flag clear
0: no effect
1: corresponding SUSPF flag cleared
Bit 12 USEF: user setting error flag clear
0: no effect
1: corresponding USEF flag cleared
Bit 11 ULEF: update link transfer error flag clear
0: no effect
1: corresponding ULEF flag cleared

<!-- pagebreak -->

