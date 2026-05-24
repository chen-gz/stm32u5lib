3134

Universal serial bus full-speed host/device interface (USB)

RM0456

Table 740. Resume event detection for host
[RXDP,RXDM] status
“00”

Wake-up event

Required resume software action

Not allowed (noise on bus)

Go back in Suspend mode

Full speed capable device:
Not allowed (noise on bus)

None

“10”
Low speed device: Device
remote wake-up resume

“01”

Full speed capable device:
Device remote wake-up
resume

None

Low speed device:
Not allowed (noise on bus)
“11”

<!-- pagebreak -->

Not allowed (noise on bus)

RM0456 Rev 6

Go back in Suspend mode

RM0456

Universal serial bus full-speed host/device interface (USB)

71.6

USB registers
The USB peripheral registers can be divided into the following groups:
•

Common registers: interrupt and control registers. These registers affect the general
behavior of the USB peripheral defining operating mode, interrupt handling, device
address and giving access to the current frame number updated by the host PC.

•

–

USB_CNTR

–

USB_ISTR

–

USB_FNR

–

USB_DADDR

–

USB_LPMCSR

–

USB_BCDR

Endpoint/channel registers: endpoint/channel configuration and status
–

USB_CHEPnR

Refer to Section 1.2 for a list of abbreviations used in register descriptions.
The peripheral registers can be accessed by words (32-bit).

71.6.1

USB control register (USB_CNTR)
Address offset: 0x40
Reset value: 0x0000 0003

31
HOST

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

DDISC
M

THR
512M

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

CTRM

PMA
OVRM

ERRM

WKUP
M

SUSP
M

RST_D
CONM

SOFM

ESOF
M

L1REQ
M

Res.

L1RE
S

L2RE
S

SUS
PEN

SUSP
RDY

PDWN

USB
RST

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

r

rw

rw

rw

Bit 31 HOST: HOST mode
HOST bit selects betweens host or device USB mode of operation. It must be set before
enabling the USB peripheral by the function enable bit.
0: USB Device function
1: USB host function
Bits 30:18 Reserved, must be kept at reset value.
Bit 17 DDISCM: Device disconnection mask
– Host mode
0: Device disconnection interrupt disabled
1: Device disconnection interrupt enabled
Bit 16 THR512M: 512 byte threshold interrupt mask
0: 512 byte threshold interrupt disabled
1: 512 byte threshold interrupt enabled

RM0456 Rev 6

<!-- pagebreak -->

3134

Universal serial bus full-speed host/device interface (USB)

RM0456

Bit 15 CTRM: Correct transfer interrupt mask
0: Correct transfer (CTR) interrupt disabled.
1: CTR interrupt enabled, an interrupt request is generated when the corresponding bit in the
USB_ISTR register is set.
Bit 14 PMAOVRM: Packet memory area over / underrun interrupt mask
0: PMAOVR interrupt disabled.
1: PMAOVR interrupt enabled, an interrupt request is generated when the corresponding bit
in the USB_ISTR register is set.
Bit 13 ERRM: Error interrupt mask
0: ERR interrupt disabled.
1: ERR interrupt enabled, an interrupt request is generated when the corresponding bit in the
USB_ISTR register is set.
Bit 12 WKUPM: Wake-up interrupt mask
0: WKUP interrupt disabled.
1: WKUP interrupt enabled, an interrupt request is generated when the corresponding bit in
the USB_ISTR register is set.
Bit 11 SUSPM: Suspend mode interrupt mask
0: Suspend mode request (SUSP) interrupt disabled.
1: SUSP interrupt enabled, an interrupt request is generated when the corresponding bit in
the USB_ISTR register is set.
Bit 10 RST_DCONM: USB reset request (Device mode) or device connect/disconnect (Host mode)
interrupt mask
0: RESET interrupt disabled.
1: RESET interrupt enabled, an interrupt request is generated when the corresponding bit in
the USB_ISTR register is set.
Bit 9 SOFM: Start of frame interrupt mask
0: SOF interrupt disabled.
1: SOF interrupt enabled, an interrupt request is generated when the corresponding bit in the
USB_ISTR register is set.
Bit 8 ESOFM: Expected start of frame interrupt mask
0: Expected start of frame (ESOF) interrupt disabled.
1: ESOF interrupt enabled, an interrupt request is generated when the corresponding bit in
the USB_ISTR register is set.
Bit 7 L1REQM: LPM L1 state request interrupt mask
0: LPM L1 state request (L1REQ) interrupt disabled.
1: L1REQ interrupt enabled, an interrupt request is generated when the corresponding bit in
the USB_ISTR register is set.
Bit 6 Reserved, must be kept at reset value.
Bit 5 L1RES: L1 remote wake-up / resume driver
– Device mode
Software sets this bit to send a LPM L1 50 μs remote wake-up signaling to the host. After the
signaling ends, this bit is cleared by hardware.
0: No effect
1: Send 50 μs remote-wake-up signaling to host

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Universal serial bus full-speed host/device interface (USB)

Bit 4 L2RES: L2 remote wake-up / resume driver
– Device mode
The microcontroller can set this bit to send remote wake-up signaling to the host. It must be
activated, according to USB specifications, for no less than 1 ms and no more than 15 ms
after which the host PC is ready to drive the resume sequence up to its end.
– Host mode
Software sets this bit to send resume signaling to the device.
Software clears this bit to send end of resume to device and restart SOF generation.
In the context of remote wake up, this bit is to be set following the WAKEUP interrupt.
0: No effect
1: Send L2 resume signaling to device
Bit 3 SUSPEN: Suspend state enable
– Condition: Device mode
Software can set this bit when the SUSP interrupt is received, which is issued when no traffic
is received by the USB peripheral for 3 ms. Software can also set this bit when the L1REQ
interrupt is received with positive acknowledge sent.
As soon as the suspend state is propagated internally all device activity is stopped, USB
clock is gated, USB transceiver is set into low power mode and the SUSPRDY bit is set by
hardware. In the case that device application wants to pursue more aggressive power saving
by stopping the USB clock source and by moving the microcontroller to stop mode, as in the
case of bus powered device application, it must first wait few cycles to see the
SUSPRDY = 1 acknowledge the suspend request.
This bit is cleared by hardware simultaneous with the WAKEUP flag set.
0: No effect
1: Enter L1/L2 suspend
– Condition: Host mode
Software can set this bit when host application has nothing scheduled for the next frames
and wants to enter long term power saving. When set, it stops immediately SOF generation
and any other host activity, gates the USB clock and sets the transceiver in low power mode.
If any USB transaction is on-going at the time SUSPEN is set, suspend is entered at the end
of the current transaction.
As soon as suspend state is propagated internally and gets effective the SUSPRDY bit is
set. In the case that host application wants to pursue more aggressive power saving by
stopping the USB clock source and by moving the micro-controller to STOP mode, it must
first wait few cycles to see SUSPRDY=1 acknowledge to the suspend request.
This bit is cleared by hardware simultaneous with the WAKEUP flag set.
0: No effect
1: Enter L1/L2 suspend
Bit 2 SUSPRDY: Suspend state effective
This bit is set by hardware as soon as the suspend state entered through the SUSPEN
control gets internally effective. In this state USB activity is suspended, USB clock is gated,
transceiver is set in low power mode by disabling the differential receiver. Only
asynchronous wake-up logic and single ended receiver is kept alive to detect remote wakeup or resume events.
Software must poll this bit to confirm it to be set before any STOP mode entry.
This bit is cleared by hardware simultaneously to the WAKEUP flag being set.
0: Normal operation
1: Suspend state

RM0456 Rev 6

<!-- pagebreak -->

3134

Universal serial bus full-speed host/device interface (USB)

RM0456

Bit 1 PDWN: Power down
This bit is used to completely switch off all USB-related analog parts if it is required to
completely disable the USB peripheral for any reason. When this bit is set, the USB
peripheral is disconnected from the transceivers and it cannot be used.
0: Exit power down
1: Enter power down mode
Bit 0 USBRST: USB Reset
– Condition: Device mode
Software can set this bit to reset the USB core, exactly as it happens when receiving a
RESET signaling on the USB.The USB peripheral, in response to a RESET, resets its
internal protocol state machine. Reception and transmission are disabled until the
RST_DCON bit is cleared. All configuration registers do not reset: the microcontroller must
explicitly clear these registers (this is to ensure that the RST_DCON interrupt can be safely
delivered, and any transaction immediately followed by a RESET can be completed). The
function address and endpoint registers are reset by an USB reset event.
0: No effect
1: USB core is under reset
– Condition: Host mode
Software sets this bit to drive USB reset state on the bus and initialize the device. USB reset
terminates as soon as this bit is cleared by software.
0: No effect
1: USB reset driven

71.6.2

USB interrupt status register (USB_ISTR)
Address offset: 0x44
Reset value: 0x0000 0000
This register contains the status of all the interrupt sources permitting application software to
determine which events caused an interrupt request.
The upper part of this register contains single bits, each of them representing a specific
event. These bits are set by the hardware when the related event occurs; if the
corresponding bit in the USB_CNTR register is set, a generic interrupt request is generated.
The interrupt routine, examining each bit, performs all necessary actions, and finally it clears
the serviced bits. If any of them is not cleared, the interrupt is considered to be still pending,
and the interrupt line is kept high again. If several bits are set simultaneously, only a single
interrupt is generated.
Endpoint/channel transaction completion can be handled in a different way to reduce
interrupt response latency. The CTR bit is set by the hardware as soon as an
endpoint/channel successfully completes a transaction, generating a generic interrupt
request if the corresponding bit in USB_CNTR is set. An endpoint/channel dedicated
interrupt condition is activated independently from the CTRM bit in the USB_CNTR register.
Both interrupt conditions remain active until software clears the pending bit in the
corresponding USB_CHEPnR register (the CTR bit is actually a read only bit). For endpoint/channel-related interrupts, the software can use the direction of transaction (DIR) and IDN
read-only bits to identify which endpoint/channel made the last interrupt request and called
the corresponding interrupt service routine.
The user can choose the relative priority of simultaneously pending USB_ISTR events by
specifying the order in which software checks USB_ISTR bits in an interrupt service routine.
Only the bits related to events, which are serviced, are cleared. At the end of the service
routine, another interrupt is requested, to service the remaining conditions.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Universal serial bus full-speed host/device interface (USB)
To avoid spurious clearing of some bits, it is recommended to clear them with a load
instruction where all bits which must not be altered are written with 1, and all bits to be
cleared are written with 0 (these bits can only be cleared by software). Read-modify-write
cycles must be avoided because between the read and the write operations another bit can
be set by the hardware and the next write clears it before the device has the time to service
the event.

31
Res.

30

29

LS_ DCON_
DCON STAT
r

r

15

14

13

CTR

PMA
OVR

ERR

r

rc_w0

rc_w0

28
Res.

12

27
Res.

26
Res.

25
Res.

24
Res.

23
Res.

22
Res.

21
Res.

20
Res.

11

10

9

8

7

6

5

4

WKUP

SUSP

RST_
DCON

SOF

ESOF

L1REQ

Res.

Res.

DIR

rc_w0

rc_w0

rc_w0

rc_w0

rc_w0

rc_w0

r

19
Res.

3

18
Res.

2

17

16

DDISC

THR
512

rc_w0

rc_w0

1

0

IDN[3:0]
r

r

r

r

Bit 31 Reserved, must be kept at reset value.
Bit 30 LS_DCON: Low speed device connected
– Host mode:
This bit is set by hardware when an LS device connection is detected. Device connection is
signaled after LS J-state is sampled for 22 consecutive cycles of the USB clock (48 MHz)
from the unconnected state.
Bit 29 DCON_STAT: Device connection status
– Host mode:
This bit contains information about device connection status. It is set by hardware when a
LS/FS device is attached to the host while it is reset when the device is disconnected.
0: No device connected
1: FS or LS device connected to the host
Bits 28:18 Reserved, must be kept at reset value.
Bit 17 DDISC: Device connection
– Host mode
This bit is set when a device connection is detected. This bit is read/write but only 0 can be
written and writing 1 has no effect.
Bit 16 THR512: 512 byte threshold interrupt
This bit is set to 1 by the hardware when 512 bytes have been transmitted or received during
isochronous transfers. This bit is read/write but only 0 can be written and writing 1 has no
effect. Note that no information is available to indicate the associated channel/endpoint,
however in practice only one ISO endpoint/channel with such large packets can be
supported, so that channel.
Bit 15 CTR: Completed transfer in host mode
This bit is set by the hardware to indicate that an endpoint/channel has successfully
completed a transaction; using DIR and IDN bits software can determine which
endpoint/channel requested the interrupt. This bit is read-only.

RM0456 Rev 6

<!-- pagebreak -->

3134

Universal serial bus full-speed host/device interface (USB)

RM0456

Bit 14 PMAOVR: Packet memory area over / underrun
This bit is set if the microcontroller has not been able to respond in time to an USB memory
request. The USB peripheral handles this event in the following way: During reception an
ACK handshake packet is not sent, during transmission a bit-stuff error is forced on the
transmitted stream; in both cases the host retries the transaction. The PMAOVR interrupt
must never occur during normal operations. Since the failed transaction is retried by the host,
the application software has the chance to speed-up device operations during this interrupt
handling, to be ready for the next transaction retry; however this does not happen during
isochronous transfers (no isochronous transaction is anyway retried) leading to a loss of data
in this case. This bit is read/write but only 0 can be written and writing 1 has no effect.
Bit 13 ERR: Error
This flag is set whenever one of the errors listed below has occurred:
NANS: No ANSwer. The timeout for a host response has expired.
CRC: Cyclic redundancy check error. One of the received CRCs, either in the token or in
the data, was wrong.
BST:
Bit stuffing error. A bit stuffing error was detected anywhere in the PID, data, and/or
CRC.
FVIO: Framing format violation. A non-standard frame was received (EOP not in the right
place, wrong token sequence, etc.).
The USB software can usually ignore errors, since the USB peripheral and the PC host
manage retransmission in case of errors in a fully transparent way. This interrupt can be
useful during the software development phase, or to monitor the quality of transmission over
the USB bus, to flag possible problems to the user (for example loose connector, too noisy
environment, broken conductor in the USB cable and so on). This bit is read/write but only 0
can be written and writing 1 has no effect.
Bit 12 WKUP: Wake-up
This bit is set to 1 by the hardware when, during suspend mode, activity is detected that
wakes up the USB peripheral. This event asynchronously clears the SUSPRDY bit in the
CTLR register and activates the USB_WAKEUP line, which can be used to notify the rest of
the device (for example wake-up unit) about the start of the resume process. This bit is
read/write but only 0 can be written and writing 1 has no effect.
Bit 11 SUSP: Suspend mode request
– Device mode
This bit is set by the hardware when no traffic has been received for 3 ms, indicating a
suspend mode request from the USB bus. The suspend condition check is enabled
immediately after any USB reset and it is disabled by the hardware when the suspend mode
is active (SUSPEN=1) until the end of resume sequence. This bit is read/write but only 0 can
be written and writing 1 has no effect.
Bit 10 RST_DCON: USB reset request (Device mode) or device connect/disconnect (Host mode)
– Device mode
This bit is set by hardware when an USB reset is released by the host and the bus returns to
idle. USB reset state is internally detected after the sampling of 60 consecutive SE0 cycles.
– Host mode
This bit is set by hardware when device connection or device disconnection is detected.
Device connection is signaled after J state is sampled for 22 cycles consecutively from
unconnected state. Device disconnection is signaled after SE0 state is seen for 22 bit times
consecutively from connected state.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Universal serial bus full-speed host/device interface (USB)

Bit 9 SOF: Start of frame
This bit signals the beginning of a new USB frame and it is set when a SOF packet arrives
through the USB bus. The interrupt service routine can monitor the SOF events to have a
1 ms synchronization event to the USB host and to safely read the USB_FNR register which
is updated at the SOF packet reception (this can be useful for isochronous applications). This
bit is read/write but only 0 can be written and writing 1 has no effect.
Bit 8 ESOF: Expected start of frame
– Device mode
This bit is set by the hardware when an SOF packet is expected but not received. The host
sends an SOF packet each 1 ms, but if the device does not receive it properly, the suspend
timer issues this interrupt. If three consecutive ESOF interrupts are generated (for example
three SOF packets are lost) without any traffic occurring in between, a SUSP interrupt is
generated. This bit is set even when the missing SOF packets occur while the suspend timer
is not yet locked. This bit is read/write but only 0 can be written and writing 1 has no effect.
Bit 7 L1REQ: LPM L1 state request
– Device mode
This bit is set by the hardware when LPM command to enter the L1 state is successfully
received and acknowledged. This bit is read/write but only 0 can be written and writing 1 has
no effect.
Bits 6:5 Reserved, must be kept at reset value.
Bit 4 DIR: Direction of transaction
This bit is written by the hardware according to the direction of the successful transaction,
which generated the interrupt request.
If DIR bit = 0, VTTX bit is set in the USB_CHEPnR register related to the interrupting
endpoint. The interrupting transaction is of IN type (data transmitted by the USB peripheral to
the host PC).
If DIR bit = 1, VTRX bit or both VTTX/VTRX are set in the USB_CHEPnR register related to
the interrupting endpoint. The interrupting transaction is of OUT type (data received by the
USB peripheral from the host PC) or two pending transactions are waiting to be processed.
This information can be used by the application software to access the USB_CHEPnR bits
related to the triggering transaction since it represents the direction having the interrupt
pending. This bit is read-only.
Bits 3:0 IDN[3:0]: Device Endpoint / host channel identification number
These bits are written by the hardware according to the host channel or device endpoint
number, which generated the interrupt request. If several endpoint/channel transactions are
pending, the hardware writes the identification number related to the endpoint/channel
having the highest priority defined in the following way: two levels are defined, in order of
priority: isochronous and double-buffered bulk channels/endpoints are considered first and
then the others are examined. If more than one endpoint/channel from the same set is
requesting an interrupt, the IDN bits in USB_ISTR register are assigned according to the
lowest requesting register, CHEP0R having the highest priority followed by CHEP1R and so
on. The application software can assign a register to each endpoint/channel according to this
priority scheme, so as to order the concurring endpoint/channel requests in a suitable way.
These bits are read only.

RM0456 Rev 6

<!-- pagebreak -->

3134

Universal serial bus full-speed host/device interface (USB)

71.6.3

RM0456

USB frame number register (USB_FNR)
Address offset: 0x48
Reset value: 0x0000 0XXX (where X is undefined)

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

RXDP

RXDM

LCK

LSOF[1:0]

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

FN[10:0]
r

Bits 31:16 Reserved, must be kept at reset value.
Bit 15 RXDP: Receive data + line status
This bit can be used to observe the status of received data plus upstream port data line. It
can be used during end-of-suspend routines to help determining the wake-up event.
Bit 14 RXDM: Receive data - line status
This bit can be used to observe the status of received data minus upstream port data line. It
can be used during end-of-suspend routines to help determining the wake-up event.
Bit 13 LCK: Locked
– Device mode
This bit is set by the hardware when at least two consecutive SOF packets have been
received after the end of an USB reset condition or after the end of an USB resume
sequence. Once locked, the frame timer remains in this state until an USB reset or USB
suspend event occurs.
Bits 12:11 LSOF[1:0]: Lost SOF
– Device mode
These bits are written by the hardware when an ESOF interrupt is generated, counting the
number of consecutive SOF packets lost. At the reception of an SOF packet, these bits are
cleared.
Bits 10:0 FN[10:0]: Frame number
This bit field contains the 11-bits frame number contained in the last received SOF packet.
The frame number is incremented for every frame sent by the host and it is useful for
isochronous transfers. This bit field is updated on the generation of an SOF interrupt.

71.6.4

USB Device address (USB_DADDR)
Address offset: 0x4C
Reset value: 0x0000 0000

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

15

14

13

12

11

10

9

8

7

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

EF
rw

<!-- pagebreak -->

RM0456 Rev 6

ADD[6:0]
rw

rw

rw

rw

RM0456

Universal serial bus full-speed host/device interface (USB)

Bits 31:8 Reserved, must be kept at reset value.
Bit 7 EF: Enable function
This bit is set by the software to enable the USB Device. The address of this device is
contained in the following ADD[6:0] bits. If this bit is at 0 no transactions are handled,
irrespective of the settings of USB_CHEPnR registers.
Bits 6:0 ADD[6:0]: Device address
– Device mode
These bits contain the USB function address assigned by the host PC during the
enumeration process. Both this field and the endpoint/channel address (EA) field in the
associated USB_CHEPnR register must match with the information contained in a USB
token in order to handle a transaction to the required endpoint.
– Host mode
These bits contain the address transmitted with the LPM transaction

71.6.5

USB LPM control and status register (USB_LPMCSR)
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

LPM
ACK

LPM
EN

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

REM
WAKE

BESL[3:0]
r

r

r

r

r

Bits 31:8 Reserved, must be kept at reset value.
Bits 7:4 BESL[3:0]: BESL value
– Device mode
These bits contain the BESL value received with last ACKed LPM Token
Bit 3 REMWAKE: bRemoteWake value
– Device mode
This bit contains the bRemoteWake value received with last ACKed LPM Token
Bit 2 Reserved, must be kept at reset value.
Bit 1 LPMACK: LPM token acknowledge enable
– Device mode:
0: the valid LPM token is NYET.
1: the valid LPM token is ACK.
The NYET/ACK is returned only on a successful LPM transaction:
No errors in both the EXT token and the LPM token (else ERROR)
A valid bLinkState = 0001B (L1) is received (else STALL)
Bit 0 LPMEN: LPM support enable
– Device mode
This bit is set by the software to enable the LPM support within the USB Device. If this bit is
at 0 no LPM transactions are handled.

RM0456 Rev 6

<!-- pagebreak -->

3134

Universal serial bus full-speed host/device interface (USB)

71.6.6

RM0456

USB battery charging detector (USB_BCDR)
Address offset: 0x58
Reset value: 0x0000 0000

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

DPPU_
DPD

Res.

Res.

Res.

Res.

Res.

Res.

Res.

PS2
DET

SDET

PDET

DC
DET

SDEN

PDEN

DCD
EN

BCD
EN

r

r

r

r

rw

rw

rw

rw

rw

Bits 31:16 Reserved, must be kept at reset value.
Bit 15 DPPU_DPD: DP pull-up / DPDM pull-down
– Device mode
This bit is set by software to enable the embedded pull-up on DP line. Clearing it to 0 can be
used to signal disconnect to the host when needed by the user software.
– Host mode
This bit is set by software to enable the embedded pull-down on DP and DM lines.
Bits 14:8 Reserved, must be kept at reset value.
Bit 7 PS2DET: DM pull-up detection status
– Device mode
This bit is active only during PD and gives the result of comparison between DM voltage
level and VLGC threshold. In normal situation, the DM level must be below this threshold. If it
is above, it means that the DM is externally pulled high. This can be caused by connection to
a PS2 port (which pulls-up both DP and DM lines) or to some proprietary charger not
following the BCD specification.
0: Normal port detected (connected to SDP, ACA, CDP or DCP).
1: PS2 port or proprietary charger detected.
Bit 6 SDET: Secondary detection (SD) status
– Device mode
This bit gives the result of SD.
0: CDP detected.
1: DCP detected.
Bit 5 PDET: Primary detection (PD) status
– Device mode
This bit gives the result of PD.
0: no BCD support detected (connected to SDP or proprietary device).
1: BCD support detected (connected to ACA, CDP or DCP).
Bit 4 DCDET: Data contact detection (DCD) status
– Device mode
This bit gives the result of DCD.
0: data lines contact not detected.
1: data lines contact detected.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Universal serial bus full-speed host/device interface (USB)

Bit 3 SDEN: Secondary detection (SD) mode enable
– Device mode
This bit is set by the software to put the BCD into SD mode. Only one detection mode (DCD,
PD, SD or OFF) must be selected to work correctly.
Bit 2 PDEN: Primary detection (PD) mode enable
– Device mode
This bit is set by the software to put the BCD into PD mode. Only one detection mode (DCD,
PD, SD or OFF) must be selected to work correctly.
Bit 1 DCDEN: Data contact detection (DCD) mode enable
– Device mode
This bit is set by the software to put the BCD into DCD mode. Only one detection mode
(DCD, PD, SD or OFF) must be selected to work correctly.
Bit 0 BCDEN: Battery charging detector (BCD) enable
– Device mode
This bit is set by the software to enable the BCD support within the USB Device. When
enabled, the USB PHY is fully controlled by BCD and cannot be used for normal
communication. Once the BCD discovery is finished, the BCD must be placed in OFF mode
by clearing this bit to 0 in order to allow the normal USB operation.

71.6.7

USB endpoint/channel n register (USB_CHEPnR)
Address offset: 0x00 + 0x4 * n, (n = 0 to 7)
Reset value: 0x0000 0000
The USB peripheral supports up to 8 bidirectional endpoints or host channels. Each USB
Device must support a control endpoint/channel whose address (EA bits) must be set to 0.
The USB peripheral behaves in an undefined way if multiple endpoints are enabled having
the same endpoint/channel number value. For each endpoint, an USB_CHEPnR register is
available to store the endpoint/channel specific information.
They are also reset when an USB reset is received from the USB bus or forced through bit
USBRST in the CTLR register, except the VTRX and VTTX bits, which are kept unchanged
to avoid missing a correct packet notification immediately followed by an USB reset event.
Each endpoint/channel has its USB_CHEPnR register where n is the endpoint/channel
identifier.
Read-modify-write cycles on these registers must be avoided because between the read
and the write operations some bits can be set by the hardware and the next write would
modify them before the CPU has the time to detect the change. For this purpose, all bits
affected by this problem have an ‘invariant’ value that must be used whenever their
modification is not required. It is recommended to modify these registers with a load
instruction where all the bits, which can be modified only by the hardware, are written with
their ‘invariant’ value.

RM0456 Rev 6

<!-- pagebreak -->

3134

Universal serial bus full-speed host/device interface (USB)

31

30

29

Res.

THREE_ERR_
RX[1:0]

28

27

26

25

24

23

THREE_ERR_
TX[1:0]

ERR_
RX

ERR_
TX

LS_EP

NAK

22

RM0456

21

20

19

18

17

16

DEVADDR[6:0]

rc_w0

rc_w0

rc_w0

rc_w0

rc_w0

rc_w0

rw

rc_w0

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

VTRX

DTOG
RX

UTYPE[1:0]

EP
KIND

VTTX

DTOG
TX

rc_w0

t

rw

rw

rc_w0

t

rw

rw

STATRX[1:0]
t

t

SETUP
r

rw

STATTX[1:0]
t

t

EA[3:0]
rw

rw

Bit 31 Reserved, must be kept at reset value.
Bits 30:29 THREE_ERR_RX[1:0]: Three errors for an IN transaction
– Host mode
This bit is set by the hardware when 3 consecutive transaction errors occurred on the USB
bus for an IN transaction. THREE_ERR_RX is not generated for isochronous transactions.
The software can only clear this bit.
Coding of the received error:
00: Less than 3 errors received.
01: More than 3 errors received, last error is timeout error.
10: More than 3 errors received, last error is data error (CRC error).
11: More than 3 errors received, last error is protocol error (invalid PID, false EOP, bitstuffing
error, SYNC error).
Bits 28:27 THREE_ERR_TX[1:0]: Three errors for an OUT or SETUP transaction
– Host mode
This bit is set by the hardware when 3 consecutive transaction errors occurred on the USB
bus for an OUT transaction. THREE_ERR_TX is not generated for isochronous
transactions. The software can only clear this bit.
Coding of the received error:
00: Less than 3 errors received.
01: More than 3 errors received, last error is timeout error.
10: More than 3 errors received, last error is data error (CRC error).
11: More than 3 errors received, last error is protocol error (invalid PID, false EOP, bitstuffing
error, SYNC error).
Bit 26 ERR_RX: Received error for an IN transaction
– Host mode
This bit is set by the hardware when an error (for example no answer by the device, CRC
error, bit stuffing error, framing format violation, etc.) has occurred during an IN transaction
on this channel. The software can only clear this bit. If the ERRM bit in USB_CNTR register
is set, a generic interrupt condition is generated together with the channel related flag, which
is always activated.
Bit 25 ERR_TX: Received error for an OUT/SETUP transaction
– Host mode
This bit is set by the hardware when an error (for example no answer by the device, CRC
error, bit stuffing error, framing format violation, etc.) has occurred during an OUT or SETUP
transaction on this channel. The software can only clear this bit. If the ERRM bit in
USB_CNTR register is set, a generic interrupt condition is generated together with the
channel related flag, which is always activated.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Universal serial bus full-speed host/device interface (USB)

Bit 24 LS_EP: Low speed endpoint – host with HUB only
– Host mode
This bit is set by the software to send an LS transaction to the corresponding endpoint.
0: Full speed endpoint
1: Low speed endpoint
Bit 23 NAK:
– Host mode
This bit is set by the hardware when a device responds with a NAK. Software can use this bit
to monitor the number of NAKs received from a device.
Bits 22:16 DEVADDR[6:0]:
– Host mode
Device address assigned to the endpoint during the enumeration process.
Bit 15 VTRX: USB valid transaction received
– Device mode
This bit is set by the hardware when an OUT/SETUP transaction is successfully completed
on this endpoint; the software can only clear this bit. If the CTRM bit in USB_CNTR register
is set accordingly, a generic interrupt condition is generated together with the endpoint
related interrupt condition, which is always activated. The type of occurred transaction, OUT
or SETUP, can be determined from the SETUP bit described below.
A transaction ended with a NAK or STALL handshake does not set this bit, since no data is
actually transferred, as in the case of protocol errors or data toggle mismatches.
This bit is read/write but only 0 can be written, writing 1 has no effect.
– Host mode
This bit is set by the hardware when an IN transaction is successfully completed on this
channel. The software can only clear this bit. If the CTRM bit in USB_CNTR register is set a
generic interrupt condition is generated together with the channel related flag, which is
always activated.
- A transaction ended with a NAK sets this bit and NAK answer is reported to application
reading the NAK state from the STATRX field of this register. One NAKed transaction keeps
pending and is automatically retried by the host at the next frame, or the host can
immediately retry by resetting STATRX state to VALID.
- A transaction ended by STALL handshake sets this bit and the STALL answer is reported
to application reading the STALL state from the STATRX field of this register. Host
application must consequently disable the channel and re-enumerate.
- A transaction ended with ACK handshake sets this bit
If double buffering is disabled, ACK answer is reported by application reading the DISABLE
state from the STATRX field of this register. Host application must read received data from
USBRAM and re-arm the channel by writing VALID to the STATRX field of this register.
If double buffering is enabled, ACK answer is reported by application reading VALID state
from the STATRX field of this register. Host application must read received data from
USBRAM and toggle the DTOGTX bit of this register.
- A transaction ended with error sets this bit.
Errors can be seen via the bits ERR_RX (host mode only).
This bit is read/write but only 0 can be written, writing 1 has no effect.

RM0456 Rev 6

<!-- pagebreak -->

3134

Universal serial bus full-speed host/device interface (USB)

RM0456

Bit 14 DTOGRX: Data Toggle, for reception transfers
If the endpoint/channel is not isochronous, this bit contains the expected value of the data
toggle bit (0 = DATA0, 1 = DATA1) for the next data packet to be received. Hardware toggles
this bit, when the ACK handshake is sent following a data packet reception having a
matching data PID value; if the endpoint is defined as a control one, hardware clears this bit
at the reception of a SETUP PID received from host (in device mode), while it sets this bit to
1 when SETUP transaction is acknowledged by device (in host mode).
If the endpoint/channel is using the double-buffering feature this bit is used to support packet
buffer swapping too (Refer to Section 71.5.3: Double-buffered endpoints and usage in
Device mode).
If the endpoint/channel is isochronous, this bit is used only to support packet buffer
swapping for data transmission since no data toggling is used for this kind of
channels/endpoints and only DATA0 packet are transmitted (Refer to Section 71.5.5:
Isochronous transfers in Device mode). Hardware toggles this bit just after the end of data
packet reception, since no handshake is used for isochronous transfers.
This bit can also be toggled by the software to initialize its value (mandatory when the
endpoint is not a control one) or to force specific data toggle/packet buffer usage. When the
application software writes 0, the value of DTOGRX remains unchanged, while writing 1
makes the bit value toggle. This bit is read/write but it can be only toggled by writing 1.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Universal serial bus full-speed host/device interface (USB)

Bits 13:12 STATRX[1:0]: Status bits, for reception transfers
– Device mode
These bits contain information about the endpoint status, which are listed in Table 741:
Reception status encoding on page 3126. These bits can be toggled by software to initialize
their value. When the application software writes 0, the value remains unchanged, while
writing 1 makes the bit value to toggle. Hardware sets the STATRX bits to NAK when a
correct transfer has occurred (VTRX = 1) corresponding to a OUT or SETUP (control only)
transaction addressed to this endpoint, so the software has the time to elaborate the
received data before it acknowledges a new transaction.
Double-buffered bulk endpoints implement a special transaction flow control, which control
the status based upon buffer availability condition (Refer to Section 71.5.3: Double-buffered
endpoints and usage in Device mode).
If the endpoint is defined as isochronous, its status can be only “VALID” or “DISABLED”, so
that the hardware cannot change the status of the endpoint after a successful transaction. If
the software sets the STATRX bits to ‘STALL’ or ‘NAK’ for an isochronous endpoint, the USB
peripheral behavior is not defined. These bits are read/write but they can be only toggled by
writing 1.
– Host mode
These bits are the host application controls to start, retry, or abort host transactions driven
by the channel.
These bits also contain information about the device answer to the last IN channel
transaction and report the current status of the channel according to the following STATRX
table of states:
- DISABLE
DISABLE value is reported in case of ACK acknowledge is received on a single-buffer
channel. When in DISABLE state the channel is unused or not active waiting for application
to restart it by writing VALID. Application can reset a VALID channel to DISABLE to abort a
transaction. In this case the transaction is immediately removed from the host execution
list. If the aborted transaction was already under execution it is regularly terminated on the
USB but the relative VTRX interrupt is not generated.
- VALID
A host channel is actively trying to submit USB transaction to device only when in VALID
state.VALID state can be set by software or automatically by hardware on a NAKED
channel at the start of a new frame. When set to VALID, an host channel enters the host
execution queue and waits permission from the host frame scheduler to submit its
configured transaction.
VALID value is also reported in case of ACK acknowledge is received on a double-buffered
channel. In this case the channel remains active on the alternate buffer while application
needs to read the current buffer and toggle DTOGTX. In case software is late in reading
and the alternate buffer is not ready, the host channel is automatically suspended
transparently to the application. The suspended double buffered channel is re-activated as
soon as delay is recovered and DTOGTX is toggled.
- NAK
NAK value is reported in case of NAK acknowledge received. When in NAK state the
channel is suspended and does not try to transmit. NAK state is moved to VALID by
hardware at the start of the next frame, or software can change it to immediately retry
transmission by writing it to VALID, or can disable it and abort the transaction by writing
DISABLE
- STALL
STALL value is reported in case of STALL acknowledge received. When in STALL state the
channel behaves as disabled. Application must not retry transmission but reset the USB
and re-enumerate.

RM0456 Rev 6

<!-- pagebreak -->

3134

Universal serial bus full-speed host/device interface (USB)

RM0456

Bit 11 SETUP: Setup transaction completed
– Device mode
This bit is read-only and it is set by the hardware when the last completed transaction is a
SETUP. This bit changes its value only for control endpoints. It must be examined, in the
case of a successful receive transaction (VTRX event), to determine the type of transaction
occurred. To protect the interrupt service routine from the changes in SETUP bits due to
next incoming tokens, this bit is kept frozen while VTRX bit is at 1; its state changes when
VTRX is at 0. This bit is read-only.
– Host mode
This bit is set by the software to send a SETUP transaction on a control endpoint. This bit
changes its value only for control endpoints. It is cleared by hardware when the SETUP
transaction is acknowledged and VTTX interrupt generated.
Bits 10:9 UTYPE[1:0]: USB type of transaction
These bits configure the behavior of this endpoint/channel as described in Table 742:
Endpoint/channel type encoding. Channel0/Endpoint0 must always be a control
endpoint/channel and each USB function must have at least one control endpoint/channel
which has address 0, but there can be other control channels/endpoints if required. Only
control channels/endpoints handle SETUP transactions, which are ignored by endpoints of
other kinds. SETUP transactions cannot be answered with NAK or STALL. If a control
endpoint/channel is defined as NAK, the USB peripheral does not answer, simulating a
receive error, in the receive direction when a SETUP transaction is received. If the control
endpoint/channel is defined as STALL in the receive direction, then the SETUP packet is
accepted anyway, transferring data and issuing the CTR interrupt. The reception of OUT
transactions is handled in the normal way, even if the endpoint/channel is a control one.
Bulk and interrupt endpoints have very similar behavior and they differ only in the special
feature available using the EPKIND configuration bit.
The usage of isochronous channels/endpoints is explained in Section 71.5.5: Isochronous
transfers in Device mode
Bit 8 EPKIND: endpoint/channel kind
The meaning of this bit depends on the endpoint/channel type configured by the UTYPE
bits. Table 743 summarizes the different meanings.
DBL_BUF: This bit is set by the software to enable the double-buffering feature for this bulk
endpoint. The usage of double-buffered bulk endpoints is explained in Section 71.5.3:
Double-buffered endpoints and usage in Device mode.
STATUS_OUT: This bit is set by the software to indicate that a status out transaction is
expected: in this case all OUT transactions containing more than zero data bytes are
answered ‘STALL’ instead of ‘ACK’. This bit can be used to improve the robustness of the
application to protocol errors during control transfers and its usage is intended for control
endpoints only. When STATUS_OUT is reset, OUT transactions can have any number of
bytes, as required.
Bit 7 VTTX: Valid USB transaction transmitted
– Device mode
This bit is set by the hardware when an IN transaction is successfully completed on this
endpoint; the software can only clear this bit. If the CTRM bit in the USB_CNTR register is
set accordingly, a generic interrupt condition is generated together with the endpoint related
interrupt condition, which is always activated.
A transaction ended with a NAK or STALL handshake does not set this bit, since no data is
actually transferred, as in the case of protocol errors or data toggle mismatches.
This bit is read/write but only 0 can be written.
– Host mode
Same as VTRX behavior but for USB OUT and SETUP transactions.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Universal serial bus full-speed host/device interface (USB)

Bit 6 DTOGTX: Data toggle, for transmission transfers
If the endpoint/channel is non-isochronous, this bit contains the required value of the data
toggle bit (0 = DATA0, 1 = DATA1) for the next data packet to be transmitted. Hardware
toggles this bit when the ACK handshake is received from the USB host, following a data
packet transmission. If the endpoint/channel is defined as a control one, hardware sets this
bit to 1 at the reception of a SETUP PID addressed to this endpoint (in device mode) or
when a SETUP transaction is acknowledged by the device (in host mode).
If the endpoint/channel is using the double buffer feature, this bit is used to support packet
buffer swapping too (Refer to Section 71.5.3: Double-buffered endpoints and usage in
Device mode).
If the endpoint/channel is isochronous, this bit is used to support packet buffer swapping
since no data toggling is used for this sort of endpoints and only DATA0 packet are
transmitted (refer to Section 71.5.5: Isochronous transfers in Device mode). Hardware
toggles this bit just after the end of data packet transmission, since no handshake is used for
isochronous transfers.
This bit can also be toggled by the software to initialize its value (mandatory when the
endpoint/channel is not a control one) or to force a specific data toggle/packet buffer usage.
When the application software writes 0, the value of DTOGTX remains unchanged, while
writing 1 makes the bit value to toggle. This bit is read/write but it can only be toggled by
writing 1.
Bits 5:4 STATTX[1:0]: Status bits, for transmission transfers
– Device mode
These bits contain the information about the endpoint status, listed in Table 744. These bits
can be toggled by the software to initialize their value. When the application software writes
0, the value remains unchanged, while writing 1 makes the bit value to toggle. Hardware
sets the STATTX bits to NAK, when a correct transfer has occurred (VTTX = 1)
corresponding to a IN or SETUP (control only) transaction addressed to this
channel/endpoint. It then waits for the software to prepare the next set of data to be
transmitted.
Double-buffered bulk endpoints implement a special transaction flow control, which controls
the status based on buffer availability condition (Refer to Section 71.5.3: Double-buffered
endpoints and usage in Device mode).
If the endpoint is defined as isochronous, its status can only be “VALID” or “DISABLED”.
Therefore, the hardware cannot change the status of the channel/endpoint/channel after a
successful transaction. If the software sets the STATTX bits to ‘STALL’ or ‘NAK’ for an
isochronous channel/endpoint, the USB peripheral behavior is not defined. These bits are
read/write but they can be only toggled by writing 1.
– Host mode
The STATTX bits contain the information about the channel status. Refer to Table 744 for the
full descriptions (“Host mode” descriptions). Whereas in Device mode, these bits contain the
status that are given out on the following transaction, in Host mode they capture the status
last received from the device. If a NAK is received, STATTX contains the value indicating
NAK.
Bits 3:0 EA[3:0]: endpoint/channel address
– Device mode
Software must write in this field the 4-bit address used to identify the transactions directed to
this endpoint. A value must be written before enabling the corresponding endpoint.
– Host mode
Software must write in this field the 4-bit address used to identify the channel addressed by
the host transaction.

RM0456 Rev 6

<!-- pagebreak -->

