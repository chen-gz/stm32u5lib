3291

USB on-the-go full-speed (OTG_FS)

72.4.5

RM0456

OTG detections
Additionally the OTG_FS uses the following functions:

72.5

•

integrated ID pull-up resistor used to sample the ID line for A/B device identification.

•

VBUS sensing comparators with hysteresis used to detect VBUS valid, A-B session valid
and session-end voltage thresholds. They are used to drive the session request
protocol (SRP), detect valid startup and end-of-session conditions, and constantly
monitor the VBUS supply during USB operations.

OTG_FS dual role device (DRD)
Figure 894. OTG_FS A-B device connection
VDD
5 V to VDD
Voltage
regulator(1)
VDD
GPIO

GPIO + IRQ

EN

STMPS2141STR
Current-limited 5 V Pwr
Overcurrent power distribution
switch(2)

VBUS
DM
OSC_IN

DP
ID

OSC_OUT

VBUS
DM
DP
ID
VSS

USBmicro-AB connector

STM32

MSv36917V2

1. External voltage regulator only needed when building a VBUS powered device.
2. STMPS2141STR needed only if the application has to support a VBUS powered device. A basic power
switch can be used if 5 V are available on the application board.

72.5.1

ID line detection
The host or peripheral (the default) role is assumed depending on the ID input pin. The ID
line status is determined on plugging in the USB cable, depending on whether a MicroA or
MicroB plug is connected to the micro-AB receptacle.

<!-- pagebreak -->

•

If the B-side of the USB cable is connected with a floating ID wire, the integrated pullup resistor detects a high ID level and the default peripheral role is confirmed. In this
configuration the OTG_FS complies with the standard FSM described in section 4.2.4:
ID pin of the On-the-Go specification Rev2.0, supplement to the USB2.0.

•

If the A-side of the USB cable is connected with a grounded ID, the OTG_FS issues an
ID line status change interrupt (CIDSCHG bit in OTG_GINTSTS) for host software
initialization, and automatically switches to the host role. In this configuration the
OTG_FS complies with the standard FSM described by section 4.2.4: ID pin of the Onthe-Go specification Rev2.0, supplement to the USB2.0.

RM0456 Rev 6

RM0456

72.5.2

USB on-the-go full-speed (OTG_FS)

HNP dual role device
The HNP capable bit in the Global USB configuration register (HNPCAP bit in OTG_
GUSBCFG) enables the OTG_FS core to dynamically change its role from A-host to Aperipheral and vice-versa, or from B-Peripheral to B-host and vice-versa according to the
host negotiation protocol (HNP). The current device status can be read by the combined
values of the connector ID status bit in the Global OTG control and status register (CIDSTS
bit in OTG_GOTGCTL) and the current mode of operation bit in the global interrupt and
status register (CMOD bit in OTG_GINTSTS).
The HNP program model is described in detail in Section 72.16: OTG_FS programming
model.

72.5.3

SRP dual role device
The SRP capable bit in the global USB configuration register (SRPCAP bit in
OTG_GUSBCFG) enables the OTG_FS core to switch off the generation of VBUS for the Adevice to save power. Note that the A-device is always in charge of driving VBUS regardless
of the host or peripheral role of the OTG_FS.
The SRP A/B-device program model is described in detail in Section 72.16: OTG_FS
programming model.

72.6

OTG_FS as a USB peripheral
This section gives the functional description of the OTG_FS in the USB peripheral mode.
The OTG_FS works as an USB peripheral in the following circumstances:
•

OTG B-Peripheral
–

•

–
•

If the ID line is present, functional and connected to the B-side of the USB cable,
and the HNP-capable bit in the Global USB Configuration register (HNPCAP bit in
OTG_GUSBCFG) is cleared.

Peripheral only (see Figure 895: OTG_FS peripheral-only connection)
–

Note:

OTG A-device state after the HNP switches the OTG_FS to its peripheral role

B-device
–

•

OTG B-device default state if B-side of USB cable is plugged in

OTG A-Peripheral

The force device mode bit (FDMOD) in the Section 72.15.4: OTG USB
configuration register (OTG_GUSBCFG) is set to 1, forcing the OTG_FS core to
work as an USB peripheral-only. In this case, the ID line is ignored even if it is
present on the USB connector.

To build a bus-powered device implementation in case of the B-device or peripheral-only
configuration, an external regulator has to be added, that generates the necessary powersupply from VBUS.

RM0456 Rev 6

<!-- pagebreak -->

3291

USB on-the-go full-speed (OTG_FS)

RM0456

Figure 895. OTG_FS peripheral-only connection
VDD
5 V to VDD
Voltage
regulator
VDD
GPIO
GPIO + IRQ

EN

STMPS2141STR
Current-limited 5 V Pwr
Overcurrent power distribution
switch(1)

DM
OSC_IN

DP

OSC_OUT

VSS

USB micro connector

VBUS

MSv36916V2

1. Use a regulator to build a bus-powered device.

72.6.1

SRP-capable peripheral
The SRP capable bit in the Global USB configuration register (SRPCAP bit in
OTG_GUSBCFG) enables the OTG_FS to support the session request protocol (SRP). In
this way, it allows the remote A-device to save power by switching off VBUS while the USB
session is suspended.
The SRP peripheral mode program model is described in detail in the B-device session
request protocol section.

72.6.2

Peripheral states
Powered state
The VBUS input detects the B-session valid voltage by which the USB peripheral is allowed
to enter the powered state (see USB2.0 section 9.1). The OTG_FS then automatically
connects the DP pull-up resistor to signal full-speed device connection to the host and
generates the session request interrupt (SRQINT bit in OTG_GINTSTS) to notify the
powered state.
The VBUS input also ensures that valid VBUS levels are supplied by the host during USB
operations. If a drop in VBUS below B-session valid happens to be detected (for instance
because of a power disturbance or if the host port has been switched off), the OTG_FS
automatically disconnects and the session end detected (SEDET bit in OTG_GOTGINT)
interrupt is generated to notify that the OTG_FS has exited the powered state.
In the powered state, the OTG_FS expects to receive some reset signaling from the host.
No other USB operation is possible. When a reset signaling is received the reset detected
interrupt (USBRST in OTG_GINTSTS) is generated. When the reset signaling is complete,
the enumeration done interrupt (ENUMDNE bit in OTG_GINTSTS) is generated and the
OTG_FS enters the Default state.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

USB on-the-go full-speed (OTG_FS)

Soft disconnect
The powered state can be exited by software with the soft disconnect feature. The DP pullup resistor is removed by setting the soft disconnect bit in the device control register (SDIS
bit in OTG_DCTL), causing a device disconnect detection interrupt on the host side even
though the USB cable was not really removed from the host port.

Default state
In the Default state the OTG_FS expects to receive a SET_ADDRESS command from the
host. No other USB operation is possible. When a valid SET_ADDRESS command is
decoded on the USB, the application writes the corresponding number into the device
address field in the device configuration register (DAD bit in OTG_DCFG). The OTG_FS
then enters the address state and is ready to answer host transactions at the configured
USB address.

Suspended state
The OTG_FS peripheral constantly monitors the USB activity. After counting 3 ms of USB
idleness, the early suspend interrupt (ESUSP bit in OTG_GINTSTS) is issued, and
confirmed 3 ms later, if appropriate, by the suspend interrupt (USBSUSP bit in
OTG_GINTSTS). The device suspend bit is then automatically set in the device status
register (SUSPSTS bit in OTG_DSTS) and the OTG_FS enters the suspended state.
The suspended state may optionally be exited by the device itself. In this case the
application sets the remote wake-up signaling bit in the device control register (RWUSIG bit
in OTG_DCTL) and clears it after 1 to 15 ms.
When a resume signaling is detected from the host, the resume interrupt (WKUPINT bit in
OTG_GINTSTS) is generated and the device suspend bit is automatically cleared.

72.6.3

Peripheral endpoints
The OTG_FS core instantiates the following USB endpoints:
•

•

Control endpoint 0:
–

Bidirectional and handles control messages only

–

Separate set of registers to handle in and out transactions

–

Proper control (OTG_DIEPCTL0/OTG_DOEPCTL0), transfer configuration
(OTG_DIEPTSIZ0/OTG_DOEPTSIZ0), and status-interrupt
(OTG_DIEPINT0/)OTG_DOEPINT0) registers. The available set of bits inside the
control and transfer size registers slightly differs from that of other endpoints

5 IN endpoints
–

Each of them can be configured to support the isochronous, bulk or interrupt
transfer type

–

Each of them has proper control (OTG_DIEPCTLx), transfer configuration
(OTG_DIEPTSIZx), and status-interrupt (OTG_DIEPINTx) registers

–

The device IN endpoints common interrupt mask register (OTG_DIEPMSK) is
available to enable/disable a single kind of endpoint interrupt source on all of the
IN endpoints (EP0 included)

–

Support for incomplete isochronous IN transfer interrupt (IISOIXFR bit in
OTG_GINTSTS), asserted when there is at least one isochronous IN endpoint on

RM0456 Rev 6

<!-- pagebreak -->

3291

USB on-the-go full-speed (OTG_FS)

RM0456

which the transfer is not completed in the current frame. This interrupt is asserted
along with the end of periodic frame interrupt (OTG_GINTSTS/EOPF).
•

5 OUT endpoints
–

Each of them can be configured to support the isochronous, bulk or interrupt
transfer type

–

Each of them has a proper control (OTG_DOEPCTLx), transfer configuration
(OTG_DOEPTSIZx) and status-interrupt (OTG_DOEPINTx) register

–

Device OUT endpoints common interrupt mask register (OTG_DOEPMSK) is
available to enable/disable a single kind of endpoint interrupt source on all of the
OUT endpoints (EP0 included)

–

Support for incomplete isochronous OUT transfer interrupt (INCOMPISOOUT bit
in OTG_GINTSTS), asserted when there is at least one isochronous OUT
endpoint on which the transfer is not completed in the current frame. This interrupt
is asserted along with the end of periodic frame interrupt (OTG_GINTSTS/EOPF).

Endpoint control
•

The following endpoint controls are available to the application through the device
endpoint-x IN/OUT control register (OTG_DIEPCTLx/OTG_DOEPCTLx):
–

Endpoint enable/disable

–

Endpoint activate in current configuration

–

Program USB transfer type (isochronous, bulk, interrupt)

–

Program supported packet size

–

Program Tx FIFO number associated with the IN endpoint

–

Program the expected or transmitted data0/data1 PID (bulk/interrupt only)

–

Program the even/odd frame during which the transaction is received or
transmitted (isochronous only)

–

Optionally program the NAK bit to always negative-acknowledge the host
regardless of the FIFO status

–

Optionally program the STALL bit to always stall host tokens to that endpoint

–

Optionally program the SNOOP mode for OUT endpoint not to check the CRC
field of received data

Endpoint transfer
The device endpoint-x transfer size registers (OTG_DIEPTSIZx/OTG_DOEPTSIZx) allow
the application to program the transfer size parameters and read the transfer status.
Programming must be done before setting the endpoint enable bit in the endpoint control
register. Once the endpoint is enabled, these fields are read-only as the OTG_FS core
updates them with the current transfer status.
The following transfer parameters can be programmed:
•

Transfer size in bytes

•

Number of packets that constitute the overall transfer size

Endpoint status/interrupt
The device endpoint-x interrupt registers (OTG_DIEPINTx/OTG_DOPEPINTx) indicate the
status of an endpoint with respect to USB- and AHB-related events. The application must
read these registers when the OUT endpoint interrupt bit or the IN endpoint interrupt bit in

<!-- pagebreak -->

RM0456 Rev 6

RM0456

USB on-the-go full-speed (OTG_FS)
the core interrupt register (OEPINT bit in OTG_GINTSTS or IEPINT bit in OTG_GINTSTS,
respectively) is set. Before the application can read these registers, it must first read the
device all endpoints interrupt (OTG_DAINT) register to get the exact endpoint number for
the device endpoint-x interrupt register. The application must clear the appropriate bit in this
register to clear the corresponding bits in the OTG_DAINT and OTG_GINTSTS registers
The peripheral core provides the following status checks and interrupt generation:

72.7

•

Transfer completed interrupt, indicating that data transfer was completed on both the
application (AHB) and USB sides

•

Setup stage has been done (control-out only)

•

Associated transmit FIFO is half or completely empty (in endpoints)

•

NAK acknowledge has been transmitted to the host (isochronous-in only)

•

IN token received when Tx FIFO was empty (bulk-in/interrupt-in only)

•

Out token received when endpoint was not yet enabled

•

Babble error condition has been detected

•

Endpoint disable by application is effective

•

Endpoint NAK by application is effective (isochronous-in only)

•

More than 3 back-to-back setup packets were received (control-out only)

•

Timeout condition detected (control-in only)

•

Isochronous out packet has been dropped, without generating an interrupt

OTG_FS as a USB host
This section gives the functional description of the OTG_FS in the USB host mode. The
OTG_FS works as a USB host in the following circumstances:
Automatic host mode direct from ID pin:
The ID pin is not always available, refer to product datasheet for available pins.
•

Use cases:
–

•

Micro connector: A-Device with a Micro-A plug inserted, and connector ID pin
connected to the STM32 ID pin which will then be detected in a low state.

Note that the integrated pulldown resistors are automatically set on the DP/DM lines.

Manual forcing of host mode when not possible via ID pin:
The force host mode bit (FHMOD) in the OTG USB configuration register
(OTG_GUSBCFG) forces the OTG_FScore to work as a USB host-only when required.
•

•
Note:

Use cases:
–

Standard-A connector which always implies Host mode.

–

Micro connector, so ID present on connector, but ID pin not available in pinout, at
A-plug insertion (detected by another means, for example: GPIO).

–

A Type-C connector, when host functionality is determined at cable attachment or
via power delivery messages.

Note that the integrated pulldown resistors are automatically set on the DP/DM lines

On-chip 5 V VBUS generation is not supported. For this reason, a charge pump or, if 5 V are
available on the application board, a basic power switch must be added externally to drive

RM0456 Rev 6

<!-- pagebreak -->

3291

USB on-the-go full-speed (OTG_FS)

RM0456

the 5 V VBUS line. The external charge pump can be driven by any GPIO output. This is
required for the OTG A-host, A-device and host-only configurations.
Figure 896. OTG_FS host-only connection
VDD
5V

GPIO + IRQ

STMPS2141STR
5 V Pwr
Current-limited
Overcurrent power distribution
switch(2)
VBUS
DM

OSC_IN

DP
VSS

OSC_OUT

USB Std-A connector

GPIO

EN

MSv36915V2

1. VDD range is between 2 V and 3.6 V.

72.7.1

SRP-capable host
SRP support is available through the SRP capable bit in the global USB configuration
register (SRPCAP bit in OTG_GUSBCFG). With the SRP feature enabled, the host can
save power by switching off the VBUS power while the USB session is suspended.
The SRP host mode program model is described in detail in the A-device session request
protocol) section.

72.7.2

USB host states
Host port power
On-chip 5 V VBUS generation is not supported. For this reason, a charge pump or, if 5 V are
available on the application board, a basic power switch, must be added externally to drive
the 5 V VBUS line. The external charge pump can be driven by any GPIO output or via an
I2C interface connected to an external PMIC (power management IC). When the application
decides to power on VBUS, it must also set the port power bit in the host port control and
status register (PPWR bit in OTG_HPRT).

VBUS valid
When HNP or SRP is enabled the VBUS sensing pin must be connected to VBUS. The VBUS
input ensures that valid VBUS levels are supplied by the charge pump during USB
operations. Any unforeseen VBUS voltage drop below the VBUS valid threshold (4.4 V) leads
to an OTG interrupt triggered by the session end detected bit (SEDET bit in
OTG_GOTGINT). The application is then required to remove the VBUS power and clear the
port power bit.
When HNP and SRP are both disabled, the VBUS sensing pin does not need to be
connected to VBUS.
The charge pump overcurrent flag can also be used to prevent electrical damage. Connect
the overcurrent flag output from the charge pump to any GPIO input and configure it to

<!-- pagebreak -->

RM0456 Rev 6

RM0456

USB on-the-go full-speed (OTG_FS)
generate a port interrupt on the active level. The overcurrent ISR must promptly disable the
VBUS generation and clear the port power bit.

Host detection of a peripheral connection
If SRP or HNP are enabled, even if USB peripherals or B-devices can be attached at any
time, the OTG_FS does not detect any bus connection until VBUS is no longer sensed at a
valid level (5 V). When VBUS is at a valid level and a remote B-device is attached, the
OTG_FS core issues a host port interrupt triggered by the device connected bit in the host
port control and status register (PCDET bit in OTG_HPRT).
When HNP and SRP are both disabled, USB peripherals or B-device are detected as soon
as they are connected. The OTG_FS core issues a host port interrupt triggered by the
device connected bit in the host port control and status (PCDET bit in OTG_HPRT).

Host detection of peripheral a disconnection
The peripheral disconnection event triggers the disconnect detected interrupt (DISCINT bit
in OTG_GINTSTS).

Host enumeration
After detecting a peripheral connection the host must start the enumeration process by
sending USB reset and configuration commands to the new peripheral.
Before starting to drive a USB reset, the application waits for the OTG interrupt triggered by
the debounce done bit (DBCDNE bit in OTG_GOTGINT), which indicates that the bus is
stable again after the electrical debounce caused by the attachment of a pull-up resistor on
DP (FS) or DM (LS).
The application drives a USB reset signaling (single-ended zero) over the USB by keeping
the port reset bit set in the host port control and status register (PRST bit in OTG_HPRT) for
a minimum of 10 ms and a maximum of 20 ms. The application takes care of the timing
count and then of clearing the port reset bit.
Once the USB reset sequence has completed, the host port interrupt is triggered by the port
enable/disable change bit (PENCHNG bit in OTG_HPRT). This informs the application that
the speed of the enumerated peripheral can be read from the port speed field in the host
port control and status register (PSPD bit in OTG_HPRT) and that the host is starting to
drive SOFs (FS) or Keep alives (LS). The host is now ready to complete the peripheral
enumeration by sending peripheral configuration commands.

Host suspend
The application decides to suspend the USB activity by setting the port suspend bit in the
host port control and status register (PSUSP bit in OTG_HPRT). The OTG_FS core stops
sending SOFs and enters the suspended state.
The suspended state can be optionally exited on the remote device’s initiative (remote
wake-up). In this case the remote wake-up interrupt (WKUPINT bit in OTG_GINTSTS) is
generated upon detection of a remote wake-up signaling, the port resume bit in the host port
control and status register (PRES bit in OTG_HPRT) self-sets, and resume signaling is
automatically driven over the USB. The application must time the resume window and then
clear the port resume bit to exit the suspended state and restart the SOF.

RM0456 Rev 6

<!-- pagebreak -->

3291

USB on-the-go full-speed (OTG_FS)

RM0456

If the suspended state is exited on the host initiative, the application must set the port
resume bit to start resume signaling on the host port, time the resume window and finally
clear the port resume bit.

72.7.3

Host channels
The OTG_FS core instantiates 12 host channels. Each host channel supports an USB host
transfer (USB pipe). The host is not able to support more than 12 transfer requests at the
same time. If more than 12 transfer requests are pending from the application, the host
controller driver (HCD) must re-allocate channels when they become available from
previous duty, that is, after receiving the transfer completed and channel halted interrupts.
Each host channel can be configured to support in/out and any type of periodic/nonperiodic
transaction. Each host channel makes us of proper control (OTG_HCCHARx), transfer
configuration (OTG_HCTSIZx) and status/interrupt (OTG_HCINTx) registers with
associated mask (OTG_HCINTMSKx) registers.

Host channel control
•

The following host channel controls are available to the application through the host
channel-x characteristics register (OTG_HCCHARx):
–

Channel enable/disable

–

Program the FS/LS speed of target USB peripheral

–

Program the address of target USB peripheral

–

Program the endpoint number of target USB peripheral

–

Program the transfer IN/OUT direction

–

Program the USB transfer type (control, bulk, interrupt, isochronous)

–

Program the maximum packet size (MPS)

–

Program the periodic transfer to be executed during odd/even frames

Host channel transfer
The host channel transfer size registers (OTG_HCTSIZx) allow the application to program
the transfer size parameters, and read the transfer status. Programming must be done
before setting the channel enable bit in the host channel characteristics register. Once the
endpoint is enabled the packet count field is read-only as the OTG_FS core updates it
according to the current transfer status.
•

The following transfer parameters can be programmed:
–

transfer size in bytes

–

number of packets making up the overall transfer size

–

initial data PID

Host channel status/interrupt
The host channel-x interrupt register (OTG_HCINTx) indicates the status of an endpoint
with respect to USB- and AHB-related events. The application must read these register
when the host channels interrupt bit in the core interrupt register (HCINT bit in
OTG_GINTSTS) is set. Before the application can read these registers, it must first read the
host all channels interrupt (OTG_HAINT) register to get the exact channel number for the
host channel-x interrupt register. The application must clear the appropriate bit in this
register to clear the corresponding bits in the OTG_HAINT and OTG_GINTSTS registers.

<!-- pagebreak -->

