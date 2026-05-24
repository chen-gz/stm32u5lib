RM0456 Rev 6

RM0456

Low-power direct memory access controller (LPDMA)

Bit 14 TOF: trigger overrun flag clear
0: no effect
1: clears the corresponding TOF flag
Bit 13 SUSPF: completed suspension flag clear
0: no effect
1: corresponding SUSPF flag cleared
Bit 12 USEF: user setting error flag clear
0: no effect
1: corresponding USEF flag cleared
Bit 11 ULEF: update link transfer error flag clear
0: no effect
1: corresponding ULEF flag cleared
Bit 10 DTEF: data transfer error flag clear
0: no effect
1: corresponding DTEF flag cleared
Bit 9 HTF: half transfer flag clear
0: no effect
1: corresponding HTF flag cleared
Bit 8 TCF: transfer complete flag clear
0: no effect
1: corresponding TCF flag cleared
Bits 7:0 Reserved, must be kept at reset value.

18.8.8

LPDMA channel x status register (LPDMA_CxSR)
Address offset: 0x060 + 0x80 * x (x = 0 to 3)
Reset value: 0x0000 0001
This is a read register, reporting the channel status.
This register is secure or nonsecure, depending on the secure state of channel x
(LPDMA_SECCFGR.SECx), and privileged or unprivileged, depending on the privileged
state of the channel (LPDMA_PRIVCFGR.PRIVx).

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

IDLEF

r

r

r

r

r

r

r

r

Bits 31:15 Reserved, must be kept at reset value.
Bit 14 TOF: trigger overrun flag clear
0: no effect
1: clears the corresponding TOF flag

RM0456 Rev 6

<!-- pagebreak -->

821

Low-power direct memory access controller (LPDMA)

RM0456

Bit 13 SUSPF: completed suspension flag
0: no completed suspension event
1: a completed suspension event occurred
Bit 12 USEF: user setting error flag
0: no user setting error event
1: a user setting error event occurred
Bit 11 ULEF: update link transfer error flag
0: no update link transfer error event
1: a master bus error event occurred while updating a linked-list register from memory
Bit 10 DTEF: data transfer error flag
0: no data transfer error event
1: a master bus error event occurred on a data transfer
Bit 9 HTF: half transfer flag
0: no half transfer event
1: a half transfer event occurred
A half transfer event is a half block transfer that occurs when half of the bytes of the source
block size (rounded-up integer of LPDMA_CxBR1.BNDT[15:0] / 2) has been transferred to
the destination.
Bit 8 TCF: transfer complete flag
0: no transfer complete event
1: a transfer complete event occurred
A transfer complete event is a block transfer complete or a LLI transfer complete including
the upload of the next LLI if any, or the full linked-list completion, depending on the transfer
complete event mode LPDMA_CxTR2.TCEM[1:0].
Bits 7:1 Reserved, must be kept at reset value.
Bit 0 IDLEF: idle flag
0: channel not in idle state
1: channel in idle state
This idle flag is de-asserted by hardware when the channel is enabled
(LPDMA_CxCR.EN = 1) with a valid channel configuration (no USEF to be immediately
reported).
This idle flag is asserted after hard reset or by hardware when the channel is back in idle
state (in suspended or disabled state).

<!-- pagebreak -->

