8 7

Description

T0 Bit 31
ESI(1)

Error state indicator
– 0: ESI bit in CAN FD format depends only on error passive flag
– 1: ESI bit in CAN FD format transmitted recessive

T0 Bit 30
XTD

Extended identifier
– 0: 11-bit standard identifier
– 1: 29-bit extended identifier

RM0456 Rev 6

RM0456

FD controller area network (FDCAN)
Table 724. Tx buffer element description (continued)
Field
T0 Bit 29
RTR(2)

Description
Remote transmission request
– 0: Transmit data frame
– 1: Transmit remote frame

Identifier
T0 Bits 28:0
Standard or extended identifier depending on bit XTD. A standard identifier has to be
ID[28:0]
written to ID[28:18].
Message marker
T1 Bits 31:24
Written by CPU during Tx buffer configuration. Copied into Tx event FIFO element for
MM[7:0]
identification of Tx message status.
T1 Bit 23
EFC

Event FIFO control
– 0: Do not store Tx events
– 1: Store Tx events

T1 Bit 21
FDF

FD format
– 0: Frame transmitted in classic CAN format
– 1: Frame transmitted in CAN FD format

T1 Bit 20
BRS(3)

Bit rate switching
– 0: CAN FD frames transmitted without bit rate switching
– 1: CAN FD frames transmitted with bit rate switching

Data length code
T1 Bits 19:16 – 0 - 8: Classic CAN + CAN FD: received frame has 0-8 data bytes
DLC[3:0]
– 9 - 15: Classic CAN: received frame has 8 data bytes
– 9 - 15: CAN FD: received frame has 12/16/20/24/32/48/64 data bytes
T2 Bits 31:24
Data byte 3
DB3[7:0]
T2 Bits 23:16
Data byte 2
DB2[7:0]
T2 Bits 15:8
Data byte 1
DB1[7:0]
T2 Bits 7:0
D[7:0]

Data byte 0

T3 Bits 31:24
Data byte 7
DB7[7:0]
T3 Bits 23:16
Data byte 6
DB6[7:0]
T3 Bits 15:8
Data byte 5
DB5[7:0]

...

Data byte 4

...

T3 Bits 7:0
DB4[7:0]

RM0456 Rev 6

<!-- pagebreak -->

3086

FD controller area network (FDCAN)

RM0456

Table 724. Tx buffer element description (continued)
Field

Description

Tn Bits 31:24
Data byte m
DBm[7:0]
Tn Bits 23:16
Data byte m-1
DBm-1[7:0]
Tn Bits 15:8
Data byte m-2
DBm-2[7:0]
Tn Bits 7:0
DBm-3[7:0]

Data byte m-3

1. The ESI bit of the transmit buffer is OR-ed with the error passive flag to decide the value of the ESI bit in
the transmitted FD frame. As required by the CAN FD protocol specification, an error active node can
optionally transmit the ESI bit recessive, but an error passive node always transmits the ESI bit recessive.
2. When RTR = 1, the FDCAN transmits a remote frame according to ISO11898-1, even if the transmission in
CAN FD format is enabled by the FDOE bit of the FDCAN_CCCR.
3. Bits ESI, FDF, and BRS are only evaluated when CAN FD operation is enabled by setting the FDOE bit of
the FDCAN_CCCR. Bit BRS is only evaluated when in addition BRSE bit is set in FDCAN_CCCR.

70.3.10

FDCAN Tx event FIFO element
Each element stores information about transmitted messages. By reading the Tx event,
FIFO the host CPU gets this information in the order that the messages were transmitted.
Status information about the Tx event FIFO can be obtained from FDCAN_TXEFS register.
Table 725. Tx event FIFO element
Bit 31
E0

ESI

E1

24 23
XTD

16 15

RTR

MM[7:0]

8 7

0

ID[28:0]
ET[1:0]

EDL

BRS

DLC[3:0]

TXTS[15:0]

Table 726. Tx event FIFO element description
Field

Description

E0 Bit 31
ESI

Error state indicator
– 0: Transmitting node is error active
– 1: Transmitting node is error passive

E0 Bit 30
XTD

Extended identifier
– 0: 11-bit standard identifier
– 1: 29-bit extended identifier

E0 Bit 29
RTR

Remote transmission request
– 0: Transmit data frame
– 1: Transmit remote frame

Identifier
E0 Bits 28:0
Standard or extended identifier depending on bit XTD. A standard identifier has to be
ID[28:0]
written to ID[28:18].

<!-- pagebreak -->

RM0456 Rev 6

RM0456

FD controller area network (FDCAN)
Table 726. Tx event FIFO element description (continued)
Field

Description

Message marker
E1 Bits 31:24
Copied from Tx buffer into Tx event FIFO element for identification of Tx message
MM[7:0]
status.
Event type
– 00: Reserved
E1 Bits 23:22 – 01: Tx event
EFC
– 10: Transmission in spite of cancellation (always set for transmissions in DAR
mode)
– 11: Reserved
E1 Bit 21
EDL

Extended data length
– 0: Standard frame format
– 1: FDCAN frame format (new DLC-coding and CRC)

E1 Bit 20
BRS

Bit rate switching
– 0: Frame transmitted without bit rate switching
– 1: Frame transmitted with bit rate switching

Data length code
T1 Bits 19:16
0 - 8: Frame with 0-8 data bytes transmitted
DLC[3:0]
9 - 15: Frame with 8 data bytes transmitted
Tx Timestamp
E1 Bits 15:0 Timestamp counter value captured on start of frame transmission. Resolution
TXTS[15:0] depending on configuration of the timestamp counter prescaler TCP[3:0] of
FDCAN_TSCC.

70.3.11

FDCAN standard message ID filter element
Up to 28 filter elements can be configured for 11-bit standard IDs. When accessing a
standard message ID filter element, its address is the filter list standard start address
FLSSA plus the index of the filter element (0 … 27).
Table 727. Standard message ID filter element
Bit

31

S0

SFT[1:0]

24 23
SFEC[2:0]

16 15
SFID1[10:0]

RM0456 Rev 6

Res.

8 7

0

SFID2[10:0]

<!-- pagebreak -->

