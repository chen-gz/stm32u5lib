RM0456 Rev 6

RM0456

USB on-the-go full-speed (OTG_FS)

Data FIFO (DFIFO) access register map
These registers, available in both host and device modes, are used to read or write the FIFO
space for a specific endpoint or a channel, in a given direction. If a host channel is of type
IN, the FIFO can only be read on the channel. Similarly, if a host channel is of type OUT, the
FIFO can only be written on the channel.
Table 756. Data FIFO (DFIFO) access register map
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

1. Where x is 5in device mode and 11 in host mode.

Power and clock gating CSR map
There is a single register for power and clock gating. It is available in both host and device
modes.
Table 757. Power and clock gating control and status registers
Acronym
OTG_PCGCCTL

72.15

Offset address
0xE00–0xE04

Register name
Section 72.15.54: OTG power and clock gating control
register (OTG_PCGCCTL)

OTG_FS registers
These registers are available in both host and device modes, and do not need to be
reprogrammed when switching between these modes.
Bit values in the register descriptions are expressed in binary unless otherwise specified.

RM0456 Rev 6

<!-- pagebreak -->

3291

USB on-the-go full-speed (OTG_FS)

72.15.1

RM0456

OTG control and status register (OTG_GOTGCTL)
Address offset: 0x000
Reset value: 0x0001 0000
The OTG_GOTGCTL register controls the behavior and reflects the status of the OTG
function of the core.

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
OTG
VER

19

18

17

16

DBCT

CID
STS

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

CUR
MOD
r

rw

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

Res.

Res.

Res.

EHEN

HNP
RQ

HNG
SCS

SRQ

SRQ
SCS

rw

r

rw

r

rw

DHNP HSHNP
EN
EN
rw

rw

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
0: The OTG_FS controller is in A-device mode
1: The OTG_FS controller is in B-device mode
Note: Accessible in both device and host modes.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

USB on-the-go full-speed (OTG_FS)

Bits 15:13 Reserved, must be kept at reset value.
Bit 12 EHEN: Embedded host enable
It is used to select between OTG A device state machine and embedded host state machine.
0: OTG A device state machine is selected
1: Embedded host state machine is selected
Bit 11 DHNPEN: Device HNP enabled
The application sets this bit when it successfully receives a SetFeature.SetHNPEnable
command from the connected USB host.
0: HNP is not enabled in the application
1: HNP is enabled in the application
Note: Only accessible in device mode.
Bit 10 HSHNPEN: host set HNP enable
The application sets this bit when it has successfully enabled HNP (using the
SetFeature.SetHNPEnable command) on the connected device.
0: Host Set HNP is not enabled
1: Host Set HNP is enabled
Note: Only accessible in host mode.
Bit 9 HNPRQ: HNP request
The application sets this bit to initiate an HNP request to the connected USB host. The
application can clear this bit by writing a 0 when the host negotiation success status change
bit in the OTG_GOTGINT register (HNSSCHG bit in OTG_GOTGINT) is set. The core clears
this bit when the HNSSCHG bit is cleared.
0: No HNP request
1: HNP request
Note: Only accessible in device mode.
Bit 8 HNGSCS: Host negotiation success
The core sets this bit when host negotiation is successful. The core clears this bit when the
HNP request (HNPRQ) bit in this register is set.
0: Host negotiation failure
1: Host negotiation success
Note: Only accessible in device mode.
Bit 7 BVALOVAL: B-peripheral session valid override value.
This bit is used to set override value for Bvalid signal when BVALOEN bit is set.
0: Bvalid value is '0' when BVALOEN = 1
1: Bvalid value is '1' when BVALOEN = 1
Note: Only accessible in device mode.
Bit 6 BVALOEN: B-peripheral session valid override enable.
This bit is used to enable/disable the software to override the Bvalid signal using the
BVALOVAL bit.
0:Override is disabled and Bvalid signal from the respective PHY selected is used internally
by the core
1:Internally Bvalid received from the PHY is overridden with BVALOVAL bit value
Note: Only accessible in device mode.
Bit 5 AVALOVAL: A-peripheral session valid override value.
This bit is used to set override value for Avalid signal when AVALOEN bit is set.
0: Avalid value is '0' when AVALOEN = 1
1: Avalid value is '1' when AVALOEN = 1
Note: Only accessible in host mode.

RM0456 Rev 6

<!-- pagebreak -->

3291

USB on-the-go full-speed (OTG_FS)

RM0456

Bit 4 AVALOEN: A-peripheral session valid override enable.
This bit is used to enable/disable the software to override the Avalid signal using the
AVALOVAL bit.
0:Override is disabled and Avalid signal from the respective PHY selected is used internally
by the core
1:Internally Avalid received from the PHY is overridden with AVALOVAL bit value
Note: Only accessible in host mode.
Bit 3 VBVALOVAL: VBUS valid override value.
This bit is used to set override value for vbusvalid signal when VBVALOEN bit is set.
0: vbusvalid value is '0' when VBVALOEN = 1
1: vbusvalid value is '1' when VBVALOEN = 1
Note: Only accessible in host mode.
Bit 2 VBVALOEN: VBUS valid override enable.
This bit is used to enable/disable the software to override the vbusvalid signal using the
VBVALOVAL bit.
0: Override is disabled and vbusvalid signal from the respective PHY selected is used
internally by the core
1: Internally vbusvalid received from the PHY is overridden with VBVALOVAL bit value
Note: Only accessible in host mode.
Bit 1 SRQ: Session request
The application sets this bit to initiate a session request on the USB. The application can
clear this bit by writing a 0 when the host negotiation success status change bit in the
OTG_GOTGINT register (HNSSCHG bit in OTG_GOTGINT) is set. The core clears this bit
when the HNSSCHG bit is cleared.
If the user uses the USB 1.1 full-speed serial transceiver interface to initiate the session
request, the application must wait until VBUS discharges to 0.2 V, after the B-session valid bit
in this register (BSVLD bit in OTG_GOTGCTL) is cleared.
0: No session request
1: Session request
Note: Only accessible in device mode.
Bit 0 SRQSCS: Session request success
The core sets this bit when a session request initiation is successful.
0: Session request failure
1: Session request success
Note: Only accessible in device mode.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

USB on-the-go full-speed (OTG_FS)

72.15.2

OTG interrupt register (OTG_GOTGINT)
Address offset: 0x04
Reset value: 0x0000 0000
The application reads this register whenever there is an OTG interrupt and clears the bits in
this register to clear the OTG interrupt.

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

DBC
DNE

ADTO
CHG

HNG
DET

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

rc_w1

rc_w1

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

HNSS
CHG

SRSS
CHG

Res.

Res.

Res.

Res.

Res.

SEDET

Res.

Res.

rc_w1

rc_w1

rc_w1

Bits 31:20 Reserved, must be kept at reset value.
Bit 19 DBCDNE: Debounce done
The core sets this bit when the debounce is completed after the device connect. The
application can start driving USB reset after seeing this interrupt. This bit is only valid when
the HNP Capable or SRP Capable bit is set in the OTG_GUSBCFG register (HNPCAP bit or
SRPCAP bit in OTG_GUSBCFG, respectively).
Note: Only accessible in host mode.
Bit 18 ADTOCHG: A-device timeout change
The core sets this bit to indicate that the A-device has timed out while waiting for the B-device
to connect.
Note: Accessible in both device and host modes.
Bit 17 HNGDET: Host negotiation detected
The core sets this bit when it detects a host negotiation request on the USB.
Note: Accessible in both device and host modes.
Bits 16:10 Reserved, must be kept at reset value.
Bit 9 HNSSCHG: Host negotiation success status change
The core sets this bit on the success or failure of a USB host negotiation request. The
application must read the host negotiation success bit of the OTG_GOTGCTL register
(HNGSCS bit in OTG_GOTGCTL) to check for success or failure.
Note: Accessible in both device and host modes.
Bits 7:3 Reserved, must be kept at reset value.
Bit 8 SRSSCHG: Session request success status change
The core sets this bit on the success or failure of a session request. The application must
read the session request success bit in the OTG_GOTGCTL register (SRQSCS bit in
OTG_GOTGCTL) to check for success or failure.
Note: Accessible in both device and host modes.
Bit 2 SEDET: Session end detected
The core sets this bit to indicate that the level of the voltage on VBUS is no longer valid for a
B-Peripheral session when VBUS < 0.8 V.
Note: Accessible in both device and host modes.
Bits 1:0 Reserved, must be kept at reset value.

RM0456 Rev 6

<!-- pagebreak -->

3291

USB on-the-go full-speed (OTG_FS)

72.15.3

RM0456

OTG AHB configuration register (OTG_GAHBCFG)
Address offset: 0x008
Reset value: 0x0000 0000
This register can be used to configure the core after power-on or a change in mode. This
register mainly contains AHB system-related configuration parameters. Do not change this
register after the initial programming. The application must program this register before
starting any transactions on either the AHB or the USB.

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

PTXFE
LVL

TXFE
LVL

Res.

GINT
MSK

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

Res.

Res.

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
In device mode, this bit indicates when IN endpoint Transmit FIFO empty interrupt (TXFE in
OTG_DIEPINTx) is triggered:
0:The TXFE (in OTG_DIEPINTx) interrupt indicates that the IN endpoint Tx FIFO is half
empty
1:The TXFE (in OTG_DIEPINTx) interrupt indicates that the IN endpoint Tx FIFO is
completely empty
In host mode, this bit indicates when the nonperiodic Tx FIFO empty interrupt (NPTXFE bit in
OTG_GINTSTS) is triggered:
0:The NPTXFE (in OTG_GINTSTS) interrupt indicates that the nonperiodic Tx FIFO is half
empty
1:The NPTXFE (in OTG_GINTSTS) interrupt indicates that the nonperiodic Tx FIFO is
completely empty
Bits 6:1 Reserved, must be kept at reset value.
Bit 0 GINTMSK: Global interrupt mask
The application uses this bit to mask or unmask the interrupt line assertion to itself.
Irrespective of this bit’s setting, the interrupt status registers are updated by the core.
0: Mask the interrupt assertion to the application.
1: Unmask the interrupt assertion to the application.
Note: Accessible in both device and host modes.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

USB on-the-go full-speed (OTG_FS)

72.15.4

OTG USB configuration register (OTG_GUSBCFG)
Address offset: 0x00C
Reset value: 0x0000 1440
This register can be used to configure the core after power-on or a changing to host mode
or device mode. It contains USB and USB-PHY related configuration parameters. The
application must program this register before starting any transactions on either the AHB or
the USB. Do not make changes to this register after the initial programming.

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

Res.

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

Res.

Res.

HNP
CAP

SRP
CAP

Res.

PHY
SEL

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

r

TOCAL[2:0]
rw

rw

rw

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
Bit 22 Reserved, must be kept at reset value.
Bits 21:16 Reserved, must be kept at reset value.
Bit 15 Reserved, must be kept at reset value.
Bit 14 Reserved, must be kept at reset value.
Bits 13:10 TRDT[3:0]: USB turnaround time
These bits are used to set the turnaround time in PHY clocks. They must be configured
according to Table 758: TRDT values, depending on the application AHB frequency. Higher
TRDT values allow stretching the USB response time to IN tokens in order to compensate for
longer AHB read access latency to the data FIFO.
Note: Only accessible in device mode.

RM0456 Rev 6

<!-- pagebreak -->

3291

USB on-the-go full-speed (OTG_FS)

RM0456

Bit 9 HNPCAP: HNP-capable
The application uses this bit to control the OTG_FS controller’s HNP capabilities.
0: HNP capability is not enabled.
1: HNP capability is enabled.
Note: Accessible in both device and host modes.
Bit 8 SRPCAP: SRP-capable
The application uses this bit to control the OTG_FS controller’s SRP capabilities. If the core
operates as a non-SRP-capable
B-device, it cannot request the connected A-device (host) to activate VBUS and start a
session.
0: SRP capability is not enabled.
1: SRP capability is enabled.
Note: Accessible in both device and host modes.
Bit 7 Reserved, must be kept at reset value.
Bit 6 PHYSEL: Full Speed serial transceiver mode select
This bit is always 1 with read-only access.
Bit 5 Reserved, must be kept at reset value.
Bit 4 Reserved, must be kept at reset value.
Bit 3 Reserved, must be kept at reset value.
Bits 2:0 TOCAL[2:0]: FS timeout calibration
The number of PHY clocks that the application programs in this field is added to the fullspeed interpacket timeout duration in the core to account for any additional delays
introduced by the PHY. This can be required, because the delay introduced by the PHY in
generating the line state condition can vary from one PHY to another.
The USB standard timeout value for full-speed operation is 16 to 18 (inclusive) bit times. The
application must program this field based on the speed of enumeration. The number of bit
times added per PHY clock is 0.25 bit times.

Table 758. TRDT values
AHB frequency range (MHz)
TRDT minimum value

<!-- pagebreak -->

Min

Max

14.2

15

0xF

15

16

0xE

16

17.2

0xD

17.2

18.5

0xC

18.5

20

0xB

20

21.8

0xA

21.8

24

0x9

24

27.5

0x8

27.5

32

0x7

32

-

0x6

RM0456 Rev 6

RM0456

USB on-the-go full-speed (OTG_FS)

72.15.5

OTG reset register (OTG_GRSTCTL)
Address offset: 0x10
Reset value: 0x8000 0000
The application uses this register to reset various hardware features inside the core.

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

14

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
15
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
Bits 30:11 Reserved, must be kept at reset value.
Bits 10:6 TXFNUM[4:0]: Tx FIFO number
This is the FIFO number that must be flushed using the Tx FIFO Flush bit. This field must not
be changed until the core clears the Tx FIFO Flush bit.
00000:
–
Non-periodic Tx FIFO flush in host mode
–
Tx FIFO 0 flush in device mode
00001:
–
Periodic Tx FIFO flush in host mode
–
Tx FIFO 1 flush in device mode
00010: Tx FIFO 2 flush in device mode
...
01111: Tx FIFO 15 flush in device mode
10000: Flush all the transmit FIFOs in device or host mode.
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

RM0456 Rev 6

<!-- pagebreak -->

3291

USB on-the-go full-speed (OTG_FS)

RM0456

Bit 4 RXFFLSH: Rx FIFO flush
The application can flush the entire Rx FIFO using this bit, but must first ensure that the core
is not in the middle of a transaction.
The application must only write to this bit after checking that the core is neither reading from
the Rx FIFO nor writing to the Rx FIFO.
The application must wait until the bit is cleared before performing any other operations. This
bit requires 8 clocks (slowest of PHY or AHB clock) to clear.
Note: Accessible in both device and host modes.
Bit 3 Reserved, must be kept at reset value.
Bit 2 FCRST: Host frame counter reset
The application writes this bit to reset the frame number counter inside the core. When the
frame counter is reset, the subsequent SOF sent out by the core has a frame number of 0.
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

<!-- pagebreak -->

RM0456 Rev 6

RM0456

USB on-the-go full-speed (OTG_FS)

72.15.6

OTG core interrupt register (OTG_GINTSTS)
Address offset: 0x014
Reset value: 0x0400 0020
This register interrupts the application for system-level events in the current mode (device
mode or host mode).
Some of the bits in this register are valid only in host mode, while others are valid in device
mode only. This register also indicates the current mode. To clear the interrupt status bits of
the rc_w1 type, the application must write 1 into the bit.
The FIFO status interrupts are read-only; once software reads from or writes to the FIFO
while servicing these interrupts, FIFO interrupt conditions are cleared automatically.
The application must clear the OTG_GINTSTS register at initialization before unmasking
the interrupt bit to avoid any interrupts generated prior to initialization.

31

30

29

28

27

26

25

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

IPXFR/
IN
COMP
ISO
OUT

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

r

r

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

Bit 31 WKUPINT: Resume/remote wake-up detected interrupt
Wake-up interrupt during suspend(L2) or LPM(L1) state.
– During suspend(L2):
In device mode, this interrupt is asserted when a resume is detected on the USB. In host
mode, this interrupt is asserted when a remote wake-up is detected on the USB.
– During LPM(L1):
This interrupt is asserted for either host initiated resume or device initiated remote wakeup on USB.
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

RM0456 Rev 6

<!-- pagebreak -->

3291

USB on-the-go full-speed (OTG_FS)

RM0456

Bit 27 LPMINT: LPM interrupt
In device mode, this interrupt is asserted when the device receives an LPM transaction and
responds with a non-ERRORed response.
In host mode, this interrupt is asserted when the device responds to an LPM transaction with
a non-ERRORed response or when the host core has completed LPM transactions for the
programmed number of times (RETRYCNT bit in OTG_GLPMCFG).
This field is valid only if the LPMEN bit in OTG_GLPMCFG is set to 1.
Bit 26 PTXFE: Periodic Tx FIFO empty
Asserted when the periodic transmit FIFO is either half or completely empty and there is
space for at least one entry to be written in the periodic request queue. The half or
completely empty status is determined by the periodic Tx FIFO empty level bit in the
OTG_GAHBCFG register (PTXFELVL bit in OTG_GAHBCFG).
Note: Only accessible in host mode.
Bit 25 HCINT: Host channels interrupt
The core sets this bit to indicate that an interrupt is pending on one of the channels of the
core (in host mode). The application must read the OTG_HAINT register to determine the
exact number of the channel on which the interrupt occurred, and then read the
corresponding OTG_HCINTx register to determine the exact cause of the interrupt. The
application must clear the appropriate status bit in the OTG_HCINTx register to clear this bit.
Note: Only accessible in host mode.
Bit 24 HPRTINT: Host port interrupt
The core sets this bit to indicate a change in port status of one of the OTG_FS controller
ports in host mode. The application must read the OTG_HPRT register to determine the
exact event that caused this interrupt. The application must clear the appropriate status bit in
the OTG_HPRT register to clear this bit.
Note: Only accessible in host mode.
Bit 23 RSTDET: Reset detected interrupt
In device mode, this interrupt is asserted when a reset is detected on the USB in partial
power-down mode when the device is in suspend.
Note: Only accessible in device mode.
Bit 22 Reserved, must be kept at reset value.
Bit 21 IPXFR: Incomplete periodic transfer
In host mode, the core sets this interrupt bit when there are incomplete periodic transactions
still pending, which are scheduled for the current frame.
INCOMPISOOUT: Incomplete isochronous OUT transfer
In device mode, the core sets this interrupt to indicate that there is at least one isochronous
OUT endpoint on which the transfer is not completed in the current frame. This interrupt is
asserted along with the End of periodic frame interrupt (EOPF) bit in this register.
Bit 20 IISOIXFR: Incomplete isochronous IN transfer
The core sets this interrupt to indicate that there is at least one isochronous IN endpoint on
which the transfer is not completed in the current frame. This interrupt is asserted along with
the End of periodic frame interrupt (EOPF) bit in this register.
Note: Only accessible in device mode.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

USB on-the-go full-speed (OTG_FS)

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
because the Rx FIFO does not have enough space to accommodate a maximum size
packet for the isochronous OUT endpoint.
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

RM0456 Rev 6

<!-- pagebreak -->

3291

USB on-the-go full-speed (OTG_FS)

RM0456

Bit 6 GINAKEFF: Global IN non-periodic NAK effective
Indicates that the Set global non-periodic IN NAK bit in the OTG_DCTL register (SGINAK bit
in OTG_DCTL), set by the application, has taken effect in the core. That is, the core has
sampled the Global IN NAK bit set by the application. This bit can be cleared by clearing the
Clear global non-periodic IN NAK bit in the OTG_DCTL register (CGINAK bit in
OTG_DCTL).
This interrupt does not necessarily mean that a NAK handshake is sent out on the USB. The
STALL bit takes precedence over the NAK bit.
Note: Only accessible in device mode.
Bit 5 NPTXFE: Non-periodic Tx FIFO empty
This interrupt is asserted when the non-periodic Tx FIFO is either half or completely empty,
and there is space for at least one entry to be written to the non-periodic transmit request
queue. The half or completely empty status is determined by the non-periodic Tx FIFO
empty level bit in the OTG_GAHBCFG register (TXFELVL bit in OTG_GAHBCFG).
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
read value of this interrupt is valid only after a valid connection between host and
device is established. If the bit is set after power on reset the application can clear the
bit.
Note: Accessible in both host and device modes.
Bit 2 OTGINT: OTG interrupt
The core sets this bit to indicate an OTG protocol event. The application must read the OTG
interrupt status (OTG_GOTGINT) register to determine the exact event that caused this
interrupt. The application must clear the appropriate status bit in the OTG_GOTGINT
register to clear this bit.
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

USB on-the-go full-speed (OTG_FS)

72.15.7

OTG interrupt mask register (OTG_GINTMSK)
Address offset: 0x018
Reset value: 0x0000 0000
This register works with the core interrupt register to interrupt the application. When an
interrupt bit is masked, the interrupt associated with that bit is not generated. However, the
core interrupt (OTG_GINTSTS) register bit corresponding to that interrupt is still set.

31
WUIM

30
SRQIM

29

28

DISCIN CIDSC
T
HGM

27

26

LPMIN PTXFE
TM
M

25

24

23

HCIM

PRTIM

RSTDE
TM

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

EOPF
M

ISOOD
RPM

Res.

Res.

rw

rw

ENUM USBRS USBSU ESUSP
DNEM
T
SPM
M
rw

rw

rw

rw

22

21

20

Res.

IPXFR
M/IISO
OXFR
M

IISOIX
FRM

rw

rw

rw

rw

5

4

3

2

6

GONA GINAK NPTXF RXFLV
KEFFM EFFM
EM
LM
rw

rw

rw

rw

19

18

OEPIN
IEPINT
T

SOFM
rw

17

16

Res.

Res.

1

0

OTGIN
MMISM
T
rw

Res.

rw

Bit 31 WUIM: Resume/remote wake-up detected interrupt mask
0: Masked interrupt
1: Unmasked interrupt
Note: Accessible in both host and device modes.
Bit 30 SRQIM: Session request/new session detected interrupt mask
0: Masked interrupt
1: Unmasked interrupt
Note: Accessible in both host and device modes.
Bit 29 DISCINT: Disconnect detected interrupt mask
0: Masked interrupt
1: Unmasked interrupt
Note: Only accessible in host mode.
Bit 28 CIDSCHGM: Connector ID status change mask
0: Masked interrupt
1: Unmasked interrupt
Note: Accessible in both host and device modes.
Bit 27 LPMINTM: LPM interrupt mask
0: Masked interrupt
1: Unmasked interrupt
Note: Accessible in both host and device modes.
Bit 26 PTXFEM: Periodic Tx FIFO empty mask
0: Masked interrupt
1: Unmasked interrupt
Note: Only accessible in host mode.
Bit 25 HCIM: Host channels interrupt mask
0: Masked interrupt
1: Unmasked interrupt
Note: Only accessible in host mode.

RM0456 Rev 6

<!-- pagebreak -->

3291

USB on-the-go full-speed (OTG_FS)

RM0456

Bit 24 PRTIM: Host port interrupt mask
0: Masked interrupt
1: Unmasked interrupt
Note: Only accessible in host mode.
Bit 23 RSTDETM: Reset detected interrupt mask
0: Masked interrupt
1: Unmasked interrupt
Note: Only accessible in device mode.
Bit 22 Reserved, must be kept at reset value.
Bit 21 IPXFRM: Incomplete periodic transfer mask
0: Masked interrupt
1: Unmasked interrupt
Note: Only accessible in host mode.
IISOOXFRM: Incomplete isochronous OUT transfer mask
0: Masked interrupt
1: Unmasked interrupt
Note: Only accessible in device mode.
Bit 20 IISOIXFRM: Incomplete isochronous IN transfer mask
0: Masked interrupt
1: Unmasked interrupt
Note: Only accessible in device mode.
Bit 19 OEPINT: OUT endpoints interrupt mask
0: Masked interrupt
1: Unmasked interrupt
Note: Only accessible in device mode.
Bit 18 IEPINT: IN endpoints interrupt mask
0: Masked interrupt
1: Unmasked interrupt
Note: Only accessible in device mode.
Bits 17:16 Reserved, must be kept at reset value.
Bit 15 EOPFM: End of periodic frame interrupt mask
0: Masked interrupt
1: Unmasked interrupt
Note: Only accessible in device mode.
Bit 14 ISOODRPM: Isochronous OUT packet dropped interrupt mask
0: Masked interrupt
1: Unmasked interrupt
Note: Only accessible in device mode.
Bit 13 ENUMDNEM: Enumeration done mask
0: Masked interrupt
1: Unmasked interrupt
Note: Only accessible in device mode.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

USB on-the-go full-speed (OTG_FS)

Bit 12 USBRST: USB reset mask
0: Masked interrupt
1: Unmasked interrupt
Note: Only accessible in device mode.
Bit 11 USBSUSPM: USB suspend mask
0: Masked interrupt
1: Unmasked interrupt
Note: Only accessible in device mode.
Bit 10 ESUSPM: Early suspend mask
0: Masked interrupt
1: Unmasked interrupt
Note: Only accessible in device mode.
Bits 9:8 Reserved, must be kept at reset value.
Bit 7 GONAKEFFM: Global OUT NAK effective mask
0: Masked interrupt
1: Unmasked interrupt
Note: Only accessible in device mode.
Bit 6 GINAKEFFM: Global non-periodic IN NAK effective mask
0: Masked interrupt
1: Unmasked interrupt
Note: Only accessible in device mode.
Bit 5 NPTXFEM: Non-periodic Tx FIFO empty mask
0: Masked interrupt
1: Unmasked interrupt
Note: Only accessible in host mode.
Bit 4 RXFLVLM: Receive FIFO non-empty mask
0: Masked interrupt
1: Unmasked interrupt
Note: Accessible in both device and host modes.
Bit 3 SOFM: Start of frame mask
0: Masked interrupt
1: Unmasked interrupt
Note: Accessible in both device and host modes.
Bit 2 OTGINT: OTG interrupt mask
0: Masked interrupt
1: Unmasked interrupt
Note: Accessible in both device and host modes.
Bit 1 MMISM: Mode mismatch interrupt mask
0: Masked interrupt
1: Unmasked interrupt
Note: Accessible in both device and host modes.
Bit 0 Reserved, must be kept at reset value.

RM0456 Rev 6

<!-- pagebreak -->

3291

USB on-the-go full-speed (OTG_FS)

72.15.8

RM0456

OTG receive status debug read register (OTG_GRXSTSR)
Address offset for read: 0x01C
Reset value: 0x0000 0000
This description is for register OTG_GRXSTSR in Device mode.
A read to the receive status debug read register returns the contents of the top of the
receive FIFO.
The core ignores the receive status read when the receive FIFO is empty and returns a
value of 0x0000 0000.

31

30

29

28

27

26

25

Res.

Res.

10

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
Bits 14:4 BCNT[10:0]: Byte count
Indicates the byte count of the received data packet.
Bits 3:0 EPNUM[3:0]: Endpoint number
Indicates the endpoint number to which the current received packet belongs.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

USB on-the-go full-speed (OTG_FS)

72.15.9

OTG receive status debug read [alternate] (OTG_GRXSTSR)
Address offset for read: 0x01C
Reset value: 0x0000 0000
This description is for register OTG_GRXSTSR in Host mode.
A read to the receive status debug read register returns the contents of the top of the
receive FIFO.
The core ignores the receive status read when the receive FIFO is empty and returns a
value of 0x0000 0000.

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

r

r

r

r

r

r

r

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
Bits 14:4 BCNT[10:0]: Byte count
Indicates the byte count of the received IN data packet.
Bits 3:0 CHNUM[3:0]: Channel number
Indicates the channel number to which the current received packet belongs.

RM0456 Rev 6

<!-- pagebreak -->

3291

USB on-the-go full-speed (OTG_FS)

RM0456

72.15.10 OTG status read and pop registers (OTG_GRXSTSP)
Address offset for pop: 0x020
Reset value: 0x0000 0000
This description is for register OTG_GRXSTSP in Device mode.
Similarly to OTG_GRXSTSR (receive status debug read register) where a read returns the
contents of the top of the receive FIFO, a read to OTG_GRXSTSP (receive status read and
pop register) additionally pops the top data entry out of the Rx FIFO.
The core ignores the receive status pop/read when the receive FIFO is empty and returns a
value of 0x0000 0000. The application must only pop the receive status FIFO when the
receive FIFO non-empty bit of the core interrupt register (RXFLVL bit in OTG_GINTSTS) is
asserted.
31

30

29

28

27

26

25

Res.

Res.

10

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
Bits 14:4 BCNT[10:0]: Byte count
Indicates the byte count of the received data packet.
Bits 3:0 EPNUM[3:0]: Endpoint number
Indicates the endpoint number to which the current received packet belongs.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

USB on-the-go full-speed (OTG_FS)

72.15.11 OTG status read and pop registers [alternate] (OTG_GRXSTSP)
Address offset for pop: 0x020
Reset value: 0x0000 0000
This description is for register OTG_GRXSTSP in Host mode.
Similarly to OTG_GRXSTSR (receive status debug read register) where a read returns the
contents of the top of the receive FIFO, a read to OTG_GRXSTSP (receive status read and
pop register) additionally pops the top data entry out of the Rx FIFO.
The core ignores the receive status pop/read when the receive FIFO is empty and returns a
value of 0x0000 0000. The application must only pop the receive status FIFO when the
receive FIFO non-empty bit of the core interrupt register (RXFLVL bit in OTG_GINTSTS) is
asserted.
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

r

r

r

r

r

r

r

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
Bits 14:4 BCNT[10:0]: Byte count
Indicates the byte count of the received IN data packet.
Bits 3:0 CHNUM[3:0]: Channel number
Indicates the channel number to which the current received packet belongs.

RM0456 Rev 6

<!-- pagebreak -->

3291

USB on-the-go full-speed (OTG_FS)

RM0456

72.15.12 OTG receive FIFO size register (OTG_GRXFSIZ)
Address offset: 0x024
Reset value: 0x0000 0200
The application can program the RAM size that must be allocated to the Rx FIFO.
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
Minimum value is 16
Programmed values must respect the available FIFO memory allocation and must not
exceed the power-on value.

72.15.13 OTG host non-periodic transmit FIFO size register
(OTG_HNPTXFSIZ)/Endpoint 0 Transmit FIFO size
(OTG_DIEPTXF0)
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

NPTXFD/TX0FD[15:0]
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

NPTXFSA/TX0FSA[15:0]
rw

rw

rw

rw

rw

rw

rw

rw

rw

Host mode
Bits 31:16 NPTXFD[15:0]: Non-periodic Tx FIFO depth
This value is in terms of 32-bit words.
Minimum value is 16
Programmed values must respect the available FIFO memory allocation and must not
exceed the power-on value.
Bits 15:0 NPTXFSA[15:0]: Non-periodic transmit RAM start address
This field configures the memory start address for non-periodic transmit FIFO RAM.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

USB on-the-go full-speed (OTG_FS)

Device mode
Bits 31:16 TX0FD: Endpoint 0 Tx FIFO depth
This value is in terms of 32-bit words.
Minimum value is 16
Programmed values must respect the available FIFO memory allocation and must not
exceed the power-on value.
Bits 15:0 TX0FSA: Endpoint 0 transmit RAM start address
This field configures the memory start address for the endpoint 0 transmit FIFO RAM.

72.15.14 OTG non-periodic transmit FIFO/queue status register
(OTG_HNPTXSTS)
Address offset: 0x02C
Reset value: 0x0008 0200
Note:

In device mode, this register is not valid.
This read-only register contains the free space information for the non-periodic Tx FIFO and
the non-periodic transmit request queue.

31

30

29

28

Res.

15

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

NPTXFSAV[15:0]
r

r

r

r

r

r

r

r

r

RM0456 Rev 6

<!-- pagebreak -->

3291

USB on-the-go full-speed (OTG_FS)

RM0456

Bit 31 Reserved, must be kept at reset value.
Bits 30:24 NPTXQTOP[6:0]: Top of the non-periodic transmit request queue
Entry in the non-periodic Tx request queue that is currently being processed by the MAC.
Bits 30:27: Channel/endpoint number
Bits 26:25:
00: IN/OUT token
01: Zero-length transmit packet (device IN/host OUT)
11: Channel halt command
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

72.15.15 OTG general core configuration register (OTG_GCCFG)
Address offset: 0x038
Reset value: 0x0000 XXXX
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

<!-- pagebreak -->

RM0456 Rev 6

21

20

19

18

17

16

BCDEN

PWR
DWN

VBDEN

SDEN

PDEN

DCD
EN

rw

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

Res.

Res.

PS2
DET

SDET

PDET

DCDET

r

r

r

r

RM0456

USB on-the-go full-speed (OTG_FS)

Bits 31:22 Reserved, must be kept at reset value.
Bit 21 VBDEN: USB VBUS detection enable
Enables VBUS sensing comparators to detect VBUS valid levels on the VBUS PAD for USB
host and device operation. If HNP and/or SRP support is enabled, VBUS comparators are
automatically enabled independently of VBDEN value.
0 = VBUS detection disabled
1 = VBUS detection enabled
Bit 20 SDEN: Secondary detection (SD) mode enable
This bit is set by the software to put the BCD into SD mode. Only one detection mode (DCD,
PD, SD or OFF) must be selected to work correctly
Bit 19 PDEN: Primary detection (PD) mode enable
This bit is set by the software to put the BCD into PD mode. Only one detection mode (DCD,
PD, SD or OFF) must be selected to work correctly.
Bit 18 DCDEN: Data contact detection (DCD) mode enable
This bit is set by the software to put the BCD into DCD mode. Only one detection mode
(DCD, PD, SD or OFF) must be selected to work correctly.
Bit 17 BCDEN: Battery charging detector (BCD) enable
This bit is set by the software to enable the BCD support within the USB device. When
enabled, the USB PHY is fully controlled by BCD and cannot be used for normal
communication. Once the BCD discovery is finished, the BCD must be placed in OFF mode
by clearing this bit to ‘0’ in order to allow the normal USB operation.
Bit 16 PWRDWN: Power down control of FS PHY
Used to activate the FS PHY in transmission/reception. When reset, the PHY is kept in
power-down. When set, the BCD function must be off (BCDEN=0).
0 = USB FS PHY disabled
1 = USB FS PHY enabled
Bits 15:4 Reserved, must be kept at reset value.
Bit 3 PS2DET: DM pull-up detection status
This bit is active only during PD and gives the result of comparison between DM voltage
level and VLGC threshold. In normal situation, the DM level must be below this threshold. If
it is above, it means that the DM is externally pulled high. This can be caused by connection
to a PS2 port (which pulls-up both DP and DM lines) or to some proprietary charger not
following the BCD specification.
0: Normal port detected (connected to SDP, CDP or DCP)
1: PS2 port or proprietary charger detected
Bit 2 SDET: Secondary detection (SD) status
This bit gives the result of SD.
0: CDP detected
1: DCP detected
Bit 1 PDET: Primary detection (PD) status
This bit gives the result of PD.
0: no BCD support detected (connected to SDP or proprietary device).
1: BCD support detected (connected to CDP or DCP).
Bit 0 DCDET: Data contact detection (DCD) status
This bit gives the result of DCD.
0: data lines contact not detected
1: data lines contact detected

RM0456 Rev 6

<!-- pagebreak -->

3291

USB on-the-go full-speed (OTG_FS)

RM0456

72.15.16 OTG core ID register (OTG_CID)
Address offset: 0x03C
Reset value: 0x0000 3000
This is a register containing the Product ID as reset value.
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

18

17

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

72.15.17 OTG core LPM configuration register (OTG_GLPMCFG)
Address offset: 0x54
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

16

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

L1RSM
OK

LPMCHIDX[3:0]

BESL[3:0]
rw

rw

rw

rw

Bits 31:29 Reserved, must be kept at reset value.
Bit 28 ENBESL: Enable best effort service latency
This bit enables the BESL feature as defined in the LPM errata:
0:The core works as described in the following document:
USB 2.0 Link Power Management Addendum Engineering Change Notice to the USB 2.0
specification, July 16, 2007
1:The core works as described in the LPM Errata:
Errata for USB 2.0 ECN: Link Power Management (LPM) - 7/2007
Note: Only the updated behavior (described in LPM Errata) is considered in this document
and so the ENBESL bit must be set to '1' by application SW.
Bits 27:25 LPMRCNTSTS[2:0]: LPM retry count status
Number of LPM host retries still remaining to be transmitted for the current LPM sequence.
Note: Accessible only in host mode.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

USB on-the-go full-speed (OTG_FS)

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
– When there is any activity on the USB line state
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

RM0456 Rev 6

<!-- pagebreak -->

