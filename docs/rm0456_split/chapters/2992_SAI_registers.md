RM0456 Rev 6

RM0456

Universal serial bus full-speed host/device interface (USB)
indicated channel, the STATTX field now has transitioned to DISABLE. In the case of a NAK
being received (when the peripheral is not ready) STATTX is now in NAK. In the case of a
STALL response, STATTX is in STALL. In this last case, the bus must be reset.
On receiving the ACK receipt by the device, the USB_CHEPnR register is updated in the
following way: DTOGTX bit is toggled.
An error condition is signaled via the bits VTTX and ERR_TX if one of the following
conditions occurs:
•

No handshake being received in time

•

False EOP

•

Bit stuffing error

•

Invalid handshake PID

Data reception in Device mode (OUT and SETUP packets)
These two tokens are handled by the USB peripheral more or less in the same way; the
differences in the handling of SETUP packets are detailed in the following paragraph about
control transfers. When receiving an OUT/SETUP PID, if the address matches a valid
endpoint, the USB peripheral accesses the contents of the ADDRn_RX and COUNTn_RX
fields inside the buffer descriptor table entry related to the addressed endpoint. The content
of the ADDRn_RX field is stored directly in its internal register ADDR. Internal register
COUNT is now reset and the values of BLSIZE and NUM_BLOCK bit fields, which are read
within USB_CHEP_RXBD_n content, are used to initialize BUF_COUNT, an internal 16-bit
counter, which is used to check the buffer overrun condition (all these internal registers are
not accessible by software). Data bytes subsequently received by the USB peripheral are
packed in half-words (the first byte received is stored as least significant byte) and then
transferred to the packet buffer starting from the address contained in the internal ADDR
register while BUF_COUNT is decremented and COUNT is incremented at each byte
transfer. When the end of DATA packet is detected, the correctness of the received CRC is
tested and only if no errors occurred during the reception, an ACK handshake packet is sent
back to the transmitting host.
In case of wrong CRC or other kinds of errors (bit-stuff violations, frame errors, etc.), data
bytes are still copied in the packet memory buffer, at least until the error detection point, but
the ACK packet is not sent and the ERR bit in USB_ISTR register is set. However, there is
usually no software action required in this case: the USB peripheral recovers from reception
errors and remains ready for the next transaction to come. If the addressed endpoint is not
valid, a NAK or STALL handshake packet is sent instead of the ACK, according to bits
STATRX in the USB_CHEPnR register, and no data is written in the reception memory
buffers.
Reception memory buffer locations are written starting from the address contained in the
ADDRn_RX for a number of bytes corresponding to the received data packet length, or up
to the last allocated memory location, as defined by BLSIZE and NUM_BLOCK, whichever
comes first. In this way, the USB peripheral never writes beyond the end of the allocated
reception memory buffer area. If the length of the data packet payload (actual number of
bytes used by the application) is greater than the allocated buffer, the USB peripheral
detects a buffer overrun condition. In this case, a STALL handshake is sent instead of the
usual ACK to notify the problem to the host, no interrupt is generated and the transaction is
considered failed.
When the transaction is completed correctly, by sending the ACK handshake packet, the
internal COUNT register is copied back in the COUNTn_RX location inside the buffer

RM0456 Rev 6

<!-- pagebreak -->

3134

Universal serial bus full-speed host/device interface (USB)

RM0456

description table entry, leaving unaffected BLSIZE and NUM_BLOCK fields, which normally
do not require to be re-written, and the USB_CHEPnR register is updated in the following
way: DTOGRX bit is toggled, the endpoint is made invalid by setting STATRX = 10 (NAK)
and bit VTRX is set. If the transaction has failed due to errors or buffer overrun condition,
none of the previously listed actions take place. The application software must first identify
the endpoint, which is requesting microcontroller attention by examining the IDN and DIR
bits in the USB_ISTR register. The VTRX event is serviced by first determining the
transaction type (SETUP bit in the USB_CHEPnR register); the application software must
clear the interrupt flag bit and get the number of received bytes reading the COUNTn_RX
location inside the buffer description table entry related to the endpoint being processed.
After the received data is processed, the application software must set the STATRX bits to
11 (VALID) in the USB_CHEPnR, enabling further transactions. While the STATRX bits are
equal to 10 (NAK), any OUT request addressed to that endpoint is NAKed, indicating a flow
control condition: the USB host retries the transaction until it succeeds. It is mandatory to
execute the sequence of operations in the above mentioned order to avoid losing the
notification of a second OUT transaction addressed to the same endpoint following
immediately the one which triggered the CTR interrupt.

Data reception in Host mode (IN packets)
Data reception in Host mode follows the same general principles as Device mode. The main
differences are again due to the protocol. In the device, data can be received or not,
depending on readiness after previous operations, whereas the host only requests receive
data when it is ready and able to store them.
ADDRn_TX must be set to the location in the packet memory reserved for the packet for
transmission. The contents received in the data phase response to the IN token packet are
then written to that address in the packet memory and COUNTn_TX gets updated by
hardware during this process to indicate the number of bytes in the packet.
DEVADDR must be written for the correct endpoint and then STATRX must be set to VALID
in order to trigger the reception. The reception is then scheduled by the HFS.
After a successful reception the interrupt CTR (correct transfer) is triggered. By examining
IDN and DIR bits, the corresponding channel and direction is understood. On the indicated
channel, the STATRX field now has transitioned to DISABLE. In the case of a NAK being
received (when the peripheral is not ready) STATRX now is in NAK. In the case of a STALL
response, STATRX is in STALL. In this last case, the bus must be reset. During an IN packet
an error condition is signaled via the bits VTRX and ERR_RX if one of the following
conditions occurs:
•

False EOP

•

Bit stuffing error

•

Wrong CRC

Control transfers in Device mode
Control transfers are made of a SETUP transaction, followed by zero or more data stages,
all of the same direction, followed by a status stage (a zero-byte transfer in the opposite
direction). SETUP transactions are handled by control endpoints only and are very similar to
OUT ones (data reception) except that the values of DTOGTX and DTOGRX bits of the
addressed endpoint registers are set to 1 and 0 respectively, to initialize the control transfer,
and both STATTX and STATRX are set to 10 (NAK) to let software decide if subsequent
transactions must be IN or OUT depending on the SETUP contents. A control endpoint must
check SETUP bit in the USB_CHEPnR register at each VTRX event to distinguish normal

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Universal serial bus full-speed host/device interface (USB)
OUT transactions from SETUP ones. A USB Device can determine the number and
direction of data stages by interpreting the data transferred in the SETUP stage, and is
required to STALL the transaction in the case of errors. To do so, at all data stages before
the last, the unused direction must be set to STALL, so that, if the host reverses the transfer
direction too soon, it gets a STALL as a status stage.
While enabling the last data stage, the opposite direction must be set to NAK, so that, if the
host reverses the transfer direction (to perform the status stage) immediately, it is kept
waiting for the completion of the control operation. If the control operation completes
successfully, the software changes NAK to VALID, otherwise to STALL. At the same time, if
the status stage is an OUT, the STATUS_OUT (EPKIND in the USB_CHEPnR register) bit
must be set, so that an error is generated if a status transaction is performed with non-zero
data. When the status transaction is serviced, the application clears the STATUS_OUT bit
and sets STATRX to VALID (to accept a new command) and STATTX to NAK (to delay a
possible status stage immediately following the next setup).
Since the USB specification states that a SETUP packet cannot be answered with a
handshake different from ACK, eventually aborting a previously issued command to start
the new one, the USB logic does not permit a control endpoint to answer with a NAK or
STALL packet to a SETUP token received from the host.
When the STATRX bits are set to 01 (STALL) or 10 (NAK) and a SETUP token is received,
the USB accepts the data, performing the required data transfers and sends back an ACK
handshake. If that endpoint has a previously issued VTRX request not yet acknowledged by
the application (for example VTRX bit is still set from a previously completed reception), the
USB discards the SETUP transaction and does not answer with any handshake packet
regardless of its state, simulating a reception error and forcing the host to send the SETUP
token again. This is done to avoid losing the notification of a SETUP transaction addressed
to the same endpoint immediately following the transaction, which triggered the VTRX
interrupt.

Control transfers in Host mode
Control transfers are made of a SETUP transaction, followed by zero or more data stages,
all of the same direction, followed by a status stage (a zero-byte transfer in the opposite
direction). SETUP transactions are handled by control endpoints only. A control endpoint
must set the SETUP bit in the USB_CHEPnR register. The values of DTOGTX and
DTOGRX bits of the addressed endpoint registers are set to 0. Depending on whether it is a
control write or control read then STATTX or STATRX are set to 11 (ACTIVE) in order to
trigger the control transfer via the host frame scheduler.
On receiving a CTR interrupt the channel (device address and endpoint) can be determined
by examining IDN and DIR bits. Devices are expected to NAK every control unless the
packet is corrupted in which case they do not acknowledge. The situation is reflected in the
value of STATTX.
In the case of an error condition the ERR bit gets set. One possible case is where a CRC
error is seen at the device, in this case no ACK is returned to the host. The host sees no
ACK and after an appropriate delay this generates a timeout error with ERR_TX set (which
can generate an interrupt).

RM0456 Rev 6

<!-- pagebreak -->

