RM0456 Rev 6

RM0456

USB on-the-go high-speed (OTG_HS)
1.

When an STALL, TXERR, BBERR or DTERR interrupt in OTG_HCINTx is received for
an IN or OUT channel. The application must be able to receive other interrupts
(DTERR, Nak, data, TXERR) for the same channel before receiving the halt.

2.

When an XFRC interrupt in OTG_HCINTx is received during a non periodic IN transfer
or high-bandwidth interrupt IN transfer

3.

When a DISCINT (disconnect device) interrupt in OTG_GINTSTS is received. (The
application is expected to disable all enabled channels).

4.

When the application aborts a transfer before normal completion.

Ping protocol
When the OTG_HS host operates in high speed, the application must initiate the ping
protocol when communicating with high-speed bulk or control (data and status stage) OUT
endpoints. The application must initiate the ping protocol when it receives a
NAK/NYET/TXERR interrupt. When the OTG_HS host receives one of the above
responses, it does not continue any transaction for a specific endpoint, drops all posted or
fetched OUT requests (from the request queue), and flushes the corresponding data (from
the transmit FIFO).This is valid in slave mode only. In Slave mode, the application can send
a ping token either by setting the DOPING bit in OTG_HCTSIZx before enabling the channel
or by just writing the OTG_HCTSIZx register with the DOPING bit set when the channel is
already enabled. This enables the OTG_HS host to write a ping request entry to the request
queue. The application must wait for the response to the ping token (a NAK, ACK, or
TXERR interrupt) before continuing the transaction or sending another ping token. The
application can continue the data transaction only after receiving an ACK from the OUT
endpoint for the requested ping. In DMA mode operation, the application does not need to
set the DOPING bit in OTG_HCTSIZx for a NAK/NYET response in case of bulk/control
OUT. The OTG_HS host automatically sets the DOPING bit in OTG_HCTSIZx, and issues
the ping tokens for bulk/control OUT. The OTG_HS host continues sending ping tokens until
it receives an ACK, and then switches automatically to the data transaction.

Operational model
The application must initialize a channel before communicating to the connected device.
This section explains the sequence of operation to be performed for different types of USB
transactions.
•

Writing the transmit FIFO
The OTG_HS host automatically writes an entry (OUT request) to the periodic/nonperiodic request queue, along with the last 32-bit word write of a packet. The
application must ensure that at least one free space is available in the periodic/nonperiodic request queue before starting to write to the transmit FIFO. The application
must always write to the transmit FIFO in 32-bit words. If the packet size is non-32-bit
word aligned, the application must use padding. The OTG_HS host determines the
actual packet size based on the programmed maximum packet size and transfer size.

RM0456 Rev 6

<!-- pagebreak -->

