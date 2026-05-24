3291

USB on-the-go full-speed (OTG_FS)

RM0456

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

<!-- pagebreak -->

RM0456 Rev 6

RM0456

USB on-the-go full-speed (OTG_FS)
OTG_HCCHAR1 to control. During the setup stage, the application is expected to set
the PID field in OTG_HCTSIZ1 to SETUP.
•

Interrupt OUT transactions
A typical interrupt OUT operation is shown in Figure 906. The assumptions are:
–

The application is attempting to send one packet in every frame (up to 1 maximum
packet size), starting with the odd frame (transfer size = 1 024 bytes)

–

The periodic transmit FIFO can hold one packet (1 Kbyte)

–

Periodic request queue depth = 4

The sequence of operations is as follows:
1.

Initialize and enable channel 1. The application must set the ODDFRM bit in
OTG_HCCHAR1.

2.

Write the first packet for channel 1.

3.

Along with the last word write of each packet, the OTG_FS host writes an entry to the
periodic request queue.

4.

The OTG_FS host attempts to send an OUT token in the next (odd) frame.

5.

The OTG_FS host generates an XFRC interrupt as soon as the last packet is
transmitted successfully.

6.

In response to the XFRC interrupt, reinitialize the channel for the next transfer.

RM0456 Rev 6

<!-- pagebreak -->

