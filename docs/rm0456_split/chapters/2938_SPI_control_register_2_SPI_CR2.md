RM0456 Rev 6

RM0456

FD controller area network (FDCAN)

Tx handling
The Tx handler handles transmission requests for the Tx FIFO and the Tx queue. It controls
the transfer of transmit messages to the CAN core, the put and get indices, and the Tx event
FIFO. Up to three Tx buffers can be set up for message transmission. The CAN message
data field is configured to 64 bytes. the Tx FIFO allocates eighteen 32-bit words for storage
of a Tx element.
Table 720. Possible configurations for frame transmission
CCCR

Tx buffer element
Frame transmission

Note:

BRSE

FDOE

FDF

BRS

Ignored

0

Ignored

Ignored

Classic CAN

0

1

0

Ignored

Classic CAN

0

1

1

Ignored

FD without bit rate switching

1

1

0

Ignored

Classic CAN

1

1

1

0

FD without bit rate switching

1

1

1

1

FD with bit rate switching

AUTOSAR requires at least three Tx queue buffers and support of transmit cancellation.
The Tx handler starts a Tx scan to check for the highest priority pending Tx request (Tx
buffer with lowest message ID) when the Tx buffer request pending register
(FDCAN_TXBRP) is updated, or when a transmission has been started.

Transmit pause
The transmit pause feature is intended for use in CAN systems where the CAN message
identifiers are permanently specified to specific values and cannot easily be changed.
These message identifiers can have a higher CAN arbitration priority than other defined
messages, while in a specific application their relative arbitration priority must be inverse.
This may lead to a case where one ECU sends a burst of CAN messages that cause
another ECU CAN messages to be delayed because that other messages have a lower
CAN arbitration priority.
As an example, if CAN ECU-1 has the feature enabled and is requested by its application
software to transmit four messages, it waits, after the first successful message transmission,
for two CAN bit times of bus-idle before it is allowed to start the next requested message. If
there are other ECUs with pending messages, these messages are started in the idle time,
and they would not need to arbitrate with the next message of ECU-1. After having received
a message, ECU-1 is allowed to start its next transmission as soon as the received
message releases the CAN bus.
The feature is controlled by the TXP bit of the CCCR register. If the bit is set, the FDCAN,
each time it has successfully transmitted a message, pauses for two CAN bit times before
starting the next transmission. This enables other CAN nodes in the network to transmit
messages even if their messages have lower prior identifiers. By default, this feature is
disabled (TXP = 0 in FDCAN_CCCR).

RM0456 Rev 6

<!-- pagebreak -->

3086

FD controller area network (FDCAN)

RM0456

This feature looses up burst transmissions coming from a single node and it protects against
"babbling idiot" scenarios where the application program erroneously requests too many
transmissions.

Tx FIFO
Tx FIFO operation is configured by clearing the TFQM bit of the FDCAN_TXBC register.
Messages stored in the Tx FIFO are transmitted starting with the message referenced by
the get index (TFGI[1:0] bitfield of FDCAN_TXFQS). After each transmission, the get index
is incremented cyclically until the Tx FIFO is empty. The Tx FIFO enables transmission of
messages with the same message ID from different Tx buffers in the order that these
messages have been written to the Tx FIFO. The FDCAN calculates the Tx FIFO free level
(TFFL[2:0] bitfield of FDCAN_TXFQS) as the difference between the get and put index. It
indicates the number of available (free) Tx FIFO elements.
New transmit messages have to be written to the Tx FIFO starting with the Tx buffer
referenced by the put index (TFQPI[1:0] bitfield of FDCAN_TXFQS). An add request
increments the put index to the next free Tx FIFO element. When the put index reaches the
get index, Tx FIFO full (TFQF = 1 in FDCAN_TXFQS) is signaled. In this case, no further
messages must be written to the Tx FIFO until the next message has been transmitted and
the get index has been incremented.
When a single message is added to the Tx FIFO, the transmission is requested by setting
the FDCAN_TXBAR bit related to the Tx buffer referenced by the Tx FIFO put index.
When multiple (n) messages are added to the Tx FIFO, they are written to n consecutive Tx
buffers starting with the put index. The transmissions are then requested via the
FDCA_TXBAR register. The put index is then cyclically incremented by n. The number of
requested Tx buffers must not exceed the number of free Tx buffers as indicated by the Tx
FIFO free level.
When a transmission request for the Tx buffer referenced by the get index is canceled, the
get index is incremented to the next Tx buffer with a transmission request is pending and the
Tx FIFO free level is recalculated. When transmission cancellation is applied to any other Tx
buffer, the get index and the FIFO Free Level remain unchanged.
A Tx FIFO element allocates eighteen 32-bit words in the message RAM. The Therefore,
the start address of the next available (free) Tx FIFO buffer, is calculated by adding 18 times
the put index TFQPI[1:0] (0 … 2) to the Tx buffer start address TBSA.

Tx queue
Tx queue operation is configured by setting the TFQM of the FDCAN_TXBC register.
Messages stored in the Tx queue are transmitted starting with the message with the lowest
message ID (highest priority).
In case of mixing of standard and extended message IDs, the standard message IDs are
compared to bits [28:18] of extended message IDs.
In case multiple queue buffers are configured with the same message ID, the queue buffer
with the lowest buffer number is transmitted first.
New messages have to be written to the Tx buffer referenced by the put index (TFQPI[1:0]
in FDCAN_TXFQS). An add request cyclically increments the put index to the next free Tx
buffer. In case the Tx queue is full (TFQF = 1 in FDCAN_TXFQS), the put index is not valid
and no further message must be written to the Tx queue until at least one of the requested
messages has been sent out or a pending transmission request has been canceled.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

FD controller area network (FDCAN)
The application can use the FDCAN_TXBRP register instead of the put index and can place
messages to any Tx buffer without pending transmission request.
A Tx queue buffer allocates eighteen 32-bit words in the message RAM. The start address
of Therefore, the next available (free) Tx queue buffer is calculated by adding 18 times the
Tx queue put index TFQPI[1:0] (0 ... 2) to the Tx buffer start address TBSA.

Transmit cancellation
The FDCAN supports transmit cancellation. To cancel a requested transmission from a Tx
queue buffer, the host has to write 1 to the corresponding bit position (= number of Tx buffer)
of the FDCAN_TXBCR register. Transmit cancellation is not intended for Tx FIFO operation.
Successful cancellation is signaled by setting the corresponding bit of the FDCAN_TXBCF
register.
In case a transmit cancellation is requested while a transmission from a Tx buffer is already
ongoing, the corresponding FDCAN _TXBRP bit remains set as long as the transmission is
in progress. If the transmission is successful, the corresponding FDCAN_TXBTO and
FDCAN_TXBCF bits are set. If the transmission is not successful, it is not repeated and only
the corresponding FDCAN_TXBCF bit is set.
Note:

In case a pending transmission is canceled immediately before it has been started, there is
a short time window where no transmission is started even if another message is pending in
the node. This can enable another node to transmit a message that can have a priority lower
than that of the second message in the node.

Tx event handling
To support Tx event handling the FDCAN has implemented a Tx event FIFO. After the
FDCAN has transmitted a message on the CAN bus, message ID and timestamp are stored
in a Tx event FIFO element. To link a Tx event to a Tx event FIFO element, the message
marker from the transmitted Tx buffer is copied into the Tx event FIFO element.
The Tx event FIFO is configured to three elements. The Tx event FIFO element is described
in Tx FIFO.
The purpose of the Tx event FIFO is to decouple handling transmit status information from
transmit message handling that is, a Tx buffer holds only the message to be transmitted,
while the transmit status is stored separately in the Tx event FIFO. This has the advantage,
especially when operating a dynamically managed transmit queue, that a Tx buffer can be
used for a new message immediately after successful transmission. There is no need to
save transmit status information from a Tx buffer before overwriting that Tx buffer.
When a Tx event FIFO full condition is signaled by the TEFF bit of the FDCAN_IR, no
further elements are written to the Tx event FIFO until at least one element has been read
out and the Tx event FIFO get index has been incremented. In case a Tx event occurs while
the Tx event FIFO is full, this event is discarded and the TEFL interrupt flag is set in the
FDCAN_IR register.
When reading from the Tx event FIFO, the Tx event FIFO get index (EFGI[1:0] of
FDCAN_TXEFS) has to be added twice to the Tx event FIFO start address EFSA.

RM0456 Rev 6

<!-- pagebreak -->

