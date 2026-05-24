RM0456 Rev 6

RM0456

FD controller area network (FDCAN)

Extended message ID filtering
Figure 890 shows the flow for extended message ID (29-bit identifier) filtering. The extended
message ID filter element is described in Section 70.3.12.
The extended message filtering is controlled by the FDCAN_RXGFC register. The extended
message ID, the remote transmission request bit (RTR), and the identifier extension bit
(IDE) of the received frames are compared against the list of configured filter elements.
Figure 890. Extended message ID filter path
Valid frame received

29-bit

Remote frame

11-bit

Bit identifier

Yes

No

Reject remote frame
RXGFC[RRFE] = 0

RXGFC[RRFE] = 1

Receive filter list enabled

RXGFC[LSE[3:0]] = 0

RXGFC[LSE[3:0]] > 0
Match filter
element #0

Yes

No
Yes
No
Match filter
element #RXGFC.LSE

Yes

Acceptance
or Rejection
Accept

No
Accept
non-matching
frames

Reject

RXGFC[ANFE[1]] = 1

Discard frame

RXGFC[ANFE[1]] = 0

Target FIFO full

Yes

No
Append to target FIFO
MS47280V1

The extended ID AND mask (XIDAM) is AND-ed with the received identifier before the filter
list is executed.

RM0456 Rev 6

<!-- pagebreak -->

3086

FD controller area network (FDCAN)

RM0456

Rx FIFOs
Rx FIFO 0 and Rx FIFO 1 can hold up to three elements each.
Received messages that passed acceptance filtering are transferred to the Rx FIFO as
configured by the matching filter element. For a description of the filter mechanisms
available for Rx FIFO 0 and Rx FIFO 1, see Acceptance filter. The Rx FIFO element is
described in Section 70.3.8.
When an Rx FIFO full condition is signaled by RFnF in FDCAN_IR (where n is the FIFO
number), no further messages are written to the corresponding Rx FIFO until at least one
message has been read out, and the Rx FIFO get index has been incremented. In case a
message is received while the corresponding Rx FIFO is full, this message is discarded,
and the interrupt flag RFnL is set in the FDCAN_IR register.
When reading from an Rx FIFO, the Rx FIFO get index (FnGI of FDCAN_RXFnS) + FIFO
element size has to be added to the corresponding Rx FIFO start address (FnSA).

Rx FIFO blocking mode
The Rx FIFO blocking mode is configured by clearing the FnOM bit in the FDCAN_RXGFC
register. This is the default operation mode for the Rx FIFOs.
When an Rx FIFO full condition is reached (FnPI = FnGI in FDCAN_RXFnS), no further
messages are written to the corresponding Rx FIFO until at least one message has been
read out and the Rx FIFO get index has been incremented. An Rx FIFO full condition is
signaled by FnF = 1 in FDCAN_RXFnS. In addition, the RFnF interrupt flag is set in
FDCAN_IR.
In case a message is received while the corresponding Rx FIFO is full, this message is
discarded, and the message lost condition is signaled by setting RFnL bit in
FDCAN_RXFnS. In addition, the RFnL interrupt flag is set in FDCAN_IR.

Rx FIFO overwrite mode
The Rx FIFO overwrite mode is configured by setting the FnOM bit of the FDCAN_RXGFC
register.
When an Rx FIFO full condition (FnPI = FnGI of FDCAN_RXFnS) is signaled by FnF = 1 in
FDCAN_RXFnS, the next message accepted for the FIFO overwrites the oldest FIFO
message. Put and get indices are both incremented by one.
When an Rx FIFO is operated in overwrite mode and an Rx FIFO full condition is signaled,
reading from the Rx FIFO elements must start at least at get index + 1. This is because it
may happen that a received message is written to the message RAM (put index) while the
CPU is reading from the message RAM (get index). In this case, inconsistent data can be
read from the respective Rx FIFO element. Adding an offset to the get index when reading
from the Rx FIFO avoids this problem. The offset depends on how fast the CPU accesses
the Rx FIFO.
After reading from the Rx FIFO, the number of the last element read has to be written to the
Rx FIFO acknowledge index (FnA of FDCAN_RXFnA). This increments the get index to that
element number. In case the put index has not been incremented to this Rx FIFO element,
the Rx FIFO full condition is reset (FnF = 0 in FDCAN_RXFnS).

<!-- pagebreak -->

