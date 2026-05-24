RM0456 Rev 6

RM0456

USB on-the-go high-speed (OTG_HS)
Figure 928. Receive FIFO read task

•

Bulk and control OUT/SETUP transactions
A typical bulk or control OUT/SETUP pipelined transaction-level operation is shown in
Figure 929. See channel 1 (ch_1). Two bulk OUT packets are transmitted. A control
SETUP transaction operates in the same way but has only one packet. The
assumptions are:

•

–

The application is attempting to send two maximum-packet-size packets (transfer
size = 1, 024 bytes).

–

The non-periodic transmit FIFO can hold two packets (1 Kbyte for HS).

–

The non-periodic request queue depth = 4.

Normal bulk and control OUT/SETUP operations
The sequence of operations in (channel 1) is as follows:

1.

Initialize channel 1

2.

Write the first packet for channel 1

3.

Along with the last word write, the core writes an entry to the non-periodic request
queue

4.

As soon as the non-periodic queue becomes non-empty, the core attempts to send an
OUT token in the current frame

5.

Write the second (last) packet for channel 1

6.

The core generates the XFRC interrupt as soon as the last transaction is completed
successfully

7.

In response to the XFRC interrupt, de-allocate the channel for other transfers

8.

Handling non-ACK responses

RM0456 Rev 6

<!-- pagebreak -->

3460

USB on-the-go high-speed (OTG_HS)

RM0456

Figure 929. Normal bulk/control OUT/SETUP
Application

1
init_reg(ch_2)

set_ch_en
(ch_2)

1

set_ch_en
(ch_2)

Host

USB

Device

init_reg(ch_1)

write_tx_fifo
(ch_1)

2
2

AHB

1
MPS

4

3

Non-Periodic Request
Queue
Assume that this queue
can hold 4 entries.

ch_1
write_tx_fifo
(ch_1)

5

1
MPS

ch_2
ch_1
ch_2

OU T

D AT A0
MPS

3
ACK
set_ch_en
(ch_2)

IN

4

D AT A0

5
RxFLvl interrupt
1
MPS

read_rx_sts
read_rx_fifo

ch_1
ch_2

set_ch_en
(ch_2)

ch_2

ACK
O UT

D AT A1
MPS

ch_2

7

ACK

XferCompl interrupt

6
IN

De-allocate
(ch_1)

D AT A1
RxFLvl interrupt
1
MPS

read_rx_stsre
ad_rx_fifo

RxFLvl interrupt
read_rx_sts

Disable
(ch_2)

7

6
8

ACK

ch_2

XferCompl interrupt

9
RxFLvl interrupt

read_rx_sts

De-allocate
(ch_2)

11

ChHltd interrupt

10
12

13
MSv36018V1

1. The grayed elements are not relevant in the context of this figure.

<!-- pagebreak -->

