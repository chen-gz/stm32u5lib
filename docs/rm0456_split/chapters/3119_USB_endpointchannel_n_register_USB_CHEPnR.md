3291

USB on-the-go full-speed (OTG_FS)

RM0456

72.15.49 OTG device control OUT endpoint 0 control register
(OTG_DOEPCTL0)
Address offset: 0xB00
Reset value: 0x0000 8000
This section describes the OTG_DOEPCTL0 register. Nonzero control endpoints use
registers for endpoints 1–3.
31

30

EPENA EPDIS

29
Res.

28
Res.

27

26

SNAK

CNAK

w

w

25
Res.

24
Res.

23
Res.

22
Res.

21

20

19

18

17

16

EPTYP[1:0]

NAK
STS

Res.

STALL

SNPM

rs

rw

r

r

r
1

w

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
Hardcoded to 2’b00 for control.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

USB on-the-go full-speed (OTG_FS)

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
Bits 14:2 Reserved, must be kept at reset value.
Bits 1:0 MPSIZ[1:0]: Maximum packet size
The maximum packet size for control OUT endpoint 0 is the same as what is programmed in
control IN endpoint 0.
00: 64 bytes
01: 32 bytes
10: 16 bytes
11: 8 bytes

72.15.50 OTG device OUT endpoint x interrupt register (OTG_DOEPINTx)
Address offset: 0xB08 + 0x20 * x, (x = 0 to 5)
Reset value: 0x0000 0080
This register indicates the status of an endpoint with respect to USB- and AHB-related
events. It is shown in Figure 901. The application must read this register when the OUT
endpoints interrupt bit of the OTG_GINTSTS register (OEPINT bit in OTG_GINTSTS) is set.
Before the application can read this register, it must first read the OTG_DAINT register to
get the exact endpoint number for the OTG_DOEPINTx register. The application must clear
the appropriate bit in this register to clear the corresponding bits in the OTG_DAINT and
OTG_GINTSTS registers.
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

NAK

BERR

Res.

Res.

Res.

Res.

Res.

Res.

STSPH
SRX

OTEP
DIS

STUP

Res.

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

Bits 31:14 Reserved, must be kept at reset value.
Bit 13 NAK: NAK input
The core generates this interrupt when a NAK is transmitted or received by the device. In
case of isochronous IN endpoints the interrupt gets generated when a zero length packet is
transmitted due to unavailability of data in the Tx FIFO.
Bit 12 BERR: Babble error interrupt
The core generates this interrupt when babble is received for the endpoint.

RM0456 Rev 6

<!-- pagebreak -->

3291

USB on-the-go full-speed (OTG_FS)

RM0456

Bits 11:10 Reserved, must be kept at reset value.
Bit 9 Reserved, must be kept at reset value.
Bit 8 Reserved, must be kept at reset value.
Bit 7 Reserved, must be kept at reset value.
Bit 6 Reserved, must be kept at reset value.
Bit 5 STSPHSRX: Status phase received for control write
This interrupt is valid only for control OUT endpoints. This interrupt is generated only after
OTG_FS has transferred all the data that the host has sent during the data phase of a
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
Bit 2 Reserved, must be kept at reset value.
Bit 1 EPDISD: Endpoint disabled interrupt
This bit indicates that the endpoint is disabled per the application’s request.
Bit 0 XFRC: Transfer completed interrupt
This field indicates that the programmed transfer is complete on the AHB as well as on the
USB, for this endpoint.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

USB on-the-go full-speed (OTG_FS)

72.15.51 OTG device OUT endpoint 0 transfer size register
(OTG_DOEPTSIZ0)
Address offset: 0xB10
Reset value: 0x0000 0000
The application must modify this register before enabling endpoint 0. Once endpoint 0 is
enabled using the endpoint enable bit in the OTG_DOEPCTL0 registers (EPENA bit in
OTG_DOEPCTL0), the core modifies this register. The application can only read this
register once the core has cleared the endpoint enable bit.

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

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

PKTCNT

Nonzero endpoints use the registers for endpoints 1–5.

Res.

Res.

Res.

6

5

4

2

1

0

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

RM0456 Rev 6

<!-- pagebreak -->

3291

USB on-the-go full-speed (OTG_FS)

RM0456

72.15.52 OTG device OUT endpoint x control register
(OTG_DOEPCTLx)
Address offset: 0xB00 + 0x20 * x, (x = 1 to 5)
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

25

24

23

22

21

20

SD1
PID/
SODD
FRM

SD0
PID/
SEVN
FRM

SNAK

CNAK

Res.

Res.

Res.

Res.

STALL

SNPM

rw

rw

rw

5

4

rw

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

9

8

7

6

19

18

17

16

EPTYP[1:0]

NAK
STS

EO
NUM/
DPID

rw

r

r

3

2

1

0

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
Bit 29 SD1PID: Set DATA1 PID
Applies to interrupt/bulk IN and OUT endpoints only. Writing to this field sets the endpoint
data PID (DPID) field in this register to DATA1.
SODDFRM: Set odd frame
Applies to isochronous IN and OUT endpoints only. Writing to this field sets the Even/Odd
frame (EONUM) field to odd frame.
Bit 28 SD0PID: Set DATA0 PID
Applies to interrupt/bulk OUT endpoints only.
Writing to this field sets the endpoint data PID (DPID) field in this register to DATA0.
SEVNFRM: Set even frame
Applies to isochronous OUT endpoints only.
Writing to this field sets the Even/Odd frame (EONUM) field to even frame.
Bit 27 SNAK: Set NAK
A write to this bit sets the NAK bit for the endpoint.
Using this bit, the application can control the transmission of NAK handshakes on an
endpoint. The core can also set this bit for OUT endpoints on a transfer completed interrupt,
or after a SETUP is received on the endpoint.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

USB on-the-go full-speed (OTG_FS)

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
Bit 16 EONUM: Even/odd frame
Applies to isochronous IN and OUT endpoints only.
Indicates the frame number in which the core transmits/receives isochronous data for this
endpoint. The application must program the even/odd frame number in which it intends to
transmit/receive isochronous data for this endpoint using the SEVNFRM and SODDFRM
fields in this register.
0: Even frame
1: Odd frame
DPID: Endpoint data PID
Applies to interrupt/bulk OUT endpoints only.
Contains the PID of the packet to be received or transmitted on this endpoint. The
application must program the PID of the first packet to be received or transmitted on this
endpoint, after the endpoint is activated. The application uses the SD0PID register field to
program either DATA0 or DATA1 PID.
0: DATA0
1: DATA1

RM0456 Rev 6

<!-- pagebreak -->

3291

USB on-the-go full-speed (OTG_FS)

RM0456

Bit 15 USBAEP: USB active endpoint
Indicates whether this endpoint is active in the current configuration and interface. The core
clears this bit for all endpoints (other than EP 0) after detecting a USB reset. After receiving
the SetConfiguration and SetInterface commands, the application must program endpoint
registers accordingly and set this bit.
Bits 14:11 Reserved, must be kept at reset value.
Bits 10:0 MPSIZ[10:0]: Maximum packet size
The application must program this field with the maximum packet size for the current logical
endpoint. This value is in bytes.

72.15.53 OTG device OUT endpoint x transfer size register
(OTG_DOEPTSIZx)
Address offset: 0xB10 + 0x20 * x, (x = 1 to 5)
Reset value: 0x0000 0000
The application must modify this register before enabling the endpoint. Once the endpoint is
enabled using endpoint enable bit of the OTG_DOEPCTLx registers (EPENA bit in
OTG_DOEPCTLx), the core modifies this register. The application can only read this
register once the core has cleared the endpoint enable bit.
31
Res.

15

30

29

28

27

26

25

RXDPID/
STUPCNT[1:0]

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

Bit 31 Reserved, must be kept at reset value.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

USB on-the-go full-speed (OTG_FS)

Bits 30:29 RXDPID[1:0]: Received data PID
Applies to isochronous OUT endpoints only.
This is the data PID received in the last packet for this endpoint.
00: DATA0
10: DATA1
STUPCNT[1:0]: SETUP packet count
Applies to control OUT endpoints only.
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

72.15.54 OTG power and clock gating control register (OTG_PCGCCTL)
Address offset: 0xE00
Reset value: 0x200B 8000
This register is available in host and device modes.
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

RM0456 Rev 6

<!-- pagebreak -->

3291

USB on-the-go full-speed (OTG_FS)

RM0456

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

72.15.55 OTG_FS register map
The table below gives the USB OTG register map and reset values.

SRQ

SRQSCS
0

Res.

Res.

Res.

Res.

Res.

Res.

SEDET

0

0

0

0

<!-- pagebreak -->

