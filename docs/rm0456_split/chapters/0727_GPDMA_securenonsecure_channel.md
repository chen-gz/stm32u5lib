RM0456 Rev 6

RM0456

General purpose direct memory access controller (GPDMA)
Figure 71. Shared GPDMA channel with circular buffering: update of the memory
start address with a linear addressing channel
Req=PERIPH_TX Req=PERIPH_TX

Reset
Init/LLI0

LLI1

...

Channel X

LLIN-1

LLIN

Ht+ tcf

Ht+ tcf

Memory

LLIN-1

LLIN-2
All Uxx=1

DMA_Cx...

DMA_CxTR1

DMA_Cx...

DMA_CxTR2

DMA_Cx...

DMA_CxBR1

DMA_CxLLR

LLIN

DMA_CxSAR
DMA_CxDAR
DMA_CxLLR

USA = 1, others Uxx = 0 LA+ = 0xC

17.4.14

MSv62641V1

GPDMA secure/nonsecure channel
The GPDMA controller is compliant with the TrustZone hardware architecture at channel
level, partitioning all its resources so that they exist in one of the secure and nonsecure
worlds at any given time.
Any channel x is a secure or a nonsecure hardware resource, as configured
by GPDMA_SECCFGR.SECx.
When a channel x is configured in secure state by a secure and privileged agent, the
following access control rules are applied:
•

A nonsecure read access to a register field of this channel is forced to return 0, except
for GPDMA_SECCFGR, GPDMA_PRIVCFGR and GPDMA_RCFGLOCKR that are
readable by a nonsecure agent.

•

A nonsecure write access to a register field of this channel has no impact.

When a channel x is configured in secure state, a secure agent can configure separately as
secure or nonsecure the GPDMA data transfer from the source (GPDMA_CxTR1.SSEC)
and the GPDMA data transfer to the destination (GPDMA_CxTR1.DSEC).
When a channel x is configured in secure state and in linked-list mode, the loading of the
next linked-list data structure from the GPDMA memory into its register file, is automatically
performed with secure transfers via the GPDMA_CxCR.LAP allocated master port.
The GPDMA generates a secure bus that reflects GPDMA_SECCFGR, to keep the other
peripherals informed of the secure/nonsecure state of each GPDMA channel x.
The GPDMA also generates a security illegal access pulse signal on an illegal nonsecure
access to a secure GPDMA register. This signal is routed to the TrustZone interrupt
controller.

RM0456 Rev 6

<!-- pagebreak -->

763

General purpose direct memory access controller (GPDMA)

RM0456

When the secure software must switch a channel from a secure state to a nonsecure state,
the secure software must abort the channel or wait until the secure channel is completed
before switching. This is needed to dynamically re-allocate a channel to a next nonsecure
transfer as a nonsecure software is not allowed to do so and must have
GPDMA_CxCR.EN = 0 before the nonsecure software can reprogram the GPDMA_CxCR
for a next transfer. The secure software may reset not only the channel x
(GPDMA_CxCR.RESET = 1) but also the full channel x register file to its reset value.

17.4.15

GPDMA privileged/unprivileged channel
Any channel x is a privileged or unprivileged hardware resource, as configured by a
privileged agent via GPDMA_PRIVCFGR.PRIVx.
When a channel x is configured in a privileged state by a privileged agent, the following
access control rules are applied:
•

An unprivileged read access to a register field of this channel is forced to return 0,
except for GPDMA_PRIVCFGR, GPDMA_SECCFGR, and GPDMA_RCFGLOCKR
that are readable by an unprivileged agent.

•

An unprivileged write access to a register field of this channel has no impact.

When a channel is configured in a privileged (or unprivileged) state, the source and
destination data transfers are privileged (respectively unprivileged) transfers over the AHB
master port.
When a channel is configured in a privileged (or unprivileged) state and in linked-list mode,
the loading of the next linked-list data structure from the GPDMA memory into its register
file, is automatically performed with privileged (respectively unprivileged) transfers, via the
GPDMA_CxCR.LAP allocated master port.
The GPDMA generates a privileged bus that reflects GPDMA_PRIVCFGR, to keep the
other peripherals informed of the privileged/unprivileged state of each GPDMA channel x.
When the privileged software must switch a channel from a privileged state to
an unprivileged state, the privileged software must abort the channel or wait until that the
privileged channel is completed before switching. This is needed to dynamically re-allocate
a channel to a next unprivileged transfer as an unprivileged software is not allowed to do so,
and must have GPDMA_CxCR.EN = 0 before the unprivileged software can reprogram the
GPDMA_CxCR for a next transfer. The privileged software may reset not only the channel x
(GPDMA_CxCR.RESET = 1) but also the full channel x register file to its reset value.

17.4.16

GPDMA error management
The GPDMA is able to manage and report to the user a transfer error, as follows, depending
on the root cause.

Data transfer error
On a bus access (as a AHB single or a burst) to the source or the destination:

<!-- pagebreak -->

•

The source or destination target reports an AHB error.

•

The programmed channel transfer is stopped (GPDMA_CxCR.EN cleared by the
GPDMA hardware). The channel status register reports an idle state
(GPDMA_CxSR.IDLEF = 1) and the data error (GPDMA_CxSR.DTEF = 1).

RM0456 Rev 6

RM0456

General purpose direct memory access controller (GPDMA)
•

After a GPDMA data transfer error, the user must perform a debug session, taking care
of the product-defined memory mapping of the source and destination, including the
protection attributes.

•

After a GPDMA data transfer error, the user must issue a channel reset (set
GPDMA_CxCR.RESET) to reset the hardware GPDMA channel data path and the
content of the FIFO, before the user enables again the same channel for a next
transfer.

Link transfer error
On a tentative update of a GPDMA channel register from the programmed LLI in the
memory:
•

The linked-list memory reports an AHB error.

•

The programmed channel transfer is stopped (GPDMA_CxCR.EN cleared by the
GPDMA hardware), the channel status register reports an idle state
(GPDMA_CxSR.IDLEF = 1) and the link error (GPDMA_CxSR.ULEF = 1).

•

After a GPDMA link error, the user must perform a debug session, taking care of the
product-defined memory mapping of the linked-list data structure (GPDMA_CxLBAR
and GPDMA_CxLLR), including the protection attributes.

•

After a GPDMA link error, the user must explicitly write the linked-list register file
(GPDMA_CxTR1, GPDMA_CxTR2, GPDMA_CxBR1, GPDMA_CxSAR,
GPDMA_CxDAR and GPDMA_CxLLR, plus GPDMA_CxTR3 and GPDMA_CxBR2),
before the user enables again the same channel for a next transfer.

User setting error
On a tentative execution of a GPDMA transfer with an unauthorized user setting:
•

The programmed channel transfer is disabled (GPDMA_CxCR.EN forced and cleared
by the GPDMA hardware) preventing the next unauthorized programmed data transfer
from being executed. The channel status register reports an idle state
(GPDMA_CxSR.IDLEF = 1) and a user setting error (GPDMA_CxSR.USEF = 1).

•

After a GPDMA user setting error, the user must perform a debug session, taking care
of the GPDMA channel programming. A user setting error can be caused by one of the
following:
–

a programmed null source block size without a programmed update of this value
from the next LLI1 (GPDMA_CxBR1.BNDT[15:0] = 0 and
GPDMA_CxLLR.UB1 = 0)

–

a programmed non-null source block size being not a multiple of the programmed
data width of a source burst transfer (GPDMA_CxBR1.BNDT[2:0] versus
GPDMA_CxTR1.SDW_LOG2[1:0])

–

when in packing/unpacking mode (if PAM[1] = 1), a programmed non-null source
block size being not a multiple of the programmed data width of a destination burst
transfer (GPDMA_CxBR1.BNDT[2:0] versus GPDMA_CxTR1.DDW_LOG2[1:0])

–

a programmed unaligned source start address, being not a multiple of the
programmed data width of a source burst transfer (GPDMA_CxSAR[2:0] versus
GPDMA_CxTR1.SDW_LOG2[1:0])

–

for channel x (x = 12 to 15): a programmed unaligned source address offset being
not a multiple of the programmed data width of a source burst transfer
(GPDMA_CxTR3.SAO[2:0] versus GPDMA_CxTR1.SDW_LOG2[1:0])

RM0456 Rev 6

<!-- pagebreak -->

763

General purpose direct memory access controller (GPDMA)

17.4.17

RM0456

–

for channel x (x = 12 to 15): a programmed unaligned block repeated source
address offset being not a multiple of the programmed data width of a source burst
transfer (GPDMA_CxBR2.BRSAO[2:0] versus GPDMA_CxTR1.SDW_LOG2[1:0])

–

a programmed unaligned destination start address, being not a multiple of the
programmed data width of a destination burst transfer (GPDMA_CxDAR[2:0]
versus GPDMA_CxTR1.DDW_LOG2[1:0])

–

for channel x (x = 12 to 15): a programmed unaligned destination address offset
being not a multiple of the programmed data width of a destination burst transfer
(GPDMA_CxTR3.DAO[2:0] versus GPDMA_CxTR1.DDW_LOG2[1:0])

–

for channel x (x = 12 to 15): a programmed unaligned block repeated destination
address offset being not a multiple of the programmed data width of a destination
burst transfer (GPDMA_CxBR2.BRDAO[2:0] versus
GPDMA_CxTR1.DDW_LOG2[1:0])

–

a programmed double-word source data width
(GPDMA_CxTR1.SDW_LOG2[1:0] = 11)

–

a programmed double-word destination data width
(GPDMA_CxTR1.DDW_LOG2[1:0] = 11)

–

a programmed linked-list item LLIn+1 with a null data transfer
(GPDMA_CxLLR.UB1 = 1 and GPDMA_CxBR1. BNDT = 0)

GPDMA autonomous mode
To save dynamic power consumption while the GPDMA executes the programmed
linked-list transfers, the GPDMA hardware automatically manages its own clock gating and
generates a clock request output signal to the RCC, whenever the device is in Run or
low-power modes, provided that the RCC is programmed with the corresponding GPDMA
enable control bits.
For more details about the RCC programming, refer to the RCC section of the reference
manual.
For mode details about the availability of the GPDMA autonomous feature vs the device
low-power modes, refer to Section 17.3.2.

Note:

Design/firmware: In low-power modes, DMA clock request is asserted upon a DMA
request/trigger from an autonomous peripheral or upon a DMA trigger from an EXTI line.
The user can program and schedule the execution of a given GPDMA transfer at a LLIn
level of a GPDMA channel x, with GPDMA_CxTR2 as follows:
•

The software controls and conditions the input of a transfer with TRIGM[1:0],
TRIGPOL[1:0], TRIGSEL[y(a):0], SWREQ, and REQSEL[z(a):0] for the input trigger and
request.

•

The software controls and signals the output of a transfer with TCEM[1:0] for
generating or not a transfer complete event, and generating or not an associated half
data transfer event).

See GPDMA channel x transfer register 2 (GPDMA_CxTR2) for more details.
When used in low-power modes, this functionality enables a CPU wake-up on a specific
transfer completion by the enabled GPDMA transfer complete interrupt

a. Refer to GPDMA channel x transfer register 2 (GPDMA_CxTR2) for y and z values.

<!-- pagebreak -->

