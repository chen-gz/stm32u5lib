3086

FD controller area network (FDCAN)

RM0456

Table 722. Rx FIFO element description (continued)
Field

Description

R3 Bits 15:8
Data byte 5
DB5[7:0]

...

Data byte 4

...

R3 Bits 7:0
DB4[7:0]

Rn Bits 31:24
Data byte m
DBm[7:0]
Rn Bits 23:16
Data byte m-1
DBm-1[7:0]
Rn Bits 15:8
Data byte m-2
DBm-2[7:0]
Rn Bits 7:0
DBm-3[7:0]

70.3.9

Data byte m-3

FDCAN Tx buffer element
The Tx buffers section (three elements) can be configured to hold Tx FIFO or Tx queue. The
Tx handler distinguishes between Tx FIFO and Tx queue using the Tx buffer configuration
TFQM bit of the FDCAN_TXBC register. The element size is configured for storage of CAN
FD messages with up to 64-byte data.
Table 723. Tx buffer and FIFO element
Bit 31
T0

ESI

24 23
XTD

16 15

RTR

0

ID[28:0]

T2

DB3[7:0]

DB2[7:0]

DB1[7:0]

D[7:0]

T3

DB7[7:0]

DB6[7:0]

DB5[7:0]

DB4[7:0]
...

Res.

...

EFC Res. FDF BRS DLC[3:0]

...

MM[7:0]

...

T1

Tn

DBm[7:0]

DBm-1[7:0]

DBm-2[7:0] DBm-3[7:0]

Table 724. Tx buffer element description
Field

<!-- pagebreak -->

