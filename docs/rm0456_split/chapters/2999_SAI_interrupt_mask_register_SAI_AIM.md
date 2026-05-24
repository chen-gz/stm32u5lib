3134

Universal serial bus full-speed host/device interface (USB)

RM0456

to ease application software development, using both ‘transmission’ and ‘reception’ packet
memory areas to manage buffer swapping on each successful transaction in order to have
always a complete buffer to be used by the application, while the USB peripheral fills the
other.
The memory buffer which is currently used by the USB peripheral is defined by the DTOG
bit related to the endpoint direction (DTOGRX for ‘reception’ isochronous endpoints,
DTOGTX for ‘transmission’ isochronous endpoints, both in the related USB_CHEPnR
register) according to Table 737.
Table 737. Isochronous memory buffers usage
Endpoint
Type

DTOG bit
value

Packet buffer used by the
USB peripheral

Packet buffer used by the
application software

0

USB_CHEP_TXRXBD_0
(ADDR_TX / COUNT_TX)
Buffer description table locations.

USB_CHEP_RXTXBD_0
(ADDR_TX / COUNT_TX)
Buffer description table locations

1

USB_CHEP_RXTXBD_0
(ADDR_TX / COUNT_TX)
Buffer description table locations

USB_CHEP_TXRXBD_0
(ADDR_TX / COUNT_TX)
Buffer description table locations.

0

USB_CHEP_RXTXBD_0
(ADDR_RX / COUNT_RX)
Buffer description table locations.

USB_CHEP_TXRXBD_0
(ADDR_RX / COUNT_RX)
Buffer description table locations.

1

USB_CHEP_TXRXBD_0
(ADDR_RX / COUNT_RX)
Buffer description table locations

USB_CHEP_RXTXBD_0
(ADDR_RX / COUNT_RX)
Buffer description table locations.

Transmit
(IN)

Receive
(OUT)

As it happens with double-buffered bulk endpoints, the USB_CHEPnR registers used to
implement isochronous endpoints are forced to be used as unidirectional ones. In case it is
required to have isochronous endpoints enabled both for reception and transmission, two
USB_CHEPnR registers must be used.
The application software is responsible for the DTOG bit initialization according to the first
buffer to be used; this has to be done considering the special toggle-only property that these
two bits have. At the end of each transaction, the VTRX or VTTX bit of the addressed
endpoint USB_CHEPnR register is set, depending on the enabled direction. At the same
time, the affected DTOG bit in the USB_CHEPnR register is hardware toggled making buffer
swapping completely software independent. STAT bit pair is not affected by transaction
completion; since no flow control is possible for isochronous transfers due to the lack of
handshake phase, the endpoint remains always 11 (VALID). CRC errors or buffer-overrun
conditions occurring during isochronous OUT transfers are anyway considered as correct
transactions and they always trigger a VTRX event. However, CRC errors set the ERR bit in
the USB_ISTR register anyway, in order to notify the software of the possible data
corruption.

71.5.6

Isochronous transfers in Host mode
From the host point of view isochronous packets are issued or requested one by frame by
the host frame scheduler. There is no NAK/ACK protocol and no resend of data or token.

<!-- pagebreak -->

