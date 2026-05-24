RM0456 Rev 6

RM0456

Low-power direct memory access controller (LPDMA)

18.8.9

LPDMA channel x control register (LPDMA_CxCR)
Address offset: 0x64 + 0x80 * x (x = 0 to 3)
Reset value: 0x0000 0000
This register is secure or nonsecure depending on the secure state of channel x
(LPDMA_SECCFGR.SECx), and privileged or unprivileged, depending on the privileged
state of the channel x (LPDMA_PRIVCFGR.PRIVx).
This register is used to control a channel (activate, suspend, abort or disable it).

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

PRIO[1:0]

Res.

Res.

Res.

Res.

Res.

LSM

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

7

6

5

4

3

2

1

0

Res.

TOIE

SUSPI
E

USEIE

ULEIE

DTEIE

HTIE

TCIE

Res.

Res.

Res.

Res.

Res.

SUSP

RESET

EN

rw

rw

rw

rw

rw

rw

rw

rw

w

rw

rw

Bits 31:24 Reserved, must be kept at reset value.
Bits 23:22 PRIO[1:0]: priority level of the channel x LPDMA transfer versus others
00: low priority, low weight
01: low priority, mid weight
10: low priority, high weight
11: high priority
Note: This bit must be written when EN = 0. This bit is read-only when EN = 1.
Bits 21:17 Reserved, must be kept at reset value.
Bit 16 LSM: Link step mode
0: channel executed for the full linked-list and completed at the end of the last LLI
(LPDMA_CxLLR = 0). The 16 low-significant bits of the link address are null (LA[15:0] = 0)
and all the update bits are null (UT1 = UB1 = UT2 = USA = UDA = ULL = 0). Then
LPDMA_CxBR1.BNDT[15:0] = 0.
1: channel executed once for the current LLI
First the block transfer is executed as defined by the current internal register file until
LPDMA_CxBR1.BNDT[15:0] = 0). Secondly the next linked-list data structure is conditionally
uploaded from memory as defined by LPDMA_CxLLR. Then channel execution is completed.
Note: This bit must be written when EN = 0. This bit is read-only when EN = 1.
Bit 15 Reserved, must be kept at reset value.
Bit 14 TOIE: trigger overrun interrupt enable
0: interrupt disabled
1: interrupt enabled
Bit 13 SUSPIE: completed suspension interrupt enable
0: interrupt disabled
1: interrupt enabled
Bit 12 USEIE: user setting error interrupt enable
0: interrupt disabled
1: interrupt enabled

RM0456 Rev 6

<!-- pagebreak -->

821

Low-power direct memory access controller (LPDMA)

RM0456

Bit 11 ULEIE: update link transfer error interrupt enable
0: interrupt disabled
1: interrupt enabled
Bit 10 DTEIE: data transfer error interrupt enable
0: interrupt disabled
1: interrupt enabled
Bit 9 HTIE: half transfer complete interrupt enable
0: interrupt disabled
1: interrupt enabled
Bit 8 TCIE: transfer complete interrupt enable
0: interrupt disabled
1: interrupt enabled
Bits 7:3 Reserved, must be kept at reset value.
Bit 2 SUSP: suspend
Writing 1 into the field RESET (bit 1) causes the hardware to de-assert this bit, whatever is
written into this bit 2. Else:
Software must write 1 in order to suspend an active channel (with an ongoing DMA transfer
over its master ports).
The software must write 0 in order to resume a suspended channel, following the
programming sequence detailed in Figure 74.
0: write: resume channel, read: channel not suspended
1: write: suspend channel, read: channel suspended.
Bit 1 RESET: reset
This bit is write only. Writing 0 has no impact. Writing 1 implies the reset of the following: the
channel internal state, SUSP and EN bits (whatever is written receptively in bit 2 and bit 0).
The reset is effective when the channel is in steady state, meaning one of the following:
- active channel in suspended state (LPDMA_CxSR.SUSPF = 1 and
LPDMA_CxSR.IDLEF = LPDMA_CxCR.EN = 1)
- channel in disabled state (LPDMA_CxSR.IDLEF = 1 and LPDMA_CxCR.EN = 0).
After writing a RESET, to continue using this channel, the user must explicitly reconfigure the
channel including the hardware-modified configuration registers (LPDMA_CxBR1,
LPDMA_CxSAR and LPDMA_CxDAR) before enabling again the channel (see the
programming sequence in Figure 75).
0: no channel reset
1: channel reset
Bit 0 EN: enable
Writing 1 into the field RESET (bit 1) causes the hardware to de-assert this bit, whatever is
written into this bit 0. Else:
this bit is de-asserted by hardware when there is a transfer error (master bus error or user
setting error) or when there is a channel transfer complete (channel ready to be configured,
for example: if LSM = 1 at the end of a single execution of the LLI).
Else, this bit can be asserted by software.
Writing 0 into this EN bit is ignored.
0: write: ignored, read: channel disabled
1: write: enable channel, read: channel enabled

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Low-power direct memory access controller (LPDMA)

18.8.10

LPDMA channel x transfer register 1 (LPDMA_CxTR1)
Address offset: 0x090 + 0x80 * x (x = 0 to 3)
Reset value: 0x0000 0000
This register is secure or nonsecure depending on the secure state of channel x
(LPDMA_SECCFGR.SECx) except for secure DSEC and SSEC, privileged or unprivileged,
depending on the privileged state of the channel x in LPDMA_PRIVCFGR.PRIVx.
This register controls the transfer of a channel x.
This register must be written when LPDMA_CxCR.EN = 0.
This register is read-only when LPDMA_CxCR.EN = 1.
This register must be written when the channel is completed. Then the hardware has
deasserted LPDMA_CxCR.EN). A channel transfer can be completed and programmed at
different levels: block or LLI or full linked-list.
In linked-list mode, during the link transfer, this register is automatically updated by LPDMA
from the memory if LPDMA_CxLLR.UT1 = 1.

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

DSEC

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

DINC

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

SSEC

Res.

Res.

Res.

PAM

Res.

Res.

Res.

Res.

Res.

Res.

Res.

SINC

Res.

rw

rw

rw

rw

rw

17

16

DDW_LOG2[1:0]
rw

rw

1

0

SDW_LOG2[1:0]
rw

rw

Bit 31 DSEC: security attribute of the LPDMA transfer to the destination
If LPDMA_SECCFGR.SECx = 1 and the access is secure:
0: LPDMA transfer nonsecure
1: LPDMA transfer secure
This is a secure register bit. This bit can only be read by a secure software. This bit must be
written by a secure software when LPDMA_SECCFGR.SECx = 1. A secure write is ignored
when LPDMA_SECCFGR.SECx = 0.
When LPDMA_SECCFGR.SECx is de-asserted, this DSEC bit is also de-asserted by
hardware (on a secure reconfiguration of the channel as nonsecure), and the LPDMA
transfer to the destination is nonsecure.
Bits 30:20 Reserved, must be kept at reset value.
Bit 19 DINC: destination incrementing single
0: fixed single
1: contiguously incremented single
The destination address, pointed by LPDMA_CxDAR, is kept constant after a single transfer,
or is incremented by the offset value corresponding to a contiguous data after a single
transfer.
Bit 18 Reserved, must be kept at reset value.

RM0456 Rev 6

<!-- pagebreak -->

821

Low-power direct memory access controller (LPDMA)

RM0456

Bits 17:16 DDW_LOG2[1:0]: binary logarithm of the destination data width of a single in bytes
00: byte
01: half-word (2 bytes)
10: word (4 bytes)
11: user setting error reported and no transfer issued
Note: Setting a 8-byte data width causes a user setting error to be reported and none transfer
is issued.
A destination single transfer must have an aligned address with its data width (start
address LPDMA_CxDAR[2:0] versus DDW_LOG2[1:0]). Otherwise a user setting error
is reported and none transfer is issued.
Bit 15 SSEC: security attribute of the LPDMA transfer from the source
If LPDMA_SECCFGR.SECx = 1 and the access is secure:
0: LPDMA transfer nonsecure
1: LPDMA transfer secure
This is a secure register bit. This bit can only be read by a secure software. This bit must be
written by a secure software when LPDMA_SECCFGR.SECx =1 . A secure write is ignored
when LPDMA_SECCFGR.SECx = 0.
When LPDMA_SECCFGR.SECx is de-asserted, this SSEC bit is also de-asserted by
hardware (on a secure reconfiguration of the channel as nonsecure), and the LPDMA
transfer from the source is nonsecure.
Bits 14:12 Reserved, must be kept at reset value.
Bit 11 PAM: padding/alignment mode
If DDW_LOG2[1:0] = SDW_LOG2[1:0]: if the data width of a single destination transfer is
equal to the data width of a single source transfer, this bit is ignored.
Else, in the following enumerated values, the condition PAM_1 is when destination data width
is higher that source data width, and the condition PAM_2 is when destination data width is
higher than source data width.
Condition: PAM_1
0: source data is transferred as right aligned, padded with 0s up to the destination data width
1: source data is transferred as right aligned, sign extended up to the destination data width
Condition: PAM_2
0: source data is transferred as right aligned, left-truncated down to the destination data width
1: source data is transferred as left-aligned, right-truncated down to the destination data
width
Bits 10:4 Reserved, must be kept at reset value.
Bit 3 SINC: source incrementing single
0: fixed single
1: contiguously incremented single
The source address, pointed by LPDMA_CxSAR, is kept constant after a single transfer or is
incremented by the offset value corresponding to a contiguous data after a single transfer.
Bit 2 Reserved, must be kept at reset value.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Low-power direct memory access controller (LPDMA)

Bits 1:0 SDW_LOG2[1:0]: binary logarithm of the source data width of a single in bytes
00: byte
01: half-word (2 bytes)
10: word (4 bytes)
11: user setting error reported and no transfer issued
Note: Setting a 8-byte data width causes a user setting error to be reported and none transfer
is issued.
a source block size must be a multiple of the source data width
(LPDMA_CxBR1.BNDT[2:0] versus SDW_LOG2[1:0]). Otherwise, a user setting error is
reported and no transfer is issued.
A source single transfer must have an aligned address with its data width (start address
LPDMA_CxSAR[2:0] versus SDW_LOG2[1:0]). Otherwise, a user setting error is
reported and none transfer is issued.

18.8.11

LPDMA channel x transfer register 2 (LPDMA_CxTR2)
Address offset: 0x094 + 0x80 * x (x = 0 to 3)
Reset value: 0x0000 0000
This register is secure or nonsecure depending on the secure state of channel x
(LPDMA_SECCFGR.SECx), and privileged or unprivileged, depending on the privileged
state of channel x (LPDMA_PRIVCFGR.PRIVx).
This register controls the transfer of a channel x.
This register must be written when LPDMA_CxCR.EN = 0.
This register is read-only when LPDMA_CxCR.EN = 1.
This register must be written when the channel is completed (the hardware de-asserted
LPDMA_CxCR.EN). A channel transfer can be completed and programmed at different
levels: block or LLI or full linked-list.
In linked-list mode, during the link transfer, this register is automatically updated by LPDMA
from the memory, if LPDMA_CxLLR.UT2 = 1.

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

TCEM[1:0]

Res.

Res.

Res.

Res.

TRIGPOL[1:0]

Res.

Res.

Res.

rw

rw

13

12

11

10

9

8

7

6

5

Res.

SWRE
Q

Res.

Res.

Res.

Res.

rw

rw

15

14

TRIGM[1:0]
rw

rw

Res.

Res.

BREQ
rw

rw

20

18

17

16

TRIGSEL[4:0]
rw

rw

rw

rw

rw

4

3

2

1

0

rw

rw

REQSEL[4:0]
rw

RM0456 Rev 6

19

rw

rw

<!-- pagebreak -->

821

Low-power direct memory access controller (LPDMA)

RM0456

Bits 31:30 TCEM[1:0]: transfer complete event mode
These bits define the transfer granularity for the transfer complete and half transfer complete
events generation.
00: at block level (when LPDMA_CxBR1.BNDT[15:0] = 0): the complete (and the half)
transfer event is generated at the (respectively half of the) end of a block.
Note: If the initial LLI0 data transfer is null/void (directly programmed by the internal register
file with LPDMA_CxBR1.BNDT[15:0] = 0), then neither the complete transfer event nor
the half transfer event is generated.
01: same as 00
10: at LLI level: the complete transfer event is generated at the end of the LLI transfer,
including the update of the LLI if any. The half transfer event is generated at the half of the
LLI data transfer (the LLI data transfer being a block transfer), if any data transfer.
Note: If the initial LLI0 data transfer is null/void (i.e. directly programmed by the internal
register file with LPDMA_CxBR1.BNDT[15:0] = 0), then the half transfer event is not
generated, and the transfer complete event is generated when is completed the loading
of the LLI1.
11: at channel level: the complete transfer event is generated at the end of the last LLI
transfer. The half transfer event is generated at the half of the data transfer of the last LLI.
The last LLI updates the link address LPDMA_CxLLR.LA[15:2] to zero and clears all the
LPDMA_CxLLR update bits (UT1, UT2, UB1, USA, UDA and ULL). If the channel transfer is
continuous/infinite, no event is generated.
Bits 29:26 Reserved, must be kept at reset value.
Bits 25:24 TRIGPOL[1:0]: trigger event polarity
These bits define the polarity of the selected trigger event input defined by TRIGSEL[4:0].
00: no trigger (masked trigger event)
01: trigger on the rising edge
10: trigger on the falling edge
11: same as 00
Bits 23:21 Reserved, must be kept at reset value.
Bits 20:16 TRIGSEL[4:0]: trigger event input selection
These bits select the trigger event input of the LPDMA transfer (as per Section 18.3.5), with
an active trigger event if TRIGPOL[1:0] = 00.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Low-power direct memory access controller (LPDMA)

Bits 15:14 TRIGM[1:0]: trigger mode
These bits define the transfer granularity for its conditioning by the trigger.
If the channel x is enabled (LPDMA_CxCR.EN asserted) with TRIGPOL[1:0] = 0b00 or 0b11,
these TRIGM[1:0] bits are ignored.
Else, a DMA transfer is conditioned by at least one trigger hit:
00: at block level: the first single read of each block transfer is conditioned by one hit trigger.
01: same as 00
10: at link level: a LLI link transfer is conditioned by one hit trigger. The LLI data transfer (if
any) is not conditioned.
11: at programmed single level: each programmed single read is conditioned by one hit
trigger.
The LPDMA monitoring of a trigger for channel x is started when the channel is
enabled/loaded with a new active trigger configuration: rising or falling edge on a selected
trigger (TRIGPOL[1:0] = 0b01 or respectively TRIGPOL[1:0] = 0b10).
The monitoring of this trigger is kept active during the triggered and uncompleted (data or
link) transfer; and if a new trigger is detected then, this hit is internally memorized to grant the
next transfer, as long as the defined rising or falling edge is not modified, and the
TRIGSEL[4:0] is not modified, and the channel is enabled.
Transferring a next LLIn+1 that updates the LPDMA_CxTR2 with a new value for any of
TRIGSEL[4:0] or TRIGPOL[1:0], resets the monitoring, trashing the memorized hit of the
formerly defined LLIn trigger.
After a first new trigger hitn+1 is memorized, if another second trigger hitn+2 is detected and if
the hitn triggered transfer is still not completed, hitn+2 is lost and not memorized, and a trigger
overrun flag is reported (LPDMA_CxSR.TOF = 1), an interrupt is generated if enabled
(LPDMA_CxCR.TOIE = 1). The channel is not automatically disabled by hardware due to a
trigger overrun.
Bits 13:12 Reserved, must be kept at reset value.
Bit 11 BREQ: block hardware request
If the channel x is activated (LPDMA_CxCR.EN asserted) with SWREQ = 1 (software request
for a memory-to-memory transfer), this bit is ignored. Else:
0: the selected hardware request is driven by a peripheral with a hardware
request/acknowledge protocol at a single level.
1: the selected hardware request is driven by a peripheral with a hardware
request/acknowledge protocol at a block level (see Section 18.3.4).
Bit 10 Reserved, must be kept at reset value.
Bit 9 SWREQ: software request
This bit is internally taken into account when LPDMA_CxCR.EN is asserted.
0: no software request. The selected hardware request REQSEL[4:0] is taken into account.
1: software request for a memory-to-memory transfer. The default selected hardware request
as per REQSEL[4:0] is ignored.
Bits 8:5 Reserved, must be kept at reset value.
Bits 4:0 REQSEL[4:0]: DMA hardware request selection
These bits are ignored if channel x is activated (LPDMA_CxCR.EN asserted) with
SWREQ = 1 (software request for a memory-to-memory transfer). Else, the selected
hardware request is internally taken into account as per Section 18.3.3.
Caution: The user must not assign a same input hardware request (same REQSEL[4:0]
value) to different active DMA channels (LPDMA_CxCR.EN = 1 and
LPDMA_CxTR2.SWREQ = 0 for these channels). DMA is not intended to hardware
support the case of simultaneous enabled channels incorrectly configured with a
same hardware peripheral request signal, and there is no user setting error
reporting.

RM0456 Rev 6

<!-- pagebreak -->

821

Low-power direct memory access controller (LPDMA)

18.8.12

RM0456

LPDMA channel x block register 1 (LPDMA_CxBR1)
Address offset: 0x098 + 0x80 * x (x = 0 to 3)
Reset value: 0x0000 0000
This register is secure or nonsecure depending on the secure state of channel x
(LPDMA_SECCFGR.SECx), and privileged or unprivileged, depending on the privileged
state of channel x (LPDMA_PRIVCFGR.PRIVx).
This register controls the transfer of a channel x at a block level.
This register must be written when LPDMA_CxCR.EN = 0.
This register is read-only when LPDMA_CxCR.EN = 1.
This register must be written when channel x is completed (then the hardware has deasserted LPDMA_CxCR.EN). A channel transfer can be completed and programmed at
different levels: block, or LLI or full linked-list.
In linked-list mode, during the link transfer:
•

if LPDMA_CxLLR.UB1 = 1, this register is automatically updated by DMA from the next
LLI in memory.

•

If LPDMA_CxLLR.UB1 = 0 and if there is at least one linked-list register to be updated
from the next LLI in memory, this register is automatically and internally restored with
the programmed values for the field BNDT[15:0].

•

If all the update bits LPDMA_CxLLR.Uxx are null and if LPDMA_CxLLR.LA[15:0] # 0,
the current LLI is the last one and is continuously executed: this register is
automatically and internally restored with the programmed value for BNDT[15:0] after
each execution of this final LLI

•

If LPDMA_CxLLR = 0, this register and BNDT[15:0] are kept as null, channel x is
completed.

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

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

BNDT[15:0]
rw

rw

Bits 31:16 Reserved, must be kept at reset value.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Low-power direct memory access controller (LPDMA)

Bits 15:0 BNDT[15:0]: block number of data bytes to transfer from the source
Block size transferred from the source. When the channel is enabled, this field becomes
read-only and is decremented, indicating the remaining number of data items in the current
source block to be transferred. BNDT[15:0] is programmed in number of bytes, maximum
source block size is 64 Kbytes -1.
Once the last data transfer is completed (BNDT[15:0] = 0):
- if LPDMA_CxLLR.UB1 = 1, this field is updated by the LLI in the memory.
- if LPDMA_CxLLR.UB1 = 0 and if there is at least one non null Uxx update bit, this field is
internally restored to the programmed value.
- if all LPDMA_CxLLR.Uxx = 0 and if LPDMA_CxLLR.LA[15:0] = 0, this field is internally
restored to the programmed value (infinite/continuous last LLI).
- if LPDMA_CxLLR = 0, this field is kept as zero following the last LLI data transfer.
Note: A non-null source block size must be a multiple of the source data width (BNDT[2:0]
versus LPDMA_CxTR1.SDW_LOG2[1:0]). Else a user setting error is reported and
none transfer is issued.

18.8.13

LPDMA channel x source address register (LPDMA_CxSAR)
Address offset: 0x09C + 0x80 * x (x = 0 to 3)
Reset value: 0x0000 0000
This register is secure or nonsecure depending on the secure state of channel x
(LPDMA_SECCFGR.SECx), and privileged or unprivileged, depending on the privileged
state of channel x (LPDMA_PRIVCFGR.PRIVx).
This register configures the source start address of a transfer.
This register must be written when LPDMA_CxCR.EN = 0.
This register is read-only when LPDMA_CxCR.EN = 1, and continuously updated by
hardware, in order to reflect the address of the next single transfer from the source.
This register must be written when the channel is completed (then the hardware has
deasserted LPDMA_CxCR.EN). A channel transfer can be completed and programmed at
different levels: block or LLI or full linked-list.
In linked-list mode, during the link transfer, this register is automatically updated by LPDMA
from the memory if LPDMA_CxLLR.USA = 1.

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

SA[31:16]
rw

rw

rw

rw

rw

rw

rw

rw

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

7

6

5

4

3

2

1

0

rw

rw

rw

rw

rw

rw

rw

SA[15:0]
rw

rw

rw

rw

rw

rw

rw

rw

rw

RM0456 Rev 6

<!-- pagebreak -->

821

Low-power direct memory access controller (LPDMA)

RM0456

Bits 31:0 SA[31:0]: source address
This field is the pointer to the address from which the next data is read.
During the channel activity, depending on the source addressing mode (LPDMA_CxTR1.SINC), this
field is either kept fixed or incremented by the data width (LPDMA_CxTR1.SDW_LOG2[1:0]) after
each single source data, reflecting the next address from which data is read.
In linked-list mode, after a LLI data transfer is completed, this register is automatically updated by
LPDMA from the memory, provided the LLI is set with LPDMA_CxLLR.USA = 1.
Note: A source address must be aligned with the programmed data width of a source single (SA[32:0]
versus LPDMA_CxTR1.SDW_LOG2[1:0]). Else, a user setting error is reported and no transfer
is issued.

18.8.14

LPDMA channel x destination address register (LPDMA_CxDAR)
Address offset: 0x0A0 + 0x80 * x (x = 0 to 3)
Reset value: 0x0000 0000
This register is secure or nonsecure depending on the secure state of channel x
(LPDMA_SECCFGR.SECx), and privileged or unprivileged, depending on the privileged
state of channel x (LPDMA_PRIVCFGR.PRIVx).
This register configures the destination start address of a transfer.
This register must be written when LPDMA_CxCR.EN = 0.
This register is read-only when LPDMA_CxCR.EN = 1, and continuously updated by
hardware, in order to reflect the address of the next single transfer to the destination.
This register must be written when the channel is completed (then the hardware has
deasserted LPDMA_CxCR.EN). A channel transfer can be completed and programmed at
different levels: block or LLI or full linked-list.
In linked-list mode, during the link transfer, this register is automatically updated by LPDMA
from the memory if LPDMA_CxLLR.UDA = 1.

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

DA[31:16]
rw

rw

rw

rw

rw

rw

rw

rw

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

7

6

5

4

3

2

1

0

rw

rw

rw

rw

rw

rw

rw

DA[15:0]
rw

rw

rw

rw

rw

rw

rw

rw

rw

Bits 31:0 DA[31:0]: destination address
This field is the pointer to the address from which the next data is written.
During the channel activity, depending on the destination addressing mode (LPDMA_CxTR1.DINC),
this field is kept fixed or incremented by the data width (LPDMA_CxTR1.DDW_LOG2[1:0]) after
each single destination data, reflecting the next address from which data is written.
In linked-list mode, after a LLI data transfer is completed, this register is automatically updated by
DMA from the memory, provided the LLI is set with LPDMA_CxLLR.UDA = 1.
Note: A destination address must be aligned with the programmed data width of a destination single
(DA[2:0] versus LPDMA_CxTR1.DDW_LOG2[1:0]). Else, a user setting error is reported and no
transfer is issued.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Low-power direct memory access controller (LPDMA)

18.8.15

LPDMA channel x linked-list address register (LPDMA_CxLLR)
Address offset: 0x0CC + 0x80 * x (x = 0 to 3)
Reset value: 0x0000 0000
This register is secure or nonsecure depending on the secure state of channel x
(LPDMA_SECCFGR.SECx), and privileged or unprivileged, depending on the privileged
state of channel x (LPDMA_PRIVCFGR.PRIVx).
This register configures the data structure of the next LLI in the memory and its address
pointer. A channel transfer is completed when this register is null.
This register must be written when LPDMA_CxCR.EN = 0.
This register is read-only when LPDMA_CxCR.EN = 1.
This register must be written when the channel is completed (then the hardware has
deasserted LPDMA_CxCR.EN). A channel transfer can be completed and programmed at
different levels: block or LLI or full linked-list.
In linked-list mode, during the link transfer, this register is automatically updated by LPDMA
from the memory if LPDMA_CxLLR.ULL = 1.

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

UT1

UT2

UB1

USA

UDA

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

ULL

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

rw

rw

rw

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

rw

rw

rw

rw

rw

rw

rw

rw

LA[15:2]
rw

Bit 31 UT1: Update LPDMA_CxTR1 from memory
This bit controls the update of the LPDMA_CxTR1 register from the memory during the link transfer.
0: no LPDMA_CxTR1 update
1: LPDMA_CxTR1 update
Bit 30 UT2: Update LPDMA_CxTR2 from memory
This bit controls the update of the LPDMA_CxTR2 register from the memory during the link transfer.
0: no LPDMA_CxTR2 update
1: LPDMA_CxTR2 update
Bit 29 UB1: Update LPDMA_CxBR1 from memory
This bit controls the update of the LPDMA_CxBR1 register from the memory during the link transfer.
0: no LPDMA_CxBR1 update from memory and internally restored to the previous programmed
value
1: LPDMA_CxBR1 update
Bit 28 USA: update LPDMA_CxSAR from memory
This bit controls the update of the LPDMA_CxSAR register from the memory during the link transfer.
0: no LPDMA_CxSAR update
1: LPDMA_CxSAR update
Bit 27 UDA: Update LPDMA_CxDAR register from memory
This bit is used to control the update of the LPDMA_CxDAR register from the memory during the link
transfer.
0: no LPDMA_CxDAR update
1: LPDMA_CxDAR update

RM0456 Rev 6

<!-- pagebreak -->

