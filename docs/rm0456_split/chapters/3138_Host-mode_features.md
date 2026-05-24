RM0456 Rev 6

RM0456

USB on-the-go full-speed (OTG_FS)
1.

When an STALL, TXERR, BBERR or DTERR interrupt in OTG_HCINTx is received for
an IN or OUT channel. The application must be able to receive other interrupts
(DTERR, Nak, data, TXERR) for the same channel before receiving the halt.

2.

When a DISCINT (disconnect device) interrupt in OTG_GINTSTS is received. (The
application is expected to disable all enabled channels).

3.

When the application aborts a transfer before normal completion.

Operational model
The application must initialize a channel before communicating to the connected device.
This section explains the sequence of operation to be performed for different types of USB
transactions.
•

Writing the transmit FIFO
The OTG_FS host automatically writes an entry (OUT request) to the periodic/nonperiodic request queue, along with the last 32-bit word write of a packet. The
application must ensure that at least one free space is available in the periodic/nonperiodic request queue before starting to write to the transmit FIFO. The application
must always write to the transmit FIFO in 32-bit words. If the packet size is non-32-bit
word aligned, the application must use padding. The OTG_FS host determines the
actual packet size based on the programmed maximum packet size and transfer size.
Figure 902. Transmit FIFO write task
Start
Read OTG_HPTXSTS /OTG_HNPTXSTS
registers for available FIFO and queue
spaces

Wait for NPTXFE/PTXFE interrupt in
OTG_GINTSTS

No

1 MPS
or LPS FIFO space
available?
Yes

Yes

Write 1 packet
data to transmit
FIFO

More
packets to
send?
No
MPS: Maximum packet size
LPS: Last packet size

•

Done
ai15673c

Reading the receive FIFO
The application must ignore all packet statuses other than IN data packet (bx0010).

RM0456 Rev 6

<!-- pagebreak -->

