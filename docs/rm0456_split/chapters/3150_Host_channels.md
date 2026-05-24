RM0456 Rev 6

RM0456

USB on-the-go full-speed (OTG_FS)
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

1.

Initialize channel 2. The application must set the ODDFRM bit in OTG_HCCHAR2.

2.

Set the CHENA bit in OTG_HCCHAR2 to write an IN request to the periodic request
queue.

3.

The OTG_FS host writes an IN request to the periodic request queue for each
OTG_HCCHAR2 register write with the CHENA bit set.

4.

The OTG_FS host attempts to send an IN token in the next (odd) frame.

5.

As soon as the IN packet is received and written to the receive FIFO, the OTG_FS host
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

<!-- pagebreak -->

