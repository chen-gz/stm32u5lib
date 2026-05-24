RM0456 Rev 6

RM0456

Universal serial bus full-speed host/device interface (USB)
Figure 892. Packet buffer areas with examples of buffer description table locations
Available buffer address range
starting from offset 0x40

Available buffer descriptor
address range
starting from offset 0x00

Buffer for
double-buffered
IN endpoint 3
..
.

0x1C
0x18
0x14
0x10
0x0C
0x08
0x04
0x00

USB_CHEP_RXTXBD_3* [TX]
USB_CHEP_TXRXBD_3 [TX]
USB_CHEP_RXTXBD_2 [RX]
USB_CHEP_TXRXBD_2* [RX]
USB_CHEP_RXTXBD_1 [RX]
USB_CHEP_TXRXBD_1 [TX]
USB_CHEP_RXTXBD_0 [RX]
USB_CHEP_TXRXBD_0 [TX]

Buffer for
double-buffered
OUT endpoint 2
..
.
Transmission buffer
for
single-buffered
endpoint 1
..
.
Reception buffer for
endpoint 0
Not used

Transmission buffer
for endpoint 0
Packet buffers

(*) indicates alternate mode.

MSv74147V1

Each packet buffer is used either during reception or transmission starting from the bottom.
The USB peripheral never changes the contents of memory locations adjacent to the
allocated memory buffers; if a packet bigger than the allocated buffer length is received
(buffer overrun condition) the data is copied to the memory only up to the last available
location.

Endpoint initialization
The first step to initialize an endpoint is to write appropriate values to the
ADDRn_TX/ADDRn_RX fields in the CHEP_TXBD_n and CHEP_RXBD_n registers (in
SRAM) so that the USB peripheral finds the data to be transmitted already available and the
data to be received can be buffered. The UTYPE bits in the USB_CHEPnR register must be
set according to the endpoint type, eventually using the EPKIND bit to enable any special
required feature. On the transmit side, the endpoint must be enabled using the STATTX bits
in the USB_CHEPnR register and COUNTn_TX must be initialized. For reception, STATRX
bits must be set to enable reception and COUNTn_RX must be written with the allocated
buffer size using the BLSIZE and NUM_BLOCK fields. Unidirectional endpoints, except
isochronous and double-buffered bulk endpoints, need to initialize only bits and registers
related to the supported direction. Once the transmission and/or reception are enabled,
register USB_CHEPnR and locations ADDRn_TX/ADDRn_RX, COUNTn_TX/COUNTn_RX
(respectively), must not be modified by the application software, as the hardware can
RM0456 Rev 6

<!-- pagebreak -->

3134

Universal serial bus full-speed host/device interface (USB)

RM0456

change their value on the fly. When the data transfer operation is completed, notified by a
CTR interrupt event, they can be accessed again to re-enable a new operation.

Data transmission in Device mode (IN packets)
When receiving an IN token packet, if the received address matches a configured and valid
endpoint, the USB peripheral accesses the contents of CHEP_TXBD_n (fields ADDRn_TX
and COUNTn_TX) inside the buffer descriptor table entry related to the addressed endpoint.
The content of these locations is stored in its internal 16-bit registers ADDR and COUNT
(not accessible by software). The packet memory is accessed again to read the first byte to
be transmitted (refer to Structure and usage of packet buffers on page 3093) and the USB
peripheral starts sending a DATA0 or DATA1 PID according to USB_CHEPnR bit DTOGTX.
When the PID is completed, the first byte, read from buffer memory, is loaded into the output
shift register to be transmitted on the USB bus. After the last data byte is transmitted, the
computed CRC is sent. If the addressed endpoint is not valid, a NAK or STALL handshake
packet is sent instead of the data packet, according to STATTX bits in the USB_CHEPnR
register.
The ADDRn_TX field in the internal register CHEP_TXBD_n is used as a pointer to the
current buffer memory location while COUNT is used to count the number of remaining
bytes to be transmitted. Each half-word read from the packet buffer memory is transmitted
over the USB bus starting from the least significant byte. Transmission buffer memory is
read starting from the address pointed by ADDRn_TX for COUNTn_TX/4 words. If a
transmitted packet is composed of an odd number of bytes, only the lower half of the last
half-word accessed is used.
On receiving the ACK receipt by the host, the USB_CHEPnR register is updated in the
following way: DTOGTX bit is toggled, the endpoint is made invalid by setting
STATTX = 10 (NAK) and bit VTTX is set. The application software must first identify the
endpoint, which is requesting microcontroller attention by examining the IDN and DIR bits in
the USB_ISTR register. Servicing of the VTTX event starts, clearing the interrupt bit; the
application software then prepares another buffer full of data to be sent, updates the
COUNTn_TX table location with the number of byte to be transmitted during the next
transfer, and finally sets STATTX to 11 (VALID) to re-enable transmission. While the
STATTX bits are equal to 10 (NAK), any IN request addressed to that endpoint is NAKed,
indicating a flow control condition: the USB host retries the transaction until it succeeds. It is
mandatory to execute the sequence of operations in the above mentioned order to avoid
losing the notification of a second IN transaction addressed to the same endpoint
immediately following the one which triggered the CTR interrupt.

Data transmission in Host mode (OUT packets)
Data transmission in Host mode follows the same general principles as Device mode. The
main differences are due to the protocol. For example the host initiates the transmission
whereas the device responds to the incoming token.
ADDRn_TX must be set to the location in the packet memory reserved for the packet for
transmission. The contents of an OUT packet are then written to that address in the packet
memory and COUNTn_TX must be updated (when necessary) to indicate the number of
bytes in the packet.
DEVADDR must be written for the correct endpoint and then STATTX must be set to 11
(VALID) in order to trigger the transmit. The transmission is then scheduled by the HFS.
After a successful transmission the CTR interrupt (correct transfer) is triggered. By
examining IDN and DIR bits, the corresponding channel and direction is understood. On the

<!-- pagebreak -->

