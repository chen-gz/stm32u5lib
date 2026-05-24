RM0456 Rev 6

RM0456

USB on-the-go full-speed (OTG_FS)
The mask bits for each interrupt source of each channel are also available in the
OTG_HCINTMSKx register.
•

72.7.4

The host core provides the following status checks and interrupt generation:
–

Transfer completed interrupt, indicating that the data transfer is complete on both
the application (AHB) and USB sides

–

Channel has stopped due to transfer completed, USB transaction error or disable
command from the application

–

Associated transmit FIFO is half or completely empty (IN endpoints)

–

ACK response received

–

NAK response received

–

STALL response received

–

USB transaction error due to CRC failure, timeout, bit stuff error, false EOP

–

Babble error

–

frame overrun

–

data toggle error

Host scheduler
The host core features a built-in hardware scheduler which is able to autonomously re-order
and manage the USB transaction requests posted by the application. At the beginning of
each frame the host executes the periodic (isochronous and interrupt) transactions first,
followed by the nonperiodic (control and bulk) transactions to achieve the higher level of
priority granted to the isochronous and interrupt transfer types by the USB specification.
The host processes the USB transactions through request queues (one for periodic and one
for nonperiodic). Each request queue can hold up to 8 entries. Each entry represents a
pending transaction request from the application, and holds the IN or OUT channel number
along with other information to perform a transaction on the USB. The order in which the
requests are written to the queue determines the sequence of the transactions on the USB
interface.
At the beginning of each frame, the host processes the periodic request queue first, followed
by the nonperiodic request queue. The host issues an incomplete periodic transfer interrupt
(IPXFR bit in OTG_GINTSTS) if an isochronous or interrupt transaction scheduled for the
current frame is still pending at the end of the current frame. The OTG_FS core is fully
responsible for the management of the periodic and nonperiodic request queues.The
periodic transmit FIFO and queue status register (OTG_HPTXSTS) and nonperiodic
transmit FIFO and queue status register (OTG_HNPTXSTS) are read-only registers which
can be used by the application to read the status of each request queue. They contain:
•

The number of free entries currently available in the periodic (nonperiodic) request
queue (8 max)

•

Free space currently available in the periodic (nonperiodic) Tx FIFO (out-transactions)

•

IN/OUT token, host channel number and other status information.

As request queues can hold a maximum of 8 entries each, the application can push to
schedule host transactions in advance with respect to the moment they physically reach the
SB for a maximum of 8 pending periodic transactions plus 8 pending non-periodic
transactions.
To post a transaction request to the host scheduler (queue) the application must check that
there is at least 1 entry available in the periodic (nonperiodic) request queue by reading the

RM0456 Rev 6

<!-- pagebreak -->

3291

USB on-the-go full-speed (OTG_FS)

RM0456

PTXQSAV bits in the OTG_HNPTXSTS register or NPTQXSAV bits in the
OTG_HNPTXSTS register.

72.8

OTG_FS SOF trigger
Figure 897. SOF connectivity (SOF trigger output to TIM and ITR1 connection)

SOF pulse output, to
external audio control
VBUS
ITR1

SOF pulse

DD+

TIM

SOFgen

ID

USB micro-AB connector

STM32

VSS

MSv36914V1

The OTG_FS core provides means to monitor, track and configure SOF framing in the host
and peripheral, as well as an SOF pulse output connectivity feature.
Such utilities are especially useful for adaptive audio clock generation techniques, where
the audio peripheral needs to synchronize to the isochronous stream provided by the PC, or
the host needs to trim its framing rate according to the requirements of the audio peripheral.

72.8.1

Host SOFs
In host mode the number of PHY clocks occurring between the generation of two
consecutive SOF (FS) or Keep-alive (LS) tokens is programmable in the host frame interval
register (HFIR), thus providing application control over the SOF framing period. An interrupt
is generated at any start of frame (SOF bit in OTG_GINTSTS). The current frame number
and the time remaining until the next SOF are tracked in the host frame number register
(HFNUM).
A SOF pulse signal, is generated at any SOF starting token and with a width of 20 HCLK
cycles. The SOF pulse is also internally connected to the input trigger of the timer, so that
the input capture feature, the output compare feature and the timer can be triggered by the
SOF pulse.

72.8.2

Peripheral SOFs
In device mode, the start of frame interrupt is generated each time an SOF token is received
on the USB (SOF bit in OTG_GINTSTS). The corresponding frame number can be read
from the device status register (FNSOF bit in OTG_DSTS). A SOF pulse signal with a width
of 20 HCLK cycles is also generated.The SOF pulse signal is also internally connected to
the TIM input trigger, so that the input capture feature, the output compare feature and the
timer can be triggered by the SOF pulse.

<!-- pagebreak -->

