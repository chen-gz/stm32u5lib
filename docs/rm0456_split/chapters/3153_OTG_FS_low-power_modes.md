3291

USB on-the-go full-speed (OTG_FS)

RM0456

packet size), starting with an odd frame. (transfer size = 1 024 bytes).
–

The periodic transmit FIFO can hold one packet (1 Kbyte).

–

Periodic request queue depth = 4.

The sequence of operations is as follows:

<!-- pagebreak -->

1.

Initialize and enable channel 1. The application must set the ODDFRM bit in
OTG_HCCHAR1.

2.

Write the first packet for channel 1.

3.

Along with the last word write of each packet, the OTG_FS host writes an entry to the
periodic request queue.

4.

The OTG_FS host attempts to send the OUT token in the next frame (odd).

5.

The OTG_FS host generates the XFRC interrupt as soon as the last packet is
transmitted successfully.

6.

In response to the XFRC interrupt, reinitialize the channel for the next transfer.

7.

Handling non-ACK responses

RM0456 Rev 6

RM0456

USB on-the-go full-speed (OTG_FS)
Figure 908. Isochronous OUT transactions
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
RM0456 Rev 6

<!-- pagebreak -->

3291

USB on-the-go full-speed (OTG_FS)

RM0456

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

<!-- pagebreak -->

RM0456 Rev 6

RM0456

USB on-the-go full-speed (OTG_FS)
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
1.

Initialize channel 2. The application must set the ODDFRM bit in OTG_HCCHAR2.

2.

Set the CHENA bit in OTG_HCCHAR2 to write an IN request to the periodic request
queue.

3.

The OTG_FS host writes an IN request to the periodic request queue for each
OTG_HCCHAR2 register write with the CHENA bit set.

4.

The OTG_FS host attempts to send an IN token in the next odd frame.

5.

As soon as the IN packet is received and written to the receive FIFO, the OTG_FS host
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

<!-- pagebreak -->

3291

USB on-the-go full-speed (OTG_FS)

RM0456

Figure 909. Isochronous IN transactions
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

<!-- pagebreak -->

RM0456 Rev 6

RM0456

USB on-the-go full-speed (OTG_FS)
transfers. The deeper the queue (along with sufficient FIFO size), the more often the
core is able to pipeline non-periodic transfers. If the queue size is small, the core is
able to put in new requests only when the queue space is freed up.
The core’s periodic request queue depth is critical to perform periodic transfers as
scheduled. Select the periodic queue depth, based on the number of periodic transfers
scheduled in a microframe. If the periodic request queue depth is smaller than the
periodic transfers scheduled in a microframe, a frame overrun condition occurs.
•

Handling babble conditions
OTG_FS controller handles two cases of babble: packet babble and port babble.
Packet babble occurs if the device sends more data than the maximum packet size for
the channel. Port babble occurs if the core continues to receive data from the device at
EOF2 (the end of frame 2, which is very close to SOF).
When OTG_FS controller detects a packet babble, it stops writing data into the Rx
buffer and waits for the end of packet (EOP). When it detects an EOP, it flushes already
written data in the Rx buffer and generates a Babble interrupt to the application.
When OTG_FS controller detects a port babble, it flushes the Rx FIFO and disables the
port. The core then generates a port disabled interrupt (HPRTINT in OTG_GINTSTS,
PENCHNG in OTG_HPRT). On receiving this interrupt, the application must determine
that this is not due to an overcurrent condition (another cause of the port disabled
interrupt) by checking POCA in OTG_HPRT, then perform a soft reset. The core does
not send any more tokens after it has detected a port babble condition.

72.16.5

Device programming model
Endpoint initialization on USB reset
1.

Set the NAK bit for all OUT endpoints
–

2.

Unmask the following interrupt bits
–

3.

4.

SNAK = 1 in OTG_DOEPCTLx (for all OUT endpoints)
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

STUPCNT = 3 in OTG_DOEPTSIZ0 (to receive up to 3 back-to-back SETUP
packets)

RM0456 Rev 6

<!-- pagebreak -->

3291

USB on-the-go full-speed (OTG_FS)

RM0456

At this point, all initialization required to receive SETUP packets is done.

Endpoint initialization on enumeration completion
1.

On the Enumeration Done interrupt (ENUMDNE in OTG_GINTSTS), read the
OTG_DSTS register to determine the enumeration speed.

2.

Program the MPSIZ field in OTG_DIEPCTL0 to set the maximum packet size. This
step configures control endpoint 0. The maximum packet size for a control endpoint
depends on the enumeration speed.

At this point, the device is ready to receive SOF packets and is configured to perform control
transfers on control endpoint 0.

Endpoint initialization on SetAddress command
This section describes what the application must do when it receives a SetAddress
command in a SETUP packet.
1.

Program the OTG_DCFG register with the device address received in the SetAddress
command

2.

Program the core to send out a status IN packet

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

<!-- pagebreak -->

RM0456 Rev 6

RM0456

USB on-the-go full-speed (OTG_FS)
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

RM0456 Rev 6

<!-- pagebreak -->

3291

USB on-the-go full-speed (OTG_FS)

RM0456

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

Figure 910 provides a flowchart of the above procedure.
Figure 910. Receive FIFO packet read

SETUP transactions

<!-- pagebreak -->

RM0456 Rev 6

RM0456

USB on-the-go full-speed (OTG_FS)
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

The core internally sets the IN NAK and OUT NAK bits for the control IN/OUT
endpoints on which the SETUP packet was received.

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

RM0456 Rev 6

<!-- pagebreak -->

3291

USB on-the-go full-speed (OTG_FS)
1.

RM0456

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

On this interrupt, the application must read the OTG_DOEPTSIZx register to
determine the number of SETUP packets received and process the last received
SETUP packet.
Figure 911. Processing a SETUP packet

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
the same endpoint. When this condition occurs, the OTG_FS controller generates an
interrupt (B2BSTUP in OTG_DOEPINTx).
•

Setting the global OUT NAK

Internal data flow:
1.

<!-- pagebreak -->

When the application sets the Global OUT NAK (SGONAK bit in OTG_DCTL), the core
stops writing data, except SETUP packets, to the receive FIFO. Irrespective of the

RM0456 Rev 6

RM0456

USB on-the-go full-speed (OTG_FS)
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
1.

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

–
6.

CGONAK = 1 in OTG_DCTL

If the application has masked this interrupt earlier, it must be unmasked as follows:
–

•

GONAKEFFM = 0 in the OTG_GINTMSK register

Whenever the application is ready to exit the Global OUT NAK mode, it must clear the
SGONAK bit in OTG_DCTL. This also clears the GONAKEFF interrupt
(OTG_GINTSTS).

GONAKEFFM = 1 in OTG_GINTMSK

Disabling an OUT endpoint

The application must use this sequence to disable an OUT endpoint that it has enabled.
Application programming sequence:

RM0456 Rev 6

<!-- pagebreak -->

3291

USB on-the-go full-speed (OTG_FS)
1.

Before disabling any OUT endpoint, the application must enable Global OUT NAK
mode in the core.
–

2.
3.

4.

5.

SGONAK = 1 in OTG_DCTL

Wait for the GONAKEFF interrupt (OTG_GINTSTS)
Disable the required OUT endpoint by programming the following fields:
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

RM0456

SGONAK = 0 in OTG_DCTL

Transfer Stop Programming for OUT endpoints

The application must use the following programing sequence to stop any transfers (because
of an interrupt from the host, typically a reset).
Sequence of operations:
1.

Enable all OUT endpoints by setting
–

2.

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
mode in the core, according to the instructions in “Setting the global OUT NAK on
page 3268”. This ensures that data in the RxFIFO is sent to the application
successfully. Set SGONAK = 1 in OTG_DCTL

4.

Wait for the GONAKEFF interrupt (OTG_GINTSTS)

5.

Disable all active OUT endpoints by programming the following register bits:

6.

<!-- pagebreak -->

EPENA = 1 in all OTG_DOEPCTLx registers.

Flush the RxFIFO as follows

–

EPDIS = 1 in registers OTG_DOEPCTLx

–

SNAK = 1 in registers OTG_DOEPCTLx

Wait for the EPDIS interrupt in OTG_DOEPINTx for each OUT endpoint programmed
in the previous step. The EPDIS interrupt in OTG_DOEPINTx indicates that the

RM0456 Rev 6

RM0456

USB on-the-go full-speed (OTG_FS)
corresponding OUT endpoint is completely disabled. When the EPDIS interrupt is
asserted, the following bits are cleared:

•

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

RM0456 Rev 6

<!-- pagebreak -->

3291

USB on-the-go full-speed (OTG_FS)

RM0456

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

<!-- pagebreak -->

RM0456 Rev 6

RM0456

USB on-the-go full-speed (OTG_FS)
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
The number of USB packets in which this payload was received =
Application programmed initial packet count – core updated final packet count

The application can discard invalid data packets.
•

Incomplete isochronous OUT data transfers

This section describes the application programming sequence when isochronous OUT data
packets are dropped inside the core.
Internal data flow:
1.

For isochronous OUT endpoints, the XFRC interrupt (in OTG_DOEPINTx) may not
always be asserted. If the core drops isochronous OUT data packets, the application

RM0456 Rev 6

<!-- pagebreak -->

3291

USB on-the-go full-speed (OTG_FS)

RM0456

may fail to detect the XFRC interrupt (OTG_DOEPINTx) under the following
circumstances:

2.

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

<!-- pagebreak -->

RM0456 Rev 6

RM0456

USB on-the-go full-speed (OTG_FS)
1.

Put the core in the Global OUT NAK mode.

2.

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

Figure 912 depicts the reception of a single Bulk OUT data packet from the USB to the AHB
and describes the events involved in the process.
Figure 912. Bulk OUT transaction
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

5

RXFLVL iintr
EPCTLx
, NAK=1
PKTCN
T0

OTG_DO

XFRSIZ
=0
r
OU T
NA K

7

idle until intr

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

RM0456 Rev 6

<!-- pagebreak -->

3291

USB on-the-go full-speed (OTG_FS)

RM0456

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

<!-- pagebreak -->

RM0456 Rev 6

RM0456

USB on-the-go full-speed (OTG_FS)
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

To exit endpoint NAK mode, the application must clear the NAK status bit (NAKSTS) in
OTG_DIEPCTLx. This also clears the INEPNE interrupt (in OTG_DIEPINTx).
–

6.

CNAK = 1 in OTG_DIEPCTLx

If the application masked this interrupt earlier, it must be unmasked as follows:
–

•

INEPNEM = 0 in OTG_DIEPMSK

INEPNEM = 1 in OTG_DIEPMSK

IN endpoint disable

Use the following sequence to disable a specific IN endpoint that has been previously
enabled.
Application programming sequence:

RM0456 Rev 6

<!-- pagebreak -->

3291

USB on-the-go full-speed (OTG_FS)

RM0456

1.

The application must stop writing data on the AHB for the IN endpoint to be disabled.

2.

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

<!-- pagebreak -->

RM0456 Rev 6

RM0456

USB on-the-go full-speed (OTG_FS)
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

RM0456 Rev 6

<!-- pagebreak -->

3291

USB on-the-go full-speed (OTG_FS)

RM0456

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

Application requirements 1, 2, 3, and 4 of Generic non-periodic IN data transfers on
page 3278 also apply to periodic IN data transfers, except for a slight modification of
requirement 2.
–

<!-- pagebreak -->

The application can only transmit multiples of maximum-packet-size data packets
or multiples of maximum-packet-size packets, plus a short packet at the end. To

RM0456 Rev 6

RM0456

USB on-the-go full-speed (OTG_FS)
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

3291

USB on-the-go full-speed (OTG_FS)

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

USB on-the-go full-speed (OTG_FS)
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

3291

USB on-the-go full-speed (OTG_FS)

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

72.16.6

Worst case response time
When the OTG_FS controller acts as a device, there is a worst case response time for any
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

USB on-the-go full-speed (OTG_FS)
Once the MAC receives an IN token, this information (token received) is synchronized to the
AHB clock by the PFC (the PFC runs on the AHB clock). The PFC then reads the data from
the SPRAM and writes them into the dual clock source buffer. The MAC then reads the data
out of the source buffer (4 deep).
If the AHB is running at a higher frequency than the PHY, the application can use a smaller
value for TRDT (in OTG_GUSBCFG).
Figure 913 has the following signals:
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

To calculate the value of TRDT, refer to Table 758: TRDT values.
Figure 913. TRDT max timing case

RM0456 Rev 6

<!-- pagebreak -->

3291

USB on-the-go full-speed (OTG_FS)

72.16.7

RM0456

OTG programming model
The OTG_FS controller is an OTG device supporting HNP and SRP. When the core is
connected to an “A” plug, it is referred to as an A-device. When the core is connected to a
“B” plug it is referred to as a B-device. In host mode, the OTG_FS controller turns off VBUS
to conserve power. SRP is a method by which the B-device signals the A-device to turn on
VBUS power. A device must perform both data-line pulsing and VBUS pulsing, but a host can
detect either data-line pulsing or VBUS pulsing for SRP. HNP is a method by which the Bdevice negotiates and switches to host role. In Negotiated mode after HNP, the B-device
suspends the bus and reverts to the device role.

A-device session request protocol
The application must set the SRP-capable bit in the core USB configuration register. This
enables the OTG_FS controller to detect SRP as an A-device.
Figure 914. A-device SRP
Suspend

6
1

DRV_VBUS

5

2
VBUS_VALID

VBUS pulsing

A_VALID

3

D+

D-

4
Data line pulsing

7
Connect

Low
ai15681c

1. DRV_VBUS = VBUS drive signal to the PHY
VBUS_VALID = VBUS valid signal from PHY
A_VALID = A-peripheral VBUS level signal to PHY
D+ = Data plus line
D- = Data minus line

The following points refer and describe the signal numeration shown in the Figure 914:

<!-- pagebreak -->

1.

To save power, the application suspends and turns off port power when the bus is idle
by writing the port suspend and port power bits in the host port control and status
register.

2.

PHY indicates port power off by deasserting the VBUS_VALID signal.

3.

The device must detect SE0 for at least 2 ms to start SRP when VBUS power is off.

4.

To initiate SRP, the device turns on its data line pull-up resistor for 5 to 10 ms. The
OTG_FS controller detects data-line pulsing.

5.

The device drives VBUS above the A-device session valid (2.0 V minimum) for VBUS
pulsing.
The OTG_FS controller interrupts the application on detecting SRP. The session

RM0456 Rev 6

RM0456

USB on-the-go full-speed (OTG_FS)
request detected bit is set in Global interrupt status register (SRQINT set in
OTG_GINTSTS).
6.

The application must service the session request detected interrupt and turn on the
port power bit by writing the port power bit in the host port control and status register.
The PHY indicates port power-on by asserting the VBUS_VALID signal.

7.

When the USB is powered, the device connects, completing the SRP process.

B-device session request protocol
The application must set the SRP-capable bit in the core USB configuration register. This
enables the OTG_FS controller to initiate SRP as a B-device. SRP is a means by which the
OTG_FS controller can request a new session from the host.
Figure 915. B-device SRP
Suspend

6
1

VBUS_VALID

2

B_VALID

3
DISCHRG_VBUS
4
SESS_END
5

8
Data line pulsing

Connect

DP

DM

Low
7
VBUS pulsing

CHRG_VBUS

ai15682c

1. VBUS_VALID = VBUS valid signal from PHY
B_VALID = B-peripheral valid session to PHY
DISCHRG_VBUS = discharge signal to PHY
SESS_END = session end signal to PHY
CHRG_VBUS = charge VBUS signal to PHY
DP = Data plus line
DM = Data minus line

The following points refer and describe the signal numeration shown in the Figure 915:
1.

To save power, the host suspends and turns off port power when the bus is idle.
The OTG_FS controller sets the early suspend bit in the core interrupt register after 3
ms of bus idleness. Following this, the OTG_FS controller sets the USB suspend bit in
the core interrupt register.
The OTG_FS controller informs the PHY to discharge VBUS.

2.

The PHY indicates the session’s end to the device. This is the initial condition for SRP.
The OTG_FS controller requires 2 ms of SE0 before initiating SRP.
For a USB 1.1 full-speed serial transceiver, the application must wait until VBUS
discharges to 0.2 V after BSVLD (in OTG_GOTGCTL) is deasserted. This discharge

RM0456 Rev 6

<!-- pagebreak -->

3291

USB on-the-go full-speed (OTG_FS)

RM0456

time can be obtained from the transceiver vendor and varies from one transceiver to
another.
3.

The OTG_FS core informs the PHY to speed up VBUS discharge.

4.

The application initiates SRP by writing the session request bit in the OTG control and
status register. The OTG_FS controller perform data-line pulsing followed by VBUS
pulsing.

5.

The host detects SRP from either the data-line or VBUS pulsing, and turns on VBUS.
The PHY indicates VBUS power-on to the device.

6.

The OTG_FS controller performs VBUS pulsing.
The host starts a new session by turning on VBUS, indicating SRP success. The
OTG_FS controller interrupts the application by setting the session request success
status change bit in the OTG interrupt status register. The application reads the session
request success bit in the OTG control and status register.

7.

When the USB is powered, the OTG_FS controller connects, completing the SRP
process.

A-device host negotiation protocol
HNP switches the USB host role from the A-device to the B-device. The application must set
the HNP-capable bit in the core USB configuration register to enable the OTG_FS controller
to perform HNP as an A-device.
Figure 916. A-device HNP

1. DPPULLDOWN = signal from core to PHY to enable/disable the pull-down on the DP line inside the PHY.
DMPULLDOWN = signal from core to PHY to enable/disable the pull-down on the DM line inside the PHY.

The following points refer and describe the signal numeration shown in the Figure 916:
1.

<!-- pagebreak -->

The OTG_FS controller sends the B-device a SetFeature b_hnp_enable descriptor to
enable HNP support. The B-device’s ACK response indicates that the B-device
supports HNP. The application must set host Set HNP enable bit in the OTG control

RM0456 Rev 6

RM0456

USB on-the-go full-speed (OTG_FS)
and status register to indicate to the OTG_FS controller that the B-device supports
HNP.
2.

When it has finished using the bus, the application suspends by writing the port
suspend bit in the host port control and status register.

3.

When the B-device observes a USB suspend, it disconnects, indicating the initial
condition for HNP. The B-device initiates HNP only when it must switch to the host role;
otherwise, the bus continues to be suspended.
The OTG_FS controller sets the host negotiation detected interrupt in the OTG
interrupt status register, indicating the start of HNP.
The OTG_FS controller deasserts the DM pull down and DM pull down in the PHY to
indicate a device role. The PHY enables the OTG_DP pull-up resistor to indicate a
connect for B-device.
The application must read the current mode bit in the OTG control and status register
to determine device mode operation.

4.

The B-device detects the connection, issues a USB reset, and enumerates the
OTG_FS controller for data traffic.

5.

The B-device continues the host role, initiating traffic, and suspends the bus when
done.
The OTG_FS controller sets the early suspend bit in the core interrupt register after 3
ms of bus idleness. Following this, the OTG_FS controller sets the USB suspend bit in
the core interrupt register.

6.

In Negotiated mode, the OTG_FS controller detects the suspend, disconnects, and
switches back to the host role. The OTG_FS controller asserts the DM pull down and
DM pull down in the PHY to indicate its assumption of the host role.

7.

The OTG_FS controller sets the connector ID status change interrupt in the OTG
interrupt status register. The application must read the connector ID status in the OTG
control and status register to determine the OTG_FS controller operation as an Adevice. This indicates the completion of HNP to the application. The application must
read the Current mode bit in the OTG control and status register to determine host
mode operation.

8.

The B-device connects, completing the HNP process.

B-device host negotiation protocol
HNP switches the USB host role from B-device to A-device. The application must set the
HNP-capable bit in the core USB configuration register to enable the OTG_FS controller to
perform HNP as a B-device.

RM0456 Rev 6

<!-- pagebreak -->

3291

USB on-the-go full-speed (OTG_FS)

RM0456
Figure 917. B-device HNP

1. DPPULLDOWN = signal from core to PHY to enable/disable the pull-down on the DP line inside the PHY.
DMPULLDOWN = signal from core to PHY to enable/disable the pull-down on the DM line inside the PHY.

The following points refer and describe the signal numeration shown in the Figure 917:
1.

The A-device sends the SetFeature b_hnp_enable descriptor to enable HNP support.
The OTG_FS controller’s ACK response indicates that it supports HNP. The application
must set the device HNP enable bit in the OTG control and status register to indicate
HNP support.
The application sets the HNP request bit in the OTG control and status register to
indicate to the OTG_FS controller to initiate HNP.

2.

When it has finished using the bus, the A-device suspends by writing the port suspend
bit in the host port control and status register.
The OTG_FS controller sets the Early suspend bit in the core interrupt register after 3
ms of bus idleness. Following this, the OTG_FS controller sets the USB suspend bit in
the core interrupt register.
The OTG_FS controller disconnects and the A-device detects SE0 on the bus,
indicating HNP. The OTG_FS controller asserts the DP pull down and DM pull down in
the PHY to indicate its assumption of the host role.
The A-device responds by activating its OTG_DP pull-up resistor within 3 ms of
detecting SE0. The OTG_FS controller detects this as a connect.
The OTG_FS controller sets the host negotiation success status change interrupt in the
OTG interrupt status register, indicating the HNP status. The application must read the
host negotiation success bit in the OTG control and status register to determine host

<!-- pagebreak -->

RM0456 Rev 6

RM0456

USB on-the-go full-speed (OTG_FS)
negotiation success. The application must read the current Mode bit in the core
interrupt register (OTG_GINTSTS) to determine host mode operation.
3.

The application sets the reset bit (PRST in OTG_HPRT) and the OTG_FS controller
issues a USB reset and enumerates the A-device for data traffic.

4.

The OTG_FS controller continues the host role of initiating traffic, and when done,
suspends the bus by writing the port suspend bit in the host port control and status
register.

5.

In Negotiated mode, when the A-device detects a suspend, it disconnects and switches
back to the host role. The OTG_FS controller deasserts the DP pull down and DM pull
down in the PHY to indicate the assumption of the device role.

6.

The application must read the current mode bit in the core interrupt (OTG_GINTSTS)
register to determine the host mode operation.

7.

The OTG_FS controller connects, completing the HNP process.

RM0456 Rev 6

<!-- pagebreak -->

3291

USB on-the-go high-speed (OTG_HS)

73

RM0456

USB on-the-go high-speed (OTG_HS)
This section applies to STM32U59x/5Ax/5Fx/5Gx devices only.

73.1

Introduction
Portions Copyright (c) Synopsys, Inc. All rights reserved. Used with permission.
This section presents the architecture and the programming model of the OTG_HS
controller.
The following acronyms are used throughout the section:
FS

Full-speed

LS

Low-speed

HS

High-speed

MAC

Media access controller

OTG

On-the-go

PFC

Packet FIFO controller

PHY

Physical layer

USB

Universal serial bus

UTMI

USB 2.0 Transceiver Macrocell interface (UTMI)

LPM

Link power management

BCD

Battery charging detector

HNP

Host negotiation protocol

SRP

Session request protocol

References are made to the following documents:
•

USB On-The-Go Supplement, Revision 2.0

•

Universal Serial Bus Revision 2.0 Specification

•

USB 2.0 Link Power Management Addendum Engineering Change Notice to the USB
2.0 specification, July 16, 2007

•

Errata for USB 2.0 ECN: Link Power Management (LPM) - 7/2007

•

Battery Charging Specification, Revision 1.2

The USB OTG is a dual-role device (DRD) controller that supports both device and host
functions and is fully compliant with the On-The-Go Supplement to the USB 2.0
Specification. It can also be configured as a host-only or device-only controller, fully
compliant with the USB 2.0 Specification. OTG_HS supports the speeds defined in the
Table 761: OTG_HS speeds supported below. The only external device required is a charge
pump for VBUS in OTG mode.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

USB on-the-go high-speed (OTG_HS)
Table 761. OTG_HS speeds supported
-

73.2

HS (480 Mbit/s)

FS (12 Mbit/s)

LS (1.5 Mbit/s)

Host mode

X

X

X

Device mode

X

X

-

OTG_HS main features
The main features can be divided into three categories: general, host-mode, and
device-mode features.

73.2.1

General features
The OTG_HS interface general features are the following:
•

It is USB-IF certified to the Universal Serial Bus Specification Rev 2.0.

•

OTG_HS supports the following PHY interfaces:
–

•

•

–

Integrated support for A-B device identification (ID line)

–

It allows host to turn VBUS off to conserve battery power in OTG applications.

–

It supports OTG monitoring of VBUS levels with internal comparators.

It is software-configurable to operate as:
–

•

A UTMI interface for internal HS PHY

It includes support (PHY) for the optional On-The-Go (OTG) protocol detailed
in the On-The-Go Supplement Rev 2.0 specification

USB On-The-Go Full-Speed Dual Role device

It supports HS SOF and LS Keep-alives with
–

SOF pulse PAD connectivity

–

SOF pulse internal connection to timer (TIMx)

–

Configurable framing period

–

Configurable end of frame interrupt

•

OTG_HS embeds an internal DMA with thresholding support and software selectable
AHB burst type in DMA mode.

•

It includes power saving features such as system stop during USB suspend, switch-off
of clock domains internal to the digital core, PHY and DFIFO power management.

•

It features a dedicated RAM of 4 Kbytes with advanced FIFO control:
–

Configurable partitioning of RAM space into different FIFOs for flexible and
efficient use of RAM

–

Each FIFO can hold multiple packets.

–

Dynamic memory allocation

–

Configurable FIFO sizes that are not powers of 2 to allow the use of contiguous
memory locations

•

It guarantees max USB bandwidth for up to one frame (1 ms) without system
intervention.

•

It supports charging port detection as described in Battery Charging Specification
Revision 1.2.

RM0456 Rev 6

<!-- pagebreak -->

3460

USB on-the-go high-speed (OTG_HS)

73.2.2

RM0456

Host-mode features
The OTG_HS interface main features and requirements in host-mode are the following:
•

External charge pump for VBUS voltage generation

•

Up to 16 host channels (pipes): each channel is dynamically reconfigurable to allocate
any type of USB transfer.

•

Built-in hardware scheduler holding:

•

73.2.3

–

Up to 16 interrupt plus isochronous transfer requests in the periodic hardware
queue

–

Up to 16 control plus bulk transfer requests in the non-periodic hardware queue

Management of a shared Rx FIFO, a periodic Tx FIFO and a nonperiodic Tx FIFO for
efficient usage of the USB data RAM.

Peripheral-mode features
The OTG_HS interface main features in peripheral-mode are the following:

73.3

•

1 bidirectional control endpoint0

•

8 IN endpoints (EPs) configurable to support bulk, interrupt or isochronous transfers

•

8 OUT endpoints configurable to support bulk, interrupt or isochronous transfers

•

Management of a shared Rx FIFO and a Tx-OUT FIFO for efficient usage of the USB
data RAM

•

Management of up to 9 dedicated Tx-IN FIFOs (one for each active IN EP) to put less
load on the application

•

Support for the soft disconnect feature.

OTG_HS implementation
Table 762. OTG_HS implementation(1)
USB features

OTG_HS for STM32U59x/5Ax/5Fx/5Gx

Device bidirectional endpoints (including EP0)

9

Host mode channels

16

Size of dedicated SRAM

<!-- pagebreak -->

4 Kbytes

USB 2.0 link power management (LPM) support

X

OTG revision supported

2.0

Battery charging detection (BCD) support

X

Integrated PHY

HS

HNP and SRP support

-

SRP: OTG_DVBUSDIS register

-

SRP: OTG_DVBUSPULSE register

-

HNP: OTG_GOTGCTL bits 11:8

-

SRP: OTG_GOTGCTL bits 1:0

-

HNP/SRP: OTG_GOTGINT bits 19; 17; 9:8

-

RM0456 Rev 6

RM0456

USB on-the-go high-speed (OTG_HS)
Table 762. OTG_HS implementation(1) (continued)
USB features

OTG_HS for STM32U59x/5Ax/5Fx/5Gx

HNP: OTG_GUSBCFG bit 9

-

SRP: OTG_GUSBCFG bit 8

-

OTG_PCGCCTL1 register

X

1. “X” = supported, “-” = not supported, “FS” = supported in FS mode, “HS” = supported in HS mode.

73.4

OTG_HS functional description

73.4.1

OTG_HS block diagram
Figure 918. OTG_HS high-speed block diagram

APB (peripheral bus)

SYSCFG

CPU

Memory

AHB (application bus)

UTMI+

OTG_HS
(USB OTG HS core)

AHB master
interface

OTG detections

OTG_HS_DP
OTG_HS_DM

OTG_HS_ID
OTG_HS_VBUS

Interrupt: async wakeup

Interrupt: global

Peripheral 1

OTG HS PHY

NVIC

AHB slave
interface

Peripheral 2

Data FIFO RAM
interface

OTG_HS_SOF

Data FIFO
single port RAM
(SPRAM)
MSv66254V1

RM0456 Rev 6

<!-- pagebreak -->

3460

USB on-the-go high-speed (OTG_HS)

73.4.2

RM0456

OTG_HS pin and internal signals
Table 763. OTG_HS input/output pins
Signal name

Signal type

Description

OTG_HS_DP

Digital input/output USB OTG D+ line

OTG_HS_DM

Digital input/output USB OTG D- line

OTG_HS_ID

Digital input

USB OTG ID

OTG_HS_VBUS

Analog input

USB OTG VBUS

OTG_HS_SOF

Digital output

USB OTG Start Of Frame (visibility)

Table 764. OTG_HS input/output signals
Signal name

73.4.3

Signal type

Description

usb_sof

Digital output

USB OTG start-of-frame event for on chip peripherals

usb_wkup

Digital output

USB OTG wake-up event output

usb_gbl_it

Digital output

USB OTG global interrupt

OTG_HS core
The OTG_HS receives the 60 MHz clock from the reset and clock controller (RCC). This is
typically generated in the PLL associated with the HS PHY and enabled in the RCC.
This clock is used for driving the 60 MHz domain at high-speed (480 Mbit/s) and must be
enabled prior to configuring the OTG core.
The CPU reads and writes from/to the OTG core registers through the AHB peripheral bus.
It is informed of USB events through the single USB OTG interrupt line described in
Section 73.12: OTG_HS interrupts.
The CPU submits data over the USB by writing 32-bit words to dedicated OTG locations
(push registers). The data are then automatically stored into Tx-data FIFOs configured
within the USB data RAM. There is one Tx FIFO push register for each in-endpoint
(peripheral mode) or out-channel (host mode).
The CPU receives the data from the USB by reading 32-bit words from dedicated OTG
addresses (pop registers). The data are then automatically retrieved from a shared Rx FIFO
configured within the 4-Kbyte USB data RAM. There is one Rx FIFO pop register for each
out-endpoint or in-channel.
The USB protocol layer is driven by the serial interface engine (SIE) and serialized over the
USB by the transceiver module within the on-chip physical layer (PHY).

Caution:

<!-- pagebreak -->

To guarantee a correct operation for the USB OTG_HS peripheral, the AHB frequency must
be higher than 30 MHz.

RM0456 Rev 6

RM0456

73.4.4

USB on-the-go high-speed (OTG_HS)

OTG detections
Additionally the OTG_HS uses the following functions:
•

Integrated ID pull-up resistor used to sample the ID line for A/B device identification.

•

VBUS sensing comparators with hysteresis used to detect VBUS valid, A-B session valid
and session-end voltage thresholds. They are used to detect valid startup and end-ofsession conditions, and constantly monitor the VBUS supply during USB operations.

73.4.5

High-speed OTG PHY connected to OTG_HS

Note:

Refer to implementation table to determine if an HS PHY is embedded.
The USB OTG core includes an internal UTMI interface which is connected to the
embedded HS PHY (see Section 73.4.1: OTG_HS block diagram).

73.4.6

Battery charging detection
Support is available for the USB battery charging specification (v1.2).
The OTG_HS includes the necessary circuits to enable battery charging detection (portable
device).
There is also a possibility to behave as a charging data port (the port advertises 1.5 A
available on it's VBUS pin).
Various fields control this functions and are implemented in register OTG_GCCFG.

OTG_HS dual role device (DRD)
Figure 919. OTG_HS A-B device connection
VDD
5 V to VDD
Voltage
regulator(1)
VDD

STM32

GPIO

GPIO + IRQ

EN

STMPS2141STR
Current-limited 5 V Pwr
Overcurrent power distribution
switch(2)

VBUS
DM
OSC_IN

DP
ID

OSC_OUT

VBUS
DM
DP
ID
VSS

USBmicro-AB connector

73.5

MSv36917V2

1. External voltage regulator only needed when building a VBUS powered device.
2. STMPS2141STR needed only if the application has to support a VBUS powered device. A basic power
switch can be used if 5 V are available on the application board.

RM0456 Rev 6

<!-- pagebreak -->

3460

USB on-the-go high-speed (OTG_HS)

73.5.1

RM0456

ID line detection
The host or peripheral (the default) role is assumed depending on the ID input pin. The ID
line status is determined on plugging in the USB cable, depending on whether a MicroA or
MicroB plug is connected to the micro-AB receptacle.

73.6

•

If the B-side of the USB cable is connected with a floating ID wire, the integrated pullup resistor detects a high ID level and the default peripheral role is confirmed. In this
configuration the OTG_HS complies with the standard FSM described in section 4.2.4:
ID pin of the On-the-Go specification Rev2.0, supplement to the USB2.0.

•

If the A-side of the USB cable is connected with a grounded ID, the OTG_HS issues an
ID line status change interrupt (CIDSCHG bit in OTG_GINTSTS) for host software
initialization, and automatically switches to the host role. In this configuration the
OTG_HS complies with the standard FSM described by section 4.2.4: ID pin of the Onthe-Go specification Rev2.0, supplement to the USB2.0.

OTG_HS as a USB peripheral
This section gives the functional description of the OTG_HS in the USB peripheral mode.
The OTG_HS works as an USB peripheral in the following circumstances:
•

OTG B-Peripheral
–

•

B-device
–

•

<!-- pagebreak -->

If the ID line is present, functional and connected to the B-side of the USB cable.

Peripheral only
–

Note:

OTG B-device default state if B-side of USB cable is plugged in

The force device mode bit (FDMOD) in the Section 73.14.4: OTG USB
configuration register (OTG_GUSBCFG) is set to 1, forcing the OTG_HS core to
work as an USB peripheral-only. In this case, the ID line is ignored even if it is
present on the USB connector.

To build a bus-powered device implementation in case of the B-device or peripheral-only
configuration, an external regulator has to be added, that generates the necessary powersupply from VBUS.

RM0456 Rev 6

RM0456

USB on-the-go high-speed (OTG_HS)
Figure 920. OTG_HS peripheral-only connection
VDD
5 V to VDD
Voltage
regulator
VDD
GPIO
GPIO + IRQ

EN

STMPS2141STR
Current-limited 5 V Pwr
Overcurrent power distribution
switch(1)

DM
OSC_IN

DP

OSC_OUT

VSS

USB micro connector

VBUS

MSv36916V2

1. Use a regulator to build a bus-powered device.

73.6.1

Peripheral states
Powered state
The VBUS input detects the B-session valid voltage by which the USB peripheral is allowed
to enter the powered state (see USB2.0 section 9.1). The OTG_HS then automatically
connects the DP pull-up resistor to signal full-speed device connection to the host and
generates the session request interrupt (SRQINT bit in OTG_GINTSTS) to notify the
powered state.
The VBUS input also ensures that valid VBUS levels are supplied by the host during USB
operations. If a drop in VBUS below B-session valid happens to be detected (for instance
because of a power disturbance or if the host port has been switched off), the OTG_HS
automatically disconnects and the session end detected (SEDET bit in OTG_GOTGINT)
interrupt is generated to notify that the OTG_HS has exited the powered state.
In the powered state, the OTG_HS expects to receive some reset signaling from the host.
No other USB operation is possible. When a reset signaling is received the reset detected
interrupt (USBRST in OTG_GINTSTS) is generated. When the reset signaling is complete,
the enumeration done interrupt (ENUMDNE bit in OTG_GINTSTS) is generated and the
OTG_HS enters the Default state.

Soft disconnect
The powered state can be exited by software with the soft disconnect feature. The DP pullup resistor is removed by setting the soft disconnect bit in the device control register (SDIS
bit in OTG_DCTL), causing a device disconnect detection interrupt on the host side even
though the USB cable was not really removed from the host port.

Default state
In the Default state the OTG_HS expects to receive a SET_ADDRESS command from the
host. No other USB operation is possible. When a valid SET_ADDRESS command is

RM0456 Rev 6

<!-- pagebreak -->

3460

USB on-the-go high-speed (OTG_HS)

RM0456

decoded on the USB, the application writes the corresponding number into the device
address field in the device configuration register (DAD bit in OTG_DCFG). The OTG_HS
then enters the address state and is ready to answer host transactions at the configured
USB address.

Suspended state
The OTG_HS peripheral constantly monitors the USB activity. After counting 3 ms of USB
idleness, the early suspend interrupt (ESUSP bit in OTG_GINTSTS) is issued, and
confirmed 3 ms later, if appropriate, by the suspend interrupt (USBSUSP bit in
OTG_GINTSTS). The device suspend bit is then automatically set in the device status
register (SUSPSTS bit in OTG_DSTS) and the OTG_HS enters the suspended state.
The suspended state may optionally be exited by the device itself. In this case the
application sets the remote wake-up signaling bit in the device control register (RWUSIG bit
in OTG_DCTL) and clears it after 1 to 15 ms.
When a resume signaling is detected from the host, the resume interrupt (WKUPINT bit in
OTG_GINTSTS) is generated and the device suspend bit is automatically cleared.

73.6.2

Peripheral endpoints
The OTG_HS core instantiates the following USB endpoints:
•

•

<!-- pagebreak -->

Control endpoint 0:
–

Bidirectional and handles control messages only

–

Separate set of registers to handle in and out transactions

–

Dedicated control (OTG_DIEPCTL0/OTG_DOEPCTL0), transfer configuration
(OTG_DIEPTSIZ0/OTG_DOEPTSIZ0), and status-interrupt
(OTG_DIEPINT0/)OTG_DOEPINT0) registers. The available set of bits inside the
control and transfer size registers slightly differs from that of other endpoints

eight IN endpoints
–

Each of them can be configured to support the isochronous, bulk or interrupt
transfer type

–

Each of them has dedicated control (OTG_DIEPCTLx), transfer configuration
(OTG_DIEPTSIZx), and status-interrupt (OTG_DIEPINTx) registers

–

The device IN endpoints common interrupt mask register (OTG_DIEPMSK) is
available to enable/disable a single kind of endpoint interrupt source on all of the
IN endpoints (EP0 included)

–

Support for incomplete isochronous IN transfer interrupt (IISOIXFR bit in
OTG_GINTSTS), asserted when there is at least one isochronous IN endpoint on

RM0456 Rev 6

RM0456

USB on-the-go high-speed (OTG_HS)
which the transfer is not completed in the current frame. This interrupt is asserted
along with the end of periodic frame interrupt (OTG_GINTSTS/EOPF).
•

eight OUT endpoints
–

Each of them can be configured to support the isochronous, bulk or interrupt
transfer type

–

Each of them has a dedicated control (OTG_DOEPCTLx), transfer configuration
(OTG_DOEPTSIZx) and status-interrupt (OTG_DOEPINTx) register

–

Device OUT endpoints common interrupt mask register (OTG_DOEPMSK) is
available to enable/disable a single kind of endpoint interrupt source on all of the
OUT endpoints (EP0 included)

–

Support for incomplete isochronous OUT transfer interrupt (INCOMPISOOUT bit
in OTG_GINTSTS), asserted when there is at least one isochronous OUT
endpoint on which the transfer is not completed in the current frame. This interrupt
is asserted along with the end of periodic frame interrupt (OTG_GINTSTS/EOPF).

Endpoint control
•

The following endpoint controls are available to the application through the device
endpoint-x IN/OUT control register (OTG_DIEPCTLx/OTG_DOEPCTLx):
–

endpoint enable/disable

–

endpoint activate in current configuration

–

program USB transfer type (isochronous, bulk, interrupt)

–

program supported packet size

–

program Tx FIFO number associated with the IN endpoint

–

program the expected or transmitted data0/data1 PID (bulk/interrupt only)

–

program the even/odd frame during which the transaction is received or
transmitted (isochronous only)

–

optionally program the NAK bit to always negative-acknowledge the host
regardless of the FIFO status

–

optionally program the STALL bit to always stall host tokens to that endpoint

–

optionally program the SNOOP mode for OUT endpoint not to check the CRC field
of received data

Endpoint transfer
The device endpoint-x transfer size registers (OTG_DIEPTSIZx/OTG_DOEPTSIZx) allow
the application to program the transfer size parameters and read the transfer status.
Programming must be done before setting the endpoint enable bit in the endpoint control
register. Once the endpoint is enabled, these fields are read-only as the OTG_HS core
updates them with the current transfer status.
The following transfer parameters can be programmed:
•

transfer size in bytes

•

number of packets that constitute the overall transfer size

Endpoint status/interrupt
The device endpoint-x interrupt registers (OTG_DIEPINTx/OTG_DOPEPINTx) indicate the
status of an endpoint with respect to USB- and AHB-related events. The application must
read these registers when the OUT endpoint interrupt bit or the IN endpoint interrupt bit in

RM0456 Rev 6

<!-- pagebreak -->

3460

USB on-the-go high-speed (OTG_HS)

RM0456

the core interrupt register (OEPINT bit in OTG_GINTSTS or IEPINT bit in OTG_GINTSTS,
respectively) is set. Before the application can read these registers, it must first read the
device all endpoints interrupt (OTG_DAINT) register to get the exact endpoint number for
the device endpoint-x interrupt register. The application must clear the appropriate bit in this
register to clear the corresponding bits in the OTG_DAINT and OTG_GINTSTS registers
The peripheral core provides the following status checks and interrupt generation:

73.7

•

transfer completed interrupt, indicating that data transfer was completed on both the
application (AHB) and USB sides

•

setup stage has been done (control-out only)

•

associated transmit FIFO is half or completely empty (in endpoints)

•

NAK acknowledge has been transmitted to the host (isochronous-in only)

•

IN token received when Tx FIFO was empty (bulk-in/interrupt-in only)

•

Out token received when endpoint was not yet enabled

•

babble error condition has been detected

•

endpoint disable by application is effective

•

endpoint NAK by application is effective (isochronous-in only)

•

more than 3 back-to-back setup packets were received (control-out only)

•

timeout condition detected (control-in only)

•

isochronous out packet has been dropped, without generating an interrupt

OTG_HS as a USB host
This section gives the functional description of the OTG_HS in the USB host mode. The
OTG_HS works as a USB host in the following circumstances:
Automatic host mode direct from ID pin:
The ID pin is not always available, refer to product datasheet for available pins.
•

Use cases:
–

•

Micro connector: A-Device with a Micro-A plug inserted, and connector ID pin
connected to the STM32 ID pin which will then be detected in a low state.

Note that the integrated pull-down resistors are automatically set on the DP/DM lines.

Manual forcing of host mode when not possible via ID pin:
The force host mode bit (FHMOD) in the OTG USB configuration register
(OTG_GUSBCFG) forces the OTG_HS core to work as a USB host-only when required.
•

•
Note:

<!-- pagebreak -->

Use cases:
–

Standard-A connector which always implies Host mode.

–

Micro connector, so ID present on connector, but ID pin not available in pinout, at
A-plug insertion (detected by another means, for example: GPIO).

–

A Type-C connector, when host functionality is determined at cable attachment or
via power delivery messages.

Note that the integrated pull-down resistors are set on the DP/DM lines by setting bit
FORCEHOSTPD.

On-chip 5 V VBUS generation is not supported. For this reason, a charge pump or, if 5 V are
available on the application board, a basic power switch must be added externally to drive

RM0456 Rev 6

RM0456

USB on-the-go high-speed (OTG_HS)
the 5 V VBUS line. The external charge pump can be driven by any GPIO output. This is
required for the OTG A-host, A-device and host-only configurations.
Figure 921. OTG_HS host-only connection
VDD
5V

GPIO + IRQ

STMPS2141STR
5 V Pwr
Current-limited
Overcurrent power distribution
switch(2)
VBUS
DM

OSC_IN

DP
VSS

OSC_OUT

USB Std-A connector

GPIO

EN

MSv36915V2

1. VDD range is between 2 V and 3.6 V.

73.7.1

USB host states
Host port power
On-chip 5 V VBUS generation is not supported. For this reason, a charge pump or, if 5 V are
available on the application board, a basic power switch, must be added externally to drive
the 5 V VBUS line. The external charge pump can be driven by any GPIO output or via an
I2C interface connected to an external PMIC (power management IC). When the application
decides to power on VBUS, it must also set the port power bit in the host port control and
status register (PPWR bit in OTG_HPRT).

VBUS valid
In Host mode, the VBUS sensing pin does not need to be connected to VBUS.
The charge pump overcurrent flag can also be used to prevent electrical damage. Connect
the overcurrent flag output from the charge pump to any GPIO input and configure it to
generate a port interrupt on the active level. The overcurrent ISR must promptly disable the
VBUS generation and clear the port power bit.

Host detection of a peripheral connection
USB peripherals or B-device are detected as soon as they are connected. The OTG_HS
core issues a host port interrupt triggered by the device connected bit in the host port control
and status (PCDET bit in OTG_HPRT).

Host detection of peripheral a disconnection
The peripheral disconnection event triggers the disconnect detected interrupt (DISCINT bit
in OTG_GINTSTS).

RM0456 Rev 6

<!-- pagebreak -->

3460

USB on-the-go high-speed (OTG_HS)

RM0456

Host enumeration
After detecting a peripheral connection the host must start the enumeration process by
sending USB reset and configuration commands to the new peripheral.
The application drives a USB reset signaling (single-ended zero) over the USB by keeping
the port reset bit set in the host port control and status register (PRST bit in OTG_HPRT) for
a minimum of 10 ms and a maximum of 20 ms. The application takes care of the timing
count and then of clearing the port reset bit.
Once the USB reset sequence has completed, the host port interrupt is triggered by the port
enable/disable change bit (PENCHNG bit in OTG_HPRT). This informs the application that
the speed of the enumerated peripheral can be read from the port speed field in the host
port control and status register (PSPD bit in OTG_HPRT) and that the host is starting to
drive SOFs (FS) or Keep alives (LS). The host is now ready to complete the peripheral
enumeration by sending peripheral configuration commands.

Host suspend
The application decides to suspend the USB activity by setting the port suspend bit in the
host port control and status register (PSUSP bit in OTG_HPRT). The OTG_HS core stops
sending SOFs and enters the suspended state.
The suspended state can be optionally exited on the remote device’s initiative (remote
wake-up). In this case the remote wake-up interrupt (WKUPINT bit in OTG_GINTSTS) is
generated upon detection of a remote wake-up signaling, the port resume bit in the host port
control and status register (PRES bit in OTG_HPRT) self-sets, and resume signaling is
automatically driven over the USB. The application must time the resume window and then
clear the port resume bit to exit the suspended state and restart the SOF.
If the suspended state is exited on the host initiative, the application must set the port
resume bit to start resume signaling on the host port, time the resume window and finally
clear the port resume bit.

73.7.2

Host channels
The OTG_HS core instantiates 16 host channels. Each host channel supports an USB host
transfer (USB pipe). The host is not able to support more than 16 transfer requests at the
same time. If more than 16 transfer requests are pending from the application, the host
controller driver (HCD) must re-allocate channels when they become available from
previous duty, that is, after receiving the transfer completed and channel halted interrupts.
Each host channel can be configured to support in/out and any type of periodic/nonperiodic
transaction. Each host channel makes us of dedicated control (OTG_HCCHARx), transfer
configuration (OTG_HCTSIZx) and status/interrupt (OTG_HCINTx) registers with
associated mask (OTG_HCINTMSKx) registers.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

USB on-the-go high-speed (OTG_HS)

Host channel control
•

The following host channel controls are available to the application through the host
channel-x characteristics register (OTG_HCCHARx):
–

Channel enable/disable

–

Program the HS/FS/LS speed of target USB peripheral

–

Program the address of target USB peripheral

–

Program the endpoint number of target USB peripheral

–

Program the transfer IN/OUT direction

–

Program the USB transfer type (control, bulk, interrupt, isochronous)

–

Program the maximum packet size (MPS)

–

Program the periodic transfer to be executed during odd/even frames

Host channel transfer
The host channel transfer size registers (OTG_HCTSIZx) allow the application to program
the transfer size parameters, and read the transfer status. Programming must be done
before setting the channel enable bit in the host channel characteristics register. Once the
endpoint is enabled the packet count field is read-only as the OTG_HS core updates it
according to the current transfer status.
•

The following transfer parameters can be programmed:
–

transfer size in bytes

–

number of packets making up the overall transfer size

–

initial data PID

Host channel status/interrupt
The host channel-x interrupt register (OTG_HCINTx) indicates the status of an endpoint
with respect to USB- and AHB-related events. The application must read these register
when the host channels interrupt bit in the core interrupt register (HCINT bit in
OTG_GINTSTS) is set. Before the application can read these registers, it must first read the
host all channels interrupt (OTG_HAINT) register to get the exact channel number for the
host channel-x interrupt register. The application must clear the appropriate bit in this
register to clear the corresponding bits in the OTG_HAINT and OTG_GINTSTS registers.

RM0456 Rev 6

<!-- pagebreak -->

3460

USB on-the-go high-speed (OTG_HS)

RM0456

The mask bits for each interrupt source of each channel are also available in the
OTG_HCINTMSKx register.
•

73.7.3

The host core provides the following status checks and interrupt generation:
–

Transfer completed interrupt, indicating that the data transfer is complete on both
the application (AHB) and USB sides

–

Channel has stopped due to transfer completed, USB transaction error or disable
command from the application

–

Associated transmit FIFO is half or completely empty (IN endpoints)

–

ACK response received

–

NAK response received

–

STALL response received

–

USB transaction error due to CRC failure, timeout, bit stuff error, false EOP

–

Babble error

–

frame overrun

–

data toggle error

Host scheduler
The host core features a built-in hardware scheduler which is able to autonomously re-order
and manage the USB transaction requests posted by the application. At the beginning of
each frame the host executes the periodic (isochronous and interrupt) transactions first,
followed by the nonperiodic (control and bulk) transactions to achieve the higher level of
priority granted to the isochronous and interrupt transfer types by the USB specification.
The host processes the USB transactions through request queues (one for periodic and one
for nonperiodic). Each request queue can hold up to 8 entries. Each entry represents a
pending transaction request from the application, and holds the IN or OUT channel number
along with other information to perform a transaction on the USB. The order in which the
requests are written to the queue determines the sequence of the transactions on the USB
interface.
At the beginning of each frame, the host processes the periodic request queue first, followed
by the nonperiodic request queue. The host issues an incomplete periodic transfer interrupt
(IPXFR bit in OTG_GINTSTS) if an isochronous or interrupt transaction scheduled for the
current frame is still pending at the end of the current frame. The OTG_HS core is fully
responsible for the management of the periodic and nonperiodic request queues.The
periodic transmit FIFO and queue status register (OTG_HPTXSTS) and nonperiodic
transmit FIFO and queue status register (OTG_HNPTXSTS) are read-only registers which
can be used by the application to read the status of each request queue. They contain:
•

The number of free entries currently available in the periodic (nonperiodic) request
queue (8 max)

•

Free space currently available in the periodic (nonperiodic) Tx FIFO (out-transactions)

•

IN/OUT token, host channel number and other status information.

As request queues can hold a maximum of eight entries each, the application can push to
schedule host transactions in advance with respect to the moment they physically reach the
SB for a maximum of eight pending periodic transactions plus 8 pending non-periodic
transactions.
To post a transaction request to the host scheduler (queue) the application must check that
there is at least 1 entry available in the periodic (nonperiodic) request queue by reading the

<!-- pagebreak -->

RM0456 Rev 6

RM0456

USB on-the-go high-speed (OTG_HS)
PTXQSAV bits in the OTG_HNPTXSTS register or NPTQXSAV bits in the
OTG_HNPTXSTS register.

73.8

OTG_HS SOF trigger
Figure 922. SOF connectivity (SOF trigger output to TIM and ITR1 connection)

SOF pulse output, to
external audio control
VBUS
ITR1

SOF pulse

DD+

TIM

SOFgen

ID

USB micro-AB connector

STM32

VSS

MSv36914V1

The OTG_HS core provides means to monitor, track and configure SOF framing in the host
and peripheral, as well as an SOF pulse output connectivity feature.
Such utilities are especially useful for adaptive audio clock generation techniques, where
the audio peripheral needs to synchronize to the isochronous stream provided by the PC, or
the host needs to trim its framing rate according to the requirements of the audio peripheral.

73.8.1

Host SOFs
In host mode the number of PHY clocks occurring between the generation of two
consecutive SOF (HS/FS) or Keep-alive (LS) tokens is programmable in the host frame
interval register (HFIR), thus providing application control over the SOF framing period. An
interrupt is generated at any start of frame (SOF bit in OTG_GINTSTS). The current frame
number and the time remaining until the next SOF are tracked in the host frame number
register (HFNUM).
A SOF pulse signal, is generated at any SOF starting token and with a width of 20 HCLK
cycles. The SOF pulse is also internally connected to the input trigger of the timer, so that
the input capture feature, the output compare feature and the timer can be triggered by the
SOF pulse.

73.8.2

Peripheral SOFs
In device mode, the start of frame interrupt is generated each time an SOF token is received
on the USB (SOF bit in OTG_GINTSTS). The corresponding frame number can be read
from the device status register (FNSOF bit in OTG_DSTS). A SOF pulse signal with a width
of 20 HCLK cycles is also generated.The SOF pulse signal is also internally connected to
the TIM input trigger, so that the input capture feature, the output compare feature and the
timer can be triggered by the SOF pulse.

RM0456 Rev 6

<!-- pagebreak -->

3460

USB on-the-go high-speed (OTG_HS)

RM0456

The end of periodic frame interrupt (OTG_GINTSTS/EOPF) is used to notify the application
when 80%, 85%, 90% or 95% of the time frame interval elapsed depending on the periodic
frame interval field in the device configuration register (PFIVL bit in OTG_DCFG). This
feature can be used to determine if all of the isochronous traffic for that frame is complete.

73.9

OTG_HS low-power modes
Table 765 below defines the STM32 low power modes and their compatibility with the OTG.
Table 765. Compatibility of STM32 low power modes with the OTG

Mode

Description

USB compatibility

Run

MCU fully active

Required when USB not in
suspend state.

Sleep

USB suspend exit causes the device to exit Sleep mode. Peripheral
registers content is kept.

Available while USB is in
suspend state.

Stop

USB suspend exit causes the device to exit Stop mode. Peripheral
registers content is kept(1).

Available while USB is in
suspend state.

Standby

Powered-down. The peripheral must be reinitialized after exiting
Standby mode.

Not compatible with USB
applications.

1. Within Stop mode there are different possible settings. Some restrictions may also exist, refer to Section 10: Power control
(PWR) to understand which (if any) restrictions apply when using OTG.

The following bits and procedures reduce power consumption.
The power consumption of the OTG PHY is controlled by the following bit in the general
core configuration register:
•

VBUS detection enable (OTG_GCCFG/VBDEN)
It switches on/off the VBUS sensing comparators associated with OTG operations

Power reduction techniques are available while in the USB suspended state, when the USB
session is not yet valid or the device is disconnected.
•

Stop PHY clock (STPPCLK bit in OTG_PCGCCTL)
When setting the stop PHY clock bit in the clock gating control register, most of the
transceiver is disabled, and only the part in charge of detecting the asynchronous
resume or remote wake-up event is kept alive.

•

Gate HCLK (GATEHCLK bit in OTG_PCGCCTL)
When setting the Gate HCLK bit in the clock gating control register, most of the system
clock domain internal to the OTG_HS core is switched off by clock gating. Only the
register read and write interface is kept alive. The dynamic power consumption due to
the USB clock switching activity is cut even if the system clock is kept running by the
application for other purposes.

•

USB system stop
When the OTG_HS is in the USB suspended state, the application may decide to
drastically reduce the overall power consumption by a complete shut down of all the
clock sources in the system. USB System Stop is activated by first setting the Stop

<!-- pagebreak -->

RM0456 Rev 6

RM0456

USB on-the-go high-speed (OTG_HS)
PHY clock bit and then configuring the system deep sleep mode in the power control
system module (PWR).
The OTG_HS core automatically reactivates both system and USB clocks by
asynchronous detection of remote wake-up (as an host) or resume (as a device)
signaling on the USB.
To save dynamic power, the USB data FIFO is clocked only when accessed by the OTG_HS
core.

73.10

OTG_HS Dynamic update of the OTG_HFIR register
The USB core embeds a dynamic trimming capability of micro-SOF framing period in host
mode allowing to synchronize an external device with the micro-SOF frames.
When the OTG_HFIR register is changed within a current micro-SOF frame, the SOF period
correction is applied in the next frame as described in Figure 923.
For a dynamic update, it is required to set RLDCTRL=1.
Figure 923. Updating OTG_HFIR dynamically (RLDCTRL = 1)
SOF reload

OTG_HFIR write

...

1
0
450
449

...

450

1
0
400
399

1
0
400
399

Frame timer

400

...

1
0
450
449

OTG_HFIR value

...

ai18440b

73.11

OTG_HS data FIFOs
The USB system features 4 Kbytes of dedicated RAM with a sophisticated FIFO control
mechanism. The packet FIFO controller module in the OTG_HS core organizes RAM space
into Tx FIFOs into which the application pushes the data to be temporarily stored before the
USB transmission, and into a single Rx FIFO where the data received from the USB are
temporarily stored before retrieval (popped) by the application. The number of instructed
FIFOs and how these are organized inside the RAM depends on the device’s role. In
peripheral mode an additional Tx FIFO is instructed for each active IN endpoint. Any FIFO
size is software configured to better meet the application requirements.

RM0456 Rev 6

<!-- pagebreak -->

3460

USB on-the-go high-speed (OTG_HS)

73.11.1

RM0456

Peripheral FIFO architecture
Figure 924. Device-mode FIFO address mapping and AHB FIFO access mapping
Single data
FIFO
IN endpoint Tx FIFO #x
DFIFO push access
from AHB

Dedicated Tx
FIFO #x control
(optional)
MAC pop

IN endpoint Tx FIFO #1
DFIFO push access
from AHB

Tx FIFO #x
packet

.
.
.

.
.
.

Dedicated Tx
FIFO #1 control
(optional)

Tx FIFO #1
packet

OTG_DIEPTXFx[31:16]
OTG_DIEPTXFx[15:0]
.
.
.
OTG_DIEPTXF1[31:16]
OTG_DIEPTXF1[15:0]

MAC pop
IN endpoint Tx FIFO #0
DFIFO push access
from AHB

Dedicated Tx
FIFO #0 control
(optional)

Tx FIFO #0
packet

OTG_DIEPTXF0[31:16]
OTG_DIEPTXF0[15:0]

MAC pop

Any OUT endpoint
DFIFO pop access
from AHB

Dedicated Tx
FIFO #1 control
(optional)

MAC push

Rx packets

OTG_GRXFSIZ[15:0]

A1=0 (Rx start address fixed
to 0)
MSv36929V1

Peripheral Rx FIFO
The OTG peripheral uses a single receive FIFO that receives the data directed to all OUT
endpoints. Received packets are stacked back-to-back until free space is available in the Rx
FIFO. The status of the received packet (which contains the OUT endpoint destination
number, the byte count, the data PID and the validity of the received data) is also stored by
the core on top of the data payload. When no more space is available, host transactions are
NACKed and an interrupt is received on the addressed endpoint. The size of the receive
FIFO is configured in the receive FIFO size register (OTG_GRXFSIZ).
The single receive FIFO architecture makes it more efficient for the USB peripheral to fill in
the receive RAM buffer:
•

All OUT endpoints share the same RAM buffer (shared FIFO)

•

The OTG_HS core can fill in the receive FIFO up to the limit for any host sequence of
OUT tokens

The application keeps receiving the Rx FIFO non-empty interrupt (RXFLVL bit in
OTG_GINTSTS) as long as there is at least one packet available for download. It reads the
packet information from the receive status read and pop register (OTG_GRXSTSP) and
finally pops data off the receive FIFO by reading from the endpoint-related pop address.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

USB on-the-go high-speed (OTG_HS)

Peripheral Tx FIFOs
The core has a dedicated FIFO for each IN endpoint. The application configures FIFO sizes
by writing the endpoint 0 transmit FIFO size register (OTG_DIEPTXF0) for IN endpoint0 and
the device IN endpoint transmit FIFOx registers (OTG_DIEPTXFx) for IN endpoint-x.

73.11.2

Host FIFO architecture
Figure 925. Host-mode FIFO address mapping and AHB FIFO access mapping
Single data
FIFO

Any periodic channel
DFIFO push access
from AHB

Periodic Tx
packets
Periodic Tx FIFO
control (optional)

OTG_HPTXFSIZ[31:16]

OTG_HPTXFSIZ[15:0]

MAC pop
Any non-periodic
channel DFIFO push
access from AHB

Non-periodic
Tx packets
Non-periodic Tx
FIFO control

OTG_HNPTXFSIZ[31:16]

OTG_HNPTXFSIZ[15:0]

MAC pop
Rx packets
Any channel DFIFO pop
access from AHB

OTG_GRXFSIZ[15:0]

Rx FIFO control
Rx start address fixed to 0
A1=0

MAC push

MSv36930V1

Host Rx FIFO
The host uses one receiver FIFO for all periodic and nonperiodic transactions. The FIFO is
used as a receive buffer to hold the received data (payload of the received packet) from the
USB until it is transferred to the system memory. Packets received from any remote IN
endpoint are stacked back-to-back until free space is available. The status of each received
packet with the host channel destination, byte count, data PID and validity of the received
data are also stored into the FIFO. The size of the receive FIFO is configured in the receive
FIFO size register (OTG_GRXFSIZ).
The single receive FIFO architecture makes it highly efficient for the USB host to fill in the
receive data buffer:
•

All IN configured host channels share the same RAM buffer (shared FIFO)

•

The OTG_HS core can fill in the receive FIFO up to the limit for any sequence of IN
tokens driven by the host software

The application receives the Rx FIFO not-empty interrupt as long as there is at least one
packet available for download. It reads the packet information from the receive status read
and pop register and finally pops the data off the receive FIFO.

RM0456 Rev 6

<!-- pagebreak -->

3460

USB on-the-go high-speed (OTG_HS)

RM0456

Host Tx FIFOs
The host uses one transmit FIFO for all non-periodic (control and bulk) OUT transactions
and one transmit FIFO for all periodic (isochronous and interrupt) OUT transactions. FIFOs
are used as transmit buffers to hold the data (payload of the transmit packet) to be
transmitted over the USB. The size of the periodic (nonperiodic) Tx FIFO is configured in the
host periodic (nonperiodic) transmit FIFO size OTG_HPTXFSIZ / OTG_HNPTXFSIZ)
register.
The two Tx FIFO implementation derives from the higher priority granted to the periodic type
of traffic over the USB frame. At the beginning of each frame, the built-in host scheduler
processes the periodic request queue first, followed by the nonperiodic request queue.
The two transmit FIFO architecture provides the USB host with separate optimization for
periodic and nonperiodic transmit data buffer management:
•

All host channels configured to support periodic (nonperiodic) transactions in the OUT
direction share the same RAM buffer (shared FIFOs)

•

The OTG_HS core can fill in the periodic (nonperiodic) transmit FIFO up to the limit for
any sequence of OUT tokens driven by the host software

The OTG_HS core issues the periodic Tx FIFO empty interrupt (PTXFE bit in
OTG_GINTSTS) as long as the periodic Tx FIFO is half or completely empty, depending on
the value of the periodic Tx FIFO empty level bit in the AHB configuration register
(PTXFELVL bit in OTG_GAHBCFG). The application can push the transmission data in
advance as long as free space is available in both the periodic Tx FIFO and the periodic
request queue. The host periodic transmit FIFO and queue status register
(OTG_HPTXSTS) can be read to know how much space is available in both.
OTG_HS core issues the non periodic Tx FIFO empty interrupt (NPTXFE bit in
OTG_GINTSTS) as long as the nonperiodic Tx FIFO is half or completely empty depending
on the non periodic Tx FIFO empty level bit in the AHB configuration register (TXFELVL bit
in OTG_GAHBCFG). The application can push the transmission data as long as free space
is available in both the nonperiodic Tx FIFO and nonperiodic request queue. The host
nonperiodic transmit FIFO and queue status register (OTG_HNPTXSTS) can be read to
know how much space is available in both.

73.11.3

FIFO RAM allocation
Device mode
Receive FIFO RAM allocation: the application must allocate RAM for SETUP packets:

<!-- pagebreak -->

•

10 locations must be reserved in the receive FIFO to receive SETUP packets on
control endpoint. The core does not use these locations, which are reserved for SETUP
packets, to write any other data.

•

One location is to be allocated for Global OUT NAK.

•

Status information is written to the FIFO along with each received packet. Therefore, a
minimum space of (largest packet size / 4) + 1 must be allocated to receive packets. If
multiple isochronous endpoints are enabled, then at least two (largest packet size / 4) +
1 spaces must be allocated to receive back-to-back packets. Typically, two (largest
packet size / 4) + 1 spaces are recommended so that when the previous packet is
being transferred to the CPU, the USB can receive the subsequent packet.

•

Along with the last packet for each endpoint, transfer complete status information is
also pushed to the FIFO. One location for each OUT endpoint is recommended.

RM0456 Rev 6

RM0456

USB on-the-go high-speed (OTG_HS)
Device RxFIFO =
(5 * number of control endpoints + 8) + ((largest USB packet used / 4) + 1 for status
information) + (2 * number of OUT endpoints) + 1 for Global NAK
Example: The MPS is 1,024 bytes for a periodic USB packet and 512 bytes for a nonperiodic USB packet. There are three OUT endpoints, three IN endpoints, one control
endpoint, and three host channels.
Device RxFIFO = (5 * 1 + 8) + ((1,024 / 4) +1) + (2 * 4) + 1 = 279
Transmit FIFO RAM allocation: the minimum RAM space required for each IN endpoint
Transmit FIFO is the maximum packet size for that particular IN endpoint.

Note:

More space allocated in the transmit IN endpoint FIFO results in better performance on the
USB.

Host mode
Receive FIFO RAM allocation:
Status information is written to the FIFO along with each received packet. Therefore, a
minimum space of (largest packet size / 4) + 1 must be allocated to receive packets. If
multiple isochronous channels are enabled, then at least two (largest packet size / 4) + 1
spaces must be allocated to receive back-to-back packets. Typically, two (largest packet
size / 4) + 1 spaces are recommended so that when the previous packet is being transferred
to the CPU, the USB can receive the subsequent packet.
Along with the last packet in the host channel, transfer complete status information is also
pushed to the FIFO. So one location must be allocated for this.
Host RxFIFO = (largest USB packet used / 4) + 1 for status information + 1 transfer
complete
Example: Host RxFIFO = ((1,024 / 4) + 1) + 1 = 258
Transmit FIFO RAM allocation:
The minimum amount of RAM required for the host Non-periodic Transmit FIFO is the
largest maximum packet size among all supported non-periodic OUT channels.
Typically, two largest packet sizes worth of space is recommended, so that when the current
packet is under transfer to the USB, the CPU can get the next packet.
Non-Periodic TxFIFO = largest non-periodic USB packet used / 4
Example: Non-Periodic TxFIFO = (512 / 4) = 128
The minimum amount of RAM required for host periodic Transmit FIFO is the largest
maximum packet size out of all the supported periodic OUT channels. If there is at least one
isochronous OUT endpoint, then the space must be at least two times the maximum packet
size of that channel.
Host Periodic TxFIFO = largest periodic USB packet used / 4
Example: Host Periodic TxFIFO = (1,024 / 4) = 256
Note:

More space allocated in the Transmit Non-periodic FIFO results in better performance on
the USB.

RM0456 Rev 6

<!-- pagebreak -->

3460

USB on-the-go high-speed (OTG_HS)

73.12

RM0456

OTG_HS interrupts
When the OTG_HS controller is operating in one mode, either device or host, the
application must not access registers from the other mode. If an illegal access occurs, a
mode mismatch interrupt is generated and reflected in the core interrupt register (MMIS bit
in the OTG_GINTSTS register). When the core switches from one mode to the other, the
registers in the new mode of operation must be reprogrammed as they would be after a
power-on reset.
Figure 926 shows the interrupt hierarchy.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

USB on-the-go high-speed (OTG_HS)
Figure 926. Interrupt hierarchy
Wake-up interrupt
OTG_FS_WKUP / OTG_HS_WKUP

(1)

AND

Global interrupt
OTG_FS / OTG_HS

OR

GI
NT

OE
PIN
IEP P
INT

HC
INT
HP
RT
INT

Global interrupt mask (bit 0)

OT

AND

OTG_AHBCFG
AHB configuration register

OTG_GINTSTS
Core register interrupt
31:26

25 24

23:20

19 18

17:3

2

1:0

OTG_GINTMSK
Core interrupt mask register

OTG_GOTGINT
OTG interrupt register

(15 + #EP):16
OUT endpoints

(#EP-1):0
IN endpoints

OTG_DAINTMSK
Device all endpoints interrupt mask
register
OTG_DAINT
Device all endpoints interrupt register

OTG_DIEPMSK/
OTG_DOEPMSK
Device IN/OUT endpoints common
interrupt mask register

x=0
...

OTG_DIEPINTx/
OTG_DOEPINTx
Device IN/OUT endpoint interrupt
registers

x = #EP-1

OTG_HPRT
Host port control and status register

OTG_HAINTMSK
Host all channels interrupt mask register
OTG_HAINT
Host all channels interrupt register
x=0
...

OTG_HCTINTMSKx
Host channels interrupt mask registers

x = #HC-1

OTG_HCTINTx
Host channels interrupt registers

MSv66261V1

1. OTG_HS_WKUP becomes active (high state) when resume condition occurs during L1 SLEEP or L2 SUSPEND states.

RM0456 Rev 6

<!-- pagebreak -->

3460

USB on-the-go high-speed (OTG_HS)

73.13

RM0456

OTG_HS control and status registers
By reading from and writing to the control and status registers (CSRs) through the AHB
slave interface, the application controls the OTG_HS controller. These registers are 32 bits
wide, and the addresses are 32-bit block aligned. The OTG_HS registers must be accessed
by words (32 bits).
CSRs are classified as follows:
•

Core global registers

•

Host-mode registers

•

Host global registers

•

Host port CSRs

•

Host channel-specific registers

•

Device-mode registers

•

Device global registers

•

Device endpoint-specific registers

•

Power and clock-gating registers

•

Data FIFO (DFIFO) access registers

Only the core global, power and clock-gating, data FIFO access, and host port control and
status registers can be accessed in both host and device modes. When the OTG_HS
controller is operating in one mode, either device or host, the application must not access
registers from the other mode. If an illegal access occurs, a mode mismatch interrupt is
generated and reflected in the core interrupt register (MMIS bit in the OTG_GINTSTS
register). When the core switches from one mode to the other, the registers in the new mode
of operation must be reprogrammed as they would be after a power-on reset.

73.13.1

CSR memory map
The host and device mode registers occupy different addresses. All registers are
implemented in the AHB clock domain.

Global CSR map
These registers are available in both host and device modes.
Table 766. Core global control and status registers (CSRs)
Acronym

Address
offset

Register name

OTG_GOTGCTL

0x000

Section 73.14.1: OTG control and status register (OTG_GOTGCTL)

OTG_GOTGINT

0x004

Section 73.14.2: OTG interrupt register (OTG_GOTGINT)

OTG_GAHBCFG

0x008

Section 73.14.3: OTG AHB configuration register (OTG_GAHBCFG)

OTG_GUSBCFG

0x00C

Section 73.14.4: OTG USB configuration register (OTG_GUSBCFG)

OTG_GRSTCTL

0x010

Section 73.14.5: OTG reset register (OTG_GRSTCTL)

OTG_GINTSTS

0x014

Section 73.14.6: OTG core interrupt register [alternate] (OTG_GINTSTS)
Section 73.14.7: OTG core interrupt register [alternate] (OTG_GINTSTS)

<!-- pagebreak -->

RM0456 Rev 6

RM0456

USB on-the-go high-speed (OTG_HS)
Table 766. Core global control and status registers (CSRs) (continued)
Address
offset

Register name

0x018

Section 73.14.8: OTG interrupt mask register [alternate] (OTG_GINTMSK)
Section 73.14.9: OTG interrupt mask register [alternate] (OTG_GINTMSK)

OTG_GRXSTSR

0x01C

Section 73.14.10: OTG receive status debug read register [alternate]
(OTG_GRXSTSR)
Section 73.14.11: OTG receive status debug read register [alternate]
(OTG_GRXSTSR)

OTG_GRXSTSP

0x020

Section 73.14.12: OTG status read and pop registers (OTG_GRXSTSP)
Section 73.14.13: OTG status read and pop registers [alternate]
(OTG_GRXSTSP)

OTG_GRXFSIZ

0x024

Section 73.14.14: OTG receive FIFO size register (OTG_GRXFSIZ)

OTG_HNPTXFSIZ/
OTG_DIEPTXF0(1)

0x028

Section 73.14.15: OTG host non-periodic transmit FIFO size register
[alternate] (OTG_HNPTXFSIZ)
Section 73.14.16: Endpoint 0 transmit FIFO size [alternate]
(OTG_DIEPTXF0)

OTG_HNPTXSTS

0x02C

Section 73.14.17: OTG non-periodic transmit FIFO/queue status register
(OTG_HNPTXSTS)

OTG_GCCFG

0x038

Section 73.14.18: OTG general core configuration register (OTG_GCCFG)

OTG_CID

0x03C

Section 73.14.19: OTG core ID register (OTG_CID)

OTG_GLPMCFG

0x54

Section 73.14.20: OTG core LPM configuration register (OTG_GLPMCFG)

OTG_HPTXFSIZ

0x100

Section 73.14.21: OTG host periodic transmit FIFO size register
(OTG_HPTXFSIZ)

OTG_DIEPTXFx

0x104
0x108
...
0x120

Section 73.14.22: OTG device IN endpoint transmit FIFO x size register
(OTG_DIEPTXFx)

Acronym
OTG_GINTMSK

1. The general rule is to use OTG_HNPTXFSIZ for host mode and OTG_DIEPTXF0 for device mode.

Host-mode CSR map
These registers must be programmed every time the core changes to host mode.
Table 767. Host-mode control and status registers (CSRs)
Acronym

Offset
address

Register name

OTG_HCFG

0x400

Section 73.14.24: OTG host configuration register (OTG_HCFG)

OTG_HFIR

0x404

Section 73.14.25: OTG host frame interval register (OTG_HFIR)

OTG_HFNUM

0x408

Section 73.14.26: OTG host frame number/frame time remaining register
(OTG_HFNUM)

OTG_HPTXSTS

0x410

Section 73.14.27: OTG_Host periodic transmit FIFO/queue status register
(OTG_HPTXSTS)

RM0456 Rev 6

<!-- pagebreak -->

3460

USB on-the-go high-speed (OTG_HS)

RM0456

Table 767. Host-mode control and status registers (CSRs) (continued)
Acronym

Offset
address

Register name

OTG_HAINT

0x414

Section 73.14.28: OTG host all channels interrupt register (OTG_HAINT)

OTG_HAINTMSK

0x418

Section 73.14.29: OTG host all channels interrupt mask register
(OTG_HAINTMSK)

OTG_HPRT

0x440

Section 73.14.30: OTG host port control and status register (OTG_HPRT)

OTG_HCCHARx

0x500
0x520
...
0x6E0

Section 73.14.31: OTG host channel x characteristics register
(OTG_HCCHARx)

OTG_HCSPLTx

0x504
0x524
....
0x6E4

Section 73.14.32: OTG host channel x split control register
(OTG_HCSPLTx)

OTG_HCINTx

0x508
0x528
....
0x6E8

Section 73.14.33: OTG host channel x interrupt register (OTG_HCINTx)

OTG_HCINTMSKx

0x50C
0x52C
....
0x6EC

Section 73.14.34: OTG host channel x interrupt mask register
(OTG_HCINTMSKx)

OTG_HCTSIZx

0x510
0x530
....
0x6F0

Section 73.14.35: OTG host channel x transfer size register
(OTG_HCTSIZx)

OTG_HCDMAx

0x514
0x534
....
0x6F4

Section 73.14.36: OTG host channel x DMA address
register(OTG_HCDMAx)

Device-mode CSR map
These registers must be programmed every time the core changes to device mode.
Table 768. Device-mode control and status registers
Acronym

Offset
address

Register name

OTG_DCFG

0x800

Section 73.14.38: OTG device configuration register (OTG_DCFG)

OTG_DCTL

0x804

Section 73.14.39: OTG device control register (OTG_DCTL)

OTG_DSTS

0x808

Section 73.14.40: OTG device status register (OTG_DSTS)

<!-- pagebreak -->

RM0456 Rev 6

RM0456

USB on-the-go high-speed (OTG_HS)
Table 768. Device-mode control and status registers (continued)

Acronym

Offset
address

Register name

OTG_DIEPMSK

0x810

Section 73.14.41: OTG device IN endpoint common interrupt mask
register (OTG_DIEPMSK)

OTG_DOEPMSK

0x814

Section 73.14.42: OTG device OUT endpoint common interrupt mask
register (OTG_DOEPMSK)

OTG_DAINT

0x818

Section 73.14.43: OTG device all endpoints interrupt register
(OTG_DAINT)

OTG_DAINTMSK

0x81C

Section 73.14.44: OTG all endpoints interrupt mask register
(OTG_DAINTMSK)

OTG_DTHRCTL

0x830

Section 73.14.45: OTG device threshold control register
(OTG_DTHRCTL)

OTG_DIEPEMPMSK

0x834

Section 73.14.46: OTG device IN endpoint FIFO empty interrupt mask
register (OTG_DIEPEMPMSK)

OTG_DIEPCTLx

0x900
0x920
...
0xA00

Section 73.14.47: OTG device IN endpoint x control register [alternate]
(OTG_DIEPCTLx)
Section 73.14.48: OTG device IN endpoint x control register [alternate]
(OTG_DIEPCTLx)

OTG_DIEPINTx

0x908
0x928
...
0x9E8

Section 73.14.49: OTG device IN endpoint x interrupt register
(OTG_DIEPINTx)

OTG_DIEPTSIZ0

0x910

Section 73.14.50: OTG device IN endpoint 0 transfer size register
(OTG_DIEPTSIZ0)

OTG_DIEPDMAx

0x914
0x934
...
0x9F4

Section 73.14.51: OTG device IN endpoint x DMA address register
(OTG_DIEPDMAx)

OTG_DTXFSTSx

0x918
0x938
.....
0x9F8

Section 73.14.52: OTG device IN endpoint transmit FIFO status register
(OTG_DTXFSTSx)

OTG_DIEPTSIZx

0x930
0x950
...
0x9F0

Section 73.14.53: OTG device IN endpoint x transfer size register
(OTG_DIEPTSIZx)

OTG_DOEPCTL0

0xB00

Section 73.14.54: OTG device control OUT endpoint 0 control register
(OTG_DOEPCTL0)

OTG_DOEPINTx

0xB08
0XB28
...
0xC08

Section 73.14.55: OTG device OUT endpoint x interrupt register
(OTG_DOEPINTx)

RM0456 Rev 6

<!-- pagebreak -->

3460

USB on-the-go high-speed (OTG_HS)

RM0456

Table 768. Device-mode control and status registers (continued)
Acronym

Offset
address

Register name

OTG_DOEPTSIZ0

0xB10

Section 73.14.56: OTG device OUT endpoint 0 transfer size register
(OTG_DOEPTSIZ0)

OTG_DOEPDMAx

0xB14
0xB34
...
0xC14

Section 73.14.57: OTG device OUT endpoint x DMA address register
(OTG_DOEPDMAx)

OTG_DOEPCTLx

0xB20
0xB40
...
0xC00

Section 73.14.58: OTG device OUT endpoint x control register
[alternate] (OTG_DOEPCTLx)
Section 73.14.59: OTG device OUT endpoint x control register
[alternate] (OTG_DOEPCTLx)

OTG_DOEPTSIZx

0xB30
0xB50
..
0xBF0

Section 73.14.60: OTG device OUT endpoint x transfer size register
(OTG_DOEPTSIZx)

Data FIFO (DFIFO) access register map
These registers, available in both host and device modes, are used to read or write the FIFO
space for a specific endpoint or a channel, in a given direction. If a host channel is of type
IN, the FIFO can only be read on the channel. Similarly, if a host channel is of type OUT, the
FIFO can only be written on the channel.
Table 769. Data FIFO (DFIFO) access register map
FIFO access register section

Offset address

Access

Device IN endpoint 0/Host OUT Channel 0: DFIFO write access
Device OUT endpoint 0/Host IN Channel 0: DFIFO read access

0x1000–0x1FFC

w
r

Device IN endpoint 1/Host OUT Channel 1: DFIFO write access
Device OUT endpoint 1/Host IN Channel 1: DFIFO read access

0x2000–0x2FFC

w
r

...

...

...

Device IN endpoint x(1)/Host OUT Channel x(1): DFIFO write access
0xX000–0xXFFC
Device OUT endpoint x(1)/Host IN Channel x(1): DFIFO read access

w
r

1. Where x is 8 in device mode and 15 in host mode.

Power and clock gating CSR map
There is a single register for power and clock gating. It is available in both host and device
modes.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

USB on-the-go high-speed (OTG_HS)
Table 770. Power and clock gating control and status registers
Acronym

Offset address

OTG_PCGCCTL

73.14

0xE00–0xE04

Register name
Section 73.14.61: OTG power and clock gating control
register (OTG_PCGCCTL)

OTG_HS registers
These registers are available in both host and device modes, and do not need to be
reprogrammed when switching between these modes.
Bit values in the register descriptions are expressed in binary unless otherwise specified.

73.14.1

OTG control and status register (OTG_GOTGCTL)
The OTG_GOTGCTL register controls the behavior and reflects the status of the OTG
function of the core.
Address offset: 0x000
Reset value: 0x0001 0000

31
Res.

15
Res.

30
Res.

14
Res.

29
Res.

13
Res.

28
Res.

12
EHEN

27
Res.

11
Res.

26
Res.

10
Res.

25
Res.

9
Res.

24
Res.

23
Res.

8
Res.

22

21

20

Res.

CUR
MOD

OTG
VER

r

rw

r

5

4

3

7

6

19

18

17

16

DBCT

CID
STS

r

r

r

2

1

0

Res.

Res.

BSVLD ASVLD

BVALO BVALO AVALO AVALO VBVAL VBVAL
VAL
EN
VAL
EN
OVAL
OEN

rw

rw

rw

rw

rw

rw

rw

Bits 31:22 Reserved, must be kept at reset value.
Bit 21 CURMOD: Current mode of operation
Indicates the current mode (host or device).
0: Device mode
1: Host mode
Bit 20 OTGVER: OTG version
Selects the OTG revision.
0:OTG Version 1.3. OTG1.3 is obsolete for new product development.
1:OTG Version 2.0. In this version the core supports only data line pulsing for SRP.
Bit 19 BSVLD: B-session valid
Indicates the device mode transceiver status.
0: B-session is not valid.
1: B-session is valid.
In OTG mode, the user can use this bit to determine if the device is connected or
disconnected.
Note: Only accessible in device mode.

RM0456 Rev 6

<!-- pagebreak -->

3460

USB on-the-go high-speed (OTG_HS)

RM0456

Bit 18 ASVLD: A-session valid
Indicates the host mode transceiver status.
0: A-session is not valid
1: A-session is valid
Note: Only accessible in host mode.
Bit 17 DBCT: Long/short debounce time
Indicates the debounce time of a detected connection.
0: Long debounce time, used for physical connections (100 ms + 2.5 µs)
1: Short debounce time, used for soft connections (2.5 µs)
Note: Only accessible in host mode.
Bit 16 CIDSTS: Connector ID status
Indicates the connector ID status on a connect event.
0: The OTG_HS controller is in A-device mode
1: The OTG_HS controller is in B-device mode
Note: Accessible in both device and host modes.
Bits 15:13 Reserved, must be kept at reset value.
Bit 12 EHEN: Embedded host enable
It is used to select between OTG A device state machine and embedded host state machine.
0: OTG A device state machine is selected
1: Embedded host state machine is selected
Bits 11:8 Reserved, must be kept at reset value.
Bit 7 BVALOVAL: B-peripheral session valid override value
This bit is used to set override value for Bvalid signal when BVALOEN bit is set.
0: Bvalid value is '0' when BVALOEN = 1
1: Bvalid value is '1' when BVALOEN = 1
Note: Only accessible in device mode.
Bit 6 BVALOEN: B-peripheral session valid override enable
This bit is used to enable/disable the software to override the Bvalid signal using the
BVALOVAL bit.
0: Override is disabled and Bvalid signal from the respective PHY selected is used internally
by the core
1 Internally Bvalid received from the PHY is overridden with BVALOVAL bit value
Note: Only accessible in device mode.
Bit 5 AVALOVAL: A-peripheral session valid override value
This bit is used to set override value for Avalid signal when AVALOEN bit is set.
0: Avalid value is '0' when AVALOEN = 1
1: Avalid value is '1' when AVALOEN = 1
Note: Only accessible in host mode.
Bit 4 AVALOEN: A-peripheral session valid override enable
This bit is used to enable/disable the software to override the Avalid signal using the
AVALOVAL bit.
0: Override is disabled and Avalid signal from the respective PHY selected is used internally
by the core
1: Internally Avalid received from the PHY is overridden with AVALOVAL bit value
Note: Only accessible in host mode.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

USB on-the-go high-speed (OTG_HS)

Bit 3 VBVALOVAL: VBUS valid override value
This bit is used to set override value for vbusvalid signal when VBVALOEN bit is set.
0: vbusvalid value is '0' when VBVALOEN = 1
1: vbusvalid value is '1' when VBVALOEN = 1
Note: Only accessible in host mode.
Bit 2 VBVALOEN: VBUS valid override enable
This bit is used to enable/disable the software to override the vbusvalid signal using the
VBVALOVAL bit.
0: Override is disabled and vbusvalid signal from the respective PHY selected is used
internally by the core
1: Internally vbusvalid received from the PHY is overridden with VBVALOVAL bit value
Note: Only accessible in host mode.
Bits 1:0 Reserved, must be kept at reset value.

73.14.2

OTG interrupt register (OTG_GOTGINT)
The application reads this register whenever there is an OTG interrupt and clears the bits in
this register to clear the OTG interrupt.
Address offset: 0x004
Reset value: 0x0000 0000

31
Res.

30

29

Res.

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
Res.

19

18

17

16

Res.

ADTO
CHG

Res.

Res.

rc_w1
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

SEDET

Res.

Res.

rc_w1

Bits 31:19 Reserved, must be kept at reset value.
Bit 18 ADTOCHG: A-device timeout change
The core sets this bit to indicate that the A-device has timed out while waiting for the B-device
to connect.
Note: Accessible in both device and host modes.
Bits 17:3 Reserved, must be kept at reset value.
Bit 2 SEDET: Session end detected
The core sets this bit to indicate that the level of the voltage on VBUS is no longer valid for a
B-Peripheral session when VBUS < 0.8 V.
Note: Accessible in both device and host modes.
Bits 1:0 Reserved, must be kept at reset value.

RM0456 Rev 6

<!-- pagebreak -->

3460

USB on-the-go high-speed (OTG_HS)

73.14.3

RM0456

OTG AHB configuration register (OTG_GAHBCFG)
This register can be used to configure the core after power-on or a change in mode. This
register mainly contains AHB system-related configuration parameters. Do not change this
register after the initial programming. The application must program this register before
starting any transactions on either the AHB or the USB.
Address offset: 0x008
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

Res.

PTXFE
LVL

TXFE
LVL

Res.

DMAE
N

rw

rw

Res.

Res.

Res.

Res.

Res.

Res.

rw

HBSTLEN[3:0]
rw

rw

rw

0
GINT
MSK

rw

rw

Bits 31:9 Reserved, must be kept at reset value.
Bit 8 PTXFELVL: Periodic Tx FIFO empty level
Indicates when the periodic Tx FIFO empty interrupt bit in the OTG_GINTSTS register
(PTXFE bit in OTG_GINTSTS) is triggered.
0: PTXFE (in OTG_GINTSTS) interrupt indicates that the Periodic Tx FIFO is half empty
1: PTXFE (in OTG_GINTSTS) interrupt indicates that the Periodic Tx FIFO is completely
empty
Note: Only accessible in host mode.
Bit 7 TXFELVL: Tx FIFO empty level
Condition: device mode
This bit indicates when IN endpoint Transmit FIFO empty interrupt (TXFE in OTG_DIEPINTx)
is triggered:
0: The TXFE (in OTG_DIEPINTx) interrupt indicates that the IN endpoint Tx FIFO is half
empty
1: The TXFE (in OTG_DIEPINTx) interrupt indicates that the IN endpoint Tx FIFO is
completely empty
Condition: host mode
This bit indicates when the nonperiodic Tx FIFO empty interrupt (NPTXFE bit in
OTG_GINTSTS) is triggered:
0: The NPTXFE (in OTG_GINTSTS) interrupt indicates that the nonperiodic Tx FIFO is half
empty
1: The NPTXFE (in OTG_GINTSTS) interrupt indicates that the nonperiodic Tx FIFO is
completely empty
Bit 6 Reserved, must be kept at reset value.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

USB on-the-go high-speed (OTG_HS)

Bit 5 DMAEN: DMA enabled
0: The core operates in slave mode
1: The core operates in DMA mode
Bits 4:1 HBSTLEN[3:0]: Burst length/type
0000 Single: Bus transactions use single 32 bit accesses (not recommended)
0001 INCR: Bus transactions use unspecified length accesses (not recommended, uses the
INCR AHB bus command)
0011 INCR4: Bus transactions target 4x 32 bit accesses
0101 INCR8: Bus transactions target 8x 32 bit accesses
0111 INCR16: Bus transactions based on 16x 32 bit accesses
Others: Reserved
Bit 0 GINTMSK: Global interrupt mask
The application uses this bit to mask or unmask the interrupt line assertion to itself.
Irrespective of this bit setting, the interrupt status registers are updated by the core.
0: Mask the interrupt assertion to the application.
1: Unmask the interrupt assertion to the application.
Note: Accessible in both device and host modes.

73.14.4

OTG USB configuration register (OTG_GUSBCFG)
This register can be used to configure the core after power-on or a changing to host mode
or device mode. It contains USB and USB-PHY related configuration parameters. The
application must program this register before starting any transactions on either the AHB or
the USB. Do not make changes to this register after the initial programming.
Address offset: 0x00C
Reset value: 0x0000 1400

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

FD
MOD

FH
MOD

Res.

Res.

Res.

Res.

Res.

Res.

TSDPS

Res.

Res.

Res.

Res.

Res.

Res.

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

PHYL
PC

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

TRDT[3:0]
rw

rw

rw

rw

TOCAL[2:0]
rw

RM0456 Rev 6

rw

rw

<!-- pagebreak -->

3460

USB on-the-go high-speed (OTG_HS)

RM0456

Bit 31 Reserved, must be kept at reset value.
Bit 30 FDMOD: Force device mode
Writing a 1 to this bit, forces the core to device mode irrespective of the OTG_ID input pin.
0: Normal mode
1: Force device mode
After setting the force bit, the application must wait at least 25 ms before the change takes
effect.
Note: Accessible in both device and host modes.
Bit 29 FHMOD: Force host mode
Writing a 1 to this bit, forces the core to host mode irrespective of the OTG_ID input pin.
0: Normal mode
1: Force host mode
After setting the force bit, the application must wait at least 25 ms before the change takes
effect.
Note: Accessible in both device and host modes.
Bits 28:26 Reserved, must be kept at reset value.
Bits 25:23 Reserved, must be kept at reset value.
Bit 22 TSDPS: TermSel DLine pulsing selection
This bit selects utmi_termselect to drive the data line pulse during SRP (session request
protocol).
0: Data line pulsing using utmi_txvalid (default)
1: Data line pulsing using utmi_termsel
Bits 21:16 Reserved, must be kept at reset value.
Bit 15 PHYLPC: PHY Low-power clock select
This bit selects either 480 MHz or 48 MHz (low-power) PHY mode. In FS and LS modes, the
PHY can usually operate on a 48 MHz clock to save power.
0: 480 MHz internal PLL clock
1: 48 MHz external clock
In 480 MHz mode, the UTMI interface operates at either 60 or 30 MHz, depending on
whether the 8- or 16-bit data width is selected. In 48 MHz mode, the UTMI interface operates
at 48 MHz in FS and LS modes.
Bit 14 Reserved, must be kept at reset value.
Bits 13:10 TRDT[3:0]: USB turnaround time
These bits allows to set the turnaround time in PHY clocks. They must be configured
according to Table 771: TRDT values, depending on the application AHB frequency. Higher
TRDT values allow stretching the USB response time to IN tokens in order to compensate for
longer AHB read access latency to the data FIFO.
Note: Only accessible in device mode.
Bits 9:8 Reserved, must be kept at reset value.
Bit 7 Reserved, must be kept at reset value.
Bit 6 Reserved, must be kept at reset value.
Bit 5 Reserved, must be kept at reset value.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

USB on-the-go high-speed (OTG_HS)

Bit 4 Reserved, must be kept at reset value.
Bit 3 Reserved, must be kept at reset value.
Bits 2:0 TOCAL[2:0]: FS timeout calibration
The number of PHY clocks that the application programs in this field is added to the fullspeed interpacket timeout duration in the core to account for any additional delays
introduced by the PHY. This can be required, because the delay introduced by the PHY in
generating the line state condition can vary from one PHY to another.
The USB standard timeout value for full-speed operation is 16 to 18 (inclusive) bit times. The
application must program this field based on the speed of enumeration. The number of bit
times added per PHY clock is 0.25 bit times.

Table 771. TRDT values
AHB frequency range (MHz)
TRDT minimum value

73.14.5

Min

Max

30

-

0x9

OTG reset register (OTG_GRSTCTL)
The application uses this register to reset various hardware features inside the core.
Address offset: 0x010
Reset value: 0x8000 0000

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

AHB
IDL

DMAR
EQ

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

13

12

11

10

9

8

7

6

2

1

0

r

r

15

14

Res.

Res.

Res.

Res.

Res.

TXFNUM[4:0]
rw

rw

rw

rw

rw

5

4

3

TXF
FLSH

RXF
FLSH

Res.

rs

rs

FCRST PSRST CSRST
rs

rs

rs

Bit 31 AHBIDL: AHB master idle
Indicates that the AHB master state machine is in the Idle condition.
Note: Accessible in both device and host modes.
Bit 30 DMAREQ: DMA request signal enabled
This bit indicates that the DMA request is in progress. Used for debug.
Bits 29:11 Reserved, must be kept at reset value.

RM0456 Rev 6

<!-- pagebreak -->

3460

USB on-the-go high-speed (OTG_HS)

RM0456

Bits 10:6 TXFNUM[4:0]: Tx FIFO number
This is the FIFO number that must be flushed using the Tx FIFO Flush bit. This field must not
be changed until the core clears the Tx FIFO Flush bit.
Condition: host mode
00000: Non-periodic Tx FIFO flush
00001: Periodic Tx FIFO flush
10000: Flush all the transmit FIFOs
Condition: device mode
00000: Tx FIFO 0 flush
00001: Tx FIFO 1 flush
00010: Tx FIFO 2 flush
...
01111: Tx FIFO 15 flush
10000: Flush all the transmit FIFOs
Note: Accessible in both device and host modes.
Bit 5 TXFFLSH: Tx FIFO flush
This bit selectively flushes a single or all transmit FIFOs, but cannot do so if the core is in the
midst of a transaction.
The application must write this bit only after checking that the core is neither writing to the Tx
FIFO nor reading from the Tx FIFO. Verify using these registers:
Read—NAK Effective interrupt ensures the core is not reading from the FIFO
Write—AHBIDL bit in OTG_GRSTCTL ensures the core is not writing anything to the FIFO.
Flushing is normally recommended when FIFOs are reconfigured. FIFO flushing is also
recommended during device endpoint disable. The application must wait until the core clears
this bit before performing any operations. This bit takes eight clocks to clear, using the slower
clock of phy_clk or hclk.
Note: Accessible in both device and host modes.
Bit 4 RXFFLSH: Rx FIFO flush
The application can flush the entire Rx FIFO using this bit, but must first ensure that the core
is not in the middle of a transaction.
The application must only write to this bit after checking that the core is neither reading from
the Rx FIFO nor writing to the Rx FIFO.
The application must wait until the bit is cleared before performing any other operations. This
bit requires 8 clocks (slowest of PHY or AHB clock) to clear.
Note: Accessible in both device and host modes.
Bit 3 Reserved, must be kept at reset value.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

USB on-the-go high-speed (OTG_HS)

Bit 2 FCRST: Host frame counter reset
The application writes this bit to reset the (micro-)frame number counter inside the core.
When the (micro-)frame counter is reset, the subsequent SOF sent out by the core has a
frame number of 0.
When application writes '1' to the bit, it might not be able to read back the value as it gets
cleared by the core in a few clock cycles.
Note: Only accessible in host mode.
Bit 1 PSRST: Partial soft reset
Resets the internal state machines but keeps the enumeration info. Can be used to recover
some specific PHY errors.
Note: Accessible in both device and host modes.
Bit 0 CSRST: Core soft reset
Resets the HCLK and PHY clock domains as follows:
Clears the interrupts and all the CSR register bits except for the following bits:
– GATEHCLK bit in OTG_PCGCCTL
– STPPCLK bit in OTG_PCGCCTL
– FSLSPCS bits in OTG_HCFG
– DSPD bit in OTG_DCFG
– SDIS bit in OTG_DCTL
– OTG_GCCFG register
– FDMOD bit in OTG_GUSBCFG
– FHMOD bit in OTG_GUSBCFG
– PHYLPC bit in OTG_GUSBCFG
– TSDPS bit in OTG_GUSBCFG
All module state machines (except for the AHB slave unit) are reset to the Idle state, and all
the transmit FIFOs and the receive FIFO are flushed.
Any transactions on the AHB Master are terminated as soon as possible, after completing the
last data phase of an AHB transfer. Any transactions on the USB are terminated immediately.
The application can write to this bit any time it wants to reset the core. This is a self-clearing
bit and the core clears this bit after all the necessary logic is reset in the core, which can take
several clocks, depending on the current state of the core. Once this bit has been cleared,
the software must wait at least 3 PHY clocks before accessing the PHY domain
(synchronization delay). The software must also check that bit 31 in this register is set to 1
(AHB Master is Idle) before starting any operation.
Typically, the software reset is used during software development and also when the user
dynamically changes the PHY selection bits in the above listed USB configuration registers.
When the user changes the PHY, the corresponding clock for the PHY is selected and used
in the PHY domain. Once a new clock is selected, the PHY domain has to be reset for proper
operation.
Note: Accessible in both device and host modes.

RM0456 Rev 6

<!-- pagebreak -->

3460

USB on-the-go high-speed (OTG_HS)

73.14.6

RM0456

OTG core interrupt register [alternate] (OTG_GINTSTS)
Valid for Host mode, see next section for Device mode.
This register also indicates the current mode. To clear the interrupt status bits of the rc_w1
type, the application must write 1 into the bit.
This register interrupts the application for system-level events in the current mode (device
mode or host mode).
The FIFO status interrupts are read-only; once software reads from or writes to the FIFO
while servicing these interrupts, FIFO interrupt conditions are cleared automatically.
The application must clear the OTG_GINTSTS register at initialization before unmasking
the interrupt bit to avoid any interrupts generated prior to initialization.
Address offset: 0x014
Reset value: 0x0400 0020

31

30

29

28

27

WKUP
INT

SRQ
INT

DISC
INT

CIDS
CHG

LPM
INT

26

rc_w1

rc_w1

rc_w1

rc_w1

rc_w1

r

15

14

13

12

11

10

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

HPRT
INT

RST
DET

DATAF
SUSP

IPXFR

IISOI
XFR

OEP
INT

IEPINT

Res.

Res.

r

r

rc_w1

rc_w1

rc_w1

rc_w1

r

r

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

GO
NAK
EFF

GI
NAK
EFF

NPTXF
E

RXF
LVL

SOF

OTG
INT

MMIS

CMOD

r

r

r

r

rc_w1

r

rc_w1

r

PTXFE HCINT

EOPF

ISOO
DRP

ENUM
DNE

USB
RST

USB
SUSP

ESUSP

rc_w1

rc_w1

rc_w1

rc_w1

rc_w1

rc_w1

Res.

Bit 31 WKUPINT: Resume/remote wake-up detected interrupt
Wake-up interrupt during suspend(L2) or LPM(L1) state.
– During suspend(L2):
In device mode, this interrupt is asserted when a resume is detected on the USB. In host
mode, this interrupt is asserted when a remote wake-up is detected on the USB.
– During LPM(L1):
This interrupt is asserted for either host initiated resume or device initiated remote wake-up
on USB.
Note: Accessible in both device and host modes.
Bit 30 SRQINT: Session request/new session detected interrupt
In host mode, this interrupt is asserted when a session request is detected from the device.
In device mode, this interrupt is asserted when VBUS is in the valid range for a B-peripheral
device. Accessible in both device and host modes.
Bit 29 DISCINT: Disconnect detected interrupt
Asserted when a device disconnect is detected.
Note: Only accessible in host mode.
Bit 28 CIDSCHG: Connector ID status change
The core sets this bit when there is a change in connector ID status.
Note: Accessible in both device and host modes.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

USB on-the-go high-speed (OTG_HS)

Bit 27 LPMINT: LPM interrupt
In device mode, this interrupt is asserted when the device receives an LPM transaction and
responds with a non-ERRORed response.
In host mode, this interrupt is asserted when the device responds to an LPM transaction with
a non-ERRORed response or when the host core has completed LPM transactions for the
programmed number of times (RETRYCNT bit in OTG_GLPMCFG).
This field is valid only if the LPMEN bit in OTG_GLPMCFG is set to 1.
Bit 26 PTXFE: Periodic Tx FIFO empty
Asserted when the periodic transmit FIFO is either half or completely empty and there is
space for at least one entry to be written in the periodic request queue. The half or completely
empty status is determined by the periodic Tx FIFO empty level bit in the OTG_GAHBCFG
register (PTXFELVL bit in OTG_GAHBCFG).
Note: Only accessible in host mode.
Bit 25 HCINT: Host channels interrupt
The core sets this bit to indicate that an interrupt is pending on one of the channels of the
core (in host mode). The application must read the OTG_HAINT register to determine the
exact number of the channel on which the interrupt occurred, and then read the
corresponding OTG_HCINTx register to determine the exact cause of the interrupt. The
application must clear the appropriate status bit in the OTG_HCINTx register to clear this bit.
Note: Only accessible in host mode.
Bit 24 HPRTINT: Host port interrupt
The core sets this bit to indicate a change in port status of one of the OTG_HS controller
ports in host mode. The application must read the OTG_HPRT register to determine the
exact event that caused this interrupt. The application must clear the appropriate status bit in
the OTG_HPRT register to clear this bit.
Note: Only accessible in host mode.
Bit 23 RSTDET: Reset detected interrupt
In device mode, this interrupt is asserted when a reset is detected on the USB in partial
power-down mode when the device is in suspend.
Note: Only accessible in device mode.
Bit 22 DATAFSUSP: Data fetch suspended
This interrupt is valid only in DMA mode. This interrupt indicates that the core has stopped
fetching data for IN endpoints due to the unavailability of TxFIFO space or request queue
space. This interrupt is used by the application for an endpoint mismatch algorithm. For
example, after detecting an endpoint mismatch, the application:
–
Sets a global nonperiodic IN NAK handshake
–
Disables IN endpoints
–
Flushes the FIFO
–
Determines the token sequence from the IN token sequence learning queue
–
Re-enables the endpoints
Clears the global nonperiodic IN NAK handshake If the global nonperiodic IN NAK is cleared,
the core has not yet fetched data for the IN endpoint, and the IN token is received: the core
generates an “IN token received when FIFO empty” interrupt. The OTG then sends a NAK
response to the host. To avoid this scenario, the application can check the FetSusp interrupt
in OTG_GINTSTS, which ensures that the FIFO is full before clearing a global NAK
handshake. Alternatively, the application can mask the “IN token received when FIFO empty”
interrupt when clearing a global IN NAK handshake.
Bit 21 IPXFR: Incomplete periodic transfer
In host mode, the core sets this interrupt bit when there are incomplete periodic transactions
still pending, which are scheduled for the current frame.

RM0456 Rev 6

<!-- pagebreak -->

3460

USB on-the-go high-speed (OTG_HS)

RM0456

Bit 20 IISOIXFR: Incomplete isochronous IN transfer
The core sets this interrupt to indicate that there is at least one isochronous IN endpoint on
which the transfer is not completed in the current frame. This interrupt is asserted along with
the End of periodic frame interrupt (EOPF) bit in this register.
Note: Only accessible in device mode.
Bit 19 OEPINT: OUT endpoint interrupt
The core sets this bit to indicate that an interrupt is pending on one of the OUT endpoints of
the core (in device mode). The application must read the OTG_DAINT register to determine
the exact number of the OUT endpoint on which the interrupt occurred, and then read the
corresponding OTG_DOEPINTx register to determine the exact cause of the interrupt. The
application must clear the appropriate status bit in the corresponding OTG_DOEPINTx
register to clear this bit.
Note: Only accessible in device mode.
Bit 18 IEPINT: IN endpoint interrupt
The core sets this bit to indicate that an interrupt is pending on one of the IN endpoints of the
core (in device mode). The application must read the OTG_DAINT register to determine the
exact number of the IN endpoint on which the interrupt occurred, and then read the
corresponding OTG_DIEPINTx register to determine the exact cause of the interrupt. The
application must clear the appropriate status bit in the corresponding OTG_DIEPINTx
register to clear this bit.
Note: Only accessible in device mode.
Bits 17:16 Reserved, must be kept at reset value.
Bit 15 EOPF: End of periodic frame interrupt
Indicates that the period specified in the periodic frame interval field of the OTG_DCFG
register (PFIVL bit in OTG_DCFG) has been reached in the current frame.
Note: Only accessible in device mode.
Bit 14 ISOODRP: Isochronous OUT packet dropped interrupt
The core sets this bit when it fails to write an isochronous OUT packet into the Rx FIFO
because the Rx FIFO does not have enough space to accommodate a maximum size packet
for the isochronous OUT endpoint.
Note: Only accessible in device mode.
Bit 13 ENUMDNE: Enumeration done
The core sets this bit to indicate that speed enumeration is complete. The application must
read the OTG_DSTS register to obtain the enumerated speed.
Note: Only accessible in device mode.
Bit 12 USBRST: USB reset
The core sets this bit to indicate that a reset is detected on the USB.
Note: Only accessible in device mode.
Bit 11 USBSUSP: USB suspend
The core sets this bit to indicate that a suspend was detected on the USB. The core enters
the suspended state when there is no activity on the data lines for an extended period of
time.
Note: Only accessible in device mode.
Bit 10 ESUSP: Early suspend
The core sets this bit to indicate that an Idle state has been detected on the USB for 3 ms.
Note: Only accessible in device mode.
Bits 9:8 Reserved, must be kept at reset value.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

USB on-the-go high-speed (OTG_HS)

Bit 7 GONAKEFF: Global OUT NAK effective
Indicates that the Set global OUT NAK bit in the OTG_DCTL register (SGONAK bit in
OTG_DCTL), set by the application, has taken effect in the core. This bit can be cleared by
writing the Clear global OUT NAK bit in the OTG_DCTL register (CGONAK bit in
OTG_DCTL).
Note: Only accessible in device mode.
Bit 6 GINAKEFF: Global IN non-periodic NAK effective
Indicates that the Set global non-periodic IN NAK bit in the OTG_DCTL register (SGINAK bit
in OTG_DCTL), set by the application, has taken effect in the core. That is, the core has
sampled the Global IN NAK bit set by the application. This bit can be cleared by clearing the
Clear global non-periodic IN NAK bit in the OTG_DCTL register (CGINAK bit in OTG_DCTL).
This interrupt does not necessarily mean that a NAK handshake is sent out on the USB. The
STALL bit takes precedence over the NAK bit.
Note: Only accessible in device mode.
Bit 5 NPTXFE: Non-periodic Tx FIFO empty
This interrupt is asserted when the non-periodic Tx FIFO is either half or completely empty,
and there is space for at least one entry to be written to the non-periodic transmit request
queue. The half or completely empty status is determined by the non-periodic Tx FIFO empty
level bit in the OTG_GAHBCFG register (TXFELVL bit in OTG_GAHBCFG).
Note: Accessible in host mode only.
Bit 4 RXFLVL: Rx FIFO non-empty
Indicates that there is at least one packet pending to be read from the Rx FIFO.
Note: Accessible in both host and device modes.
Bit 3 SOF: Start of frame
In host mode, the core sets this bit to indicate that an SOF (FS), or Keep-Alive (LS) is
transmitted on the USB. The application must write a 1 to this bit to clear the interrupt.
In device mode, in the core sets this bit to indicate that an SOF token has been received on
the USB. The application can read the OTG_DSTS register to get the current frame number.
This interrupt is seen only when the core is operating in FS.
Note: This register may return '1' if read immediately after power on reset. If the register bit
reads '1' immediately after power on reset it does not indicate that an SOF has been
sent (in case of host mode) or SOF has been received (in case of device mode). The
read value of this interrupt is valid only after a valid connection between host and device
is established. If the bit is set after power on reset the application can clear the bit.
Note: Accessible in both host and device modes.

RM0456 Rev 6

<!-- pagebreak -->

3460

USB on-the-go high-speed (OTG_HS)

RM0456

Bit 2 OTGINT: OTG interrupt
The core sets this bit to indicate an OTG protocol event. The application must read the OTG
interrupt status (OTG_GOTGINT) register to determine the exact event that caused this
interrupt. The application must clear the appropriate status bit in the OTG_GOTGINT register
to clear this bit.
Note: Accessible in both host and device modes.
Bit 1 MMIS: Mode mismatch interrupt
The core sets this bit when the application is trying to access:
– A host mode register, when the core is operating in device mode
– A device mode register, when the core is operating in host mode
The register access is completed on the AHB with an OKAY response, but is ignored by the
core internally and does not affect the operation of the core.
Note: Accessible in both host and device modes.
Bit 0 CMOD: Current mode of operation
Indicates the current mode.
0: Device mode
1: Host mode
Note: Accessible in both host and device modes.

73.14.7

OTG core interrupt register [alternate] (OTG_GINTSTS)
Valid for Device mode, see previous section for Host mode.
This register also indicates the current mode. To clear the interrupt status bits of the rc_w1
type, the application must write 1 into the bit.
This register interrupts the application for system-level events in the current mode (device
mode or host mode).
The FIFO status interrupts are read-only; once software reads from or writes to the FIFO
while servicing these interrupts, FIFO interrupt conditions are cleared automatically.
The application must clear the OTG_GINTSTS register at initialization before unmasking
the interrupt bit to avoid any interrupts generated prior to initialization.
Address offset: 0x014
Reset value: 0x0400 0020

31

30

29

28

27

26

25

23

22

21

20

19

18

17

16

IISOI
XFR

OEP
INT

IEPINT

Res.

Res.

HPRT
INT

RST
DET

DATAF
SUSP

IN
COMP
ISO
OUT

r

r

rc_w1

rc_w1

rc_w1

rc_w1

r

r

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

GO
NAK
EFF

GI
NAK
EFF

NPTXF
E

RXF
LVL

SOF

OTG
INT

MMIS

CMOD

r

r

r

r

rc_w1

r

rc_w1

r

WKUP
INT

SRQ
INT

DISC
INT

CIDS
CHG

LPM
INT

rc_w1

rc_w1

rc_w1

rc_w1

rc_w1

r

15

14

13

12

11

10

EOPF

ISOO
DRP

ENUM
DNE

USB
RST

USB
SUSP

ESUSP

Res.

rc_w1

rc_w1

rc_w1

rc_w1

rc_w1

rc_w1

<!-- pagebreak -->

24

PTXFE HCINT

RM0456 Rev 6

RM0456

USB on-the-go high-speed (OTG_HS)

Bit 31 WKUPINT: Resume/remote wake-up detected interrupt
Wake-up interrupt during suspend(L2) or LPM(L1) state.
– During suspend(L2):
In device mode, this interrupt is asserted when a resume is detected on the USB. In host
mode, this interrupt is asserted when a remote wake-up is detected on the USB.
– During LPM(L1):
This interrupt is asserted for either host initiated resume or device initiated remote wake-up
on USB.
Note: Accessible in both device and host modes.
Bit 30 SRQINT: Session request/new session detected interrupt
In host mode, this interrupt is asserted when a session request is detected from the device.
In device mode, this interrupt is asserted when VBUS is in the valid range for a B-peripheral
device. Accessible in both device and host modes.
Bit 29 DISCINT: Disconnect detected interrupt
Asserted when a device disconnect is detected.
Note: Only accessible in host mode.
Bit 28 CIDSCHG: Connector ID status change
The core sets this bit when there is a change in connector ID status.
Note: Accessible in both device and host modes.
Bit 27 LPMINT: LPM interrupt
In device mode, this interrupt is asserted when the device receives an LPM transaction and
responds with a non-ERRORed response.
In host mode, this interrupt is asserted when the device responds to an LPM transaction with
a non-ERRORed response or when the host core has completed LPM transactions for the
programmed number of times (RETRYCNT bit in OTG_GLPMCFG).
This field is valid only if the LPMEN bit in OTG_GLPMCFG is set to 1.
Bit 26 PTXFE: Periodic Tx FIFO empty
Asserted when the periodic transmit FIFO is either half or completely empty and there is
space for at least one entry to be written in the periodic request queue. The half or completely
empty status is determined by the periodic Tx FIFO empty level bit in the OTG_GAHBCFG
register (PTXFELVL bit in OTG_GAHBCFG).
Note: Only accessible in host mode.
Bit 25 HCINT: Host channels interrupt
The core sets this bit to indicate that an interrupt is pending on one of the channels of the
core (in host mode). The application must read the OTG_HAINT register to determine the
exact number of the channel on which the interrupt occurred, and then read the
corresponding OTG_HCINTx register to determine the exact cause of the interrupt. The
application must clear the appropriate status bit in the OTG_HCINTx register to clear this bit.
Note: Only accessible in host mode.
Bit 24 HPRTINT: Host port interrupt
The core sets this bit to indicate a change in port status of one of the OTG_HS controller
ports in host mode. The application must read the OTG_HPRT register to determine the
exact event that caused this interrupt. The application must clear the appropriate status bit in
the OTG_HPRT register to clear this bit.
Note: Only accessible in host mode.

RM0456 Rev 6

<!-- pagebreak -->

3460

USB on-the-go high-speed (OTG_HS)

RM0456

Bit 23 RSTDET: Reset detected interrupt
In device mode, this interrupt is asserted when a reset is detected on the USB in partial
power-down mode when the device is in suspend.
Note: Only accessible in device mode.
Bit 22 DATAFSUSP: Data fetch suspended
This interrupt is valid only in DMA mode. This interrupt indicates that the core has stopped
fetching data for IN endpoints due to the unavailability of TxFIFO space or request queue
space. This interrupt is used by the application for an endpoint mismatch algorithm. For
example, after detecting an endpoint mismatch, the application:
–
Sets a global nonperiodic IN NAK handshake
–
Disables IN endpoints
–
Flushes the FIFO
–
Determines the token sequence from the IN token sequence learning queue
–
Re-enables the endpoints
Clears the global nonperiodic IN NAK handshake If the global nonperiodic IN NAK is cleared,
the core has not yet fetched data for the IN endpoint, and the IN token is received: the core
generates an “IN token received when FIFO empty” interrupt. The OTG then sends a NAK
response to the host. To avoid this scenario, the application can check the FetSusp interrupt
in OTG_GINTSTS, which ensures that the FIFO is full before clearing a global NAK
handshake. Alternatively, the application can mask the “IN token received when FIFO empty”
interrupt when clearing a global IN NAK handshake.
Bit 21 INCOMPISOOUT: Incomplete isochronous OUT transfer
In device mode, the core sets this interrupt to indicate that there is at least one isochronous
OUT endpoint on which the transfer is not completed in the current frame. This interrupt is
asserted along with the End of periodic frame interrupt (EOPF) bit in this register.
Bit 20 IISOIXFR: Incomplete isochronous IN transfer
The core sets this interrupt to indicate that there is at least one isochronous IN endpoint on
which the transfer is not completed in the current frame. This interrupt is asserted along with
the End of periodic frame interrupt (EOPF) bit in this register.
Note: Only accessible in device mode.
Bit 19 OEPINT: OUT endpoint interrupt
The core sets this bit to indicate that an interrupt is pending on one of the OUT endpoints of
the core (in device mode). The application must read the OTG_DAINT register to determine
the exact number of the OUT endpoint on which the interrupt occurred, and then read the
corresponding OTG_DOEPINTx register to determine the exact cause of the interrupt. The
application must clear the appropriate status bit in the corresponding OTG_DOEPINTx
register to clear this bit.
Note: Only accessible in device mode.
Bit 18 IEPINT: IN endpoint interrupt
The core sets this bit to indicate that an interrupt is pending on one of the IN endpoints of the
core (in device mode). The application must read the OTG_DAINT register to determine the
exact number of the IN endpoint on which the interrupt occurred, and then read the
corresponding OTG_DIEPINTx register to determine the exact cause of the interrupt. The
application must clear the appropriate status bit in the corresponding OTG_DIEPINTx
register to clear this bit.
Note: Only accessible in device mode.
Bits 17:16 Reserved, must be kept at reset value.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

USB on-the-go high-speed (OTG_HS)

Bit 15 EOPF: End of periodic frame interrupt
Indicates that the period specified in the periodic frame interval field of the OTG_DCFG
register (PFIVL bit in OTG_DCFG) has been reached in the current frame.
Note: Only accessible in device mode.
Bit 14 ISOODRP: Isochronous OUT packet dropped interrupt
The core sets this bit when it fails to write an isochronous OUT packet into the Rx FIFO
because the Rx FIFO does not have enough space to accommodate a maximum size packet
for the isochronous OUT endpoint.
Note: Only accessible in device mode.
Bit 13 ENUMDNE: Enumeration done
The core sets this bit to indicate that speed enumeration is complete. The application must
read the OTG_DSTS register to obtain the enumerated speed.
Note: Only accessible in device mode.
Bit 12 USBRST: USB reset
The core sets this bit to indicate that a reset is detected on the USB.
Note: Only accessible in device mode.
Bit 11 USBSUSP: USB suspend
The core sets this bit to indicate that a suspend was detected on the USB. The core enters
the suspended state when there is no activity on the data lines for an extended period of
time.
Note: Only accessible in device mode.
Bit 10 ESUSP: Early suspend
The core sets this bit to indicate that an Idle state has been detected on the USB for 3 ms.
Note: Only accessible in device mode.
Bits 9:8 Reserved, must be kept at reset value.
Bit 7 GONAKEFF: Global OUT NAK effective
Indicates that the Set global OUT NAK bit in the OTG_DCTL register (SGONAK bit in
OTG_DCTL), set by the application, has taken effect in the core. This bit can be cleared by
writing the Clear global OUT NAK bit in the OTG_DCTL register (CGONAK bit in
OTG_DCTL).
Note: Only accessible in device mode.
Bit 6 GINAKEFF: Global IN non-periodic NAK effective
Indicates that the Set global non-periodic IN NAK bit in the OTG_DCTL register (SGINAK bit
in OTG_DCTL), set by the application, has taken effect in the core. That is, the core has
sampled the Global IN NAK bit set by the application. This bit can be cleared by clearing the
Clear global non-periodic IN NAK bit in the OTG_DCTL register (CGINAK bit in OTG_DCTL).
This interrupt does not necessarily mean that a NAK handshake is sent out on the USB. The
STALL bit takes precedence over the NAK bit.
Note: Only accessible in device mode.
Bit 5 NPTXFE: Non-periodic Tx FIFO empty
This interrupt is asserted when the non-periodic Tx FIFO is either half or completely empty,
and there is space for at least one entry to be written to the non-periodic transmit request
queue. The half or completely empty status is determined by the non-periodic Tx FIFO empty
level bit in the OTG_GAHBCFG register (TXFELVL bit in OTG_GAHBCFG).
Note: Accessible in host mode only.

RM0456 Rev 6

<!-- pagebreak -->

3460

USB on-the-go high-speed (OTG_HS)

RM0456

Bit 4 RXFLVL: Rx FIFO non-empty
Indicates that there is at least one packet pending to be read from the Rx FIFO.
Note: Accessible in both host and device modes.
Bit 3 SOF: Start of frame
In host mode, the core sets this bit to indicate that an SOF (FS), or Keep-Alive (LS) is
transmitted on the USB. The application must write a 1 to this bit to clear the interrupt.
In device mode, in the core sets this bit to indicate that an SOF token has been received on
the USB. The application can read the OTG_DSTS register to get the current frame number.
This interrupt is seen only when the core is operating in FS.
Note: This register may return '1' if read immediately after power on reset. If the register bit
reads '1' immediately after power on reset it does not indicate that an SOF has been
sent (in case of host mode) or SOF has been received (in case of device mode). The
read value of this interrupt is valid only after a valid connection between host and device
is established. If the bit is set after power on reset the application can clear the bit.
Note: Accessible in both host and device modes.
Bit 2 OTGINT: OTG interrupt
The core sets this bit to indicate an OTG protocol event. The application must read the OTG
interrupt status (OTG_GOTGINT) register to determine the exact event that caused this
interrupt. The application must clear the appropriate status bit in the OTG_GOTGINT register
to clear this bit.
Note: Accessible in both host and device modes.
Bit 1 MMIS: Mode mismatch interrupt
The core sets this bit when the application is trying to access:
– A host mode register, when the core is operating in device mode
– A device mode register, when the core is operating in host mode
The register access is completed on the AHB with an OKAY response, but is ignored by the
core internally and does not affect the operation of the core.
Note: Accessible in both host and device modes.
Bit 0 CMOD: Current mode of operation
Indicates the current mode.
0: Device mode
1: Host mode
Note: Accessible in both host and device modes.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

USB on-the-go high-speed (OTG_HS)

73.14.8

OTG interrupt mask register [alternate] (OTG_GINTMSK)
Valid for Host mode, see next section for Device mode.
This register works with the core interrupt register to interrupt the application. When an
interrupt bit is masked, the interrupt associated with that bit is not generated. However, the
core interrupt (OTG_GINTSTS) register bit corresponding to that interrupt is still set.
Address offset: 0x018
Reset value: 0x0000 0000

31
WUIM

30

29

28

SRQIM

DISC
INT

CIDSC
HGM

27

26

LPMIN PTXFE
TM
M

25
HCIM

24
PRTIM

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

IPXFR
M

Res.

Res.

Res.

Res.

Res.

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

rw

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

5

NPTXF RXFLV
EM
LM
rw

rw

SOFM
rw

OTGIN
MMISM
T
rw

Res.

rw

Bit 31 WUIM: Resume/remote wake-up detected interrupt mask
0: Masked interrupt
1: Unmasked interrupt
Bit 30 SRQIM: Session request/new session detected interrupt mask
0: Masked interrupt
1: Unmasked interrupt
Bit 29 DISCINT: Disconnect detected interrupt mask
0: Masked interrupt
1: Unmasked interrupt
Bit 28 CIDSCHGM: Connector ID status change mask
0: Masked interrupt
1: Unmasked interrupt
Bit 27 LPMINTM: LPM interrupt mask
0: Masked interrupt
1: Unmasked interrupt
Bit 26 PTXFEM: Periodic Tx FIFO empty mask
0: Masked interrupt
1: Unmasked interrupt
Bit 25 HCIM: Host channels interrupt mask
0: Masked interrupt
1: Unmasked interrupt
Bit 24 PRTIM: Host port interrupt mask
0: Masked interrupt
1: Unmasked interrupt
Bits 23:22 Reserved, must be kept at reset value.
Bit 21 IPXFRM: Incomplete periodic transfer mask
0: Masked interrupt
1: Unmasked interrupt

RM0456 Rev 6

<!-- pagebreak -->

3460

USB on-the-go high-speed (OTG_HS)

RM0456

Bits 20:6 Reserved, must be kept at reset value.
Bit 5 NPTXFEM: Non-periodic Tx FIFO empty mask
0: Masked interrupt
1: Unmasked interrupt
Bit 4 RXFLVLM: Receive FIFO non-empty mask
0: Masked interrupt
1: Unmasked interrupt
Bit 3 SOFM: Start of frame mask
0: Masked interrupt
1: Unmasked interrupt
Bit 2 OTGINT: OTG interrupt mask
0: Masked interrupt
1: Unmasked interrupt
Bit 1 MMISM: Mode mismatch interrupt mask
0: Masked interrupt
1: Unmasked interrupt
Bit 0 Reserved, must be kept at reset value.

73.14.9

OTG interrupt mask register [alternate] (OTG_GINTMSK)
Valid for Device mode, see previous section for Host mode.
This register works with the core interrupt register to interrupt the application. When an
interrupt bit is masked, the interrupt associated with that bit is not generated. However, the
core interrupt (OTG_GINTSTS) register bit corresponding to that interrupt is still set.
Address offset: 0x018
Reset value: 0x0000 0000

31
WUIM

30
SRQIM

rw

rw

15

14

EOPF
M

ISOOD
RPM

rw

rw

29

28

27

Res.

CIDSC
HGM

LPMIN
TM

rw

rw

13

12

11

26

rw

rw

24

23

22

RSTDE
TM

FSUS
PM

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

Res.

RXFLV
LM

SOFM

rw

rw

Res.

Res.

Res.

10

9

8

ENUM USBRS USBSU ESUSP
DNEM
T
SPM
M
rw

25

Res.

Res.

GONA GINAK
KEFFM EFFM

rw

rw

21

20

IISOOX IISOIX
FRM
FRM

rw

Bit 31 WUIM: Resume/remote wake-up detected interrupt mask
0: Masked interrupt
1: Unmasked interrupt
Bit 30 SRQIM: Session request/new session detected interrupt mask
0: Masked interrupt
1: Unmasked interrupt
Bit 29 Reserved, must be kept at reset value.
Bit 28 CIDSCHGM: Connector ID status change mask
0: Masked interrupt
1: Unmasked interrupt

<!-- pagebreak -->

RM0456 Rev 6

19

18

OEPIN
IEPINT
T

17

16

Res.

Res.

1

0

rw
2

OTGIN
MMISM
T
rw

rw

Res.

RM0456

USB on-the-go high-speed (OTG_HS)

Bit 27 LPMINTM: LPM interrupt mask
0: Masked interrupt
1: Unmasked interrupt
Bits 26:24 Reserved, must be kept at reset value.
Bit 23 RSTDETM: Reset detected interrupt mask
0: Masked interrupt
1: Unmasked interrupt
Bit 22 FSUSPM: Data fetch suspended mask
0: Masked interrupt
1: Unmasked interrupt
Bit 21 IISOOXFRM: Incomplete isochronous OUT transfer mask
0: Masked interrupt
1: Unmasked interrupt
Bit 20 IISOIXFRM: Incomplete isochronous IN transfer mask
0: Masked interrupt
1: Unmasked interrupt
Bit 19 OEPINT: OUT endpoints interrupt mask
0: Masked interrupt
1: Unmasked interrupt
Bit 18 IEPINT: IN endpoints interrupt mask
0: Masked interrupt
1: Unmasked interrupt
Bits 17:16 Reserved, must be kept at reset value.
Bit 15 EOPFM: End of periodic frame interrupt mask
0: Masked interrupt
1: Unmasked interrupt
Bit 14 ISOODRPM: Isochronous OUT packet dropped interrupt mask
0: Masked interrupt
1: Unmasked interrupt
Bit 13 ENUMDNEM: Enumeration done mask
0: Masked interrupt
1: Unmasked interrupt
Bit 12 USBRST: USB reset mask
0: Masked interrupt
1: Unmasked interrupt
Bit 11 USBSUSPM: USB suspend mask
0: Masked interrupt
1: Unmasked interrupt
Bit 10 ESUSPM: Early suspend mask
0: Masked interrupt
1: Unmasked interrupt
Bits 9:8 Reserved, must be kept at reset value.

RM0456 Rev 6

<!-- pagebreak -->

3460

USB on-the-go high-speed (OTG_HS)

RM0456

Bit 7 GONAKEFFM: Global OUT NAK effective mask
0: Masked interrupt
1: Unmasked interrupt
Bit 6 GINAKEFFM: Global non-periodic IN NAK effective mask
0: Masked interrupt
1: Unmasked interrupt
Bit 5 Reserved, must be kept at reset value.
Bit 4 RXFLVLM: Receive FIFO non-empty mask
0: Masked interrupt
1: Unmasked interrupt
Bit 3 SOFM: Start of frame mask
0: Masked interrupt
1: Unmasked interrupt
Bit 2 OTGINT: OTG interrupt mask
0: Masked interrupt
1: Unmasked interrupt
Bit 1 MMISM: Mode mismatch interrupt mask
0: Masked interrupt
1: Unmasked interrupt
Bit 0 Reserved, must be kept at reset value.

73.14.10 OTG receive status debug read register [alternate]
(OTG_GRXSTSR)
This description is for register OTG_GRXSTSR in Device mode.
A read to the receive status debug read register returns the contents of the top of the
receive FIFO.
The core ignores the receive status read when the receive FIFO is empty and returns a
value of 0x0000 0000.
Address offset: 0x01C
Reset value: 0x0000 0000
31

30

29

28

27

26

25

Res.

Res.

9

Res.

Res.

Res.

Res.

STSPH
ST

15

14

13

12

11

10

r

r

r

r

r

24

r

22

21

20

FRMNUM[3:0]

r

DPID[0]

23

19

17

PKTSTS[3:0]

16
DPID[1]

r

r

r

r

r

r

r

r

r

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

BCNT[10:0]
r

18

EPNUM[3:0]
r

r

r

Bits 31:28 Reserved, must be kept at reset value.
Bit 27 STSPHST: Status phase start
Indicates the start of the status phase for a control write transfer. This bit is set along with
the OUT transfer completed PKTSTS pattern.
Bits 26:25 Reserved, must be kept at reset value.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

USB on-the-go high-speed (OTG_HS)

Bits 24:21 FRMNUM[3:0]: Frame number
This is the least significant 4 bits of the frame number in which the packet is received on the
USB. This field is supported only when isochronous OUT endpoints are supported.
Bits 20:17 PKTSTS[3:0]: Packet status
Indicates the status of the received packet
0001: Global OUT NAK (triggers an interrupt)
0010: OUT data packet received
0011: OUT transfer completed (triggers an interrupt)
0100: SETUP transaction completed (triggers an interrupt)
0110: SETUP data packet received
Others: Reserved
Bits 16:15 DPID[1:0]: Data PID
Indicates the data PID of the received OUT data packet
00: DATA0
10: DATA1
01: DATA2
11: MDATA
Bits 14:4 BCNT[10:0]: Byte count
Indicates the byte count of the received data packet.
Bits 3:0 EPNUM[3:0]: Endpoint number
Indicates the endpoint number to which the current received packet belongs.

73.14.11 OTG receive status debug read register [alternate]
(OTG_GRXSTSR)
This description is for register OTG_GRXSTSR in Host mode.
A read to the receive status debug read register returns the contents of the top of the
receive FIFO.
The core ignores the receive status read when the receive FIFO is empty and returns a
value of 0x0000 0000.
Address offset: 0x01C
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

Res.

Res.

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

DPID
r

9

8

7

6

5

20

19

r

r

r

r

r

17

PKTSTS[3:0]

16
DPID

r

r

r

r

r

4

3

2

1

0

BCNT[10:0]
r

18

CHNUM[3:0]
r

r

RM0456 Rev 6

r

r

r

r

r

r

r

<!-- pagebreak -->

3460

USB on-the-go high-speed (OTG_HS)

RM0456

Bits 31:21 Reserved, must be kept at reset value.
Bits 20:17 PKTSTS[3:0]: Packet status
Indicates the status of the received packet
0010: IN data packet received
0011: IN transfer completed (triggers an interrupt)
0101: Data toggle error (triggers an interrupt)
0111: Channel halted (triggers an interrupt)
Others: Reserved
Bits 16:15 DPID[1:0]: Data PID
Indicates the data PID of the received packet
00: DATA0
10: DATA1
01: DATA2
11: MDATA
Bits 14:4 BCNT[10:0]: Byte count
Indicates the byte count of the received IN data packet.
Bits 3:0 CHNUM[3:0]: Channel number
Indicates the channel number to which the current received packet belongs.

73.14.12 OTG status read and pop registers (OTG_GRXSTSP)
This description is for register OTG_GRXSTSP in Device mode.
Similarly to OTG_GRXSTSR (receive status debug read register) where a read returns the
contents of the top of the receive FIFO, a read to OTG_GRXSTSP (receive status read and
pop register) additionally pops the top data entry out of the Rx FIFO.
The core ignores the receive status pop/read when the receive FIFO is empty and returns a
value of 0x0000 0000. The application must only pop the receive status FIFO when the
receive FIFO non-empty bit of the core interrupt register (RXFLVL bit in OTG_GINTSTS) is
asserted.
Address offset: 0x020
Reset value: 0x0000 0000
31

30

29

28

27

26

25

Res.

Res.

Res.

Res.

STSPH
ST

Res.

Res.

15

14

13

12

11

10

9

24

r

22

21

20

FRMNUM[3:0]

r

DPID[0]

23

19

r

r

r

r

r

17

PKTSTS[3:0]

16
DPID[1]

r

r

r

r

r

r

r

r

r

8

7

6

5

4

3

2

1

0

BCNT[10:0]
r

18

EPNUM[3:0]
r

r

r

r

r

r

r

r

r

Bits 31:28 Reserved, must be kept at reset value.
Bit 27 STSPHST: Status phase start
Indicates the start of the status phase for a control write transfer. This bit is set along with
the OUT transfer completed PKTSTS pattern.
Bits 26:25 Reserved, must be kept at reset value.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

USB on-the-go high-speed (OTG_HS)

Bits 24:21 FRMNUM[3:0]: Frame number
This is the least significant 4 bits of the frame number in which the packet is received on the
USB. This field is supported only when isochronous OUT endpoints are supported.
Bits 20:17 PKTSTS[3:0]: Packet status
Indicates the status of the received packet
0001: Global OUT NAK (triggers an interrupt)
0010: OUT data packet received
0011: OUT transfer completed (triggers an interrupt)
0100: SETUP transaction completed (triggers an interrupt)
0110: SETUP data packet received
Others: Reserved
Bits 16:15 DPID[1:0]: Data PID
Indicates the data PID of the received OUT data packet
00: DATA0
10: DATA1
01: DATA2
11: MDATA
Bits 14:4 BCNT[10:0]: Byte count
Indicates the byte count of the received data packet.
Bits 3:0 EPNUM[3:0]: Endpoint number
Indicates the endpoint number to which the current received packet belongs.

73.14.13 OTG status read and pop registers [alternate] (OTG_GRXSTSP)
This description is for register OTG_GRXSTSP in Host mode.
Similarly to OTG_GRXSTSR (receive status debug read register) where a read returns the
contents of the top of the receive FIFO, a read to OTG_GRXSTSP (receive status read and
pop register) additionally pops the top data entry out of the Rx FIFO.
The core ignores the receive status pop/read when the receive FIFO is empty and returns a
value of 0x0000 0000. The application must only pop the receive status FIFO when the
receive FIFO non-empty bit of the core interrupt register (RXFLVL bit in OTG_GINTSTS) is
asserted.
Address offset: 0x020
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

Res.

Res.

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

DPID
r

9

8

7

6

5

20

19

r

r

4

3

BCNT[10:0]
r

r

r

r

r

r

18

17

PKTSTS[3:0]

16
DPID

r

r

r

2

1

0

CHNUM[3:0]
r

r

RM0456 Rev 6

r

r

r

r

r

r

r

<!-- pagebreak -->

3460

USB on-the-go high-speed (OTG_HS)

RM0456

Bits 31:21 Reserved, must be kept at reset value.
Bits 20:17 PKTSTS[3:0]: Packet status
Indicates the status of the received packet
0010: IN data packet received
0011: IN transfer completed (triggers an interrupt)
0101: Data toggle error (triggers an interrupt)
0111: Channel halted (triggers an interrupt)
Others: Reserved
Bits 16:15 DPID[1:0]: Data PID
Indicates the data PID of the received packet
00: DATA0
10: DATA1
01: DATA2
11: MDATA
Bits 14:4 BCNT[10:0]: Byte count
Indicates the byte count of the received IN data packet.
Bits 3:0 CHNUM[3:0]: Channel number
Indicates the channel number to which the current received packet belongs.

73.14.14 OTG receive FIFO size register (OTG_GRXFSIZ)
The application can program the RAM size that must be allocated to the Rx FIFO.
Address offset: 0x024
Reset value: 0x0000 0400
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

rw

rw

rw

rw

rw

rw

rw

RXFD[15:0]
rw

rw

rw

rw

rw

rw

rw

rw

rw

Bits 31:16 Reserved, must be kept at reset value.
Bits 15:0 RXFD[15:0]: Rx FIFO depth
This value is in terms of 32-bit words.
Maximum value is 1024
Programmed values must respect the available FIFO memory allocation and must not
exceed the power-on value.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

USB on-the-go high-speed (OTG_HS)

73.14.15 OTG host non-periodic transmit FIFO size register [alternate]
(OTG_HNPTXFSIZ)
Valid for Host mode, see next section for Device mode.
Address offset: 0x028
Reset value: 0x0200 0200
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

NPTXFD[15:0]
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

NPTXFSA[15:0]
rw

rw

Bits 31:16 NPTXFD[15:0]: Non-periodic Tx FIFO depth
This value is in terms of 32-bit words.
Minimum value is 16
Programmed values must respect the available FIFO memory allocation and must not
exceed the power-on value.
Bits 15:0 NPTXFSA[15:0]: Non-periodic transmit RAM start address
This field configures the memory start address for non-periodic transmit FIFO RAM.

73.14.16 Endpoint 0 transmit FIFO size [alternate] (OTG_DIEPTXF0)
Valid for Device mode, see previous section for Host mode.
Address offset: 0x028
Reset value: 0x0200 0200
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

TX0FD[15:0]
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

TX0FSA[15:0]
rw

rw

Bits 31:16 TX0FD[15:0]: Endpoint 0 Tx FIFO depth
This value is in terms of 32-bit words.
Minimum value is 16
Programmed values must respect the available FIFO memory allocation and must not
exceed the power-on value.
Bits 15:0 TX0FSA[15:0]: Endpoint 0 transmit RAM start address
This field configures the memory start address for the endpoint 0 transmit FIFO RAM.

RM0456 Rev 6

<!-- pagebreak -->

3460

USB on-the-go high-speed (OTG_HS)

RM0456

73.14.17 OTG non-periodic transmit FIFO/queue status register
(OTG_HNPTXSTS)
Note:

In device mode, this register is not valid.
This read-only register contains the free space information for the non-periodic Tx FIFO and
the non-periodic transmit request queue.
Address offset: 0x02C
Reset value: 0x0008 0400

31

30

29

28

Res.

27

26

25

24

23

22

21

NPTXQTOP[6:0]

20

19

18

17

16

NPTQXSAV[7:0]

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

NPTXFSAV[15:0]
r

r

Bit 31 Reserved, must be kept at reset value.
Bits 30:24 NPTXQTOP[6:0]: Top of the non-periodic transmit request queue
Entry in the non-periodic Tx request queue that is currently being processed by the MAC.
Bits 30:27: Channel/endpoint number
Bits 26:25:
XXXX00X: IN/OUT token
XXXX01X: Zero-length transmit packet (device IN/host OUT)
XXXX11X: Channel halt command
Bit 24: Terminate (last entry for selected channel/endpoint)
Bits 23:16 NPTQXSAV[7:0]: Non-periodic transmit request queue space available
Indicates the amount of free space available in the non-periodic transmit request queue.
This queue holds both IN and OUT requests.
0: Non-periodic transmit request queue is full
1: 1 location available
2: locations available
n: n locations available (0 ≤ n ≤ 8)
Others: Reserved
Bits 15:0 NPTXFSAV[15:0]: Non-periodic Tx FIFO space available
Indicates the amount of free space available in the non-periodic Tx FIFO.
Values are in terms of 32-bit words.
0: Non-periodic Tx FIFO is full
1: 1 word available
2: 2 words available
n: n words available (where 0 ≤ n ≤ 512)
Others: Reserved

<!-- pagebreak -->

RM0456 Rev 6

RM0456

USB on-the-go high-speed (OTG_HS)

73.14.18 OTG general core configuration register (OTG_GCCFG)
This register is available in host and device modes.
Address offset: 0x038
Reset value: 0x0000 XXXX
31

30

29

28

27

26

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

Res.

Res.

Res.

Res.

Res.

Res.

25

24

23

FORCE
VBVAL VBVAL
HOSTP
OVEN OVAL
D

22

21

20

SDEN

VBDEN

PDEN

19

18

17

16

DCDE HVDM HCDP HCDPE
N
SRCEN DETEN
N

rw

rw

rw

rw

rw

rw

rw

rw

rw

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

SESS
VLD

FSVMI
NUS

FSVPL
US

CHGD
ET

r

r

r

r

Res.

Res.

Res.

Res.

Res.

rw

Bits 31:29 Reserved, must be kept at reset value.
Bit 28 Reserved, must be kept at reset value.
Bit 27 Reserved, must be kept at reset value.
Bit 26 Reserved, must be kept at reset value.
Bit 25 FORCEHOSTPD: Force host mode pull-downs
If the ID pin functions are enabled, the host mode pull-downs on DP and DM activate
automatically. However, whenever that is not the case, yet host mode is required, this bit
must be used to force the pull-downs active.
0: Do not force host mode pull-downs
1: Force host mode pull-downs
Bit 24 VBVALOVEN: Enables a software override of the VBUS B-session detection.
0: Use hardware
1: Use VBVALOVAL to indicate B-session active
Bit 23 VBVALOVAL: Software override value of the VBUS B-session detection
0: B-session inactive
1: B-session active
Bit 22 SDEN: Battery charging specification – secondary detection enable
0: Secondary detection disabled
1: Secondary detection enabled
Bit 21 VBDEN: VBUS detection enable
Enables VBUS sensing comparators in order to detect VBUS presence and/or perform OTG
operation.
0: VBUS detection disabled
1: VBUS detection enabled
Bit 20 PDEN: Battery charging specification – primary detection enable
0: Primary detection disabled
1: Primary detection enabled
Bit 19 DCDEN: Battery charging specification – data contact detection enable
0: Data Contact Detection disabled
1: Data Contact Detection enabled

RM0456 Rev 6

<!-- pagebreak -->

3460

USB on-the-go high-speed (OTG_HS)

RM0456

Bit 18 HVDMSRCEN: Battery charging specification – host CDP port voltage source enable on DM
0: DM voltage source disabled
1: DM Voltage source enabled
Bit 17 HCDPDETEN: Battery charging specification – host CDP port voltage detector enable on DP
0: DP voltage detection disabled
1: DP voltage detection enabled
Bit 16 HCDPEN: Battery charging specification – host CDP behavior enable
0: Disable CDP behavior
1: Enable CDP behavior
Bits 15:5 Reserved, must be kept at reset value.
Bit 4 Reserved, must be kept at reset value.
Bit 3 SESSVLD: VBUS session indicator
Indicates if VBUS is above VBUS session threshold.
0: VBUS is below VBUS session threshold
1: VBUS is above VBUS session threshold
Bit 2 FSVMINUS: Battery charging specification – single-ended DM indicator
This bit gives the voltage level on DM (also result of the comparison with VLGC threshold as
defined in BC v1.2 standard).
0: DM voltage at low level
1: DM voltage at high level
Bit 1 FSVPLUS: Battery charging specification – single-ended DP indicator
This bit gives the voltage level on DP (also result of the comparison with VLGC threshold as
defined in BC v1.2 standard).
0: DM voltage at low level
1: DM voltage at high level
Bit 0 CHGDET: Battery charging specification – charger detection, result of the current mode
(primary or secondary).
0: Low value on pin
1: High value on pin

73.14.19 OTG core ID register (OTG_CID)
This is a register containing the Product ID as reset value.
Address offset: 0x03C
Reset value: 0x0000 5000
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

PRODUCT_ID[31:16]
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

rw

rw

rw

rw

rw

rw

rw

rw

rw

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

PRODUCT_ID[15:0]
rw

rw

rw

rw

rw

rw

rw

rw

rw

Bits 31:0 PRODUCT_ID[31:0]: Product ID field
Application-programmable ID field.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

USB on-the-go high-speed (OTG_HS)

73.14.20 OTG core LPM configuration register (OTG_GLPMCFG)
Address offset: 0x054
Reset value: 0x0000 0000
31

30

29

28

27

26

25

24

LPMRCNTSTS[2:0]

SND
LPM

23

22

21

20

19

18

17

16
L1RSM
OK

Res.

Res.

Res.

EN
BESL
rw

r

r

r

rs

rw

rw

rw

rw

rw

rw

rw

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

SLP
STS

LPMRSP[1:0]

L1DS
EN

L1SS
EN

REM
WAKE

LPM
ACK

LPM
EN

rw

rw

rw

rw

r

r

r

rw

BESLTHRS[3:0]
rw

rw

rw

rw

LPMRCNT[2:0]

LPMCHIDX[3:0]

BESL[3:0]
rw

rw

rw

rw

Bits 31:29 Reserved, must be kept at reset value.
Bit 28 ENBESL: Enable best effort service latency
This bit enables the BESL feature as defined in the LPM errata:
0: The core works as described in the following document:
USB 2.0 Link Power Management Addendum Engineering Change Notice to the USB 2.0
specification, July 16, 2007
1: The core works as described in the LPM Errata:
Errata for USB 2.0 ECN: Link Power Management (LPM) - 7/2007
Note: Only the updated behavior (described in LPM Errata) is considered in this document
and so the ENBESL bit must be set to '1' by application SW.
Bits 27:25 LPMRCNTSTS[2:0]: LPM retry count status
Number of LPM host retries still remaining to be transmitted for the current LPM sequence.
Note: Accessible only in host mode.
Bit 24 SNDLPM: Send LPM transaction
When the application software sets this bit, an LPM transaction containing two tokens, EXT
and LPM is sent. The hardware clears this bit once a valid response (STALL, NYET, or
ACK) is received from the device or the core has finished transmitting the programmed
number of LPM retries.
Note: This bit must be set only when the host is connected to a local port.
Note: Accessible only in host mode.
Bits 23:21 LPMRCNT[2:0]: LPM retry count
When the device gives an ERROR response, this is the number of additional LPM retries
that the host performs until a valid device response (STALL, NYET, or ACK) is received.
Note: Accessible only in host mode.
Bits 20:17 LPMCHIDX[3:0]: LPM Channel Index
The channel number on which the LPM transaction has to be applied while sending an LPM
transaction to the local device. Based on the LPM channel index, the core automatically
inserts the device address and endpoint number programmed in the corresponding channel
into the LPM transaction.
Note: Accessible only in host mode.

RM0456 Rev 6

<!-- pagebreak -->

3460

USB on-the-go high-speed (OTG_HS)

RM0456

Bit 16 L1RSMOK: Sleep state resume OK
Indicates that the device or host can start resume from Sleep state. This bit is valid in LPM
sleep (L1) state. It is set in sleep mode after a delay of 50 μs (TL1Residency).
This bit is reset when SLPSTS is 0.
1: The application or host can start resume from Sleep state
0: The application or host cannot start resume from Sleep state
Bit 15 SLPSTS: Port sleep status
Device mode:
This bit is set as long as a Sleep condition is present on the USB bus. The core enters the
Sleep state when an ACK response is sent to an LPM transaction and the TL1TokenRetry
timer has expired. To stop the PHY clock, the application must set the STPPCLK bit in
OTG_PCGCCTL, which asserts the PHY suspend input signal.
The application must rely on SLPSTS and not ACK in LPMRSP to confirm transition into
sleep.
The core comes out of sleep:
– When there is any activity on the USB linestate
– When the application writes to the RWUSIG bit in OTG_DCTL or when the application
resets or soft-disconnects the device.
Host mode:
The host transitions to Sleep (L1) state as a side-effect of a successful LPM transaction by
the core to the local port with ACK response from the device. The read value of this bit
reflects the current Sleep status of the port.
The core clears this bit after:
– The core detects a remote L1 wake-up signal,
– The application sets the PRST bit or the PRES bit in the OTG_HPRT register, or
– The application sets the L1Resume/ remote wake-up detected interrupt bit or disconnect
detected interrupt bit in the core interrupt register (WKUPINT or DISCINT bit in
OTG_GINTSTS, respectively).
0: Core not in L1
1: Core in L1
Bits 14:13 LPMRSP[1:0]: LPM response
Device mode:
The response of the core to LPM transaction received is reflected in these two bits.
Host mode:
Handshake response received from local device for LPM transaction
11: ACK
10: NYET
01: STALL
00: ERROR (No handshake response)
Bit 12 L1DSEN: L1 deep sleep enable
Enables suspending the PHY in L1 Sleep mode. For maximum power saving during L1
Sleep mode, this bit must be set to '1' by application SW in all the cases.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

USB on-the-go high-speed (OTG_HS)

Bits 11:8 BESLTHRS[3:0]: BESL threshold
Device mode:
The core puts the PHY into deep low power mode in L1 when BESL value is greater than or
equal to the value defined in this field BESL_Thres[3:0].
Host mode:
The core puts the PHY into deep low power mode in L1. BESLTHRS[3:0] specifies the time
for which resume signaling is to be reflected by host (TL1HubDrvResume2) on the USB bus
when it detects device initiated resume.
BESLTHRS must not be programmed with a value greater than 1100b in host mode,
because this exceeds maximum TL1HubDrvResume2.
Thres[3:0] host mode resume signaling time (μs):
0000: 75
0001: 100
0010: 150
0011: 250
0100: 350
0101: 450
0110: 950
All other values: reserved
Bit 7 L1SSEN: L1 Shallow Sleep enable
Enables suspending the PHY in L1 Sleep mode. For maximum power saving during L1
Sleep mode, this bit must be set to '1' by application SW in all the cases.
Bit 6 REMWAKE: bRemoteWake value
Host mode:
The value of remote wake up to be sent in the wIndex field of LPM transaction.
Device mode (read-only):
This field is updated with the received LPM token bRemoteWake bmAttribute when an ACK,
NYET, or STALL response is sent to an LPM transaction.

RM0456 Rev 6

<!-- pagebreak -->

3460

USB on-the-go high-speed (OTG_HS)

RM0456

Bits 5:2 BESL[3:0]: Best effort service latency
Host mode:
The value of BESL to be sent in an LPM transaction. This value is also used to initiate
resume for a duration TL1HubDrvResume1 for host initiated resume.
Device mode (read-only):
This field is updated with the received LPM token BESL bmAttribute when an ACK, NYET,
or STALL response is sent to an LPM transaction.
BESL[3:0]TBESL (μs)
0000: 125
0001: 150
0010: 200
0011: 300
0100: 400
0101: 500
0110: 1000
0111: 2000
1000: 3000
1001: 4000
1010: 5000
1011: 6000
1100: 7000
1101: 8000
1110: 9000
1111: 10000
Bit 1 LPMACK: LPM token acknowledge enable
Handshake response to LPM token preprogrammed by device application software.
1: ACK
Even though ACK is preprogrammed, the core device responds with ACK only on
successful LPM transaction. The LPM transaction is successful if:
– No PID/CRC5 errors in either EXT token or LPM token (else ERROR)
– Valid bLinkState = 0001B (L1) received in LPM transaction (else STALL)
– No data pending in transmit queue (else NYET).
0: NYET
The preprogrammed software bit is over-ridden for response to LPM token when:
– The received bLinkState is not L1 (STALL response), or
– An error is detected in either of the LPM token packets because of corruption (ERROR
response).
Note: Accessible only in device mode.
Bit 0 LPMEN: LPM support enable
The application uses this bit to control the OTG_HS core LPM capabilities.
If the core operates as a non-LPM-capable host, it cannot request the connected device or
hub to activate LPM mode.
If the core operates as a non-LPM-capable device, it cannot respond to any LPM
transactions.
0: LPM capability is not enabled
1: LPM capability is enabled

<!-- pagebreak -->

RM0456 Rev 6

RM0456

USB on-the-go high-speed (OTG_HS)

73.14.21 OTG host periodic transmit FIFO size register
(OTG_HPTXFSIZ)
Address offset: 0x100
Reset value: 0x0400 0800
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

PTXFSIZ[15:0]
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

PTXSA[15:0]
rw

rw

rw

rw

rw

rw

rw

rw

rw

Bits 31:16 PTXFSIZ[15:0]: Host periodic Tx FIFO depth
This value is in terms of 32-bit words.
Minimum value is 16
Bits 15:0 PTXSA[15:0]: Host periodic Tx FIFO start address
This field configures the memory start address for periodic transmit FIFO RAM.

73.14.22 OTG device IN endpoint transmit FIFO x size register
(OTG_DIEPTXFx)
Address offset: 0x104 + 0x04 * (x - 1), (x = 1 to 8)
Reset value: 0x0200 0400, 0x0200 0600, 0x0200 0800, 0x0200 0A00, 0x0200 0C00,
0x0200 0E00, 0x0200 1000, 0x0200 1200
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

INEPTXFD[15:0]

INEPTXSA[15:0]
rw

rw

rw

rw

rw

rw

rw

rw

rw

Bits 31:16 INEPTXFD[15:0]: IN endpoint Tx FIFO depth
This value is in terms of 32-bit words.
Minimum value is 16
Bits 15:0 INEPTXSA[15:0]: IN endpoint FIFOx transmit RAM start address
This field contains the memory start address for IN endpoint transmit FIFOx. The address
must be aligned with a 32-bit memory location.

73.14.23 Host-mode registers
Bit values in the register descriptions are expressed in binary unless otherwise specified.
Host-mode registers affect the operation of the core in the host mode. Host mode registers
must not be accessed in device mode, as the results are undefined. Host mode registers
can be categorized as follows:

RM0456 Rev 6

<!-- pagebreak -->

3460

USB on-the-go high-speed (OTG_HS)

RM0456

73.14.24 OTG host configuration register (OTG_HCFG)
This register configures the core after power-on. Do not make changes to this register after
initializing the host.
Address offset: 0x400
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

3

2

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

FSLSS
r

FSLSPCS[1:0]
rw

rw

Bits 31:3 Reserved, must be kept at reset value.
Bit 2 FSLSS: FS- and LS-only support
The application uses this bit to control the core enumeration speed. Using this bit, the
application can make the core enumerate as an FS host, even if the connected device
supports HS traffic. Do not make changes to this field after initial programming.
Bits 1:0 FSLSPCS[1:0]: FS/LS PHY clock select
Condition: FS host mode
01: PHY clock is running at 48 MHz
Others: Reserved
Condition: LS host mode
00: Reserved
01: Select 48 MHz PHY clock frequency
10: Select 6 MHz PHY clock frequency
11: Reserved
Note: The FSLSPCS must be set on a connection event according to the speed of the
connected device (after changing this bit, a software reset must be performed).

73.14.25 OTG host frame interval register (OTG_HFIR)
This register stores the frame interval information for the current speed to which the
OTG_HS controller has enumerated.
Address offset: 0x404
Reset value: 0x0000 EA60
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
RLD
CTRL

Res.

Res.

Res.

Res.

Res.

Res.

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

rw

rw

rw

rw

rw

rw

rw

rw

FRIVL[15:0]
rw

rw

<!-- pagebreak -->

rw

rw

rw

rw

rw

rw

rw

RM0456 Rev 6

RM0456

USB on-the-go high-speed (OTG_HS)

Bits 31:17 Reserved, must be kept at reset value.
Bit 16 RLDCTRL: Reload control
This bit allows dynamic reloading of the HFIR register during run time.
0: The HFIR cannot be reloaded dynamically
1: The HFIR can be dynamically reloaded during run time.
This bit needs to be programmed during initial configuration and its value must not be
changed during run time.
Caution: RLDCTRL = 0 is not recommended.
Bits 15:0 FRIVL[15:0]: Frame interval
The value that the application programs to this field, specifies the interval between two
consecutive micro-SOFs (HS) or Keep-Alive tokens (LS). This field contains the number of
PHY clocks that constitute the required frame interval. The application can write a value to
this register only after the port enable bit of the host port control and status register (PENA
bit in OTG_HPRT) has been set. If no value is programmed, the core calculates the value
based on the PHY clock specified in the FS/LS PHY clock select field of the host
configuration register (FSLSPCS in OTG_HCFG). Do not change the value of this field after
the initial configuration, unless the RLDCTRL bit is set. In such case, the FRIVL is reloaded
with each SOF event.
– Frame interval = 125 μs × (FRIVL - 1)
in high speed operation
– Frame interval = 1 ms × (FRIVL - 1)
in low/full speed operation

73.14.26 OTG host frame number/frame time remaining register
(OTG_HFNUM)
This register indicates the current frame number. It also indicates the time remaining (in
terms of the number of PHY clocks) in the current frame.
Address offset: 0x408
Reset value: 0x0000 3FFF
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

FTREM[15:0]
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

FRNUM[15:0]
r

r

Bits 31:16 FTREM[15:0]: Frame time remaining
Indicates the amount of time remaining in the current frame, in terms of PHY clocks. This
field decrements on each PHY clock. When it reaches zero, this field is reloaded with the
value in the Frame interval register and a new SOF is transmitted on the USB.
Bits 15:0 FRNUM[15:0]: Frame number
This field increments when a new SOF is transmitted on the USB, and is cleared to 0 when
it reaches 0x3FFF.

RM0456 Rev 6

<!-- pagebreak -->

3460

USB on-the-go high-speed (OTG_HS)

RM0456

73.14.27 OTG_Host periodic transmit FIFO/queue status register
(OTG_HPTXSTS)
This read-only register contains the free space information for the periodic Tx FIFO and the
periodic transmit request queue.
Address offset: 0x410
Reset value: 0x0008 0100
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

PTXQTOP[7:0]

20

19

18

17

16

PTXQSAV[7:0]

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

PTXFSAVL[15:0]
r

r

Bits 31:24 PTXQTOP[7:0]: Top of the periodic transmit request queue
This indicates the entry in the periodic Tx request queue that is currently being processed by
the MAC.
This register is used for debugging.
Bit 31: Odd/Even frame
0XXXXXXX: send in even frame
1XXXXXXX: send in odd frame
Bits 30:27: Channel/endpoint number
Bits 26:25: Type
XXXXX00X: IN/OUT
XXXXX01X: Zero-length packet
XXXXX11X: Disable channel command
Bit 24: Terminate (last entry for the selected channel/endpoint)
Bits 23:16 PTXQSAV[7:0]: Periodic transmit request queue space available
Indicates the number of free locations available to be written in the periodic transmit request
queue. This queue holds both IN and OUT requests.
0: Periodic transmit request queue is full
1: 1 location available
2: 2 locations available
n: n locations available (0 ≤ n ≤ 8)
Others: Reserved
Bits 15:0 PTXFSAVL[15:0]: Periodic transmit data FIFO space available
Indicates the number of free locations available to be written to in the periodic Tx FIFO.
Values are in terms of 32-bit words
0: Periodic Tx FIFO is full
1: 1 word available
2: 2 words available
n: n words available (where 0 ≤ n ≤ PTXFD)
Others: Reserved

<!-- pagebreak -->

RM0456 Rev 6

RM0456

USB on-the-go high-speed (OTG_HS)

73.14.28 OTG host all channels interrupt register (OTG_HAINT)
When a significant event occurs on a channel, the host all channels interrupt register
interrupts the application using the host channels interrupt bit of the core interrupt register
(HCINT bit in OTG_GINTSTS). This is shown in Figure 926. There is one interrupt bit per
channel, up to a maximum of 16 bits. Bits in this register are set and cleared when the
application sets and clears bits in the corresponding host channel-x interrupt register.
Address offset: 0x414
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

r

r

r

r

r

r

r

HAINT[15:0]
r

r

r

r

r

r

r

r

r

Bits 31:16 Reserved, must be kept at reset value.
Bits 15:0 HAINT[15:0]: Channel interrupts
One bit per channel: Bit 0 for Channel 0, bit 15 for Channel 15

73.14.29 OTG host all channels interrupt mask register
(OTG_HAINTMSK)
The host all channel interrupt mask register works with the host all channel interrupt register
to interrupt the application when an event occurs on a channel. There is one interrupt mask
bit per channel, up to a maximum of 16 bits.
Address offset: 0x418
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

HAINTM[15:0]
rw

rw

Bits 31:16 Reserved, must be kept at reset value.
Bits 15:0 HAINTM[15:0]: Channel interrupt mask
0: Masked interrupt
1: Unmasked interrupt
One bit per channel: Bit 0 for channel 0, bit 15 for channel 15

RM0456 Rev 6

<!-- pagebreak -->

3460

USB on-the-go high-speed (OTG_HS)

RM0456

73.14.30 OTG host port control and status register (OTG_HPRT)
This register is available only in host mode. Currently, the OTG host supports only one port.
A single register holds USB port-related information such as USB reset, enable, suspend,
resume, connect status, and test mode for each port. It is shown in Figure 926. The rc_w1
bits in this register can trigger an interrupt to the application through the host port interrupt
bit of the core interrupt register (HPRTINT bit in OTG_GINTSTS). On a port interrupt, the
application must read this register and clear the bit that caused the interrupt. For the rc_w1
bits, the application must write a 1 to the bit to clear the interrupt.
Address offset: 0x440
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

Res.

Res.

Res.

Res.

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

PRST

PSUSP

PRES

POC
CHNG

POCA

PEN
CHNG

PENA

PCDET PCSTS

rw

rs

rw

rc_w1

r

rc_w1

rc_w1

rc_w1

PTCTL[2:0]
rw

rw

PPWR
rw

rw

PLSTS[1:0]
r

r

PSPD[1:0]

16
PTCTL
[3]

r

Bits 31:19 Reserved, must be kept at reset value.
Bits 18:17 PSPD[1:0]: Port speed
Indicates the speed of the device attached to this port.
01: Full speed
10: Low speed
11: Reserved
00: High speed
Bits 16:13 PTCTL[3:0]: Port test control
The application writes a nonzero value to this field to put the port into a Test mode, and the
corresponding pattern is signaled on the port.
0000: Test mode disabled
0001: Test_J mode
0010: Test_K mode
0011: Test_SE0_NAK mode
0100: Test_Packet mode
0101: Test_Force_Enable
Others: Reserved
Bit 12 PPWR: Port power
The application uses this field to control power to this port, and the core clears this bit on an
overcurrent condition. Note that this bit does not directly activate the voltage on the
connector VBUS.
0: Power off
1: Power on
Bits 11:10 PLSTS[1:0]: Port line status
Indicates the current logic level USB data lines
Bit 10: Logic level of OTG_HS_DP
Bit 11: Logic level of OTG_HS_DM

<!-- pagebreak -->

RM0456 Rev 6

RM0456

USB on-the-go high-speed (OTG_HS)

Bit 9 Reserved, must be kept at reset value.
Bit 8 PRST: Port reset
When the application sets this bit, a reset sequence is started on this port. The application
must time the reset period and clear this bit after the reset sequence is complete.
0: Port not in reset
1: Port in reset
The application must leave this bit set for a minimum duration of at least 10 ms to start a
reset on the port. The application can leave it set for another 10 ms in addition to the required
minimum duration, before clearing the bit, even though there is no maximum limit set by the
USB standard.
High speed: 50 ms
Full speed/Low speed: 10 ms
Bit 7 PSUSP: Port suspend
The application sets this bit to put this port in suspend mode. The core only stops sending
SOFs when this is set. To stop the PHY clock, the application must set the port clock stop bit,
which asserts the suspend input pin of the PHY.
The read value of this bit reflects the current suspend status of the port. This bit is cleared by
the core after a remote wake-up signal is detected or the application sets the port reset bit or
port resume bit in this register or the resume/remote wake-up detected interrupt bit or
disconnect detected interrupt bit in the core interrupt register (WKUPINT or DISCINT in
OTG_GINTSTS, respectively).
0: Port not in suspend mode
1: Port in suspend mode
Bit 6 PRES: Port resume
The application sets this bit to drive resume signaling on the port. The core continues to drive
the resume signal until the application clears this bit.
If the core detects a USB remote wake-up sequence, as indicated by the port resume/remote
wake-up detected interrupt bit of the core interrupt register (WKUPINT bit in
OTG_GINTSTS), the core starts driving resume signaling without application intervention
and clears this bit when it detects a disconnect condition. The read value of this bit indicates
whether the core is currently driving resume signaling.
0: No resume driven
1: Resume driven
When LPM is enabled and the core is in L1 state, the behavior of this bit is as follow:
1. The application sets this bit to drive resume signaling on the port.
2. The core continues to drive the resume signal until a predetermined time specified in
BESLTHRS[3:0] field of OTG_GLPMCFG register.
3. If the core detects a USB remote wake-up sequence, as indicated by the port
L1Resume/Remote L1wake-up detected interrupt bit of the core interrupt register (WKUPINT
in OTG_GINTSTS), the core starts driving resume signaling without application intervention
and clears this bit at the end of resume.This bit can be set or cleared by both the core and
the application. This bit is cleared by the core even if there is no device connected to the
host.
Bit 5 POCCHNG: Port overcurrent change
The core sets this bit when the status of the port overcurrent active bit (bit 4) in this register
changes.
Bit 4 POCA: Port overcurrent active
Indicates the overcurrent condition of the port.
0: No overcurrent condition
1: Overcurrent condition

RM0456 Rev 6

<!-- pagebreak -->

3460

USB on-the-go high-speed (OTG_HS)

RM0456

Bit 3 PENCHNG: Port enable/disable change
The core sets this bit when the status of the port enable bit 2 in this register changes.
Bit 2 PENA: Port enable
A port is enabled only by the core after a reset sequence, and is disabled by an overcurrent
condition, a disconnect condition, or by the application clearing this bit. The application
cannot set this bit by a register write. It can only clear it to disable the port. This bit does not
trigger any interrupt to the application.
0: Port disabled
1: Port enabled
Bit 1 PCDET: Port connect detected
The core sets this bit when a device connection is detected to trigger an interrupt to the
application using the host port interrupt bit in the core interrupt register (HPRTINT bit in
OTG_GINTSTS). The application must write a 1 to this bit to clear the interrupt.
Bit 0 PCSTS: Port connect status
0: No device is attached to the port
1: A device is attached to the port

73.14.31 OTG host channel x characteristics register (OTG_HCCHARx)
Address offset: 0x500 + 0x20 * x, (x = 0 to 15)
Reset value: 0x0000 0000
31

30

CHENA CHDIS
rs

rs

15

14

EPDIR
rw

29

28

27

26

ODD
FRM

25

24

23

22

DAD[6:0]

rw

rw

rw

rw

rw

rw

rw

rw

13

12

11

10

9

8

7

6

EPNUM[3:0]
rw

rw

rw

21

20

19

18

17

16

LSDEV

Res.

MCNT[1:0]

EPTYP[1:0]

rw

rw

rw

rw

rw

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

MPSIZ[10:0]
rw

rw

rw

rw

rw

rw

rw

Bit 31 CHENA: Channel enable
This field is set by the application and cleared by the OTG host.
0: Channel disabled
1: Channel enabled
Bit 30 CHDIS: Channel disable
The application sets this bit to stop transmitting/receiving data on a channel, even before
the transfer for that channel is complete. The application must wait for the Channel disabled
interrupt before treating the channel as disabled.
Bit 29 ODDFRM: Odd frame
This field is set (reset) by the application to indicate that the OTG host must perform a
transfer in an odd frame. This field is applicable for only periodic (isochronous and interrupt)
transactions.
0: Even frame
1: Odd frame
Bits 28:22 DAD[6:0]: Device address
This field selects the specific device serving as the data source or sink.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

USB on-the-go high-speed (OTG_HS)

Bits 21:20 MCNT[1:0]: Multicount
This field indicates to the host the number of transactions that must be executed per frame
for this periodic endpoint. For non-periodic transfers, this field is not used
00: Reserved. This field yields undefined results
01: 1 transaction
10: 2 transactions per frame to be issued for this endpoint
11: 3 transactions per frame to be issued for this endpoint
Note: This field must be set to at least 01.
Bits 19:18 EPTYP[1:0]: Endpoint type
Indicates the transfer type selected.
00: Control
01: Isochronous
10: Bulk
11: Interrupt
Bit 17 LSDEV: Low-speed device
This field is set by the application to indicate that this channel is communicating to a lowspeed device.
Bit 16 Reserved, must be kept at reset value.
Bit 15 EPDIR: Endpoint direction
Indicates whether the transaction is IN or OUT.
0: OUT
1: IN
Bits 14:11 EPNUM[3:0]: Endpoint number
Indicates the endpoint number on the device serving as the data source or sink.
Bits 10:0 MPSIZ[10:0]: Maximum packet size
Indicates the maximum packet size of the associated endpoint.

73.14.32 OTG host channel x split control register (OTG_HCSPLTx)
Address offset: 0x504 + 0x20 * x, (x = 0 to 15)
Reset value: 0x0000 0000
31
SPLIT
EN

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
COMP
LSPLT

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

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
15

rw

XACTPOS[1:0]
rw

rw

HUBADDR[6:0]
rw

PRTADDR[6:0]
rw

Bit 31 SPLITEN: Split enable
The application sets this bit to indicate that this channel is enabled to perform split
transactions.
Bits 30:17

Reserved, must be kept at reset value.

Bit 16 COMPLSPLT: Do complete split
The application sets this bit to request the OTG host to perform a complete split transaction.

RM0456 Rev 6

<!-- pagebreak -->

3460

USB on-the-go high-speed (OTG_HS)

RM0456

Bits 15:14 XACTPOS[1:0]: Transaction position
This field is used to determine whether to send all, first, middle, or last payloads with each
OUT transaction.
11: All. This is the entire data payload of this transaction (which is less than or equal to 188
bytes)
10: Begin. This is the first data payload of this transaction (which is larger than 188 bytes)
00: Mid. This is the middle payload of this transaction (which is larger than 188 bytes)
01: End. This is the last payload of this transaction (which is larger than 188 bytes)
Bits 13:7 HUBADDR[6:0]: Hub address
This field holds the device address of the transaction translator hub.
Bits 6:0 PRTADDR[6:0]: Port address
This field is the port number of the recipient transaction translator.

73.14.33 OTG host channel x interrupt register (OTG_HCINTx)
This register indicates the status of a channel with respect to USB- and AHB-related events.
It is shown in Figure 926. The application must read this register when the host channels
interrupt bit in the core interrupt register (HCINT bit in OTG_GINTSTS) is set. Before the
application can read this register, it must first read the host all channels interrupt
(OTG_HAINT) register to get the exact channel number for the host channel-x interrupt
register. The application must clear the appropriate bit in this register to clear the
corresponding bits in the OTG_HAINT and OTG_GINTSTS registers.
Address offset: 0x508 + 0x20 * x, (x = 0 to 15)
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

DTERR

FRM
OR

BBERR TXERR

NYET

ACK

NAK

STALL

AHB
ERR

CHH

XFRC

rc_w1

rc_w1

rc_w1

rc_w1

rc_w1

rc_w1

rc_w1

rc_w1

rc_w1

rc_w1

Res.

Res.

Res.

Res.

Res.

rc_w1

Bits 31:11 Reserved, must be kept at reset value.
Bit 10 DTERR: Data toggle error.
Bit 9 FRMOR: Frame overrun.
Bit 8 BBERR: Babble error.
Bit 7 TXERR: Transaction error.
Indicates one of the following errors occurred on the USB.
CRC check failure
Timeout
Bit stuff error
False EOP
Bit 6 NYET: Not yet ready response received interrupt.
Bit 5 ACK: ACK response received/transmitted interrupt.
Bit 4 NAK: NAK response received interrupt.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

USB on-the-go high-speed (OTG_HS)

Bit 3 STALL: STALL response received interrupt.
Bit 2 AHBERR: AHB error
This error is generated only in Internal DMA mode when an AHB error occurs during an AHB
read/write operation. The application can read the corresponding DMA channel address
register to get the error address.
Bit 1 CHH: Channel halted
This indicates that the transfer completed:
– because of any USB transaction error
– in response to disable request by the application
– normally
Bit 0 XFRC: Transfer completed.
Transfer completed normally without any errors.

73.14.34 OTG host channel x interrupt mask register (OTG_HCINTMSKx)
This register reflects the mask for each channel status described in the previous section.
Address offset: 0x50C + 0x20 * x, (x = 0 to 15)
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

DTERR
M

FRM
ORM

NYET

ACKM

NAKM

STALL
M

AHB
ERRM

CHHM

XFRC
M

rw

rw

rw

rw

rw

rw

rw

rw

rw

BBERR TXERR
M
M
rw

rw

Bits 31:11 Reserved, must be kept at reset value.
Bit 10 DTERRM: Data toggle error mask.
0: Masked interrupt
1: Unmasked interrupt
Bit 9 FRMORM: Frame overrun mask.
0: Masked interrupt
1: Unmasked interrupt
Bit 8 BBERRM: Babble error mask.
0: Masked interrupt
1: Unmasked interrupt
Bit 7 TXERRM: Transaction error mask.
0: Masked interrupt
1: Unmasked interrupt
Bit 6 NYET: response received interrupt mask.
0: Masked interrupt
1: Unmasked interrupt

RM0456 Rev 6

<!-- pagebreak -->

3460

USB on-the-go high-speed (OTG_HS)

RM0456

Bit 5 ACKM: ACK response received/transmitted interrupt mask.
0: Masked interrupt
1: Unmasked interrupt
Bit 4 NAKM: NAK response received interrupt mask.
0: Masked interrupt
1: Unmasked interrupt
Bit 3 STALLM: STALL response received interrupt mask.
0: Masked interrupt
1: Unmasked interrupt
Bit 2 AHBERRM: AHB error.
0: Masked interrupt
1: Unmasked interrupt
Bit 1 CHHM: Channel halted mask
0: Masked interrupt
1: Unmasked interrupt
Bit 0 XFRCM: Transfer completed mask
0: Masked interrupt
1: Unmasked interrupt

73.14.35 OTG host channel x transfer size register (OTG_HCTSIZx)
Address offset: 0x510 + 0x20 * x, (x = 0 to 15)
Reset value: 0x0000 0000
31

30

DO
PNG

29

DPID[1:0]

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

PKTCNT[9:0]

17

16

XFRSIZ[18:16]

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

XFRSIZ[15:0]
rw

rw

rw

rw

rw

rw

rw

rw

rw

Bit 31 DOPNG: Do Ping
This bit is used only for OUT transfers. Setting this field to 1 directs the host to do PING
protocol.
Note: Do not set this bit for IN transfers. If this bit is set for IN transfers, it disables the
channel.
Bits 30:29 DPID[1:0]: Data PID
The application programs this field with the type of PID to use for the initial transaction. The
host maintains this field for the rest of the transfer.
00: DATA0
01: DATA2
10: DATA1
11: SETUP (control) / MDATA (non-control)

<!-- pagebreak -->

RM0456 Rev 6

RM0456

USB on-the-go high-speed (OTG_HS)

Bits 28:19 PKTCNT[9:0]: Packet count
This field is programmed by the application with the expected number of packets to be
transmitted (OUT) or received (IN).
The host decrements this count on every successful transmission or reception of an OUT/IN
packet. Once this count reaches zero, the application is interrupted to indicate normal
completion.
Bits 18:0 XFRSIZ[18:0]: Transfer size
For an OUT, this field is the number of data bytes the host sends during the transfer.
For an IN, this field is the buffer size that the application has reserved for the transfer. The
application is expected to program this field as an integer multiple of the maximum packet
size for IN transactions (periodic and non-periodic).

73.14.36 OTG host channel x DMA address register(OTG_HCDMAx)
Address offset: 0x514 + 0x20 * x, (x = 0 to 15)
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

DMAADDR[31:16]
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

rw

rw

rw

rw

rw

rw

rw

rw

rw

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

DMAADDR[15:0]
rw

rw

rw

rw

rw

rw

rw

rw

rw

Bits 31:0 DMAADDR[31:0]: DMA address
This field holds the start address in the external memory from which the data for the endpoint
must be fetched or to which it must be stored. This register is incremented on every AHB
transaction.

73.14.37 Device-mode registers
These registers must be programmed every time the core changes to device mode

73.14.38 OTG device configuration register (OTG_DCFG)
This register configures the core in device mode after power-on or after certain control
commands or enumeration. Do not make changes to this register after initial programming.
Address offset: 0x800
Reset value: 0x0220 0000
31

30

29

28

27

26

Res.

Res.

Res.

Res.

Res.

Res.

15
ERRAT
IM
rw

14
Res.

13
Res.

12

11

10

25

24

PERSCHIVL[1:0]
rw

rw

9

8

PFIVL[1:0]
rw

rw

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

NZLSO
HSK

DAD[6:0]
rw

rw

rw

rw

rw

rw

rw

rw

DSPD[1:0]
rw

rw

Bits 31:26 Reserved, must be kept at reset value.

RM0456 Rev 6

<!-- pagebreak -->

3460

USB on-the-go high-speed (OTG_HS)

RM0456

Bits 25:24 PERSCHIVL[1:0]: Periodic schedule interval
This field specifies the amount of time the Internal DMA engine must allocate for fetching
periodic IN endpoint data. Based on the number of periodic endpoints, this value must be
specified as 25, 50 or 75% of the (micro) frame.

–

When any periodic endpoints are active, the internal DMA engine allocates
the specified amount of time in fetching periodic IN endpoint data

–

When no periodic endpoint is active, then the internal DMA engine services
nonperiodic endpoints, ignoring this field

–

After the specified time within a (micro) frame, the DMA switches to
fetching nonperiodic endpoints

00: 25% of (micro)frame
01: 50% of (micro)frame
10: 75% of (micro)frame
11: Reserved
Bits 23:16 Reserved, must be kept at reset value.
Bit 15 ERRATIM: Erratic error interrupt mask
1: Mask early suspend interrupt on erratic error
0: Early suspend interrupt is generated on erratic error
Bit 14 Reserved, must be kept at reset value.
Bit 13 Reserved, must be kept at reset value.
Bits 12:11 PFIVL[1:0]: Periodic frame interval
Indicates the time within a frame at which the application must be notified using the end of
periodic frame interrupt. This can be used to determine if all the isochronous traffic for that
frame is complete.
00: 80% of the frame interval
01: 85% of the frame interval
10: 90% of the frame interval
11: 95% of the frame interval
Bits 10:4 DAD[6:0]: Device address
The application must program this field after every SetAddress control command.
Bit 3 Reserved, must be kept at reset value.
Bit 2 NZLSOHSK: Non-zero-length status OUT handshake
The application can use this field to select the handshake the core sends on receiving a
nonzero-length data packet during the OUT transaction of a control transfer’s status stage.
1: Send a STALL handshake on a nonzero-length status OUT transaction and do not send
the received OUT packet to the application.
0: Send the received OUT packet to the application (zero-length or nonzero-length) and
send a handshake based on the NAK and STALL bits for the endpoint in the device
endpoint control register.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

USB on-the-go high-speed (OTG_HS)

Bits 1:0 DSPD[1:0]: Device speed
Indicates the speed at which the application requires the core to enumerate, or the
maximum speed the application can support. However, the actual bus speed is determined
only after the chirp sequence is completed, and is based on the speed of the USB host to
which the core is connected.
00: High speed
01: Full speed
10: Reserved
11: Reserved

73.14.39 OTG device control register (OTG_DCTL)
Address offset: 0x804
Reset value: 0x0000 0002
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

21

Res.

Res.

20
Res.

19

18

17

16

Res.

DS
BESL
RJCT

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

11

10

9

8

7

Res.

PO
PRG
DNE

CGO
NAK

SGO
NAK

CGI
NAK

SGI
NAK

rw

w

w

w

w

6

5

4

TCTL[2:0]
rw

rw

rw

3

2

1

0

GON
STS

GIN
STS

SDIS

RWU
SIG

r

r

rw

rw

Bits 31:19 Reserved, must be kept at reset value.
Bit 18 DSBESLRJCT: Deep sleep BESL reject
Core rejects LPM request with BESL value greater than BESL threshold programmed. NYET
response is sent for LPM tokens with BESL value greater than BESL threshold. By default,
the deep sleep BESL reject feature is disabled.
Bits 17:12 Reserved, must be kept at reset value.
Bit 11 POPRGDNE: Power-on programming done
The application uses this bit to indicate that register programming is completed after a wakeup from power down mode.
Bit 10 CGONAK: Clear global OUT NAK
Writing 1 to this field clears the Global OUT NAK.
Bit 9 SGONAK: Set global OUT NAK
Writing 1 to this field sets the Global OUT NAK.
The application uses this bit to send a NAK handshake on all OUT endpoints.
The application must set the this bit only after making sure that the Global OUT NAK effective
bit in the core interrupt register (GONAKEFF bit in OTG_GINTSTS) is cleared.
Bit 8 CGINAK: Clear global IN NAK
Writing 1 to this field clears the Global IN NAK.
Bit 7 SGINAK: Set global IN NAK
Writing 1 to this field sets the Global non-periodic IN NAK.The application uses this bit to
send a NAK handshake on all non-periodic IN endpoints.
The application must set this bit only after making sure that the Global IN NAK effective bit in
the core interrupt register (GINAKEFF bit in OTG_GINTSTS) is cleared.

RM0456 Rev 6

<!-- pagebreak -->

3460

USB on-the-go high-speed (OTG_HS)

RM0456

Bits 6:4 TCTL[2:0]: Test control
000: Test mode disabled
001: Test_J mode
010: Test_K mode
011: Test_SE0_NAK mode
100: Test_Packet mode
101: Test_Force_Enable
Others: Reserved
Bit 3 GONSTS: Global OUT NAK status
0: A handshake is sent based on the FIFO status and the NAK and STALL bit settings.
1: No data is written to the Rx FIFO, irrespective of space availability. Sends a NAK
handshake on all packets, except on SETUP transactions. All isochronous OUT packets
are dropped.
Bit 2 GINSTS: Global IN NAK status
0: A handshake is sent out based on the data availability in the transmit FIFO.
1: A NAK handshake is sent out on all non-periodic IN endpoints, irrespective of the data
availability in the transmit FIFO.
Bit 1 SDIS: Soft disconnect
The application uses this bit to signal the USB OTG core to perform a soft disconnect. As
long as this bit is set, the host does not see that the device is connected, and the device does
not receive signals on the USB. The core stays in the disconnected state until the application
clears this bit.
0: Normal operation. When this bit is cleared after a soft disconnect, the core generates a
device connect event to the USB host. When the device is reconnected, the USB host
restarts device enumeration.
1: The core generates a device disconnect event to the USB host.
Bit 0 RWUSIG: Remote wake-up signaling
When the application sets this bit, the core initiates remote signaling to wake up the USB
host. The application must set this bit to instruct the core to exit the suspend state. As
specified in the USB 2.0 specification, the application must clear this bit 1 ms to 15 ms after
setting it.
If LPM is enabled and the core is in the L1 (sleep) state, when the application sets this bit, the
core initiates L1 remote signaling to wake up the USB host. The application must set this bit
to instruct the core to exit the sleep state. As specified in the LPM specification, the hardware
automatically clears this bit 50 µs (TL1DevDrvResume) after being set by the application. The
application must not set this bit when bRemoteWake from the previous LPM transaction is
zero (refer to REMWAKE bit in GLPMCFG register).

Table 772 contains the minimum duration (according to device state) for which the Soft
disconnect (SDIS) bit must be set for the USB host to detect a device disconnect. To
accommodate clock jitter, it is recommended that the application add some extra delay to
the specified minimum duration.
Table 772. Minimum duration for soft disconnect
Operating speed

<!-- pagebreak -->

Device state

Minimum duration

Full speed

Suspended

1 ms + 2.5 µs

Full speed

Idle

2.5 µs

Full speed

Not Idle or suspended (Performing transactions)

2.5 µs

High speed

Not Idle or suspended (Performing transactions)

125 µs

RM0456 Rev 6

RM0456

USB on-the-go high-speed (OTG_HS)

73.14.40 OTG device status register (OTG_DSTS)
This register indicates the status of the core with respect to USB-related events. It must be
read on interrupts from the device all interrupts (OTG_DAINT) register.
Address offset: 0x808
Reset value: 0x0000 0010
31

30

29

28

27

26

25

24

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

FNSOF[7:0]
r

r

r

r

r

r

r

23

22

21

20

DEVLNSTS[1:0]

19

18

17

16

FNSOF[13:8]

r

r

r

r

r

r

r

r

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

EERR

r

r

ENUMSPD[1:0]
r

SUSP
STS

r

r

Bits 31:24 Reserved, must be kept at reset value.
Bits 23:22 DEVLNSTS[1:0]: Device line status
Indicates the current logic level USB data lines.
Bit [23]: Logic level of D+
Bit [22]: Logic level of DBits 21:8 FNSOF[13:0]: Frame number of the received SOF
Bits 7:4 Reserved, must be kept at reset value.
Bit 3 EERR: Erratic error
The core sets this bit to report any erratic errors.
Due to erratic errors, the OTG_HS controller goes into suspended state and an interrupt is
generated to the application with Early suspend bit of the OTG_GINTSTS register (ESUSP
bit in OTG_GINTSTS). If the early suspend is asserted due to an erratic error, the application
can only perform a soft disconnect recover.
Bits 2:1 ENUMSPD[1:0]: Enumerated speed
Indicates the speed at which the OTG_HS controller has come up after speed detection
through a chirp sequence.
00: High Speed
01: Full Speed
11: Reserved
Others: reserved
Bit 0 SUSPSTS: Suspend status
In device mode, this bit is set as long as a suspend condition is detected on the USB. The
core enters the suspended state when there is no activity on the USB data lines for a period
of 3 ms. The core comes out of the suspend:
– When there is an activity on the USB data lines
– When the application writes to the remote wake-up signaling bit in the OTG_DCTL register
(RWUSIG bit in OTG_DCTL).

RM0456 Rev 6

<!-- pagebreak -->

3460

USB on-the-go high-speed (OTG_HS)

RM0456

73.14.41 OTG device IN endpoint common interrupt mask register
(OTG_DIEPMSK)
This register works with each of the OTG_DIEPINTx registers for all endpoints to generate
an interrupt per IN endpoint. The IN endpoint interrupt for a specific status in the
OTG_DIEPINTx register can be masked by writing to the corresponding bit in this register.
Status bits are masked by default.
Address offset: 0x810
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

NAKM

Res.

Res.

Res.

Res.

TXFU
RM

Res.

INEPN
EM

TOM

AHB
ERRM

EPDM

XFRC
M

rw

rw

rw

rw

rw

rw

rw

INEPN ITTXFE
MM
MSK
rw

Bits 31:14 Reserved, must be kept at reset value.
Bit 13 NAKM: NAK interrupt mask
0: Masked interrupt
1: Unmasked interrupt
Bits 12:10 Reserved, must be kept at reset value.
Bit 9 Reserved, must be kept at reset value.
Bit 8 TXFURM: FIFO underrun mask
0: Masked interrupt
1: Unmasked interrupt
Bit 7 Reserved, must be kept at reset value.
Bit 6 INEPNEM: IN endpoint NAK effective mask
0: Masked interrupt
1: Unmasked interrupt
Bit 5 INEPNMM: IN token received with EP mismatch mask
0: Masked interrupt
1: Unmasked interrupt
Bit 4 ITTXFEMSK: IN token received when Tx FIFO empty mask
0: Masked interrupt
1: Unmasked interrupt
Bit 3 TOM: Timeout condition mask (Non-isochronous endpoints)
0: Masked interrupt
1: Unmasked interrupt
Bit 2 AHBERRM: AHB error mask
0: Masked interrupt
1: Unmasked interrupt

<!-- pagebreak -->

RM0456 Rev 6

rw

RM0456

USB on-the-go high-speed (OTG_HS)

Bit 1 EPDM: Endpoint disabled interrupt mask
0: Masked interrupt
1: Unmasked interrupt
Bit 0 XFRCM: Transfer completed interrupt mask
0: Masked interrupt
1: Unmasked interrupt

73.14.42 OTG device OUT endpoint common interrupt mask register
(OTG_DOEPMSK)
This register works with each of the OTG_DOEPINTx registers for all endpoints to generate
an interrupt per OUT endpoint. The OUT endpoint interrupt for a specific status in the
OTG_DOEPINTx register can be masked by writing into the corresponding bit in this
register. Status bits are masked by default.
Address offset: 0x814
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

OUT
PKT
ERRM

EPDM

XFRC
M

rw

rw

Res.

NYET
MSK

NAK
MSK

BERR
M

rw

rw

rw

Res.

Res.

Res.

rw

STS
B2B
PHSR
STUPM
XM
rw

OTEPD
AHB
STUPM
M
ERRM

rw

rw

rw

rw

Bits 31:15 Reserved, must be kept at reset value.
Bit 14 NYETMSK: NYET interrupt mask
0: Masked interrupt
1: Unmasked interrupt
Bit 13 NAKMSK: NAK interrupt mask
0: Masked interrupt
1: Unmasked interrupt
Bit 12 BERRM: Babble error interrupt mask
0: Masked interrupt
1: Unmasked interrupt
Bits 11:10 Reserved, must be kept at reset value.
Bit 9 Reserved, must be kept at reset value.
Bit 8 OUTPKTERRM: Out packet error mask
0: Masked interrupt
1: Unmasked interrupt
Bit 7 Reserved, must be kept at reset value.
Bit 6 B2BSTUPM: Back-to-back SETUP packets received mask
Applies to control OUT endpoints only.
0: Masked interrupt
1: Unmasked interrupt

RM0456 Rev 6

<!-- pagebreak -->

3460

USB on-the-go high-speed (OTG_HS)

RM0456

Bit 5 STSPHSRXM: Status phase received for control write mask
0: Masked interrupt
1: Unmasked interrupt
Bit 4 OTEPDM: OUT token received when endpoint disabled mask. Applies to control OUT
endpoints only.
0: Masked interrupt
1: Unmasked interrupt
Bit 3 STUPM: SETUP phase done mask. Applies to control endpoints only.
0: Masked interrupt
1: Unmasked interrupt
Bit 2 AHBERRM: AHB error mask
0: Masked interrupt
1: Unmasked interrupt
Bit 1 EPDM: Endpoint disabled interrupt mask
0: Masked interrupt
1: Unmasked interrupt
Bit 0 XFRCM: Transfer completed interrupt mask
0: Masked interrupt
1: Unmasked interrupt

73.14.43 OTG device all endpoints interrupt register (OTG_DAINT)
When a significant event occurs on an endpoint, a OTG_DAINT register interrupts the
application using the device OUT endpoints interrupt bit or device IN endpoints interrupt bit
of the OTG_GINTSTS register (OEPINT or IEPINT in OTG_GINTSTS, respectively). There
is one interrupt bit per endpoint, up to a maximum of 16 bits for OUT endpoints and 16 bits
for IN endpoints. For a bidirectional endpoint, the corresponding IN and OUT interrupt bits
are used. Bits in this register are set and cleared when the application sets and clears bits in
the corresponding device endpoint-x interrupt register (OTG_DIEPINTx/OTG_DOEPINTx).
Address offset: 0x818
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

OEPINT[15:0]

IEPINT[15:0]
r

r

r

r

r

r

r

r

r

Bits 31:16 OEPINT[15:0]: OUT endpoint interrupt bits
One bit per OUT endpoint:
Bit 16 for OUT endpoint 0, bit 19 for OUT endpoint 3.
Bits 15:0 IEPINT[15:0]: IN endpoint interrupt bits
One bit per IN endpoint:
Bit 0 for IN endpoint 0, bit 3 for endpoint 3.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

USB on-the-go high-speed (OTG_HS)

73.14.44 OTG all endpoints interrupt mask register
(OTG_DAINTMSK)
The OTG_DAINTMSK register works with the device endpoint interrupt register to interrupt
the application when an event occurs on a device endpoint. However, the OTG_DAINT
register bit corresponding to that interrupt is still set.
Address offset: 0x81C
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

OEPM[15:0]
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

18

17

16

IEPM[15:0]
rw

Bits 31:16 OEPM[15:0]: OUT EP interrupt mask bits
One per OUT endpoint:
Bit 16 for OUT EP 0, bit 19 for OUT EP 3
0: Masked interrupt
1: Unmasked interrupt
Bits 15:0 IEPM[15:0]: IN EP interrupt mask bits
One bit per IN endpoint:
Bit 0 for IN EP 0, bit 3 for IN EP 3
0: Masked interrupt
1: Unmasked interrupt

73.14.45 OTG device threshold control register (OTG_DTHRCTL)
Address offset: 0x0830
Reset value: 0x0000 0000
31

30

29

28

27

26

Res.

Res.

Res.

Res.

ARPEN

Res.

15

14

13

12

rw

Res.

Res.

Res.

Res.

11

10

25

24

23

22

21

20

19

RXTH
REN

RXTHRLEN[8:0]
rw

rw

rw

rw

rw

rw

rw

rw

rw

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

ISOT
HREN

NONIS
OTH
REN

rw

rw

Res.

TXTHRLEN[8:0]
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

Bits 31:28 Reserved, must be kept at reset value.
Bit 27 ARPEN: Arbiter parking enable
This bit controls internal DMA arbiter parking for IN endpoints. When thresholding is enabled
and this bit is set to one, then the arbiter parks on the IN endpoint for which there is a token
received on the USB. This is done to avoid getting into underrun conditions. By default
parking is enabled.
Bit 26

Reserved, must be kept at reset value.

RM0456 Rev 6

<!-- pagebreak -->

3460

USB on-the-go high-speed (OTG_HS)

RM0456

Bits 25:17 RXTHRLEN[8:0]: Receive threshold length
This field specifies the receive thresholding size in 32-bit words. This field also specifies the
amount of data received on the USB before the core can start transmitting on the AHB. The
threshold length has to be at least eight 32-bit words. The recommended value for
RXTHRLEN is to be the same as the programmed AHB burst length (HBSTLEN bit in
OTG_GAHBCFG).
Bit 16 RXTHREN: Receive threshold enable
When this bit is set, the core enables thresholding in the receive direction.
Bits 15:11

Reserved, must be kept at reset value.

Bits 10:2 TXTHRLEN[8:0]: Transmit threshold length
This field specifies the transmit thresholding size in 32-bit words. This field specifies the
amount of data in bytes to be in the corresponding endpoint transmit FIFO, before the core
can start transmitting on the USB. The threshold length has to be at least eight 32-bit words.
This field controls both isochronous and nonisochronous IN endpoint thresholds. The
recommended value for TXTHRLEN is to be the same as the programmed AHB burst length
(HBSTLEN bit in OTG_GAHBCFG).
Bit 1 ISOTHREN: ISO IN endpoint threshold enable
When this bit is set, the core enables thresholding for isochronous IN endpoints.
Bit 0 NONISOTHREN: Nonisochronous IN endpoints threshold enable
When this bit is set, the core enables thresholding for nonisochronous IN endpoints.

73.14.46 OTG device IN endpoint FIFO empty interrupt mask register
(OTG_DIEPEMPMSK)
This register is used to control the IN endpoint FIFO empty interrupt generation
(TXFE_OTG_DIEPINTx).
Address offset: 0x834
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

INEPTXFEM[15:0]
rw

rw

Bits 31:16 Reserved, must be kept at reset value.
Bits 15:0 INEPTXFEM[15:0]: IN EP Tx FIFO empty interrupt mask bits
These bits act as mask bits for OTG_DIEPINTx.
TXFE interrupt one bit per IN endpoint:
Bit 0 for IN endpoint 0, bit 3 for IN endpoint 3
0: Masked interrupt
1: Unmasked interrupt

<!-- pagebreak -->

RM0456 Rev 6

RM0456

USB on-the-go high-speed (OTG_HS)

73.14.47 OTG device IN endpoint x control register [alternate]
(OTG_DIEPCTLx)
Valid for INT/BULK endpoints, see next section for ISO endpoints.
The application uses this register to control the behavior of each logical endpoint other than
endpoint 0.
Address offset: 0x900 + 0x20 * x, (x = 0 to 8)
Reset value: 0x0000 0000
31

30

EPENA EPDIS

29

28

SD1
PID

SD0
PID

27
SNAK

26

25

CNAK

24

23

22

TXFNUM[3:0]

21
STALL

rs

rs

w

w

w

w

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

USBA
EP

Res.

Res.

Res.

Res.

rw

20
Res.

19

18

17

16

EPTYP[1:0]

NAK
STS

DPID

rw

rw

r

r

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

MPSIZ[10:0]
rw

rw

rw

rw

rw

rw

Bit 31 EPENA: Endpoint enable
The application sets this bit to start transmitting data on an endpoint.
The core clears this bit before setting any of the following interrupts on this endpoint:
– SETUP phase done
– Endpoint disabled
– Transfer completed
Bit 30 EPDIS: Endpoint disable
The application sets this bit to stop transmitting/receiving data on an endpoint, even before
the transfer for that endpoint is complete. The application must wait for the endpoint
disabled interrupt before treating the endpoint as disabled. The core clears this bit before
setting the endpoint disabled interrupt. The application must set this bit only if endpoint
enable is already set for this endpoint.
Bit 29 SD1PID: Set DATA1 PID
Writing to this field sets the endpoint data PID (DPID) field in this register to DATA1.
Bit 28 SD0PID: Set DATA0 PID
Applies to interrupt/bulk IN endpoints only.
Writing to this field sets the endpoint data PID (DPID) field in this register to DATA0.
Bit 27 SNAK: Set NAK
A write to this bit sets the NAK bit for the endpoint.
Using this bit, the application can control the transmission of NAK handshakes on an
endpoint. The core can also set this bit for OUT endpoints on a transfer completed interrupt,
or after a SETUP is received on the endpoint.
Bit 26 CNAK: Clear NAK
A write to this bit clears the NAK bit for the endpoint.
Bits 25:22 TXFNUM[3:0]: Tx FIFO number
These bits specify the FIFO number associated with this endpoint. Each active IN endpoint
must be programmed to a separate FIFO number.
This field is valid only for IN endpoints.

RM0456 Rev 6

<!-- pagebreak -->

3460

USB on-the-go high-speed (OTG_HS)

RM0456

Bit 21 STALL: STALL handshake
Applies to non-control, non-isochronous IN endpoints only (access type is rw).
The application sets this bit to stall all tokens from the USB host to this endpoint. If a NAK
bit, Global IN NAK, or Global OUT NAK is set along with this bit, the STALL bit takes priority.
Only the application can clear this bit, never the core.
Bit 20 Reserved, must be kept at reset value.
Bits 19:18 EPTYP[1:0]: Endpoint type
This is the transfer type supported by this logical endpoint.
00: Control
01: Isochronous
10: Bulk
11: Interrupt
Bit 17 NAKSTS: NAK status
It indicates the following:
0: The core is transmitting non-NAK handshakes based on the FIFO status.
1: The core is transmitting NAK handshakes on this endpoint.
When either the application or the core sets this bit:
For non-isochronous IN endpoints: The core stops transmitting any data on an IN endpoint,
even if there are data available in the Tx FIFO.
For isochronous IN endpoints: The core sends out a zero-length data packet, even if there
are data available in the Tx FIFO.
Irrespective of this bit’s setting, the core always responds to SETUP data packets with an
ACK handshake.
Bit 16 DPID: Endpoint data PID
Applies to interrupt/bulk IN endpoints only.
Contains the PID of the packet to be received or transmitted on this endpoint. The
application must program the PID of the first packet to be received or transmitted on this
endpoint, after the endpoint is activated. The application uses the SD0PID and SD1PID
register fields to program either DATA0 or DATA1 PID.
0: DATA0
1: DATA1
Bit 15 USBAEP: USB active endpoint
Indicates whether this endpoint is active in the current configuration and interface. The core
clears this bit for all endpoints (other than EP 0) after detecting a USB reset. After receiving
the SetConfiguration and SetInterface commands, the application must program endpoint
registers accordingly and set this bit.
Bits 14:11

Reserved, must be kept at reset value.

Bits 10:0 MPSIZ[10:0]: Maximum packet size
The application must program this field with the maximum packet size for the current logical
endpoint. This value is in bytes.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

USB on-the-go high-speed (OTG_HS)

73.14.48 OTG device IN endpoint x control register [alternate]
(OTG_DIEPCTLx)
Valid for ISO endpoints, see previous section for INT/BULK endpoints.
The application uses this register to control the behavior of each logical endpoint other than
endpoint 0.
Address offset: 0x900 + 0x20 * x, (x = 0 to 8)
Reset value: 0x0000 0000
31

30

EPENA EPDIS

29

28

SODD
FRM

SEVN
FRM

27
SNAK

26

25

CNAK

24

23

22

TXFNUM[3:0]

21
STALL

rs

rs

w

w

w

w

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

USBA
EP

Res.

Res.

Res.

Res.

rw

20
Res.

19

18

17

16

EPTYP[1:0]

NAK
STS

EO
NUM

rw

rw

r

r

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

MPSIZ[10:0]
rw

rw

rw

rw

rw

rw

Bit 31 EPENA: Endpoint enable
The application sets this bit to start transmitting data on an endpoint.
The core clears this bit before setting any of the following interrupts on this endpoint:
– SETUP phase done
– Endpoint disabled
– Transfer completed
Bit 30 EPDIS: Endpoint disable
The application sets this bit to stop transmitting/receiving data on an endpoint, even before
the transfer for that endpoint is complete. The application must wait for the endpoint
disabled interrupt before treating the endpoint as disabled. The core clears this bit before
setting the endpoint disabled interrupt. The application must set this bit only if endpoint
enable is already set for this endpoint.
Bit 29 SODDFRM: Set odd frame
Applies to isochronous IN and OUT endpoints only.
Writing to this field sets the Even/Odd frame (EONUM) field to odd frame.
Bit 28 SEVNFRM: Set even frame
Applies to isochronous IN endpoints only.
Writing to this field sets the Even/Odd frame (EONUM) field to even frame.
Bit 27 SNAK: Set NAK
A write to this bit sets the NAK bit for the endpoint.
Using this bit, the application can control the transmission of NAK handshakes on an
endpoint. The core can also set this bit for OUT endpoints on a transfer completed interrupt,
or after a SETUP is received on the endpoint.
Bit 26 CNAK: Clear NAK
A write to this bit clears the NAK bit for the endpoint.
Bits 25:22 TXFNUM[3:0]: Tx FIFO number
These bits specify the FIFO number associated with this endpoint. Each active IN endpoint
must be programmed to a separate FIFO number.
This field is valid only for IN endpoints.

RM0456 Rev 6

<!-- pagebreak -->

3460

USB on-the-go high-speed (OTG_HS)

RM0456

Bit 21 STALL: STALL handshake
Applies to non-control, non-isochronous IN endpoints only (access type is rw).
The application sets this bit to stall all tokens from the USB host to this endpoint. If a NAK
bit, Global IN NAK, or Global OUT NAK is set along with this bit, the STALL bit takes priority.
Only the application can clear this bit, never the core.
Bit 20 Reserved, must be kept at reset value.
Bits 19:18 EPTYP[1:0]: Endpoint type
This is the transfer type supported by this logical endpoint.
00: Control
01: Isochronous
10: Bulk
11: Interrupt
Bit 17 NAKSTS: NAK status
It indicates the following:
0: The core is transmitting non-NAK handshakes based on the FIFO status.
1: The core is transmitting NAK handshakes on this endpoint.
When either the application or the core sets this bit:
For non-isochronous IN endpoints: The core stops transmitting any data on an IN endpoint,
even if there are data available in the Tx FIFO.
For isochronous IN endpoints: The core sends out a zero-length data packet, even if there
are data available in the Tx FIFO.
Irrespective of this bit’s setting, the core always responds to SETUP data packets with an
ACK handshake.
Bit 16 EONUM: Even/odd frame
Applies to isochronous IN endpoints only.
Indicates the frame number in which the core transmits/receives isochronous data for this
endpoint. The application must program the even/odd frame number in which it intends to
transmit/receive isochronous data for this endpoint using the SEVNFRM and SODDFRM
fields in this register.
0: Even frame
1: Odd frame
Bit 15 USBAEP: USB active endpoint
Indicates whether this endpoint is active in the current configuration and interface. The core
clears this bit for all endpoints (other than EP 0) after detecting a USB reset. After receiving
the SetConfiguration and SetInterface commands, the application must program endpoint
registers accordingly and set this bit.
Bits 14:11

Reserved, must be kept at reset value.

Bits 10:0 MPSIZ[10:0]: Maximum packet size
The application must program this field with the maximum packet size for the current logical
endpoint. This value is in bytes.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

USB on-the-go high-speed (OTG_HS)

73.14.49 OTG device IN endpoint x interrupt register (OTG_DIEPINTx)
This register indicates the status of an endpoint with respect to USB- and AHB-related
events. It is shown in Figure 926. The application must read this register when the IN
endpoints interrupt bit of the core interrupt register (IEPINT in OTG_GINTSTS) is set.
Before the application can read this register, it must first read the device all endpoints
interrupt (OTG_DAINT) register to get the exact endpoint number for the device endpoint-x
interrupt register. The application must clear the appropriate bit in this register to clear the
corresponding bits in the OTG_DAINT and OTG_GINTSTS registers.
Address offset: 0x908 + 0x20 * x, (x = 0 to 8)
Reset value: 0x0000 0080
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

TXFIF
OUD
RN

TXFE

IN
EPNE

IN
ITTXFE
EPNM

TOC

AHB
ERR

EP
DISD

XFRC

rc_w1

r

rc_w1

rc_w1

rc_w1

rc_w1

rc_w1

rc_w1

Res.

Res.

NAK
rc_w1

Res.

PKTD
RPSTS
rc_w1

Res.

rc_w1

Bits 31:14 Reserved, must be kept at reset value.
Bit 13 NAK: NAK input
The core generates this interrupt when a NAK is transmitted or received by the device. In
case of isochronous IN endpoints the interrupt gets generated when a zero length packet is
transmitted due to unavailability of data in the Tx FIFO.
Bit 12 Reserved, must be kept at reset value.
Bit 11 PKTDRPSTS: Packet dropped status
This bit indicates to the application that an ISOC OUT packet has been dropped. This bit
does not have an associated mask bit and does not generate an interrupt.
Bit 10 Reserved, must be kept at reset value.
Bit 9 Reserved, must be kept at reset value.
Bit 8 TXFIFOUDRN: Transmit Fifo Underrun (TxfifoUndrn)
The core generates this interrupt when it detects a transmit FIFO underrun condition for this
endpoint. Dependency: This interrupt is valid only when Thresholding is enabled
Bit 7 TXFE: Transmit FIFO empty
This interrupt is asserted when the Tx FIFO for this endpoint is either half or completely
empty. The half or completely empty status is determined by the Tx FIFO Empty Level bit in
the OTG_GAHBCFG register (TXFELVL bit in OTG_GAHBCFG).
Bit 6 INEPNE: IN endpoint NAK effective
This bit can be cleared when the application clears the IN endpoint NAK by writing to the
CNAK bit in OTG_DIEPCTLx.
This interrupt indicates that the core has sampled the NAK bit set (either by the application
or by the core). The interrupt indicates that the IN endpoint NAK bit set by the application
has taken effect in the core.
This interrupt does not guarantee that a NAK handshake is sent on the USB. A STALL bit
takes priority over a NAK bit.

RM0456 Rev 6

<!-- pagebreak -->

3460

USB on-the-go high-speed (OTG_HS)

RM0456

Bit 5 INEPNM: IN token received with EP mismatch
Indicates that the data in the top of the non-periodic TxFIFO belongs to an endpoint other
than the one for which the IN token was received. This interrupt is asserted on the endpoint
for which the IN token was received.
Bit 4 ITTXFE: IN token received when Tx FIFO is empty
Indicates that an IN token was received when the associated Tx FIFO (periodic/nonperiodic) was empty. This interrupt is asserted on the endpoint for which the IN token was
received.
Bit 3 TOC: Timeout condition
Indicates that the core has detected a timeout condition on the USB for the last IN token on
this endpoint.
Bit 2 AHBERR: AHB error
This is generated only in internal DMA mode when there is an AHB error during an AHB
read/write. The application can read the corresponding endpoint DMA address register to
get the error address.
Bit 1 EPDISD: Endpoint disabled interrupt
This bit indicates that the endpoint is disabled per the application’s request.
Bit 0 XFRC: Transfer completed interrupt
This field indicates that the programmed transfer is complete on the AHB as well as on the
USB, for this endpoint.

73.14.50 OTG device IN endpoint 0 transfer size register
(OTG_DIEPTSIZ0)
The application must modify this register before enabling endpoint 0. Once endpoint 0 is
enabled using the endpoint enable bit in the device control endpoint 0 control registers
(EPENA in OTG_DIEPCTL0), the core modifies this register. The application can only read
this register once the core has cleared the endpoint enable bit.
Nonzero endpoints use the registers for endpoints 1–3.
Address offset: 0x910
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

PKTCNT[1:0]

Res.

Res.

Res.

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

XFRSIZ[6:0]
rw

Bits 31:21 Reserved, must be kept at reset value.
Bits 20:19 PKTCNT[1:0]: Packet count
Indicates the total number of USB packets that constitute the transfer size amount of data for
endpoint 0.
This field is decremented every time a packet (maximum size or short packet) is read from
the Tx FIFO.
Bits 18:7 Reserved, must be kept at reset value.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

USB on-the-go high-speed (OTG_HS)

Bits 6:0 XFRSIZ[6:0]: Transfer size
Indicates the transfer size in bytes for endpoint 0. The core interrupts the application only
after it has exhausted the transfer size amount of data. The transfer size can be set to the
maximum packet size of the endpoint, to be interrupted at the end of each packet.
The core decrements this field every time a packet from the external memory is written to
the Tx FIFO.

73.14.51 OTG device IN endpoint x DMA address register
(OTG_DIEPDMAx)
Address offset: 0x914 + 0x20 * x, (x = 0 to 8)
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

DMAADDR[31:16]

DMAADDR[15:0]
rw

rw

rw

rw

rw

rw

rw

rw

rw

Bits 31:0 DMAADDR[31:0]: DMA Address
This field holds the start address in the external memory from which the data for the
endpoint must be fetched. This register is incremented on every AHB transaction.

73.14.52 OTG device IN endpoint transmit FIFO status register
(OTG_DTXFSTSx)
This read-only register contains the free space information for the device IN endpoint Tx
FIFO.
Address offset: 0x918 + 0x20 * x, (x = 0 to 8)
Reset value: 0x0000 0200
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

r

r

r

r

r

r

r

INEPTFSAV[15:0]
r

r

r

r

r

r

r

r

r

Bits 31:16 Reserved, must be kept at reset value.
Bits 15:0 INEPTFSAV[15:0]: IN endpoint Tx FIFO space available
Indicates the amount of free space available in the endpoint Tx FIFO.
Values are in terms of 32-bit words:
0x0: Endpoint Tx FIFO is full
0x1: 1 word available
0x2: 2 words available
0xn: n words available
Others: Reserved

RM0456 Rev 6

<!-- pagebreak -->

3460

USB on-the-go high-speed (OTG_HS)

RM0456

73.14.53 OTG device IN endpoint x transfer size register (OTG_DIEPTSIZx)
The application must modify this register before enabling the endpoint. Once the endpoint is
enabled using the endpoint enable bit in the OTG_DIEPCTLx registers (EPENA bit in
OTG_DIEPCTLx), the core modifies this register. The application can only read this register
once the core has cleared the endpoint enable bit.
Address offset: 0x910 + 0x20 * x, (x = 1 to 8)
Reset value: 0x0000 0000
31

30

Res.

MCNT[1:0]

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

PKTCNT[9:0]

17

16

XFRSIZ[18:16]

XFRSIZ[15:0]
rw

rw

rw

rw

rw

rw

rw

rw

rw

Bit 31 Reserved, must be kept at reset value.
Bits 30:29 MCNT[1:0]: Multi count
For periodic IN endpoints, this field indicates the number of packets that must be transmitted
per frame on the USB. The core uses this field to calculate the data PID for isochronous IN
endpoints.
01: 1 packet
10: 2 packets
11: 3 packets
Bits 28:19 PKTCNT[9:0]: Packet count
Indicates the total number of USB packets that constitute the transfer size amount of data for
this endpoint.
This field is decremented every time a packet (maximum size or short packet) is read from
the Tx FIFO.
Bits 18:0 XFRSIZ[18:0]: Transfer size
This field contains the transfer size in bytes for the current endpoint. The core only interrupts
the application after it has exhausted the transfer size amount of data. The transfer size can
be set to the maximum packet size of the endpoint, to be interrupted at the end of each
packet.
The core decrements this field every time a packet from the external memory is written to the
Tx FIFO.

73.14.54 OTG device control OUT endpoint 0 control register
(OTG_DOEPCTL0)
This section describes the OTG_DOEPCTL0 register. Nonzero control endpoints use
registers for endpoints 1–8.
Address offset: 0xB00
Reset value: 0x0000 8000

<!-- pagebreak -->

RM0456 Rev 6

RM0456

31

USB on-the-go high-speed (OTG_HS)

30

EPENA EPDIS

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

SNAK

CNAK

Res.

Res.

Res.

Res.

STALL

SNPM

19

18

17

16

EPTYP[1:0]

NAK
STS

Res.

w

r

w

w

rs

rw

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

USBA
EP

Res.

Res.

Res.

Res.

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

0

MPSIZ[1:0]
r

r

Bit 31 EPENA: Endpoint enable
The application sets this bit to start transmitting data on endpoint 0.
The core clears this bit before setting any of the following interrupts on this endpoint:
– SETUP phase done
– Endpoint disabled
– Transfer completed
Bit 30 EPDIS: Endpoint disable
The application cannot disable control OUT endpoint 0.
Bits 29:28 Reserved, must be kept at reset value.
Bit 27 SNAK: Set NAK
A write to this bit sets the NAK bit for the endpoint.
Using this bit, the application can control the transmission of NAK handshakes on an
endpoint. The core can also set this bit on a transfer completed interrupt, or after a SETUP
is received on the endpoint.
Bit 26 CNAK: Clear NAK
A write to this bit clears the NAK bit for the endpoint.
Bits 25:22 Reserved, must be kept at reset value.
Bit 21 STALL: STALL handshake
The application can only set this bit, and the core clears it, when a SETUP token is received
for this endpoint. If a NAK bit or Global OUT NAK is set along with this bit, the STALL bit
takes priority. Irrespective of this bit’s setting, the core always responds to SETUP data
packets with an ACK handshake.
Bit 20 SNPM: Snoop mode
This bit configures the endpoint to Snoop mode. In Snoop mode, the core does not check
the correctness of OUT packets before transferring them to application memory.
Bits 19:18 EPTYP[1:0]: Endpoint type
Hardcoded to 00 for control.
Bit 17 NAKSTS: NAK status
Indicates the following:
0: The core is transmitting non-NAK handshakes based on the FIFO status.
1: The core is transmitting NAK handshakes on this endpoint.
When either the application or the core sets this bit, the core stops receiving data, even if
there is space in the Rx FIFO to accommodate the incoming packet. Irrespective of this bit’s
setting, the core always responds to SETUP data packets with an ACK handshake.
Bit 16 Reserved, must be kept at reset value.
Bit 15 USBAEP: USB active endpoint
This bit is always set to 1, indicating that a control endpoint 0 is always active in all
configurations and interfaces.

RM0456 Rev 6

<!-- pagebreak -->

3460

USB on-the-go high-speed (OTG_HS)

RM0456

Bits 14:2 Reserved, must be kept at reset value.
Bits 1:0 MPSIZ[1:0]: Maximum packet size
The maximum packet size for control OUT endpoint 0 is the same as what is programmed in
control IN endpoint 0.
00: 64 bytes
01: 32 bytes
10: 16 bytes
11: 8 bytes

73.14.55 OTG device OUT endpoint x interrupt register (OTG_DOEPINTx)
This register indicates the status of an endpoint with respect to USB- and AHB-related
events. It is shown in Figure 926. The application must read this register when the OUT
endpoints interrupt bit of the OTG_GINTSTS register (OEPINT bit in OTG_GINTSTS) is set.
Before the application can read this register, it must first read the OTG_DAINT register to
get the exact endpoint number for the OTG_DOEPINTx register. The application must clear
the appropriate bit in this register to clear the corresponding bits in the OTG_DAINT and
OTG_GINTSTS registers.
Address offset: 0xB08 + 0x20 * x, (x = 0 to 8)
Reset value: 0x0000 0080
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

OUT
PKT
ERR

Res.

B2B
STUP

STSPH
SRX

OTEP
DIS

STUP

AHB
ERR

EP
DISD

XFRC

rc_w1

rc_w1

rc_w1

rc_w1

rc_w1

rc_w1

rc_w1

STPK
TRX

NYET

NAK

BERR

rc_w1

rc_w1

rc_w1

rc_w1

Res.

Res.

rc_w1

Bits 31:16 Reserved, must be kept at reset value.
Bit 15 STPKTRX: Setup packet received
Applicable for control OUT endpoints in only in the Buffer DMA Mode. Set by the OTG_HS,
this bit indicates that this buffer holds 8 bytes of setup data. There is only one setup packet
per buffer. On receiving a setup packet, the OTG_HS closes the buffer and disables the
corresponding endpoint after SETUP_COMPLETE status is seen in the Rx FIFO. OTG_HS
puts a SETUP_COMPLETE status into the Rx FIFO when it sees the first IN or OUT token
after the SETUP packet for that particular endpoint. The application must then re-enable the
endpoint to receive any OUT data for the control transfer and reprogram the buffer start
address. Because of the above behavior, OTG_HS can receive any number of back to back
setup packets and one buffer for every setup packet is used.
Bit 14 NYET: NYET interrupt
This interrupt is generated when a NYET response is transmitted for a non isochronous
OUT endpoint.
Bit 13 NAK: NAK input
The core generates this interrupt when a NAK is transmitted or received by the device. In
case of isochronous IN endpoints the interrupt gets generated when a zero length packet is
transmitted due to unavailability of data in the Tx FIFO.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

USB on-the-go high-speed (OTG_HS)

Bit 12 BERR: Babble error interrupt
The core generates this interrupt when babble is received for the endpoint.
Bits 11:10 Reserved, must be kept at reset value.
Bit 9 Reserved, must be kept at reset value.
Bit 8 OUTPKTERR: OUT packet error
This interrupt is asserted when the core detects an overflow or a CRC error for an OUT
packet. This interrupt is valid only when thresholding is enabled.
Bit 7 Reserved, must be kept at reset value.
Bit 6 B2BSTUP: Back-to-back SETUP packets received
Applies to control OUT endpoint only.
This bit indicates that the core has received more than three back-to-back SETUP packets
for this particular endpoint.
Bit 5 STSPHSRX: Status phase received for control write
This interrupt is valid only for control OUT endpoints. This interrupt is generated only after
OTG_HS has transferred all the data that the host has sent during the data phase of a
control write transfer, to the system memory buffer. The interrupt indicates to the application
that the host has switched from data phase to the status phase of a control write transfer.
The application can use this interrupt to ACK or STALL the status phase, after it has
decoded the data phase.
Bit 4 OTEPDIS: OUT token received when endpoint disabled
Applies only to control OUT endpoints.
Indicates that an OUT token was received when the endpoint was not yet enabled. This
interrupt is asserted on the endpoint for which the OUT token was received.
Bit 3 STUP: SETUP phase done
Applies to control OUT endpoint only.Indicates that the SETUP phase for the control
endpoint is complete and no more back-to-back SETUP packets were received for the
current control transfer. On this interrupt, the application can decode the received SETUP
data packet.
Bit 2 AHBERR: AHB error
This is generated only in internal DMA mode when there is an AHB error during an AHB
read/write. The application can read the corresponding endpoint DMA address register to
get the error address.
Bit 1 EPDISD: Endpoint disabled interrupt
This bit indicates that the endpoint is disabled per the application’s request.
Bit 0 XFRC: Transfer completed interrupt
This field indicates that the programmed transfer is complete on the AHB as well as on the
USB, for this endpoint.

RM0456 Rev 6

<!-- pagebreak -->

3460

USB on-the-go high-speed (OTG_HS)

RM0456

73.14.56 OTG device OUT endpoint 0 transfer size register
(OTG_DOEPTSIZ0)
The application must modify this register before enabling endpoint 0. Once endpoint 0 is
enabled using the endpoint enable bit in the OTG_DOEPCTL0 registers (EPENA bit in
OTG_DOEPCTL0), the core modifies this register. The application can only read this
register once the core has cleared the endpoint enable bit.
Nonzero endpoints use the registers for endpoints 1–8.
Address offset: 0xB10
Reset value: 0x0000 0000
31
Res.

30

29

STUPCNT[1:0]

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

PKTCN
T

Res.

Res.

Res.

2

1

0

rw

rw

rw

Res.

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

rw

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
XFRSIZ[6:0]

rw

rw

rw

rw

Bit 31 Reserved, must be kept at reset value.
Bits 30:29 STUPCNT[1:0]: SETUP packet count
This field specifies the number of back-to-back SETUP data packets the endpoint can
receive.
01: 1 packet
10: 2 packets
11: 3 packets
Bits 28:20 Reserved, must be kept at reset value.
Bit 19 PKTCNT: Packet count
This field is decremented to zero after a packet is written into the Rx FIFO.
Bits 18:7 Reserved, must be kept at reset value.
Bits 6:0 XFRSIZ[6:0]: Transfer size
Indicates the transfer size in bytes for endpoint 0. The core interrupts the application only
after it has exhausted the transfer size amount of data. The transfer size can be set to the
maximum packet size of the endpoint, to be interrupted at the end of each packet.
The core decrements this field every time a packet is read from the Rx FIFO and written to
the external memory.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

USB on-the-go high-speed (OTG_HS)

73.14.57 OTG device OUT endpoint x DMA address register
(OTG_DOEPDMAx)
Address offset: 0xB14 + 0x20 * x, (x = 0 to 8)
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

DMAADDR[31:16]
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

DMAADDR[15:0]
rw

rw

rw

rw

rw

rw

rw

rw

rw

Bits 31:0 DMAADDR[31:0]: DMA Address
This field holds the start address in the external memory from which the data for the
endpoint must be fetched. This register is incremented on every AHB transaction.

73.14.58 OTG device OUT endpoint x control register [alternate]
(OTG_DOEPCTLx)
Valid for INT/BULK endpoints, see next section for ISO endpoints.
The application uses this register to control the behavior of each logical endpoint other than
endpoint 0.
Address offset: 0xB00 + 0x20 * x, (x = 1 to 8)
Reset value: 0x0000 0000
31

30

EPENA EPDIS

29

28

SD1
PID

SD0
PID

27
SNAK

26
CNAK

rs

rs

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

USBA
EP

Res.

Res.

Res.

Res.

rw

25

24

23

22

Res.

Res.

Res.

Res.

9

8

7

6

21

20

19

18

17

16

EPTYP[1:0]

NAK
STS

DPID

STALL

SNPM

rw

rw

rw

rw

r

r

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

MPSIZ[10:0]
rw

rw

rw

rw

rw

rw

Bit 31 EPENA: Endpoint enable
Applies to IN and OUT endpoints.
The application sets this bit to start transmitting data on an endpoint.
The core clears this bit before setting any of the following interrupts on this endpoint:
– SETUP phase done
– Endpoint disabled
– Transfer completed
Bit 30 EPDIS: Endpoint disable
The application sets this bit to stop transmitting/receiving data on an endpoint, even before
the transfer for that endpoint is complete. The application must wait for the endpoint
disabled interrupt before treating the endpoint as disabled. The core clears this bit before
setting the endpoint disabled interrupt. The application must set this bit only if endpoint
enable is already set for this endpoint.

RM0456 Rev 6

<!-- pagebreak -->

3460

USB on-the-go high-speed (OTG_HS)

RM0456

Bit 29 SD1PID: Set DATA1 PID
Writing to this field sets the endpoint data PID (DPID) field in this register to DATA1.
Bit 28 SD0PID: Set DATA0 PID
Applies to interrupt/bulk OUT endpoints only.
Writing to this field sets the endpoint data PID (DPID) field in this register to DATA0.
Bit 27 SNAK: Set NAK
A write to this bit sets the NAK bit for the endpoint.
Using this bit, the application can control the transmission of NAK handshakes on an
endpoint. The core can also set this bit for OUT endpoints on a transfer completed interrupt,
or after a SETUP is received on the endpoint.
Bit 26 CNAK: Clear NAK
A write to this bit clears the NAK bit for the endpoint.
Bits 25:22 Reserved, must be kept at reset value.
Bit 21 STALL: STALL handshake
Applies to non-control, non-isochronous OUT endpoints only (access type is rw).
The application sets this bit to stall all tokens from the USB host to this endpoint. If a NAK
bit, Global IN NAK, or Global OUT NAK is set along with this bit, the STALL bit takes
priority. Only the application can clear this bit, never the core.
Applies to control endpoints only (access type is rs).
The application can only set this bit, and the core clears it, when a SETUP token is received
for this endpoint. If a NAK bit, Global IN NAK, or Global OUT NAK is set along with this bit,
the STALL bit takes priority. Irrespective of this bit’s setting, the core always responds to
SETUP data packets with an ACK handshake.
Bit 20 SNPM: Snoop mode
This bit configures the endpoint to Snoop mode. In Snoop mode, the core does not check
the correctness of OUT packets before transferring them to application memory.
Bits 19:18 EPTYP[1:0]: Endpoint type
This is the transfer type supported by this logical endpoint.
00: Control
01: Isochronous
10: Bulk
11: Interrupt
Bit 17 NAKSTS: NAK status
Indicates the following:
0: The core is transmitting non-NAK handshakes based on the FIFO status.
1: The core is transmitting NAK handshakes on this endpoint.
When either the application or the core sets this bit:
The core stops receiving any data on an OUT endpoint, even if there is space in the Rx
FIFO to accommodate the incoming packet.
Irrespective of this bit’s setting, the core always responds to SETUP data packets with an
ACK handshake.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

USB on-the-go high-speed (OTG_HS)

Bit 16 DPID: Endpoint data PID
Applies to interrupt/bulk OUT endpoints only.
Contains the PID of the packet to be received or transmitted on this endpoint. The
application must program the PID of the first packet to be received or transmitted on this
endpoint, after the endpoint is activated. The application uses the SD0PID and SD1PID
register fields to program either DATA0 or DATA1 PID.
0: DATA0
1: DATA1
Bit 15 USBAEP: USB active endpoint
Indicates whether this endpoint is active in the current configuration and interface. The core
clears this bit for all endpoints (other than EP 0) after detecting a USB reset. After receiving
the SetConfiguration and SetInterface commands, the application must program endpoint
registers accordingly and set this bit.
Bits 14:11 Reserved, must be kept at reset value.
Bits 10:0 MPSIZ[10:0]: Maximum packet size
The application must program this field with the maximum packet size for the current logical
endpoint. This value is in bytes.

73.14.59 OTG device OUT endpoint x control register [alternate]
(OTG_DOEPCTLx)
Valid for ISO endpoints, see previous section for INT/BULK endpoints.
The application uses this register to control the behavior of each logical endpoint other than
endpoint 0.
Address offset: 0xB00 + 0x20 * x, (x = 1 to 8)
Reset value: 0x0000 0000
31

30

EPENA EPDIS

29

28

SODD
FRM

SEVN
FRM

27
SNAK

26
CNAK

rs

rs

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

USBA
EP

Res.

Res.

Res.

Res.

rw

25

24

23

22

Res.

Res.

Res.

Res.

9

8

7

6

21

20

19

18

17

16

EPTYP[1:0]

NAK
STS

EO
NUM

STALL

SNPM

rw

rw

rw

rw

r

r

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

MPSIZ[10:0]
rw

rw

rw

rw

rw

rw

Bit 31 EPENA: Endpoint enable
Applies to IN and OUT endpoints.
The application sets this bit to start transmitting data on an endpoint.
The core clears this bit before setting any of the following interrupts on this endpoint:
– SETUP phase done
– Endpoint disabled
– Transfer completed

RM0456 Rev 6

<!-- pagebreak -->

3460

USB on-the-go high-speed (OTG_HS)

RM0456

Bit 30 EPDIS: Endpoint disable
The application sets this bit to stop transmitting/receiving data on an endpoint, even before
the transfer for that endpoint is complete. The application must wait for the endpoint
disabled interrupt before treating the endpoint as disabled. The core clears this bit before
setting the endpoint disabled interrupt. The application must set this bit only if endpoint
enable is already set for this endpoint.
Bit 29 SODDFRM: Set odd frame
Applies to isochronous IN and OUT endpoints only.
Writing to this field sets the Even/Odd frame (EONUM) field to odd frame.
Bit 28 SEVNFRM: Set even frame
Applies to isochronous OUT endpoints only.
Writing to this field sets the Even/Odd frame (EONUM) field to even frame.
Bit 27 SNAK: Set NAK
A write to this bit sets the NAK bit for the endpoint.
Using this bit, the application can control the transmission of NAK handshakes on an
endpoint. The core can also set this bit for OUT endpoints on a transfer completed interrupt,
or after a SETUP is received on the endpoint.
Bit 26 CNAK: Clear NAK
A write to this bit clears the NAK bit for the endpoint.
Bits 25:22 Reserved, must be kept at reset value.
Bit 21 STALL: STALL handshake
Applies to non-control, non-isochronous OUT endpoints only (access type is rw).
The application sets this bit to stall all tokens from the USB host to this endpoint. If a NAK
bit, Global IN NAK, or Global OUT NAK is set along with this bit, the STALL bit takes
priority. Only the application can clear this bit, never the core.
Applies to control endpoints only (access type is rs).
The application can only set this bit, and the core clears it, when a SETUP token is received
for this endpoint. If a NAK bit, Global IN NAK, or Global OUT NAK is set along with this bit,
the STALL bit takes priority. Irrespective of this bit’s setting, the core always responds to
SETUP data packets with an ACK handshake.
Bit 20 SNPM: Snoop mode
This bit configures the endpoint to Snoop mode. In Snoop mode, the core does not check
the correctness of OUT packets before transferring them to application memory.
Bits 19:18 EPTYP[1:0]: Endpoint type
This is the transfer type supported by this logical endpoint.
00: Control
01: Isochronous
10: Bulk
11: Interrupt
Bit 17 NAKSTS: NAK status
Indicates the following:
0: The core is transmitting non-NAK handshakes based on the FIFO status.
1: The core is transmitting NAK handshakes on this endpoint.
When either the application or the core sets this bit:
The core stops receiving any data on an OUT endpoint, even if there is space in the Rx
FIFO to accommodate the incoming packet.
Irrespective of this bit’s setting, the core always responds to SETUP data packets with an
ACK handshake.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

USB on-the-go high-speed (OTG_HS)

Bit 16 EONUM: Even/odd frame
Applies to isochronous OUT endpoints only.
Indicates the frame number in which the core transmits/receives isochronous data for this
endpoint. The application must program the even/odd frame number in which it intends to
transmit/receive isochronous data for this endpoint using the SEVNFRM and SODDFRM
fields in this register.
0: Even frame
1: Odd frame
Bit 15 USBAEP: USB active endpoint
Indicates whether this endpoint is active in the current configuration and interface. The core
clears this bit for all endpoints (other than EP 0) after detecting a USB reset. After receiving
the SetConfiguration and SetInterface commands, the application must program endpoint
registers accordingly and set this bit.
Bits 14:11 Reserved, must be kept at reset value.
Bits 10:0 MPSIZ[10:0]: Maximum packet size
The application must program this field with the maximum packet size for the current logical
endpoint. This value is in bytes.

73.14.60 OTG device OUT endpoint x transfer size register
(OTG_DOEPTSIZx)
The application must modify this register before enabling the endpoint. Once the endpoint is
enabled using endpoint enable bit of the OTG_DOEPCTLx registers (EPENA bit in
OTG_DOEPCTLx), the core modifies this register. The application can only read this
register once the core has cleared the endpoint enable bit.
Address offset: 0xB10 + 0x20 * x, (x = 1 to 8)
Reset value: 0x0000 0000
31

30

Res.

RXDPID[1:0]

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

PKTCNT[9:0]

17

16

XFRSIZ[18:16]

XFRSIZ[15:0]
rw

rw

rw

rw

rw

rw

rw

rw

rw

Bit 31 Reserved, must be kept at reset value.

RM0456 Rev 6

<!-- pagebreak -->

3460

USB on-the-go high-speed (OTG_HS)

RM0456

Bits 30:29 RXDPID[1:0]:
Condition: isochronous OUT endpoints
Received data PID
This is the data PID received in the last packet for this endpoint.
00: DATA0
01: DATA2
10: DATA1
11: MDATA
Condition: control OUT endpoints
STUPCNT[1:0]: SETUP packet count
This field specifies the number of back-to-back SETUP data packets the endpoint can
receive.
01: 1 packet
10: 2 packets
11: 3 packets
Bits 28:19 PKTCNT[9:0]: Packet count
Indicates the total number of USB packets that constitute the transfer size amount of data for
this endpoint.
This field is decremented every time a packet (maximum size or short packet) is written to
the Rx FIFO.
Bits 18:0 XFRSIZ[18:0]: Transfer size
This field contains the transfer size in bytes for the current endpoint. The core only interrupts
the application after it has exhausted the transfer size amount of data. The transfer size can
be set to the maximum packet size of the endpoint, to be interrupted at the end of each
packet.
The core decrements this field every time a packet is read from the Rx FIFO and written to
the external memory.

73.14.61 OTG power and clock gating control register (OTG_PCGCCTL)
This register is available in host and device modes.
Address offset: 0xE00
Reset value: 0x200B 8000
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

SUSP

PHY
SLEEP

ENL1
GTG

PHY
SUSP

Res.

GATE
HCLK

STPP
CLK

r

r

rw

r

rw

rw

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Bits 31:8 Reserved, must be kept at reset value.
Bit 7 SUSP: Deep Sleep
This bit indicates that the PHY is in Deep Sleep when in L1 state.
Bit 6 PHYSLEEP: PHY in Sleep
This bit indicates that the PHY is in the Sleep state.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

USB on-the-go high-speed (OTG_HS)

Bit 5 ENL1GTG: Enable sleep clock gating
When this bit is set, core internal clock gating is enabled in Sleep state if the core cannot
assert utmi_l1_suspend_n. When this bit is not set, the PHY clock is not gated in Sleep
state.
Bit 4 PHYSUSP: PHY suspended
Indicates that the PHY has been suspended. This bit is updated once the PHY is suspended
after the application has set the STPPCLK bit.
Bits 3:2 Reserved, must be kept at reset value.
Bit 1 GATEHCLK: Gate HCLK
The application sets this bit to gate HCLK to modules other than the AHB Slave and Master
and wake-up logic when the USB is suspended or the session is not valid. The application
clears this bit when the USB is resumed or a new session starts.
Bit 0 STPPCLK: Stop PHY clock
The application sets this bit to stop the PHY clock when the USB is suspended, the session
is not valid, or the device is disconnected. The application clears this bit when the USB is
resumed or a new session starts.

73.14.62 OTG power and clock gating control register 1 (OTG_PCGCCTL1)
This register is available in host and device modes.
Address offset: 0xE04
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

RAM
GATE
EN

Res.

Res.

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

CNTGATECLK
[1:0]
rw

rw

GATE
EN
rw

Bits 31:4 Reserved, must be kept at reset value.
Bit 3 RAMGATEEN: Enable RAM clock gating
Enable gating of the FIFO RAM.
Bits 2:1 CNTGATECLK[1:0]: Counter for clock gating
Indicates to the controller how many PHY Clock cycles and AHB Clock cycles of 'IDLE' (no
activity) the controller waits for before Gating the respective PHY and AHB clocks internal to
the controller.
00: 64 clocks
01: 128 clocks
10: Reserved
11: Reserved
Bit 0 GATEEN: Enable active clock gating
The application programs GATEEN to enable Active Clock Gating feature for the PHY and
AHB clocks.

RM0456 Rev 6

<!-- pagebreak -->

3460

0

OTG_GINTSTS
(Device mode)
CIDSCHG
LPMINT
PTXFE
HCINT
HPRTINT
RSTDET
DATAFSUSP
IPXFR
IISOIXFR
OEPINT
IEPINT

Reset value
0
0
0
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

OTG_GINTSTS
(Host mode)
CIDSCHG
LPMINT
PTXFE
HCINT
HPRTINT
RSTDET
DATAFSUSP
INCOMPISOOUT
IISOIXFR
OEPINT
IEPINT

Reset value

0

0

0

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

<!-- pagebreak -->

