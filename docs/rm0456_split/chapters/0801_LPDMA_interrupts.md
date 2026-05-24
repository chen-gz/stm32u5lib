RM0456 Rev 6

RM0456

18.7

Low-power direct memory access controller (LPDMA)

LPDMA interrupts
There is one LPDMA interrupt line for each channel, and separately for each CPU (if several
ones in the devices).
Table 155. LPDMA interrupt requests

LPDMA_CHx

Interrupt
acronym

Interrupt event

Interrupt enable

Event flag

Event clear method

Transfer
complete

LPDMA_CxCR.TCIE

LPDMA_CxSR.TCF

Write 1 to LPDMA_CxFCR.TCF

Half transfer

LPDMA_CxCR.HTIE

LPDMA_CxSR.HTF

Write 1 to LPDMA_CxFCR.HTF

Data transfer
error

LPDMA_CxCR.DTEIE

LPDMA_CxSR.DTEF

Write 1 to LPDMA_CxFCR.DTEF

Update link error LPDMA_CxCR.ULEIE

LPDMA_CxSR.ULEF

Write 1 to LPDMA_CxFCR.ULEF

User setting error LPDMA_CxCR.USEIE

LPDMA_CxSR.USEF

Write 1 to LPDMA_CxFCR.USEF

Suspended

LPDMA_CxCR.SUSPIE LPDMA_CxSR.SUSPF Write 1 to LPDMA_CxFCR.SUSPF

Trigger overrun

LPDMA_CxCR.TOFIE

LPDMA_CxSR.TOF

Write 1 to LPDMA_CxFCR.TOF

A LPDMA channel x event may be:
•

a transfer complete

•

a half-transfer complete

•

a transfer error, due to either:

•
Note:

–

a data transfer error

–

an update link error

–

a user setting error completed suspension

a trigger overrun

When a channel x transfer complete event occurs, the output signal lpdma_chx_tc is
generated as a high pulse of one clock cycle.
An interrupt is generated following any xx event, provided that both:
•

the corresponding interrupt event xx is enabled (LPDMA_CxCR.xxIE = 1)

•

the corresponding event flag is cleared (LPDMA_CxSR.xxF = 0). This means that, after
a previous same xx event occurrence, a software agent must have written 1 into the
corresponding xx flag clear control bit (write 1 into LPDMA_CxFCR.xxF).

TCF (transfer complete) and HTF (half transfer) events generation is controlled by
LPDMA_CxTR2.TCEM[1:0] as follows:
•

A transfer complete event is a block transfer complete or a LLI transfer complete
including the upload of the next LLI if any, or the full linked-list completion, depending
on the transfer complete event mode LPDMA_CxTR2.TCEM[1:0].

•

A half transfer event is a half block transfer. A half-block transfer occurs when half of
the source block size bytes (rounded-up integer of LPDMA_CxBR1.BNDT[15:0] / 2) is
transferred to the destination.

See LPDMA channel x transfer register 2 (LPDMA_CxTR2) for more details.

RM0456 Rev 6

<!-- pagebreak -->

