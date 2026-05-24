763

General purpose direct memory access controller (GPDMA)

RM0456

Bit 11 ULEF: update link transfer error flag
0: no update link transfer error event
1: a master bus error event occurred while updating a linked-list register from memory
Bit 10 DTEF: data transfer error flag
0: no data transfer error event
1: a master bus error event occurred on a data transfer
Bit 9 HTF: half transfer flag
0: no half transfer event
1: a half transfer event occurred
A half transfer event is either a half block transfer or a half 2D/repeated block transfer,
depending on the transfer complete event mode (GPDMA_CxTR2.TCEM[1:0]).
A half block transfer occurs when half of the bytes of the source block size (rounded up
integer of GPDMA_CxBR1.BNDT[15:0]/2) has been transferred to the destination.
A half 2D/repeated block transfer occurs when half of the repeated blocks (rounded up
integer of (GPDMA_CxBR1.BRC[10:0] + 1) / 2)) has been transferred to the destination.
Bit 8 TCF: transfer complete flag
0: no transfer complete event
1: a transfer complete event occurred
A transfer complete event is either a block transfer complete, a 2D/repeated block transfer
complete, or a LLI transfer complete including the upload of the next LLI if any, or the full
linked-list completion, depending on the transfer complete event mode
(GPDMA_CxTR2.TCEM[1:0]).
Bits 7:1 Reserved, must be kept at reset value.
Bit 0 IDLEF: idle flag
0: channel not in idle state
1: channel in idle state
This idle flag is deasserted by hardware when the channel is enabled
(GPDMA_CxCR.EN = 1) with a valid channel configuration (no USEF to be immediately
reported).
This idle flag is asserted after hard reset or by hardware when the channel is back in idle
state (in suspended or disabled state).

17.8.9

GPDMA channel x control register (GPDMA_CxCR)
Address offset: 0x64 + 0x80 * x (x = 0 to 15)
Reset value: 0x0000 0000
This register is secure or nonsecure depending on the secure state of channel x
(GPDMA_SECCFGR.SECx), and privileged or unprivileged, depending on the privileged
state of the channel x (GPDMA_PRIVCFGR.PRIVx).
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

LAP

LSM

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

<!-- pagebreak -->

RM0456 Rev 6

RM0456

General purpose direct memory access controller (GPDMA)

Bits 31:24 Reserved, must be kept at reset value.
Bits 23:22 PRIO[1:0]: priority level of the channel x GPDMA transfer versus others
00: low priority, low weight
01: low priority, mid weight
10: low priority, high weight
11: high priority
Note: This bit must be written when EN = 0. This bit is read-only when EN = 1.
Bits 21:18 Reserved, must be kept at reset value.
Bit 17 LAP: linked-list allocated port
This bit is used to allocate the master port for the update of the GPDMA linked-list registers
from the memory.
0: port 0 (AHB) allocated
1: port 1 (AHB) allocated
Note: This bit must be written when EN = 0. This bit is read-only when EN = 1.
Bit 16 LSM: Link step mode
0: channel executed for the full linked-list and completed at the end of the last LLI
(GPDMA_CxLLR = 0). The 16 low-significant bits of the link address are null (LA[15:0] = 0)
and all the update bits are null (UT1 = UB1 = UT2 = USA = UDA = ULL = 0 and
UT3 = UB2 = 0). Then GPDMA_CxBR1.BNDT[15:0] = 0 and GPDMA_CxBR1.BRC[10:0] = 0.
1: channel executed once for the current LLI
First the (possible 1D/repeated) block transfer is executed as defined by the current internal
register file until GPDMA_CxBR1.BNDT[15:0] = 0 and GPDMA_CxBR1.BRC[10:0] = 0.
Secondly the next linked-list data structure is conditionally uploaded from memory as defined
by GPDMA_CxLLR. Then channel execution is completed.
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
Bit 11 ULEIE: update link transfer error interrupt enable
0: interrupt disabled
1: interrupt enabled
Bit 10 DTEIE: data transfer error interrupt enable
0: interrupt disabled
1: interrupt enabled
Bit 9 HTIE: half transfer complete interrupt enable
0: interrupt disabled
1: interrupt enabled

RM0456 Rev 6

<!-- pagebreak -->

763

General purpose direct memory access controller (GPDMA)

RM0456

Bit 8 TCIE: transfer complete interrupt enable
0: interrupt disabled
1: interrupt enabled
Bits 7:3 Reserved, must be kept at reset value.
Bit 2 SUSP: suspend
Writing 1 into the field RESET (bit 1) causes the hardware to de-assert this bit, whatever is
written into this bit 2. Else:
Software must write 1 in order to suspend an active channel (channel with an ongoing
GPDMA transfer over its master ports).
The software must write 0 in order to resume a suspended channel, following the
programming sequence detailed in Figure 53.
0: write: resume channel, read: channel not suspended
1: write: suspend channel, read: channel suspended.
Bit 1 RESET: reset
This bit is write only. Writing 0 has no impact. Writing 1 implies the reset of the following: the
FIFO, the channel internal state, SUSP and EN bits (whatever is written receptively in bit 2
and bit 0).
The reset is effective when the channel is in steady state, meaning one of the following:
- active channel in suspended state (GPDMA_CxSR.SUSPF = 1 and
GPDMA_CxSR.IDLEF = GPDMA_CxCR.EN = 1)
- channel in disabled state (GPDMA_CxSR.IDLEF = 1 and GPDMA_CxCR.EN = 0).
After writing a RESET, to continue using this channel, the user must explicitly reconfigure the
channel including the hardware-modified configuration registers (GPDMA_CxBR1,
GPDMA_CxSAR, and GPDMA_CxDAR) before enabling again the channel
(see the programming sequence in Figure 54).
0: no channel reset
1: channel reset
Bit 0 EN: enable
Writing 1 into the field RESET (bit 1) causes the hardware to de-assert this bit, whatever is
written into this bit 0. Else:
this bit is deasserted by hardware when there is a transfer error (master bus error or user
setting error) or when there is a channel transfer complete (channel ready to be configured,
for example if LSM = 1 at the end of a single execution of the LLI).
Else, this bit can be asserted by software.
Writing 0 into this EN bit is ignored.
0: write: ignored, read: channel disabled
1: write: enable channel, read: channel enabled

17.8.10

GPDMA channel x transfer register 1 (GPDMA_CxTR1)
Address offset: 0x90 + 0x80 * x (x = 0 to 15)
Reset value: 0x0000 0000
This register is secure or nonsecure depending on the secure state of channel x
(GPDMA_SECCFGR.SECx) except for secure DSEC and SSEC, privileged or
non-privileged, depending on the privileged state of the channel x in
GPDMA_PRIVCFGR.PRIVx.
This register controls the transfer of a channel x.
This register must be written when GPDMA_CxCR.EN = 0.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

General purpose direct memory access controller (GPDMA)
This register is read-only when GPDMA_CxCR.EN = 1.
This register must be written when the channel is completed. Then the hardware has
deasserted GPDMA_CxCR.EN). A channel transfer can be completed and programmed at
different levels: block, 2D/repeated block, LLI or full linked-list.
In linked-list mode, during the link transfer, this register is automatically updated by GPDMA
from the memory if GPDMA_CxLLR.UT1 = 1.

31

30

29

28

27

26

DSEC

DAP

Res.

Res.

DHX

DBX

rw

rw

rw

rw

rw

rw

9

8

15

14

13

12

11

10

SSEC

SAP

SBX

PAM[1:0]

Res.

rw

rw

rw

rw

rw

25

24

23

22

21

20

DBL_1[5:0]
rw

rw

rw

rw

7

6

5

4

SBL_1[5:0]
rw

rw

rw

rw

rw

rw

19

18

DINC

Res.

rw
3

2

SINC

Res.

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

Bit 31 DSEC: security attribute of the GPDMA transfer to the destination
If GPDMA_SECCFGR.SECx = 1 and the access is secure:
0: GPDMA transfer nonsecure
1: GPDMA transfer secure
This is a secure register bit. This bit can only be read by a secure software. This bit must be
written by a secure software when GPDMA_SECCFGR.SECx = 1. A secure write is ignored
when GPDMA_SECCFGR.SECx = 0.
When GPDMA_SECCFGR.SECx is deasserted, this DSEC bit is also deasserted by
hardware (on a secure reconfiguration of the channel as nonsecure), and the GPDMA
transfer to the destination is nonsecure.
Bit 30 DAP: destination allocated port
This bit is used to allocate the master port for the destination transfer
0: port 0 (AHB) allocated
1: port 1 (AHB) allocated
Note: This bit must be written when EN = 0. This bit is read-only when EN = 1.
Bits 29:28 Reserved, must be kept at reset value.
Bit 27 DHX: destination half-word exchange
If the destination data size is shorter than a word, this bit is ignored.
If the destination data size is a word:
0: no halfword-based exchanged within word
1: the two consecutive (post PAM) half-words are exchanged in each destination word.
Bit 26 DBX: destination byte exchange
If the destination data size is a byte, this bit is ignored.
If the destination data size is not a byte:
0:no byte-based exchange within half-word
1: the two consecutive (post PAM) bytes are exchanged in each destination half-word.

RM0456 Rev 6

<!-- pagebreak -->

763

General purpose direct memory access controller (GPDMA)

RM0456

Bits 25:20 DBL_1[5:0]: destination burst length minus 1, between 0 and 63
The burst length unit is one data named beat within a burst. If DBL_1[5:0] = 0, the burst can
be named as single. Each data/beat has a width defined by the destination data width
DDW_LOG2[1:0].
Note: If a burst transfer crossed a 1-Kbyte address boundary on a AHB transfer, the GPDMA
modifies and shortens the programmed burst into singles or bursts of lower length, to be
compliant with the AHB protocol.
If a burst transfer is of length greater than the FIFO size of the channel x, the GPDMA
modifies and shortens the programmed burst into singles or bursts of lower length, to be
compliant with the FIFO size. Transfer performance is lower, with GPDMA re-arbitration
between effective and lower singles/bursts, but the data integrity is guaranteed.
Bit 19 DINC: destination incrementing burst
0: fixed burst
1: contiguously incremented burst
The destination address, pointed by GPDMA_CxDAR, is kept constant after a burst
beat/single transfer, or is incremented by the offset value corresponding to a contiguous data
after a burst beat/single transfer.
Bit 18 Reserved, must be kept at reset value.
Bits 17:16 DDW_LOG2[1:0]: binary logarithm of the destination data width of a burst, in bytes
00: byte
01: half-word (2 bytes)
10: word (4 bytes)
11: user setting error reported and no transfer issued
Note: A destination burst transfer must have an aligned address with its data width (start
address GPDMA_CxDAR[2:0] and if present address offset GPDMA_CxTR3.DAO[2:0],
versus DDW_LOG2[1:0]). Otherwise a user setting error is reported and no transfer is
issued.
When configured in packing mode (PAM[1] = 1 and destination data width different from
source data width), a source block size must be a multiple of the destination data width
(see GPDMA_CxBR1.BNDT[2:0] versus DDW_LOG2[1:0]). Else a user setting error is
reported and none transfer is issued.
Setting a 8-byte data width causes a user setting error to be reported and none transfer
is issued.
Bit 15 SSEC: security attribute of the GPDMA transfer from the source
If GPDMA_SECCFGR.SECx = 1 and the access is secure:
0: GPDMA transfer nonsecure
1: GPDMA transfer secure
This is a secure register bit. This bit can only be read by a secure software. This bit must be
written by a secure software when GPDMA_SECCFGR.SECx = 1. A secure write is ignored
when GPDMA_SECCFGR.SECx = 0.
When GPDMA_SECCFGR.SECx is deasserted, this SSEC bit is also deasserted by
hardware (on a secure reconfiguration of the channel as nonsecure), and the GPDMA
transfer from the source is nonsecure.
Bit 14 SAP: source allocated port
This bit is used to allocate the master port for the source transfer
0: port 0 (AHB) allocated
1: port 1 (AHB) allocated
Note: This bit must be written when EN = 0. This bit is read-only when EN = 1.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

General purpose direct memory access controller (GPDMA)

Bit 13 SBX: source byte exchange within the unaligned half-word of each source word
If the source data width is shorter than a word, this bit is ignored.
If the source data width is a word:
0:no byte-based exchange within the unaligned half-word of each source word
1: the two consecutive bytes within the unaligned half-word of each source word are
exchanged.
Bits 12:11 PAM[1:0]: padding/alignment mode
If DDW_LOG2[1:0] = SDW_LOG2[1:0]: if the data width of a burst destination transfer is
equal to the data width of a burst source transfer, these bits are ignored.
Else, in the following enumerated values, the condition PAM_1 is when destination data width
is higher than source data width, and the condition PAM_2 is when source data width is
higher than destination data width.
Condition: PAM_1
00: source data is transferred as right aligned, padded with 0s up to the destination data
width
01: source data is transferred as right aligned, sign extended up to the destination data width
10-11: successive source data are FIFO queued and packed at the destination data width, in
a left (LSB) to right (MSB) order (named little endian), before a destination transfer
Condition: PAM_2
00: source data is transferred as right aligned, left-truncated down to the destination data
width
01: source data is transferred as left-aligned, right-truncated down to the destination data
width
Note: 10-11: source data is FIFO queued and unpacked at the destination data width, to be
transferred in a left (LSB) to right (MSB) order (named little endian) to the destination
Bit 10 Reserved, must be kept at reset value.
Bits 9:4 SBL_1[5:0]: source burst length minus 1, between 0 and 63
The burst length unit is one data named beat within a burst. If SBL_1[5:0] = 0, the burst can
be named as single. Each data/beat has a width defined by the destination data width
SDW_LOG2[1:0].
Note: If a burst transfer crossed a 1-Kbyte address boundary on a AHB transfer, the GPDMA
modifies and shortens the programmed burst into singles or bursts of lower length, to be
compliant with the AHB protocol.
If a burst transfer is of length greater than the FIFO size of the channel x, the GPDMA
modifies and shortens the programmed burst into singles or bursts of lower length, to be
compliant with the FIFO size. Transfer performance is lower, with GPDMA re-arbitration
between effective and lower singles/bursts, but the data integrity is guaranteed.
Bit 3 SINC: source incrementing burst
0: fixed burst
1: contiguously incremented burst
The source address, pointed by GPDMA_CxSAR, is kept constant after a burst beat/single
transfer or is incremented by the offset value corresponding to a contiguous data after a burst
beat/single transfer.
Bit 2 Reserved, must be kept at reset value.

RM0456 Rev 6

<!-- pagebreak -->

763

General purpose direct memory access controller (GPDMA)

RM0456

Bits 1:0 SDW_LOG2[1:0]: binary logarithm of the source data width of a burst in bytes
00: byte
01: half-word (2 bytes)
10: word (4 bytes)
11: user setting error reported and no transfer issued
Note: A source block size must be a multiple of the source data width
(GPDMA_CxBR1.BNDT[2:0] versus SDW_LOG2[1:0]). Otherwise, a user setting error
is reported and no transfer is issued.
A source burst transfer must have an aligned address with its data width (start address
GPDMA_CxSAR[2:0] versus SDW_LOG2[1:0]). Otherwise, a user setting error is
reported and none transfer is issued.
Setting a 8-byte data width causes a user setting error to be reported and no transfer is
issued.

17.8.11

GPDMA channel x transfer register 2 (GPDMA_CxTR2)
Address offset: 0x94 + 0x80 * x (x = 0 to 15)
Reset value: 0x0000 0000
This register is secure or nonsecure depending on the secure state of channel x
(GPDMA_SECCFGR.SECx), and privileged or unprivileged, depending on the privileged
state of channel x (GPDMA_PRIVCFGR.PRIVx).
This register controls the transfer of a channel x.
This register must be written when GPDMA_CxCR.EN = 0.
This register is read-only when GPDMA_CxCR.EN = 1.
This register must be written when the channel is completed (the hardware deasserted
GPDMA_CxCR.EN). A channel transfer can be completed and programmed at different
levels: block or LLI or full linked-list.
In linked-list mode, during the link transfer, this register is automatically updated by GPDMA
from the memory, if GPDMA_CxLLR.UT2 = 1.

31

30

29

28

27

26

25

24

23

TCEM[1:0]

Res.

Res.

Res.

Res.

TRIGPOL[1:0]

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

BREQ

DREQ

SWRE
Q

Res.

Res.

rw

rw

rw

rw

rw

15

14

TRIGM[1:0]
rw

<!-- pagebreak -->

rw

Res.

Res.

22

20

19

18

17

16

TRIGSEL[6:0]
rw

rw

rw

rw

rw

rw

rw

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

REQSEL[6:0]
rw

RM0456 Rev 6

21

rw

rw

rw

RM0456

General purpose direct memory access controller (GPDMA)

Bits 31:30 TCEM[1:0]: transfer complete event mode
These bits define the transfer granularity for the transfer complete and half transfer complete
events generation.
00: at block level (when GPDMA_CxBR1.BNDT[15:0] = 0): the complete (and the half)
transfer event is generated at the (respectively half of the) end of a block.
Note: If the initial LLI0 data transfer is null/void (directly programmed by the internal register
file with GPDMA_CxBR1.BNDT[15:0] = 0), then neither the complete transfer event nor
the half transfer event is generated.
01: channel x (x = 0 to 11), same as 00, channel x (x =12 to 15), at 2D/repeated block level
(when GPDMA_CxBR1.BRC[10:0] = 0 and GPDMA_CxBR1.BNDT[15:0] = 0). The complete
(and the half) transfer event is generated at the end (respectively half of the end) of the
2D/repeated block.
Note: If the initial LLI0 data transfer is null/void (directly programmed by the internal register
file with GPDMA_CxBR1.BNDT[15:0] = 0), then neither the complete transfer event nor
the half transfer event is generated.
10: at LLI level: the complete transfer event is generated at the end of the LLI transfer,
including the update of the LLI if any. The half transfer event is generated at the half of the
LLI data transfer The LLI data transfer is a block transfer or a 2D/repeated block transfer for
channel x (x =12 to 15), if any data transfer.
Note: If the initial LLI0 data transfer is null/void (directly programmed by the internal register
file with GPDMA_CxBR1.BNDT[15:0] = 0), then the half transfer event is not generated,
and the transfer complete event is generated when is completed the loading of the LLI 1.
11: at channel level: the complete transfer event is generated at the end of the last LLI
transfer. The half transfer event is generated at the half of the data transfer of the last LLI.
The last LLI updates the link address GPDMA_CxLLR.LA[15:2] to zero and clears all the
GPDMA_CxLLR update bits (UT1, UT2, UB1, USA, UDA and ULL, plus UT3 and UB2). If the
channel transfer is continuous/infinite, no event is generated.
Bits 29:26 Reserved, must be kept at reset value.
Bits 25:24 TRIGPOL[1:0]: trigger event polarity
These bits define the polarity of the selected trigger event input defined by TRIGSEL[6:0].
00: no trigger (masked trigger event)
01: trigger on the rising edge
10: trigger on the falling edge
11: same as 00
Bit 23 Reserved, must be kept at reset value.
Bits 22:16 TRIGSEL[6:0]: trigger event input selection
These bits select the trigger event input of the GPDMA transfer (as per Section 17.3.5),
with an active trigger event if TRIGPOL[1:0] ≠ 00.

RM0456 Rev 6

<!-- pagebreak -->

763

General purpose direct memory access controller (GPDMA)

RM0456

Bits 15:14 TRIGM[1:0]: trigger mode
These bits define the transfer granularity for its conditioning by the trigger.
If the channel x is enabled (GPDMA_CxCR.EN asserted) with TRIGPOL[1:0] = 00 or 11,
these TRIGM[1:0] bits are ignored.
Else, a GPDMA transfer is conditioned by at least one trigger hit:
00: at block level: the first burst read of each block transfer is conditioned by one hit trigger
(channel x (x = 12 to 15), for each block if a 2D/repeated block is configured with
GPDMA_CxBR1.BRC[10:0] ≠ 0).
01: channel x (x = 0 to 11), same as 00; channel x (x = 12 to 15), at 2D/repeated block level.
The first burst read of a 2D/repeated block transfer is conditioned by one hit trigger.
10: at link level: a LLI link transfer is conditioned by one hit trigger. The LLI data transfer
(if any) is not conditioned.
11: at programmed burst level: If SWREQ = 1, each programmed burst read is conditioned
by one hit trigger. If SWREQ = 0, each programmed burst that is requested by the selected
peripheral, is conditioned by one hit trigger.
– If the peripheral is programmed as a source (DREQ = 0) of the LLI data transfer, each
programmed burst read is conditioned.
– If the peripheral is programmed as a destination (DREQ = 1) of the LLI data transfer, each
programmed burst write is conditioned. The first memory burst read of a (possibly
2D/repeated) block, also named as the first ready FIFO-based source burst, is gated by the
occurrence of both the hardware request and the first trigger hit.
The GPDMA monitoring of a trigger for channel x is started when the channel is
enabled/loaded with a new active trigger configuration: rising or falling edge on a selected
trigger (TRIGPOL[1:0] = 01 or respectively TRIGPOL[1:0] = 10).
The monitoring of this trigger is kept active during the triggered and uncompleted (data or
link) transfer; and if a new trigger is detected then, this hit is internally memorized to grant the
next transfer, as long as the defined rising or falling edge is not modified, and the
TRIGSEL[6:0] is not modified, and the channel is enabled.
Transferring a next LLIn+1 that updates the GPDMA_CxTR2 with a new value for any of
TRIGSEL[6:0] or TRIGPOL[1:0], resets the monitoring, trashing the memorized hit of the
formerly defined LLIn trigger.
After a first new trigger hitn+1 is memorized, if another second trigger hitn+2 is detected and if
the hitn triggered transfer is still not completed, hitn+2 is lost and not memorized. A trigger
overrun flag is reported (GPDMA_CxSR.TOF = 1), and an interrupt is generated if enabled
(GPDMA_CxCR.TOIE = 1). The channel is not automatically disabled by hardware due to a
trigger overrun.
Note: When the source block size is not a multiple of the source burst size and is a multiple of
the source data width, then the last programmed source burst is not completed and is
internally shorten to match the block size. In this case, if TRIGM[1:0] = 11 and
(SWREQ = 1 or (SWREQ = 0 and DREQ = 0)), the shortened burst transfer (by singles
or/and by bursts of lower length) is conditioned once by the trigger.
When the programmed destination burst is internally shortened by singles or/and by
bursts of lower length (versus FIFO size, versus block size, 1-Kbyte boundary address
crossing): if the trigger is conditioning the programmed destination burst
(if TRIGM[1:0] = 11 and SWREQ = 0 and DREQ = 1), this shortened destination burst
transfer is conditioned once by the trigger.
Bits 13:12 Reserved, must be kept at reset value.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

General purpose direct memory access controller (GPDMA)

Bit 11 BREQ: Block hardware request
If the channel x is activated (GPDMA_CxCR.EN asserted) with SWREQ = 1 (software
request for a memory-to-memory transfer), this bit is ignored. Else:
0: the selected hardware request is driven by a peripheral with a hardware
request/acknowledge protocol at a burst level.
1: the selected hardware request is driven by a peripheral with a hardware
request/acknowledge protocol at a block level (see Section 17.3.3).
Bit 10 DREQ: destination hardware request
This bit is ignored if channel x is activated (GPDMA_CxCR.EN asserted) with SWREQ = 1
(software request for a memory-to-memory transfer). Else:
0: selected hardware request driven by a source peripheral (request signal taken into
account by the GPDMA transfer scheduler over the source/read port)
1: selected hardware request driven by a destination peripheral (request signal taken into
account by the GPDMA transfer scheduler over the destination/write port)
Note:
Bit 9 SWREQ: software request
This bit is internally taken into account when GPDMA_CxCR.EN is asserted.
0: no software request. The selected hardware request REQSEL[6:0] is taken into account.
1: software request for a memory-to-memory transfer. The default selected hardware request
as per REQSEL[6:0] is ignored.
Bits 8:7 Reserved, must be kept at reset value.
Bits 6:0 REQSEL[6:0]: GPDMA hardware request selection
These bits are ignored if channel x is activated (GPDMA_CxCR.EN asserted) with
SWREQ = 1 (software request for a memory-to-memory transfer). Else, the selected
hardware request is internally taken into account as per Section 17.3.3.
Caution: The user must not assign a same input hardware request (same REQSEL[6:0]
value) to different active GPDMA channels (GPDMA_CxCR.EN = 1 and
GPDMA_CxTR2.SWREQ = 0 for these channels). GPDMA is not intended to
hardware support the case of simultaneous enabled channels incorrectly configured
with a same hardware peripheral request signal, and there is no user setting error
reporting.

RM0456 Rev 6

<!-- pagebreak -->

763

General purpose direct memory access controller (GPDMA)

17.8.12

RM0456

GPDMA channel x block register 1 (GPDMA_CxBR1)
Address offset: 0x98 + 0x80 * x (x = 0 to 11)
Reset value: 0x0000 0000
This register is secure or nonsecure depending on the secure state of channel x
(GPDMA_SECCFGR.SECx), and privileged or unprivileged, depending on the privileged
state of channel x (GPDMA_PRIVCFGR.PRIVx).
This register controls the transfer of a channel x at a block level.
This register must be written when GPDMA_CxCR.EN = 0.
This register is read-only when GPDMA_CxCR.EN = 1.
This register must be written when channel x is completed (then the hardware has
deasserted GPDMA_CxCR.EN). A channel transfer can be completed and programmed at
different levels: block, or LLI or full linked-list.
In linked-list mode, during the link transfer:
•

if GPDMA_CxLLR.UB1 = 1, this register is automatically updated by the GPDMA from
the next LLI in memory.

•

If GPDMA_CxLLR.UB1 = 0 and if there is at least one linked-list register to be updated
from the next LLI in memory, this register is automatically and internally restored with
the programmed value for the field BNDT[15:0].

•

If all the update bits GPDMA_CxLLR.Uxx are null and if GPDMA_CxLLR.LA[15:0] ≠ 0,
the current LLI is the last one and is continuously executed: this register is
automatically and internally restored with the programmed value for BNDT[15:0] after
each execution of this final LLI

•

If GPDMA_CxLLR = 0, this register and BNDT[15:0] are kept as null, channel x is
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

General purpose direct memory access controller (GPDMA)

Bits 15:0 BNDT[15:0]: block number of data bytes to transfer from the source
Block size transferred from the source. When the channel is enabled, this field becomes
read-only and is decremented, indicating the remaining number of data items in the current
source block to be transferred. BNDT[15:0] is programmed in number of bytes, maximum
source block size is 64 Kbytes -1.
Once the last data transfer is completed (BNDT[15:0] = 0):
- if GPDMA_CxLLR.UB1 = 1, this field is updated by the LLI in the memory.
- if GPDMA_CxLLR.UB1 = 0 and if there is at least one non null Uxx update bit, this field is
internally restored to the programmed value.
- if all GPDMA_CxLLR.Uxx = 0 and if GPDMA_CxLLR.LA[15:0] = 0, this field is internally
restored to the programmed value (infinite/continuous last LLI).
- if GPDMA_CxLLR = 0, this field is kept as zero following the last LLI data transfer.
Note: A non-null source block size must be a multiple of the source data width (BNDT[2:0]
versus GPDMA_CxTR1.SDW_LOG2[1:0]). Else a user setting error is reported and no
transfer is issued.
When configured in packing mode (GPDMA_CxTR1.PAM[1] = 1 and destination data
width different from source data width), a non-null source block size must be a multiple
of the destination data width (BNDT[2:0] versus GPDMA_CxTR1.DDW_LOG2[1:0]).
Else a user setting error is reported and no transfer is issued.

17.8.13

GPDMA channel x alternate block register 1 (GPDMA_CxBR1)
Address offset: 0x98 + 0x80 * x (x = 12 to 15)
Reset value: 0x0000 0000
This register is secure or nonsecure depending on the secure state of channel x
(GPDMA_SECCFGR.SECx), and privileged or unprivileged, depending on the privileged
state of channel x (GPDMA_PRIVCFGR.PRIVx).
This register controls the transfer of a channel x at a block level.
This register must be written when GPDMA_CxCR.EN = 0.
This register is read-only when GPDMA_CxCR.EN = 1.
This register must be written when channel x is completed (then the hardware has
deasserted GPDMA_CxCR.EN). A channel transfer can be completed and programmed at
different levels: block, or LLI or full linked-list.
In linked-list mode, during the link transfer:
•

if GPDMA_CxLLR.UB1 = 1, this register is automatically updated by the GPDMA from
the next LLI in memory.

•

If GPDMA_CxLLR.UB1 = 0 and if there is at least one linked-list register to be updated
from the next LLI in memory, this register is automatically and internally restored with
the programmed value for the fields BNDT[15:0] and BRC[10:0].

•

If all the update bits GPDMA_CxLLR.Uxx are null and if GPDMA_CxLLR.LA[15:0] ≠ 0,
the current LLI is the last one and is continuously executed: this register is

RM0456 Rev 6

<!-- pagebreak -->

763

General purpose direct memory access controller (GPDMA)

RM0456

automatically and internally restored with the programmed value for the fields
BNDT[15:0] and BRC[10:0] after each execution of this final LLI
•

31

30

If GPDMA_CxLLR = 0, BNDT[15:0] and BRC[10:0] are kept as null, channel x is
completed.
29

BRDDE BRSDE
DDEC
C
C

28

27

SDEC

Res.

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

BRC[10:0]

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

Bit 31 BRDDEC: Block repeat destination address decrement
0: at the end of a block transfer, the GPDMA_CxDAR register is updated by adding the
programmed offset GPDMA_CxBR2.BRDAO to the current GPDMA_CxDAR value (current
destination address)
1: at the end of a block transfer, the GPDMA_CxDAR register is updated by subtracting the
programmed offset GPDMA_CxBR2.BRDAO from the current GPDMA_CxDAR value
(current destination address)
Note: On top of this increment/decrement (depending on BRDDEC), GPDMA_CxDAR is in the
same time also updated by the increment/decrement (depending on DDEC) of the
GPDMA_CxTR3.DAO value, as it is usually done at the end of each programmed
burst transfer.
Bit 30 BRSDEC: Block repeat source address decrement
0: at the end of a block transfer, the GPDMA_CxSAR register is updated by adding the
programmed offset GPDMA_CxBR2.BRSAO to the current GPDMA_CxSAR value (current
source address)
1: at the end of a block transfer, the GPDMA_CxSAR register is updated by subtracting the
programmed offset GPDMA_CxBR2.BRSAO from the current GPDMA_CxSAR value
(current source address)
Note: On top of this increment/decrement (depending on BRSDEC), GPDMA_CxSAR is in the
same time also updated by the increment/decrement (depending on SDEC) of the
GPDMA_CxTR3.SAO value, as it is done after any programmed burst transfer.
Bit 29 DDEC: destination address decrement
0: At the end of a programmed burst transfer to the destination, the GPDMA_CxDAR register
is updated by adding the programmed offset GPDMA_CxTR3.DAO to the current
GPDMA_CxDAR value (current destination address)
1: At the end of a programmed burst transfer to the destination, the GPDMA_CxDAR register
is updated by subtracting the programmed offset GPDMA_CxTR3.DAO to the current
GPDMA_CxDAR value (current destination address)
Bit 28 SDEC: source address decrement
0: At the end of a programmed burst transfer from the source, the GPDMA_CxSAR register is
updated by adding the programmed offset GPDMA_CxTR3.SAO to the current
GPDMA_CxSAR value (current source address)
1: At the end of a programmed burst transfer from the source, the GPDMA_CxSAR register is
updated by subtracting the programmed offset GPDMA_CxTR3.SAO to the current
GPDMA_CxSAR value (current source address)
Bit 27 Reserved, must be kept at reset value.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

General purpose direct memory access controller (GPDMA)

Bits 26:16 BRC[10:0]: Block repeat counter
This field contains the number of repetitions of the current block (0 to 2047).
When the channel is enabled, this field becomes read-only. After decrements, this field
indicates the remaining number of blocks, excluding the current one. This counter is
hardware decremented for each completed block transfer.
Once the last block transfer is completed (BRC[10:0] = BNDT[15:0] = 0):
– If GPDMA_CxLLR.UB1 = 1, all GPDMA_CxBR1 fields are updated by the next LLI in the
memory.
– If GPDMA_CxLLR.UB1 = 0 and if there is at least one not null Uxx update bit, this field is
internally restored to the programmed value.
– if all GPDMA_CxLLR.Uxx = 0 and if GPDMA_CxLLR.LA[15:0] ≠ 0, this field is internally
restored to the programmed value (infinite/continuous last LLI).
– if GPDMA_CxLLR = 0, this field is kept as zero following the last LLI and data transfer.
Bits 15:0 BNDT[15:0]: block number of data bytes to transfer from the source
Block size transferred from the source. When the channel is enabled, this field becomes
read-only and is decremented, indicating the remaining number of data items in the current
source block to be transferred. BNDT[15:0] is programmed in number of bytes, maximum
source block size is 64 Kbytes -1.
Once the last data transfer is completed (BNDT[15:0] = 0):
- if GPDMA_CxLLR.UB1 = 1, this field is updated by the LLI in the memory.
- if GPDMA_CxLLR.UB1 = 0 and if there is at least one not null Uxx update bit, this field is
internally restored to the programmed value.
- if all GPDMA_CxLLR.Uxx = 0 and if GPDMA_CxLLR.LA[15:0] ≠ 0, this field is internally
restored to the programmed value (infinite/continuous last LLI).
- if GPDMA_CxLLR = 0, this field is kept as zero following the last LLI data transfer.
Note: A non-null source block size must be a multiple of the source data width (BNDT[2:0]
versus GPDMA_CxTR1.SDW_LOG2[1:0]). Else a user setting error is reported and no
transfer is issued.
When configured in packing mode (GPDMA_CxTR1.PAM[1] = 1 and destination data
width different from source data width), a non-null source block size must be a multiple
of the destination data width (BNDT[2:0] versus GPDMA_CxTR1.DDW_LOG2[1:0]).
Else a user setting error is reported and no transfer is issued.

RM0456 Rev 6

<!-- pagebreak -->

763

General purpose direct memory access controller (GPDMA)

17.8.14

RM0456

GPDMA channel x source address register (GPDMA_CxSAR)
Address offset: 0x9C + 0x80 * x (x = 0 to 15)
Reset value: 0x0000 0000
This register is secure or nonsecure depending on the secure state of channel x
(GPDMA_SECCFGR.SECx), and privileged or unprivileged, depending on the privileged
state of channel x (GPDMA_PRIVCFGR.PRIVx).
This register configures the source start address of a transfer.
This register must be written when GPDMA_CxCR.EN = 0.
This register is read-only when GPDMA_CxCR.EN = 1, and continuously updated by
hardware, in order to reflect the address of the next burst transfer from the source.
This register must be written when the channel is completed (then the hardware has
deasserted GPDMA_CxCR.EN). A channel transfer can be completed and programmed at
different levels: block, 2D/repeated block, LLI or full linked-list.
In linked-list mode, during the link transfer, this register is automatically updated by the
GPDMA from the memory if GPDMA_CxLLR.USA = 1.

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

rw

rw

rw

rw

rw

rw

rw

rw

SA[15:0]

<!-- pagebreak -->

rw

RM0456 Rev 6

RM0456

General purpose direct memory access controller (GPDMA)

Bits 31:0 SA[31:0]: source address
This field is the pointer to the address from which the next data is read.
During the channel activity, depending on the source addressing mode
(GPDMA_CxTR1.SINC), this field is kept fixed or incremented by the data width
(GPDMA_CxTR1.SDW_LOG2[1:0]) after each burst source data, reflecting the next address
from which data is read.
During the channel activity, this address is updated after each completed source burst,
consequently to:
– the programmed source burst; either in fixed addressing mode or in contiguous-data
incremented mode. If contiguously incremented (GPDMA_CxTR1.SINC = 1), then the
additional address offset value is the programmed burst size, as defined by
GPDMA_CxTR1.SBL_1[5:0] and GPDMA_CxTR1.SDW_LOG2[1:0]
– the additional source incremented/decremented offset value as programmed by
GPDMA_CxBR1.SDEC and GPDMA_CxTR3.SAO[12:0].
– once/if completed source block transfer, for a channel x with 2D addressing capability
(x = 12 to 15). additional block repeat source incremented/decremented offset value as
programmed by GPDMA_CxBR1.BRSDEC and GPDMA_CxBR2.BRSAO[15:0]
In linked-list mode, after a LLI data transfer is completed, this register is automatically
updated by GPDMA from the memory, provided the LLI is set with GPDMA_CxLLR.USA = 1.
Note: A source address must be aligned with the programmed data width of a source burst
(SA[2:0] versus GPDMA_CxTR1.SDW_LOG2[1:0]). Else, a user setting error is
reported and no transfer is issued.
When the source block size is not a multiple of the source burst size and is a multiple of
the source data width, the last programmed source burst is not completed and is
internally shorten to match the block size. In this case, the additional
GPDMA_CxTR3.SAO[12:0] is not applied.

RM0456 Rev 6

<!-- pagebreak -->

763

General purpose direct memory access controller (GPDMA)

17.8.15

RM0456

GPDMA channel x destination address register (GPDMA_CxDAR)
Address offset: 0xA0 + 0x80 * x (x = 0 to 15)
Reset value: 0x0000 0000
This register is secure or nonsecure depending on the secure state of channel x
(GPDMA_SECCFGR.SECx), and privileged or unprivileged, depending on the privileged
state of channel x (GPDMA_PRIVCFGR.PRIVx).
This register configures the destination start address of a transfer.
This register must be written when GPDMA_CxCR.EN = 0.
This register is read-only when GPDMA_CxCR.EN = 1, and continuously updated by
hardware, in order to reflect the address of the next burst transfer to the destination.
This register must be written when the channel is completed (then the hardware has
deasserted GPDMA_CxCR.EN). A channel transfer can be completed and programmed at
different levels: block, 2D/repeated block, LLI or full linked-list.
In linked-list mode, during the link transfer, this register is automatically updated by GPDMA
from the memory if GPDMA_CxLLR.UDA = 1.

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

rw

rw

rw

rw

rw

rw

rw

rw

DA[15:0]
rw

Bits 31:0 DA[31:0]: destination address
This field is the pointer to the address from which the next data is written.
During the channel activity, depending on the destination addressing mode
(GPDMA_CxTR1.DINC), this field is kept fixed or incremented by the data width
(GPDMA_CxTR1.DDW_LOG2[1:0]) after each burst destination data, reflecting the next
address from which data is written.
During the channel activity, this address is updated after each completed destination burst,
consequently to:
– the programmed destination burst; either in fixed addressing mode or in contiguous-data
incremented mode. If contiguously incremented (GPDMA_CxTR1.DINC = 1), then the
additional address offset value is the programmed burst size, as defined by
GPDMA_CxTR1.DBL_1[5:0] and GPDMA_CxTR1.DDW_LOG2[1:0]
– the additional destination incremented/decremented offset value as programmed by
GPDMA_CxBR1.DDEC and GPDMA_CxTR3.DAO[12:0].
– once/if completed destination block transfer, for a channel x with 2D addressing capability
(x = 12 to 15), the additional block repeat destination incremented/decremented offset value
as programmed by GPDMA_CxBR1.BRDDEC and GPDMA_CxBR2.BRDAO[15:0]
In linked-list mode, after a LLI data transfer is completed, this register is automatically
updated by the GPDMA from the memory, provided the LLI is set
with GPDMA_CxLLR.UDA = 1.
Note: A destination address must be aligned with the programmed data width of a destination
burst (DA[2:0] versus GPDMA_CxTR1.DDW_LOG2[1:0]). Else, a user setting error is
reported and no transfer is issued.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

General purpose direct memory access controller (GPDMA)

17.8.16

GPDMA channel x transfer register 3 (GPDMA_CxTR3)
Address offset: 0xA4 + 0x80 * x (x = 12 to 15)
Reset value: 0x0000 0000
This register is secure or nonsecure depending on the secure state of channel x
(GPDMA_SECCFGR.SECx), and privileged or unprivileged, depending on the privileged
state of channel x (GPDMA_PRIVCFGR.PRIVx).
This register controls the transfer of a channel x.
This register must be written when GPDMA_CxCR.EN = 0.
This register is read-only when GPDMA_CxCR.EN = 1.
This register must be written when the channel is completed (then the hardware has
deasserted GPDMA_CxCR.EN). A channel transfer can be completed and programmed at
different levels: block or LLI or full linked-list.
In linked-list mode, during the link transfer, this register is automatically updated by the
GPDMA from the memory if GPDMA_CxLLR.UT3 = 1.

31

30

29

Res.

Res.

Res.

15

14

13

Res.

Res.

Res.

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

DAO[12:0]
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

SAO[12:0]
rw

Bits 31:29 Reserved, must be kept at reset value.
Bits 28:16 DAO[12:0]: destination address offset increment
The destination address, pointed by GPDMA_CxDAR, is incremented or decremented
(depending on GPDMA_CxBR1.DDEC) by this offset DAO[12:0] for each programmed
destination burst. This offset is not including and is added to the programmed burst size
when the completed burst is addressed in incremented mode (GPDMA_CxTR1.DINC = 1).
Note: A destination address offset must be aligned with the programmed data width of a
destination burst (DAO[2:0] versus GPDMA_CxTR1.DDW_LOG2[1:0]). Else, a user
setting error is reported and no transfer is issued.
Bits 15:13 Reserved, must be kept at reset value.
Bits 12:0 SAO[12:0]: source address offset increment
The source address, pointed by GPDMA_CxSAR, is incremented or decremented
(depending on GPDMA_CxBR1.SDEC) by this offset SAO[12:0] for each programmed
source burst. This offset is not including and is added to the programmed burst size when the
completed burst is addressed in incremented mode (GPDMA_CxTR1.SINC = 1).
Note: A source address offset must be aligned with the programmed data width of a source
burst (SAO[2:0] versus GPDMA_CxTR1.SDW_LOG2[1:0]). Else a user setting error is
reported and none transfer is issued.
When the source block size is not a multiple of the destination burst size, and
is a multiple of the source data width, then the last programmed source burst is not
completed and is internally shorten to match the block size. In this case, the additional
GPDMA_CxTR3.SAO[12:0] is not applied.

RM0456 Rev 6

<!-- pagebreak -->

763

General purpose direct memory access controller (GPDMA)

17.8.17

RM0456

GPDMA channel x block register 2 (GPDMA_CxBR2)
Address offset: 0xA8 + 0x80 * x (x = 12 to 15)
Reset value: 0x0000 0000
This register is secure or nonsecure depending on the secure state of channel x
(GPDMA_SECCFGR.SECx), and privileged or unprivileged, depending on the privileged
state of channel x (GPDMA_PRIVCFGR.PRIVx).
This register controls the transfer of a channel x at a 2D/repeated block level.
This register must be written when GPDMA_CxCR.EN = 0.
This register is read-only when GPDMA_CxCR.EN = 1.
This register must be written when the channel is completed (then the hardware has
deasserted GPDMA_CxCR.EN). A channel transfer can be completed and programmed at
different levels: block, 2D/repeated block, LLI or full linked-list.
In linked-list mode, during the link transfer, this register is automatically updated by the
GPDMA from the memory if GPDMA_CxLLR.UB2 = 1.

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

BRDAO[15:0]
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

rw

rw

rw

rw

rw

rw

rw

BRSAO[15:0]
rw

rw

Bits 31:16 BRDAO[15:0]: Block repeated destination address offset
For a channel with 2D addressing capability, this field is used to update (by addition or
subtraction depending on GPDMA_CxBR1.BRDDEC) the current destination address
(GPDMA_CxDAR) at the end of a block transfer.
Note: A block repeated destination address offset must be aligned with the programmed data
width of a destination burst (BRDAO[2:0] versus GPDMA_CxTR1.DDW_LOG2[1:0]).
Else a user setting error is reported and no transfer is issued.
Bits 15:0 BRSAO[15:0]: Block repeated source address offset
For a channel with 2D addressing capability, this field is used to update (by addition or
subtraction depending on GPDMA_CxBR1.BRSDEC) the current source address
(GPDMA_CxSAR) at the end of a block transfer.
Note: A block repeated source address offset must be aligned with the programmed data
width of a source burst (BRSAO[2:0] versus GPDMA_CxTR1.SDW_LOG2[1:0]).
Else a user setting error is reported and no transfer is issued.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

General purpose direct memory access controller (GPDMA)

17.8.18

GPDMA channel x linked-list address register (GPDMA_CxLLR)
Address offset: 0xCC + 0x80 * x (x = 0 to 11)
Reset value: 0x0000 0000
This register is secure or nonsecure depending on the secure state of channel x
(GPDMA_SECCFGR.SECx), and privileged or unprivileged, depending on the privileged
state of channel x (GPDMA_PRIVCFGR.PRIVx).
This register configures the data structure of the next LLI in the memory and its address
pointer. A channel transfer is completed when this register is null.
This register must be written when GPDMA_CxCR.EN = 0.
This register is read-only when GPDMA_CxCR.EN = 1.
This register must be written when the channel is completed (then the hardware has
deasserted GPDMA_CxCR.EN). A channel transfer can be completed and programmed at
different levels: block or LLI or full linked-list.
In linked-list mode, during the link transfer, this register is automatically updated by the
GPDMA from the memory if GPDMA_CxLLR.ULL = 1.

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

Bit 31 UT1: Update GPDMA_CxTR1 from memory
This bit controls the update of GPDMA_CxTR1 from the memory during the link transfer.
0: no GPDMA_CxTR1 update
1: GPDMA_CxTR1 update
Bit 30 UT2: Update GPDMA_CxTR2 from memory
This bit controls the update of GPDMA_CxTR2 from the memory during the link transfer.
0: no GPDMA_CxTR2 update
1: GPDMA_CxTR2 update
Bit 29 UB1: Update GPDMA_CxBR1 from memory
This bit controls the update of GPDMA_CxBR1 from the memory during the link transfer.
If UB1 = 0 and if GPDMA_CxLLR ≠ 0, the linked-list is not completed.
GPDMA_CxBR1.BNDT[15:0] is then restored to the programmed value after data transfer is
completed and before the link transfer.
0: no GPDMA_CxBR1 update from memory (GPDMA_CxBR1.BNDT[15:0] restored if any
link transfer)
1: GPDMA_CxBR1 update
Bit 28 USA: update GPDMA_CxSAR from memory
This bit controls the update of GPDMA_CxSAR from the memory during the link transfer.
0: no GPDMA_CxSAR update
1: GPDMA_CxSAR update

RM0456 Rev 6

<!-- pagebreak -->

763

General purpose direct memory access controller (GPDMA)

RM0456

Bit 27 UDA: Update GPDMA_CxDAR register from memory
This bit is used to control the update of GPDMA_CxDAR from the memory during the link
transfer.
0: no GPDMA_CxDAR update
1: GPDMA_CxDAR update
Bits 26:17 Reserved, must be kept at reset value.
Bit 16 ULL: Update GPDMA_CxLLR register from memory
This bit is used to control the update of GPDMA_CxLLR from the memory during the link
transfer.
0: no GPDMA_CxLLR update
1: GPDMA_CxLLR update
Bits 15:2 LA[15:2]: pointer (16-bit low-significant address) to the next linked-list data structure
If UT1 = UT2 = UB1 = USA = UDA = ULL = 0 and if LA[15:2] = 0, the current LLI is the last
one. The channel transfer is completed without any update of the linked-list GPDMA register
file.
Else, this field is the pointer to the memory address offset from which the next linked-list data
structure is automatically fetched from, once the data transfer is completed, in order to
conditionally update the linked-list GPDMA internal register file (GPDMA_CxTR1,
GPDMA_CxTR2, GPDMA_CxBR1, GPDMA_CxSAR, GPDMA_CxDAR, and
GPDMA_CxLLR).
Note: The user must program the pointer to be 32-bit aligned. The two low-significant bits are
write ignored.
Bits 1:0 Reserved, must be kept at reset value.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

General purpose direct memory access controller (GPDMA)

17.8.19

GPDMA channel x alternate linked-list address register
(GPDMA_CxLLR)
Address offset: 0xCC + 0x80 * x (x = 12 to 15)
Reset value: 0x0000 0000
This register is secure or nonsecure depending on the secure state of channel x
(GPDMA_SECCFGR.SECx), and privileged or unprivileged, depending on the privileged
state of channel x (GPDMA_PRIVCFGR.PRIVx).
This register configures the data structure of the next LLI in the memory and its address
pointer. A channel transfer is completed when this register is null.
This register must be written when GPDMA_CxCR.EN = 0.
This register is read-only when GPDMA_CxCR.EN = 1.
This register must be written when the channel is completed (then the hardware has
deasserted GPDMA_CxCR.EN). A channel transfer can be completed and programmed at
different levels: block or LLI or full linked-list.
In linked-list mode, during the link transfer, this register is automatically updated by the
GPDMA from the memory if GPDMA_CxLLR.ULL = 1.

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

UT3

UB2

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

15

14

13

12

11

10

9

rw

LA[15:2]
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

Bit 31 UT1: Update GPDMA_CxTR1 from memory
This bit controls the update of GPDMA_CxTR1 from the memory during the link transfer.
0: no GPDMA_CxTR1 update
1: GPDMA_CxTR1 update
Bit 30 UT2: Update GPDMA_CxTR2 from memory
This bit controls the update of GPDMA_CxTR2 from the memory during the link transfer.
0: no GPDMA_CxTR2 update
1: GPDMA_CxTR2 update
Bit 29 UB1: Update GPDMA_CxBR1 from memory
This bit controls the update of GPDMA_CxBR1 from the memory during the link transfer.
If UB1 = 0 and if GPDMA_CxLLR ≠ 0, the linked-list is not completed.
GPDMA_CxBR1.BNDT[15:0] is then restored to the programmed value after data transfer is
completed and before the link transfer.
0: no GPDMA_CxBR1 update from memory (GPDMA_CxBR1.BNDT[15:0] restored if any
link transfer)
1: GPDMA_CxBR1 update
Bit 28 USA: update GPDMA_CxSAR from memory
This bit controls the update of GPDMA_CxSAR from the memory during the link transfer.
0: no GPDMA_CxSAR update
1: GPDMA_CxSAR update

RM0456 Rev 6

<!-- pagebreak -->

