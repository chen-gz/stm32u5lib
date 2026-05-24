RM0456 Rev 6

RM0456

Universal serial bus full-speed host/device interface (USB)
Table 736. Bulk double-buffering memory buffers usage (Host mode)
Endpoint
DTOG SW_BUF
type

Packet buffer used by
USB peripheral

Packet buffer used by
Application Software

0

1

USB_CHEP_TXRXBD_0
(ADDR_TX / COUNT_TX)
Buffer description table locations.

USB_CHEP_RXTXBD_0
(ADDR_TX / COUNT_TX)
Buffer description table locations

1

0

USB_CHEP_RXTXBD_0
(ADDR_TX / COUNT_TX)
Buffer description table locations

USB_CHEP_TXRXBD_0
(ADDR_TX / COUNT_TX)
Buffer description table locations.

0

0

None (1)

USB_CHEP_TXRXBD_0
(ADDR_TX / COUNT_TX)
Buffer description table locations.

1

1

None (1)

USB_CHEP_RXTXBD_0
(ADDR_TX / COUNT_TX)
Buffer description table locations.

0

1

USB_CHEP_RXTXBD_0
USB_CHEP_TXRXBD_0
(ADDR_RX / COUNT_RX)
(ADDR_RX / COUNT_RX)
Buffer description table locations. Buffer description table locations.

1

0

USB_CHEP_TXRXBD_0
(ADDR_RX / COUNT_RX)
Buffer description table locations

USB_CHEP_RXTXBD_0
(ADDR_RX / COUNT_RX)
Buffer description table locations.

0

0

None (1)

USB_CHEP_RXTXBD_0
(ADDR_RX / COUNT_RX)
Buffer description table locations.

1

1

None (1)

USB_CHEP_TXRXBD_0
(ADDR_RX / COUNT_RX)
Buffer description table locations.

Transmit
(OUT)

Receive
(IN)

1. Endpoint in NAK Status.

71.5.5

Isochronous transfers in Device mode
The USB standard supports full speed peripherals requiring a fixed and accurate data
production/consume frequency, defining this kind of traffic as ‘isochronous’. Typical
examples of this data are: audio samples, compressed video streams, and in general any
sort of sampled data having strict requirements for the accuracy of delivered frequency.
When an endpoint is defined to be ‘isochronous’ during the enumeration phase, the host
allocates in the frame the required bandwidth and delivers exactly one IN or OUT packet
each frame, depending on endpoint direction. To limit the bandwidth requirements, no retransmission of failed transactions is possible for isochronous traffic; this leads to the fact
that an isochronous transaction does not have a handshake phase and no ACK packet is
expected or sent after the data packet. For the same reason, isochronous transfers do not
support data toggle sequencing and always use DATA0 PID to start any data packet.
The isochronous behavior for an endpoint is selected by setting the UTYPE bits at 10 in its
USB_CHEPnR register; since there is no handshake phase the only legal values for the
STATRX/STATTX bit pairs are 00 (DISABLED) and 11 (VALID), any other value produces
results not compliant to USB standard. Isochronous endpoints implement double-buffering
RM0456 Rev 6

<!-- pagebreak -->

