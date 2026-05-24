RM0456 Rev 6

RM0456

USB on-the-go full-speed (OTG_FS)
The end of periodic frame interrupt (OTG_GINTSTS/EOPF) is used to notify the application
when 80%, 85%, 90% or 95% of the time frame interval elapsed depending on the periodic
frame interval field in the device configuration register (PFIVL bit in OTG_DCFG). This
feature can be used to determine if all of the isochronous traffic for that frame is complete.

72.9

OTG_FS low-power modes
Table 752 below defines the STM32 low power modes and their compatibility with the OTG.
Table 752. Compatibility of STM32 low power modes with the OTG

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
The power consumption of the OTG PHY is controlled by two or three bits in the general
core configuration register, depending on OTG revision supported.
•

PHY power down (OTG_GCCFG/PWRDWN)
It switches on/off the full-speed transceiver module of the PHY. It must be preliminarily
set to allow any USB operation

•

VBUS detection enable (OTG_GCCFG/VBDEN)
It switches on/off the VBUS sensing comparators associated with OTG operations

Power reduction techniques are available while in the USB suspended state, when the USB
session is not yet valid or the device is disconnected.
•

Stop PHY clock (STPPCLK bit in OTG_PCGCCTL)
When setting the stop PHY clock bit in the clock gating control register, most of the
48 MHz clock domain internal to the OTG core is switched off by clock gating. The
dynamic power consumption due to the USB clock switching activity is cut even if the
48 MHz clock input is kept running by the application
Most of the transceiver is also disabled, and only the part in charge of detecting the
asynchronous resume or remote wake-up event is kept alive.

•

Gate HCLK (GATEHCLK bit in OTG_PCGCCTL)
When setting the Gate HCLK bit in the clock gating control register, most of the system
clock domain internal to the OTG_FS core is switched off by clock gating. Only the
register read and write interface is kept alive. The dynamic power consumption due to

RM0456 Rev 6

<!-- pagebreak -->

3291

USB on-the-go full-speed (OTG_FS)

RM0456

the USB clock switching activity is cut even if the system clock is kept running by the
application for other purposes.
•

USB system stop
When the OTG_FS is in the USB suspended state, the application may decide to
drastically reduce the overall power consumption by a complete shut down of all the
clock sources in the system. USB System Stop is activated by first setting the Stop
PHY clock bit and then configuring the system deep sleep mode in the power control
system module (PWR).
The OTG_FS core automatically reactivates both system and USB clocks by
asynchronous detection of remote wake-up (as an host) or resume (as a device)
signaling on the USB.

To save dynamic power, the USB data FIFO is clocked only when accessed by the OTG_FS
core.

72.10

OTG_FS Dynamic update of the OTG_HFIR register
The USB core embeds a dynamic trimming capability of SOF framing period in host mode
allowing to synchronize an external device with the SOF frames.
When the OTG_HFIR register is changed within a current SOF frame, the SOF period
correction is applied in the next frame as described in Figure 898.
For a dynamic update, it is required to set RLDCTRL=1.
Figure 898. Updating OTG_HFIR dynamically (RLDCTRL = 1)
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

72.11

OTG_FS data FIFOs
The USB system features 1.25 Kbytes of dedicated RAM with a sophisticated FIFO control
mechanism. The packet FIFO controller module in the OTG_FS core organizes RAM space
into Tx FIFOs into which the application pushes the data to be temporarily stored before the
USB transmission, and into a single Rx FIFO where the data received from the USB are
temporarily stored before retrieval (popped) by the application. The number of instructed
FIFOs and how these are organized inside the RAM depends on the device’s role. In
peripheral mode an additional Tx FIFO is instructed for each active IN endpoint. Any FIFO
size is software configured to better meet the application requirements.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

72.11.1

USB on-the-go full-speed (OTG_FS)

Peripheral FIFO architecture
Figure 899. Device-mode FIFO address mapping and AHB FIFO access mapping
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

The OTG_FS core can fill in the receive FIFO up to the limit for any host sequence of
OUT tokens

The application keeps receiving the Rx FIFO non-empty interrupt (RXFLVL bit in
OTG_GINTSTS) as long as there is at least one packet available for download. It reads the
packet information from the receive status read and pop register (OTG_GRXSTSP) and
finally pops data off the receive FIFO by reading from the endpoint-related pop address.

RM0456 Rev 6

<!-- pagebreak -->

3291

USB on-the-go full-speed (OTG_FS)

RM0456

Peripheral Tx FIFOs
The core has a dedicated FIFO for each IN endpoint. The application configures FIFO sizes
by writing the endpoint 0 transmit FIFO size register (OTG_DIEPTXF0) for IN endpoint0 and
the device IN endpoint transmit FIFOx registers (OTG_DIEPTXFx) for IN endpoint-x.

72.11.2

Host FIFO architecture
Figure 900. Host-mode FIFO address mapping and AHB FIFO access mapping
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

The OTG_FS core can fill in the receive FIFO up to the limit for any sequence of IN
tokens driven by the host software

The application receives the Rx FIFO not-empty interrupt as long as there is at least one
packet available for download. It reads the packet information from the receive status read
and pop register and finally pops the data off the receive FIFO.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

USB on-the-go full-speed (OTG_FS)

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

The OTG_FS core can fill in the periodic (nonperiodic) transmit FIFO up to the limit for
any sequence of OUT tokens driven by the host software

The OTG_FS core issues the periodic Tx FIFO empty interrupt (PTXFE bit in
OTG_GINTSTS) as long as the periodic Tx FIFO is half or completely empty, depending on
the value of the periodic Tx FIFO empty level bit in the AHB configuration register
(PTXFELVL bit in OTG_GAHBCFG). The application can push the transmission data in
advance as long as free space is available in both the periodic Tx FIFO and the periodic
request queue. The host periodic transmit FIFO and queue status register
(OTG_HPTXSTS) can be read to know how much space is available in both.
OTG_FS core issues the non periodic Tx FIFO empty interrupt (NPTXFE bit in
OTG_GINTSTS) as long as the nonperiodic Tx FIFO is half or completely empty depending
on the non periodic Tx FIFO empty level bit in the AHB configuration register (TXFELVL bit
in OTG_GAHBCFG). The application can push the transmission data as long as free space
is available in both the nonperiodic Tx FIFO and nonperiodic request queue. The host
nonperiodic transmit FIFO and queue status register (OTG_HNPTXSTS) can be read to
know how much space is available in both.

72.11.3

FIFO RAM allocation
Device mode
Receive FIFO RAM allocation: the application must allocate RAM for SETUP packets:
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

<!-- pagebreak -->

3291

USB on-the-go full-speed (OTG_FS)

RM0456

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

<!-- pagebreak -->

