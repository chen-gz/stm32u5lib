3086

FD controller area network (FDCAN)

70.3.7

RM0456

FIFO acknowledge handling
The get indices of Rx FIFO 0, Rx FIFO 1, and the Tx event FIFO are controlled by writing to
the corresponding FIFO acknowledge index (see Section 70.4.23 and Section 70.4.25).
Writing to the FIFO acknowledge index sets the FIFO get index to the FIFO acknowledge
index plus one and thereby updates the FIFO fill level. There are two use cases:
•

When only a single element has been read from the FIFO (the one being pointed to by
the get index), this get index value is written to the FIFO acknowledge index.

•

When a sequence of elements has been read from the FIFO, it is sufficient to write the
FIFO acknowledge index only once at the end of that read sequence (value = index of
the last element read), to update the FIFO get index.

Because the CPU has free access to the FDCAN message RAM, special care has to be
taken when reading FIFO elements in an arbitrary order (get index not considered). This
might be useful when reading a high priority message from one of the two Rx FIFOs. In this
case, the FIFO acknowledge index must not be written because this would set the get index
to a wrong position and alter the FIFO fill level. In this case, some of the older FIFO
elements would be lost.
Note:

The application has to ensure that a valid value is written to the FIFO acknowledge index.
The FDCAN does not check for erroneous values.

70.3.8

FDCAN Rx FIFO element
Two Rx FIFOs are configured in the message RAM. Each Rx FIFO section can be
configured to store up to three received messages. The structure of an Rx FIFO element is
described in Table 721. The description is provided in Table 722.
Table 721. Rx FIFO element
Bit 31
R0

ESI

R1

ANMF

24 23
XTD

16 15

RTR
FIDX[6:0]

8 7

0

ID[28:0]
Res.

FDF

BRS DLC[3:0]

RXTS[15:0]
D[7:0]

R3

DB7[7:0]

DB6[7:0]

DB5[7:0]

DB4[7:0]

Rn

DBm[7:0]

DBm-1[7:0]

...

DB1[7:0]

...

DB2[7:0]

...

DB3[7:0]

...

R2

DBm-2[7:0]

DBm-3[7:0]

The element size configured for storage of CAN FD messages is set to 64-byte data field.
Table 722. Rx FIFO element description
Field

<!-- pagebreak -->

Description

R0 Bit 31
ESI

Error state indicator
– 0: Transmitting node is error active
– 1: Transmitting node is error passive

R0 Bit 30
XTD

Extended identifier
Signals to the host whether the received frame has a standard or extended identifier.
– 0: 11-bit standard identifier
– 1: 29-bit extended identifier

RM0456 Rev 6

RM0456

FD controller area network (FDCAN)
Table 722. Rx FIFO element description (continued)
Field
R0 Bit 29
RTR

Description
Remote transmission request
Signals to the host whether the received frame is a data frame or a remote frame.
– 0: Received frame is a data frame
– 1: Received frame is a remote frame

Identifier
R0 Bits 28:0
Standard or extended identifier depending on bit XTD. A standard identifier is stored
ID[28:0]
into ID[28:18].

R1 Bit 31
ANMF

Accepted non-matching frame
Acceptance of non-matching frames can be enabled via ANFS[1:0] and ANFE[1:0]
bitfield of FDCAN_RXGFC.
– 0: Received frame matching filter index FIDX
– 1: Received frame did not match any Rx filter element

Filter index
R1 Bits 30:24
0-27=Index of matching Rx acceptance filter element (invalid if ANMF = 1).
FIDX[6:0]
Range: 0 to LSS[4:0] - 1 or LSE[3:0] - 1 in FDCAN_RXGFC.
R1 Bit 21
FDF

FD format
– 0: Standard frame format
– 1: FDCAN frame format (new DLC-coding and CRC)

R1 Bit 20
BRS

Bit rate switch
– 0: Frame received without bit rate switching
– 1: Frame received with bit rate switching

Data length code
R1 Bits 19:16 – 0-8: Classic CAN + CAN FD: received frame has 0-8 data bytes
– 9-15: Classic CAN: received frame has 8 data bytes
DLC[3:0]
– 9-15: CAN FD: received frame has 12/16/20/24/32/48/64 data bytes
Rx timestamp
R1 Bits 15:0
Timestamp Counter value captured on start of frame reception. Resolution depending
RXTS[15:0]
on configuration of the timestamp counter prescaler TCP[3:0] of FDCAN_TSCC.
R2 Bits 31:24
Data byte 3
DB3[7:0]
R2 Bits 23:16
Data byte 2
DB2[7:0]
R2 Bits 15:8
Data byte 1
DB1[7:0]
R2 Bits 7:0
D[7:0]

Data byte 0

R3 Bits 31:24
Data byte 7
DB7[7:0]
R3 Bits 23:16
Data byte 6
DB6[7:0]

RM0456 Rev 6

<!-- pagebreak -->

