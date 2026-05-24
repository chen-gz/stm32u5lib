RM0456 Rev 6

RM0456

USB on-the-go full-speed (OTG_FS)

Bit 21 STALL: STALL handshake
The application can only set this bit, and the core clears it when a SETUP token is received
for this endpoint. If a NAK bit, a Global IN NAK or Global OUT NAK is set along with this bit,
the STALL bit takes priority.
Bit 20 Reserved, must be kept at reset value.
Bits 19:18 EPTYP[1:0]: Endpoint type
Hardcoded to ‘00’ for control.
Bit 17 NAKSTS: NAK status
Indicates the following:
0: The core is transmitting non-NAK handshakes based on the FIFO status
1: The core is transmitting NAK handshakes on this endpoint.
When this bit is set, either by the application or core, the core stops transmitting data, even
if there are data available in the Tx FIFO. Irrespective of this bit’s setting, the core always
responds to SETUP data packets with an ACK handshake.
Bit 16 Reserved, must be kept at reset value.
Bit 15 USBAEP: USB active endpoint
This bit is always set to 1, indicating that control endpoint 0 is always active in all
configurations and interfaces.
Bits 14:2 Reserved, must be kept at reset value.
Bits 1:0 MPSIZ[1:0]: Maximum packet size
The application must program this field with the maximum packet size for the current logical
endpoint.
00: 64 bytes
01: 32 bytes
10: 16 bytes
11: 8 bytes

72.15.44 OTG device IN endpoint x control register (OTG_DIEPCTLx)
Address offset: 0x900 + 0x20 * x, (x = 1 to 5)
Reset value: 0x0000 0000
The application uses this register to control the behavior of each logical endpoint other than
endpoint 0.
31

30

EPENA EPDIS

29

28

27

26

SODD
FRM

SD0
PID/
SEVN
FRM

SNAK

CNAK

25

24

23

22

TXFNUM[3:0]

21

20

19

18

17

16

STALL

Res.

EPTYP[1:0]

NAK
STS

EO
NUM/
DPID

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

RM0456 Rev 6

rw

rw

<!-- pagebreak -->

3291

USB on-the-go full-speed (OTG_FS)

RM0456

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
Bit 28 SD0PID: Set DATA0 PID
Applies to interrupt/bulk IN endpoints only.
Writing to this field sets the endpoint data PID (DPID) field in this register to DATA0.
SEVNFRM: Set even frame
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

<!-- pagebreak -->

RM0456 Rev 6

RM0456

USB on-the-go full-speed (OTG_FS)

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
DPID: Endpoint data PID
Applies to interrupt/bulk IN endpoints only.
Contains the PID of the packet to be received or transmitted on this endpoint. The
application must program the PID of the first packet to be received or transmitted on this
endpoint, after the endpoint is activated. The application uses the SD0PID register field to
program either DATA0 or DATA1 PID.
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

RM0456 Rev 6

<!-- pagebreak -->

3291

USB on-the-go full-speed (OTG_FS)

RM0456

72.15.45 OTG device IN endpoint x interrupt register (OTG_DIEPINTx)
Address offset: 0x908 + 0x20 * x, (x = 0 to 5)
Reset value: 0x0000 0080
This register indicates the status of an endpoint with respect to USB- and AHB-related
events. It is shown in Figure 901. The application must read this register when the IN
endpoints interrupt bit of the core interrupt register (IEPINT in OTG_GINTSTS) is set.
Before the application can read this register, it must first read the device all endpoints
interrupt (OTG_DAINT) register to get the exact endpoint number for the device endpoint-x
interrupt register. The application must clear the appropriate bit in this register to clear the
corresponding bits in the OTG_DAINT and OTG_GINTSTS registers.
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

PKTD
RPSTS

TXFE

IN
EPNE

IN
ITTXFE
EPNM

TOC

Res.

EP
DISD

XFRC

r

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

Res.

Res.

rc_w1

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
Bit 8 Reserved, must be kept at reset value.
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

<!-- pagebreak -->

