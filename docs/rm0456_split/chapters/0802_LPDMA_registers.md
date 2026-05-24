821

Low-power direct memory access controller (LPDMA)

RM0456

A transfer error rises in one of the following situations:
•

during a single data transfer from the source or to the destination (DTEF)

•

during an update of a LPDMA channel register from the programmed LLI in memory
(ULEF)

•

during a tentative execution of a LPDMA channel with an unauthorized setting (USEF)
The user must perform a debug session to correct the LPDMA channel programming
versus the USEF root causes list (see Section 18.4.16).

A trigger overrun is described in Trigger hit memorization and trigger overrun flag
generation.

18.8

LPDMA registers
The LPDMA registers must be accessed with an aligned 32-bit word data access.

18.8.1

LPDMA secure configuration register (LPDMA_SECCFGR)
Address offset: 0x000
Reset value: 0x0000 0000
A write access is ignored at bit level if the corresponding channel x is locked
(LPDMA_RCFGLOCKR.LOCKx = 1).
A write access to this register must be secure and privileged. A read access is secure or
nonsecure, privileged or unprivileged.
This register must be written when LPDMA_CxCR.EN = 0.
This register is read-only when LPDMA_CxCR.EN = 1.
This register must be programmed at a bit level, at the initialization/closure of a LPDMA
channel (when LPDMA_CxCR.EN = 0), to securely allocate individually any channel x to the
secure or nonsecure world.

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

SEC3

SEC2

SEC1

SEC0

rw

rw

rw

rw

Bits 31:4 Reserved, must be kept at reset value.
Bits 3:0 SECx: secure state of channel x (x = 3 to 0)
0: nonsecure
1: secure

<!-- pagebreak -->

