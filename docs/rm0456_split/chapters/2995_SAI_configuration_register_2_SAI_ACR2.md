3134

Universal serial bus full-speed host/device interface (USB)

71.5.3

RM0456

Double-buffered endpoints and usage in Device mode
All different endpoint types defined by the USB standard represent different traffic models,
and describe the typical requirements of different kind of data transfer operations. When
large portions of data are to be transferred between the host PC and the USB function, the
bulk endpoint type is the most suited model. This is because the host schedules bulk
transactions so as to fill all the available bandwidth in the frame, maximizing the actual
transfer rate as long as the USB function is ready to handle a bulk transaction addressed to
it. If the USB function is still busy with the previous transaction when the next one arrives, it
answers with a NAK handshake and the host PC issues the same transaction again until the
USB function is ready to handle it, reducing the actual transfer rate due to the bandwidth
occupied by re-transmissions. For this reason, a dedicated feature called ‘double-buffering’
can be used with bulk endpoints.
When ‘double-buffering’ is activated, data toggle sequencing is used to select, which buffer
is to be used by the USB peripheral to perform the required data transfers, using both
‘transmission’ and ‘reception’ packet memory areas to manage buffer swapping on each
successful transaction in order to always have a complete buffer to be used by the
application, while the USB peripheral fills the other one. For example, during an OUT
transaction directed to a ‘reception’ double-buffered bulk endpoint, while one buffer is being
filled with new data coming from the USB host, the other one is available for the
microcontroller software usage (the same would happen with a ‘transmission’ doublebuffered bulk endpoint and an IN transaction).
Since the swapped buffer management requires the usage of all 4 buffer description table
locations hosting the address pointer and the length of the allocated memory buffers, the
USB_CHEPnR registers used to implement double-buffered bulk endpoints are forced to be
used as unidirectional ones. Therefore, only one STAT bit pair must be set at a value
different from 00 (DISABLED): STATRX if the double-buffered bulk endpoint is enabled for
reception, STATTX if the double-buffered bulk endpoint is enabled for transmission. In case
it is required to have double-buffered bulk endpoints enabled both for reception and
transmission, two USB_CHEPnR registers must be used.
To exploit the double-buffering feature and reach the highest possible transfer rate, the
endpoint flow control structure, described in previous chapters, has to be modified, in order
to switch the endpoint status to NAK only when a buffer conflict occurs between the USB
peripheral and application software, instead of doing it at the end of each successful
transaction. The memory buffer which is currently being used by the USB peripheral is
defined by the DTOG bit related to the endpoint direction: DTOGRX (bit 14 of
USB_CHEPnR register) for ‘reception’ double-buffered bulk endpoints or DTOGTX (bit 6 of
USB_CHEPnR register) for ‘transmission’ double-buffered bulk endpoints. To implement the
new flow control scheme, the USB peripheral must know which packet buffer is currently in
use by the application software, so to be aware of any conflict. Since in the USB_CHEPnR
register, there are two DTOG bits but only one is used by USB peripheral for data and buffer
sequencing (due to the unidirectional constraint required by double-buffering feature) the
other one can be used by the application software to show which buffer it is currently using.
This new buffer flag is called SW_BUF. In the following table the correspondence between
USB_CHEPnR register bits and DTOG/SW_BUF definition is explained, for the cases of
‘transmission’ and ‘reception’ double-buffered bulk endpoints.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Universal serial bus full-speed host/device interface (USB)
Table 734. Double-buffering buffer flag definition
Buffer flag

‘Transmission’ endpoint

‘Reception’ endpoint

DTOG

DTOGTX (USB_CHEPnR bit 6)

DTOGRX (USB_CHEPnR bit 14)

SW_BUF

USB_CHEPnR bit 14

USB_CHEPnR bit 6

The memory buffer which is currently being used by the USB peripheral is defined by DTOG
buffer flag, while the buffer currently in use by application software is identified by SW_BUF
buffer flag. The relationship between the buffer flag value and the used packet buffer is the
same in both cases, and it is listed in the following table.
Table 735. Bulk double-buffering memory buffers usage (Device mode)
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
(ADDR_RX / COUNT_RX)
Buffer description table locations.

USB_CHEP_TXRXBD_0
(ADDR_RX / COUNT_RX)
Buffer description table locations.

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
(IN)

Receive
(OUT)

1. Endpoint in NAK Status.

Double-buffering feature for a bulk endpoint is activated by performing the two following
actions:
•

Writing UTYPE bit field at 00 in its USB_CHEPnR register, to define the endpoint as a
bulk

•

Setting EPKIND bit at 1 (DBL_BUF), in the same register.
RM0456 Rev 6

<!-- pagebreak -->

