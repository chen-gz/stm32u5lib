RM0456 Rev 6

RM0456

USB on-the-go high-speed (OTG_HS)
else if (DTERR)
{
Reset Error Count
}

The application is expected to write the requests as and when the request queue space is
available and until the XFRC interrupt is received.
•

Bulk and control IN transactions
A typical bulk or control IN pipelined transaction-level operation is shown in Figure 930.
See channel 2 (ch_2). The assumptions are:
–

The application is attempting to receive two maximum-packet-size packets
(transfer size = 1 024 bytes).

–

The receive FIFO can contain at least one maximum-packet-size packet and two
status words per packet (520 bytes for HS).

–

The non-periodic request queue depth = 4.

RM0456 Rev 6

<!-- pagebreak -->

3460

USB on-the-go high-speed (OTG_HS)

RM0456

Figure 930. Bulk/control IN transactions
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
ai15675b

1. The grayed elements are not relevant in the context of this figure.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

USB on-the-go high-speed (OTG_HS)
The sequence of operations is as follows:
1.

Initialize channel 2.

2.

Set the CHENA bit in OTG_HCCHAR2 to write an IN request to the non-periodic
request queue.

3.

The core attempts to send an IN token after completing the current OUT transaction.

4.

The core generates an RXFLVL interrupt as soon as the received packet is written to
the receive FIFO.

5.

In response to the RXFLVL interrupt, mask the RXFLVL interrupt and read the received
packet status to determine the number of bytes received, then read the receive FIFO
accordingly. Following this, unmask the RXFLVL interrupt.

6.

The core generates the RXFLVL interrupt for the transfer completion status entry in the
receive FIFO.

7.

The application must read and ignore the receive packet status when the receive
packet status is not an IN data packet (PKTSTS in OTG_GRXSTSR ≠ 0b0010).

8.

The core generates the XFRC interrupt as soon as the receive packet status is read.

9.

In response to the XFRC interrupt, disable the channel and stop writing the
OTG_HCCHAR2 register for further requests. The core writes a channel disable
request to the non-periodic request queue as soon as the OTG_HCCHAR2 register is
written.

10. The core generates the RXFLVL interrupt as soon as the halt status is written to the
receive FIFO.
11. Read and ignore the receive packet status.
12. The core generates a CHH interrupt as soon as the halt status is popped from the
receive FIFO.
13. In response to the CHH interrupt, de-allocate the channel for other transfers.
14. Handling non-ACK responses
•

Control transactions
Setup, data, and status stages of a control transfer must be performed as three
separate transfers. setup-, data- or status-stage OUT transactions are performed
similarly to the bulk OUT transactions explained previously. Data- or status-stage IN
transactions are performed similarly to the bulk IN transactions explained previously.
For all three stages, the application is expected to set the EPTYP field in

RM0456 Rev 6

<!-- pagebreak -->

3460

USB on-the-go high-speed (OTG_HS)

RM0456

OTG_HCCHAR1 to control. During the setup stage, the application is expected to set
the PID field in OTG_HCTSIZ1 to SETUP.
•

Interrupt OUT transactions
A typical interrupt OUT operation is shown in Figure 931. The assumptions are:
–

The application is attempting to send one packet in every frame (up to 1 maximum
packet size), starting with the odd frame (transfer size = 1 024 bytes)

–

The periodic transmit FIFO can hold one packet (1 Kbyte)

–

Periodic request queue depth = 4

The sequence of operations is as follows:

<!-- pagebreak -->

1.

Initialize and enable channel 1. The application must set the ODDFRM bit in
OTG_HCCHAR1.

2.

Write the first packet for channel 1.

3.

Along with the last word write of each packet, the OTG_HS host writes an entry to the
periodic request queue.

4.

The OTG_HS host attempts to send an OUT token in the next (odd) frame.

5.

The OTG_HS host generates an XFRC interrupt as soon as the last packet is
transmitted successfully.

6.

In response to the XFRC interrupt, reinitialize the channel for the next transfer.

RM0456 Rev 6

RM0456

USB on-the-go high-speed (OTG_HS)
Figure 931. Normal interrupt OUT
Application

1

AHB

Host

USB

init_reg(ch_1)

init_reg(ch_2)

1

write_tx_fifo
(ch_1)

2

set_ch_en
(ch_2)

Device

Periodic Request Queue
Assume that this queue
can hold 4 entries.

3

1
MPS

4

ch_1

2

ch_2

3
OU T

Odd
(micro)

D ATA0
M PS

frame

5
6

AC K

XferCompl interrupt

4

init_reg(ch_1)
write_tx_fifo
(ch_1)

IN

5

1
MPS

D ATA0

RxFLvl interrupt
ACK

read_rx_sts
read_rx_fifo

1
MPS

6

RxFLvl interrupt
read_rx_sts

7

init_reg(ch_2)

XferCompl interrupt

8

ch_1
ch_2

9

set_ch_en
(ch_2)
Even
OU T

frame

XferCompl interrupt
D ATA1
MPS

init_reg(ch_1)

write_tx_fifo
(ch_1)

(micro)

1
MPS

ACK
IN

D ATA1

MSv36020V1

1. The grayed elements are not relevant in the context of this figure.

•

Interrupt service routine for interrupt OUT/IN transactions
a)

Interrupt OUT

Unmask (NAK/TXERR/STALL/XFRC/FRMOR)

RM0456 Rev 6

<!-- pagebreak -->

3460

USB on-the-go high-speed (OTG_HS)

RM0456

if (XFRC)
{
Reset Error Count
Mask ACK
De-allocate Channel
}
else
if (STALL or FRMOR)
{
Mask ACK
Unmask CHH
Disable Channel
if (STALL)
{
Transfer Done = 1
}
}
else
if (NAK or TXERR)
{
Rewind Buffer Pointers
Reset Error Count
Mask ACK
Unmask CHH
Disable Channel
}
else
if (CHH)
{
Mask CHH
if (Transfer Done or (Error_count == 3))
{
De-allocate Channel
}
else
{
Re-initialize Channel (in next b_interval - 1 Frame)
}
}
else
if (ACK)
{
Reset Error Count
Mask ACK
}

<!-- pagebreak -->

RM0456 Rev 6

RM0456

USB on-the-go high-speed (OTG_HS)
The application uses the NPTXFE interrupt in OTG_GINTSTS to find the
transmit FIFO space.
Interrupt IN
Unmask (NAK/TXERR/XFRC/BBERR/STALL/FRMOR/DTERR)
if (XFRC)
{
Reset Error Count
Mask ACK
if (OTG_HCTSIZx.PKTCNT == 0)
{
De-allocate Channel
}
else
{
Transfer Done = 1
Unmask CHH
Disable Channel
}
}
else
if (STALL or FRMOR or NAK or DTERR or BBERR)
{
Mask ACK
Unmask CHH
Disable Channel
if (STALL or BBERR)
{
Reset Error Count
Transfer Done = 1
}
else
if (!FRMOR)
{
Reset Error Count
}
}
else
if (TXERR)
{
Increment Error Count
Unmask ACK
Unmask CHH
Disable Channel
}
else

RM0456 Rev 6

<!-- pagebreak -->

3460

USB on-the-go high-speed (OTG_HS)

RM0456

if (CHH)
{
Mask CHH
if (Transfer Done or (Error_count == 3))
{
De-allocate Channel
}
else
Re-initialize Channel (in next b_interval - 1 /Frame)
}
}
else
if (ACK)
{
Reset Error Count
Mask ACK
}

•

Interrupt IN transactions
The assumptions are:

•

–

The application is attempting to receive one packet (up to 1 maximum packet size)
in every frame, starting with odd (transfer size = 1 024 bytes).

–

The receive FIFO can hold at least one maximum-packet-size packet and two
status words per packet (1 031 bytes).

–

Periodic request queue depth = 4.

Normal interrupt IN operation
The sequence of operations is as follows:

<!-- pagebreak -->

1.

Initialize channel 2. The application must set the ODDFRM bit in OTG_HCCHAR2.

2.

Set the CHENA bit in OTG_HCCHAR2 to write an IN request to the periodic request
queue.

3.

The OTG_HS host writes an IN request to the periodic request queue for each
OTG_HCCHAR2 register write with the CHENA bit set.

4.

The OTG_HS host attempts to send an IN token in the next (odd) frame.

5.

As soon as the IN packet is received and written to the receive FIFO, the OTG_HS host
generates an RXFLVL interrupt.

6.

In response to the RXFLVL interrupt, read the received packet status to determine the
number of bytes received, then read the receive FIFO accordingly. The application
must mask the RXFLVL interrupt before reading the receive FIFO, and unmask after
reading the entire packet.

7.

The core generates the RXFLVL interrupt for the transfer completion status entry in the
receive FIFO. The application must read and ignore the receive packet status when the
receive packet status is not an IN data packet (PKTSTS in GRXSTSR ≠ 0b0010).

8.

The core generates an XFRC interrupt as soon as the receive packet status is read.

9.

In response to the XFRC interrupt, read the PKTCNT field in OTG_HCTSIZ2. If the
PKTCNT bit in OTG_HCTSIZ2 is not equal to 0, disable the channel before re-

RM0456 Rev 6

RM0456

USB on-the-go high-speed (OTG_HS)
initializing the channel for the next transfer, if any). If PKTCNT bit in OTG_HCTSIZ2 =
0, reinitialize the channel for the next transfer. This time, the application must reset the
ODDFRM bit in OTG_HCCHAR2.

RM0456 Rev 6

<!-- pagebreak -->

3460

USB on-the-go high-speed (OTG_HS)

RM0456

Figure 932. Normal interrupt IN
Application

1

AHB

Host

USB

init_reg(ch_1)

init_reg(ch_2)

1

write_tx_fifo
(ch_1)

2

set_ch_en
(ch_2)

Device

Periodic Request Queue
Assume that this queue
can hold 4 entries.

3

1
MPS

4

ch_1

2

ch_2

3
OU T

Odd
(micro)

D ATA0
M PS

frame

5
6

AC K

Xfer Com pl i nterr upt

4

init_reg(ch_1)
write_tx_fifo
(ch_1)

IN

5

1
MPS

D ATA0

RxFLvl interrupt
ACK

read_rx_sts
read_rx_fifo

1
MPS

6

RxFLvl interrupt
read_rx_sts

7

init_reg(ch_2)

XferCompl interrupt

8

ch_1
ch_2

9

set_ch_en
(ch_2)
Even
OU T

frame

Xfer Compl interr upt
D ATA1
MPS

init_reg(ch_1)

write_tx_fifo
(ch_1)

(micro)

1
MPS

ACK
IN

D ATA1

ai15676b

1. The grayed elements are not relevant in the context of this figure.

•

Isochronous OUT transactions
A typical isochronous OUT operation is shown in Figure 933. The assumptions are:
–

<!-- pagebreak -->

The application is attempting to send one packet every frame (up to 1 maximum

RM0456 Rev 6

RM0456

USB on-the-go high-speed (OTG_HS)
packet size), starting with an odd frame. (transfer size = 1 024 bytes).
–

The periodic transmit FIFO can hold one packet (1 Kbyte).

–

Periodic request queue depth = 4.

The sequence of operations is as follows:
1.

Initialize and enable channel 1. The application must set the ODDFRM bit in
OTG_HCCHAR1.

2.

Write the first packet for channel 1.

3.

Along with the last word write of each packet, the OTG_HS host writes an entry to the
periodic request queue.

4.

The OTG_HS host attempts to send the OUT token in the next frame (odd).

5.

The OTG_HS host generates the XFRC interrupt as soon as the last packet is
transmitted successfully.

6.

In response to the XFRC interrupt, reinitialize the channel for the next transfer.

7.

Handling non-ACK responses

RM0456 Rev 6

<!-- pagebreak -->

3460

USB on-the-go high-speed (OTG_HS)

RM0456

Figure 933. Isochronous OUT transactions
Application

1

AHB

Host

USB

init_reg(ch_1)
Periodic Request
Queue
Assume that this queue
can hold 4 entries.

init_reg(ch_2)

1

write_tx_fifo
(ch_1)

2

set_ch_en
(ch_2)

Device

3

1
MPS

4

ch_1

2

ch_2

3
OUT

Odd
(micro)
frame

D A T A0
MPS

5
6

XferCompl interrupt
init_reg(ch_1)
write_tx_fifo
(ch_1)

4
1
MPS

5

IN

D A T A0

RxFLvl interrupt
1
MPS

6

read_rx_sts
read_rx_fifo

RxFLvl interrupt
read_rx_sts

7
XferCompl interrupt

init_reg(ch_2)

8

ch_1
ch_2

9

set_ch_en
(ch_2)
Even
OUT

(micro)
frame

D A T A0
MPS

XferCompl interrupt
init_reg(ch_1)
IN

write_tx_fifo
(ch_1)

1
MPS

DA TA
0

MSv36022V1

1. The grayed elements are not relevant in the context of this figure.

•

Interrupt service routine for isochronous OUT/IN transactions
Code sample: isochronous OUT

Unmask (FRMOR/XFRC)
if (XFRC)

<!-- pagebreak -->

RM0456 Rev 6

RM0456

USB on-the-go high-speed (OTG_HS)
{
De-allocate Channel
}
else
if (FRMOR)
{
Unmask CHH
Disable Channel
}
else
if (CHH)
{
Mask CHH
De-allocate Channel
}
Code sample: Isochronous IN
Unmask (TXERR/XFRC/FRMOR/BBERR)
if (XFRC or FRMOR)
{
if (XFRC and (OTG_HCTSIZx.PKTCNT == 0))
{
Reset Error Count
De-allocate Channel
}
else
{
Unmask CHH
Disable Channel
}
}
else
if (TXERR or BBERR)
{
Increment Error Count
Unmask CHH
Disable Channel
}
else
if (CHH)
{
Mask CHH
if (Transfer Done or (Error_count == 3))
{
De-allocate Channel
}

RM0456 Rev 6

<!-- pagebreak -->

3460

USB on-the-go high-speed (OTG_HS)

RM0456

else
{
Re-initialize Channel
}
}

•

Isochronous IN transactions
The assumptions are:
–

The application is attempting to receive one packet (up to 1 maximum packet size)
in every frame starting with the next odd frame (transfer size = 1 024 bytes).

–

The receive FIFO can hold at least one maximum-packet-size packet and two
status word per packet (1 031 bytes).

–

Periodic request queue depth = 4.

The sequence of operations is as follows:

<!-- pagebreak -->

1.

Initialize channel 2. The application must set the ODDFRM bit in OTG_HCCHAR2.

2.

Set the CHENA bit in OTG_HCCHAR2 to write an IN request to the periodic request
queue.

3.

The OTG_HS host writes an IN request to the periodic request queue for each
OTG_HCCHAR2 register write with the CHENA bit set.

4.

The OTG_HS host attempts to send an IN token in the next odd frame.

5.

As soon as the IN packet is received and written to the receive FIFO, the OTG_HS host
generates an RXFLVL interrupt.

6.

In response to the RXFLVL interrupt, read the received packet status to determine the
number of bytes received, then read the receive FIFO accordingly. The application
must mask the RXFLVL interrupt before reading the receive FIFO, and unmask it after
reading the entire packet.

7.

The core generates an RXFLVL interrupt for the transfer completion status entry in the
receive FIFO. This time, the application must read and ignore the receive packet status
when the receive packet status is not an IN data packet (PKTSTS bit in
OTG_GRXSTSR ≠ 0b0010).

8.

The core generates an XFRC interrupt as soon as the receive packet status is read.

9.

In response to the XFRC interrupt, read the PKTCNT field in OTG_HCTSIZ2. If
PKTCNT ≠ 0 in OTG_HCTSIZ2, disable the channel before re-initializing the channel
for the next transfer, if any. If PKTCNT = 0 in OTG_HCTSIZ2, reinitialize the channel
for the next transfer. This time, the application must reset the ODDFRM bit in
OTG_HCCHAR2.

RM0456 Rev 6

RM0456

USB on-the-go high-speed (OTG_HS)
Figure 934. Isochronous IN transactions
Application

1

AHB

Host

USB

init_reg(ch_1)
Periodic Request
Queue
Assume that this queue
can hold 4 entries.

init_reg(ch_2)

1

write_tx_fifo
(ch_1)

2

set_ch_en
(ch_2)

Device

3

1
MPS

4

ch_1

2

ch_2

3
Odd

OUT

(micro)
frame

DATA0
MPS

5
6

XferCompl interrupt
init_reg(ch_1)
write_tx_fifo
(ch_1)

4
1
MPS

5

IN

DATA0

RxFLvl interrupt
1
MPS

6

read_rx_sts
read_rx_fifo

RxFLvl interrupt
read_rx_sts

7
XferCompl interrupt

init_reg(ch_2)

8

ch_1
ch_2

9

set_ch_en
(ch_2)
Even
OUT

(micro)
frame

DATA0
MPS

XferCompl interrupt
init_reg(ch_1)
IN

write_tx_fifo
(ch_1)

1
MPS
DATA0

MSv36021V1

1. The grayed elements are not relevant in the context of this figure.

•

Selecting the queue depth
Choose the periodic and non-periodic request queue depths carefully to match the
number of periodic/non-periodic endpoints accessed.
The non-periodic request queue depth affects the performance of non-periodic
RM0456 Rev 6

<!-- pagebreak -->

3460

USB on-the-go high-speed (OTG_HS)

RM0456

transfers. The deeper the queue (along with sufficient FIFO size), the more often the
core is able to pipeline non-periodic transfers. If the queue size is small, the core is
able to put in new requests only when the queue space is freed up.
The core’s periodic request queue depth is critical to perform periodic transfers as
scheduled. Select the periodic queue depth, based on the number of periodic transfers
scheduled in a microframe. If the periodic request queue depth is smaller than the
periodic transfers scheduled in a microframe, a frame overrun condition occurs.
•

Handling babble conditions
OTG_HS controller handles two cases of babble: packet babble and port babble.
Packet babble occurs if the device sends more data than the maximum packet size for
the channel. Port babble occurs if the core continues to receive data from the device at
EOF2 (the end of frame 2, which is very close to SOF).
When OTG_HS controller detects a packet babble, it stops writing data into the Rx
buffer and waits for the end of packet (EOP). When it detects an EOP, it flushes already
written data in the Rx buffer and generates a Babble interrupt to the application.
When OTG_HS controller detects a port babble, it flushes the Rx FIFO and disables
the port. The core then generates a port disabled interrupt (HPRTINT in
OTG_GINTSTS, PENCHNG in OTG_HPRT). On receiving this interrupt, the
application must determine that this is not due to an overcurrent condition (another
cause of the port disabled interrupt) by checking POCA in OTG_HPRT, then perform a
soft reset. The core does not send any more tokens after it has detected a port babble
condition.

•

Bulk and control OUT/SETUP transactions in DMA mode
The sequence of operations is as follows:

<!-- pagebreak -->

1.

Initialize and enable channel 1 as explained in Section : Channel initialization.

2.

The OTG_HS host starts fetching the first packet as soon as the channel is enabled.
For internal DMA mode, the OTG_HS host uses the programmed DMA address to
fetch the packet.

3.

After fetching the last 32-bit word of the second (last) packet, the OTG_HS host masks
channel 1 internally for further arbitration.

4.

The OTG_HS host generates a CHH interrupt as soon as the last packet is sent.

5.

In response to the CHH interrupt, de-allocate the channel for other transfers.

RM0456 Rev 6

RM0456

USB on-the-go high-speed (OTG_HS)
Figure 935. Normal bulk/control OUT/SETUP transactions - DMA
Application

1

Host

AHB

USB

Device

init_reg(ch_1)

2

init_reg(ch_2)

Non-Periodic
Request Queue
Assume that this queue
can hold 4 entries.

1
1
MPS

ch_1

1
MPS

2

ch_2
OUT

ch_1

D ATA0
MPS

ch_2

3

AC K
IN

D ATA0

3
AC K

1
MPS

ch_1
OUT

ch_2

D ATA1
MPS

ch_2

5
ch_2

5

AC K

C hHltd interrupt

4
IN

De-allocate
(ch_1)

D ATA1

4
1
MPS

AC K

ch_2

6
7
ChHltd interrupt
De-allocate
(ch_2)

8
MSv36927V1

•

NAK and NYET handling with internal DMA:

1.

The OTG_HS host sends a bulk OUT transaction.

2.

The device responds with NAK or NYET.

3.

If the application has unmasked NAK or NYET, the core generates the corresponding
interrupt(s) to the application. The application is not required to service these interrupts,
since the core takes care of rewinding the buffer pointers and re-initializing the Channel
without application intervention.

4.

The core automatically issues a ping token.

5.

When the device returns an ACK, the core continues with the transfer. Optionally, the
application can utilize these interrupts, in which case the NAK or NYET interrupt is
masked by the application.

RM0456 Rev 6

<!-- pagebreak -->

3460

USB on-the-go high-speed (OTG_HS)

RM0456

The core does not generate a separate interrupt when NAK or NYET is received by the
host functionality.
•

Bulk and control IN transactions in DMA mode
The sequence of operations is as follows:

<!-- pagebreak -->

1.

Initialize and enable the used channel (channel x) as explained in Section : Channel
initialization.

2.

The OTG_HS host writes an IN request to the request queue as soon as the channel
receives the grant from the arbiter (arbitration is performed in a round-robin fashion).

3.

The OTG_HS host starts writing the received data to the system memory as soon as
the last byte is received with no errors.

4.

When the last packet is received, the OTG_HS host sets an internal flag to remove any
extra IN requests from the request queue.

5.

The OTG_HS host flushes the extra requests.

6.

The final request to disable channel x is written to the request queue. At this point,
channel 2 is internally masked for further arbitration.

7.

The OTG_HS host generates the CHH interrupt as soon as the disable request comes
to the top of the queue.

8.

In response to the CHH interrupt, de-allocate the channel for other transfers.

RM0456 Rev 6

RM0456

USB on-the-go high-speed (OTG_HS)
Figure 936. Normal bulk/control IN transaction - DMA

Application

1

Host

AHB

USB

Device

init_reg(ch_1)

2

init_reg(ch_2)

1

Non-Periodic
Request Queue
Assume that this queue
can hold 4 entries.

1
MPS
ch_1

1
MPS

2

ch_2
OUT

ch_1

DATA0
MPS

ch_2

3

AC K

IN

DATA0
3
AC K

1
MPS

ch_1
OUT

ch_2

D ATA1
MPS

ch_2

5
ch_2

5

AC K

C hHltd interrupt

4
IN

De-allocate
(ch_1)

D ATA1

4
1
MPS

AC K

ch_2

6
7
ChHltd interrupt
De-allocate
(ch_2)

8

MSv36928V1

•

Interrupt OUT transactions in DMA mode

1.

Initialize and enable channel x as explained in Section : Channel initialization.

2.

The OTG_HS host starts fetching the first packet as soon the channel is enabled and
writes the OUT request along with the last 32-bit word fetch. In high-bandwidth
transfers, the OTG_HS host continues fetching the next packet (up to the value
specified in the MC field) before switching to the next channel.

3.

The OTG_HS host attempts to send the OUT token at the beginning of the next odd
frame/micro-frame.

RM0456 Rev 6

<!-- pagebreak -->

3460

USB on-the-go high-speed (OTG_HS)

RM0456

4.

After successfully transmitting the packet, the OTG_HS host generates a CHH
interrupt.

5.

In response to the CHH interrupt, reinitialize the channel for the next transfer.
Figure 937. Normal interrupt OUT transactions - DMA mode
Application

1

Host

AHB

USB

Device

init_reg(ch_1)
Periodic Request
Queue
Assume that this
queue can hold
4 entries.

init_reg(ch_2)

1

2

1
MPS

3

ch_1
ch_2

2
O UT

DATA0
MP S

Odd
(micro)
frame

4
5

A CK

C hH ltd interrupt

init_reg(ch_1)

3

IN

1
MPS
DATA0

AC K

1
MPS

ChHltd interrupt

4

ch_1
ch_2

init_reg(ch_2)

5

OUT

Even
(micro)
frame

D ATA1
MP S

C hH ltd interrupt

init_reg(ch_1)

AC K

IN

1
MPS

DATA1

MSv36924V1

•

Interrupt IN transactions in DMA mode
The sequence of operations (channelx) is as follows:

<!-- pagebreak -->

1.

Initialize and enable channel x as explained in Section : Channel initialization.

2.

The OTG_HS host writes an IN request to the request queue as soon as the channel x
gets the grant from the arbiter (round-robin with fairness). In high-bandwidth transfers,
the OTG_HS host writes consecutive writes up to MC times.

RM0456 Rev 6

RM0456

USB on-the-go high-speed (OTG_HS)
3.

The OTG_HS host attempts to send an IN token at the beginning of the next (odd)
frame/micro-frame.

4.

As soon the packet is received and written to the receive FIFO, the OTG_HS host
generates a CHH interrupt.

5.

In response to the CHH interrupt, reinitialize the channel for the next transfer.
Figure 938. Normal interrupt IN transactions - DMA mode
Application

1

AHB

USB

Host

Device

init_reg(ch_1)
Periodic Request
Queue
Assume that this
queue can hold
4 entries.

init_reg(ch_2)

1

2

1
MPS

3

ch_1
ch_2

2
O UT

DATA0
MP S

Odd
(micro)
frame

4
5
A CK

C h H ltd interrupt

init_reg(ch_1)

3

IN

1
MPS
DATA0

AC K

1
MPS

ChHltd interrupt

4

ch_1
ch_2

init_reg(ch_2)

5

OUT

Even
(micro)
frame

D A TA 1
MP S

C h H ltd inte rrupt

init_reg(ch_1)

AC K

IN

1
MPS

DATA1

MSv36925V1

•

Isochronous OUT transactions in DMA mode

1.

Initialize and enable channel x as explained in Section : Channel initialization.

2.

The OTG_HS host starts fetching the first packet as soon as the channel is enabled,
and writes the OUT request along with the last 32-bit word fetch. In high-bandwidth

RM0456 Rev 6

<!-- pagebreak -->

3460

USB on-the-go high-speed (OTG_HS)

RM0456

transfers, the OTG_HS host continues fetching the next packet (up to the value
specified in the MC field) before switching to the next channel.
3.

The OTG_HS host attempts to send an OUT token at the beginning of the next (odd)
frame/micro-frame.

4.

After successfully transmitting the packet, the OTG_HS host generates a CHH
interrupt.

5.

In response to the CHH interrupt, reinitialize the channel for the next transfer.
Figure 939. Normal isochronous OUT transaction - DMA mode

Application
1

Host

AHB

USB

Device

init_reg(ch_1)
Periodic Request
Queue
Assume that this
queue can hold
4 entries.

init_reg(ch_2)

1

2

1
MPS

3

ch_1
ch_2

2
OUT

DATA0
MPS

4

Odd
(micro)
frame

5
C hHltd interrupt

init_reg(ch_1)

3

IN

1
MPS

DATA0

1
MPS

ChHltd interrupt
init_reg(ch_2)

4

ch_1
ch_2

5

OUT

Even
(micro)
frame

DATA0
MPS
C hH ltd interrupt

init_reg(ch_1)
IN

1
MPS

DATA0

MSv36923V1

•

Isochronous IN transactions in DMA mode
The sequence of operations ((channel x) is as follows:

<!-- pagebreak -->

1.

Initialize and enable channel x as explained in Section : Channel initialization.

2.

The OTG_HS host writes an IN request to the request queue as soon as the channel x
gets the grant from the arbiter (round-robin with fairness). In high-bandwidth transfers,
the OTG_HS host performs consecutive write operations up to MC times.

RM0456 Rev 6

RM0456

USB on-the-go high-speed (OTG_HS)
3.

The OTG_HS host attempts to send an IN token at the beginning of the next (odd)
frame/micro-frame.

4.

As soon the packet is received and written to the receive FIFO, the OTG_HS host
generates a CHH interrupt.

5.

In response to the CHH interrupt, reinitialize the channel for the next transfer.
Figure 940. Normal isochronous IN transactions - DMA mode

Application
1

Host

AHB

USB

Device

init_reg(ch_1)
Periodic Request
Queue
Assume that this
queue can hold
4 entries.

init_reg(ch_2)

1

2

1
MPS

3

ch_1
ch_2

2
OUT

Odd
(micro)
frame

DATA0
MPS

4
5
C hHltd interrupt

init_reg(ch_1)

3

IN

1
MPS

DATA0

1
MPS

ChHltdInterrupt

4

ch_1
ch_2

init_reg(ch_2)

5

OUT

Even
(micro)
frame

DATA0
MPS
C hH ltd interrupt

init_reg(ch_1)
IN

1
MPS
DATA0

MS36922V1

•

Bulk and control OUT/SETUP split transactions in DMA mode
The sequence of operations in (channel x) is as follows:

1.

Initialize and enable channel x for start split as explained in Section : Channel
initialization.

2.

The OTG_HS host starts fetching the first packet as soon the channel is enabled and
writes the OUT request along with the last 32-bit word fetch.

3.

After successfully transmitting start split, the OTG_HS host generates the CHH
interrupt.

4.

In response to the CHH interrupt, set the COMPLSPLT bit in OTG_HCSPLT1 to send
the complete split.

RM0456 Rev 6

<!-- pagebreak -->

3460

USB on-the-go high-speed (OTG_HS)

RM0456

5.

After successfully transmitting complete split, the OTG_HS host generates the CHH
interrupt.

6.

In response to the CHH interrupt, de-allocate the channel.

•

Bulk/control IN split transactions in DMA mode
The sequence of operations (channel x) is as follows:

1.

Initialize and enable channel x as explained in Section : Channel initialization.

2.

The OTG_HS host writes the start split request to the nonperiodic request after getting
the grant from the arbiter. The OTG_HS host masks the channel x internally for the
arbitration after writing the request.

3.

As soon as the IN token is transmitted, the OTG_HS host generates the CHH interrupt.

4.

In response to the CHH interrupt, set the COMPLSPLT bit in OTG_HCSPLT2 and reenable the channel to send the complete split token. This unmasks channel x for
arbitration.

5.

The OTG_HS host writes the complete split request to the nonperiodic request after
receiving the grant from the arbiter.

6.

The OTG_HS host starts writing the packet to the system memory after receiving the
packet successfully.

7.

As soon as the received packet is written to the system memory, the OTG_HS host
generates a CHH interrupt.

8.

In response to the CHH interrupt, de-allocate the channel.

•

Interrupt OUT split transactions in DMA mode
The sequence of operations in (channel x) is as follows:

1.

Initialize and enable channel 1 for start split as explained in Section : Channel
initialization. The application must set the ODDFRM bit in OTG_HCCHAR1.

2.

The OTG_HS host starts reading the packet.

3.

The OTG_HS host attempts to send the start split transaction.

4.

After successfully transmitting the start split, the OTG_HS host generates the CHH
interrupt.

5.

In response to the CHH interrupt, set the COMPLSPLT bit in OTG_HCSPLT1 to send
the complete split.

6.

After successfully completing the complete split transaction, the OTG_HS host
generates the CHH interrupt.

7.

In response to CHH interrupt, de-allocate the channel.

•

Interrupt IN split transactions in DMA mode
The sequence of operations in (channel x) is as follows:

<!-- pagebreak -->

1.

Initialize and enable channel x for start split as explained in Section : Channel
initialization.

2.

The OTG_HS host writes an IN request to the request queue as soon as channel x
receives the grant from the arbiter.

3.

The OTG_HS host attempts to send the start split IN token at the beginning of the next
odd micro-frame.

4.

The OTG_HS host generates the CHH interrupt after successfully transmitting the start
split IN token.

5.

In response to the CHH interrupt, set the COMPLSPLT bit in OTG_HCSPLT2 to send
the complete split.

RM0456 Rev 6

RM0456

USB on-the-go high-speed (OTG_HS)
6.

As soon as the packet is received successfully, the OTG_HS host starts writing the
data to the system memory.

7.

The OTG_HS host generates the CHH interrupt after transferring the received data to
the system memory.

8.

In response to the CHH interrupt, de-allocate or reinitialize the channel for the next start
split.

•

Isochronous OUT split transactions in DMA mode
The sequence of operations (channel x) is as follows:

1.

Initialize and enable channel x for start split (begin) as explained in Section : Channel
initialization. The application must set the ODDFRM bit in OTG_HCCHAR1. Program
the MPS field.

2.

The OTG_HS host starts reading the packet.

3.

After successfully transmitting the start split (begin), the OTG_HS host generates the
CHH interrupt.

4.

In response to the CHH interrupt, reinitialize the registers to send the start split (end).

5.

After successfully transmitting the start split (end), the OTG_HS host generates a CHH
interrupt.

6.

In response to the CHH interrupt, de-allocate the channel.

•

Isochronous IN split transactions in DMA mode
The sequence of operations (channel x) is as follows:

1.

Initialize and enable channel x for start split as explained in Section : Channel
initialization.

2.

The OTG_HS host writes an IN request to the request queue as soon as channel x
receives the grant from the arbiter.

3.

The OTG_HS host attempts to send the start split IN token at the beginning of the next
odd micro-frame.

4.

The OTG_HS host generates the CHH interrupt after successfully transmitting the start
split IN token.

5.

In response to the CHH interrupt, set the COMPLSPLT bit in OTG_HCSPLT2 to send
the complete split.

6.

As soon as the packet is received successfully, the OTG_HS host starts writing the
data to the system memory.
The OTG_HS host generates the CHH interrupt after transferring the received data to
the system memory. In response to the CHH interrupt, de-allocate the channel or
reinitialize the channel for the next start split.

RM0456 Rev 6

<!-- pagebreak -->

3460

USB on-the-go high-speed (OTG_HS)

73.15.6

RM0456

Device programming model
Endpoint initialization on USB reset
1.

Set the NAK bit for all OUT endpoints
–

2.

3.

4.

Unmask the following interrupt bits
–

INEP0 = 1 in OTG_DAINTMSK (control 0 IN endpoint)

–

OUTEP0 = 1 in OTG_DAINTMSK (control 0 OUT endpoint)

–

STUPM = 1 in OTG_DOEPMSK

–

XFRCM = 1 in OTG_DOEPMSK

–

XFRCM = 1 in OTG_DIEPMSK

–

TOM = 1 in OTG_DIEPMSK

Set up the data FIFO RAM for each of the FIFOs
–

Program the OTG_GRXFSIZ register, to be able to receive control OUT data and
setup data. If thresholding is not enabled, at a minimum, this must be equal to 1
max packet size of control endpoint 0 + 2 words (for the status of the control OUT
data packet) + 10 words (for setup packets).

–

Program the OTG_DIEPTXF0 register (depending on the FIFO number chosen) to
be able to transmit control IN data. At a minimum, this must be equal to 1 max
packet size of control endpoint 0.

Program the following fields in the endpoint-specific registers for control OUT endpoint
0 to receive a SETUP packet
–

5.

SNAK = 1 in OTG_DOEPCTLx (for all OUT endpoints)

STUPCNT = 3 in OTG_DOEPTSIZ0 (to receive up to 3 back-to-back SETUP
packets)

For USB OTG_HS in DMA mode, the OTG_DOEPDMA0 register must have a valid
memory address to store any SETUP packets received.

At this point, all initialization required to receive SETUP packets is done.

Endpoint initialization on enumeration completion
1.

On the Enumeration Done interrupt (ENUMDNE in OTG_GINTSTS), read the
OTG_DSTS register to determine the enumeration speed.

2.

Program the MPSIZ field in OTG_DIEPCTL0 to set the maximum packet size. This
step configures control endpoint 0. The maximum packet size for a control endpoint
depends on the enumeration speed.

3.

For USB OTG_HS in DMA mode, program the OTG_DOEPCTL0 register to enable
control OUT endpoint 0, to receive a SETUP packet.

At this point, the device is ready to receive SOF packets and is configured to perform control
transfers on control endpoint 0.

Endpoint initialization on SetAddress command
This section describes what the application must do when it receives a SetAddress
command in a SETUP packet.

<!-- pagebreak -->

1.

Program the OTG_DCFG register with the device address received in the SetAddress
command

2.

Program the core to send out a status IN packet

RM0456 Rev 6

RM0456

USB on-the-go high-speed (OTG_HS)

Endpoint initialization on SetConfiguration/SetInterface command
This section describes what the application must do when it receives a SetConfiguration or
SetInterface command in a SETUP packet.
1.

When a SetConfiguration command is received, the application must program the
endpoint registers to configure them with the characteristics of the valid endpoints in
the new configuration.

2.

When a SetInterface command is received, the application must program the endpoint
registers of the endpoints affected by this command.

3.

Some endpoints that were active in the prior configuration or alternate setting are not
valid in the new configuration or alternate setting. These invalid endpoints must be
deactivated.

4.

Unmask the interrupt for each active endpoint and mask the interrupts for all inactive
endpoints in the OTG_DAINTMSK register.

5.

Set up the data FIFO RAM for each FIFO.

6.

After all required endpoints are configured; the application must program the core to
send a status IN packet.

At this point, the device core is configured to receive and transmit any type of data packet.

Endpoint activation
This section describes the steps required to activate a device endpoint or to configure an
existing device endpoint to a new type.
1.

2.

Program the characteristics of the required endpoint into the following fields of the
OTG_DIEPCTLx register (for IN or bidirectional endpoints) or the OTG_DOEPCTLx
register (for OUT or bidirectional endpoints).
–

Maximum packet size

–

USB active endpoint = 1

–

Endpoint start data toggle (for interrupt and bulk endpoints)

–

Endpoint type

–

Tx FIFO number

Once the endpoint is activated, the core starts decoding the tokens addressed to that
endpoint and sends out a valid handshake for each valid token received for the
endpoint.

Endpoint deactivation
This section describes the steps required to deactivate an existing endpoint.

Note:

1.

In the endpoint to be deactivated, clear the USB active endpoint bit in the
OTG_DIEPCTLx register (for IN or bidirectional endpoints) or the OTG_DOEPCTLx
register (for OUT or bidirectional endpoints).

2.

Once the endpoint is deactivated, the core ignores tokens addressed to that endpoint,
which results in a timeout on the USB.

The application must meet the following conditions to set up the device core to handle
traffic:
NPTXFEM and RXFLVLM in the OTG_GINTMSK register must be cleared.

RM0456 Rev 6

<!-- pagebreak -->

3460

USB on-the-go high-speed (OTG_HS)

RM0456

Operational model
SETUP and OUT data transfers:
This section describes the internal data flow and application-level operations during data
OUT transfers and SETUP transactions.
•

Packet read

This section describes how to read packets (OUT data and SETUP packets) from the
receive FIFO.
1.

On catching an RXFLVL interrupt (OTG_GINTSTS register), the application must read
the receive status pop register (OTG_GRXSTSP).

2.

The application can mask the RXFLVL interrupt (in OTG_GINTSTS) by writing to
RXFLVLM = 0 (in OTG_GINTMSK), until it has read the packet from the receive FIFO.

3.

If the received packet’s byte count is not 0, the byte count amount of data is popped
from the receive data FIFO and stored in memory. If the received packet byte count is
0, no data is popped from the receive data FIFO.

4.

The receive status readout of the packet of FIFO indicates one of the following:
a)

Global OUT NAK pattern:
PKTSTS = Global OUT NAK, BCNT = 0x000, EPNUM = (0x0),
DPID = (0b00).
These data indicate that the global OUT NAK bit has taken effect.

b)

SETUP packet pattern:
PKTSTS = SETUP, BCNT = 0x008, EPNUM = Control EP Num,
DPID = DATA0. These data indicate that a SETUP packet for the specified
endpoint is now available for reading from the receive FIFO.

c)

Setup stage done pattern:
PKTSTS = Setup Stage Done, BCNT = 0x0, EPNUM = Control EP Num,
DPID = (0b00).
These data indicate that the setup stage for the specified endpoint has completed
and the data stage has started. After this entry is popped from the receive FIFO,
the core asserts a setup interrupt on the specified control OUT endpoint.

d)

Data OUT packet pattern:
PKTSTS = DataOUT, BCNT = size of the received data OUT packet (0 ≤ BCNT
≤ 1 024), EPNUM = EPNUM on which the packet was received, DPID = Actual
Data PID.

e)

Data transfer completed pattern:
PKTSTS = Data OUT transfer done, BCNT = 0x0, EPNUM = OUT EP Num on
which the data transfer is complete, DPID = (0b00).
These data indicate that an OUT data transfer for the specified OUT endpoint has
completed. After this entry is popped from the receive FIFO, the core asserts a
transfer completed interrupt on the specified OUT endpoint.

5.

After the data payload is popped from the receive FIFO, the RXFLVL interrupt
(OTG_GINTSTS) must be unmasked.

6.

Steps 1–5 are repeated every time the application detects assertion of the interrupt line
due to RXFLVL in OTG_GINTSTS. Reading an empty receive FIFO can result in
undefined core behavior.

Figure 941 provides a flowchart of the above procedure.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

USB on-the-go high-speed (OTG_HS)
Figure 941. Receive FIFO packet read

SETUP transactions
This section describes how the core handles SETUP packets and the application’s
sequence for handling SETUP transactions.
•

Application requirements

1.

To receive a SETUP packet, the STUPCNT field (OTG_DOEPTSIZx) in a control OUT
endpoint must be programmed to a non-zero value. When the application programs the
STUPCNT field to a non-zero value, the core receives SETUP packets and writes them
to the receive FIFO, irrespective of the NAK status and EPENA bit setting in
OTG_DOEPCTLx. The STUPCNT field is decremented every time the control endpoint
receives a SETUP packet. If the STUPCNT field is not programmed to a proper value
before receiving a SETUP packet, the core still receives the SETUP packet and
decrements the STUPCNT field, but the application may not be able to determine the
correct number of SETUP packets received in the setup stage of a control transfer.
–

2.

STUPCNT = 3 in OTG_DOEPTSIZx

The application must always allocate some extra space in the receive data FIFO, to be
able to receive up to three SETUP packets on a control endpoint.
–

The space to be reserved is 10 words. Three words are required for the first
SETUP packet, 1 word is required for the setup stage done word and 6 words are
required to store two extra SETUP packets among all control endpoints.

–

3 words per SETUP packet are required to store 8 bytes of SETUP data and 4
bytes of SETUP status (setup packet pattern). The core reserves this space in the

RM0456 Rev 6

<!-- pagebreak -->

3460

USB on-the-go high-speed (OTG_HS)

RM0456

receive data FIFO to write SETUP data only, and never uses this space for data
packets.
3.

The application must read the 2 words of the SETUP packet from the receive FIFO.

4.

The application must read and discard the setup stage done word from the receive
FIFO.

•

Internal data flow

1.

When a SETUP packet is received, the core writes the received data to the receive
FIFO, without checking for available space in the receive FIFO and irrespective of the
endpoint’s NAK and STALL bit settings.
–

2.

For every SETUP packet received on the USB, 3 words of data are written to the
receive FIFO, and the STUPCNT field is decremented by 1.
–

The first word contains control information used internally by the core

–

The second word contains the first 4 bytes of the SETUP command

–

The third word contains the last 4 bytes of the SETUP command

3.

When the setup stage changes to a data IN/OUT stage, the core writes an entry (setup
stage done word) to the receive FIFO, indicating the completion of the setup stage.

4.

On the AHB side, SETUP packets are emptied by the application.

5.

When the application pops the setup stage done word from the receive FIFO, the core
interrupts the application with an STUP interrupt (OTG_DOEPINTx), indicating it can
process the received SETUP packet.

6.

The core clears the endpoint enable bit for control OUT endpoints.

•

Application programming sequence

1.

Program the OTG_DOEPTSIZx register.
–

STUPCNT = 3

2.

Wait for the RXFLVL interrupt (OTG_GINTSTS) and empty the data packets from the
receive FIFO.

3.

Assertion of the STUP interrupt (OTG_DOEPINTx) marks a successful completion of
the SETUP data transfer.
–

<!-- pagebreak -->

The core internally sets the IN NAK and OUT NAK bits for the control IN/OUT
endpoints on which the SETUP packet was received.

On this interrupt, the application must read the OTG_DOEPTSIZx register to
determine the number of SETUP packets received and process the last received
SETUP packet.

RM0456 Rev 6

RM0456

USB on-the-go high-speed (OTG_HS)
Figure 942. Processing a SETUP packet

Wait for STP in OTG_DOEPINTx

rem_supcnt=
rd_reg(OTG_DOEPTSIZx)

setup_cmd[31:0 = mem[4 – 2 * rem_supcnt]
setup_cmd[63:32] = mem[5 – 2 * rem_supcnt]

Find setup cmd type

ctrl_rd/wr/2 stage

Read

Write

2-stage
setup_np_in_pkt
Data IN phase

setup_np_in_pkt
Status IN phase

rcv_out_pkt
Data OUT phase
MSv37035V1

•

Handling more than three back-to-back SETUP packets

Per the USB 2.0 specification, normally, during a SETUP packet error, a host does not send
more than three back-to-back SETUP packets to the same endpoint. However, the USB 2.0
specification does not limit the number of back-to-back SETUP packets a host can send to
the same endpoint. When this condition occurs, the OTG_HS controller generates an
interrupt (B2BSTUP in OTG_DOEPINTx).
•

Setting the global OUT NAK

Internal data flow:
1.

When the application sets the Global OUT NAK (SGONAK bit in OTG_DCTL), the core
stops writing data, except SETUP packets, to the receive FIFO. Irrespective of the
space availability in the receive FIFO, non-isochronous OUT tokens receive a NAK
handshake response, and the core ignores isochronous OUT data packets

2.

The core writes the Global OUT NAK pattern to the receive FIFO. The application must
reserve enough receive FIFO space to write this data pattern.

3.

When the application pops the Global OUT NAK pattern word from the receive FIFO,
the core sets the GONAKEFF interrupt (OTG_GINTSTS).

4.

Once the application detects this interrupt, it can assume that the core is in Global OUT
NAK mode. The application can clear this interrupt by clearing the SGONAK bit in
OTG_DCTL.

Application programming sequence:

RM0456 Rev 6

<!-- pagebreak -->

3460

USB on-the-go high-speed (OTG_HS)
1.

RM0456

To stop receiving any kind of data in the receive FIFO, the application must set the
Global OUT NAK bit by programming the following field:
–

SGONAK = 1 in OTG_DCTL

2.

Wait for the assertion of the GONAKEFF interrupt in OTG_GINTSTS. When asserted,
this interrupt indicates that the core has stopped receiving any type of data except
SETUP packets.

3.

The application can receive valid OUT packets after it has set SGONAK in OTG_DCTL
and before the core asserts the GONAKEFF interrupt (OTG_GINTSTS).

4.

The application can temporarily mask this interrupt by writing to the GONAKEFFM bit in
the OTG_GINTMSK register.
–

5.

Whenever the application is ready to exit the Global OUT NAK mode, it must clear the
SGONAK bit in OTG_DCTL. This also clears the GONAKEFF interrupt
(OTG_GINTSTS).
–

6.

CGONAK = 1 in OTG_DCTL

If the application has masked this interrupt earlier, it must be unmasked as follows:
–

•

GONAKEFFM = 0 in the OTG_GINTMSK register

GONAKEFFM = 1 in OTG_GINTMSK

Disabling an OUT endpoint

The application must use this sequence to disable an OUT endpoint that it has enabled.
Application programming sequence:
1.

Before disabling any OUT endpoint, the application must enable Global OUT NAK
mode in the core.
–

SGONAK = 1 in OTG_DCTL

2.

Wait for the GONAKEFF interrupt (OTG_GINTSTS)

3.

Disable the required OUT endpoint by programming the following fields:

4.

5.

–

EPDIS = 1 in OTG_DOEPCTLx

–

SNAK = 1 in OTG_DOEPCTLx

Wait for the EPDISD interrupt (OTG_DOEPINTx), which indicates that the OUT
endpoint is completely disabled. When the EPDISD interrupt is asserted, the core also
clears the following bits:
–

EPDIS = 0 in OTG_DOEPCTLx

–

EPENA = 0 in OTG_DOEPCTLx

The application must clear the Global OUT NAK bit to start receiving data from other
non-disabled OUT endpoints.
–

•

SGONAK = 0 in OTG_DCTL

Transfer Stop Programming for OUT endpoints

The application must use the following programing sequence to stop any transfers (because
of an interrupt from the host, typically a reset).
Sequence of operations:

<!-- pagebreak -->

RM0456 Rev 6

RM0456

USB on-the-go high-speed (OTG_HS)
1.

Enable all OUT endpoints by setting
–

2.

EPENA = 1 in all OTG_DOEPCTLx registers.

Flush the RxFIFO as follows
–

Poll OTG_GRSTCTL.AHBIDL until it is 1. This indicates that AHB master is idle.

–

Perform read modify write operation on OTG_GRSTCTL.RXFFLSH =1

–

Poll OTG_GRSTCTL.RXFFLSH until it is 0, but also using a timeout of less than
10 milli-seconds (corresponds to minimum reset signaling duration). If 0 is seen
before the timeout, then the RxFIFO flush is successful. If at the moment the
timeout occurs, there is still a 1, (this may be due to a packet on EP0 coming from
the host) then go back (once only) to the previous step (“Perform read modify write
operation”).

3.

Before disabling any OUT endpoint, the application must enable Global OUT NAK
mode in the core, according to the instructions in “Setting the global OUT NAK”. This
ensures that data in the RxFIFO is sent to the application successfully. Set SGONAK =
1 in OTG_DCTL

4.

Wait for the GONAKEFF interrupt (OTG_GINTSTS)

5.

Disable all active OUT endpoints by programming the following register bits:

6.

•

–

EPDIS = 1 in registers OTG_DOEPCTLx

–

SNAK = 1 in registers OTG_DOEPCTLx

Wait for the EPDIS interrupt in OTG_DOEPINTx for each OUT endpoint programmed
in the previous step. The EPDIS interrupt in OTG_DOEPINTx indicates that the
corresponding OUT endpoint is completely disabled. When the EPDIS interrupt is
asserted, the following bits are cleared:
–

EPENA = 0 in registers OTG_DOEPCTLx

–

EPDIS = 0 in registers OTG_DOEPCTLx

–

SNAK = 0 in registers OTG_DOEPCTLx

Generic non-isochronous OUT data transfers

This section describes a regular non-isochronous OUT data transfer (control, bulk, or
interrupt).
Application requirements:
1.

Before setting up an OUT transfer, the application must allocate a buffer in the memory
to accommodate all data to be received as part of the OUT transfer.

2.

For OUT transfers, the transfer size field in the endpoint’s transfer size register must be
a multiple of the maximum packet size of the endpoint, adjusted to the word boundary.

3.

–

transfer size[EPNUM] = n × (MPSIZ[EPNUM] + 4 – (MPSIZ[EPNUM] mod 4))

–

packet count[EPNUM] = n

–

n>0

On any OUT endpoint interrupt, the application must read the endpoint’s transfer size
register to calculate the size of the payload in the memory. The received payload size
can be less than the programmed transfer size.
–

Payload size in memory = application programmed initial transfer size – core
updated final transfer size

–

Number of USB packets in which this payload was received = application
programmed initial packet count – core updated final packet count

RM0456 Rev 6

<!-- pagebreak -->

3460

USB on-the-go high-speed (OTG_HS)

RM0456

Internal data flow:
1.

The application must set the transfer size and packet count fields in the endpointspecific registers, clear the NAK bit, and enable the endpoint to receive the data.

2.

Once the NAK bit is cleared, the core starts receiving data and writes it to the receive
FIFO, as long as there is space in the receive FIFO. For every data packet received on
the USB, the data packet and its status are written to the receive FIFO. Every packet
(maximum packet size or short packet) written to the receive FIFO decrements the
packet count field for that endpoint by 1.
–

OUT data packets received with bad data CRC are flushed from the receive FIFO
automatically.

–

After sending an ACK for the packet on the USB, the core discards nonisochronous OUT data packets that the host, which cannot detect the ACK, resends. The application does not detect multiple back-to-back data OUT packets
on the same endpoint with the same data PID. In this case the packet count is not
decremented.

–

If there is no space in the receive FIFO, isochronous or non-isochronous data
packets are ignored and not written to the receive FIFO. Additionally, nonisochronous OUT tokens receive a NAK handshake reply.

–

In all the above three cases, the packet count is not decremented because no data
are written to the receive FIFO.

3.

When the packet count becomes 0 or when a short packet is received on the endpoint,
the NAK bit for that endpoint is set. Once the NAK bit is set, the isochronous or nonisochronous data packets are ignored and not written to the receive FIFO, and nonisochronous OUT tokens receive a NAK handshake reply.

4.

After the data are written to the receive FIFO, the application reads the data from the
receive FIFO and writes it to external memory, one packet at a time per endpoint.

5.

At the end of every packet write on the AHB to external memory, the transfer size for
the endpoint is decremented by the size of the written packet.

6.

The OUT data transfer completed pattern for an OUT endpoint is written to the receive
FIFO on one of the following conditions:

7.

–

The transfer size is 0 and the packet count is 0

–

The last OUT data packet written to the receive FIFO is a short packet
(0 ≤ packet size < maximum packet size)

When either the application pops this entry (OUT data transfer completed), a transfer
completed interrupt is generated for the endpoint and the endpoint enable is cleared.

Application programming sequence:

<!-- pagebreak -->

RM0456 Rev 6

RM0456

USB on-the-go high-speed (OTG_HS)
1.

Program the OTG_DOEPTSIZx register for the transfer size and the corresponding
packet count.

2.

Program the OTG_DOEPCTLx register with the endpoint characteristics, and set the
EPENA and CNAK bits.

3.

–

EPENA = 1 in OTG_DOEPCTLx

–

CNAK = 1 in OTG_DOEPCTLx

Wait for the RXFLVL interrupt (in OTG_GINTSTS) and empty the data packets from the
receive FIFO.
–

This step can be repeated many times, depending on the transfer size.

4.

Asserting the XFRC interrupt (OTG_DOEPINTx) marks a successful completion of the
non-isochronous OUT data transfer.

5.

Read the OTG_DOEPTSIZx register to determine the size of the received data
payload.

•

Generic isochronous OUT data transfer

This section describes a regular isochronous OUT data transfer.
Application requirements:
1.

All the application requirements for non-isochronous OUT data transfers also apply to
isochronous OUT data transfers.

2.

For isochronous OUT data transfers, the transfer size and packet count fields must
always be set to the number of maximum-packet-size packets that can be received in a
single frame and no more. Isochronous OUT data transfers cannot span more than 1
frame.

3.

The application must read all isochronous OUT data packets from the receive FIFO
(data and status) before the end of the periodic frame (EOPF interrupt in
OTG_GINTSTS).

4.

To receive data in the following frame, an isochronous OUT endpoint must be enabled
after the EOPF (OTG_GINTSTS) and before the SOF (OTG_GINTSTS).

Internal data flow:
1.

The internal data flow for isochronous OUT endpoints is the same as that for nonisochronous OUT endpoints, but for a few differences.

2.

When an isochronous OUT endpoint is enabled by setting the endpoint enable and
clearing the NAK bits, the Even/Odd frame bit must also be set appropriately. The core
receives data on an isochronous OUT endpoint in a particular frame only if the
following condition is met:
–

3.

EONUM (in OTG_DOEPCTLx) = FNSOF[0] (in OTG_DSTS)

When the application completely reads an isochronous OUT data packet (data and
status) from the receive FIFO, the core updates the RXDPID field in OTG_DOEPTSIZx
with the data PID of the last isochronous OUT data packet read from the receive FIFO.

Application programming sequence:

RM0456 Rev 6

<!-- pagebreak -->

3460

USB on-the-go high-speed (OTG_HS)

RM0456

1.

Program the OTG_DOEPTSIZx register for the transfer size and the corresponding
packet count

2.

Program the OTG_DOEPCTLx register with the endpoint characteristics and set the
endpoint enable, ClearNAK, and Even/Odd frame bits.

3.

–

EPENA = 1

–

CNAK = 1

–

EONUM = (0: Even/1: Odd)

Wait for the RXFLVL interrupt (in OTG_GINTSTS) and empty the data packets from the
receive FIFO
–

This step can be repeated many times, depending on the transfer size.

4.

The assertion of the XFRC interrupt (in OTG_DOEPINTx) marks the completion of the
isochronous OUT data transfer. This interrupt does not necessarily mean that the data
in memory are good.

5.

This interrupt cannot always be detected for isochronous OUT transfers. Instead, the
application can detect the INCOMPISOOUT interrupt in OTG_GINTSTS.

6.

Read the OTG_DOEPTSIZx register to determine the size of the received transfer and
to determine the validity of the data received in the frame. The application must treat
the data received in memory as valid only if one of the following conditions is met:
–

RXDPID = DATA0 (in OTG_DOEPTSIZx) and the number of USB packets in
which this payload was received = 1

–

RXDPID = DATA1 (in OTG_DOEPTSIZx) and the number of USB packets in
which this payload was received = 2
RXDPID = D2 (in OTG_DOEPTSIZx) and the number of USB packets in which
this payload was received = 3The number of USB packets in which this payload
was received =
Application programmed initial packet count – core updated final packet count

The application can discard invalid data packets.
•

Incomplete isochronous OUT data transfers

This section describes the application programming sequence when isochronous OUT data
packets are dropped inside the core.
Internal data flow:
1.

2.

<!-- pagebreak -->

For isochronous OUT endpoints, the XFRC interrupt (in OTG_DOEPINTx) may not
always be asserted. If the core drops isochronous OUT data packets, the application
may fail to detect the XFRC interrupt (OTG_DOEPINTx) under the following
circumstances:
–

When the receive FIFO cannot accommodate the complete ISO OUT data packet,
the core drops the received ISO OUT data

–

When the isochronous OUT data packet is received with CRC errors

–

When the isochronous OUT token received by the core is corrupted

–

When the application is very slow in reading the data from the receive FIFO

When the core detects an end of periodic frame before transfer completion to all
isochronous OUT endpoints, it asserts the incomplete isochronous OUT data interrupt
(INCOMPISOOUT in OTG_GINTSTS), indicating that an XFRC interrupt (in
OTG_DOEPINTx) is not asserted on at least one of the isochronous OUT endpoints. At

RM0456 Rev 6

RM0456

USB on-the-go high-speed (OTG_HS)
this point, the endpoint with the incomplete transfer remains enabled, but no active
transfers remain in progress on this endpoint on the USB.
Application programming sequence:
1.

Asserting the INCOMPISOOUT interrupt (OTG_GINTSTS) indicates that in the current
frame, at least one isochronous OUT endpoint has an incomplete transfer.

2.

If this occurs because isochronous OUT data is not completely emptied from the
endpoint, the application must ensure that the application empties all isochronous OUT
data (data and status) from the receive FIFO before proceeding.
–

3.

When all data are emptied from the receive FIFO, the application can detect the
XFRC interrupt (OTG_DOEPINTx). In this case, the application must re-enable
the endpoint to receive isochronous OUT data in the next frame.

When it receives an INCOMPISOOUT interrupt (in OTG_GINTSTS), the application
must read the control registers of all isochronous OUT endpoints (OTG_DOEPCTLx) to
determine which endpoints had an incomplete transfer in the current microframe. An
endpoint transfer is incomplete if both the following conditions are met:
–

EONUM bit (in OTG_DOEPCTLx) = FNSOF[0] (in OTG_DSTS)

–

EPENA = 1 (in OTG_DOEPCTLx)

4.

The previous step must be performed before the SOF interrupt (in OTG_GINTSTS) is
detected, to ensure that the current frame number is not changed.

5.

For isochronous OUT endpoints with incomplete transfers, the application must discard
the data in the memory and disable the endpoint by setting the EPDIS bit in
OTG_DOEPCTLx.

6.

Wait for the EPDISD interrupt (in OTG_DOEPINTx) and enable the endpoint to receive
new data in the next frame.
–

•

Because the core can take some time to disable the endpoint, the application may
not be able to receive the data in the next frame after receiving bad isochronous
data.

Stalling a non-isochronous OUT endpoint

This section describes how the application can stall a non-isochronous endpoint.
1.
2.

Put the core in the Global OUT NAK mode.
Disable the required endpoint
–

When disabling the endpoint, instead of setting the SNAK bit in OTG_DOEPCTL,
set STALL = 1 (in OTG_DOEPCTL).
The STALL bit always takes precedence over the NAK bit.

3.

When the application is ready to end the STALL handshake for the endpoint, the
STALL bit (in OTG_DOEPCTLx) must be cleared.

4.

If the application is setting or clearing a STALL for an endpoint due to a
SetFeature.Endpoint Halt or ClearFeature.Endpoint Halt command, the STALL bit must
be set or cleared before the application sets up the status stage transfer on the control
endpoint.

Examples
This section describes and depicts some fundamental transfer types and scenarios.
•

Bulk OUT transaction

RM0456 Rev 6

<!-- pagebreak -->

3460

USB on-the-go high-speed (OTG_HS)

RM0456

Figure 943 depicts the reception of a single Bulk OUT data packet from the USB to the AHB
and describes the events involved in the process.
Figure 943. Bulk OUT transaction
Host

USB

Device

Application
init _out_ep
1

2

XFRSIZ = 64 bytes
PKTCNT = 1

Wr_reg(OTG_DOEPTSIZx)

EPENA = 1
CN AK = 1

O UT

Wr_reg(OTG_DOEPCTLx)

3
64 bytes
4

6

xact_1
AC K

RXFLVL iintr

OTG_DO

5

EPCTLx

PKTCN

, NAK=1

XFRSIZ
=0
r
OU T
NA K

idle until intr

T0

7

rcv_out _pkt()

XF
int r RC

8

On new xfer
or RxFIFO
not em pty

idle until intr

MS36931V1

After a SetConfiguration/SetInterface command, the application initializes all OUT endpoints
by setting CNAK = 1 and EPENA = 1 (in OTG_DOEPCTLx), and setting a suitable
XFRSIZ and PKTCNT in the OTG_DOEPTSIZx register.
1.

host attempts to send data (OUT token) to an endpoint.

2.

When the core receives the OUT token on the USB, it stores the packet in the Rx FIFO
because space is available there.

3.

After writing the complete packet in the Rx FIFO, the core then asserts the RXFLVL
interrupt (in OTG_GINTSTS).

4.

On receiving the PKTCNT number of USB packets, the core internally sets the NAK bit
for this endpoint to prevent it from receiving any more packets.

5.

The application processes the interrupt and reads the data from the Rx FIFO.

6.

When the application has read all the data (equivalent to XFRSIZ), the core generates
an XFRC interrupt (in OTG_DOEPINTx).

7.

The application processes the interrupt and uses the setting of the XFRC interrupt bit
(in OTG_DOEPINTx) to determine that the intended transfer is complete.

IN data transfers
•

Packet write

This section describes how the application writes data packets to the endpoint FIFO when
dedicated transmit FIFOs are enabled.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

USB on-the-go high-speed (OTG_HS)
1.

2.

The application can either choose the polling or the interrupt mode.
–

In polling mode, the application monitors the status of the endpoint transmit data
FIFO by reading the OTG_DTXFSTSx register, to determine if there is enough
space in the data FIFO.

–

In interrupt mode, the application waits for the TXFE interrupt (in OTG_DIEPINTx)
and then reads the OTG_DTXFSTSx register, to determine if there is enough
space in the data FIFO.

–

To write a single non-zero length data packet, there must be space to write the
entire packet in the data FIFO.

–

To write zero length packet, the application must not look at the FIFO space.

Using one of the above mentioned methods, when the application determines that
there is enough space to write a transmit packet, the application must first write into the
endpoint control register, before writing the data into the data FIFO. Typically, the
application, must do a read modify write on the OTG_DIEPCTLx register to avoid
modifying the contents of the register, except for setting the endpoint enable bit.

The application can write multiple packets for the same endpoint into the transmit FIFO, if
space is available. For periodic IN endpoints, the application must write packets only for one
microframe. It can write packets for the next periodic transaction only after getting transfer
complete for the previous transaction.
•

Setting IN endpoint NAK

Internal data flow:
1.

When the application sets the IN NAK for a particular endpoint, the core stops
transmitting data on the endpoint, irrespective of data availability in the endpoint’s
transmit FIFO.

2.

Non-isochronous IN tokens receive a NAK handshake reply
–

Isochronous IN tokens receive a zero-data-length packet reply

3.

The core asserts the INEPNE (IN endpoint NAK effective) interrupt in OTG_DIEPINTx
in response to the SNAK bit in OTG_DIEPCTLx.

4.

Once this interrupt is seen by the application, the application can assume that the
endpoint is in IN NAK mode. This interrupt can be cleared by the application by setting
the CNAK bit in OTG_DIEPCTLx.

Application programming sequence:
1.

To stop transmitting any data on a particular IN endpoint, the application must set the
IN NAK bit. To set this bit, the following field must be programmed.
–

SNAK = 1 in OTG_DIEPCTLx

2.

Wait for assertion of the INEPNE interrupt in OTG_DIEPINTx. This interrupt indicates
that the core has stopped transmitting data on the endpoint.

3.

The core can transmit valid IN data on the endpoint after the application has set the
NAK bit, but before the assertion of the NAK Effective interrupt.

4.

The application can mask this interrupt temporarily by writing to the INEPNEM bit in
OTG_DIEPMSK.
–

5.

INEPNEM = 0 in OTG_DIEPMSK

To exit endpoint NAK mode, the application must clear the NAK status bit (NAKSTS) in
OTG_DIEPCTLx. This also clears the INEPNE interrupt (in OTG_DIEPINTx).

RM0456 Rev 6

<!-- pagebreak -->

3460

USB on-the-go high-speed (OTG_HS)
–
6.

CNAK = 1 in OTG_DIEPCTLx

If the application masked this interrupt earlier, it must be unmasked as follows:
–

•

RM0456

INEPNEM = 1 in OTG_DIEPMSK

IN endpoint disable

Use the following sequence to disable a specific IN endpoint that has been previously
enabled.
Application programming sequence:
1.
2.

The application must stop writing data on the AHB for the IN endpoint to be disabled.
The application must set the endpoint in NAK mode.
–

SNAK = 1 in OTG_DIEPCTLx

3.

Wait for the INEPNE interrupt in OTG_DIEPINTx.

4.

Set the following bits in the OTG_DIEPCTLx register for the endpoint that must be
disabled.

5.

–

EPDIS = 1 in OTG_DIEPCTLx

–

SNAK = 1 in OTG_DIEPCTLx

Assertion of the EPDISD interrupt in OTG_DIEPINTx indicates that the core has
completely disabled the specified endpoint. Along with the assertion of the interrupt, the
core also clears the following bits:
–

EPENA = 0 in OTG_DIEPCTLx

–

EPDIS = 0 in OTG_DIEPCTLx

6.

The application must read the OTG_DIEPTSIZx register for the periodic IN EP, to
calculate how much data on the endpoint were transmitted on the USB.

7.

The application must flush the data in the endpoint transmit FIFO, by setting the
following fields in the OTG_GRSTCTL register:
–

TXFNUM (in OTG_GRSTCTL) = Endpoint transmit FIFO number

–

TXFFLSH in (OTG_GRSTCTL) = 1

The application must poll the OTG_GRSTCTL register, until the TXFFLSH bit is cleared by
the core, which indicates the end of flush operation. To transmit new data on this endpoint,
the application can re-enable the endpoint at a later point.
•

Transfer Stop Programming for IN endpoints

The application must use the following programing sequence to stop any transfers (because
of an interrupt from the host, typically a reset).

<!-- pagebreak -->

RM0456 Rev 6

RM0456

USB on-the-go high-speed (OTG_HS)
Sequence of operations:
1.

Disable the IN endpoint by setting:
–

2.

3.

EPDIS = 1 in all OTG_DIEPCTLx registers

Wait for the EPDIS interrupt in OTG_DIEPINTx, which indicates that the IN endpoint is
completely disabled. When the EPDIS interrupt is asserted the following bits are
cleared:
–

EPDIS = 0 in OTG_DIEPCTLx

–

EPENA = 0 in OTG_DIEPCTLx

Flush the TxFIFO by programming the following bits:
–

TXFFLSH = 1 in OTG_GRSTCTL

–

TXFNUM = “FIFO number specific to endpoint” in OTG_GRSTCTL

4.

The application can start polling till TXFFLSH in OTG_GRSTCTL is cleared. When this
bit is cleared, it ensures that there is no data left in the Tx FIFO.

•

Generic non-periodic IN data transfers

Application requirements:
1.

Before setting up an IN transfer, the application must ensure that all data to be
transmitted as part of the IN transfer are part of a single buffer.

2.

For IN transfers, the transfer size field in the endpoint transfer size register denotes a
payload that constitutes multiple maximum-packet-size packets and a single short
packet. This short packet is transmitted at the end of the transfer.
–

To transmit a few maximum-packet-size packets and a short packet at the end of
the transfer:
Transfer size[EPNUM] = x × MPSIZ[EPNUM] + sp
If (sp > 0), then packet count[EPNUM] = x + 1.
Otherwise, packet count[EPNUM] = x

–

To transmit a single zero-length data packet:
Transfer size[EPNUM] = 0
Packet count[EPNUM] = 1

–

To transmit a few maximum-packet-size packets and a zero-length data packet at
the end of the transfer, the application must split the transfer into two parts. The
first sends maximum-packet-size data packets and the second sends the zerolength data packet alone.
First transfer: transfer size[EPNUM] = x × MPSIZ[epnum]; packet count = n;
Second transfer: transfer size[EPNUM] = 0; packet count = 1;

3.

Once an endpoint is enabled for data transfers, the core updates the transfer size
register. At the end of the IN transfer, the application must read the transfer size
register to determine how much data posted in the transmit FIFO have already been
sent on the USB.

4.

Data fetched into transmit FIFO = Application-programmed initial transfer size – coreupdated final transfer size
–

Data transmitted on USB = (application-programmed initial packet count – core
updated final packet count) × MPSIZ[EPNUM]

–

Data yet to be transmitted on USB = (Application-programmed initial transfer size
– data transmitted on USB)

RM0456 Rev 6

<!-- pagebreak -->

3460

USB on-the-go high-speed (OTG_HS)

RM0456

Internal data flow:
1.

The application must set the transfer size and packet count fields in the endpointspecific registers and enable the endpoint to transmit the data.

2.

The application must also write the required data to the transmit FIFO for the endpoint.

3.

Every time a packet is written into the transmit FIFO by the application, the transfer size
for that endpoint is decremented by the packet size. The data is fetched from the
memory by the application, until the transfer size for the endpoint becomes 0. After
writing the data into the FIFO, the “number of packets in FIFO” count is incremented
(this is a 3-bit count, internally maintained by the core for each IN endpoint transmit
FIFO. The maximum number of packets maintained by the core at any time in an IN
endpoint FIFO is eight). For zero-length packets, a separate flag is set for each FIFO,
without any data in the FIFO.

4.

Once the data are written to the transmit FIFO, the core reads them out upon receiving
an IN token. For every non-isochronous IN data packet transmitted with an ACK
handshake, the packet count for the endpoint is decremented by one, until the packet
count is zero. The packet count is not decremented on a timeout.

5.

For zero length packets (indicated by an internal zero length flag), the core sends out a
zero-length packet for the IN token and decrements the packet count field.

6.

If there are no data in the FIFO for a received IN token and the packet count field for
that endpoint is zero, the core generates an “IN token received when Tx FIFO is empty”
(ITTXFE) interrupt for the endpoint, provided that the endpoint NAK bit is not set. The
core responds with a NAK handshake for non-isochronous endpoints on the USB.

7.

The core internally rewinds the FIFO pointers and no timeout interrupt is generated.

8.

When the transfer size is 0 and the packet count is 0, the transfer complete (XFRC)
interrupt for the endpoint is generated and the endpoint enable is cleared.

Application programming sequence:
1.

Program the OTG_DIEPTSIZx register with the transfer size and corresponding packet
count.

2.

Program the OTG_DIEPCTLx register with the endpoint characteristics and set the
CNAK and EPENA (endpoint enable) bits.

3.

When transmitting non-zero length data packet, the application must poll the
OTG_DTXFSTSx register (where x is the FIFO number associated with that endpoint)
to determine whether there is enough space in the data FIFO. The application can
optionally use TXFE (in OTG_DIEPINTx) before writing the data.

•

Generic periodic IN data transfers

This section describes a typical periodic IN data transfer.
Application requirements:
1.

Application requirements 1, 2, 3, and 4 of Generic non-periodic IN data transfers also
apply to periodic IN data transfers, except for a slight modification of requirement 2.
–

<!-- pagebreak -->

The application can only transmit multiples of maximum-packet-size data packets
or multiples of maximum-packet-size packets, plus a short packet at the end. To

RM0456 Rev 6

RM0456

USB on-the-go high-speed (OTG_HS)
transmit a few maximum-packet-size packets and a short packet at the end of the
transfer, the following conditions must be met:
transfer size[EPNUM] = x × MPSIZ[EPNUM] + sp
(where x is an integer ≥ 0, and 0 ≤ sp < MPSIZ[EPNUM])
If (sp > 0), packet count[EPNUM] = x + 1
Otherwise, packet count[EPNUM] = x;
MCNT[EPNUM] = packet count[EPNUM]
–

The application cannot transmit a zero-length data packet at the end of a transfer.
It can transmit a single zero-length data packet by itself. To transmit a single zerolength data packet:

–

transfer size[EPNUM] = 0
packet count[EPNUM] = 1
MCNT[EPNUM] = packet count[EPNUM]

2.

3.

The application can only schedule data transfers one frame at a time.
–

(MCNT – 1) × MPSIZ ≤ XFERSIZ ≤ MCNT × MPSIZ

–

PKTCNT = MCNT (in OTG_DIEPTSIZx)

–

If XFERSIZ < MCNT × MPSIZ, the last data packet of the transfer is a short
packet.

–

Note that: MCNT is in OTG_DIEPTSIZx, MPSIZ is in OTG_DIEPCTLx, PKTCNT
is in OTG_DIEPTSIZx and XFERSIZ is in OTG_DIEPTSIZx

The complete data to be transmitted in the frame must be written into the transmit FIFO
by the application, before the IN token is received. Even when 1 word of the data to be
transmitted per frame is missing in the transmit FIFO when the IN token is received, the
core behaves as when the FIFO is empty. When the transmit FIFO is empty:
–

A zero data length packet would be transmitted on the USB for isochronous IN
endpoints

–

A NAK handshake would be transmitted on the USB for interrupt IN endpoints

Internal data flow:
1.

The application must set the transfer size and packet count fields in the endpointspecific registers and enable the endpoint to transmit the data.

2.

The application must also write the required data to the associated transmit FIFO for
the endpoint.

3.

Every time the application writes a packet to the transmit FIFO, the transfer size for that
endpoint is decremented by the packet size. The data are fetched from application
memory until the transfer size for the endpoint becomes 0.

4.

When an IN token is received for a periodic endpoint, the core transmits the data in the
FIFO, if available. If the complete data payload (complete packet, in dedicated FIFO

RM0456 Rev 6

<!-- pagebreak -->

3460

USB on-the-go high-speed (OTG_HS)

RM0456

mode) for the frame is not present in the FIFO, then the core generates an IN token
received when Tx FIFO empty interrupt for the endpoint.

5.

6.

–

A zero-length data packet is transmitted on the USB for isochronous IN endpoints

–

A NAK handshake is transmitted on the USB for interrupt IN endpoints

The packet count for the endpoint is decremented by 1 under the following conditions:
–

For isochronous endpoints, when a zero- or non-zero-length data packet is
transmitted

–

For interrupt endpoints, when an ACK handshake is transmitted

–

When the transfer size and packet count are both 0, the transfer completed
interrupt for the endpoint is generated and the endpoint enable is cleared.

At the “Periodic frame Interval” (controlled by PFIVL in OTG_DCFG), when the core
finds non-empty any of the isochronous IN endpoint FIFOs scheduled for the current
frame non-empty, the core generates an IISOIXFR interrupt in OTG_GINTSTS.

Application programming sequence:
1.

Program the OTG_DIEPCTLx register with the endpoint characteristics and set the
CNAK and EPENA bits.

2.

Write the data to be transmitted in the next frame to the transmit FIFO.

3.

Asserting the ITTXFE interrupt (in OTG_DIEPINTx) indicates that the application has
not yet written all data to be transmitted to the transmit FIFO.

4.

If the interrupt endpoint is already enabled when this interrupt is detected, ignore the
interrupt. If it is not enabled, enable the endpoint so that the data can be transmitted on
the next IN token attempt.

5.

Asserting the XFRC interrupt (in OTG_DIEPINTx) with no ITTXFE interrupt in
OTG_DIEPINTx indicates the successful completion of an isochronous IN transfer. A
read to the OTG_DIEPTSIZx register must give transfer size = 0 and packet count = 0,
indicating all data were transmitted on the USB.

6.

Asserting the XFRC interrupt (in OTG_DIEPINTx), with or without the ITTXFE interrupt
(in OTG_DIEPINTx), indicates the successful completion of an interrupt IN transfer. A
read to the OTG_DIEPTSIZx register must give transfer size = 0 and packet count = 0,
indicating all data were transmitted on the USB.

7.

Asserting the incomplete isochronous IN transfer (IISOIXFR) interrupt in
OTG_GINTSTS with none of the aforementioned interrupts indicates the core did not
receive at least 1 periodic IN token in the current frame.

•

Incomplete isochronous IN data transfers

This section describes what the application must do on an incomplete isochronous IN data
transfer.
Internal data flow:
1.

<!-- pagebreak -->

An isochronous IN transfer is treated as incomplete in one of the following conditions:
a)

The core receives a corrupted isochronous IN token on at least one isochronous
IN endpoint. In this case, the application detects an incomplete isochronous IN
transfer interrupt (IISOIXFR in OTG_GINTSTS).

b)

The application is slow to write the complete data payload to the transmit FIFO
and an IN token is received before the complete data payload is written to the
FIFO. In this case, the application detects an IN token received when Tx FIFO
empty interrupt in OTG_DIEPINTx. The application can ignore this interrupt, as it
RM0456 Rev 6

RM0456

USB on-the-go high-speed (OTG_HS)
eventually results in an incomplete isochronous IN transfer interrupt (IISOIXFR in
OTG_GINTSTS) at the end of periodic frame.
The core transmits a zero-length data packet on the USB in response to the
received IN token.
2.

The application must stop writing the data payload to the transmit FIFO as soon as
possible.

3.

The application must set the NAK bit and the disable bit for the endpoint.

4.

The core disables the endpoint, clears the disable bit, and asserts the endpoint disable
interrupt for the endpoint.

Application programming sequence:
1.

The application can ignore the IN token received when Tx FIFO empty interrupt in
OTG_DIEPINTx on any isochronous IN endpoint, as it eventually results in an
incomplete isochronous IN transfer interrupt (in OTG_GINTSTS).

2.

Assertion of the incomplete isochronous IN transfer interrupt (in OTG_GINTSTS)
indicates an incomplete isochronous IN transfer on at least one of the isochronous IN
endpoints.

3.

The application must read the endpoint control register for all isochronous IN endpoints
to detect endpoints with incomplete IN data transfers.

4.

The application must stop writing data to the Periodic Transmit FIFOs associated with
these endpoints on the AHB.

5.

6.

Program the following fields in the OTG_DIEPCTLx register to disable the endpoint:
–

SNAK = 1 in OTG_DIEPCTLx

–

EPDIS = 1 in OTG_DIEPCTLx

The assertion of the endpoint disabled interrupt in OTG_DIEPINTx indicates that the
core has disabled the endpoint.
–

•

At this point, the application must flush the data in the associated transmit FIFO or
overwrite the existing data in the FIFO by enabling the endpoint for a new transfer
in the next microframe. To flush the data, the application must use the
OTG_GRSTCTL register.

Stalling non-isochronous IN endpoints

This section describes how the application can stall a non-isochronous endpoint.
Application programming sequence:

RM0456 Rev 6

<!-- pagebreak -->

3460

USB on-the-go high-speed (OTG_HS)

RM0456

1.

Disable the IN endpoint to be stalled. Set the STALL bit as well.

2.

EPDIS = 1 in OTG_DIEPCTLx, when the endpoint is already enabled
–

STALL = 1 in OTG_DIEPCTLx

–

The STALL bit always takes precedence over the NAK bit

3.

Assertion of the endpoint disabled interrupt (in OTG_DIEPINTx) indicates to the
application that the core has disabled the specified endpoint.

4.

The application must flush the non-periodic or periodic transmit FIFO, depending on
the endpoint type. In case of a non-periodic endpoint, the application must re-enable
the other non-periodic endpoints that do not need to be stalled, to transmit data.

5.

Whenever the application is ready to end the STALL handshake for the endpoint, the
STALL bit must be cleared in OTG_DIEPCTLx.

6.

If the application sets or clears a STALL bit for an endpoint due to a
SetFeature.Endpoint Halt command or ClearFeature.Endpoint Halt command, the
STALL bit must be set or cleared before the application sets up the status stage
transfer on the control endpoint.

Special case: stalling the control OUT endpoint
The core must stall IN/OUT tokens if, during the data stage of a control transfer, the host
sends more IN/OUT tokens than are specified in the SETUP packet. In this case, the
application must enable the ITTXFE interrupt in OTG_DIEPINTx and the OTEPDIS interrupt
in OTG_DOEPINTx during the data stage of the control transfer, after the core has
transferred the amount of data specified in the SETUP packet. Then, when the application
receives this interrupt, it must set the STALL bit in the corresponding endpoint control
register, and clear this interrupt.

73.15.7

Worst case response time
When the OTG_HS controller acts as a device, there is a worst case response time for any
tokens that follow an isochronous OUT. This worst case response time depends on the AHB
clock frequency.
The core registers are in the AHB domain, and the core does not accept another token
before updating these register values. The worst case is for any token following an
isochronous OUT, because for an isochronous transaction, there is no handshake and the
next token may come sooner. This worst case value is 7 PHY clocks when the AHB clock is
the same as the PHY clock. When the AHB clock is faster, this value is smaller.
If this worst case condition occurs, the core responds to bulk/interrupt tokens with a NAK
and drops isochronous and SETUP tokens. The host interprets this as a timeout condition
for SETUP and retries the SETUP packet. For isochronous transfers, the Incomplete
isochronous IN transfer interrupt (IISOIXFR) and Incomplete isochronous OUT transfer
interrupt (IISOOXFR) inform the application that isochronous IN/OUT packets were
dropped.

Choosing the value of TRDT in OTG_GUSBCFG
The value in TRDT (OTG_GUSBCFG) is the time it takes for the MAC, in terms of PHY
clocks after it has received an IN token, to get the FIFO status, and thus the first data from
the PFC block. This time involves the synchronization delay between the PHY and AHB
clocks. The worst case delay for this is when the AHB clock is the same as the PHY clock.
In this case, the delay is 5 clocks.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

USB on-the-go high-speed (OTG_HS)
Once the MAC receives an IN token, this information (token received) is synchronized to the
AHB clock by the PFC (the PFC runs on the AHB clock). The PFC then reads the data from
the SPRAM and writes them into the dual clock source buffer. The MAC then reads the data
out of the source buffer (4 deep).
If the AHB is running at a higher frequency than the PHY, the application can use a smaller
value for TRDT (in OTG_GUSBCFG).
Figure 944 has the following signals:
•

tkn_rcvd: Token received information from MAC to PFC

•

dynced_tkn_rcvd: Doubled sync tkn_rcvd, from PCLK to HCLK domain

•

spr_read: Read to SPRAM

•

spr_addr: Address to SPRAM

•

spr_rdata: Read data from SPRAM

•

srcbuf_push: Push to the source buffer

•

srcbuf_rdata: Read data from the source buffer. Data seen by MAC

To calculate the value of TRDT, refer to Table 771: TRDT values.
Figure 944. TRDT max timing case

RM0456 Rev 6

<!-- pagebreak -->

3460

USB on-the-go high-speed (OTG_HS)

73.15.8

RM0456

OTG programming model
The OTG_HS controller is an OTG device. When the core is connected to an “A” plug, it is
referred to as an A-device. When the core is connected to a “B” plug it is referred to as a Bdevice.

<!-- pagebreak -->

RM0456 Rev 6

USB Type-C®/USB Power Delivery interface (UCPD)

RM0456

74

USB Type-C®/USB Power Delivery interface (UCPD)
This section only applies to STM32U575/585/59x/5Ax/U5Fx/U5Gx devices.

74.1

Introduction
The USB Type-C/USB Power Delivery interface complies with:
•

Universal Serial Bus Type-C Cable and Connector Specification: release 2.3, Oct 2023

•

Universal Serial Bus Power Delivery specifications:
–

revision 2.0, version 1.3, January 12, 2017

–

revision 3.2, version 1.0, October 2023

It integrates the physical layer of the Power Delivery (PD) specification, with CC signaling
method (no VBUS), for operation with Type-C cables.

74.2

UCPD main features
•

Compliance with USB Type-C specification release 2.3

•

Compliance with USB Power Delivery specifications revision 2.0 and 3.2
–

Enabling advanced applications such as PPS (programmable power supply)

•

Stop mode low-power operation support

•

Built-in analog PHY

•

–

USB Type-C pull-up (Rp, all values) and pull-down (Rd) resistors

–

“Dead battery” Rd support

–

USB Power Delivery message transmission and reception

–

FRS (fast role swap) Rx support

Digital controller
–

BMC (bi-phase mark coding) encode and decode

–

4b5b encode and decode

–

USB Type-C level detection with de-bounce, generating interrupts

–

FRS signaling

–

FRS detection, generating an interrupt

–

DMA-compatible byte-level interface for USB Power Delivery payload, generating
interrupts

–

USB Power Delivery clock pre-scaler / dividers

–

CRC generation/checking

–

Support of ordered sets, with a programmable ordered set mask at receive

–

Clock recovery from incoming Rx stream

RM0456 Rev 6

<!-- pagebreak -->

3499

USB Type-C®/USB Power Delivery interface (UCPD)

74.3

RM0456

UCPD implementation
The devices have one UCPD controller to support one USB Type-C port.
Table 774. UCPD implementation(1)
UCPD feature

STM32U575/585
Rev. X

Other STM32U5xx
device variants
except
STM32U535/545

Dead battery support via UCPDx_DBCC1 and
UCPDx_DBCC2 external signals

X

X

UCPDx_FRSTX1/2 as alternate function pins

X

X

Fully automatic trimming

X(2)

-(3)

USB PD receiver hardware filter control

X

X

Discrete-component PHY support

-

-

1. “X” = supported, “-” = not supported
2. No software trimming required.
3. Apply software trimming as described in Section 74.5.5: UCPD software trimming.

The following table gives the memory locations of trim data stored in the non-volatile
memory, to use in the software trimming procedure described in Section 74.5.5: UCPD
software trimming.
Table 775. UCPD software trim data
Non-volatile memory location
Name

74.4

Address

Bits

3A0_CC1[3:0]

0x0BFA 0545

3:0

3A0_CC2[3:0]

0x0BFA 0547

3:0

1A5_CC1[3:0]

0x0BFA 07A7

3:0

1A5_CC2[3:0]

0x0BFA 07A8

3:0

Rd_CC1[3:0]

0x0BFA 0544

3:0

Rd_CC2[3:0]

0x0BFA 0546

3:0

UCPD functional description
The UCPD peripheral provides hardware support for the USB Power Delivery control
interface specification, using I/Os specifically designed for that purpose.
The built-in PHY directly detects Type-C voltage levels, supports Power Delivery BIST
carrier mode 2 (Tx only), BIST test data (Tx and Rx), and Power Delivery Rx FRS signaling.
For Power Delivery FRS Tx signaling, the device can be configured to control, through
UCPD_FRSTX1/2 pins (alternate functions), external NMOS transistors that ensure lowresistance pull-down on CC lines.

<!-- pagebreak -->

RM0456 Rev 6

USB Type-C®/USB Power Delivery interface (UCPD)

RM0456

The UCPD transmitter BMC (bi-phase mark) encodes and transmits data: preamble, SOP,
payload data from protocol layer (after 4b5b-encoding), CRC, and EOP on the Type-C
connector CC lines. It automatically inserts inter-frame gap and executes “Hard Reset”.
The UCPD receiver detects SOP, BMC-decodes the incoming stream, recovers the
preamble, 4b5b-decodes payload data, detects EOP, and checks CRC. It automatically
detects five K-code SOP and two Reset ordered sets, plus two software-defined patterns
(allows for only three out of four K-codes being correctly received, as defined by the
standard).
In Stop mode, the peripheral maintains the ability to detect incoming USB Power Delivery
messages and FRS signaling, which allows low-power operation.

74.4.1

UCPD block diagram
Figure 945. UCPD block diagram
Registers

To/from RCC:
ucpd_ker_ck
clk_rq
ucpd_pclk

UCPD

Power Delivery Tx
(CC1 or CC2)

Control
register

Ch.1
FRS
(pulse)

UCPDx_FRSTX1

Ch.2

UCPDx_FRSTX2

Tx ordered
set register
Tx byte
register

4b5b
encode

CRC add

Ordered
set insert

Analog
PHY

BMC
encode

UCPDx_DBCC1
32-bit APB bus

Ch. 1
4b5b
decode

Rx byte
register
Rx ordered
set register
CRC
check

To NVIC:
ucpd_it
To EXTI:
ucpd_wkup

Ordered
set detect /
filter

Power Delivery Rx
(CC1 or CC2)

BMC
frequency
estimate

UCPDx_DBCC2
Ch. 2
UCPDx_CC2

FRS
(detect)

Status
register
(interrupts/
events)

To/from DMA:

UCPDx_CC1
BMC
decode

Type-C control
CC1 + CC2

Alternate or
additional
I/O function

Type-C controller (CC1 and CC2)

ucpd_tx_dma

Type-C debounce
CC1 + CC2

ucpd_rx_dma

MSv66135V1

The following table lists external signals (alternate or additional I/O functions).
Table 776. UCPD signals on pins
Pin name

Signal type

Description

UCPDx_FRSTX1

Output

USB Type-C fast role swap (FRS) signaling, applicable to DRPs
only. The signal (active high) drives an external NMOS
transistor that pulls down the CC1 line.

UCPDx_FRSTX2

Output

USB Type-C fast role swap (FRS) signaling, applicable to DRPs
only. The signal (active high) drives an external NMOS
transistor that pulls down the CC2 line.

UCPDx_CC1

Input/output

USB Type-C configuration control line 1, to be routed to the
USB Type-C connector CC1 terminal.

RM0456 Rev 6

<!-- pagebreak -->

3499

USB Type-C®/USB Power Delivery interface (UCPD)

RM0456

Table 776. UCPD signals on pins (continued)
Pin name

Signal type

Description

UCPDx_CC2

Input/output

UCPDx_DBCC1

Input

USB Type-C configuration control line 1 dead battery signal, to
be routed to the USB Type-C connector CC1 terminal if dead
battery support is required.

UCPDx_DBCC2

Input

USB Type-C configuration control line 2 dead battery signal, to
be routed to the USB Type-C connector CC2 terminal if dead
battery support is required.

USB Type-C configuration control line 2, to be routed to the
USB Type-C connector CC2 terminal.

The following table lists key internal signals.
Table 777. UCPD internal signals

74.4.2

Internal signal name

Signal type

Description

ucpd_pclk

Input

APB clock for registers

ucpd_ker_ck

Input

Kernel clock

ucpd_tx_dma

Input/Output

Rx DMA acknowledge / request

ucpd_rx_dma

Input/Output

Tx DMA acknowledge / request

ucpd_it

Output

Interrupt request (all interrupts OR-ed) connected to NVIC

ucpd_wkup

Output

Wake-up request connected to EXTI

clk_rq

Output

Clock request connected to RCC

UCPD reset and clocks
The peripheral has a single reset signal (APB bus reset).
The register section is clocked with the APB clock (ucpd_pclk).
The main functional part of the transmitter is clocked with ucpd_clk clock, pre-scaled from
the ucpd_ker_ck clock according to the PSC_UCPDCLK[2:0] bitfield of the UCPD_CFGR1
register. The main functional part of the receiver is clocked with the ucpd_rx_clk recovered
from the incoming bitstream.
The receiver is designed to work in the clock frequency range from 6 to 18 MHz. However,
the optimum performance is ensured in the range from 6 to 12 MHz.
The following diagram shows the clocking and timing elements of the UCPD peripheral.

<!-- pagebreak -->

RM0456 Rev 6

USB Type-C®/USB Power Delivery interface (UCPD)

RM0456

Figure 946. Clock division and timing elements
Clock division
ucpd_ker_ck

Pre-scaler
/1 to /16

ucpd_clk

PSC_UCPDCLK[2:0]

BMC receiver
“Half bit” divider
/1 to /64

hbit_clk (~ 600 kHz)

Counters
HBITCLKDIV[5:0]

BMC transmitter

“tTransitionWindow”
2 to 32
TRANSWIN[4:0]

ucpd_pclk

“tInterFrameGap”
2 to 32

Registers

IFRGAP[4:0]
MSv45536V3

Refer to the USB PD specification in order to set appropriate delays. For tTransitionWindow
and especially for tInterFrameGap, the clock frequency uncertainty must be taken into
account so as to respect specified timings in all cases.

74.4.3

Physical layer protocol
The physical layer covers the signaling underlying the USB Power Delivery specification.
On the transmitter side its main function is to form packets according to the defined packet
format including generally:
•

preamble

•

start of packet (SOP, ordered set)

•

payload header

•

payload data

•

cyclic redundancy check (CRC) information

•

end of packet (EOP)

Before going on the CC line, the data stream is BMC-encoded, respecting specified timing
restrictions.
On the receive side, the principle task is to:
•

extract start of packet (SOP, ordered set) information

•

extract payload header

•

extract payload data

•

receive and check CRC

•

receive end of packet (EOP)

The receive is basically a reverse of the transmit process, thus starting with BMC data
stream decoding.

Symbol encoding
Apart from the preamble all symbols are encoded with a 4b5b scheme according to the
specification shown in the following table.

RM0456 Rev 6

<!-- pagebreak -->

3499

USB Type-C®/USB Power Delivery interface (UCPD)

RM0456

Table 778. 4b5b symbol encoding table

<!-- pagebreak -->

Name

4b

5b

Symbol description

0

0000

11110

hex data 0

1

0001

01001

hex data 1

2

0010

10100

hex data 2

3

0011

10101

hex data 3

4

0100

01010

hex data 4

5

0101

01011

hex data 5

6

0110

01110

hex data 6

7

0111

01111

hex data 7

8

1000

10010

hex data 8

9

1001

10011

hex data 9

A

1010

10110

hex data A

B

1011

10111

hex data B

C

1100

11010

hex data C

D

1101

11011

hex data D

E

1110

11100

hex data E

F

1111

11101

hex data F

Sync-1

K-code

11000

Startsynch #1

Sync-2

K-code

10001

Startsynch #2

RST-1

K-code

00111

Hard Reset #1

RST-2

K-code

11001

Hard Reset #2

EOP

K-code

01101

EOP

Reserved

Error

00000

Do Not Use

Reserved

Error

00001

Do Not Use

Reserved

Error

00010

Do Not Use

Reserved

Error

00011

Do Not Use

Reserved

Error

00100

Do Not Use

Reserved

Error

00101

Do Not Use

Sync-3

K-code

00110

Startsynch #3

Reserved

Error

01000

Do Not Use

Reserved

Error

01100

Do Not Use

Reserved

Error

10000

Do Not Use

Reserved

Error

11111

Do Not Use

RM0456 Rev 6

USB Type-C®/USB Power Delivery interface (UCPD)

RM0456

Ordered sets
An ordered set consists of four K-codes as shown in the following figure.
Figure 947. K-code transmission

Transmit last

Transmit first

K-code 4

K-code 3

K-code 2

K-code 1

b4

Bit 3

Bit 4

Bit 2

Bit 1

Transmit last

b0

Bit 0

Transmit first
MSv45537V1

The following table lists the defined ordered sets, including all possible SOP* sequences.
At the physical layer, the Hard Reset has higher priority than the other ordered sets so it can
interrupt an ongoing Tx message.
Table 779. Ordered sets
Ordered set name

K-code #1

K-code #2

K-code #3

K-code #4

SOP

Sync-1

Sync-1

Sync-1

Sync-2

SOP’

Sync-1

Sync-1

Sync-3

Sync-3

SOP’’

Sync-1

Sync-3

Sync-1

Sync-3

Hard Reset

RST-1

RST-1

RST-1

RST-2

Cable Reset

RST-1

Sync-1

RST-1

Sync-3

SOP’_Debug

Sync-1

RST-2

RST-2

Sync-3

SOP’’_Debug

Sync-1

RST-2

Sync-3

Sync-2

On reception, the physical layer must accept ordered sets with any combination of three
correct K-codes out of four, as shown in the following table:
Table 780. Validation of ordered sets
Status

1st code

2nd code

3rd code

4th code

Valid

Corrupt

K-code

K-code

K-code

Valid

K-code

Corrupt

K-code

K-code

RM0456 Rev 6

<!-- pagebreak -->

3499

USB Type-C®/USB Power Delivery interface (UCPD)

RM0456

Table 780. Validation of ordered sets (continued)
Status

1st code

2nd code

3rd code

4th code

Valid

K-code

K-code

Corrupt

K-code

Valid

K-code

K-code

K-code

Corrupt

Valid (perfect)

K-code

K-code

K-code

K-code

Not valid (example)

K-code

Corrupt

K-code

Corrupt

Bit ordering at transmission
Allowed transmission data units / data sizes are in the following table.
Table 781. Data size
Data unit

Non-encoded

Encoded

Byte

8-bits

10-bits

Word

16-bits

20-bits

DWord

32-bits

40-bits

The bit transmission order is shown in the following figure.
Figure 948. Transmit order for various sizes of data
b31

b31

b0

Transmit last b16

b15

b15

Transmit first

b0

b7

b0

b7 b4

b3 b0

b8

4b5b
bit4 bit0

Bit 4

Bit 3

Bit 2

Transmit last

Bit 1

Bit 0

Transmit first
MSv45538V1

<!-- pagebreak -->

RM0456 Rev 6

USB Type-C®/USB Power Delivery interface (UCPD)

RM0456

Packet format
Messages other than Hard Reset and Cable Reset
The packet format is shown in the following figure, with information on 4b5b encode and
data source.
Figure 949. Packet format
Preamble
(training for receiver)

SOP*
(start of packet)

Header

...

Byte 0

Byte n-1

Byte 1

Byte n

...

EOP
(end of packet)

CRC

Legend:
Provided by the physical layer, not 4b5b-encoded
Provided by the physical layer, 4b5b-encoded
Provided by the protocol layer, 4b5b-encoded
MSv45539V2

Hard Reset
The physical layer handles the Hard Reset signaling differently than the other types of
message as it has higher priority to be able to interrupt an ongoing transfer.
The physical layer specification implies the following sequence in the case of an ongoing Tx
message:
1.

Terminate the message by sending an EOP K-code and discard the rest of the
message.

2.

Wait for tInterFrameGap time.

3.

If the CC line is not idle, wait until it goes idle.

4.

Send the preamble followed by the four K-codes of Hard Reset signaling.

5.

Disable the CC channel (stop sending and receiving), reset the physical layer and
inform the protocol layer that the physical layer is reset.

6.

Re-enable the channel when requested by the protocol layer.
Figure 950. Line format of Hard Reset

Preamble
(training for receiver)

RST-1

RST-1

RST-1

RST-2

Hard Reset ordered set
Legend:
Provided by the physical layer, not 4b5b-encoded
Provided by the physical layer, 4b5b-encoded
MSv45540V2

RM0456 Rev 6

<!-- pagebreak -->

3499

USB Type-C®/USB Power Delivery interface (UCPD)

RM0456

Cable Reset
Cable Reset shown in the following figure is similar in format to Hard Reset, but unlike Hard
Reset it does not require a specific high-priority treatment.
Figure 951. Line format of Cable Reset

Preamble
(training for receiver)

RST-1

Sync-1

RST-1

Sync-3

Cable Reset ordered set
Legend:
Provided by the physical layer, not 4b5b-encoded
Provided by the physical layer, 4b5b-encoded
MSv45541V2

Collision avoidance
The physical layer respects the tInterFrameGap delay between end of last-transmitted bit of
a Tx message, and the first bit of a following message.
It also checks the idle state of the CC line before starting transmission. The CC line is
considered idle if it shows less than three (nTransitionCount) transitions within
tTransitionWindow (12 to 20 μs). The Power Delivery specification revision 3.1 also requires
to manage the Rp value (source) and monitor Type-C voltage level for these Rp
modifications (at the sink).

Physical layer signaling schemes
The bit are signaled with bi-phase mark coding (BMC).

BIST
Depending on the BIST action required by the protocol layer, either of the following can be
run:
•

a Tx BIST pattern test, achieved by writing TXMODE and TXSEND

•

an Rx BIST pattern test, achieved by writing RXMODE to the correct value for RXBIST.

The two possible patterns supported in UCPD (corresponding to “BMC” mode) are:
•

BIST Test Data (192 bit pattern), applies to Tx and Rx. In the case of Rx, the message
is received (but discarded rather than passing to the protocol layer, which must
nevertheless still generate a GoodCRC Tx message in acknowledgment).

•

BIST Carrier Mode 2 (single pattern, infinite length message), applies to Tx only. As
opposed to Tx, the receiver in this mode simply ignores the CC line during this state.

BIST test data pattern
The test data pattern is not viewed as a special case in UCPD.

<!-- pagebreak -->

RM0456 Rev 6

USB Type-C®/USB Power Delivery interface (UCPD)

RM0456

The BIST test data packet frame format is shown in the following figure.
Figure 952. BIST test data frame
Preamble
(training for receiver)

SOP*
(start of packet)

Header
Data objects = 7

BIST Test Data
BDO

...

CRC

BIST test data
(192 bits)

...

EOP
(end of packet)

Legend:
Provided by the physical layer, not 4b5b-encoded
Provided by the physical layer, 4b5b-encoded
Provided by the protocol layer, 4b5b-encoded
MSv45542V2

This is a fixed length test data pattern. In reality the only aspect that marks its difference
from the general packet format already shown in Figure 949: Packet format is the contents
of the Header. As UCPD receives the Tx Header contents via programming (it is simply
viewed as part of the payload), it is only this programming (and not the block’s behavior) that
differentiates the general packet from the BIST Test Data packet.
BIST Carrier Mode 2
When required, this BIST test mode sends an alternating pattern of 1010 that is continually
repeated. As this mode is intended for signal analysis it is stable condition with (in V1.0 of
the USB PD specification) no defined length. Starting from V1.1 of the USB PD
specification, the protocol layer defines a counter that indicates when to exit this mode.
The way to quit the infinite 1010 sequence (according to requirements of the USB PD
specification) is to disable the UCPD peripheral via the UCPDEN bit.
Figure 953. BIST Carrier Mode 2 frame

Preamble
(training for receiver)

SOP*
(start of packet)

10101010101010101010……
(repeated until UCPD is disabled)

UCPDEN = 0

Legend:
Provided by the physical layer, not 4b5b-encoded
Provided by the physical layer, 4b5b-encoded

MSv45543V2

74.4.4

UCPD BMC transmitter
The BMC transmitter comprises 4b5b encoding, CRC generation, and BMC encode, as
shown in the following figure. Its output goes to the analog PHY through a channel switch.

RM0456 Rev 6

<!-- pagebreak -->

3499

USB Type-C®/USB Power Delivery interface (UCPD)

RM0456

Figure 954. UCPD BMC transmitter architecture
ucpd_clk
clock domain

ucpd_pclk
clock domain

UCPD_TXDR

Message
data

CRC

Registers
Tx message
contents

UCPD_TX_ORDSET

UCPD_TX_PAYSZ
Tx message
control

UCPD_CR

Configuration of
the peripheral

UCPD_CFG1
UCPD_CFG2
UCPD_CFG3

hbit_clk
clock domain

Message
data

Fixed
framing

4b5b
encode

BMC
encode

To analog
PHY

Message
parameters
Message
control

USB PD
transmitter
state machine

General
configuration

MSv45544V2

BMC encoder
The bi-phase mark coding method is defined in the IEC 60958-1 Digital Audio Interface
Part:1 General Edition 3.0 2008-09 www.iec.ch specification.
The half-bit clock hbit_clk is derived from ucpd_clk through a simple divider controlled by the
HBITCLKDIV[5:0] bitfield of the UCPD_CFGR1 register. This ensures the same duration of
high and low half-bit periods (if neglecting a small difference due to different rising and
falling edge duration and due to jitter), and the same bit duration (if neglecting jitter).
Transmitter timing and collision avoidance
Hardware support of collision avoidance is made as a function of the half bit time for the
transmitter. Two counters are implemented:
–

tInterFrameGap: via IFRGAP (pre-defined value, can be altered)

–

tTransitionWindow: via TRANSWIN (pre-defined value, can be altered)

These two counters once set correctly generates the interframe gap.
Hard Reset in transmitter
In order to facilitate generation of a Hard Reset, a special code of TXMODE field is used. No
other fields need to be written.
On writing the correct code, the hardware forces Hard Reset Tx under the correct (optimal)
timings with respect to an ongoing Tx message, which (if still in progress) is cleanly
terminated by truncating the current sequence and directly appending an EOP K-code
sequence. No specific interrupt is generated relating to this truncation event.
Transmitter behavior in the case of errors
The under-run condition (TXUND interrupt) may happen by accident and in this case, the
UCPD is starved of (the correct) Tx payload and is not able to complete the Tx message
correctly. This is a serious error (for this to happen the software fails to respond in time). As
a result the hardware ensures the CRC is incorrect at the end of the message, thus
guaranteeing the message to be discarded at the receiver.

<!-- pagebreak -->

RM0456 Rev 6

USB Type-C®/USB Power Delivery interface (UCPD)

RM0456

74.4.5

UCPD BMC receiver
The UCPD BMC receiver performs:
•

Clock recovery

•

Preamble detection / timing derivation

•

BMC decoding

•

4b5b decoding

•

K-code ordered set recognition

•

CRC checking

•

SOP detection

•

EOP detection

The receiver is activated as soon as the UCPD peripheral is enabled (via UCPDEN), but it
waits for an idle CC line state before attempting to receive a message.
The following figure shows the UCPD BMC receiver high-level architecture.
Figure 955. UCPD BMC receiver architecture
ucpd_clk
clock domain

From
analog PHY

Clock
recovery /

ucpd_rx_clk
clock domain
1

20-bit FIFO
20

BMC
decode
ucpd_clk

ucpd_pclk
clock domain

5 latest

Registers

4b5b decode
CRC check

ucpd_rx_clk
Manage
status flags

EOP found

USB PD
receiver
state machine

Ordered
set /
Ordered set
EOP
detect/type
detect

Message
type

UCPD_RXDR

Rx payload data

UCPD_RX_PAYSZ

Rx payload size

UCPD_SR

Rx status flags

UCPD_ICR

Interrupt clear

UCPD_IMR

Interrupt mask

UCPD_CR

Rx control

UCPD_RX_ORDSET
UCPD_RX_ORDSET1

For future
extensions

Ordered
set type

UCPD_RX_ORDSET2
MSv45545V2

CRC checker
The received bits are fed into a CRC checker which evolves a 32-bit state during the
received the payload bitstream. At the end the 32 bits of the CRC also fed into the logic
The EOP detection (5 bits) halts the process and a check is performed for the fixed residual
state which confirms correct reception of the payload (in fact the residual is 0xC704DD78).
At this point the UCPD raises interrupt RXMSGEND. If the CRC was not correct then RXERR
is set true and the receive data must be discarded.
Under normal operation, this interrupt would previously have been acknowledged and thus
cleared. If this is not the case, a different interrupt RXOVR is generated in place of
RXMSGEND.

RM0456 Rev 6

<!-- pagebreak -->

3499

USB Type-C®/USB Power Delivery interface (UCPD)

RM0456

Ordered set detection
This function detects the different ordered sets each consisting of four 5-bit K-codes.
Once we are in the preamble we opens a sliding window detection of the ordered set (4
words of 5 bits).
The ordered sets detected include all SOP* codes (SOP, SOP’, and SOP’’), but also Hard
Reset, Cable Reset, SOP’_Debug, SOP’’_Debug, and two extensions defined by registers
UCPD_RX_ORDEXT1 and UCPD_RX_ORDEXT2.
EOP detection and Hard Reset exception handling
EOP is a fixed 5-bit K-code marking the end of a message.
The way in which a transmitter is required to send a Hard Reset (if a previous message
transmit is still in progress) is that this previous message is truncated early with an EOP.
If Hard Reset were ignored, then the EOP detection can be done only at the expected time.
However, due to the Hard Reset issue, the EOP detector must be active while an Rx
message is arriving. When an “early EOP” is detected, the truncated Rx message is
immediately discarded.
Truncated or corrupted message exception
Once the ordered set has been detected, depending on the message, there may be data
bytes to be received which is completed with a CRC and EOP. If at any point during these
phases an error condition happens:
•

the line becomes static for a time significantly longer than one “UI” period (the exact
threshold for this condition is not critical but the exception must occur before three UIs),
or

•

the message goes to the end but it is not recognized (for example EOP is corrupted).

In both cases, the receiver quits the current message, raising RXMSGEND and RXERR
flags.
Short preamble or incomplete ordered set exception
In the exceptional case of the receiver seeing less that half of the expected preamble, the
frequency estimation allowing correct BMC-decode becomes impossible. Even if the full
preamble is seen, allowing frequency estimation, but the ordered set is not fully received
before the line becomes static, the receiver state machine does not start.
In both of these cases, the clock-recovery/BMC decoder re-starts, checking initially for an
IDLE condition, followed by a preamble, and then estimating frequency.

74.4.6

UCPD Type-C pull-ups (Rp) and pull-downs (Rd)
UCPD offers simple control of these resistors via ANAMODE and ANASUBMODE[1:0]. In
case only one of the CC lines is to be used, it is possible to optimize power consumption by
disabling control on the other line, through the CCENABLE[1:0] bitfield.
When the MCU is unpowered, it still presents the “dead battery” Rd, provided that
UCPDx_DBCC1 and UCPDx_DBCC2 pins are each connected to UCPDx_CC1 and
UCPDx_CC2 pins, respectively.
If dead battery behavior is not required (for example for source only products), then
UCPDx_DBCC1 and UCPDx_DBCC2 pins must both be tied to ground.

<!-- pagebreak -->

RM0456 Rev 6

USB Type-C®/USB Power Delivery interface (UCPD)

RM0456

After power arrives and the MCU boots, the desired behavior (for example source) must be
programmed into ANAMODE and ANASUBMODE[1:0] before setting the UCPD_DBDIS bit
of the PWR_CR3 register to remove dead battery pull-down resistor and allow the values
just programmed to take effect.
Use of Standby low-power mode is possible for sinks in the unattached state.

74.4.7

UCPD Type-C voltage monitoring and de-bouncing
For correct operation of the Type-C state machine and for detecting the cable orientation,
the CC1/2 lines must be monitored for voltage level, while ignoring fast events such as
peaks.
Thresholds between voltage levels on the CC1/2 lines are determined through PHY
threshold detector settings.
The TYPEC_VSTATE_CC1/2[1:0] bitfields reflect the CC1/2 line levels processed with a
hardware de-bouncing filter that suppresses high-speed line events such as peaks. The
PHYCCSEL bit selects the line, CC1 or CC2, to be used for Power Delivery signaling.
For minimizing the power consumption, it is recommended to use the polling method, with
the Type-C detectors only turned on for the instant of polling, rather than keeping the
Type-C detectors permanently on and wake the device up from Stop mode upon CC1/2 line
events.

74.4.8

UCPD fast role swap (FRS)
FRS signaling
The FRS condition (a pulse of a specific length), is generated upon setting the FRSTX bit.
For the duration of FRS condition, the currently active I/O configured as UCPD_FRSTX1 (or
2) (alternate function) controls, with high level, the gate of an external NMOS transistor that
pulls the active CC line down.

FRS detection
FRS monitoring is enabled by setting the bit FRSRXEN, after writing PHYCCSEL that
selects the active CC line depending on the cable orientation detected.

74.4.9

UCPD DMA Interface
DMA is implemented in the UCPD and when it is enabled the byte-level interrupts to handle
UCPD1_TXDR and UCPD1_RXDR registers (Tx and Rx data register, each one byte) are
no longer needed.
By enabling bits TXDMAEN and/or RXDMAEN, DMA can be activated independently for Tx
and/or Rx functionality.

74.4.10

Wake-up from Stop mode
For power consumption optimization, it is useful to use Stop mode and wait for events on
CC lines to wake the MCU up.
In order for this to work, it must be first enabled by writing a 1 to WUPEN.

RM0456 Rev 6

<!-- pagebreak -->

3499

USB Type-C®/USB Power Delivery interface (UCPD)

RM0456

The events causing the wake-up can be:

74.5

•

Events on the BMC receiver (RXORDDET, RXHRSTDET), hardware enable
PHYRXEN

•

Event on the FRS detector (FRSEVT), hardware enable FRSRXEN

•

Events on the Type-C detectors (TYPECEVT1, TYPECEVT2), hardware enables
CC1TCDIS, CC2TCDIS

UCPD programming sequences
The normal sequence of use of the UCPD unit is:
1.

Configure UCPD.

2.

Enable UCPD.

3.

Concurrently:
–

On demand from protocol layer, send Tx message

–

Intercept (poll or wait for interrupt) relevant Rx messages and recover detail to
hand off to protocol layer

Repeat the last point infinitely.

74.5.1

Initialization phase
Use the following sequence for a clean startup:

74.5.2

1.

Prepare all initial clock divider values, by writing the UCPD_CFG register.

2.

Enable the unit, by setting the UCPDEN bit.

3.

Enable the analog Rx filter of either CC line, via the RXAFILTEN bit of the
UCPD_CFGR2 register.

Type-C state machine handling
For the general application cases of source, sink, or dual-role port (the last alternating the
source and the sink), the software must implement a corresponding USB Type-C state
machine. The basic coding is in the following table.
Table 782. Coding for ANAMODE, ANASUBMODE and link with TYPEC_VSTATE_CCx
TYPEC_VSTATE_CCx[1:0]
ANAMODE

ANASUBMODE[1:0]

Notes
00

0: Source

1: Sink

01

10

00: Disabled

Disabled

01: Default USB Rp

-

vRa[Def]

vRd[Def]

vOPEN[Def]

10: 1.5A Rp

-

vRa[1.5]

vRd[1.5]

vOPEN[1.5]

11: 3.0A Rp

-

vRa[3.0]

vRd[3.0]

vOPEN[3.0]

xx

-

vRa

vRd-USB

vRd-1.5

11

N/A

N/A

vRd-3.0

The CCENABLE[1:0] bitfield can disable pull-up/pull-downs on one of the CC lines.
Note:

<!-- pagebreak -->

The Type-C state machine depends not only on CC line levels, but also on VBUS presence
detection (sink mode) and, when in source mode, determines VCONN generation and
RM0456 Rev 6

USB Type-C®/USB Power Delivery interface (UCPD)

RM0456

VBUS state (ON/OFF/+voltage level); discharge). UCPD does not directly control VBUS
generation circuitry nor VCONN load switch (enabling VCONN supply generator to be
connected to the CC line), but the application needs these inputs and controls, to function
correctly.
General programming sequence (with UCPD configured then enabled)
1.

Set ANAMODE and ANASUBMODE[1:0] based on the current position in USB Type-C
state machine (and also the current advertisement in the case of a source). This turns
on the appropriate pull-ups/pull-downs on the CC lines, and defines the voltage levels
that the TYPEC_VSTATE fields represent. Note that before programming, the PHY is
effectively off.

2.

Read TYPEC_VSTATE_CC1/2 to determine the initial Type-C state (for example
whether the local source is connected to a remote sink).

3.

In the case of no connection, wait for a connection event.

4.

Assuming a connection is detected and assuming a local Power Delivery functionality
is implemented, start sending/receiving Power Delivery messages.

5.

When a new interrupt/event occurs on PHYEVT1/2 indicating a change in stable
voltage, re-evaluate the implications and give this input to the Type-C state machine.

Case of a source that needs to change between one of the three possible Rp values
(Default-USB / 1.5A / 3.0A) and the sink connected to it:
•

[Source] Simply reprogram ANASUBMODE[1:0]

•

[Sink behavior from that time] PHYEVT1/2 occurs and the TYPEC_VSTATE1/2
changes accordingly

Programming for a dual-role port (DRP) toggling from source to sink:
•

Simply re-program ANAMODE and ANASUBMODE[1:0] to start the new behavior

Detailed programming sequence (example):

RM0456 Rev 6

<!-- pagebreak -->

3499

USB Type-C®/USB Power Delivery interface (UCPD)

RM0456

Table 783. Type-C sequence (source: 3A); cable/sink connected (Rd on CC1; Ra on CC2)
ANAMODE;
Type-C state ANASUBMO
DE[1:0]

CCENABL
E

Unattached.
SRC
Attachwait.
SRC

PHYCCSE
L

CC[x]
VCONN
EN(1)

0 (don’t
care)
0:Source;
11:Rp3A0

11:both
enabled

00:
[neither]

0:Source;
11:Rp3A0
[SinkTxOK]

Attached.
SRC
[VCONN =>
CC2]

RDCH

0:Source;
10:Rp1A5
[SinkTxNG]

01: CC2
disable
(possible
and
recommend
ed due to
external
VCONN
switch)

0
[Rd on
CC1]

0:
[Norm
al]
10: [CC2
active]

Unattached.
SRC

0:Source;
11:Rp3A0

Comments

PHYEVT Wait for sink attach
1: [VRd- detect ; seen on CC1
3A0]
[EVT1]
Attachwait started (100PHYEVT 200 ms) ; now also see
2: [VRa] the Ra => requesting
VCONN
Timer
(100 ms)
and no
PHYEVT
x

0:Source;
11:Rp3A0
[SinkTxOK]
Unattached
wait.
SRC

Event =>
go to
next line

Local CC2 disconnected
from PHY (VCONN
switch connects VCONN
source to CC2
externally;
Continue to monitor
PHYEVT1
Source wants to initiate
message sequence
(SinkTxNG condition set
first)

SW
timers
(SinkTxN Source finished
G)
message sequence
(SinkTxOK condition
afterwards)

PHYEVT
Wait for Sink
1:
disconnected (Vopen on
[VOpenCC1)
3A0]

11:both
enabled

0 (do not
care)

1:
[discha
rge]
0:
[Norm
al]

00:
[neither]

Discharge VCONN
> 0.8V
[CC2] actively [Rdch];
detection
to < 0.8V
[Details as first line of
table]

1. Two GPIOs to enable VCONN through external load switch components

74.5.3

USB PD transmit
On reception of a message from the protocol layer (that is, to be sent), prepare Tx message
contents by writing the UCPD_TX_ORDSET and UCPD_TX_PAYSZ registers.
The message transmission is triggered by setting the TXSEND bit, with an appropriate
value of the TXMODE bitfield.
When the data byte is transmitted, the TXIS flag is raised to request a new data write to the
UCPD_TXDR register.
This re-iterates until the entire payload of data is transmitted.

<!-- pagebreak -->

RM0456 Rev 6

USB Type-C®/USB Power Delivery interface (UCPD)

RM0456

Upon sending the CRC packet, the TXMSGSENT flag is set to indicate the completion of
the message transmission.

Hard Reset transmission
As soon as it is known that a Hard Reset needs to be transmitted, setting the TXHRST bit of
the UCPD_CR register forces the internal state machine to generate the correct sequence.
The value of UCPD_TX_ORDSET does not require update in this precise case (the correct
code for Hard Reset is sent by UCPD).
The USB Power Delivery specification requires that in the case of an ongoing message
transmission, the Hard Reset takes precedence. In this case, for example, UCPD truncates
the payload of the current message, appending EOP to the end. No notification is available
via the registers (for example through the TXMSGSEND flag). This is justified by the fact
that the Hard Reset takes precedence over any previous activity (for which it is therefore no
longer important to know if it is completed).

Use of DMA for transmission
DMA (Direct Memory Access) can be enabled for transmission by setting the TXDMAEN bit
in the UCPD_CR register.
For each message:

74.5.4

•

Prepare the whole message in memory (starting with two header bytes)

•

Program the DMA operation with a length corresponding to the two header bytes plus a
number of data bytes corresponding to the number of data words multiplied by four

•

Write TXSEND to initiate the message transfer

•

If TXMSGDISC then go back to previous line (TXSEND)

•

Wait for DMA transfer complete interrupt (that is, when last Tx byte written to UCPD)

•

Double-check subsequent TXMSGSENT interrupt appears

USB PD receive
Notification of start of the receive message sequence is triggered by an interrupt on
UCPD_SR (bit RXORDDET).
The information is recovered by reading:
•

UCPD_RX_SOP (on interrupt RXORDDET)

•

UCPD_RXDR (on interrupt RXNE, repeats for each byte)

•

UCPD_RXPAYSZ (on interrupt RXMSGEND)

The data previously read from UCPD_RXDR above must be discarded at this point if the
RXERR flag is set.
If the CRC is valid, the received data is transferred to the protocol layer.
For debug purposes, it may be desirable to track statistics of the number of incorrect
K-codes received (this is done only when 3/4 K-codes were valid as defined in the
specification). This is facilitated through:
•

RXSOP3OF4 bit indicating the presence of at least one invalid K-code

•

RXSOPKINVALID bitfield identifying the order of invalid K-code in the ordered set

RM0456 Rev 6

<!-- pagebreak -->

3499

USB Type-C®/USB Power Delivery interface (UCPD)

RM0456

Use of DMA for reception
DMA (Direct Memory Access) can be enabled for reception by setting the RXDMAEN bit in
the UCPD_CR register.
Whenever a Rx message is expected:

74.5.5

•

Program a DMA receive operation (and spare buffer) a little longer than the maximum
possible message (length depends on extended message support).

•

After receiving RXORDDET, DMA operation starts working in the background.

•

On reception of RXMSGEND interrupt, read RXPAYSZ.

•

Double-check RXPAYSZ vs. the number of DMA Rx bytes (must correspond but DMA
read of RXDR is slightly after RXDR gets last byte).

•

Process the DMA Rx buffer.

•

Prepare next Rx DMA buffer as soon as possible in order to be ready.

UCPD software trimming
The CC pull-up (Rp) and pull-down (Rd) devices must be trimmed on each part, to meet the
required accuracy. The trimming values are saved in the non-volatile memory.
To trim the CC pull-up and pull-down devices by software, apply the following procedure:
1.

Retrieve the trim values from the non-volatile memory (refer to Table 775: UCPD
software trim data)

2.

At initialization, write the trim values to the UCPD_CFGR3 register bitfields as follows:
–

3.

74.6

3A0_CC1[3:0] to TRIM_CC1_RP[3:0]

–

3A0_CC2[3:0] to TRIM_CC2_RP[3:0]

–

Rd_CC1[3:0] to TRIM_CC1_RD[3:0]

–

Rd_CC2[3:0] to TRIM_CC2_RD[3:0]

At each setting of ANASUBMODE to 1A5 or 3A0, respectively, write the trimming
values to the UCPD_CFGR3 register bitfields as follows:
–

1A5_CC1[3:0] or 3A0_CC1[3:0], respectively, to TRIM_CC1_RP[3:0]

–

1A5_CC2[3:0] or 3A0_CC2[3:0], respectively, to TRIM_CC2_RP[3:0]

UCPD low-power modes
A summary of low-power modes is shown below in Table 784: Effect of low power modes on
the UCPD.
Table 784. Effect of low power modes on the UCPD

<!-- pagebreak -->

Mode

Description

Sleep

No effect

Stop

Detection of events (Type-C, BMC Rx, FRS detection) remains operational and
can wake up the MCU.

Standby

UCPD is not operating, and cannot wake up the MCU. Pull-downs remain active
if configured.

Unpowered

Dead battery pull-downs remain active.

RM0456 Rev 6

USB Type-C®/USB Power Delivery interface (UCPD)

RM0456

The UCPD is able to wake up the MCU from Stop mode when it recognizes a relevant
event, either:
•

Type-C event relating to a change in the voltage range seen on either of the CC lines,
visible in TYPEC_VSTATE_CCx

•

Power delivery receive message with an ordered set matching those filtered according
to RXORDSETEN[8:0], visible by reading RXORDSET

Wake-up from Stop mode is enabled by setting the WUPEN bit in the UCPD_CFG2 register.
At UCPD level three types of event requiring kernel clock activity may occur during Stop
mode:
•

Activity on the analog PHY voltage threshold detectors which can later be confirmed to
be a stable change between voltage ranges defined in the Type-C specification

•

Activity on Power Delivery BMC receiver (coming from the selected CC line) which can
potentially generate an Rx message event (that is, RXORDSET) later

•

Activity on Power Delivery FRS detector which can potentially generate an FRS
signaling detection event (that is, FRSEVT) later

It order to function correctly with the RCC, the clock request signal is activated (conditional
on WUPEN) when there is asynchronous activity on:

74.7

•

Type-C voltage threshold detectors (coming from either CC line)

•

Power Delivery receiver signal (from the selected CC line)

•

FRS detection signal (from the selected CC line)

UCPD interrupts
The table below lists the UCPD event flags, with the associated flag clear bits and interrupt
enable bits.
Table 785. UCPD interrupt requests
Interrupt event

Event flag

Event flag/Interrupt
clearing method

Interrupt enable
control bit

FRS detection

FRSEVT

Set FRSEVTCF

FRSEVTIE

Type C voltage level change on CC2

TYPECEVT2

Set TYPECEVT2CF

TYPECEVT2IE

Type C voltage level change on CC1

TYPECEVT1

Set TYPECEVT1CF

TYPECEVT1IE

Rx message received

RXMSGEND

Set RXMSGENDCF

RXMSGENDIE

Rx data overflow

RXOVR

Set RXOVRCF

RXOVR

Rx Hard Reset detected

RXHRSTDET

Set RXHRSTDETCF

RXHRSTDETIE

Rx ordered set (4 K-codes) detected

RXORDDET

Set RXORDDETCF

RXORDDETIE

Receive data register not empty

RXNE

Read data in
UCPD_RXDR

RXNEIE

Tx data underrun

TXUND

Set TXUNDCF

TXUNDIE

Hard Reset sent

HRSTSENT

Set HRSTSENTCF

HRSTSENTIE

Hard Reset discarded

HRSTDISC

Set HRSTDISCCF

HRSTDISCIE

Transmit message aborted

TXMSGABT

Set TXMSGABTCF

TXMSGABTIE

RM0456 Rev 6

<!-- pagebreak -->

3499

USB Type-C®/USB Power Delivery interface (UCPD)

RM0456

Table 785. UCPD interrupt requests (continued)
Interrupt event

Event flag

Event flag/Interrupt
clearing method

Interrupt enable
control bit

Transmit message sent

TXMSGSENT

Set TXMSGSENTCF

TXMSGSENTIE

Transmit message discarded

TXMSGDISC

Set TXMSGDISCCF

TXMSGDISCIE

Transmit data required

TXIS

Write data to the
UCPD_TXDR register

TXISIE

When an interrupt from the UCPD is received, then the software has to check what is the
source of the interrupt by reading the UCPD_SR register.
Depending on which bit is at 1, the ISR must handle that condition and clear the bit by a
write to the appropriate bit of the UCPD_ICR register.

74.8

UCPD registers

74.8.1

UCPD configuration register 1 (UCPD_CFGR1)
Address offset: 0x000
Reset value: 0x0000 0000
General configuration of the peripheral. Writing to this register is only effective when UCPD
is disabled (UCPDEN = 0).

31

30

29

28

27

26

UCPDE RXDM TXDMA
N
AEN
EN

25

24

23

22

21

20

RXORDSETEN[8:0]

19

18

17

16

PSC_UCPDCLK[2:0]

Res.

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

15

14

13

12

11

10

9

8

7

6

5

4

3

2

1

0

rw

rw

TRANSWIN[4:0]
rw

rw

rw

rw

IFRGAP[4:0]
rw

rw

rw

rw

HBITCLKDIV[5:0]
rw

rw

rw

rw

rw

rw

Bit 31 UCPDEN: UCPD peripheral enable
General enable of the UCPD peripheral.
0: Disable
1: Enable
Upon disabling, the peripheral instantly quits any ongoing activity and all control bits and
bitfields default to their reset values. They must be set to their desired values each time the
peripheral transits from disabled to enabled state.
Bit 30 RXDMAEN: Reception DMA mode enable
When set, the bit enables DMA mode for reception.
0: Disable
1: Enable
Bit 29 TXDMAEN: Transmission DMA mode enable
When set, the bit enables DMA mode for transmission.
0: Disable
1: Enable

<!-- pagebreak -->

RM0456 Rev 6

RM0456

USB Type-C®/USB Power Delivery interface (UCPD)

Bits 28:20 RXORDSETEN[8:0]: Receiver ordered set enable
The bitfield determines the types of ordered sets that the receiver must detect. When
set/cleared, each bit enables/disables a specific function:
0bXXXXXXXX1: SOP detect enabled
0bXXXXXXX1X: SOP' detect enabled
0bXXXXXX1XX: SOP'' detect enabled
0bXXXXX1XXX: Hard Reset detect enabled
0bXXXX1XXXX: Cable Detect reset enabled
0bXXX1XXXXX: SOP'_Debug enabled
0bXX1XXXXXX: SOP''_Debug enabled
0bX1XXXXXXX: SOP extension#1 enabled
0b1XXXXXXXX: SOP extension#2 enabled
Bits 19:17 PSC_UCPDCLK[2:0]: Pre-scaler division ratio for generating ucpd_clk
The bitfield determines the division ratio of a kernel clock pre-scaler producing UCPD
peripheral clock (ucpd_clk).
0x0: 1 (bypass)
0x1: 2
0x2: 4
0x3: 8
0x4: 16
It is recommended to use the pre-scaler so as to set the ucpd_clk frequency in the range
from 6 to 9 MHz.
Bit 16 Reserved, must be kept at reset value.
Bits 15:11 TRANSWIN[4:0]: Transition window duration
The bitfield determines the division ratio (the bitfield value minus one) of a hbit_clk divider
producing tTransitionWindow interval.
0x00: Not supported
0x01: 2
0x09: 10 (recommended)
0x1F: 32
Set a value that produces an interval of 12 to 20 us, taking into account the ucpd_clk
frequency and the HBITCLKDIV[5:0] bitfield setting.
Bits 10:6 IFRGAP[4:0]: Division ratio for producing inter-frame gap timer clock
The bitfield determines the division ratio (the bitfield value minus one) of a ucpd_clk divider
producing inter-frame gap timer clock (tInterFrameGap).
0x00: Not supported
0x01: 2
0x0D: 14
0x0E: 15
0x0F: 16
0x1F: 32
The division ratio 15 is to apply for Tx clock at the USB PD 2.0 specification nominal value.
The division ratios below 15 are to apply for Tx clock below nominal, and the division ratios
above 15 for Tx clock above nominal.
Bits 5:0 HBITCLKDIV[5:0]: Division ratio for producing half-bit clock
The bitfield determines the division ratio (the bitfield value plus one) of a ucpd_clk divider
producing half-bit clock (hbit_clk).
0x00: 1 (bypass)
0x1A: 27
0x3F: 64

RM0456 Rev 6

<!-- pagebreak -->

3499

USB Type-C®/USB Power Delivery interface (UCPD)

74.8.2

RM0456

UCPD configuration register 2 (UCPD_CFGR2)
Address offset: 0x004
Reset value: 0x0000 0000
Configuration of the UCPD Rx signal filtering. Writing to this register is only effective when
UCPD is disabled (UCPDEN = 0).

31

30

29

28

27

26

25

24

23

22

21

20

19

18

17

16

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

15

14

13

12

11

10

9

8

7

6

5

4

3

2

1

0

Res.

RXAFIL
TEN

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

rw

Res.

WUPE FORCE RXFILT RXFILT
N
CLK
2N3
DIS
rw

rw

rw

rw

Bits 31:9 Reserved, must be kept at reset value.
Bit 8 RXAFILTEN: Rx analog filter enable
Setting the bit enables the Rx analog filter required for optimum Power Delivery reception.
0: Disable
1: Enable
Bits 7:4 Reserved, must be kept at reset value.
Bit 3 WUPEN: Wake-up from Stop mode enable
Setting the bit enables the UCPD_ASYNC_INT signal.
0: Disable
1: Enable
Bit 2 FORCECLK: Force ClkReq clock request
0: Do not force clock request
1: Force clock request
Bit 1 RXFILT2N3: BMC decoder Rx pre-filter sampling method
Number of consistent consecutive samples before confirming a new value.
0: 3 samples
1: 2 samples
Bit 0 RXFILTDIS: BMC decoder Rx pre-filter enable
0: Enable
1: Disable
The sampling clock is that of the receiver (that is, after pre-scaler).

74.8.3

UCPD configuration register 3 (UCPD_CFGR3)
Address offset: 0x008
Reset value: 0x0000 0000
Configuration of UCPD trimming of the CC pull-up and pull-down devices. The trimming is
managed by hardware until the first software write into this register.
The register is reserved (must not be written) for devices that support the fully automatic
trimming. Refer to Table 774: UCPD implementation.

<!-- pagebreak -->

RM0456 Rev 6

USB Type-C®/USB Power Delivery interface (UCPD)

RM0456

31

30

29

Res.

Res.

Res.

15

14

13

Res.

Res.

Res.

28

27

26

25

TRIM_CC2_RP[3:0]
rw

rw

rw

rw

12

11

10

9

TRIM_CC1_RP[3:0]
rw

rw

rw

24

23

22

21

20

Res.

Res.

Res.

Res.

Res.

8

7

6

5

4

Res.

Res.

Res.

Res.

Res.

rw

19

18

17

16

TRIM_CC2_RD[3:0]
rw

rw

rw

rw

3

2

1

0

TRIM_CC1_RD[3:0]
rw

rw

rw

rw

Bits 31:29 Reserved, must be kept at reset value.
Bits 28:25 TRIM_CC2_RP[3:0]: SW trim value for Rp current sources on the CC2 line
Bits 24:20 Reserved, must be kept at reset value.
Bits 19:16 TRIM_CC2_RD[3:0]: SW trim value for Rd resistor on the CC2 line
Bits 15:13 Reserved, must be kept at reset value.
Bits 12:9 TRIM_CC1_RP[3:0]: SW trim value for Rp current sources on the CC1 line
Bits 8:4 Reserved, must be kept at reset value.
Bits 3:0 TRIM_CC1_RD[3:0]: SW trim value for Rd resistor on the CC1 line

74.8.4

UCPD control register (UCPD_CR)
Address offset: 0x00C
Reset value: 0x0000 0000
Writing to this register is only effective when the peripheral is enabled (UCPDEN = 1).

31

30

29

28

27

26

25

24

23

22

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

15

14

13

12

11

10

9

8

7

6

Res.

Res.

Res.

Res.

CCENABLE[1:0]
rw

rw

ANAM
ODE
rw

21

20

CC2TC CC1TC
DIS
DIS
rw

rw

5

4

19
Res.

3

18
RDCH

rw

rw

rw

rw

rs

16

FRSRX
FRSTX
EN

rw

rs

rw

2

1

0

ANASUBMODE[ PHYCC PHYRX RXMO TXHRS TXSEN
1:0]
SEL
EN
DE
T
D
rw

17

rs

TXMODE[1:0]
rw

rw

Bits 31:22 Reserved, must be kept at reset value.
Bit 21 CC2TCDIS: CC2 Type-C detector disable
The bit disables the Type-C detector on the CC2 line.
0: Enable
1: Disable
When enabled, the Type-C detector for CC2 is configured through ANAMODE and
ANASUBMODE[1:0].
Bit 20 CC1TCDIS: CC1 Type-C detector disable
The bit disables the Type-C detector on the CC1 line.
0: Enable
1: Disable
When enabled, the Type-C detector for CC1 is configured through ANAMODE and
ANASUBMODE[1:0].
Bit 19 Reserved, must be kept at reset value.

RM0456 Rev 6

<!-- pagebreak -->

3499

USB Type-C®/USB Power Delivery interface (UCPD)

RM0456

Bit 18 RDCH: Rdch condition drive
The bit drives Rdch condition on the CC line selected through the PHYCCSEL bit (thus
associated with VCONN), by remaining set during the source-only UnattachedWait.SRC
state, to respect the Type-C state. Refer to "USB Type-C ECN for Source VCONN
Discharge". The CCENABLE[1:0] bitfield must be set accordingly, too.
0: No effect
1: Rdch condition drive
Bit 17 FRSTX: FRS Tx signaling enable.
Setting the bit enables FRS Tx signaling.
0: No effect
1: Enable
The bit is cleared by hardware after a delay respecting the USB Power Delivery specification
Revision 3.1.
Bit 16 FRSRXEN: FRS event detection enable
Setting the bit enables FRS Rx event (FRSEVT) detection on the CC line selected through
the PHYCCSEL bit. 0: Disable
1: Enable
Clear the bit when the device is attached to an FRS-incapable source/sink.
Bit 15 Reserved, must be kept at reset value.
Bit 14 Reserved, must be kept at reset value.
Bit 13 Reserved, must be kept at reset value.
Bit 12 Reserved, must be kept at reset value.
Bits 11:10 CCENABLE[1:0]: CC line enable
This bitfield enables CC1 and CC2 line analog PHYs (pull-ups and pull-downs) according to
ANAMODE and ANASUBMODE[1:0] setting.
0x0: Disable both PHYs
0x1: Enable CC1 PHY
0x2: Enable CC2 PHY
0x3: Enable CC1 and CC2 PHY
A single line PHY can be enabled when, for example, the other line is driven by VCONN via
an external VCONN switch. Enabling both PHYs is the normal usage for sink/source.
Bit 9 ANAMODE: Analog PHY operating mode
0: Source
1: Sink
The use of CC1 and CC2 depends on CCENABLE. Refer to Table 782: Coding for
ANAMODE, ANASUBMODE and link with TYPEC_VSTATE_CCx for the effect of this bitfield
in conjunction with ANASUBMODE[1:0].
Bits 8:7 ANASUBMODE[1:0]: Analog PHY sub-mode
Refer to Table 782: Coding for ANAMODE, ANASUBMODE and link with
TYPEC_VSTATE_CCx for the effect of this bitfield.
Bit 6 PHYCCSEL: CC1/CC2 line selector for USB Power Delivery signaling
0: Use CC1 IO for Power Delivery communication
1: Use CC2 IO for Power Delivery communication
The selection depends on the cable orientation as discovered at attach.

<!-- pagebreak -->

RM0456 Rev 6

USB Type-C®/USB Power Delivery interface (UCPD)

RM0456

Bit 5 PHYRXEN: USB Power Delivery receiver enable
0: Disable
1: Enable
Both CC1 and CC2 receivers are disabled when the bit is cleared. Only the CC receiver
selected via the PHYCCSEL bit is enabled when the bit is set.
Bit 4 RXMODE: Receiver mode
Determines the mode of the receiver.
0: Normal receive mode
1: BIST receive mode (BIST test data mode)
When the bit is set, RXORDSET behaves normally, RXDR no longer receives bytes yet the
CRC checking still proceeds as for a normal message. As this mode prevents reception of
the header (containing MessageID), software has to auto-increment a received MessageID
counter for inclusion in the GoodCRC acknowledge that must still be transmitted during this
test.
Bit 3 TXHRST: Command to send a Tx Hard Reset
0: No effect
1: Start Tx Hard Reset message
The bit is cleared by hardware as soon as the message transmission begins or is discarded.
Bit 2 TXSEND: Command to send a Tx packet
0: No effect
1: Start Tx packet transmission
The bit is cleared by hardware as soon as the packet transmission begins or is discarded.
Bits 1:0 TXMODE[1:0]: Type of Tx packet
Writing the bitfield triggers the action as follows, depending on the value:
0x0: Transmission of Tx packet previously defined in other registers
0x1: Cable Reset sequence
0x2: BIST test sequence (BIST Carrier Mode 2)
Others: invalid
From V1.1 of the USB PD specification, there is a counter defined for the duration of the BIST
Carrier Mode 2. To quit this mode correctly (after the "tBISTContMode" delay), disable the
peripheral (UCPDEN = 0).

74.8.5

UCPD interrupt mask register (UCPD_IMR)
Address offset: 0x010
Reset value: 0x0000 0000
Writing to this register is only effective when the peripheral is enabled (UCPDEN = 1).

31
Res.

30
Res.

29
Res.

28
Res.

27
Res.

26
Res.

25
Res.

24
Res.

23
Res.

22
Res.

21

20

19

18

17

16

Res.

FRSEV
TIE

Res.

Res.

Res.

Res.

3

2

1

0

r
15

14

TYPEC TYPEC
EVT2IE EVT1IE
rw

rw

13

12

Res.

RXMS
GENDI
E
rw

11

10

9

RXHRS RXOR
RXOV
TDETI DDETI
RIE
E
E
rw

rw

rw

8
RXNEI
E

7
Res.

rw

6

5

4

TXMS TXMS TXMS
TXUND HRSTS HRSTD
GABTI GSENT GDISCI TXISIE
IE
ENTIE ISCIE
E
IE
E
rw

rw

rw

rw

rw

rw

rw

Bits 31:21 Reserved, must be kept at reset value.

RM0456 Rev 6

<!-- pagebreak -->

3499

USB Type-C®/USB Power Delivery interface (UCPD)

Bit 20 FRSEVTIE: FRSEVT interrupt enable
0: Disable
1: Enable
Bits 19:16 Reserved, must be kept at reset value.
Bit 15 TYPECEVT2IE: TYPECEVT2 interrupt enable
0: Disable
1: Enable
Bit 14 TYPECEVT1IE: TYPECEVT1 interrupt enable
Bit 13 Reserved, must be kept at reset value.
Bit 12 RXMSGENDIE: RXMSGEND interrupt enable
0: Disable
1: Enable
Bit 11 RXOVRIE: RXOVR interrupt enable
0: Disable
1: Enable
Bit 10 RXHRSTDETIE: RXHRSTDET interrupt enable
0: Disable
1: Enable
Bit 9 RXORDDETIE: RXORDDET interrupt enable
0: Disable
1: Enable
Bit 8 RXNEIE: RXNE interrupt enable
0: Disable
1: Enable
Bit 7 Reserved, must be kept at reset value.
Bit 6 TXUNDIE: TXUND interrupt enable
0: Disable
1: Enable
Bit 5 HRSTSENTIE: HRSTSENT interrupt enable
0: Disable
1: Enable
Bit 4 HRSTDISCIE: HRSTDISC interrupt enable
0: Disable
1: Enable
Bit 3 TXMSGABTIE: TXMSGABT interrupt enable
0: Disable
1: Enable
Bit 2 TXMSGSENTIE: TXMSGSENT interrupt enable
0: Disable
1: Enable
Bit 1 TXMSGDISCIE: TXMSGDISC interrupt enable
0: Disable
1: Enable

<!-- pagebreak -->

RM0456 Rev 6

RM0456

USB Type-C®/USB Power Delivery interface (UCPD)

RM0456

Bit 0 TXISIE: TXIS interrupt enable
0: Disable
1: Enable

74.8.6

UCPD status register (UCPD_SR)
Address offset: 0x014
Reset value: 0x0000 0000
The flags (single-bit status bitfields) are associated with interrupt request. Interrupt is
generated if enabled by the corresponding bit of the UCPD_IMR register.

31
Res.

30
Res.

29
Res.

28
Res.

27
Res.

26
Res.

25
Res.

24
Res.

23
Res.

22
Res.

21
Res.

20

r
15

14

13

12

TYPEC TYPEC
RXMS
RXERR
EVT2
EVT1
GEND
r

r

r

r

11

10

9

RXOV RXHRS RXOR
R
TDET DDET
r

r

r

8
RXNE

7
Res.

r

6

5

4

HRSTS HRSTD
TXUND
ENT
ISC
r

r

19

18

17

16

FRSEV TYPEC_VSTATE TYPEC_VSTATE
T
_CC2[1:0]
_CC1[1:0]

r

r
3
TXMS
GABT
r

r

r

r

2

1

0

TXMS TXMS
GSENT GDISC
r

r

TXIS
r

Bits 31:21 Reserved, must be kept at reset value.
Bit 20 FRSEVT: FRS detection event
The flag is cleared by setting the FRSEVTCF bit.
0: No new event
1: New FRS receive event occurred
Bits 19:18 TYPEC_VSTATE_CC2[1:0]: CC2 line voltage level
The status bitfield indicates the voltage level on the CC2 line in its steady state.
0x0: Lowest
0x1: Low
0x2: High
0x3: Highest
The voltage variation on the CC2 line during USB PD messages due to the BMC PHY
modulation does not impact the bitfield value.
Bits 17:16 TYPEC_VSTATE_CC1[1:0]:
The status bitfield indicates the voltage level on the CC1 line in its steady state.
0x0: Lowest
0x1: Low
0x2: High
0x3: Highest
The voltage variation on the CC1 line during USB PD messages due to the BMC PHY
modulation does not impact the bitfield value.
Bit 15 TYPECEVT2: Type-C voltage level event on CC2 line
The flag indicates a change of the TYPEC_VSTATE_CC2[1:0] bitfield value, which
corresponds to a new Type-C event. It is cleared by setting the TYPECEVT2CF bit.
0: No new event
1: A new Type-C event

RM0456 Rev 6

<!-- pagebreak -->

3499

USB Type-C®/USB Power Delivery interface (UCPD)

RM0456

Bit 14 TYPECEVT1: Type-C voltage level event on CC1 line
The flag indicates a change of the TYPEC_VSTATE_CC1[1:0] bitfield value, which
corresponds to a new Type-C event. It is cleared by setting the TYPECEVT2CF bit.
0: No new event
1: A new Type-C event
Bit 13 RXERR: Receive message error
The flag indicates errors of the last Rx message declared (via RXMSGEND), such as
incorrect CRC or truncated message (a line becoming static before EOP is met). It is
asserted whenever the RXMSGEND flag is set.
0: No error detected
1: Error(s) detected
Bit 12 RXMSGEND: Rx message received
The flag indicates whether a message (except Hard Reset message) has been received,
regardless the CRC value. The flag is cleared by setting the RXMSGENDCF bit.
0: No new Rx message received
1: A new Rx message received
The RXERR flag set when the RXMSGEND flag goes high indicates errors in the lastreceived message.
Bit 11 RXOVR: Rx data overflow detection
The flag indicates Rx data buffer overflow. It is cleared by setting the RXOVRCF bit.
0: No overflow
1: Overflow
The buffer overflow can occur if the received data are not read fast enough.
Bit 10 RXHRSTDET: Rx Hard Reset receipt detection
The flag indicates the receipt of valid Hard Reset message. It is cleared by setting the
RXHRSTDETCF bit.
0: Hard Reset not received
1: Hard Reset received
Bit 9 RXORDDET: Rx ordered set (4 K-codes) detection
The flag indicates the detection of an ordered set. The relevant information is stored in the
RXORDSET[2:0] bitfield of the UCPD_RX_ORDSET register. It is cleared by setting the
RXORDDETCF bit.
0: No ordered set detected
1: A new ordered set detected
Bit 8 RXNE: Receive data register not empty detection
The flag indicates that the UCPD_RXDR register is not empty. It is automatically cleared
upon reading UCPD_RXDR.
0: Rx data register empty
1: Rx data register not empty
Bit 7 Reserved, must be kept at reset value.
Bit 6 TXUND: Tx data underrun detection
The flag indicates that the Tx data register (UCPD_TXDR) was not written in time for a
transmit message to execute normally. It is cleared by setting the TXUNDCF bit.
0: No Tx data underrun detected
1: Tx data underrun detected

<!-- pagebreak -->

RM0456 Rev 6

RM0456

USB Type-C®/USB Power Delivery interface (UCPD)

Bit 5 HRSTSENT: Hard Reset message sent
The flag indicates that the Hard Reset message is sent. The flag is cleared by setting the
HRSTSENTCF bit.
0: No Hard Reset message sent
1: Hard Reset message sent
Bit 4 HRSTDISC: Hard Reset discarded
The flag indicates that the Hard Reset message is discarded. The flag is cleared by setting
the HRSTDISCCF bit.
0: No Hard Reset discarded
1: Hard Reset discarded
Bit 3 TXMSGABT: Transmit message abort
The flag indicates that a Tx message is aborted due to a subsequent Hard Reset message
send request taking priority during transmit. It is cleared by setting the TXMSGABTCF bit.
0: No transmit message abort
1: Transmit message abort
Bit 2 TXMSGSENT: Message transmission completed
The flag indicates the completion of packet transmission. It is cleared by setting the
TXMSGSENTCF bit.
0: No Tx message completed
1: Tx message completed
In the event of a message transmission interrupted by a Hard Reset, the flag is not raised.
Bit 1 TXMSGDISC: Message transmission discarded
The flag indicates that a message transmission was dropped. The flag is cleared by setting
the TXMSGDISCCF bit.
0: No Tx message discarded
1: Tx message discarded
Transmission of a message can be dropped if there is a concurrent receive in progress or at
excessive noise on the line. After a Tx message is discarded, the flag is only raised when the
CC line becomes idle.
Bit 0 TXIS: Transmit interrupt status
The flag indicates that the UCPD_TXDR register is empty and new data write is required (as
the amount of data sent has not reached the payload size defined in the TXPAYSZ bitfield).
The flag is cleared with the data write into the UCPD_TXDR register.
0: New Tx data write not required
1: New Tx data write required

RM0456 Rev 6

<!-- pagebreak -->

3499

USB Type-C®/USB Power Delivery interface (UCPD)

74.8.7

RM0456

UCPD interrupt clear register (UCPD_ICR)
Address offset: 0x018
Reset value: 0x0000 0000
Writing to this register is only effective when the peripheral is enabled (UCPDEN = 1).

31

30

29

28

27

26

25

24

23

22

21

20

19

18

17

16

FRSEV
TCF

Res.

Res.

Res.

Res.

3

2

1

0

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

15

14

13

12

11

10

9

8

7

6

5

Res.

RXMS
GEND
CF

w

TYPEC TYPEC
EVT2C EVT1C
F
F
w

w

w

RXHRS RXOR
RXOV
TDETC DDETC
RCF
F
F
w

w

Res.

Res.

w

4

TXMS TXMS TXMS
TXUND HRSTS HRSTD
GABTC GSENT GDISC
CF
ENTCF ISCCF
F
CF
CF
w

w

w

w

Bits 31:21 Reserved, must be kept at reset value.
Bit 20 FRSEVTCF: FRS event flag (FRSEVT) clear
Setting the bit clears the FRSEVT flag in the UCPD_SR register.
Bits 19:16 Reserved, must be kept at reset value.
Bit 15 TYPECEVT2CF: Type-C CC2 line event flag (TYPECEVT2) clear
Setting the bit clears the TYPECEVT2 flag in the UCPD_SR register
Bit 14 TYPECEVT1CF: Type-C CC1 event flag (TYPECEVT1) clear
Setting the bit clears the TYPECEVT1 flag in the UCPD_SR register
Bit 13 Reserved, must be kept at reset value.
Bit 12 RXMSGENDCF: Rx message received flag (RXMSGEND) clear
Setting the bit clears the RXMSGEND flag in the UCPD_SR register.
Bit 11 RXOVRCF: Rx overflow flag (RXOVR) clear
Setting the bit clears the RXOVR flag in the UCPD_SR register.
Bit 10 RXHRSTDETCF: Rx Hard Reset detect flag (RXHRSTDET) clear
Setting the bit clears the RXHRSTDET flag in the UCPD_SR register.
Bit 9 RXORDDETCF: Rx ordered set detect flag (RXORDDET) clear
Setting the bit clears the RXORDDET flag in the UCPD_SR register.
Bits 8:7 Reserved, must be kept at reset value.
Bit 6 TXUNDCF: Tx underflow flag (TXUND) clear
Setting the bit clears the TXUND flag in the UCPD_SR register.
Bit 5 HRSTSENTCF: Hard reset send flag (HRSTSENT) clear
Setting the bit clears the HRSTSENT flag in the UCPD_SR register.
Bit 4 HRSTDISCCF: Hard reset discard flag (HRSTDISC) clear
Setting the bit clears the HRSTDISC flag in the UCPD_SR register.
Bit 3 TXMSGABTCF: Tx message abort flag (TXMSGABT) clear
Setting the bit clears the TXMSGABT flag in the UCPD_SR register.
Bit 2 TXMSGSENTCF: Tx message send flag (TXMSGSENT) clear
Setting the bit clears the TXMSGSENT flag in the UCPD_SR register.

<!-- pagebreak -->

RM0456 Rev 6

w

w

Res.

USB Type-C®/USB Power Delivery interface (UCPD)

RM0456

Bit 1 TXMSGDISCCF: Tx message discard flag (TXMSGDISC) clear
Setting the bit clears the TXMSGDISC flag in the UCPD_SR register.
Bit 0 Reserved, must be kept at reset value.

74.8.8

UCPD Tx ordered set type register (UCPD_TX_ORDSETR)
Address offset: 0x01C
Reset value: 0x0000 0000
Writing to this register is only effective when the peripheral is enabled (UCPDEN = 1) and
no packet transmission is in progress (TXSEND and TXHRST bits are both low).

31

30

29

28

27

26

25

24

23

22

21

20

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

15

14

13

12

11

10

9

8

7

19

18

17

16

TXORDSET[19:16]
rw

rw

rw

rw

6

5

4

3

2

1

0

rw

rw

rw

rw

rw

rw

rw

TXORDSET[15:0]
rw

rw

rw

rw

rw

rw

rw

rw

rw

Bits 31:20 Reserved, must be kept at reset value.
Bits 19:0 TXORDSET[19:0]: Ordered set to transmit
The bitfield determines a full 20-bit sequence to transmit, consisting of four K-codes, each of
five bits, defining the packet to transmit. The bit 0 (bit 0 of K-code1) is the first, the bit 19 (bit
4 of K-code4) the last.

74.8.9

UCPD Tx payload size register (UCPD_TX_PAYSZR)
Address offset: 0x020
Reset value: 0x0000 0000
Writing to this register is only effective when the peripheral is enabled (UCPDEN = 1).

31

30

29

28

27

26

25

24

23

22

21

20

19

18

17

16

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

9

8

7

6

5

4

3

2

1

0

rw

rw

rw

rw

15

14

13

12

11

10

Res.

Res.

Res.

Res.

Res.

Res.

TXPAYSZ[9:0]
rw

rw

rw

rw

rw

rw

Bits 31:10 Reserved, must be kept at reset value.

RM0456 Rev 6

<!-- pagebreak -->

3499

USB Type-C®/USB Power Delivery interface (UCPD)

RM0456

Bits 9:0 TXPAYSZ[9:0]: Payload size yet to transmit
The bitfield is modified by software and by hardware. It contains the number of bytes of a
payload (including header but excluding CRC) yet to transmit: each time a data byte is written
into the UCPD_TXDR register, the bitfield value decrements and the TXIS bit is set, except
when the bitfield value reaches zero. The enumerated values are standard payload sizes
before the start of transmission.
0x2: 2 bytes - the size of Control message from the protocol layer
0x6: 6 bytes - the shortest Data message allowed from the protocol layer)
0x1E: 30 bytes - the longest non-extended Data message allowed from the protocol layer
0x106: 262 bytes - the longest possible extended message
0x3FF: 1024 bytes - the longest possible payload (for future expansion)

74.8.10

UCPD Tx data register (UCPD_TXDR)
Address offset: 0x024
Reset value: 0x0000 0000
Writing to this register is only effective when the peripheral is enabled (UCPDEN = 1).

31

30

29

28

27

26

25

24

23

22

21

20

19

18

17

16

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

7

6

5

4

3

2

1

0

rw

rw

rw

15

14

13

12

11

10

9

8

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

TXDATA[7:0]
rw

rw

rw

rw

rw

Bits 31:8 Reserved, must be kept at reset value.
Bits 7:0 TXDATA[7:0]: Data byte to transmit

74.8.11

UCPD Rx ordered set register (UCPD_RX_ORDSETR)
Address offset: 0x028
Reset value: 0x0000 0000

31

30

29

28

27

26

25

24

23

22

21

20

19

18

17

16

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

15

14

13

12

11

10

9

8

7

6

5

4

3

2

1

0

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

RXSOPKINVALID[2:0]
r

Bits 31:7 Reserved, must be kept at reset value.

<!-- pagebreak -->

RM0456 Rev 6

r

r

RXSOP
3OF4
r

RXORDSET[2:0]
r

r

r

USB Type-C®/USB Power Delivery interface (UCPD)

RM0456

Bits 6:4 RXSOPKINVALID[2:0]:
The bitfield is for debug purposes only.
0x0: No K-code corrupted
0x1: First K-code corrupted
0x2: Second K-code corrupted
0x3: Third K-code corrupted
0x4: Fourth K-code corrupted
Others: Invalid
Bit 3 RXSOP3OF4:
The bit indicates the number of correct K-codes. For debug purposes only.
0: 4 correct K-codes out of 4
1: 3 correct K-codes out of 4
Bits 2:0 RXORDSET[2:0]: Rx ordered set code detected
0x0: SOP code detected in receiver
0x1: SOP' code detected in receiver
0x2: SOP'' code detected in receiver
0x3: SOP'_Debug detected in receiver
0x4: SOP''_Debug detected in receiver
0x5: Cable Reset detected in receiver
0x6: SOP extension#1 detected in receiver
0x7: SOP extension#2 detected in receiver

74.8.12

UCPD Rx payload size register (UCPD_RX_PAYSZR)
Address offset: 0x02C
Reset value: 0x0000 0000

31

30

29

28

27

26

25

24

23

22

21

20

19

18

17

16

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

9

8

7

6

5

4

3

2

1

0

r

r

r

r

15

14

13

12

11

10

Res.

Res.

Res.

Res.

Res.

Res.

RXPAYSZ[9:0]
r

r

r

r

r

r

Bits 31:10 Reserved, must be kept at reset value.
Bits 9:0 RXPAYSZ[9:0]: Rx payload size received
This bitfield contains the number of bytes of a payload (including header but excluding CRC)
received: each time a new data byte is received in the UCPD_RXDR register, the bitfield
value increments and the RXMSGEND flag is set (and an interrupt generated if enabled).
0x2: 2 bytes - the size of Control message from the protocol layer
0x6: 6 bytes - the shortest Data message allowed from the protocol layer)
0x1E: 30 bytes - the longest non-extended Data message allowed from the protocol layer
0x106: 262 bytes - the longest possible extended message
0x3FF: 1024 bytes - the longest possible payload (for future expansion)
The bitfield may return a spurious value when a byte reception is ongoing (the RXMSGEND
flag is low).

RM0456 Rev 6

<!-- pagebreak -->

3499

USB Type-C®/USB Power Delivery interface (UCPD)

74.8.13

RM0456

UCPD receive data register (UCPD_RXDR)
Address offset: 0x030
Reset value: 0x0000 0000

31

30

29

28

27

26

25

24

23

22

21

20

19

18

17

16

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

15

14

13

12

11

10

9

8

7

6

5

4

3

2

1

0

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.
r

r

r

r

r

r

RXDATA[7:0]
r

r

Bits 31:8 Reserved, must be kept at reset value.
Bits 7:0 RXDATA[7:0]: Data byte received

74.8.14

UCPD Rx ordered set extension register 1
(UCPD_RX_ORDEXTR1)
Address offset: 0x034
Reset value: 0x0000 0000
Writing to this register is only effective when the peripheral is disabled (UCPDEN = 0).

31

30

29

28

27

26

25

24

23

22

21

20

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

15

14

13

12

11

10

9

8

7

19

18

17

16

RXSOPX1[19:16]
rw

rw

rw

rw

6

5

4

3

2

1

0

rw

rw

rw

rw

rw

rw

rw

RXSOPX1[15:0]
rw

rw

rw

rw

rw

rw

rw

rw

rw

Bits 31:20 Reserved, must be kept at reset value.
Bits 19:0 RXSOPX1[19:0]: Ordered set 1 received
The bitfield contains a full 20-bit sequence received, consisting of four K-codes, each of five
bits. The bit 0 (bit 0 of K-code1) is receive first, the bit 19 (bit 4 of K-code4) last.

<!-- pagebreak -->

RM0456 Rev 6

USB Type-C®/USB Power Delivery interface (UCPD)

RM0456

74.8.15

UCPD Rx ordered set extension register 2
(UCPD_RX_ORDEXTR2)
Address offset: 0x038
Reset value: 0x0000 0000
Writing to this register is only effective when the peripheral is disabled (UCPDEN = 0).

31

30

29

28

27

26

25

24

23

22

21

20

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

19

18

17

16

rw

rw

rw

rw

15

14

13

12

11

10

9

8

7

6

5

4

3

2

1

0

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

RXSOPX2[19:16]

RXSOPX2[15:0]
rw

rw

Bits 31:20 Reserved, must be kept at reset value.
Bits 19:0 RXSOPX2[19:0]: Ordered set 2 received
The bitfield contains a full 20-bit sequence received, consisting of four K-codes, each of five
bits. The bit 0 (bit 0 of K-code1) is receive first, the bit 19 (bit 4 of K-code4) last.

74.8.16

UCPD register map

0

0

UCPD_CFGR2

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

0

0

0

0

0

Res.

Res.

Res.

Res.

Res.

Res.

Res.

0

0

0

0

0

0

Reset value

0

0

0

0

0

0

RM0456 Rev 6

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

Res.

Res.

Res.

Res.

Res.

TRIM_CC1_RP[3:0]

Res.

Res.

Res.

TRIM_CC2_RD[3:0]

Res.

Res.

Res.

Res.

Res.

TRIM_CC2_RP[3:0]

Res.

UCPD_CFGR3

Res.

0x008

0

0

Res.

Reset value

0

RXFILTDIS

0

RXFILT2N3

0

HBITCLKDIV[5:0]

0

FORCECLK

0

TRIM_CC1_RD[3:0]

0

Res.

0

WUPEN

0

Res.

0

Res.

0

Res.

0

Res.

0

IFRGAP[4:0]

RXAFILTEN

0

TRANSWIN[4:0]

0

RXORDSETEN[8:0]

Res.

TXDMAEN

Reset value

Res.

PSC_UCPDCLK[2:0]

UCPDEN

RXDMAEN

0x004

UCPD_CFGR1

Res.

0x000

Register name

Res.

Offset

31
30
29
28
27
26
25
24
23
22
21
20
19
18
17
16
15
14
13
12
11
10
9
8
7
6
5
4
3
2
1
0

Table 786. UCPD register map and reset values

0

0

<!-- pagebreak -->

3499

0x02C

UCPD_RX_PAYSZR

<!-- pagebreak -->

Reset value

RM0456 Rev 6

Res.

Res.

Res.
Res.
Res.

RXOVR
RXHRSTDET
RXORDDET
RXNE

0
0
0
0

RXOVRCF
RXHRSTDETCF
RXORDDETCF
Res.

0
0
0
0
0
0

Reset value
0

0
0

0
0

Reset value

0
0

Reset value

0
0
0

TXUND
HRSTSENT
HRSTDISC
TXMSGABT
TXMSGSENT
TXMSGDISC
TXIS

0
0
0
0
0
0
0

Res.

TXMSGDISCCF

TXDATA[7:0]

TXMSGSENTCF

0

TXMSGABTCF

TXPAYSZ[9:0]

HRSTDISCCF

0

TXUNDCF

TXORDSET[19:0]
HRSTSENTCF

Res.

RXMSGEND
0

RXMSGENDCF

TXMODE[1:0]

0
0
0
0
0

TXISIE

0
TXMSGDISCIE

TXHRST
TXSEND

0
TXMSGSENTIE

RXMODE

0
TXMSGABTIE

PHYRXEN

0
HRSTDISCIE

0
TXUNDIE

0
HRSTSENTIE

PHYCCSEL

RXNEIE

ANASUBMODE[1:0]

RXORDDETIE

0

Res.

RXHRSTDETIE

Res.

ANAMODE

RXOVRIE

Res.

Res.

Res.

CCENABLE[1:0]

RXMSGENDIE

Res.

TYPECEVT1IE

0

Res.

RXERR

0

Res.

TYPECEVT2IE

0

0
0
0
0
0
0

0
0
0
0
0
0
0

0
0
0

0
0

0

0
0

0

0

0
0

0

0

0
0

0

0

0

RXORDSET[2:0]

Res.

TYPECEVT1

0

TYPECEVT1CF

Res.

0

RXSOP3OF4

Res.

TYPECEVT2

0

TYPECEVT2CF

FRSTX
FRSRXEN

Res.

0

RXSOPKINVALID[2:0]

0

Res.

0

Res.

0

Res.

0

Res.

0

Res.

0

Res.

Res.

Res.

Res.

0

Res.

Res.

Res.

Res.

Res.

0

Res.

TYPEC_VSTATE_CC1[1:0]

0

Res.

0

Res.

0

Res.

Res.
RDCH

Res.

0

Res.

Res.

Res.

0

Res.

TYPEC_VSTATE_CC2[1:0]

0

Res.

0

Res.

0

Res.

CC1TCDIS
Res.

FRSEVTIE

0

Res.

Res.

Res.

Res.

FRSEVT

0

Res.

0

0

FRSEVTCF

CC2TCDIS

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

0

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

0

Res.

Res.

Res.

Res.

0

Res.

Res.

Reset value

Res.

Reset value

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

0

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Reset value

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

0

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Reset value

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Reset value

Res.

Res.

Res.

Res.

Res.

UCPD_RX
_ORDSETR

Res.

0x028

Res.

UCPD_TXDR

Res.

0x024

Res.

UCPD_TX_PAYSZR

Res.

UCPD_TX
_ORDSETR

Res.

0x01C
Res.

UCPD_ICR

Res.

0x018

Res.

UCPD_SR

Res.

0x014

Res.

UCPD_IMR

Res.

0x010

Res.

UCPD_CR

Res.

0x00C

Res.

0x020

31
30
29
28
27
26
25
24
23
22
21
20
19
18
17
16
15
14
13
12
11
10
9
8
7
6
5
4
3
2
1
0

Register name

Res.

Offset

Res.

USB Type-C®/USB Power Delivery interface (UCPD)
RM0456

Table 786. UCPD register map and reset values (continued)

0
0
0
0
0
0
0

0

RXPAYSZ[9:0]

0

0

0

0

0

0

USB Type-C®/USB Power Delivery interface (UCPD)

RM0456

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

UCPD_RXDR

Res.

0x030

Register name

Res.

Offset

31
30
29
28
27
26
25
24
23
22
21
20
19
18
17
16
15
14
13
12
11
10
9
8
7
6
5
4
3
2
1
0

Table 786. UCPD register map and reset values (continued)

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

UCPD_RX
_ORDEXTR1

Res.

0x034

Res.

Reset value

Reset value
0x03C 0x3FF

0

0

0

0

0

0

0

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

RXSOPX1[19:0]
0

Res.

0x038

UCPD_RX
_ORDEXTR2

Res.

Reset value

RXDATA[7:0]

0

0

0

0

RXSOPX2[19:0]
0

0

0

0

0

0

0

0

0

0

0

0

Reserved

Refer to Section 2.3 for the register boundary addresses.

RM0456 Rev 6

<!-- pagebreak -->

3499

Debug support (DBG)

RM0456

75

Debug support (DBG)

75.1

DBG introduction
A comprehensive set of debug features is provided to support software development and
system integration:
•

Breakpoint debugging of the CPU core

•

Code execution tracing

•

Software instrumentation

•

Cross-triggering

The debug features can be controlled via a JTAG/Serial-wire debug access port, using
industry standard debugging tools. A trace port allows data to be captured for logging and
analysis.
The debug features are based on Arm CoreSight components.
•

SWJ-DP: JTAG/Serial-wire debug port

•

AHB-AP: AHB access port

•

ROM table

•

System control space (SCS)

•

Breakpoint unit (BPU)

•

Data watchpoint and trace unit (DWT)

•

Instrumentation trace macrocell (ITM)

•

Embedded Trace Macrocell™ (ETM)

•

Cross trigger interface (CTI)

•

Trace port interface unit (TPU)

The debug features are accessible by the debugger via the AHB-AP.
Additional information can be found in the Arm documents referenced in Section 75.13.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Debug support (DBG)

75.2

DBG functional description

75.2.1

DBG block diagram
Figure 956. Block diagram of debug support infrastructure
Trace port

JTAG/Serial-wire port
DWT

JTDI
SWJ-DP

JTDO

AHB-AP

JTMS/SWDIO

BPU

ITM

TRACECK

TPIU

TRACED[3:0]

AHB

TRACESWO

ETM

Core

JTCK/SWCLK

PPB

nJTRST
CPU
Cortex®-M33

DBG_MCU
ROM
table

CTI

MSv49702V1

75.2.2

DBG pins and internal signals
Table 787. JTAG/Serial-wire debug port pins
JTAG debug port

SW debug port

Pin
assignment

Pin name
Type

Description

Type

Description

JTMS/SWDIO

I

JTAG test mode select

IO

Serial-wire data in/out

PA13

JTCK/SWCLK

I

JTAG test clock

I

Serial-wire clock

PA14

JTDI(1)

I

JTAG test data input

-

-

PA15

JTDO

O

JTAG test data output

-

-

PB3

nJTRST

I

JTAG test reset

-

-

PB4

1. TDI is hosted on the same IO as a USBPD-CC line. To avoid pull-up/down conflict, a user option can help to decide
whether the pad is used as TDI or as CC.

Table 788. Trace port pins
Pin name

Type

Description

TRACED0

Trace synchronous data out 0

TRACED1

Trace synchronous data out 1

TRACED2

O

Trace synchronous data out 2

TRACED3

Trace synchronous data out 3

TRACECK

Trace clock

RM0456 Rev 6

Pin assignment

Refer to the datasheet

<!-- pagebreak -->

3631

Debug support (DBG)

RM0456
Table 789. Single-wire trace port pins

Pin name

Type

TRACESWO

O

Description

Pin assignment

Single-wire trace asynchronous data out

PB3(1)

1. TRACESWO is multiplexed with JTDO. This means that single-wire trace is only available when using the serial-wire debug
interface, and not when using JTAG.

75.2.3

DBG reset and clocks
The debug port (SWJ-DP) is reset by a power-on reset and when waking up
from Standby mode.
The debugger supplies the clock for the debug port via the debug interface pin
JTCK/SWCLK. This clock is used to register the serial input data in both serial-wire and
JTAG modes, as well as to operate the state machines and internal logic of the debug port.
This clock must therefore continue to toggle for several cycles after the end of an access, to
ensure that the debug port returns to the idle state.
The SWJ-DP contains an asynchronous interface to the DCLK domain, that covers the rest
of the SWJ-DP and the access port.
The DCLK is a gated version of the system clock.
The DCLK domain is enabled by the debugger using CDBGPWRUPREQ
in DP_CTRL/STATR. The clock must be enabled before the debugger can access any of the
device debug features. The availability of the clock is reflected by CDBGPWRUPACK
in DP_CTRL/STATR. The DCLK is disabled at power-up, and must be disabled when
the debugger is disconnected, to avoid wasting energy.
The debug and trace components included in the processor are clocked with the processor
clock.

75.2.4

DBG power domains
The debug components are located in the core power domain. This means that the
debugger connection is not possible in Shutdown or Standby low-power mode. To avoid
losing the connection when the device enters Standby mode, the power can be maintained
to the core by setting a bit in DBGMCU_CR. This also keeps the processor clocks active
and holds off the reset, so that the debug session is maintained.

75.2.5

Debug and low-power modes
The devices include power saving features that allow the core power domain to be switched
off or stopped when not required. If the power is switched off or if the core is not clocked, all
debug components are inaccessible to the debugger. To avoid this, power-saving mode
emulation is implemented. If the emulation is enabled for a domain, the domain still enters
power-saving mode, but its clock and power are maintained. In other words, the domain
behaves as if it is in power-saving mode, but the debugger does not lose the connection.
The emulation mode is programmed in the microcontroller debug (DBGMCU) unit. For more
information, refer to Section 75.12.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

75.2.6

Debug support (DBG)

Security
The trace and debug components allow a high degree of access to the processor and
system during product development. In order to protect user code and ensure that the
debug features can not be used to alter or compromise the normal operation of the finished
product, these features can be disabled or limited in scope. For example, secure software
debug and trace can be disabled without preventing the debug of nonsecure code.
The following authentication signals are used by the system to determine which debug
features are enabled or disabled:
•

dbgen: global enable for all debug features
0: All debug features are disabled.
1: Debug features in nonsecure state are enabled. Debug features in secure state are
dependent on the state of the spiden signal.

•

spiden: enables debug in secure state when dbgen = 1.
0: Debug features are disabled in secure state.
1: Debug features are enabled in secure state.

•

niden: enables trace and performance monitoring (non-invasive debug).
0: Trace generation is disabled.
1: Trace generation in nonsecure state is enabled. Trace generation in secure state is
dependent on the state of the spniden signal.

•

spniden: enables trace and performance monitoring in secure state when niden = 1.
0: Trace generation is disabled in secure state.
1: Trace generation is enabled in secure state.

For detailed information on the behavior of each component according to the state of the
authentication signals, refer to the relevant component chapter or to the relevant Arm
technical documentation.
The state of the signals are set according to the readout protection (RDP) level
(see Section 7.6.2: Readout protection (RDP)), as shown in the table below:
Table 790. Authentication signal states
RDP level

Authentication signal
state

0

dbgen = 1, spiden = 1
niden = 1, spniden = 1

Debug and trace is enabled whatever the state of the processor. The
debugger access to secure memory is permitted.

0.5

dbgen = 1, spiden = 0
niden = 1, spniden = 0

Debug and trace is enabled when the processor is in nonsecure state. The
debugger access to secure memory is disabled.

1

dbgen = 1, spiden = 0
niden = 1, spniden = 0

Debug and trace is enabled when the processor is in nonsecure state. The
debugger access to secure memory is disabled, as well as to the following
areas: flash memory, SRAM2, backup registers, ICACHE, on-the-fly
decryption region (OCTOSPI).

2

dbgen = 0, spiden = 0
niden = 0, spniden = 0

Debug and trace is disabled.

Description

RM0456 Rev 6

<!-- pagebreak -->

3631

Debug support (DBG)
Note:

RM0456

Security features are only relevant when the option bit TZEN = 1. If security features are
disabled, the authentication signals are still set according to the RDP level, but since the
processor and all memories are nonsecure, spniden and spiden are redundant.
The state of the authentication signals can be read from DAUTHSTATUS register in the
system control space (SCS) of the Cortex-M33.
The debugger access to secure memory (when permitted) must be performed using secure
transactions on the debug AHB, that is, with PROT[6] set in AP_CSWR.
The debugger access is disabled while the processor is booting from system flash memory
(RSS), whatever the RDP level, if security features are enabled (TZEN = 1).

75.3

Serial-wire and JTAG debug port (SWJ-DP)
The SWJ-DP is a CoreSight component that implements an external access port
for connecting debugging equipment.
Two types of interface can be configured:
•

a 5-pin standard JTAG interface (JTAG-DP)

•

a 2-pin (clock + data) serial-wire debug port (SW-DP)

These two modes are mutually exclusive, since they share the same IO pins.
By default, the JTAG-DP is selected after a system or a power-on reset. The five IO pins are
configured by hardware in debug alternative function mode. The SWJ-DP incorporates
pull-up resistors on JTDI, JTMS/SWDIO, and nJTRST, as well as a pull-down resistor on
JTCK/SWCLK.
A debugger can select the SW-DP by transmitting the following serial data sequence on
JTMS/SWDIO:
... (50 or more ones) ..., 0, 1, 1, 1, 1, 0, 0, 1, 1, 1, 1, 0, 0, 1, 1, 1, ... (50 or more ones) ...
JTCK/SWCLK must be cycled for each data bit.
In SW-DP mode, the unused JTAG pins JTDI, JTDO and nJTRST can be used for other
functions.
Note:

<!-- pagebreak -->

All SWJ port I/Os can be reconfigured to other functions by software, but debugging is no
longer possible.

RM0456 Rev 6

RM0456

75.3.1

Debug support (DBG)

JTAG debug port
The JTAG-DP implements a TAP state machine (TAPSM), shown in the figure below, based
on IEEE Std 1149.1-1990. The state machine controls two scan chains, one associated with
an instruction register (IR) and the other one with a number of data registers (DR).
Figure 957. JTAG TAP state machine

JTMS=1
Test-LogicReset

JTMS=0

JTMS=0

Run-Test/
Idle

JTMS=1

JTMS=1

SelectDR-Scan

SelectIR-Scan

JTMS=1
JTMS=0

JTMS=0
JTMS=1

JTMS=1

CaptureDR

CaptureIR

JTMS=0

JTMS=0

Shift-DR

JTMS=1
Exit1-DR

Shift-IR

JTMS=0

JTMS=1

JTMS=1

JTMS=0

PauseDR

JTMS=0

Pause-IR

JTMS=1

JTMS=0

JTMS=0

Exit2-DR

JTMS=0

Exit2-IR

JTMS=1

JTMS=1
UpdateDR

JTMS=1

JTMS=1

Exit1-IR

JTMS=0

JTMS=1

JTMS=0

UpdateIR

JTMS=0

JTMS=1

JTMS=0

MS44486V1

The operation of the JTAG-DP is as follows:
1.

When the TAPSM goes through the Capture-IR state, 0b0001 is transferred to the
instruction register (IR) scan chain. The IR scan chain is connected between JTDI and
JTDO.

2.

While the TAPSM is in the Shift-IR state, the IR scan chain shifts one bit for each rising
edge of JTCK. This means that on the first tick:
–

The LSB of the IR scan chain is output on JTDO.

–

Bit[n] of the IR scan chain is transferred to bit[n-1].

–

The value on JTDI is transferred to the MSB of the IR scan chain.

RM0456 Rev 6

<!-- pagebreak -->

3631

Debug support (DBG)

RM0456

3.

When the TAPSM goes through the Update-IR state, the value scanned into the IR
scan chain is transferred to the instruction register.

4.

When the TAPSM goes through the Capture-DR state, a value is transferred from one
of the data registers to one of the DR scan chains, connected between JTDI and JTDO.

5.

The value held in the instruction register determines which data register, and
associated DR scan chain, are selected.

6.

This data is then shifted while the TAPSM is in the Shift-DR state, in the same manner
as the IR shifts in the Shift-IR state.

7.

When the TAPSM goes through the Update-DR state, the value scanned into the DR
scan chain is transferred to the selected data register.

8.

When the TAPSM is in the Run-Test/Idle state, no special actions occurs. The IDCODE
instruction is loaded in IR.

When active, the nJTRST signal resets the state machine asynchronously
to the test-logic-reset state.
The data registers corresponding to the 4-bit IR instructions are listed in the table below.
Table 791. JTAG-DP data registers
IR
instruction

DR
register

Scan chain
length

0000 to 0111

(BYPASS)

1

Not implemented: BYPASS selected

1000

ABORT

35

ABORT register
– bits 31:1 = reserved
– bit 0 = APABORT: write 1 to generate an AP abort.

1001

(BYPASS)

1

Reserved: BYPASS selected

35

Debug port access register
Initiates the debug port and gives access to a debug port register.
– When transferring data IN:
bits 34:3 = DATA[31:0] = 32-bit data to transfer for a write request
bits 2:1 = A[3:2] = 2-bit address of a debug port register
bit 0 = RnW = read request (1) or write request (0)
– When transferring data OUT:
bits 34:3 = DATA[31:0] = 32-bit data read following a read request
bits 2:0 = ACK[2:0] = 3-bit acknowledge:
–
010 = OK/FAULT
–
001 = WAIT
–
others = reserved

1010

<!-- pagebreak -->

DPACC

Description

RM0456 Rev 6

RM0456

Debug support (DBG)
Table 791. JTAG-DP data registers (continued)

IR
instruction

DR
register

Scan chain
length

Description

1011

APACC

35

Access port access register
Initiates an access port and gives access to an access port register.
– When transferring data IN:
bits 34:3 = DATA[31:0] = 32-bit data to shift in for a write request
bits 2:1 = A[3:2] = 2-bit sub-address of an access port register
bit 0 = RnW= Read request (1) or write request (0)
– When transferring data OUT:
bits 34:3 = DATA[31:0] = 32-bit data read following a read request
bits 2:0 = ACK[2:0] = 3-bit Acknowledge:
–
010 = OK/FAULT
–
001 = WAIT
–
others= reserved

1100

(BYPASS)

1

Reserved: BYPASS selected

1101

(BYPASS)

1

Reserved: BYPASS selected

1110

IDCODE

32

Identification code
0x0BA0 4477: Cortex-M33 JTAG debug port ID code

1111

BYPASS

1

Bypass: A single JTCK cycle delay is inserted between JTDI and JTDO.

The DR registers are detailed in the Arm Debug Interface Architecture Specification
(see Section 75.13).

75.3.2

Serial-wire debug port
The serial-wire debug protocol uses the following pins:
•

SWCLK: clock from host to target

•

SWDIO: bi-directional serial data

Serial data is transferred LSB first, synchronously with the clock.
A transfer comprises three phases:
1.

packet request (8 bits) transmitted by the host (see Table 792)

2.

acknowledge response (3 bits) transmitted by the target (see Table 793)

3.

data transfer (33 bits) transmitted by the host (in case of a write) or target
(in case of a read) (see Table 794)

The data transfer only occurs if the acknowledge response is OK.
Between each phase, if the direction of the data is reversed, a single clock cycle
turn-around time is inserted.
Table 792. Packet request
Bit field

Name

Description

0

Start

Must be 1.

1

APnDP

– 0: DP register access - see Section 75.3.3: Debug port registers
– 1: AP register access - see Section 75.4: Access ports

RM0456 Rev 6

<!-- pagebreak -->

3631

Debug support (DBG)

RM0456
Table 792. Packet request (continued)

Bit field

Name

Description

2

RnW

– 0: write request
– 1: read request

4:3

A(3:2)

Address field of the DP or AP register (refer to Table 795 or Table 796)

5

Parity

Single bit parity of preceding bits

6

Stop

0

7

Park

Not driven by host, must be read as 1 by target.

Table 793. ACK response
Bit field

Name

2:0

Description
– 000: FAULT
– 010: WAIT
– 100: OK

ACK

Table 794. Data transfer
Bit field

Name

31:0

WDATA or RDATA

32

Parity

Description
Write or read data
Single bit parity of 32 data bits

In the case of a FAULT or WAIT ACK response from the target, the data transfer phase is
canceled, unless overrun detection is enabled: in this case, the data is ignored by the target
(in the case of a write), or not driven (in the case of a read).
A line reset must be generated by the host when it is first connected, or following a protocol
error. The line reset consists in 50 or more SWCLK cycles with SWDIO high, followed by
two SWCLK cycles with SWDIO low.
For more details on the serial-wire debug protocol, refer to the Arm Debug Interface
Architecture Specification [1].
Note:

The SWJ-DP implements SWD protocol version 2.

75.3.3

Debug port registers
Both SW-DP and JTAG-DP access the debug port (DP) registers listed in Table 795.
The debugger can access the DP registers as follows:

<!-- pagebreak -->

1.

Program the A(3:2) field in the DPACC register, if using JTAG, with the register address
within the bank. Program the RnW bit to select a read or write. In the case of a write,
program the data field with the write data. If using SWD, the A(3:2) and RnW fields are
part of the packet request word sent to the SW-DP with the APnDP bit reset
(see Table 792). The write data are sent in the data phase.

2.

To access one of the banked DP registers at address 0x4, the register number must
first be written to the DP_SELECTR register at address 0x8. Any subsequent read or

RM0456 Rev 6

RM0456

Debug support (DBG)
write to address 0x4 access the register corresponding to the contents
of DP_SELECTR.

DP debug port identification register (DP_DPIDR)
Address offset: 0x0
Reset value: 0x0BE0 2477 (SW-DP), 0x0BE0 1477 (JTAG-DP)
31

30

29

28

27

26

25

24

REVISION[3:0]
r
15

23

r

r

r

r

r

r

r

r

14

13

12

11

10

9

8

7

VERSION[3:0]
r

r

r

22

21

20

PARTNO[7:0]
r

r

r

6

5

4

19

18

17

16

Res.

Res.

Res.

MIN
r

3

2

1

DESIGNER[10:0]
r

r

r

r

r

r

r

0
Res.

r

r

r

r

r

Bits 31:28 REVISION[3:0]: revision code
0x0 (JTAG-DP): r0p0
0x0 (SW-DP): r0p0
Bits 27:20 PARTNO[7:0]: part number for the debug port
0xBE
Bits 19:17 Reserved, must be kept at reset value.
Bit 16 MIN: minimal debug port (MINDP) implementation
0x1: MINDP implemented (transaction counter and pushed operations are not supported)
Bits 15:12 VERSION[3:0]: debug port architecture version
0x1 (JTAG-DP): DPv1
0x2 (SW-DP): DPv2
Bits 11:1 DESIGNER[10:0]: JEDEC designer identity code
0x23B: Arm JEDEC code
Bit 0 Reserved, must be kept at reset value.

DP abort register (DP_ABORTR)
Address offset: 0x0
Reset value: 0x0000 0000
31

30

29

28

27

26

25

24

23

22

21

20

19

18

17

16

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

15

14

13

12

11

10

9

8

7

6

5

4

3

2

1

0

Res.

DAPAB
ORT

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

ORUN
WDER STKER
ERRCL
RCLR RCLR
R
w

w

w

w

Bits 31:5 Reserved, must be kept at reset value.

RM0456 Rev 6

<!-- pagebreak -->

3631

Debug support (DBG)

RM0456

Bit 4 ORUNERRCLR: overrun error clear
0: no effect
1: STICKYORUN bit cleared in DP_CTRL/STATR register
Bit 3 WDERRCLR: write data error clear
0: no effect
1: WDATAERR bit cleared in DP_CTRL/STATR register
Bit 2 STKERRCLR: sticky error clear
0: no effect
1: STICKYERR bit cleared in DP_CTRL/STATR register
Bit 1 Reserved, must be kept at reset value.
Bit 0 DAPABORT: current AP transaction aborted if excessive number of WAIT responses returned
This bit indicates that the transaction is stalled.
0: no effect
1: transaction aborted

DP control and status register (DP_CTRL/STATR)
Address offset: 0x4
Reset value: 0x0000 0000
This register is accessible when DP_SELECTR.DPBANKSEL[3:0] = 0x0.
31

30

29

28

27

26

25

24

23

22

21

20

19

18

17

16

Res.

Res.

CDBG
PWRU
PACK

CDBG
PWRU
PREQ

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

r

r

15

14

13

12

11

10

9

8

7

6

5

4

3

2

1

0

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

WDATA
ERR

READ
OK

STICK
YERR

r

r

r

Res.

Res.

Res.

STICK ORUN
YORU DETEC
N
T
r

Bits 31:30 Reserved, must be kept at reset value.
Bit 29 CDBGPWRUPACK: debug power-up acknowledge (see description in Section 75.2.5)
0: DCLK gated
1: DCLK enabled
Bit 28 CDBGPWRUPREQ: debug power-up request
This bit controls the DCLK enable request signal.
0: requests DCLK gating
1: requests DCLK enable
Bits 27:8 Reserved, must be kept at reset value.

<!-- pagebreak -->

RM0456 Rev 6

r

RM0456

Debug support (DBG)

Bit 7 WDATAERR: write data error (read-only) in SW-DP
This bit indicates that there is a parity or framing error on the data phase of a write, or a write
accepted by the DP is then discarded without being submitted to the AP.
This bit is reset by writing one to the ABORT.WDERRCLR bit.
0: no error
1: an error occurred
Note: This bit is reserved in JTAG-DP.
Bit 6 READOK: AP read response (read-only) in SW-DP
This bit indicates the response to the last AP read access.
0: read not OK
1: read OK
Note: This bit is reserved in JTAG-DP.
Bit 5 STICKYERR: transaction error (read-only in SW-DP, read/write in JTAG-DP)
This bit indicates that an error occurred in an AP transaction.It is reset by writing 1 to the
DP_ABORTR.STKERRCLR bit (in SW-DP and JTAG-DP)
0: no error
1: an error occurred
Bits 4:2 Reserved, must be kept at reset value.
Bit 1 STICKYORUN: overrun (read-only in SW-DP, read/write in JTAG-DP).
This bit indicates that an overrun occurred (new transaction received before previous
transaction completed). This bit is only set if the ORUNDETECT bit is set. it is reset by
writing 1 to the DP_ABORTR.ORUNERRCLR bit (in SW-DP and JTAG-DP).
0: no overrun
1: an overrun occurred
Bit 0 ORUNDETECT: overrun detection mode enable.
0: disabled
1: enabled. In the event of an overrun, the STICKYORUN bit is set and subsequent
transactions are blocked until the STICKYORUN bit is cleared.

DP data link control register (DP_DLCR)
Address offset: 0x4
Reset value: 0x0000 0000
This register is accessible when DP_SELECTR.DPBANKSEL[3:0] = 0x1.
31

30

29

28

27

26

25

24

23

22

21

20

19

18

17

16

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

15

14

13

12

11

10

9

8

7

6

5

4

3

2

1

0

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

TURNROUND
[1:0]
r

WIREMODE[1:0]

r

r

r

Bits 31:10 Reserved, must be kept at reset value.
Bits 9:8 TURNROUND[1:0]: tristate period for SWDIO
0x0: 1 data bit period

RM0456 Rev 6

<!-- pagebreak -->

3631

Debug support (DBG)

RM0456

Bits 7:6 WIREMODE[1:0]: SW-DP mode
0x0: synchronous mode
Bits 5:0 Reserved, must be kept at reset value.

DP target identification register (DP_TARGETIDR)
Address offset: 0x4
Reset value: 0xXXXX 0041
This register is accessible when DP_SELECTR.DPBANKSEL[3:0] = 0x2.
31

30

29

28

27

26

25

24

23

TREVISION[3:0]

22

21

20

19

18

17

16

TPARTNO[15:4]

r

r

r

r

r

r

r

r

r

r

r

r

r

r

r

r

15

14

13

12

11

10

9

8

7

6

5

4

3

2

1

0

r

r

r

r

r

r

r

r

r

r

TPARTNO[3:0]
r

r

r

TDESIGNER[10:0]
r

Bits 31:28 TREVISION[3:0]: target revision
For STM32U5Fx/5Gx
0x1: revision Z
For STM32U59x/5Ax
0x3: revision X, W
For STM32U575/585
0x2: revision X, W, U
For STM32U535/545
0x1: revision Z, Y
Bits 27:12 TPARTNO[15:0]: target part number
0x4550: STM32U535/545
0x4760: STM32U5Fx/5Gx
0x4810: STM32U59x/5Ax
0x4820: STM32U575/585
Bits 11:1 TDESIGNER[10:0]: target designer JEDEC code
0x020: STMicroelectronics
Bit 0 Reserved, must be kept at reset value.

<!-- pagebreak -->

RM0456 Rev 6

r

Res.

RM0456

Debug support (DBG)

DP data link protocol identification register (DP_DLPIDR)
Address offset: 0x4
Reset value: 0x0000 0001
This register is accessible when DP_SELECTR.DPBANKSEL[3:0] = 0x3.
31

30

29

28

TINSTANCE[3:0]
r

r

r

27

26

25

24

23

22

21

20

19

18

17

16

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

3

2

1

0

r

15

14

13

12

11

10

9

8

7

6

5

4

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

PROTSVN[3:0]
r

r

r

r

Bits 31:28 TINSTANCE[3:0]: target instance number
this field defines the instance number for the device in a multi-drop system.
0x0: instance number 0
Bits 27:4 Reserved, must be kept at reset value.
Bits 3:0 PROTSVN[3:0]: Serial-wire debug protocol version
0x1: version 2

DP event status (DP_EVENTSTATR)
Address offset: 0x4
Reset value: 0x0000 0001
This register is accessible when DP_SELECTR.DPBANKSEL[3:0] = 0x4.
31

30

29

28

27

26

25

24

23

22

21

20

19

18

17

16

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

15

14

13

12

11

10

9

8

7

6

5

4

3

2

1

0

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

EA
r

Bits 31:1 Reserved, must be kept at reset value.
Bit 0 EA: event status flag
0: Cortex-M33 processor halted
1: Cortex-M33 processor not halted

RM0456 Rev 6

<!-- pagebreak -->

3631

Debug support (DBG)

RM0456

DP event status register (DP_RESENDR)
Address offset: 0x8
Reset value: 0x0000 0000
31

30

29

28

27

26

25

24

23

22

21

20

19

18

17

16

r

r

r

r

r

r

r

r

r

r

r

r

r

r

r

r

15

14

13

12

11

10

9

8

7

6

5

4

3

2

1

0

r

r

r

r

r

r

r

RESEND[31:16]

RESEND[15:0]
r

r

r

r

r

r

r

r

r

Bits 31:0 RESEND[31:0]: value returned by the last AP read or DP_RDBUFF read
This register is used in the event of a corrupted read transfer.

DP access port select register (DP_SELECTR)
Address offset: 0x8
Reset value: 0xXXXX XXXX
31

30

29

28

27

26

25

24

APSEL[7:0]
w

w

w

w

w

w

w

23

22

21

20

19

18

17

16

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

6

5

4

3

2

1

0

w

15

14

13

12

11

10

9

8

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

7

APBANKSEL[3:0]
w

w

w

DPBANKSEL[3:0]
w

w

w

w

Bits 31:24 APSEL[7:0]: access port select
This field selects the access port for the next transaction.
0x0: AP0 - Cortex-M33 debug access port (AHB-AP)
others: reserved
Bits 23:8 Reserved, must be kept at reset value.
Bits 7:4 APBANKSEL[3:0]: AP register bank select
This field selects the 4-word register bank on the active AP for the next transaction.
Bits 3:0 DPBANKSEL[3:0]: DP register bank select
This field selects the register at address 0x4 of the debug port.
0x0: DP_CTRL/STAT register
0x1: DP_DLCR register
0x2: DP_TARGETID register
0x3: DP_DLPIDR register
0x4: DP_EVENTSTAT register
others: reserved

<!-- pagebreak -->

RM0456 Rev 6

w

RM0456

Debug support (DBG)

DP read buffer register (DP_RDBUFFR)
Address offset: 0xC
Reset value: 0x0000 0000
31

30

29

28

27

26

25

24

23

22

21

20

19

18

17

16

r

r

r

r

r

r

r

r

r

r

r

r

r

r

r

r

15

14

13

12

11

10

9

8

7

6

5

4

3

2

1

0

r

r

r

r

r

r

r

RDBUFF[31:16]

RDBUFF[15:0]
r

r

r

r

r

r

r

r

r

Bits 31:0 RDBUFF[31:0]: value returned by the last AP read access
The value returned by an AP read access can either be obtained using a second read access
to the same address, that initiates a new transaction on the corresponding bus, or else it can
be read from this register, in which case no new AP transaction occurs.

75.3.4

Debug port register map
These registers are not on the CPU memory bus, they are only accessed through SW-DP
and JTAG-DP debug interface.
The debug port address is 2-bit wide, defined in the JTAG-DP register DPACC or SW-DP
packet request A[3:2] field.

Reset value

X X X X X X X X X X X X X X X X

RM0456 Rev 6

READOK

STICKYERR
Res.

0

Res.
Res.

DAPABORT

0

0
STICKYORUN

WDATAERR

TPARTNO[15:0]

Res.

0

Res.
0

0

0

0

Res.

DP_TARGETIDR

0

0

1

0

0

0

0
Res.

0x4(3)

TREVISION
[3:0]

0

WIREMODE
[1:0]

Reset value

0
TURNROUND
[1:0]

Res.
Res.

Res.
Res.

Res.
Res.

Res.

Res.
Res.

Res.

Res.
Res.

Res.
Res.

Res.
Res.

Res.
Res.

Res.
Res.

Res.
Res.

Res.
Res.

Res.
Res.

Res.
Res.

Res.
Res.

Res.
Res.

Res.
Res.

Res.

0

Res.

CDBGPWRUPACK

CDBGPWRUPREQ

0

Res.

DP_DLCR

Res.

0x4(2)

Res.

Reset value

Res.

DP_CTRL/STATR

Res.

0x4(1)

Res.

Reset value

1

ORUNDETECT

1

Res.

0
WDERRCLR

1

STKERRCLR

1

Res.

Res.

1

Res.

Res.

0

Res.

Res.

0

Res.

Res.

0

Res.

Res.

1

ORUNERRCLR

Res.

0

Res.

Res.

0

DESIGNER[10:0]

Res.

Res.

1

Res.

Res.

0

Res.

Res.

0

Res.

DP_ABORTR

0

Res.

0

Res.

1

Res.

1

Res.

1

Res.

1

Res.

1

MIN

0

Res.

1

Res.

0

VERSION
[3:0]

Res.

0

Res.

0

Res.

0

Res.

Reset value

PARTNO[7:0]

Res.

REVISION
[3:0]

Res.

0x0

DP_DPIDR

Res.

0x0

Res.

Offset Register name

31
30
29
28
27
26
25
24
23
22
21
20
19
18
17
16
15
14
13
12
11
10
9
8
7
6
5
4
3
2
1
0

Table 795. Debug port register map and reset values

TDESIGNER[10:0]
0

0

0

0

0

0

0

0

0

1

0

0

<!-- pagebreak -->

3631

Debug support (DBG)

RM0456

PROTSVN
[3:0]
0

0

1

Res.

Res.

EA

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.
0
Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

DP_EVENTSTATR

Res.

0

Reset value

1

x

x

x

x

x

x

DP_RDBUFFR
Reset value

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

x

x

x

x

x

x

x

x

0

0

0

0

0

0

0

0

RDBUFF[31:0]
0

0

0

0

1.

DP_SELECTR.DPBANKSEL[3:0] = 0x0.

2.

DP_SELECTR.DPBANKSEL[3:0] = 0x1.

3.

DP_SELECTR.DPBANKSEL[3:0] = 0x2.

4.

DP_SELECTR.DPBANKSEL[3:0] = 0x3.

5.

DP_SELECTR.DPBANKSEL[3:0] = 0x4.

75.4

0

DPBANKSEL
[3:0]

0

APBANKSEL
[3:0]

0

Res.

0

Res.

0

Res.

x

0

Res.

x

APSEL[7:0]

0

Res.

0

Res.

0

Res.

0

Res.

0

Res.

0

Res.

DP_SELECTR

0

Res.

0

Res.

RESEND[31:0]
0

Res.

Reset value

Res.

DP_RESENDR

Reset value
0xC

Res.

0

Res.

0

Res.

0x8

0

Res.

0x8

Reset value

Res.

DP_DLPIDR

0x4(4)

0x4(5)

TINSTANCE
[3:0]

Offset Register name

31
30
29
28
27
26
25
24
23
22
21
20
19
18
17
16
15
14
13
12
11
10
9
8
7
6
5
4
3
2
1
0

Table 795. Debug port register map and reset values (continued)

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

Access ports
There is one access port (AP) attached to the DP. It enables the access to the debug and
trace features integrated in the Cortex-M33 processor core via its internal AHB bus.

75.4.1

Access port registers
The access port is of MEM-AP type: the debug and trace component registers are mapped
in the address space of the AHB. The AP is seen by the debugger as a set of 32-bit
registers organized in banks of four registers each. Some of these registers are used to
configure or monitor the AP itself, while others are used to perform a transfer on the bus.
The AP registers are listed in Table 796.
The address of the AP registers is composed of the following fields:
•

bits [7:4]: content of APBANKSEL[3:0] in DP_SELECTR

•

bits [3:2]: content of the A(3:2) field of the APACC data register in the JTAG-DP
(see Table 791), or of the SW-DP packet request (see Table 792), depending on the
debug interface used

•

bits [1:0]: always set to 0

The content of DP_SELECTR.APSEL[3:0] defines which MEM-AP is being accessed.
The debugger can access the AP registers as follows:

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Debug support (DBG)
1.

Program APSEL[3:0] in DP_SELECTR to choose the AP, and APBANKSEL[3:0]
in DP_SELECTR to select the register bank to be accessed.

2.

Program the A(3:2) field in the APACC data register, if using JTAG, with the register
address within the bank. Program the RnW bit to select a read or write. In the case of a
write, program the DATA field with the write data. If using SWD, the A(3:2) and RnW
fields are part of the packet request word sent to the SW-DP with the APnDP bit set
(see Table 792). The write data is sent in the data phase.

The debugger can access the memory mapped debug component registers through the AP
registers (using the above AP register access procedure) as follows:
1.

Program the transaction target address in AP_TAR.

2.

Program AP_CSWR, if necessary, with the transfer parameters (AddrInc for example).

3.

Write to or read from AP_DRWR to initiate a bus transaction at the address held in
AP_TAR. Alternatively, a read or write to AP_BDnR triggers an access to TAR[31:4] + n
address, allowing up to four consecutive addresses to be accessed without changing
the address in the AP_TAR register.

For more detailed information on the MEM-AP, refer to the Arm Debug Interface Architecture
Specification [1].

AP control/status word register (AP_CSWR)
Address offset: 0x0
Reset value: 0x0100 00X0
31

30

29

28

Res.

PROT
[6]

Res.

Res.

rw
15
Res.

14
Res.

13
Res.

12
Res.

27

26

25

24

PROT[3:0]
rw

rw

rw

r

11

10

9

8

Res.

Res.

Res.

Res.

23

22

21

20

19

18

17

16

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

7

6

5

4

3

2

1

0

Res.

DBGST
ATUS
r

ADDRINC[1:0]
rw

Res.

rw

SIZE[2:0]
rw

rw

rw

Bit 31 Reserved, must be kept at reset value.
Bit 30 PROT[6]: secure transfer request
This field sets the protection attribute HPROT[6] of the bus transfer.
0: secure transfer
1: nonsecure transfer
Bits 29:28 Reserved, must be kept at reset value.
Bits 27:24 PROT[3:0]: bus transfer protection
This field sets the protection attributes HPROT[3:0] of the bus transfer.
0bXXX1: data access (bit 24 is read only)
0bXX0X: unprivilege mode
0bXX1X: privilege mode
0bX0XX: non-bufferable
0bX1XX: bufferable
0b0XXX: non-shareable, no look-up, non-modifiable
0b1XXX: shareable, look-up, modifiable
Bits 23:7 Reserved, must be kept at reset value.

RM0456 Rev 6

<!-- pagebreak -->

3631

Debug support (DBG)

RM0456

Bit 6 DBGSTATUS: device enable (DEVICEEN) status
0: AHB transfers blocked
1: AHB transfers enabled
Bits 5:4 ADDRINC[1:0]: auto-increment mode
Defines whether TAR address is automatically incremented after a transaction.
0x0: no auto-increment
0x1: address incremented by the size in bytes of the transaction (SIZE field)
other: reserved
Bit 3 Reserved, must be kept at reset value.
Bits 2:0 SIZE[2:0]: size of next memory access transaction
0x0: byte (8-bit)
0x1: halfword (16-bit)
0x2: word (32-bit)
others: reserved

AP transfer address register (AP_TAR)
Address offset: 0x04
Reset value: 0xXXXX XXXX
31

30

29

28

27

26

25

24

23

22

21

20

19

18

17

16

TA[31:16]
rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

15

14

13

12

11

10

9

8

7

6

5

4

3

2

1

0

rw

rw

rw

rw

rw

rw

rw

22

21

20

19

18

17

16

TA[15:0]
rw

rw

rw

rw

rw

rw

rw

rw

rw

Bits 31:0 TA[31:0]: address of current transfer

AP data read/write register (AP_DRWR)
Address offset: 0x0C
Reset value: 0xXXXX XXXX
31

30

29

28

27

26

25

24

23

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

15

14

13

12

11

10

9

8

7

6

5

4

3

2

1

0

rw

rw

rw

rw

rw

rw

rw

TD[31:16]

TD[15:0]
rw

rw

rw

rw

rw

rw

rw

rw

rw

Bits 31:0 TD[31:0]: data of current transfer

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Debug support (DBG)

AP banked data n register (AP_BDnR)
Address offset: 0x10 + 0x4 * n, (n = 0 to 3)
Reset value: 0x0000 0000
31

30

29

28

27

26

25

24

rw

rw

rw

rw

rw

rw

rw

rw

15

14

13

12

11

10

9

8

23

22

21

20

19

18

17

16

rw

rw

rw

rw

rw

rw

rw

rw

7

6

5

4

3

2

1

0

rw

rw

rw

rw

rw

rw

rw

TBD[31:16]

TBD[15:0]
rw

rw

rw

rw

rw

rw

rw

rw

rw

Bits 31:0 TBD[31:0]: banked data of current transfer to address TAR
TA + AP_BDnR address [3:2] + 0b00.
The auto address incrementing is not performed on AP_BD0-3R. Banked transfers are only
supported for word transfers.

AP configuration register (AP_CFGR)
Address offset: 0xF4
Reset value: 0x0000 0000
31

30

29

28

27

26

25

24

23

22

21

20

19

18

17

16

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

15

14

13

12

11

10

9

8

7

6

5

4

3

2

1

0

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

LD

LA

BE

r

r

r

Bits 31:3 Reserved, must be kept at reset value.
Bit 2 LD: large data
0: data not larger than 32-bits supported
Bit 1 LA: long address
0: Physical addresses not larger than 32-bits supported
Bit 0 BE: big endian
0: only little-endian supported

RM0456 Rev 6

<!-- pagebreak -->

3631

Debug support (DBG)

RM0456

AP base address register (AP_BASER)
Address offset: 0xF8
Reset value: 0xE00F E003
31

30

29

28

27

26

25

24

23

22

21

20

19

18

17

16

r

r

r

r

r

r

r

r

r

r

r

r

r

r

r

r

15

14

13

12

11

10

9

8

7

6

5

4

3

2

1

0

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

BASEADDR[31:16]

BASEADDR[15:12]
r

r

r

r

ENTRY
FORM
PRESE
AT
NT
r

r

Bits 31:12 BASEADDR[31:12]: base address (bits 31 to 12) of the first ROM table
The 12 LSBs are zero since the ROM table must be aligned on a 4-Kbyte boundary.
0xE00FE
Bits 11:2 Reserved, must be kept at reset value.
Bit 1 FORMAT: base-address register format
1: Arm debug interface v5
Bit 0 ENTRYPRESENT: debug components presence
Indicates that debug components are present on the access port bus.
1: debug components present

AP identification register (AP_IDR)
Address offset: 0xFC
Reset value: 0x1477 0015
31

30

29

28

27

REVISION[3:0]

26

25

24

23

22

JEDECBANK[3:0]

21

20

19

18

17

16
CLASS
[3]

JEDECCODE[6:0]

r

r

r

r

r

r

r

r

r

r

r

r

r

r

r

r

15

14

13

12

11

10

9

8

7

6

5

4

3

2

1

0

Res.

Res.

Res.

Res.

Res.

r

r

CLASS[2:0]
r

r

r

VARIANT[3:0]
r

Bits 31:28 REVISION[3:0]: revision number
0x1: r0p1
Bits 27:24 JEDECBANK[3:0]: JEDEC bank
0x4: Arm
Bits 23:17 JEDECCODE[6:0]: JEDEC code
0x3B: Arm
Bits 16:13 CLASS[3:0]:
0x8: MEM-AP
Bits 12:8 Reserved, must be kept at reset value.

<!-- pagebreak -->

RM0456 Rev 6

r

r

TYPE[3:0]
r

r

r

RM0456

Debug support (DBG)

Bits 7:4 VARIANT[3:0]:
0x1: Cortex-M33
Bits 3:0 TYPE[3:0]:
0x5: AHB5

75.4.2

Access port register map
These registers are not on the CPU memory bus, they are only accessed through SW-DP
and JTAG-DP debug interfaces.
The access port address is 8-bit wide, defined by DP_SELECTR.APBANKSEL[3:0] field
and by JTAG-DP register DPACC or SW-DP packet request A[3:2] field.

Reset value

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

AP_BD1R

AP_BD2R
0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

AP_BD3R

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

TBD[31:0]
0

0

0

0

0

1

0

1

0

0

0

0

1

1

JEDECCODE[6:0]

0

0

1

1

1

0

1

1

RM0456 Rev 6

1

1

0

CLASS[3:0]

1

1

0

1

0

0

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.
1

BE

0

0

0
ENTRYPRESENT

0

LA

0

0

FORMAT

0

Res.

0

Res.

0

Res.

1

JEDECBANK
[3:0]

AP_IDR

Reset value

1

REVISION
[3:0]

1

Res.

BASEADDR[31:12]

Res.

AP_BASER

LD

Res.

Res.

Res.

Res.

Reserved
Res.

Reserved
AP_CFGR

0

Res.

Reset value

0

TBD[31:0]

Res.

Reset value

0

TBD[31:0]

Res.

Reset value

0

Res.

0

Res.

0

Res.

0

Res.

0

Res.

0

Res.

0

Res.

0

Res.

0

Res.

0

Res.

0

Res.

TBD[31:0]
0

Res.

AP_BD0R
Reset value

Reset value

0xFC

SIZE[2:0]

Res.

ADDRINC[1:0]

Res.

DBGSTATUS

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

TD[31:0]
X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X

Reset value

0xF8

0

AP_DRWR

Res.

0xF4

0

Reset value

Res.

0x20 to
0xF0

0

Reserved

Res.

0x1C

X X X

X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X

Res.

0x18

1

Reserved

Res.

0x14

0

Reset value

Res.

0x10

0

TA[31:0]

Res.

0x0C

0

Res.

0x08

0

PROT[3:0]

AP_TAR

Res.

0x04

Res.

Res.

AP_CSWR

Res.

0x00

PROT[6]

Offset Register name

31
30
29
28
27
26
25
24
23
22
21
20
19
18
17
16
15
14
13
12
11
10
9
8
7
6
5
4
3
2
1
0

Table 796. Access port register map and reset values

1

1

VARIANT[3:0]

TYPE[3:0]

0

0

0

0

1

1

0

1

<!-- pagebreak -->

3631

Debug support (DBG)

75.5

RM0456

ROM tables
The ROM table is a CoreSight component that contains the base addresses of all the
CoreSight debug components accessible via the AHB-AP. These tables allow a debugger
to discover the topology of the CoreSight system automatically.
There are two ROM tables in the CPU sub-system. The MCU ROM table is pointed to by the
base register in the AHB-AP. It contains the base-address pointer for the processor ROM
table and for the TPIU registers, as well as for the MCU debug unit.
The MCU ROM table (see the table below) occupies a 4-Kbyte, 32-bit wide chunk of
address space, from 0xE00F E000 to 0xE00F EFFC.
Table 797. MCU ROM table

Address
in ROM table

Component
name

Component
base address

Component
Size
address offset (Kbytes)

0xE00F E000

Processor ROM table

0xE00F F000

0x0000 1000

4

0x0000 1003

0xE00F E004

TPIU

0xE004 0000

0xFFF4 2000

4

0xFFF4 2003

0xE00F E008

DBGMCU

0xE004 4000

0xFFF4 6000

4

0xFFF4 6003

0xE00F E00C

Reserved

-

-

-

0x1FF0 2002

0xE00F E010

Top of table

-

-

-

0x0000 0000

0xE00F E014 to
0xE00F EFC8

Reserved

-

-

-

0x0000 0000

0xE00F EFCC to
0xE00F EFFC

ROM table registers

-

-

-

See Table 799

Entry

The processor ROM table contains the base-address pointer for the system control space
(SCS) registers, that allow the debugger to identify the CPU core, as well as for the BPU,
DWT, ITM, ETM and CTI.
The processor ROM table (see the table below) occupies a 4-Kbyte, 32-bit wide chunk of
address space, from 0xE00F F000 to 0xE00F FFFC.
Table 798. Processor ROM table
Address
in ROM table

Component
name

Component
base address

Component
Size
address offset (Kbytes)

0xE00F F000

SCS

0xE000 E000

0xFFF0 F000

4

0xFFF0 F003

0xE00F F004

DWT

0xE000 1000

0xFFF0 2000

4

0xFFF0 2003

0xE00F F008

BPU

0xE000 2000

0xFFF0 3000

4

0xFFF0 3003

0xE00F F00C

ITM

0xE000 0000

0xFFF0 1000

4

0xFFF0 1003

0xE00F F010

Reserved

-

-

-

0xFFF4 1002

0xE00F F014

ETM

0xE004 1000

0xFFF4 2000

4

0xFFF4 2003

0xE00F F018

CTI

0xE004 2000

0xFFF4 3000

4

0xFFF4 3003

0xE00F F01C

Reserved

-

-

-

0xFFF4 4002

0xE00F F020

Top of table

-

-

-

0x0000 0000

<!-- pagebreak -->

RM0456 Rev 6

Entry

RM0456

Debug support (DBG)
Table 798. Processor ROM table (continued)

Address
in ROM table

Component
name

Component
base address

Component
Size
address offset (Kbytes)

0xE00F F024 to
0xE00F FFC8

Reserved

-

-

-

0x0000 0000

0xE00F FFCC to
0xE00F FFFC

ROM table registers

-

-

-

See Table 800

Entry

The topology for the CoreSight components in the Cortex -M33 is shown in the figure below.
Figure 958. CoreSight topology

AHB-AP
Base
register 0xE00FE000
(0xF8)

Processor ROM table
@0xE00FF000
0x000 Offset: 0xFFF0F000

0x000

Offset: 0x00001000

0x004

Offset: 0xFFF42000

0x008 Offset: 0xFFF46000

0xFD0

0xFFC

0x000

Register file base

0xFD0

PIDR4

0xFFC

CIDR3

0x004 Offset: 0xFFF02000
0x008 Offset: 0xFFF03000

MCU ROM table
@0xE00FE000

System control space (SCS)
@0xE000E000

0x00C Offset: 0xFFF01000
0x010

Reserved

0x014 Offset: 0xFFF42000
0x018 Offset: 0xFFF43000

Top of table

0x01C

Top of table

PIDR4

0xFD0

PIDR4

0x000

Register file base

CIDR3

0xFFC

CIDR3

0xFD0

PIDR4

Trace port (TPIU)
@0xE0040000

0xFFC

CIDR3

Breakpoint unit (BPU)
@0xE0002000

0x000 Register file base

0xFD0

0xFFC

Register file base

0xFD0

PIDR4

0xFFC

CIDR3

Instrumentation trace
(ITM)
@0xE0000000

Embedded trace (ETM)
@0xE0041000
0xFD0
0x000

Register file base

0xFD0

PIDR4

0xFFC

CIDR3

CIDR3

0xFFC

0x000 Register file base

0xFFC

0x000

0x000

PIDR4

MCU debug (DBGMCU)
@0xE0044000

0xFD0

Data watchpoint/trace
(DWT)
@0xE0001000

Register file base

PIDR4

CIDR3

Cross trigger (CTI)
@0xE0042000
0x000

Register file base

0xFD0

PIDR4

0xFFC

CIDR3

PIDR4

CIDR3

MSv62650V2

RM0456 Rev 6

<!-- pagebreak -->

3631

Debug support (DBG)

75.5.1

RM0456

MCU ROM table registers
MCU ROM memory type register (MCUROM_MEMTYPER)
Address offset: 0xFCC
Reset value: 0x0000 0001

31

30

29

28

27

26

25

24

23

22

21

20

19

18

17

16

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

15

14

13

12

11

10

9

8

7

6

5

4

3

2

1

0

Res.

SYSME
M

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

r

Bits 31:1 Reserved, must be kept at reset value.
Bit 0 SYSMEM: system memory
0x1: system memory present on this bus

MCU ROM CoreSight peripheral identity register 4 (MCUROM_PIDR4)
Address offset: 0xFD0
Reset value: 0x0000 0000
31

30

29

28

27

26

25

24

23

22

21

20

19

18

17

16

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

7

6

5

4

3

2

1

0

15

14

13

12

11

10

9

8

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

SIZE[3:0]
r

r

r

JEP106CON[3:0]
r

r

r

r

r

Bits 31:8 Reserved, must be kept at reset value.
Bits 7:4 SIZE[3:0]: register file size
0x0: The register file occupies a single 4-Kbyte region.
Bits 3:0 JEP106CON[3:0]: JEP106 continuation code
0x0: STMicroelectronics JEDEC continuation code

MCU ROM CoreSight peripheral identity register 0 (MCUROM_PIDR0)
Address offset: 0xFE0
Reset value: 0x0000 00XX
31

30

29

28

27

26

25

24

23

22

21

20

19

18

17

16

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

7

6

5

4

3

2

1

0

r

r

r

15

14

13

12

11

10

9

8

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

PARTNUM[7:0]
r

<!-- pagebreak -->

RM0456 Rev 6

r

r

r

r

RM0456

Debug support (DBG)

Bits 31:8 Reserved, must be kept at reset value.
Bits 7:0 PARTNUM[7:0]: part number bits [7:0]
0x55: STM32U535/545
0x76: STM32U5Fx/5Gx
0x81: STM32U59x/5Ax
0x82: STM32U575/585

MCU ROM CoreSight peripheral identity register 1(MCUROM_PIDR1)
Address offset: 0xFE4
Reset value: 0x0000 0004
31

30

29

28

27

26

25

24

23

22

21

20

19

18

17

16

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

15

14

13

12

11

10

9

8

7

6

5

4

3

2

1

0

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

r

r

JEP106ID[3:0]
r

r

r

PARTNUM[11:8]
r

r

r

Bits 31:8 Reserved, must be kept at reset value.
Bits 7:4 JEP106ID[3:0]: JEP106 identity code bits [3:0]
0x0: STMicroelectronics JEDEC code
Bits 3:0 PARTNUM[11:8]: part number bits [11:8]
0x4: STM32U5 series

MCU ROM CoreSight peripheral identity register 2 (MCUROM_PIDR2)
Address offset: 0xFE8
Reset value: 0x0000 000A
31

30

29

28

27

26

25

24

23

22

21

20

19

18

17

16

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

7

6

5

4

3

2

1

0

15

14

13

12

11

10

9

8

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

REVISION[3:0]
r

r

r

JEDEC
r

r

JEP106ID[6:4]
r

r

r

Bits 31:8 Reserved, must be kept at reset value.
Bits 7:4 REVISION[3:0]: component revision number
0x0: rev r0p0
Bit 3 JEDEC: JEDEC assigned value
1: designer identification specified by JEDEC
Bits 2:0 JEP106ID[6:4]: JEP106 identity code bits [6:4]
0x2: STMicroelectronics JEDEC code

RM0456 Rev 6

<!-- pagebreak -->

3631

Debug support (DBG)

RM0456

MCU ROM CoreSight peripheral identity register 3 (MCUROM_PIDR3)
Address offset: 0xFEC
Reset value: 0x0000 0000
31

30

29

28

27

26

25

24

23

22

21

20

19

18

17

16

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

7

6

5

4

3

2

1

0

15

14

13

12

11

10

9

8

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

REVAND[3:0]
r

r

r

CMOD[3:0]
r

r

r

r

r

Bits 31:8 Reserved, must be kept at reset value.
Bits 7:4 REVAND[3:0]: metal fix version
0x0: No metal fix
Bits 3:0 CMOD[3:0]: customer modified
0x0: No customer modifications

MCU ROM CoreSight component identity register 0 (MCUROM_CIDR0)
Address offset: 0xFF0
Reset value: 0x0000 000D
31

30

29

28

27

26

25

24

23

22

21

20

19

18

17

16

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

15

14

13

12

11

10

9

8

7

6

5

4

3

2

1

0

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.
r

r

r

r

r

r

PREAMBLE[7:0]
r

r

Bits 31:8 Reserved, must be kept at reset value.
Bits 7:0 PREAMBLE[7:0]: component identification bits [7:0]
0x0D: Common identification value

MCU ROM CoreSight peripheral identity register 1 (MCUROM_CIDR1)
Address offset: 0xFF4
Reset value: 0x0000 0010
31

30

29

28

27

26

25

24

23

22

21

20

19

18

17

16

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

7

6

5

4

3

2

1

0

15

14

13

12

11

10

9

8

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

CLASS[3:0]
r

Bits 31:8 Reserved, must be kept at reset value.

<!-- pagebreak -->

RM0456 Rev 6

r

r

PREAMBLE[11:8]
r

r

r

r

r

RM0456

Debug support (DBG)

Bits 7:4 CLASS[3:0]: Component identification bits [15:12] - component class
0x1: ROM table component
Bits 3:0 PREAMBLE[11:8]: Component identification bits [11:8]
0x0: Common identification value

MCU ROM CoreSight component identity register 2 (MCUROM_CIDR2)
Address offset: 0xFF8
Reset value: 0x0000 0005
31

30

29

28

27

26

25

24

23

22

21

20

19

18

17

16

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

7

6

5

4

3

2

1

0

r

r

r

15

14

13

12

11

10

9

8

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

PREAMBLE[19:12]
r

r

r

r

r

Bits 31:8 Reserved, must be kept at reset value.
Bits 7:0 PREAMBLE[19:12]: component identification bits [23:16]
0x05: common identification value

MCU ROM CoreSight component identity register 3 (MCUROM_CIDR3)
Address offset: 0xFFC
Reset value: 0x0000 00B1
31

30

29

28

27

26

25

24

23

22

21

20

19

18

17

16

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

7

6

5

4

3

2

1

0

r

r

r

15

14

13

12

11

10

9

8

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

PREAMBLE[27:20]
r

r

r

r

r

Bits 31:8 Reserved, must be kept at reset value.
Bits 7:0 PREAMBLE[27:20]: Component identification bits [31:24]
0xB1: Common identification value

75.5.2

MCU ROM table register map

Reset value

Res.

SYSMEM

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

MCUROM_
MEMTYPER

Res.

0xFCC

Res.

Offset Register name

31
30
29
28
27
26
25
24
23
22
21
20
19
18
17
16
15
14
13
12
11
10
9
8
7
6
5
4
3
2
1
0

Table 799. MCU ROM table register map and reset values

1

RM0456 Rev 6

<!-- pagebreak -->

3631

Debug support (DBG)

RM0456

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Reset value

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

MCUROM_PIDR3

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

MCUROM_CIDR1

Reset value

0

0

1

0

1

0

0

JEP106ID
[6:4]

0

0

1

0

0

0

0

0

0

0

0

0

0

0

1

1

0

1

CLASS[3:0]

PREAMBLE
[11:8]

0

0

0

0

1

0

0

0

PREAMBLE[19:12]

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

MCUROM_CIDR3

0

PREAMBLE[7:0]

0
Res.

Reset value
0xFFC

0

REVAND[3:0] CMOD[3:0]

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

0xFF8

Res.

Reset value
MCUROM_CIDR2

0

0
Res.

Reset value
0xFF4

0

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

0xFF0

0

PARTNUM
[11:8]

0
Res.

Reset value
MCUROM_CIDR0

0

JEP106ID
[3:0]

0
Res.

Reset value
0xFEC

0

PARTNUM[7:0]

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

MCUROM_PIDR2

Res.

Reset value

0xFE8

0

JEDEC

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

MCUROM_PIDR1

0

X X X X X X X X
Res.

Reset value
0xFE4

0

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

MCUROM_PIDR0

0

Reserved
Res.

Reserved
Res.

0xFE0

0

Res.

0xFD4FDC

JEP106CON
[3:0]

SIZE[3:0]

REVISION
[3:0]

MCUROM_PIDR4

Res.

0xFD0

Res.

Offset Register name

31
30
29
28
27
26
25
24
23
22
21
20
19
18
17
16
15
14
13
12
11
10
9
8
7
6
5
4
3
2
1
0

Table 799. MCU ROM table register map and reset values (continued)

0

0

0

0

1

0

1

PREAMBLE[27:20]
1

0

1

1

0

0

0

1

Refer to Table 797: MCU ROM table for register boundary addresses.

75.5.3

Processor ROM table registers
CPU ROM memory type register (CPUROM_MEMTYPER)
Address offset: 0xFCC
Reset value: 0x0000 0001

31

30

29

28

27

26

25

24

23

22

21

20

19

18

17

16

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

15

14

13

12

11

10

9

8

7

6

5

4

3

2

1

0

Res.

SYSME
M

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

r

Bits 31:1 Reserved, must be kept at reset value.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Debug support (DBG)

Bit 0 SYSMEM: system memory
1: system memory present on this bus

CPU ROM CoreSight peripheral identity register 4 (CPUROM_PIDR4)
Address offset: 0xFD0
Reset value: 0x0000 0004
31

30

29

28

27

26

25

24

23

22

21

20

19

18

17

16

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

7

6

5

4

3

2

1

0

15

14

13

12

11

10

9

8

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

SIZE[3:0]
r

r

r

JEP106CON[3:0]
r

r

r

r

r

Bits 31:8 Reserved, must be kept at reset value.
Bits 7:4 SIZE[3:0]: register file size
0x0: The register file occupies a single 4-Kbyte region.
Bits 3:0 JEP106CON[3:0]: JEP106 continuation code
0x4: Arm JEDEC continuation code

CPU ROM CoreSight peripheral identity register 0 (CPUROM_PIDR0)
Address offset: 0xFE0
Reset value: 0x0000 00C9
31

30

29

28

27

26

25

24

23

22

21

20

19

18

17

16

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

7

6

5

4

3

2

1

0

r

r

r

15

14

13

12

11

10

9

8

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

PARTNUM[7:0]
r

r

r

r

r

Bits 31:8 Reserved, must be kept at reset value.
Bits 7:0 PARTNUM[7:0]: Part number bits [7:0]
0xC9: Cortex-M33

RM0456 Rev 6

<!-- pagebreak -->

3631

Debug support (DBG)

RM0456

CPU ROM CoreSight peripheral identity register 1 (CPUROM_PIDR1)
Address offset: 0xFE4
Reset value: 0x0000 00B4
31

30

29

28

27

26

25

24

23

22

21

20

19

18

17

16

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

7

6

5

4

3

2

1

0

15

14

13

12

11

10

9

8

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

JEP106ID[3:0]
r

r

r

PARTNUM[11:8]
r

r

r

r

r

Bits 31:8 Reserved, must be kept at reset value.
Bits 7:4 JEP106ID[3:0]: JEP106 identity code bits [3:0]
0xB: Arm JEDEC code
Bits 3:0 PARTNUM[11:8]: part number bits [11:8]
0x4: Cortex-M33

CPU ROM CoreSight peripheral identity register 2 (CPUROM_PIDR2)
Address offset: 0xFE8
Reset value: 0x0000 000B
31

30

29

28

27

26

25

24

23

22

21

20

19

18

17

16

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

15

14

13

12

11

10

9

8

7

6

5

4

3

2

1

0

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

REVISION[3:0]
r

Bits 31:8 Reserved, must be kept at reset value.
Bits 7:4 REVISION[3:0]: component revision number
0x0: rev r0p0
Bit 3 JEDEC: JEDEC assigned value
1: Designer ID specified by JEDEC
Bits 2:0 JEP106ID[6:4]: JEP106 identity code bits [6:4]
0x3: Arm JEDEC code

<!-- pagebreak -->

RM0456 Rev 6

r

r

JEDEC
r

r

JEP106ID[6:4]
r

r

r

RM0456

Debug support (DBG)

CPU ROM CoreSight peripheral identity register 3 (CPUROM_PIDR3)
Address offset: 0xFEC
Reset value: 0x0000 0000
31

30

29

28

27

26

25

24

23

22

21

20

19

18

17

16

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

7

6

5

4

3

2

1

0

15

14

13

12

11

10

9

8

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

REVAND[3:0]
r

r

r

CMOD[3:0]
r

r

r

r

r

Bits 31:8 Reserved, must be kept at reset value.
Bits 7:4 REVAND[3:0]: metal fix version
0x0: No metal fix
Bits 3:0 CMOD[3:0]: customer modified
0x0: no customer modifications

CPU ROM CoreSight component identity register 0 (CPUROM_CIDR0)
Address offset: 0xFF0
Reset value: 0x0000 000D
31

30

29

28

27

26

25

24

23

22

21

20

19

18

17

16

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

15

14

13

12

11

10

9

8

7

6

5

4

3

2

1

0

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.
r

r

r

r

r

r

PREAMBLE[7:0]
r

r

Bits 31:8 Reserved, must be kept at reset value.
Bits 7:0 PREAMBLE[7:0]: Component identification bits [7:0]
0x0D: Common identification value

CPU ROM CoreSight peripheral identity register 1 (CPUROM_CIDR1)
Address offset: 0xFF4
Reset value: 0x0000 0010
31

30

29

28

27

26

25

24

23

22

21

20

19

18

17

16

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

7

6

5

4

3

2

1

0

15

14

13

12

11

10

9

8

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

CLASS[3:0]
r

r

r

PREAMBLE[11:8]
r

r

r

r

r

Bits 31:8 Reserved, must be kept at reset value.

RM0456 Rev 6

<!-- pagebreak -->

3631

Debug support (DBG)

RM0456

Bits 7:4 CLASS[3:0]: Component identification bits [15:12] - component class
0x1: ROM table component
Bits 3:0 PREAMBLE[11:8]: Component identification bits [11:8]
0x0: Common identification value

CPU ROM CoreSight component identity register 2 (CPUROM_CIDR2)
Address offset: 0xFF8
Reset value: 0x0000 0005
31

30

29

28

27

26

25

24

23

22

21

20

19

18

17

16

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

7

6

5

4

3

2

1

0

r

r

r

15

14

13

12

11

10

9

8

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

PREAMBLE[19:12]
r

r

r

r

r

Bits 31:8 Reserved, must be kept at reset value.
Bits 7:0 PREAMBLE[19:12]: component identification bits [23:16]
0x05: common identification value

CPU ROM CoreSight component identity register 3 (CPUROM_CIDR3)
Address offset: 0xFFC
Reset value: 0x0000 00B1
31

30

29

28

27

26

25

24

23

22

21

20

19

18

17

16

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

7

6

5

4

3

2

1

0

r

r

r

15

14

13

12

11

10

9

8

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

PREAMBLE[27:20]
r

r

r

r

r

Bits 31:8 Reserved, must be kept at reset value.
Bits 7:0 PREAMBLE[27:20]: component identification bits [31:24]
0xB1: common identification value

75.5.4

Processor ROM table register map

Reset value

<!-- pagebreak -->

Res.

SYSMEM

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

CPUROM_
MEMTYPER

Res.

0xFCC

Res.

Offset Register name

31
30
29
28
27
26
25
24
23
22
21
20
19
18
17
16
15
14
13
12
11
10
9
8
7
6
5
4
3
2
1
0

Table 800. CPU ROM table register map and reset values

1

RM0456 Rev 6

RM0456

Debug support (DBG)

Res.
Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

CPUROM_PIDR1

Res.

Reset value
0xFE4

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

CPUROM_PIDR2

Res.

Reset value

0xFE8

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

CPUROM_PIDR3

Res.

Reset value
0xFEC

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

CPUROM_CIDR1

Res.

Reset value
0xFF4

Reset value

1

0

0

0

1

0

0

0

1

PARTNUM[7:0]
0

0

1

0

JEP106ID
[3:0]

PARTNUM
[11:8]

1

1

0

1

REVISION
[3:0]

JEP106ID
[6:4]

0

1

0

0

0

1

0

0

0

1

0

1

REVAND[3:0] CMOD[3:0]
0

0

0

0

0

0

0

0

0

0

0

1

PREAMBLE[7:0]
0

0

1

1

CLASS[3:0]

PREAMBLE
[11:8]

0

0

0

0

0

0

1

0

0

0

PREAMBLE[19:12]

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

CPUROM_CIDR3

Res.

Reset value
0xFFC

1

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

CPUROM_CIDR2

Res.

Reset value
0xFF8

0

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

CPUROM_CIDR0

Res.

Reset value
0xFF0

0
Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

CPUROM_PIDR0

Res.

0xFE0

Res.

Reset value

JEP106CON
[3:0]

SIZE[3:0]

JEDEC

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

CPUROM_PIDR4

Reserved
Res.

0xFD0

Reserved
Res.

0xFD4FDC

Res.

Offset Register name

31
30
29
28
27
26
25
24
23
22
21
20
19
18
17
16
15
14
13
12
11
10
9
8
7
6
5
4
3
2
1
0

Table 800. CPU ROM table register map and reset values (continued)

0

0

0

1

0

1

PREAMBLE[27:20]
1

0

1

1

0

0

0

1

Refer to Table 798: Processor ROM table for register boundary addresses.

75.6

Data watchpoint and trace unit (DWT)
The DWT provides four comparators that can be used as one of the following:
•

watchpoint

•

ETM trigger

•

PC sampling trigger

•

data address sampling trigger

•

data comparator (COMP 1 only)

•

clock cycle counter comparator (COMP 0 only)

It also contains counters for:
•

clock cycles

•

folded instructions

•

load store unit (LSU) operations

•

sleep cycles

•

number of cycles per instruction

RM0456 Rev 6

<!-- pagebreak -->

3631

Debug support (DBG)
•

RM0456

interrupt overhead

A DWT comparator compares the value held in its DWT_COMPxR with one of the following:
•

a data address

•

an instruction address

•

a data value

•

the cycle-count value, for COMP 0 only

For address matching, the comparator can use a mask, so it matches a range of addresses.
On a successful match, the comparator generates one of the following:
•

one or more DWT data trace packets, containing one or more of:
–

the address of the instruction that caused a data access

–

an address offset, bits[15:0] of the data access address

–

the matched data value

•

a watchpoint debug event, on either the PC value or the accessed data address

•

a CMPMATCH[N] event, that signals the match outside the DWT unit

A watchpoint debug event either generates a DebugMonitor exception, or causes the
processor to halt execution and enter debug state.
For more details on how to use the DWT, refer to the Armv8-M Architecture Reference
Manual [3].

75.6.1

DWT registers
The DWT registers are located at address range 0xE000 1000 to 0xE000 1FFC.

DWT control register (DWT_CTRLR)
Address offset: 0x000
Reset value: 0x4000 0000
31

30

29

28

27

26

25

24

23

22

21

20

19

18

17

16

SLEEP
NOTR NOEXT NOCY NOPRF CYCDI CYCEV FOLDE LSUEV
EXCEV CPIEV EXCTR
EVTEN
CPKT
TRIG
CCNT
CNT
SS
TENA VTENA TENA
TENA TENA CENA
A

NUMCOMP[3:0]
r

r

r

r

r

r

r

r

rw

rw

rw

rw

rw

rw

rw

rw

15

14

13

12

11

10

9

8

7

6

5

4

3

2

1

0

Res.

Res.

Res.

PCSA
MPLEN
A
rw

SYNCTAP[1:0]
rw

rw

CYCTA
P
rw

POSTINIT[3:0]
rw

rw

rw

rw

rw

rw

Bits 31:28 NUMCOMP[3:0]: number of comparators implemented (read only)
0x4: four comparators
Bit 27 NOTRCPKT: trace sampling and exception tracing support (read only)
0: supported
Bit 26 NOEXTTRIG: external match signal, CMPMATCH support (read only)
0: supported

<!-- pagebreak -->

RM0456 Rev 6

CYCC
NTENA

POSTRESET[3:0]
rw

rw

rw

RM0456

Debug support (DBG)

Bit 25 NOCYCCNT: cycle counter support (read only)
0: supported
Bit 24 NOPRFCNT: profiling counter support (read only)
0: supported
Bit 23 CYCDISS: cycle counter disabled secure.
Controls whether the cycle counter is disabled in secure mode.
0: no effect
1: disable incrementing of the cycle counter when the processor is in secure state
Bit 22 CYCEVTENA: enable for POSTCNT underflow event counter packet generation
0: disabled
1: enabled
Bit 21 FOLDEVTENA: enable for folded instruction counter overflow event generation
0: disabled
1: enabled
Bit 20 LSUEVTENA: enable for LSU counter overflow event generation
0: disabled
1: enabled
Bit 19 SLEEPEVTENA: enable for sleep counter overflow event generation
0: disabled
1: enabled
Bit 18 EXCEVTENA: enable for exception overhead counter overflow event generation
0: disabled
1: enabled
Bit 17 CPIEVTENA: enable for CPI counter overflow event generation
0: disabled
1: enabled
Bit 16 EXCTRCENA: enable for exception trace generation
0: disabled
1: enabled
Bits 15:13 Reserved, must be kept at reset value.
Bit 12 PCSAMPLENA: enable for POSTCNT counter to be used as a timer for periodic PC sample
packet generation
0: disabled
1: enabled
Bits 11:10 SYNCTAP[1:0]: position of the synchronization packet counter tap on the CYCCNT counter
This field determines the synchronization packet rate.
00: disabled, no synchronization packets
01: Tap at CYCCNT[24]
10: Tap at CYCCNT[26]
11: Tap at CYCCNT[28]
Bit 9 CYCTAP: Selects the position of the POSTCNT tap on the CYCCNT counter.
0: Tap at CYCCNT[6]
1: Tap at CYCCNT[10]

RM0456 Rev 6

<!-- pagebreak -->

3631

Debug support (DBG)

RM0456

Bits 8:5 POSTINIT[3:0]: initial value of the POSTCNT counter
Writes to this field are ignored if POSTCNT counter is enabled. CYCEVTENA or
PCSAMPLENA bits must be reset prior to writing POSTINIT.
Bits 4:1 POSTRESET[3:0]: reload value of the POSTCNT counter
Bit 0 CYCCNTENA: enable CYCCNT counter
0: disabled
1: enabled

DWT cycle count register (DWT_CYCCNTR)
Address offset: 0x004
Reset value: 0x0000 0000
31

30

29

28

27

26

25

24

23

22

21

20

19

18

17

16

r

r

r

r

r

r

r

r

r

r

r

r

r

r

r

r

15

14

13

12

11

10

9

8

7

6

5

4

3

2

1

0

r

r

r

r

r

r

r

CYCCNT[31:16]

CYCCNT[15:0]
r

r

r

r

r

r

r

r

r

Bits 31:0 CYCCNT[31:0]: processor clock-cycle counter

DWT CPI count register (DWT_CPICNTR)
Address offset: 0x008
Reset value: 0xXXXX XXXX
31

30

29

28

27

26

25

24

23

22

21

20

19

18

17

16

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

15

14

13

12

11

10

9

8

7

6

5

4

3

2

1

0

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.
rw

rw

rw

rw

rw

rw

CPICNT[7:0]
rw

rw

Bits 31:8 Reserved, must be kept at reset value.
Bits 7:0 CPICNT[7:0]: CPI counter
Counts additional cycles required to execute multi-cycle instructions, except those recorded
by DWT_LSUCNTR, and counts any instruction fetch stalls.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Debug support (DBG)

DWT exception count register (DWT_EXCCNTR)
Address offset: 0x00C
Reset value: 0xXXXX XXXX
31

30

29

28

27

26

25

24

23

22

21

20

19

18

17

16

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

7

6

5

4

3

2

1

0

rw

rw

rw

15

14

13

12

11

10

9

8

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

EXCCNT[7:0]
rw

rw

rw

rw

rw

Bits 31:8 Reserved, must be kept at reset value.
Bits 7:0 EXCCNT[7:0]: exception overhead cycle counter
Counts the number of cycles spent in exception processing.

DWT sleep count register (DWT_SLPCNTR)
Address offset: 0x010
Reset value: 0xXXXX XXXX
31

30

29

28

27

26

25

24

23

22

21

20

19

18

17

16

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

7

6

5

4

3

2

1

0

rw

rw

rw

15

14

13

12

11

10

9

8

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

SLEEPCNT[7:0]
rw

rw

rw

rw

rw

Bits 31:8 Reserved, must be kept at reset value.
Bits 7:0 SLEEPCNT[7:0]: sleep cycle counter
Counts the number of cycles spent in Sleep mode (WFI, WFE, sleep-on-exit).

DWT LSU count register (DWT_LSUCNTR)
Address offset: 0x014
Reset value: 0xXXXX XXXX
31

30

29

28

27

26

25

24

23

22

21

20

19

18

17

16

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

7

6

5

4

3

2

1

0

rw

rw

rw

15

14

13

12

11

10

9

8

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

LSUCNT[7:0]
rw

rw

rw

rw

rw

Bits 31:8 Reserved, must be kept at reset value.
Bits 7:0 LSUCNT[7:0]: load store counter
Counts additional cycles required to execute load and store instructions.

RM0456 Rev 6

<!-- pagebreak -->

3631

Debug support (DBG)

RM0456

DWT fold count register (DWT_FOLDCNTR)
Address offset: 0x018
Reset value: 0xXXXX XXXX
31

30

29

28

27

26

25

24

23

22

21

20

19

18

17

16

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

7

6

5

4

3

2

1

0

rw

rw

rw

rw

19

18

17

16

15

14

13

12

11

10

9

8

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

FOLDCNT[7:0]
rw

rw

rw

rw

Bits 31:8 Reserved, must be kept at reset value.
Bits 7:0 FOLDCNT[7:0]: folded instruction counter
Increments on each instruction that takes 0 cycles.

DWT program counter sample register (DWT_PCSR)
Address offset: 0x01C
Reset value: 0xXXXX XXXX
31

30

29

28

27

26

25

24

23

22

21

20

EIASAMPLE[31:16]
r

r

r

r

r

r

r

r

r

r

r

r

r

r

r

r

15

14

13

12

11

10

9

8

7

6

5

4

3

2

1

0

r

r

r

r

r

r

r

r

r

r

r

r

r

r

19

18

17

16

EIASAMPLE[15:0]
r

r

Bits 31:0 EIASAMPLE[31:0]: executed instruction address sample value.
Samples the current value of the program counter.

DWT comparator x register (DWT_COMPxR)
Address offset: 0x020 + 0x010 * x, (x = 0 to 3)
Reset value: 0xXXXX XXXX
31

30

29

28

27

26

25

24

23

22

21

20

COMP[31:16]
r

r

r

r

r

r

r

r

r

r

r

r

r

r

r

r

15

14

13

12

11

10

9

8

7

6

5

4

3

2

1

0

r

r

r

r

r

r

r

r

r

r

r

r

r

r

COMP[15:0]
r

r

Bits 31:0 COMP[31:0]: reference value for comparison

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Debug support (DBG)

DWT function register 0 (DWT_FUNCTR0)
Address offset: 0x028
Reset value: 0x5800 0000
31

30

29

28

27

ID[4:0]
r

r

r

r

r

15

14

13

12

11

Res.

Res.

Res.

Res.

26

25

24

23

22

21

20

19

18

17

16

Res.

Res.

MATCH
ED

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

10

9

8

7

6

5

4

3

2

1

0

Res.

Res.

Res.

Res.

r

DATAVSIZE[1:0]
rw

rw

ACTION[1:0]
rw

rw

MATCH[3:0]
rw

rw

rw

rw

Bits 31:27 ID[4:0]: capability identification
Identifies the capability for match for comparator 0.
0b01011: cycle counter, instruction address, data address, and data address with value
Bits 26:25 Reserved, must be kept at reset value.
Bit 24 MATCHED: comparator match
Indicates if a comparator match has occurred since the register was last read.
0: no match
1: a match occurred.
Bits 23:12 Reserved, must be kept at reset value.
Bits 11:10 DATAVSIZE[1:0]: data value size
Defines the size of the object being watched for by data value and data address comparators.
0x0: 1 byte
0x1: 2 bytes
0x2: 4 bytes
0x3: reserved
Bits 9:6 Reserved, must be kept at reset value.
Bits 5:4 ACTION[1:0]: action on match
0x0: trigger only
0x1: generate debug event
0x2: For a cycle counter, instruction address, data address, data value, or linked data value
comparator, generate a data trace match packet. For a data address With value comparator,
generate a data trace data value packet.
0x3: For a data address limit comparator, generate a data trace data address packet.
For a cycle counter, instruction address limit, or data address comparator, generate a data
trace PC value packet. For a data address with value comparator, generate both a data trace
PC value packet and a data trace data value packet.
Bits 3:0 MATCH[3:0]: match type
Controls the type of match generated by comparator 0.
For possible values of this field, refer to [3].

RM0456 Rev 6

<!-- pagebreak -->

3631

Debug support (DBG)

RM0456

DWT function register 1 (DWT_FUNCTR1)
Address offset: 0x038
Reset value: 0xD000 0000
31

30

29

28

27

ID[4:0]
r

r

r

r

r

15

14

13

12

11

Res.

Res.

Res.

Res.

26

25

24

23

22

21

20

19

18

17

16

Res.

Res.

MATCH
ED

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

10

9

8

7

6

5

4

3

2

1

0

Res.

Res.

Res.

Res.

r

DATAVSIZE[1:0]
rw

rw

ACTION[1:0]
rw

rw

MATCH[3:0]
rw

rw

rw

rw

Bits 31:27 ID[4:0]: capability identification
Identifies the capability for match for comparator 1.
0b11010: instruction address, instruction address limit, data address, data address limit, and
data address with value
Bits 26:25 Reserved, must be kept at reset value.
Bit 24 MATCHED: Comparator match
Indicates if a comparator match has occurred since the register was last read.
0: no match
1: a match occurred
Bits 23:12 Reserved, must be kept at reset value.
Bits 11:10 DATAVSIZE[1:0]: data value size
Defines the size of the object being watched for by data value and data address comparators.
0x0: 1 byte
0x1: 2 bytes
0x2: 4 bytes
0x3: reserved
Bits 9:6 Reserved, must be kept at reset value.
Bits 5:4 ACTION[1:0]: action on match
0x0: trigger only
0x1: generate debug event
0x2: For a cycle counter, instruction address, data address, data value, or linked data value
comparator, generate a data trace match packet. For a data address with value comparator,
generate a data trace Data value packet.
0x3: For a data address limit comparator, generate a data trace data address packet.
For a cycle counter, instruction address limit, or data address comparator, generate a data
trace PC value packet. For a data address with value comparator, generate both a data trace
PC value packet and a data trace data value packet.
Bits 3:0 MATCH[3:0]: match type
Controls the type of match generated by comparator 1.
For possible values of this field, refer to [3].

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Debug support (DBG)

DWT function register 2 (DWT_FUNCTR2)
Address offset: 0x048
Reset value: 0x5000 0000
31

30

29

28

27

ID[4:0]
r

r

r

r

r

15

14

13

12

11

Res.

Res.

Res.

Res.

26

25

24

23

22

21

20

19

18

17

16

Res.

Res.

MATCH
ED

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

10

9

8

7

6

5

4

3

2

1

0

Res.

Res.

Res.

Res.

r

DATAVSIZE[1:0]
rw

rw

ACTION[1:0]
rw

rw

MATCH[3:0]
rw

rw

rw

rw

Bits 31:27 ID[4:0]: capability identification
Identifies the capability for match for comparator 2
0b01010: instruction address, data address, and data address with value
Bits 26:25 Reserved, must be kept at reset value.
Bit 24 MATCHED: comparator match
Indicates if a comparator match has occurred since the register was last read.
0: no match
1: a match occurred
Bits 23:12 Reserved, must be kept at reset value.
Bits 11:10 DATAVSIZE[1:0]: Data value size:
Defines the size of the object being watched for by data value and data address comparators.
0x0: 1 byte
0x1: 2 bytes
0x2: 4 bytes
0x3: reserved
Bits 9:6 Reserved, must be kept at reset value.
Bits 5:4 ACTION[1:0]: action on match
0x0: trigger only
0x1: Generate debug event
0x2: For a cycle counter, instruction address, data address, data value, or linked data value
comparator, generate a data trace match packet. For a data address with value comparator,
generate a data trace data value packet.
0x3: For a data address limit comparator, generate a data trace data address packet.
For a cycle counter, instruction address limit, or data address comparator, generate a data
trace PC value packet. For a data address with value comparator, generate both a data trace
PC value packet and a data trace data value packet.
Bits 3:0 MATCH[3:0]: match type
Controls the type of match generated by comparator 2.
For possible values of this field, refer to [3]

RM0456 Rev 6

<!-- pagebreak -->

3631

Debug support (DBG)

RM0456

DWT function register 3 (DWT_FUNCTR3)
Address offset: 0x058
Reset value: 0xF000 0000
31

30

29

28

27

ID[4:0]
r

r

r

r

r

15

14

13

12

11

Res.

Res.

Res.

Res.

26

25

24

23

22

21

20

19

18

17

16

Res.

Res.

MATCH
ED

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

10

9

8

7

6

5

4

3

2

1

0

Res.

Res.

Res.

Res.

r

DATAVSIZE[1:0]
rw

rw

ACTION[1:0]
rw

rw

MATCH[3:0]
rw

rw

rw

rw

Bits 31:27 ID[4:0]: capability identification
Identifies the capability for match for comparator 2.
0b11110: instruction address, instruction address limit, data address, data address limit, data
value, linked data value, and data address with value
Bits 26:25 Reserved, must be kept at reset value.
Bit 24 MATCHED: comparator match
Indicates if a comparator match has occurred since the register was last read.
0: no match
1: a match occurred
Bits 23:12 Reserved, must be kept at reset value.
Bits 11:10 DATAVSIZE[1:0]: data value size
Defines the size of the object being watched for by data value and data address comparators.
0x0: 1 byte
0x1: 2 bytes
0x2: 4 bytes
0x3: reserved
Bits 9:6 Reserved, must be kept at reset value.
Bits 5:4 ACTION[1:0]: action on match
0x0: trigger only
0x1: Generate debug event
0x2: For a cycle counter, instruction address, data address, data value, or linked data value
comparator, generate a data trace match packet. For a data address with value comparator,
generate a data trace data value packet.
0x3: For a data address limit comparator, generate a data trace data address packet.
For a cycle counter, instruction address limit, or data address comparator, generate a data
trace PC value packet. For a data address with value comparator, generate both a data trace
PC value packet and a data trace data value packet.
Bits 3:0 MATCH[3:0]: match type
Controls the type of match generated by comparator 2.
For possible values of this field, refer to [3]

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Debug support (DBG)

DWT device type architecture register (DWT_DEVARCHR)
Address offset: 0xFC8
Reset value: 0x4770 1A02
31

30

29

28

27

26

25

24

23

22

21

20

19

PRESE
NT

ARCHITECT[10:0]

18

17

16

REVISION[3:0]

r

r

r

r

r

r

r

r

r

r

r

r

r

r

r

r

15

14

13

12

11

10

9

8

7

6

5

4

3

2

1

0

r

r

r

r

r

ARCHVER[3:0]
r

r

r

ARCHPART[11:0]
r

r

r

r

r

r

r

r

Bits 31:21 ARCHITECT[10:0]: architect JEP106 code
0x23B: JEP106 continuation code 0x4, JEP106 ID code 0x3B. Arm limited.
Bit 20 PRESENT: DWT_DEVARCH register present
0x1: present
Bits 19:16 REVISION[3:0]: architecture revision
0x0: DWT architecture v2.0
Bits 15:12 ARCHVER[3:0]: architecture version
0x1: DWT architecture v2.0
Bits 11:0 ARCHPART[11:0]: architecture part
0xA02: DWT architecture

DWT device type register (DWT_DEVTYPER)
Address offset: 0xFCC
Reset value: 0x0000 0000
31

30

29

28

27

26

25

24

23

22

21

20

19

18

17

16

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

7

6

5

4

3

2

1

0

15

14

13

12

11

10

9

8

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

SUB[3:0]
r

r

r

MAJOR[3:0]
r

r

r

r

r

Bits 31:8 Reserved, must be kept at reset value.
Bits 7:4 SUB[3:0]: subtype
0x0: other
Bits 3:0 MAJOR[3:0]: major type
0x0: miscellaneous

RM0456 Rev 6

<!-- pagebreak -->

3631

Debug support (DBG)

RM0456

DWT CoreSight peripheral identity register 4 (DWT_PIDR4)
Address offset: 0xFD0
Reset value: 0x0000 0004
31

30

29

28

27

26

25

24

23

22

21

20

19

18

17

16

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

7

6

5

4

3

2

1

0

15

14

13

12

11

10

9

8

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

SIZE[3:0]
r

r

r

JEP106CON[3:0]
r

r

r

r

r

Bits 31:8 Reserved, must be kept at reset value.
Bits 7:4 SIZE[3:0]: register file size
0x0: The register file occupies a single 4-Kbyte region.
Bits 3:0 JEP106CON[3:0]: JEP106 continuation code
0x4: Arm JEDEC code

DWT CoreSight peripheral identity register 0 (DWT_PIDR0)
Address offset: 0xFE0
Reset value: 0x0000 0021
31

30

29

28

27

26

25

24

23

22

21

20

19

18

17

16

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

15

14

13

12

11

10

9

8

7

6

5

4

3

2

1

0

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.
r

r

r

r

r

r

PARTNUM[7:0]
r

r

Bits 31:8 Reserved, must be kept at reset value.
Bits 7:0 PARTNUM[7:0]: part number bits [7:0]
0x21: Cortex-M33 DWT part number

DWT CoreSight peripheral identity register 1 (DWT_PIDR1)
Address offset: 0xFE4
Reset value: 0x0000 00BD
31

30

29

28

27

26

25

24

23

22

21

20

19

18

17

16

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

7

6

5

4

3

2

1

0

15

14

13

12

11

10

9

8

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

JEP106ID[3:0]
r

Bits 31:8 Reserved, must be kept at reset value.

<!-- pagebreak -->

RM0456 Rev 6

r

r

PARTNUM[11:8]
r

r

r

r

r

RM0456

Debug support (DBG)

Bits 7:4 JEP106ID[3:0]: JEP106 identity code bits [3:0]
0xB: Arm JEDEC code
Bits 3:0 PARTNUM[11:8]: part number bits [11:8]
0xD: Cortex-M33 DWT part number

DWT CoreSight peripheral identity register 2 (DWT_PIDR2)
Address offset: 0xFE8
Reset value: 0x0000 000B
31

30

29

28

27

26

25

24

23

22

21

20

19

18

17

16

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

7

6

5

4

3

2

1

0

15

14

13

12

11

10

9

8

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

REVISION[3:0]
r

r

r

JEDEC
r

r

JEP106ID[6:4]
r

r

r

Bits 31:8 Reserved, must be kept at reset value.
Bits 7:4 REVISION[3:0]: component revision number
0x0: r0p0
Bit 3 JEDEC: JEDEC assigned value
0x1: designer identification specified by JEDEC
Bits 2:0 JEP106ID[6:4]: JEP106 identity code bits [6:4]
0x3: Arm JEDEC code

DWT CoreSight peripheral identity register 3 (DWT_PIDR3)
Address offset: 0xFEC
Reset value: 0x0000 0000
31

30

29

28

27

26

25

24

23

22

21

20

19

18

17

16

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

7

6

5

4

3

2

1

0

15

14

13

12

11

10

9

8

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

REVAND[3:0]
r

r

r

CMOD[3:0]
r

r

r

r

r

Bits 31:8 Reserved, must be kept at reset value.
Bits 7:4 REVAND[3:0]: metal fix version
0x0: no metal fix
Bits 3:0 CMOD[3:0]: customer modified
0x0: No customer modifications

RM0456 Rev 6

<!-- pagebreak -->

3631

Debug support (DBG)

RM0456

DWT CoreSight component identity register 0 (DWT_CIDR0)
Address offset: 0xFF0
Reset value: 0x0000 000D
31

30

29

28

27

26

25

24

23

22

21

20

19

18

17

16

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

7

6

5

4

3

2

1

0

r

r

r

15

14

13

12

11

10

9

8

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

PREAMBLE[7:0]
r

r

r

r

r

Bits 31:8 Reserved, must be kept at reset value.
Bits 7:0 PREAMBLE[7:0]: component identification bits [7:0]
0x0D: Common identification value

DWT CoreSight peripheral identity register 1 (DWT_CIDR1)
Address offset: 0xFF4
Reset value: 0x0000 0090
31

30

29

28

27

26

25

24

23

22

21

20

19

18

17

16

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

7

6

5

4

3

2

1

0

15

14

13

12

11

10

9

8

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

CLASS[3:0]
r

r

r

PREAMBLE[11:8]
r

r

r

r

r

Bits 31:8 Reserved, must be kept at reset value.
Bits 7:4 CLASS[3:0]: component identification bits [15:12] - component class
0x9: debug component
Bits 3:0 PREAMBLE[11:8]: component identification bits [11:8]
0x0: common identification value

DWT CoreSight component identity register 2 (DWT_CIDR2)
Address offset: 0xFF8
Reset value: 0x0000 0005
31

30

29

28

27

26

25

24

23

22

21

20

19

18

17

16

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

7

6

5

4

3

2

1

0

r

r

r

15

14

13

12

11

10

9

8

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

PREAMBLE[19:12]
r

Bits 31:8 Reserved, must be kept at reset value.

<!-- pagebreak -->

RM0456 Rev 6

r

r

r

r

RM0456

Debug support (DBG)

Bits 7:0 PREAMBLE[19:12]: component identification bits [23:16]
0x05: common identification value

DWT CoreSight component identity register 3 (DWT_CIDR3)
Address offset: 0xFFC
Reset value: 0x0000 00B1
31

30

29

28

27

26

25

24

23

22

21

20

19

18

17

16

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

7

6

5

4

3

2

1

0

r

r

r

15

14

13

12

11

10

9

8

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

PREAMBLE[27:20]
r

r

r

r

r

Bits 31:8 Reserved, must be kept at reset value.
Bits 7:0 PREAMBLE[27:20]: component identification bits [31:24]
0xB1: common identification value

75.6.2

DWT register map
The DWT registers are located at address range 0xE000 1000 to 0xE000 1FFC.

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

DWT_CPICNTR

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.
Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

DWT_SLPCNTR

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

0x014

Reset value
0x01C

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

DWT_FOLDCNTR

LSUCNT[7:0]
X X X X X X X X

Res.

Reset value
0x018

SLEEPCNT[7:0]
X X X X X X X X

Res.

Reset value
DWT_LSUCNTR

EXCCNT[7:0]
X X X X X X X X

Res.

Reset value
0x010

CPICNT[7:0]
X X X X X X X X

Res.

Reset value
0x00C

CYCCNTENA

POSTINIT
[3:0]

CYCTAP

0

Res.

CYCCNT[31:0]

Reset value

DWT_EXCCNTR

POSTPRESET
[3:0]

SYNCTAP[1:0]

0

Res.

0

PCSAMPLENA

0

Res.

0

Res.

0

CPIEVTENA

0

EXCTRCENA

0

EXCEVTENA

0

LSUEVTENA

0

SLEEPEVTENA

0

FOLDEVTENA

0

CYCDISS

0

CYCEVTENA

0

NOPRFCNT

0

Res.

0x008

1

DWT_CYCCNTR

Res.

0x004

0

NOCYCCNT

Reset value

NOTRCPKT

DWT_CTRLR

NOEXTTRIG

0x000

NUMCOMP
[3:0]

Offset Register name

31
30
29
28
27
26
25
24
23
22
21
20
19
18
17
16
15
14
13
12
11
10
9
8
7
6
5
4
3
2
1
0

Table 801. DWT register map and reset values

FOLDCNT[7:0]
X X X X X X X X

DWT_PCSR

EIASAMPLE[31:0]

Reset value

X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X

RM0456 Rev 6

<!-- pagebreak -->

3631

Debug support (DBG)

RM0456

Offset Register name

Res.

Res.

Res.

Res.

DATAVSIZE
[1:0]

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

0

0

0

0

0

0

1

0

1

0

0

Res.

Res.

Res.

Res.

DATAVSIZE
[1:0]

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

0

0

ACTION
[1:0]

1

MATCHED

ID[4:0]

Res.

Res.

DWT_FUNCTR1

MATCH[3:0]

0

0

0

0

0

0

1

0

1

0

0

Res.

Res.

Res.

Res.

DATAVSIZE
[1:0]

Res.

Res.

Res.

Res.

Res.

Res.

Res.

0

Reserved

0

ACTION
[1:0]

0

Res.

ID[4:0]

Res.

DWT_FUNCTR2

Res.

Reserved
Res.

X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X

Reserved
Res.

Reset value

MATCHED

Reserved
COMP[31:0]

Res.

Reserved
DWT_COMP2R

MATCH[3:0]

0

0

0

0

0

0

Reserved

1

1

1

0

0

PRESENT

Reset value

0

1

0

0

0

1

1

1

0

1

1

1

0

0

0

0

0

0

0

1

1

0

1

0

DWT_DEVTYPER

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

<!-- pagebreak -->

Res.

0

0

Reserved

RM0456 Rev 6

0

0

0

0

0

0

0

0

0

0

0

1

0

MAJOR[3:0]
0

0

0

0

0

JEP106CON
[3:0]

SIZE[3:0]
0

Reserved

0

SUB[3:0]

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Reset value
0xFD4 to
0xFDC

0

0
Res.

Reset value
DWT_PIDR4

0

ARCHPART[11:0]

Res.

ARCHITECT[10:0]

Res.

DWT_DEVARCHR

0xFD0

Res.

Res.

0

MATCH[3:0]

Reserved
REVISION
[3:0]

Reserved

Res.

DATAVSIZE
[1:0]

Res.

Res.

Res.

Res.

Res.

Res.

Res.

0

ACTION
[1:0]

1

ARCHVER
[3:0]

Reset value
0x05C to
0xFC4

ID[4:0]

Res.

DWT_FUNCTR3

Res.

Reserved
Res.

Reserved
Res.

X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X

Res.

COMP[31:0]

Reset value

MATCHED

DWT_COMP3R

0x054

0xFCC

0

Reserved

Reset value

0xFC8

0
Reserved

Reserved

0x04C

0x058

0

COMP[31:0]

0x044

0x050

1

X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X

0x03C

0x048

1

Reset value

Reset value

0x040

0

MATCH[3:0]

DWT_COMP1R

0x034

0x038

1

Res.

0x030

0

Reserved

ACTION
[1:0]

Reset value
0x02C

ID[4:0]

Res.

DWT_FUNCTR0

Res.

0x028

Res.

Reserved
Res.

X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X

Reserved
Res.

Reset value

0x024

MATCHED

COMP[31:0]

Res.

DWT_COMP0R

Res.

0x020

31
30
29
28
27
26
25
24
23
22
21
20
19
18
17
16
15
14
13
12
11
10
9
8
7
6
5
4
3
2
1
0

Table 801. DWT register map and reset values (continued)

0

0

1

0

0

RM0456

Debug support (DBG)

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

DWT_CIDR1

Res.

Reset value

0xFF4

Reset value

0

0

0

0

0

0

0

1

0

0

1

JEP106ID
[6:4]

1

1

0

1

1

0

0

0

0

0

0

0

1

PREAMBLE[7:0]
0

0

1

CLASS[3:0]

1

0

0

0

0

1

1

0

0

0

0

PREAMBLE[19:12]

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

DWT_CIDR3

Res.

0xFFC

Res.

Reset value

1

0

REVAND[3:0] CMOD[3:0]

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

DWT_CIDR2

Res.

Reset value
0xFF8

0

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

DWT_CIDR0

Res.

0xFF0

Res.

Reset value

1

0

PREAMBLE
[11:8]

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

DWT_PIDR3

Res.

0xFEC

0
Res.

Reset value

0

1

JEDEC

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

DWT_PIDR2

0

REVISION
[3:0]

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

1

Res.

1

Res.

PARTNUM
[11:8]

Res.

DWT_PIDR1

JEP106ID
[3:0]

Res.

0

Reset value

0xFE8

PARTNUM[7:0]

Reset value
Res.

0xFE4

DWT_PIDR0

Res.

0xFE0

Res.

Offset Register name

31
30
29
28
27
26
25
24
23
22
21
20
19
18
17
16
15
14
13
12
11
10
9
8
7
6
5
4
3
2
1
0

Table 801. DWT register map and reset values (continued)

0

0

0

1

0

1

PREAMBLE[27:20]
1

0

1

1

0

0

0

1

Refer to Table 798: Processor ROM table for register boundary addresses.

75.7

Instrumentation trace macrocell (ITM)
The ITM generates trace information in packets. Three sources can generate packets.
If multiple sources generate packets at the same time, the ITM arbitrates the order in which
packets are output. The three sources in decreasing order of priority are the following:
•

Software trace
The software can write directly to any of 32 x 32-bit ITM stimulus registers to generate
packets. The permission level for each port can be programmed. When software writes
to an enabled stimulus port, the ITM combines the identity of the port, the size of the
write access and the data written, into a packet that it writes to a FIFO. The ITM outputs
packets from the FIFO onto the trace bus. Reading a stimulus port register returns the
status of the stimulus register (empty or pending) in bit 0.

•

Hardware trace
The DWT generates trace packets in response to a data trace event, a PC sample or a
performance profiling counter wraparound. The ITM outputs these packets on the trace
bus.

•

Local timestamping
The ITM contains a 21-bit counter clocked by the (pre-divided) processor clock. The
counter value is output in a timestamp packet on the trace bus. The counter is reset to

RM0456 Rev 6

<!-- pagebreak -->

3631

Debug support (DBG)

RM0456

zero every time a timestamp packet is generated. The timestamps thus indicate the
time elapsed since the previous timestamp packet.
For more information on the ITM and how to use it, refer to [3].

75.7.1

ITM registers
The ITM registers are located at address range 0xE000 0000 to 0xE000 0FFC.

ITM stimulus register x (ITM_STIMRx)
Address offset: 0x000 + 0x4 * x, (x = 0 to 31)
Reset value: 0xXXXX XXXX
31

30

29

28

27

26

25

24

23

22

21

20

19

18

17

16

STIMULUS[31:16]
w

w

w

w

w

w

w

w

w

w

w

w

w

w

w

w

15

14

13

12

11

10

9

8

7

6

5

4

3

2

1

0

w

w

w

w

w

rw

rw

STIMULUS[15:0]
w

w

w

w

w

w

w

w

w

Bits 31:0 STIMULUS[31:0]: trace output data
When writing, write data is output on the trace bus as a software event packet.
When reading:
l
bit 1 is a disable flag:
–
0: stimulus port and ITM enabled
–
1: stimulus port and ITM disabled
l
bit 0 is a FIFO ready indicator:
–
0: stimulus port buffer is full (or port is disabled)
–
1: stimulus port can accept new write data

ITM trace enable register (ITM_TER)
Address offset: 0xE00
Reset value: 0x0000 0000
31

30

29

28

27

26

25

24

23

22

21

20

19

18

17

16

STIMENA[31:16]
rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

15

14

13

12

11

10

9

8

7

6

5

4

3

2

1

0

rw

rw

rw

rw

rw

rw

rw

STIMENA[15:0]
rw

rw

rw

rw

rw

rw

rw

rw

rw

Bits 31:0 STIMENA[31:0]: stimulus port enable
Each bit x(0 to 31) enables the stimulus port associated with the ITM_STIMRx register.
0: port disabled
1: port enabled

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Debug support (DBG)

ITM trace privilege register (ITM_TPR)
Address offset: 0xE40
Reset value: 0x0000 0000
31

30

29

28

27

26

25

24

23

22

21

20

19

18

17

16

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

3

2

1

0

15

14

13

12

11

10

9

8

7

6

5

4

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

PRIVMASK[3:0]
r

r

r

r

19

18

17

16

Bits 31:4 Reserved, must be kept at reset value.
Bits 3:0 PRIVMASK[3:0]: disable unprivileged access to ITM stimulus ports
Each bit controls eight stimulus ports.
XXX0: unprivileged access permitted on ports 0 to 7
XXX1: only privileged access permitted on ports 0 to 7
XX0X: unprivileged access permitted on ports 8 to 15
XX1X: only privileged access permitted on ports 8 to 15
X0XX: unprivileged access permitted on ports 16 to 23
X1XX: only privileged access permitted on ports 16 to 23
0XXX: unprivileged access permitted on ports 24 to 31
1XXX: only privileged access permitted on ports 24 to 31

ITM trace control register (ITM_TCR)
Address offset: 0xE80
Reset value: 0x0000 0000
31

30

29

28

27

26

25

24

23

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

BUSY
r

rw

rw

rw

rw

rw

rw

rw

15

14

13

12

11

10

9

8

7

6

5

4

3

2

1

0

Res.

STALL
ENA

Res.

Res.

Res.

Res.

Res.

Res.

TSPRESCALE[1
:0]
rw

Res.

rw

22

21

20

TRACEBUSID[6:0]

rw

SWOE
SYNCE
ITMEN
TXENA
TSENA
A
NA
NA
r

rw

rw

rw

rw

Bits 31:24 Reserved, must be kept at reset value.
Bit 23 BUSY: indicates whether the ITM is currently processing events
0: not busy
1: busy
Bits 22:16 TRACEBUSID[6:0]: identifier for multi-source trace stream formatting
If multi-source trace is in use, the debugger must write a non-zero value to this field.
Note: Different identifiers must be used for each trace source in the system.
Bits 15:10 Reserved, must be kept at reset value.

RM0456 Rev 6

<!-- pagebreak -->

3631

Debug support (DBG)

RM0456

Bits 9:8 TSPRESCALE[1:0]: local timestamp prescaler, used with the trace packet reference clock
0x0: no prescaling
0x1: Divide by 4.
0x2: Divide by 16.
0x3: Divide by 64.
Bits 7:6 Reserved, must be kept at reset value.
Bit 5 STALLENA: stall enable
0: Drop hardware source packets and generate an overflow if the ITM output is stalled.
1: Stall the processor to guarantee delivery of data trace packets.
Bit 4 SWOENA: SWO enable
Enables asynchronous clocking of the timestamp counter (read only).
0: Timestamp counter uses processor clock.
Bit 3 TXENA: transmit enable
Enables forwarding of hardware event packets from the DWT unit to the trace port.
0: disabled
1: enabled
Bit 2 SYNCENA: synchronization packet transmission enable
The debugger setting this bit must also configure the DWT_CTRLR.SYNCTAP field for the
correct synchronization speed.
0: disabled
1: enabled
Bit 1 TSENA: local timestamp generation enable
0: disabled
1: enabled
Bit 0 ITMENA: ITM enable
0: disabled
1: enabled

ITM device type architecture register (ITM_DEVARCHR)
Address offset: 0xFBC
Reset value: 0x4770 1A01
31

30

29

28

27

26

25

24

23

22

21

ARCHITECT[10:0]
r
15

r

r

r

r

r

r

r

r

14

13

12

11

10

9

8

7

ARCHVER[3:0]
r

r

r

20

19

PRESE
NT

18

16

REVISION[3:0]

r

r

r

r

r

r

r

6

5

4

3

2

1

0

r

r

r

r

r

ARCHPART[11:0]
r

r

r

r

r

r

r

r

Bits 31:21 ARCHITECT[10:0]: architect JEP106 code
0x23B: JEP106 continuation code 0x4, JEP106 ID code 0x3B. Arm limited.
Bit 20 PRESENT: DEVARCH register presence
0x1: present

<!-- pagebreak -->

17

RM0456 Rev 6

RM0456

Debug support (DBG)

Bits 19:16 REVISION[3:0]: architecture revision
0x0: ITM architecture v2.0
Bits 15:12 ARCHVER[3:0]: architecture version
0x1: ITM architecture v2.0
Bits 11:0 ARCHPART[11:0]: architecture part
0xA01: ITM architecture

ITM device type register (ITM_DEVTYPER)
Address offset: 0xFCC
Reset value: 0x0000 0043
31

30

29

28

27

26

25

24

23

22

21

20

19

18

17

16

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

7

6

5

4

3

2

1

0

15

14

13

12

11

10

9

8

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

SUB[3:0]
r

r

r

MAJOR[3:0]
r

r

r

r

r

Bits 31:8 Reserved, must be kept at reset value.
Bits 7:4 SUB[3:0]: sub-type
0x4: associated with a bus, stimulus derived from bus activity
Bits 3:0 MAJOR[3:0]: major type
0x3: trace source

ITM CoreSight peripheral identity register 4 (ITM_PIDR4)
Address offset: 0xFD0
Reset value: 0x0000 0004
31

30

29

28

27

26

25

24

23

22

21

20

19

18

17

16

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

7

6

5

4

3

2

1

0

15

14

13

12

11

10

9

8

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

SIZE[3:0]
r

r

r

JEP106CON[3:0]
r

r

r

r

r

Bits 31:8 Reserved, must be kept at reset value.
Bits 7:4 SIZE[3:0]: register file size
0x0: The register file occupies a single 4-Kbyte region.
Bits 3:0 JEP106CON[3:0]: JEP106 continuation code
0x4: Arm JEDEC code

RM0456 Rev 6

<!-- pagebreak -->

3631

Debug support (DBG)

RM0456

ITM CoreSight peripheral identity register 0 (ITM_PIDR0)
Address offset: 0xFE0
Reset value: 0x0000 0021
31

30

29

28

27

26

25

24

23

22

21

20

19

18

17

16

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

7

6

5

4

3

2

1

0

r

r

r

15

14

13

12

11

10

9

8

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

PARTNUM[7:0]
r

r

r

r

r

Bits 31:8 Reserved, must be kept at reset value.
Bits 7:0 PARTNUM[7:0]: part number bits [7:0]
0x21: ITM part number

ITM CoreSight peripheral identity register 1 (ITM_PIDR1)
Address offset: 0xFE4
Reset value: 0x0000 00BD
31

30

29

28

27

26

25

24

23

22

21

20

19

18

17

16

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

7

6

5

4

3

2

1

0

15

14

13

12

11

10

9

8

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

JEP106ID[3:0]
r

r

r

PARTNUM[11:8]
r

r

r

r

r

Bits 31:8 Reserved, must be kept at reset value.
Bits 7:4 JEP106ID[3:0]: JEP106 identity code bits [3:0]
0xB: Arm JEDEC code
Bits 3:0 PARTNUM[11:8]: part number bits [11:8]
0xD: ITM part number

ITM CoreSight peripheral identity register 2 (ITM_PIDR2)
Address offset: 0xFE8
Reset value: 0x0000 000B
31

30

29

28

27

26

25

24

23

22

21

20

19

18

17

16

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

7

6

5

4

3

2

1

0

15

14

13

12

11

10

9

8

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

REVISION[3:0]
r

Bits 31:8 Reserved, must be kept at reset value.

<!-- pagebreak -->

RM0456 Rev 6

r

r

JEDEC
r

r

JEP106ID[6:4]
r

r

r

RM0456

Debug support (DBG)

Bits 7:4 REVISION[3:0]: component revision number
0x0: r0p0
Bit 3 JEDEC: JEDEC assigned value
0x1: designer identification specified by JEDEC
Bits 2:0 JEP106ID[6:4]: JEP106 identity code bits [6:4]
0x3: Arm JEDEC code

ITM CoreSight peripheral identity register 3 (ITM_PIDR3)
Address offset: 0xFEC
Reset value: 0x0000 0000
31

30

29

28

27

26

25

24

23

22

21

20

19

18

17

16

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

7

6

5

4

3

2

1

0

15

14

13

12

11

10

9

8

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

REVAND[3:0]
r

r

r

CMOD[3:0]
r

r

r

r

r

Bits 31:8 Reserved, must be kept at reset value.
Bits 7:4 REVAND[3:0]: metal fix version
0x0: no metal fix
Bits 3:0 CMOD[3:0]: customer modified
0x0: no customer modifications

ITM CoreSight component identity register 0 (ITM_CIDR0)
Address offset: 0xFF0
Reset value: 0x0000 000D
31

30

29

28

27

26

25

24

23

22

21

20

19

18

17

16

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

7

6

5

4

3

2

1

0

r

r

r

15

14

13

12

11

10

9

8

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

PREAMBLE[7:0]
r

r

r

r

r

Bits 31:8 Reserved, must be kept at reset value.
Bits 7:0 PREAMBLE[7:0]: Component identification bits [7:0]
0x0D: Common identification value

RM0456 Rev 6

<!-- pagebreak -->

3631

Debug support (DBG)

RM0456

ITM CoreSight peripheral identity register 1 (ITM_CIDR1)
Address offset: 0xFF4
Reset value: 0x0000 00E0
31

30

29

28

27

26

25

24

23

22

21

20

19

18

17

16

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

7

6

5

4

3

2

1

0

15

14

13

12

11

10

9

8

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

CLASS[3:0]
r

r

r

PREAMBLE[11:8]
r

r

r

r

r

Bits 31:8 Reserved, must be kept at reset value.
Bits 7:4 CLASS[3:0]: Component identification bits [15:12] - component class
0xE: Trace generator component
Bits 3:0 PREAMBLE[11:8]: Component identification bits [11:8]
0x0: Common identification value

ITM CoreSight component identity register 2 (ITM_CIDR2)
Address offset: 0xFF8
Reset value: 0x0000 0005
31

30

29

28

27

26

25

24

23

22

21

20

19

18

17

16

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

15

14

13

12

11

10

9

8

7

6

5

4

3

2

1

0

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.
r

r

r

r

r

r

r

PREAMBLE[19:12]
r

Bits 31:8 Reserved, must be kept at reset value.
Bits 7:0 PREAMBLE[19:12]: Component identification bits [23:16]
0x05: Common identification value

ITM CoreSight component identity register 3 (ITM_CIDR3)
Address offset: 0xFFC
Reset value: 0x0000 00B1
31

30

29

28

27

26

25

24

23

22

21

20

19

18

17

16

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

7

6

5

4

3

2

1

0

r

r

r

15

14

13

12

11

10

9

8

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

PREAMBLE[27:20]
r

Bits 31:8 Reserved, must be kept at reset value.

<!-- pagebreak -->

RM0456 Rev 6

r

r

r

r

RM0456

Debug support (DBG)

Bits 7:0 PREAMBLE[27:20]: Component identification bits [31:24]
0xB1: Common identification value

75.7.2

ITM register map
The ITM registers are located at address range 0xE000 0000 to 0xE000 0FFC.

Offset Register name

Reserved

ITM_TER

STIMENA[31:0]
0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

ITM_TPR

0

0

0

PRIVMASK
[3:0]

Reset value

0

0

0

1

1

0

1

1

1

0

0

0

0

0

0

0

1

1

0

1

0

Res.

1

Res.

0

Res.

0

Res.

0

0

0

0

0

0

0

0

0

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

SUB[3:0]

Res.

0

0

0

1

1

0

MAJOR[3:0]
0

SIZE[3:0]

0

1

1

0

0

Reset value

0

0

ITM_PIDR1

JEP106ID
[3:0]

PARTNUM
[11:8]

Reset value

1

1

1

1

ITM_PIDR2

REVISION
[3:0]

JEP106ID
[6:4]

0

1

0

Reset value

RM0456 Rev 6

Res.
Res.
Res.

Res.
Res.
Res.

Res.
Res.
Res.

Res.
Res.
Res.

Res.
Res.
Res.

Res.
Res.
Res.

Res.
Res.
Res.

Res.
Res.
Res.

Res.
Res.
Res.

Res.
Res.
Res.

Res.
Res.
Res.

Res.
Res.
Res.

Res.
Res.
Res.

Res.
Res.
Res.

Res.
Res.
Res.

Res.
Res.
Res.

Res.
Res.
Res.

Res.
Res.
Res.

Res.
Res.
Res.

Res.
Res.
Res.

Res.

Res.
Res.

Res.
Res.

Res.
Res.

Res.

ITM_PIDR0

0

0

1

0

0

0

1

Reserved
Res.

Reserved

0

0

JEP106CON
[3:0]

JEDEC

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

0

Res.

0xFE8

0

Reserved

Res.

0xFE4

0

ARCHPART[3:0]

Res.

1

ARCHVER
[3:0]

Res.

0

REVISION
[3:0]

Res.

ARCHITECT[10:0]

Res.

0xFE0

Res.

Res.

TSPRESCALE
[1:0]

Res.

Res.

Res.

Res.

Res.

Res.
0

TSENA

0

ITMENA

0

Reset value
0xFD40xFDC

0

SYNCENA

0

Reserved

ITM_PIDR4

0

TXENA

0

Reset value
0xFD0

0

SWOENA

0

PRESEN

ITM_DEVARCHR

ITM_DEVTYPER

0

Reserved

Res.

0xFCC

TRACEBUSID[6:0]

Reserved

Reset value
0xFC00xFC8

0

Res.

0xFBC

0

STALLENA

Reset value
0xE840xFB8

BUSY

Res.

Res.

Res.

Res.

Res.

Res.

ITM_TCR

0

Reserved

Res.

0xE80

Reserved

Res.

0xE440xE7C

0

Reserved
Res.

Reserved

0

Res.

Reset value

Res.

Reserved

Res.

0xE40

X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X

Res.

0xE040xE3C

Reset value

Res.

0xE00

STIMULUS[31:0]

Res.

0x07C0xDFC

ITM_STIMR0 to
ITM_STIMR31

Res.

0x000 to
0x07C

31
30
29
28
27
26
25
24
23
22
21
20
19
18
17
16
15
14
13
12
11
10
9
8
7
6
5
4
3
2
1
0

Table 802. ITM register map and reset values

PARTNUM[7:0]

0

0

1

1

0

0

0

0

0

0

1

1

1

<!-- pagebreak -->

3631

Debug support (DBG)

RM0456

Table 802. ITM register map and reset values (continued)

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

ITM_PIDR3

0xFEC

31
30
29
28
27
26
25
24
23
22
21
20
19
18
17
16
15
14
13
12
11
10
9
8
7
6
5
4
3
2
1
0

Offset Register name

REVAND[3:0] CMOD[3:0]

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

ITM_CIDR1

Reset value

0

0

0

0

0

0

1

1

0

1

1

0

1

1

0

0

0

0

PREAMBLE[19:12]

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

ITM_CIDR3

0xFFC

0

CLASS[3:0]

0
Res.

Reset value

0

PREAMBLE
[11:8]

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

0xFF8

Res.

ITM_CIDR2

Res.

Reset value

0

PREAMBLE[7:0]
0

Res.

Reset value
0xFF4

0

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

0xFF0

0
Res.

ITM_CIDR0

Res.

Reset value

0

0

0

0

1

0

1

PREAMBLE[27:20]
1

0

1

1

0

0

0

1

Refer to Table 798: Processor ROM table for register boundary addresses.

75.8

Breakpoint unit (BPU)
The BPU allows the user to set hardware breakpoints. It contains eight comparators that
monitor the instruction fetch address. If a match occurs, the instruction comparators can be
configured to generate a breakpoint instruction.
For more information on the breakpoint unit and how to use it, refer to [3].

75.8.1

BPU registers
The BPU registers are located at address range 0xE0002000 to 0xE0002FFC.

BPU control register (BPU_CTRLR)
Address offset: 0x000
Reset value: 0x1000 0080
31

30

29

28

REV[3:0]
r

r

r

r

15

14

13

12

Res.

NUM_CODE[6:4]
r

r

27

26

25

24

23

22

21

20

19

18

17

16

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

11

10

9

8

7

6

5

4

3

2

1

0

KEY

ENABL
E

rw

rw

Res.

Res.

Res.

Res.

r

NUM_CODE[3:0]
r

r

r

Res.
r

Bits 31:28 REV[3:0]: revision number
0x1: BPU version 2
Bits 27:15 Reserved, must be kept at reset value.
Bits 14:12, 7:4 NUM_CODE[6:0]: number of instruction address comparators supported
0x08: 8 instruction comparators supported

<!-- pagebreak -->

RM0456 Rev 6

Res.

RM0456

Debug support (DBG)

Bits 11:8, 3:2 Reserved, must be kept at reset value.
Bit 1 KEY: Write protect key
A write to FPB_CTRLR register is ignored if this bit is not set to 1.
Bit 0 ENABLE: FPB enable
0: disabled
1: enabled

BPU comparator x register (BPU_COMPxR)
Address offset: 0x008 + 0x4 * x, (x = 0 to 7)
Reset value: 0x0000 0000
31

30

29

28

27

26

25

24

23

22

21

20

19

18

17

16

BPADDR[31:16]
rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

15

14

13

12

11

10

9

8

7

6

5

4

3

2

1

0

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

19

18

17

16

BPADDR[15:1]
rw

BE

Bits 31:1 BPADDR[31:1]: breakpoint address
Bit 0 BE: breakpoint enable
0: disabled
1: enabled

BPU device type architecture register (BPU_DEVARCHR)
Address offset: 0xFBC
Reset value: 0x4770 1A03
31

30

29

28

27

26

25

24

23

22

21

20
PRESE
NT

ARCHITECT[10:0]

REVISION[3:0]

r

r

r

r

r

r

r

r

r

r

r

r

r

r

r

r

15

14

13

12

11

10

9

8

7

6

5

4

3

2

1

0

r

r

r

r

r

ARCHVER[3:0]
r

r

r

ARCHPART[11:0]
r

r

r

r

r

r

r

r

Bits 31:21 ARCHITECT[10:0]: architect JEP106 code
0x23B: JEP106 continuation code 0x4, JEP106 ID code 0x3B. Arm limited.
Bit 20 PRESENT: DEVARCH register present
0x1: present
Bits 19:16 REVISION[3:0]: architecture revision
0x0: BPU architecture v2.0
Bits 15:12 ARCHVER[3:0]: architecture version
0x1: BPU architecture v2.0

RM0456 Rev 6

<!-- pagebreak -->

3631

Debug support (DBG)

RM0456

Bits 11:0 ARCHPART[11:0]: architecture part
0xA03: BPU architecture

BPU device type register (BPU_DEVTYPER)
Address offset: 0xFCC
Reset value: 0x0000 0000
31

30

29

28

27

26

25

24

23

22

21

20

19

18

17

16

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

7

6

5

4

3

2

1

0

15

14

13

12

11

10

9

8

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

SUB[3:0]
r

r

r

MAJOR[3:0]
r

r

r

r

r

Bits 31:8 Reserved, must be kept at reset value.
Bits 7:4 SUB[3:0]: sub-type
0x0: other
Bits 3:0 MAJOR[3:0]: major type
0x0: miscellaneous

BPU CoreSight peripheral identity register 4 (BPU_PIDR4)
Address offset: 0xFD0
Reset value: 0x0000 0004
31

30

29

28

27

26

25

24

23

22

21

20

19

18

17

16

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

7

6

5

4

3

2

1

0

15

14

13

12

11

10

9

8

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

SIZE[3:0]
r

r

r

Bits 31:8 Reserved, must be kept at reset value.
Bits 7:4 SIZE[3:0]: register file size
0x0: The register file occupies a single 4-Kbyte region.
Bits 3:0 JEP106CON[3:0]: JEP106 continuation code
0x4: Arm JEDEC code

<!-- pagebreak -->

RM0456 Rev 6

JEP106CON[3:0]
r

r

r

r

r

RM0456

Debug support (DBG)

BPU CoreSight peripheral identity register 0 (BPU_PIDR0)
Address offset: 0xFE0
Reset value: 0x0000 0021
31

30

29

28

27

26

25

24

23

22

21

20

19

18

17

16

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

7

6

5

4

3

2

1

0

r

r

r

15

14

13

12

11

10

9

8

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

PARTNUM[7:0]
r

r

r

r

r

Bits 31:8 Reserved, must be kept at reset value.
Bits 7:0 PARTNUM[7:0]: part number bits [7:0]
0x21: BPU part number

BPU CoreSight peripheral identity register 1 (BPU_PIDR1)
Address offset: 0xFE4
Reset value: 0x0000 00BD
31

30

29

28

27

26

25

24

23

22

21

20

19

18

17

16

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

7

6

5

4

3

2

1

0

15

14

13

12

11

10

9

8

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

JEP106ID[3:0]
r

r

r

PARTNUM[11:8]
r

r

r

r

r

Bits 31:8 Reserved, must be kept at reset value.
Bits 7:4 JEP106ID[3:0]: JEP106 identity code bits [3:0]
0xB: Arm JEDEC code
Bits 3:0 PARTNUM[11:8]: part number bits [11:8]
0xD: BPU part number

BPU CoreSight peripheral identity register 2 (BPU_PIDR2)
Address offset: 0xFE8
Reset value: 0x0000 000B
31

30

29

28

27

26

25

24

23

22

21

20

19

18

17

16

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

7

6

5

4

3

2

1

0

15

14

13

12

11

10

9

8

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

REVISION[3:0]
r

r

r

JEDEC
r

r

JEP106ID[6:4]
r

r

r

Bits 31:8 Reserved, must be kept at reset value.

RM0456 Rev 6

<!-- pagebreak -->

3631

Debug support (DBG)

RM0456

Bits 7:4 REVISION[3:0]: component revision number
0x0: r0p0
Bit 3 JEDEC: JEDEC assigned value
0x1: designer identification specified by JEDEC
Bits 2:0 JEP106ID[6:4]: JEP106 identity code bits [6:4]
0x3: Arm JEDEC code

BPU CoreSight peripheral identity register 3 (BPU_PIDR3)
Address offset: 0xFEC
Reset value: 0x0000 0000
31

30

29

28

27

26

25

24

23

22

21

20

19

18

17

16

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

7

6

5

4

3

2

1

0

15

14

13

12

11

10

9

8

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

REVAND[3:0]
r

r

r

CMOD[3:0]
r

r

r

r

r

Bits 31:8 Reserved, must be kept at reset value.
Bits 7:4 REVAND[3:0]: metal fix version
0x0: no metal fix
Bits 3:0 CMOD[3:0]: customer modified
0x0: no customer modifications

BPU CoreSight component identity register 0 (BPU_CIDR0)
Address offset: 0xFF0
Reset value: 0x0000 000D
31

30

29

28

27

26

25

24

23

22

21

20

19

18

17

16

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

7

6

5

4

3

2

1

0

r

r

r

15

14

13

12

11

10

9

8

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

PREAMBLE[7:0]
r

r

Bits 31:8 Reserved, must be kept at reset value.
Bits 7:0 PREAMBLE[7:0]: component identification bits [7:0]
0x0D: common identification value

<!-- pagebreak -->

RM0456 Rev 6

r

r

r

RM0456

Debug support (DBG)

BPU CoreSight peripheral identity register 1 (BPU_CIDR1)
Address offset: 0xFF4
Reset value: 0x0000 0090
31

30

29

28

27

26

25

24

23

22

21

20

19

18

17

16

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

7

6

5

4

3

2

1

0

15

14

13

12

11

10

9

8

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

CLASS[3:0]
r

r

r

PREAMBLE[11:8]
r

r

r

r

r

Bits 31:8 Reserved, must be kept at reset value.
Bits 7:4 CLASS[3:0]: component identification bits [15:12] - component class
0x9: debug component
Bits 3:0 PREAMBLE[11:8]: component identification bits [11:8]
0x0: common identification value

BPU CoreSight component identity register 2 (BPU_CIDR2)
Address offset: 0xFF8
Reset value: 0x0000 0005
31

30

29

28

27

26

25

24

23

22

21

20

19

18

17

16

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

15

14

13

12

11

10

9

8

7

6

5

4

3

2

1

0

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.
r

r

r

r

r

r

PREAMBLE[19:12]
r

r

Bits 31:8 Reserved, must be kept at reset value.
Bits 7:0 PREAMBLE[19:12]: component identification bits [23:16]
0x05: common identification value

BPU CoreSight component identity register 3 (BPU_CIDR3)
Address offset: 0xFFC
Reset value: 0x0000 00B1
31

30

29

28

27

26

25

24

23

22

21

20

19

18

17

16

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

7

6

5

4

3

2

1

0

r

r

r

15

14

13

12

11

10

9

8

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

PREAMBLE[27:20]
r

r

r

r

r

Bits 31:8 Reserved, must be kept at reset value.

RM0456 Rev 6

<!-- pagebreak -->

3631

Debug support (DBG)

RM0456

Bits 7:0 PREAMBLE[27:20]: component identification bits [31:24]
0xB1: common identification value

75.8.2

BPU register map
The BPU registers are located at address range 0xE000 2000 to 0xE000 2FFC.

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

1

0

1

1

1

0

0

0

0

0

0

0

1

1

0

1

0

Res.

1

Res.

1

Res.

0

Res.

0

Res.

Res.

0

0

0

0

0

0

0

1

1

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

SUB[3:0]

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

0

0

MAJOR[3:0]
0

SIZE[3:0]
0

0

0

0

0

0

JEP106CON
[3:0]
0

0

1

0

0

0

1

Res.
Res.
Res.

Res.
Res.

Res.
Res.

Res.

Res.
Res.

Res.

Res.
Res.

Res.
Res.

Res.
Res.

Res.
Res.

Res.
Res.

Res.
Res.

Res.
Res.

Res.
Res.

Res.
Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

0

Res.

1

Res.

0

Res.

BPU_PIDR2

Res.

1

JEP106ID
[6:4]

Res.

1

Res.

1

REVISION
[3:0]

Res.

1

Res.

Reset value

Res.

BPU_PIDR1

PARTNUM
[11:8]

JEDEC

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

PARTNUM[7:0]

JEP106ID
[3:0]

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

BPU_PIDR3

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

0

0

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.
Res.

Res.

RM0456 Rev 6

0

1

1

0

0

0

0

0

0

1

1

1

0

0

0

0

0

0

0

1

PREAMBLE[7:0]
0

Res.

BPU_CIDR1

0

REVAND[3:0] CMOD[3:0]
0

Res.

BPU_CIDR0

Reset value

<!-- pagebreak -->

0

0

Reset value
0xFF4

0

Reset value

Reset value
0xFF0

0

Reserved

Reset value
0xFEC

0

0

Res.

0xFE8

0

0

Res.

0xFE4

0

0

Reserved

Reserved
BPU_PIDR0

0

0

ARCHPART[11:0]

Res.

0

ARCHVER
[3:0]

Res.

1

REVISION
[3:0]

Res.

0

PRESEN

ARCHITECT[10:0]

Res.

0xFE0

NUM_CODE
[3:0]

Res.

Res.
0

Reset value
0xFD40xFDC

0

BE
0

Reset value
BPU_PIDR4

0

Reserved

Reserved

0xFD0

0

KEY

0

1

ENABLE

0

Res.

Res.

NUM_CODE
[6:4]
0

BPADDR[31:1]

BPU_DEVARCHR

BPU_DEVTYPER

Res.
0

Reserved

Reset value

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

1

Res.

Reset value

0xFC00xFC8
0xFCC

0

BPU_COMP0R to
BPU_COMP7R

0x0280xFB8

0xFBC

0

Reserved

Res.

0x008 to
0x024

0

Reserved

Res.

Reset value
0x004

REV[3:0]

Res.

BPU_CTRLR

0x000

Res.

Offset Register name

31
30
29
28
27
26
25
24
23
22
21
20
19
18
17
16
15
14
13
12
11
10
9
8
7
6
5
4
3
2
1
0

Table 803. BPU register map and reset values

0

0

0

1

1

CLASS[3:0]

PREAMBLE
[11:8]

1

0

0

0

1

0

0

0

RM0456

Debug support (DBG)

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

BPU_CIDR2

Res.

0xFF8

Res.

Offset Register name

31
30
29
28
27
26
25
24
23
22
21
20
19
18
17
16
15
14
13
12
11
10
9
8
7
6
5
4
3
2
1
0

Table 803. BPU register map and reset values (continued)

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

0xFFC

0
Res.

Reset value
BPU_CIDR3

PREAMBLE[19:12]

Reset value

0

0

0

0

1

0

1

PREAMBLE[27:20]
1

0

1

1

0

0

0

1

Refer to Table 798: Processor ROM table for register boundary addresses.

75.9

Embedded Trace Macrocell (ETM)
The ETM is a CoreSight component closely coupled to the CPU. The ETM generates trace
packets that allow the execution of the Cortex-M33 core to be traced. In STM32U5 series,
the ETM is configured for instruction trace only. Data accesses are not included in the trace
information.
The ETM receives information from the CPU over the processor trace interface, including:
•

number of instructions executed in the same cycle

•

changes in program flow

•

current processor instruction state

•

addresses of memory locations accessed by load and store instructions

•

type, direction and size of a transfer

•

Condition code information

•

exception information

•

wait for interrupt state information

For more information, refer to the Arm CoreSight ETM-M33 Technical
Reference Manual [5].

75.9.1

ETM registers
The ETM registers are located at address range 0xE004 1000 to 0xE004 1FFC.

ETM programming control register (ETM_PRGCTLR)
Address offset: 0x004
Reset value: 0x0000 0000
31

30

29

28

27

26

25

24

23

22

21

20

19

18

17

16

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

15

14

13

12

11

10

9

8

7

6

5

4

3

2

1

0

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

EN
rw

Bits 31:1 Reserved, must be kept at reset value.

RM0456 Rev 6

<!-- pagebreak -->

