763

General purpose direct memory access controller (GPDMA)

RM0456

Table 142. Effect of low-power modes on GPDMA (continued)
Mode

Description

Stop(1)

The content of the GPDMA registers is kept when entering Stop mode. The content of the
GPDMA registers can be autonomously updated by a next linked-list item from memory, to
perform autonomous data transfers. GPDMA interrupts can cause the device to exit Stop 0 and 1
modes(1).

Standby

The GPDMA is powered down and must be reinitialized after exiting Standby mode.

1. Refer to Section 17.3.2 to know if any Stop mode is supported.

17.7

GPDMA interrupts
There is one GPDMA interrupt line for each channel, and separately for each CPU
(if several ones in the devices).
Table 143. GPDMA interrupt requests

GPDMA_CHx

Interrupt
acronym

Interrupt
event

Interrupt enable

Event flag

Event clear method

Transfer
complete

GPDMA_CxCR.TCIE

GPDMA_CxSR.TCF

Write 1 to GPDMA_CxFCR.TCF

Half transfer

GPDMA_CxCR.HTIE

GPDMA_CxSR.HTF

Write 1 to GPDMA_CxFCR.HTF

Data transfer
GPDMA_CxCR.DTEIE
error

GPDMA_CxSR.DTEF

Write 1 to GPDMA_CxFCR.DTEF

Update link
error

GPDMA_CxCR.ULEIE

GPDMA_CxSR.ULEF

Write 1 to GPDMA_CxFCR.ULEF

User setting
error

GPDMA_CxCR.USEIE

GPDMA_CxSR.USEF

Write 1 to GPDMA_CxFCR.USEF

Suspended

GPDMA_CxCR.SUSPIE GPDMA_CxSR.SUSPF Write 1 to GPDMA_CxFCR.SUSPF

Trigger
overrun

GPDMA_CxCR.TOFIE

GPDMA_CxSR.TOF

Write 1 to GPDMA_CxFCR.TOF

A GPDMA channel x event may be:
•

a transfer complete

•

a half-transfer complete

•

a transfer error, due to either:

•
Note:

<!-- pagebreak -->

–

a data transfer error

–

an update link error

–

a user setting error completed suspension

a trigger overrun

When a channel x transfer complete event occurs, the output signal gpdma_chx_tc is
generated as a high pulse of one clock cycle.

RM0456 Rev 6

RM0456

General purpose direct memory access controller (GPDMA)
An interrupt is generated following any xx event, provided that both:
•

the corresponding interrupt event xx is enabled (GPDMA_CxCR.xxIE = 1)

•

the corresponding event flag is cleared (GPDMA_CxSR.xxF = 0). This means that,
after a previous same xx event occurrence, a software agent must have written 1 into
the corresponding xx flag clear control bit (write 1 into GPDMA_CxFCR.xxF).

TCF (transfer complete) and HTF (half transfer) events generation is controlled by
GPDMA_CxTR2.TCEM[1:0] as follows:
•

A transfer complete event is a block transfer complete, a 2D/repeated block transfer
complete, or a LLI transfer complete including the upload of the next LLI if any, or the
full linked-list completion, depending on the transfer complete event mode
GPDMA_CxTR2.TCEM[1:0].

•

A half transfer event is an half block transfer or a half 2D/repeated block transfer,
depending on the transfer complete event mode GPDMA_CxTR2.TCEM[1:0].
A half-block transfer occurs when half of the source block size bytes (rounded-up
integer of GPDMA_CxBR1.BNDT[15:0] / 2) is transferred to the destination.
A half 2D/repeated block transfer occurs when half of the repeated blocks (rounded-up
integer of (GPDMA_CxBR1.BRC[10:0] + 1) / 2) is transferred to the destination.

See GPDMA channel x transfer register 2 (GPDMA_CxTR2) for more details.
A transfer error rises in one of the following situations:
•

during a single/burst data transfer from the source or to the destination (DTEF)

•

during an update of a GPDMA channel register from the programmed LLI in memory
(ULEF)

•

during a tentative execution of a GPDMA channel with an unauthorized setting (USEF)
The user must perform a debug session to correct the GPDMA channel programming
versus the USEF root causes list (see Section 17.4.16).

A trigger overrun is described in Trigger hit memorization and trigger overrun flag
generation.

RM0456 Rev 6

<!-- pagebreak -->

